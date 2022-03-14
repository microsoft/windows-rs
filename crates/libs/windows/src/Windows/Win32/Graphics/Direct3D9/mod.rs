#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D9_RESOURCE_PRIORITY_HIGH: u32 = 2684354560u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D9_RESOURCE_PRIORITY_LOW: u32 = 1342177280u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D9_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D9_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D9_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D9b_SDK_VERSION: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DADAPTER_DEFAULT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [super::super::Foundation::CHAR; 512],
    pub Description: [super::super::Foundation::CHAR; 512],
    pub DeviceName: [super::super::Foundation::CHAR; 32],
    pub DriverVersion: i64,
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DeviceIdentifier: ::windows::core::GUID,
    pub WHQLLevel: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DADAPTER_IDENTIFIER9 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DADAPTER_IDENTIFIER9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DADAPTER_IDENTIFIER9 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DADAPTER_IDENTIFIER9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DADAPTER_IDENTIFIER9>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DADAPTER_IDENTIFIER9 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [super::super::Foundation::CHAR; 512],
    pub Description: [super::super::Foundation::CHAR; 512],
    pub DeviceName: [super::super::Foundation::CHAR; 32],
    pub DriverVersion: i64,
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DeviceIdentifier: ::windows::core::GUID,
    pub WHQLLevel: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DADAPTER_IDENTIFIER9 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DADAPTER_IDENTIFIER9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DADAPTER_IDENTIFIER9 {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DADAPTER_IDENTIFIER9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DADAPTER_IDENTIFIER9>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DADAPTER_IDENTIFIER9 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct D3DAES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for D3DAES_CTR_IV {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for D3DAES_CTR_IV {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for D3DAES_CTR_IV {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for D3DAES_CTR_IV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAES_CTR_IV>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for D3DAES_CTR_IV {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for D3DAES_CTR_IV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(target_arch = "x86")]
pub struct D3DAES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for D3DAES_CTR_IV {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for D3DAES_CTR_IV {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for D3DAES_CTR_IV {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for D3DAES_CTR_IV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAES_CTR_IV>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for D3DAES_CTR_IV {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for D3DAES_CTR_IV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DAUTHENTICATEDCHANNELTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DAUTHENTICATEDCHANNEL_D3D9: D3DAUTHENTICATEDCHANNELTYPE = D3DAUTHENTICATEDCHANNELTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_SOFTWARE: D3DAUTHENTICATEDCHANNELTYPE = D3DAUTHENTICATEDCHANNELTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_HARDWARE: D3DAUTHENTICATEDCHANNELTYPE = D3DAUTHENTICATEDCHANNELTYPE(3i32);
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNELTYPE {}
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNELTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DAUTHENTICATEDCHANNELTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNELTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNELTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DAUTHENTICATEDCHANNELTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub DXVA2DecodeHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION").field("Parameters", &self.Parameters).field("DXVA2DecodeHandle", &self.DXVA2DecodeHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub StartSequenceQuery: u32,
    pub StartSequenceConfigure: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE").field("Parameters", &self.Parameters).field("StartSequenceQuery", &self.StartSequenceQuery).field("StartSequenceConfigure", &self.StartSequenceConfigure).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub Protections: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub ProcessIdentiferType: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::super::Foundation::HANDLE,
    pub AllowAccess: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE").field("Parameters", &self.Parameters).field("ProcessIdentiferType", &self.ProcessIdentiferType).field("ProcessHandle", &self.ProcessHandle).field("AllowAccess", &self.AllowAccess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub EncryptionGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION").field("Parameters", &self.Parameters).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: ::windows::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT").field("omac", &self.omac).field("ConfigureType", &self.ConfigureType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: ::windows::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT").field("omac", &self.omac).field("ConfigureType", &self.ConfigureType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).field("ReturnCode", &self.ReturnCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const PROCESSIDTYPE_UNKNOWN: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const PROCESSIDTYPE_DWM: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const PROCESSIDTYPE_HANDLE: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(2i32);
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE {}
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0,
}
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {}
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {}
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub union D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0,
    pub Value: u32,
}
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {}
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {}
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {}
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {}
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT").field("Output", &self.Output).field("ChannelType", &self.ChannelType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DXVA2DecodeHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT").field("Input", &self.Input).field("DXVA2DecodeHandle", &self.DXVA2DecodeHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DXVA2DecodeHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT").field("Output", &self.Output).field("DXVA2DecodeHandle", &self.DXVA2DecodeHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT").field("Output", &self.Output).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumEncryptionGuids: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT").field("Output", &self.Output).field("NumEncryptionGuids", &self.NumEncryptionGuids).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub EncryptionGuidIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT").field("Input", &self.Input).field("EncryptionGuidIndex", &self.EncryptionGuidIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT").field("Output", &self.Output).field("EncryptionGuidIndex", &self.EncryptionGuidIndex).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub BusType: D3DBUSTYPE,
    pub bAccessibleInContiguousBlocks: super::super::Foundation::BOOL,
    pub bAccessibleInNonContiguousBlocks: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT").field("Output", &self.Output).field("BusType", &self.BusType).field("bAccessibleInContiguousBlocks", &self.bAccessibleInContiguousBlocks).field("bAccessibleInNonContiguousBlocks", &self.bAccessibleInNonContiguousBlocks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT").field("Input", &self.Input).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub NumOutputIDs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT").field("Output", &self.Output).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("NumOutputIDs", &self.NumOutputIDs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT").field("Input", &self.Input).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("OutputIDIndex", &self.OutputIDIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProtectionFlags: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumRestrictedSharedResourceProcesses: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT").field("Output", &self.Output).field("NumRestrictedSharedResourceProcesses", &self.NumRestrictedSharedResourceProcesses).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub ProcessIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT").field("Input", &self.Input).field("ProcessIndex", &self.ProcessIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProcessIndex: u32,
    pub ProcessIdentifer: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT").field("Output", &self.Output).field("ProcessIndex", &self.ProcessIndex).field("ProcessIdentifer", &self.ProcessIdentifer).field("ProcessHandle", &self.ProcessHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT").field("Output", &self.Output).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumUnrestrictedProtectedSharedResources: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT").field("Output", &self.Output).field("NumUnrestrictedProtectedSharedResources", &self.NumUnrestrictedProtectedSharedResources).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    pub QueryType: ::windows::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERY_INPUT").field("QueryType", &self.QueryType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERY_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    pub omac: D3D_OMAC,
    pub QueryType: ::windows::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT").field("omac", &self.omac).field("QueryType", &self.QueryType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).field("ReturnCode", &self.ReturnCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3DAUTHENTICATEDCONFIGURE_CRYPTOSESSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6346cc54_2cfc_4ad4_8224_d15837de7700);
pub const D3DAUTHENTICATEDCONFIGURE_ENCRYPTIONWHENACCESSIBLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41fff286_6ae0_4d43_9d55_a46e9efd158a);
pub const D3DAUTHENTICATEDCONFIGURE_INITIALIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06114bdb_3523_470a_8dca_fbc2845154f0);
pub const D3DAUTHENTICATEDCONFIGURE_PROTECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50455658_3f47_4362_bf99_bfdfcde9ed29);
pub const D3DAUTHENTICATEDCONFIGURE_SHAREDRESOURCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0772d047_1b40_48e8_9ca6_b5f510de9f01);
pub const D3DAUTHENTICATEDQUERY_ACCESSIBILITYATTRIBUTES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6214d9d2_432c_4abb_9fce_216eea269e3b);
pub const D3DAUTHENTICATEDQUERY_CHANNELTYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc1b18a5_b1fb_42ab_bd94_b5828b4bf7be);
pub const D3DAUTHENTICATEDQUERY_CRYPTOSESSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2634499e_d018_4d74_ac17_7f724059528d);
pub const D3DAUTHENTICATEDQUERY_CURRENTENCRYPTIONWHENACCESSIBLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec1791c7_dad3_4f15_9ec3_faa93d60d4f0);
pub const D3DAUTHENTICATEDQUERY_DEVICEHANDLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec1c539d_8cff_4e2a_bcc4_f5692f99f480);
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf83a5958_e986_4bda_beb0_411f6a7a01b7);
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUIDCOUNT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb30f7066_203c_4b07_93fc_ceaafd61241e);
pub const D3DAUTHENTICATEDQUERY_OUTPUTID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x839ddca3_9b4e_41e4_b053_892bd2a11ee7);
pub const D3DAUTHENTICATEDQUERY_OUTPUTIDCOUNT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c042b5e_8c07_46d5_aabe_8f75cbad4c31);
pub const D3DAUTHENTICATEDQUERY_PROTECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa84eb584_c495_48aa_b94d_8bd2d6fbce05);
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x649bbadb_f0f4_4639_a15b_24393fc3abac);
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESSCOUNT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0db207b3_9450_46a6_82de_1b96d44f9cf2);
pub const D3DAUTHENTICATEDQUERY_UNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x012f0bd6_e662_4474_befd_aa53e5143c6d);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DBACKBUFFER_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBACKBUFFER_TYPE_MONO: D3DBACKBUFFER_TYPE = D3DBACKBUFFER_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBACKBUFFER_TYPE_LEFT: D3DBACKBUFFER_TYPE = D3DBACKBUFFER_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBACKBUFFER_TYPE_RIGHT: D3DBACKBUFFER_TYPE = D3DBACKBUFFER_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBACKBUFFER_TYPE_FORCE_DWORD: D3DBACKBUFFER_TYPE = D3DBACKBUFFER_TYPE(2147483647u32);
impl ::core::marker::Copy for D3DBACKBUFFER_TYPE {}
impl ::core::clone::Clone for D3DBACKBUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DBACKBUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DBACKBUFFER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DBACKBUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBACKBUFFER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DBASISTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBASIS_BEZIER: D3DBASISTYPE = D3DBASISTYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBASIS_BSPLINE: D3DBASISTYPE = D3DBASISTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBASIS_CATMULL_ROM: D3DBASISTYPE = D3DBASISTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBASIS_FORCE_DWORD: D3DBASISTYPE = D3DBASISTYPE(2147483647i32);
impl ::core::marker::Copy for D3DBASISTYPE {}
impl ::core::clone::Clone for D3DBASISTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DBASISTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DBASISTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DBASISTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBASISTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DBLEND(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_ZERO: D3DBLEND = D3DBLEND(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_ONE: D3DBLEND = D3DBLEND(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_SRCCOLOR: D3DBLEND = D3DBLEND(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_INVSRCCOLOR: D3DBLEND = D3DBLEND(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_SRCALPHA: D3DBLEND = D3DBLEND(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_INVSRCALPHA: D3DBLEND = D3DBLEND(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_DESTALPHA: D3DBLEND = D3DBLEND(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_INVDESTALPHA: D3DBLEND = D3DBLEND(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_DESTCOLOR: D3DBLEND = D3DBLEND(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_INVDESTCOLOR: D3DBLEND = D3DBLEND(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_SRCALPHASAT: D3DBLEND = D3DBLEND(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_BOTHSRCALPHA: D3DBLEND = D3DBLEND(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_BOTHINVSRCALPHA: D3DBLEND = D3DBLEND(13u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_BLENDFACTOR: D3DBLEND = D3DBLEND(14u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_INVBLENDFACTOR: D3DBLEND = D3DBLEND(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_SRCCOLOR2: D3DBLEND = D3DBLEND(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_INVSRCCOLOR2: D3DBLEND = D3DBLEND(17u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLEND_FORCE_DWORD: D3DBLEND = D3DBLEND(2147483647u32);
impl ::core::marker::Copy for D3DBLEND {}
impl ::core::clone::Clone for D3DBLEND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DBLEND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DBLEND {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DBLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBLEND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DBLENDOP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLENDOP_ADD: D3DBLENDOP = D3DBLENDOP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLENDOP_SUBTRACT: D3DBLENDOP = D3DBLENDOP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLENDOP_REVSUBTRACT: D3DBLENDOP = D3DBLENDOP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLENDOP_MIN: D3DBLENDOP = D3DBLENDOP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLENDOP_MAX: D3DBLENDOP = D3DBLENDOP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBLENDOP_FORCE_DWORD: D3DBLENDOP = D3DBLENDOP(2147483647u32);
impl ::core::marker::Copy for D3DBLENDOP {}
impl ::core::clone::Clone for D3DBLENDOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DBLENDOP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DBLENDOP {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DBLENDOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBLENDOP").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DBOX {
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub Front: u32,
    pub Back: u32,
}
impl ::core::marker::Copy for D3DBOX {}
impl ::core::clone::Clone for D3DBOX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DBOX").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).field("Front", &self.Front).field("Back", &self.Back).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DBOX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DBOX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DBOX>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DBOX {}
impl ::core::default::Default for D3DBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DBUSTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSTYPE_OTHER: D3DBUSTYPE = D3DBUSTYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSTYPE_PCI: D3DBUSTYPE = D3DBUSTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSTYPE_PCIX: D3DBUSTYPE = D3DBUSTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSTYPE_PCIEXPRESS: D3DBUSTYPE = D3DBUSTYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSTYPE_AGP: D3DBUSTYPE = D3DBUSTYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSIMPL_MODIFIER_INSIDE_OF_CHIPSET: D3DBUSTYPE = D3DBUSTYPE(65536i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: D3DBUSTYPE = D3DBUSTYPE(131072i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: D3DBUSTYPE = D3DBUSTYPE(196608i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR: D3DBUSTYPE = D3DBUSTYPE(262144i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: D3DBUSTYPE = D3DBUSTYPE(327680i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DBUSIMPL_MODIFIER_NON_STANDARD: D3DBUSTYPE = D3DBUSTYPE(-2147483648i32);
impl ::core::marker::Copy for D3DBUSTYPE {}
impl ::core::clone::Clone for D3DBUSTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DBUSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DBUSTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DBUSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBUSTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS2_CANCALIBRATEGAMMA: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS2_CANMANAGERESOURCE: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS2_CANSHARERESOURCE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS2_DYNAMICTEXTURES: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS2_FULLSCREENGAMMA: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS2_RESERVED: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS3_COPY_TO_SYSTEMMEM: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS3_COPY_TO_VIDMEM: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS3_DXVAHD: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS3_DXVAHD_LIMITED: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS3_RESERVED: i32 = -2147483617i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DCAPS9 {
    pub DeviceType: D3DDEVTYPE,
    pub AdapterOrdinal: u32,
    pub Caps: u32,
    pub Caps2: u32,
    pub Caps3: u32,
    pub PresentationIntervals: u32,
    pub CursorCaps: u32,
    pub DevCaps: u32,
    pub PrimitiveMiscCaps: u32,
    pub RasterCaps: u32,
    pub ZCmpCaps: u32,
    pub SrcBlendCaps: u32,
    pub DestBlendCaps: u32,
    pub AlphaCmpCaps: u32,
    pub ShadeCaps: u32,
    pub TextureCaps: u32,
    pub TextureFilterCaps: u32,
    pub CubeTextureFilterCaps: u32,
    pub VolumeTextureFilterCaps: u32,
    pub TextureAddressCaps: u32,
    pub VolumeTextureAddressCaps: u32,
    pub LineCaps: u32,
    pub MaxTextureWidth: u32,
    pub MaxTextureHeight: u32,
    pub MaxVolumeExtent: u32,
    pub MaxTextureRepeat: u32,
    pub MaxTextureAspectRatio: u32,
    pub MaxAnisotropy: u32,
    pub MaxVertexW: f32,
    pub GuardBandLeft: f32,
    pub GuardBandTop: f32,
    pub GuardBandRight: f32,
    pub GuardBandBottom: f32,
    pub ExtentsAdjust: f32,
    pub StencilCaps: u32,
    pub FVFCaps: u32,
    pub TextureOpCaps: u32,
    pub MaxTextureBlendStages: u32,
    pub MaxSimultaneousTextures: u32,
    pub VertexProcessingCaps: u32,
    pub MaxActiveLights: u32,
    pub MaxUserClipPlanes: u32,
    pub MaxVertexBlendMatrices: u32,
    pub MaxVertexBlendMatrixIndex: u32,
    pub MaxPointSize: f32,
    pub MaxPrimitiveCount: u32,
    pub MaxVertexIndex: u32,
    pub MaxStreams: u32,
    pub MaxStreamStride: u32,
    pub VertexShaderVersion: u32,
    pub MaxVertexShaderConst: u32,
    pub PixelShaderVersion: u32,
    pub PixelShader1xMaxValue: f32,
    pub DevCaps2: u32,
    pub MaxNpatchTessellationLevel: f32,
    pub Reserved5: u32,
    pub MasterAdapterOrdinal: u32,
    pub AdapterOrdinalInGroup: u32,
    pub NumberOfAdaptersInGroup: u32,
    pub DeclTypes: u32,
    pub NumSimultaneousRTs: u32,
    pub StretchRectFilterCaps: u32,
    pub VS20Caps: D3DVSHADERCAPS2_0,
    pub PS20Caps: D3DPSHADERCAPS2_0,
    pub VertexTextureFilterCaps: u32,
    pub MaxVShaderInstructionsExecuted: u32,
    pub MaxPShaderInstructionsExecuted: u32,
    pub MaxVertexShader30InstructionSlots: u32,
    pub MaxPixelShader30InstructionSlots: u32,
}
impl ::core::marker::Copy for D3DCAPS9 {}
impl ::core::clone::Clone for D3DCAPS9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DCAPS9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCAPS9")
            .field("DeviceType", &self.DeviceType)
            .field("AdapterOrdinal", &self.AdapterOrdinal)
            .field("Caps", &self.Caps)
            .field("Caps2", &self.Caps2)
            .field("Caps3", &self.Caps3)
            .field("PresentationIntervals", &self.PresentationIntervals)
            .field("CursorCaps", &self.CursorCaps)
            .field("DevCaps", &self.DevCaps)
            .field("PrimitiveMiscCaps", &self.PrimitiveMiscCaps)
            .field("RasterCaps", &self.RasterCaps)
            .field("ZCmpCaps", &self.ZCmpCaps)
            .field("SrcBlendCaps", &self.SrcBlendCaps)
            .field("DestBlendCaps", &self.DestBlendCaps)
            .field("AlphaCmpCaps", &self.AlphaCmpCaps)
            .field("ShadeCaps", &self.ShadeCaps)
            .field("TextureCaps", &self.TextureCaps)
            .field("TextureFilterCaps", &self.TextureFilterCaps)
            .field("CubeTextureFilterCaps", &self.CubeTextureFilterCaps)
            .field("VolumeTextureFilterCaps", &self.VolumeTextureFilterCaps)
            .field("TextureAddressCaps", &self.TextureAddressCaps)
            .field("VolumeTextureAddressCaps", &self.VolumeTextureAddressCaps)
            .field("LineCaps", &self.LineCaps)
            .field("MaxTextureWidth", &self.MaxTextureWidth)
            .field("MaxTextureHeight", &self.MaxTextureHeight)
            .field("MaxVolumeExtent", &self.MaxVolumeExtent)
            .field("MaxTextureRepeat", &self.MaxTextureRepeat)
            .field("MaxTextureAspectRatio", &self.MaxTextureAspectRatio)
            .field("MaxAnisotropy", &self.MaxAnisotropy)
            .field("MaxVertexW", &self.MaxVertexW)
            .field("GuardBandLeft", &self.GuardBandLeft)
            .field("GuardBandTop", &self.GuardBandTop)
            .field("GuardBandRight", &self.GuardBandRight)
            .field("GuardBandBottom", &self.GuardBandBottom)
            .field("ExtentsAdjust", &self.ExtentsAdjust)
            .field("StencilCaps", &self.StencilCaps)
            .field("FVFCaps", &self.FVFCaps)
            .field("TextureOpCaps", &self.TextureOpCaps)
            .field("MaxTextureBlendStages", &self.MaxTextureBlendStages)
            .field("MaxSimultaneousTextures", &self.MaxSimultaneousTextures)
            .field("VertexProcessingCaps", &self.VertexProcessingCaps)
            .field("MaxActiveLights", &self.MaxActiveLights)
            .field("MaxUserClipPlanes", &self.MaxUserClipPlanes)
            .field("MaxVertexBlendMatrices", &self.MaxVertexBlendMatrices)
            .field("MaxVertexBlendMatrixIndex", &self.MaxVertexBlendMatrixIndex)
            .field("MaxPointSize", &self.MaxPointSize)
            .field("MaxPrimitiveCount", &self.MaxPrimitiveCount)
            .field("MaxVertexIndex", &self.MaxVertexIndex)
            .field("MaxStreams", &self.MaxStreams)
            .field("MaxStreamStride", &self.MaxStreamStride)
            .field("VertexShaderVersion", &self.VertexShaderVersion)
            .field("MaxVertexShaderConst", &self.MaxVertexShaderConst)
            .field("PixelShaderVersion", &self.PixelShaderVersion)
            .field("PixelShader1xMaxValue", &self.PixelShader1xMaxValue)
            .field("DevCaps2", &self.DevCaps2)
            .field("MaxNpatchTessellationLevel", &self.MaxNpatchTessellationLevel)
            .field("Reserved5", &self.Reserved5)
            .field("MasterAdapterOrdinal", &self.MasterAdapterOrdinal)
            .field("AdapterOrdinalInGroup", &self.AdapterOrdinalInGroup)
            .field("NumberOfAdaptersInGroup", &self.NumberOfAdaptersInGroup)
            .field("DeclTypes", &self.DeclTypes)
            .field("NumSimultaneousRTs", &self.NumSimultaneousRTs)
            .field("StretchRectFilterCaps", &self.StretchRectFilterCaps)
            .field("VS20Caps", &self.VS20Caps)
            .field("PS20Caps", &self.PS20Caps)
            .field("VertexTextureFilterCaps", &self.VertexTextureFilterCaps)
            .field("MaxVShaderInstructionsExecuted", &self.MaxVShaderInstructionsExecuted)
            .field("MaxPShaderInstructionsExecuted", &self.MaxPShaderInstructionsExecuted)
            .field("MaxVertexShader30InstructionSlots", &self.MaxVertexShader30InstructionSlots)
            .field("MaxPixelShader30InstructionSlots", &self.MaxPixelShader30InstructionSlots)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for D3DCAPS9 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DCAPS9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DCAPS9>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DCAPS9 {}
impl ::core::default::Default for D3DCAPS9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS_OVERLAY: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCAPS_READ_SCANLINE: i32 = 131072i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DCLIPSTATUS9 {
    pub ClipUnion: u32,
    pub ClipIntersection: u32,
}
impl ::core::marker::Copy for D3DCLIPSTATUS9 {}
impl ::core::clone::Clone for D3DCLIPSTATUS9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DCLIPSTATUS9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCLIPSTATUS9").field("ClipUnion", &self.ClipUnion).field("ClipIntersection", &self.ClipIntersection).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DCLIPSTATUS9 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DCLIPSTATUS9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DCLIPSTATUS9>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DCLIPSTATUS9 {}
impl ::core::default::Default for D3DCLIPSTATUS9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DCMPFUNC(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_NEVER: D3DCMPFUNC = D3DCMPFUNC(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_LESS: D3DCMPFUNC = D3DCMPFUNC(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_EQUAL: D3DCMPFUNC = D3DCMPFUNC(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_LESSEQUAL: D3DCMPFUNC = D3DCMPFUNC(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_GREATER: D3DCMPFUNC = D3DCMPFUNC(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_NOTEQUAL: D3DCMPFUNC = D3DCMPFUNC(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_GREATEREQUAL: D3DCMPFUNC = D3DCMPFUNC(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_ALWAYS: D3DCMPFUNC = D3DCMPFUNC(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCMP_FORCE_DWORD: D3DCMPFUNC = D3DCMPFUNC(2147483647i32);
impl ::core::marker::Copy for D3DCMPFUNC {}
impl ::core::clone::Clone for D3DCMPFUNC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DCMPFUNC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DCMPFUNC {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DCMPFUNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCMPFUNC").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for D3DCOLORVALUE {}
impl ::core::clone::Clone for D3DCOLORVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DCOLORVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCOLORVALUE").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DCOLORVALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DCOLORVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DCOLORVALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DCOLORVALUE {}
impl ::core::default::Default for D3DCOLORVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DCOMPOSERECTDESC {
    pub X: u16,
    pub Y: u16,
    pub Width: u16,
    pub Height: u16,
}
impl ::core::marker::Copy for D3DCOMPOSERECTDESC {}
impl ::core::clone::Clone for D3DCOMPOSERECTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DCOMPOSERECTDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCOMPOSERECTDESC").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DCOMPOSERECTDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DCOMPOSERECTDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DCOMPOSERECTDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DCOMPOSERECTDESC {}
impl ::core::default::Default for D3DCOMPOSERECTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DCOMPOSERECTDESTINATION {
    pub SrcRectIndex: u16,
    pub Reserved: u16,
    pub X: i16,
    pub Y: i16,
}
impl ::core::marker::Copy for D3DCOMPOSERECTDESTINATION {}
impl ::core::clone::Clone for D3DCOMPOSERECTDESTINATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DCOMPOSERECTDESTINATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCOMPOSERECTDESTINATION").field("SrcRectIndex", &self.SrcRectIndex).field("Reserved", &self.Reserved).field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DCOMPOSERECTDESTINATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DCOMPOSERECTDESTINATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DCOMPOSERECTDESTINATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DCOMPOSERECTDESTINATION {}
impl ::core::default::Default for D3DCOMPOSERECTDESTINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DCOMPOSERECTSOP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCOMPOSERECTS_COPY: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCOMPOSERECTS_OR: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCOMPOSERECTS_AND: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCOMPOSERECTS_NEG: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCOMPOSERECTS_FORCE_DWORD: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(2147483647i32);
impl ::core::marker::Copy for D3DCOMPOSERECTSOP {}
impl ::core::clone::Clone for D3DCOMPOSERECTSOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DCOMPOSERECTSOP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DCOMPOSERECTSOP {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DCOMPOSERECTSOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCOMPOSERECTSOP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCOMPOSERECTS_MAXNUMRECTS: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCONVOLUTIONMONO_MAXHEIGHT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCONVOLUTIONMONO_MAXWIDTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_CONTENTKEY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_ENCRYPTEDREADBACK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_ENCRYPTEDREADBACKKEY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_ENCRYPTSLICEDATAONLY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_FRESHENSESSIONKEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_PARTIALDECRYPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_PROTECTIONALWAYSON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_SEQUENTIAL_CTR_IV: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCPCAPS_SOFTWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_ADAPTERGROUP_DEVICE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_DISABLE_PRINTSCREEN: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_DISABLE_PSGP_THREADING: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_ENABLE_PRESENTSTATS: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_FPU_PRESERVE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_MIXED_VERTEXPROCESSING: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_MULTITHREADED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_NOWINDOWCHANGES: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_PUREDEVICE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_SCREENSAVER: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: i32 = 32i32;
pub const D3DCRYPTOTYPE_AES128_CTR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b6bd711_4f74_41c9_9e7b_0be2d7d93b4f);
pub const D3DCRYPTOTYPE_PROPRIETARY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab4e9afd_1d1c_46e6_a72f_0869917b0de8);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_BACK: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_BOTTOM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_FRONT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_LEFT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_PLANE0: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_PLANE1: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_PLANE2: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_PLANE3: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_PLANE4: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_PLANE5: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_RIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCS_TOP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DCUBEMAP_FACES(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCUBEMAP_FACE_POSITIVE_X: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCUBEMAP_FACE_NEGATIVE_X: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCUBEMAP_FACE_POSITIVE_Y: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCUBEMAP_FACE_NEGATIVE_Y: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCUBEMAP_FACE_POSITIVE_Z: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCUBEMAP_FACE_NEGATIVE_Z: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCUBEMAP_FACE_FORCE_DWORD: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(2147483647i32);
impl ::core::marker::Copy for D3DCUBEMAP_FACES {}
impl ::core::clone::Clone for D3DCUBEMAP_FACES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DCUBEMAP_FACES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DCUBEMAP_FACES {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DCUBEMAP_FACES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCUBEMAP_FACES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DCULL(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCULL_NONE: D3DCULL = D3DCULL(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCULL_CW: D3DCULL = D3DCULL(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCULL_CCW: D3DCULL = D3DCULL(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCULL_FORCE_DWORD: D3DCULL = D3DCULL(2147483647u32);
impl ::core::marker::Copy for D3DCULL {}
impl ::core::clone::Clone for D3DCULL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DCULL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DCULL {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DCULL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCULL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCURSORCAPS_COLOR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCURSORCAPS_LOWRES: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DCURSOR_IMMEDIATE_UPDATE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DDEBUGMONITORTOKENS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDMT_ENABLE: D3DDEBUGMONITORTOKENS = D3DDEBUGMONITORTOKENS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDMT_DISABLE: D3DDEBUGMONITORTOKENS = D3DDEBUGMONITORTOKENS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDMT_FORCE_DWORD: D3DDEBUGMONITORTOKENS = D3DDEBUGMONITORTOKENS(2147483647i32);
impl ::core::marker::Copy for D3DDEBUGMONITORTOKENS {}
impl ::core::clone::Clone for D3DDEBUGMONITORTOKENS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DDEBUGMONITORTOKENS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DDEBUGMONITORTOKENS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DDEBUGMONITORTOKENS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDEBUGMONITORTOKENS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DDECLMETHOD(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLMETHOD_DEFAULT: D3DDECLMETHOD = D3DDECLMETHOD(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLMETHOD_PARTIALU: D3DDECLMETHOD = D3DDECLMETHOD(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLMETHOD_PARTIALV: D3DDECLMETHOD = D3DDECLMETHOD(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLMETHOD_CROSSUV: D3DDECLMETHOD = D3DDECLMETHOD(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLMETHOD_UV: D3DDECLMETHOD = D3DDECLMETHOD(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLMETHOD_LOOKUP: D3DDECLMETHOD = D3DDECLMETHOD(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLMETHOD_LOOKUPPRESAMPLED: D3DDECLMETHOD = D3DDECLMETHOD(6i32);
impl ::core::marker::Copy for D3DDECLMETHOD {}
impl ::core::clone::Clone for D3DDECLMETHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DDECLMETHOD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DDECLMETHOD {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DDECLMETHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDECLMETHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DDECLTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_FLOAT1: D3DDECLTYPE = D3DDECLTYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_FLOAT2: D3DDECLTYPE = D3DDECLTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_FLOAT3: D3DDECLTYPE = D3DDECLTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_FLOAT4: D3DDECLTYPE = D3DDECLTYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_D3DCOLOR: D3DDECLTYPE = D3DDECLTYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_UBYTE4: D3DDECLTYPE = D3DDECLTYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_SHORT2: D3DDECLTYPE = D3DDECLTYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_SHORT4: D3DDECLTYPE = D3DDECLTYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_UBYTE4N: D3DDECLTYPE = D3DDECLTYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_SHORT2N: D3DDECLTYPE = D3DDECLTYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_SHORT4N: D3DDECLTYPE = D3DDECLTYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_USHORT2N: D3DDECLTYPE = D3DDECLTYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_USHORT4N: D3DDECLTYPE = D3DDECLTYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_UDEC3: D3DDECLTYPE = D3DDECLTYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_DEC3N: D3DDECLTYPE = D3DDECLTYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_FLOAT16_2: D3DDECLTYPE = D3DDECLTYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_FLOAT16_4: D3DDECLTYPE = D3DDECLTYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLTYPE_UNUSED: D3DDECLTYPE = D3DDECLTYPE(17i32);
impl ::core::marker::Copy for D3DDECLTYPE {}
impl ::core::clone::Clone for D3DDECLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DDECLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DDECLTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DDECLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDECLTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DDECLUSAGE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_POSITION: D3DDECLUSAGE = D3DDECLUSAGE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_BLENDWEIGHT: D3DDECLUSAGE = D3DDECLUSAGE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_BLENDINDICES: D3DDECLUSAGE = D3DDECLUSAGE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_NORMAL: D3DDECLUSAGE = D3DDECLUSAGE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_PSIZE: D3DDECLUSAGE = D3DDECLUSAGE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_TEXCOORD: D3DDECLUSAGE = D3DDECLUSAGE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_TANGENT: D3DDECLUSAGE = D3DDECLUSAGE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_BINORMAL: D3DDECLUSAGE = D3DDECLUSAGE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_TESSFACTOR: D3DDECLUSAGE = D3DDECLUSAGE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_POSITIONT: D3DDECLUSAGE = D3DDECLUSAGE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_COLOR: D3DDECLUSAGE = D3DDECLUSAGE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_FOG: D3DDECLUSAGE = D3DDECLUSAGE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_DEPTH: D3DDECLUSAGE = D3DDECLUSAGE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDECLUSAGE_SAMPLE: D3DDECLUSAGE = D3DDECLUSAGE(13i32);
impl ::core::marker::Copy for D3DDECLUSAGE {}
impl ::core::clone::Clone for D3DDECLUSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DDECLUSAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DDECLUSAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DDECLUSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDECLUSAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DDEGREETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEGREE_LINEAR: D3DDEGREETYPE = D3DDEGREETYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEGREE_QUADRATIC: D3DDEGREETYPE = D3DDEGREETYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEGREE_CUBIC: D3DDEGREETYPE = D3DDEGREETYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEGREE_QUINTIC: D3DDEGREETYPE = D3DDEGREETYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEGREE_FORCE_DWORD: D3DDEGREETYPE = D3DDEGREETYPE(2147483647i32);
impl ::core::marker::Copy for D3DDEGREETYPE {}
impl ::core::clone::Clone for D3DDEGREETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DDEGREETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DDEGREETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DDEGREETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDEGREETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS2_ADAPTIVETESSNPATCH: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS2_ADAPTIVETESSRTPATCH: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS2_DMAPNPATCH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS2_STREAMOFFSET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS_NPATCHES: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS_PUREDEVICE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS_QUINTICRTPATCHES: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS_RTPATCHES: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVCAPS_RTPATCHHANDLEZERO: i32 = 8388608i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DDEVICE_CREATION_PARAMETERS {
    pub AdapterOrdinal: u32,
    pub DeviceType: D3DDEVTYPE,
    pub hFocusWindow: super::super::Foundation::HWND,
    pub BehaviorFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DDEVICE_CREATION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DDEVICE_CREATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DDEVICE_CREATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVICE_CREATION_PARAMETERS").field("AdapterOrdinal", &self.AdapterOrdinal).field("DeviceType", &self.DeviceType).field("hFocusWindow", &self.hFocusWindow).field("BehaviorFlags", &self.BehaviorFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DDEVICE_CREATION_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DDEVICE_CREATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVICE_CREATION_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DDEVICE_CREATION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DDEVICE_CREATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    pub MaxBandwidthUtilized: f32,
    pub FrontEndUploadMemoryUtilizedPercent: f32,
    pub VertexRateUtilizedPercent: f32,
    pub TriangleSetupRateUtilizedPercent: f32,
    pub FillRateUtilizedPercent: f32,
}
impl ::core::marker::Copy for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {}
impl ::core::clone::Clone for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9BANDWIDTHTIMINGS").field("MaxBandwidthUtilized", &self.MaxBandwidthUtilized).field("FrontEndUploadMemoryUtilizedPercent", &self.FrontEndUploadMemoryUtilizedPercent).field("VertexRateUtilizedPercent", &self.VertexRateUtilizedPercent).field("TriangleSetupRateUtilizedPercent", &self.TriangleSetupRateUtilizedPercent).field("FillRateUtilizedPercent", &self.FillRateUtilizedPercent).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVINFO_D3D9BANDWIDTHTIMINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {}
impl ::core::default::Default for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDEVINFO_D3D9CACHEUTILIZATION {
    pub TextureCacheHitRate: f32,
    pub PostTransformVertexCacheHitRate: f32,
}
impl ::core::marker::Copy for D3DDEVINFO_D3D9CACHEUTILIZATION {}
impl ::core::clone::Clone for D3DDEVINFO_D3D9CACHEUTILIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9CACHEUTILIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9CACHEUTILIZATION").field("TextureCacheHitRate", &self.TextureCacheHitRate).field("PostTransformVertexCacheHitRate", &self.PostTransformVertexCacheHitRate).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDEVINFO_D3D9CACHEUTILIZATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9CACHEUTILIZATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVINFO_D3D9CACHEUTILIZATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9CACHEUTILIZATION {}
impl ::core::default::Default for D3DDEVINFO_D3D9CACHEUTILIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDEVINFO_D3D9INTERFACETIMINGS {
    pub WaitingForGPUToUseApplicationResourceTimePercent: f32,
    pub WaitingForGPUToAcceptMoreCommandsTimePercent: f32,
    pub WaitingForGPUToStayWithinLatencyTimePercent: f32,
    pub WaitingForGPUExclusiveResourceTimePercent: f32,
    pub WaitingForGPUOtherTimePercent: f32,
}
impl ::core::marker::Copy for D3DDEVINFO_D3D9INTERFACETIMINGS {}
impl ::core::clone::Clone for D3DDEVINFO_D3D9INTERFACETIMINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9INTERFACETIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9INTERFACETIMINGS")
            .field("WaitingForGPUToUseApplicationResourceTimePercent", &self.WaitingForGPUToUseApplicationResourceTimePercent)
            .field("WaitingForGPUToAcceptMoreCommandsTimePercent", &self.WaitingForGPUToAcceptMoreCommandsTimePercent)
            .field("WaitingForGPUToStayWithinLatencyTimePercent", &self.WaitingForGPUToStayWithinLatencyTimePercent)
            .field("WaitingForGPUExclusiveResourceTimePercent", &self.WaitingForGPUExclusiveResourceTimePercent)
            .field("WaitingForGPUOtherTimePercent", &self.WaitingForGPUOtherTimePercent)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDEVINFO_D3D9INTERFACETIMINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9INTERFACETIMINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVINFO_D3D9INTERFACETIMINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9INTERFACETIMINGS {}
impl ::core::default::Default for D3DDEVINFO_D3D9INTERFACETIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDEVINFO_D3D9PIPELINETIMINGS {
    pub VertexProcessingTimePercent: f32,
    pub PixelProcessingTimePercent: f32,
    pub OtherGPUProcessingTimePercent: f32,
    pub GPUIdleTimePercent: f32,
}
impl ::core::marker::Copy for D3DDEVINFO_D3D9PIPELINETIMINGS {}
impl ::core::clone::Clone for D3DDEVINFO_D3D9PIPELINETIMINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9PIPELINETIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9PIPELINETIMINGS").field("VertexProcessingTimePercent", &self.VertexProcessingTimePercent).field("PixelProcessingTimePercent", &self.PixelProcessingTimePercent).field("OtherGPUProcessingTimePercent", &self.OtherGPUProcessingTimePercent).field("GPUIdleTimePercent", &self.GPUIdleTimePercent).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDEVINFO_D3D9PIPELINETIMINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9PIPELINETIMINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVINFO_D3D9PIPELINETIMINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9PIPELINETIMINGS {}
impl ::core::default::Default for D3DDEVINFO_D3D9PIPELINETIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDEVINFO_D3D9STAGETIMINGS {
    pub MemoryProcessingPercent: f32,
    pub ComputationProcessingPercent: f32,
}
impl ::core::marker::Copy for D3DDEVINFO_D3D9STAGETIMINGS {}
impl ::core::clone::Clone for D3DDEVINFO_D3D9STAGETIMINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9STAGETIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9STAGETIMINGS").field("MemoryProcessingPercent", &self.MemoryProcessingPercent).field("ComputationProcessingPercent", &self.ComputationProcessingPercent).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDEVINFO_D3D9STAGETIMINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9STAGETIMINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVINFO_D3D9STAGETIMINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9STAGETIMINGS {}
impl ::core::default::Default for D3DDEVINFO_D3D9STAGETIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDEVINFO_D3DVERTEXSTATS {
    pub NumRenderedTriangles: u32,
    pub NumExtraClippingTriangles: u32,
}
impl ::core::marker::Copy for D3DDEVINFO_D3DVERTEXSTATS {}
impl ::core::clone::Clone for D3DDEVINFO_D3DVERTEXSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDEVINFO_D3DVERTEXSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3DVERTEXSTATS").field("NumRenderedTriangles", &self.NumRenderedTriangles).field("NumExtraClippingTriangles", &self.NumExtraClippingTriangles).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDEVINFO_D3DVERTEXSTATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3DVERTEXSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVINFO_D3DVERTEXSTATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3DVERTEXSTATS {}
impl ::core::default::Default for D3DDEVINFO_D3DVERTEXSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DDEVINFO_RESOURCEMANAGER {
    pub stats: [D3DRESOURCESTATS; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DDEVINFO_RESOURCEMANAGER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DDEVINFO_RESOURCEMANAGER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DDEVINFO_RESOURCEMANAGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_RESOURCEMANAGER").field("stats", &self.stats).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DDEVINFO_RESOURCEMANAGER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DDEVINFO_RESOURCEMANAGER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVINFO_RESOURCEMANAGER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DDEVINFO_RESOURCEMANAGER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DDEVINFO_RESOURCEMANAGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDEVINFO_VCACHE {
    pub Pattern: u32,
    pub OptMethod: u32,
    pub CacheSize: u32,
    pub MagicNumber: u32,
}
impl ::core::marker::Copy for D3DDEVINFO_VCACHE {}
impl ::core::clone::Clone for D3DDEVINFO_VCACHE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDEVINFO_VCACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_VCACHE").field("Pattern", &self.Pattern).field("OptMethod", &self.OptMethod).field("CacheSize", &self.CacheSize).field("MagicNumber", &self.MagicNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDEVINFO_VCACHE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDEVINFO_VCACHE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDEVINFO_VCACHE>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_VCACHE {}
impl ::core::default::Default for D3DDEVINFO_VCACHE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DDEVTYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVTYPE_HAL: D3DDEVTYPE = D3DDEVTYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVTYPE_REF: D3DDEVTYPE = D3DDEVTYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVTYPE_SW: D3DDEVTYPE = D3DDEVTYPE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVTYPE_NULLREF: D3DDEVTYPE = D3DDEVTYPE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDEVTYPE_FORCE_DWORD: D3DDEVTYPE = D3DDEVTYPE(2147483647u32);
impl ::core::marker::Copy for D3DDEVTYPE {}
impl ::core::clone::Clone for D3DDEVTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DDEVTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DDEVTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DDEVTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDEVTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDISPLAYMODE {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: u32,
    pub Format: D3DFORMAT,
}
impl ::core::marker::Copy for D3DDISPLAYMODE {}
impl ::core::clone::Clone for D3DDISPLAYMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDISPLAYMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDISPLAYMODE").field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDISPLAYMODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDISPLAYMODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDISPLAYMODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDISPLAYMODE {}
impl ::core::default::Default for D3DDISPLAYMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDISPLAYMODEEX {
    pub Size: u32,
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: u32,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
impl ::core::marker::Copy for D3DDISPLAYMODEEX {}
impl ::core::clone::Clone for D3DDISPLAYMODEEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDISPLAYMODEEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDISPLAYMODEEX").field("Size", &self.Size).field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).field("ScanLineOrdering", &self.ScanLineOrdering).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDISPLAYMODEEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDISPLAYMODEEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDISPLAYMODEEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDISPLAYMODEEX {}
impl ::core::default::Default for D3DDISPLAYMODEEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DDISPLAYMODEFILTER {
    pub Size: u32,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
impl ::core::marker::Copy for D3DDISPLAYMODEFILTER {}
impl ::core::clone::Clone for D3DDISPLAYMODEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DDISPLAYMODEFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDISPLAYMODEFILTER").field("Size", &self.Size).field("Format", &self.Format).field("ScanLineOrdering", &self.ScanLineOrdering).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DDISPLAYMODEFILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DDISPLAYMODEFILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DDISPLAYMODEFILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DDISPLAYMODEFILTER {}
impl ::core::default::Default for D3DDISPLAYMODEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DDISPLAYROTATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDISPLAYROTATION_IDENTITY: D3DDISPLAYROTATION = D3DDISPLAYROTATION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDISPLAYROTATION_90: D3DDISPLAYROTATION = D3DDISPLAYROTATION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDISPLAYROTATION_180: D3DDISPLAYROTATION = D3DDISPLAYROTATION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDISPLAYROTATION_270: D3DDISPLAYROTATION = D3DDISPLAYROTATION(4i32);
impl ::core::marker::Copy for D3DDISPLAYROTATION {}
impl ::core::clone::Clone for D3DDISPLAYROTATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DDISPLAYROTATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DDISPLAYROTATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DDISPLAYROTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDISPLAYROTATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDMAPSAMPLER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_DEC3N: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_FLOAT16_2: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_FLOAT16_4: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_SHORT2N: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_SHORT4N: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_UBYTE4: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_UBYTE4N: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_UDEC3: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_USHORT2N: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DDTCAPS_USHORT4N: i32 = 32i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: u32,
    pub NumBytesInSkipPattern: u32,
    pub NumBytesInEncryptPattern: u32,
}
impl ::core::marker::Copy for D3DENCRYPTED_BLOCK_INFO {}
impl ::core::clone::Clone for D3DENCRYPTED_BLOCK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DENCRYPTED_BLOCK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DENCRYPTED_BLOCK_INFO").field("NumEncryptedBytesAtBeginning", &self.NumEncryptedBytesAtBeginning).field("NumBytesInSkipPattern", &self.NumBytesInSkipPattern).field("NumBytesInEncryptPattern", &self.NumBytesInEncryptPattern).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DENCRYPTED_BLOCK_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DENCRYPTED_BLOCK_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DENCRYPTED_BLOCK_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DENCRYPTED_BLOCK_INFO {}
impl ::core::default::Default for D3DENCRYPTED_BLOCK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DENUM_NO_DRIVERVERSION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DENUM_WHQL_LEVEL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DFILLMODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFILL_POINT: D3DFILLMODE = D3DFILLMODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFILL_WIREFRAME: D3DFILLMODE = D3DFILLMODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFILL_SOLID: D3DFILLMODE = D3DFILLMODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFILL_FORCE_DWORD: D3DFILLMODE = D3DFILLMODE(2147483647i32);
impl ::core::marker::Copy for D3DFILLMODE {}
impl ::core::clone::Clone for D3DFILLMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DFILLMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DFILLMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DFILLMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DFILLMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A1_SURFACE_MAXHEIGHT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A1_SURFACE_MAXWIDTH: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DFOGMODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFOG_NONE: D3DFOGMODE = D3DFOGMODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFOG_EXP: D3DFOGMODE = D3DFOGMODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFOG_EXP2: D3DFOGMODE = D3DFOGMODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFOG_LINEAR: D3DFOGMODE = D3DFOGMODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFOG_FORCE_DWORD: D3DFOGMODE = D3DFOGMODE(2147483647i32);
impl ::core::marker::Copy for D3DFOGMODE {}
impl ::core::clone::Clone for D3DFOGMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DFOGMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DFOGMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DFOGMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DFOGMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DFORMAT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_UNKNOWN: D3DFORMAT = D3DFORMAT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_R8G8B8: D3DFORMAT = D3DFORMAT(20u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A8R8G8B8: D3DFORMAT = D3DFORMAT(21u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_X8R8G8B8: D3DFORMAT = D3DFORMAT(22u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_R5G6B5: D3DFORMAT = D3DFORMAT(23u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_X1R5G5B5: D3DFORMAT = D3DFORMAT(24u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A1R5G5B5: D3DFORMAT = D3DFORMAT(25u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A4R4G4B4: D3DFORMAT = D3DFORMAT(26u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_R3G3B2: D3DFORMAT = D3DFORMAT(27u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A8: D3DFORMAT = D3DFORMAT(28u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A8R3G3B2: D3DFORMAT = D3DFORMAT(29u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_X4R4G4B4: D3DFORMAT = D3DFORMAT(30u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A2B10G10R10: D3DFORMAT = D3DFORMAT(31u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A8B8G8R8: D3DFORMAT = D3DFORMAT(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_X8B8G8R8: D3DFORMAT = D3DFORMAT(33u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_G16R16: D3DFORMAT = D3DFORMAT(34u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A2R10G10B10: D3DFORMAT = D3DFORMAT(35u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A16B16G16R16: D3DFORMAT = D3DFORMAT(36u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A8P8: D3DFORMAT = D3DFORMAT(40u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_P8: D3DFORMAT = D3DFORMAT(41u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_L8: D3DFORMAT = D3DFORMAT(50u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A8L8: D3DFORMAT = D3DFORMAT(51u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A4L4: D3DFORMAT = D3DFORMAT(52u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_V8U8: D3DFORMAT = D3DFORMAT(60u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_L6V5U5: D3DFORMAT = D3DFORMAT(61u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_X8L8V8U8: D3DFORMAT = D3DFORMAT(62u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_Q8W8V8U8: D3DFORMAT = D3DFORMAT(63u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_V16U16: D3DFORMAT = D3DFORMAT(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A2W10V10U10: D3DFORMAT = D3DFORMAT(67u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_UYVY: D3DFORMAT = D3DFORMAT(1498831189u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_R8G8_B8G8: D3DFORMAT = D3DFORMAT(1195525970u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_YUY2: D3DFORMAT = D3DFORMAT(844715353u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_G8R8_G8B8: D3DFORMAT = D3DFORMAT(1111970375u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_DXT1: D3DFORMAT = D3DFORMAT(827611204u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_DXT2: D3DFORMAT = D3DFORMAT(844388420u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_DXT3: D3DFORMAT = D3DFORMAT(861165636u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_DXT4: D3DFORMAT = D3DFORMAT(877942852u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_DXT5: D3DFORMAT = D3DFORMAT(894720068u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D16_LOCKABLE: D3DFORMAT = D3DFORMAT(70u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D32: D3DFORMAT = D3DFORMAT(71u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D15S1: D3DFORMAT = D3DFORMAT(73u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D24S8: D3DFORMAT = D3DFORMAT(75u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D24X8: D3DFORMAT = D3DFORMAT(77u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D24X4S4: D3DFORMAT = D3DFORMAT(79u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D16: D3DFORMAT = D3DFORMAT(80u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D32F_LOCKABLE: D3DFORMAT = D3DFORMAT(82u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D24FS8: D3DFORMAT = D3DFORMAT(83u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_D32_LOCKABLE: D3DFORMAT = D3DFORMAT(84u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_S8_LOCKABLE: D3DFORMAT = D3DFORMAT(85u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_L16: D3DFORMAT = D3DFORMAT(81u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_VERTEXDATA: D3DFORMAT = D3DFORMAT(100u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_INDEX16: D3DFORMAT = D3DFORMAT(101u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_INDEX32: D3DFORMAT = D3DFORMAT(102u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_Q16W16V16U16: D3DFORMAT = D3DFORMAT(110u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_MULTI2_ARGB8: D3DFORMAT = D3DFORMAT(827606349u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_R16F: D3DFORMAT = D3DFORMAT(111u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_G16R16F: D3DFORMAT = D3DFORMAT(112u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A16B16G16R16F: D3DFORMAT = D3DFORMAT(113u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_R32F: D3DFORMAT = D3DFORMAT(114u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_G32R32F: D3DFORMAT = D3DFORMAT(115u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A32B32G32R32F: D3DFORMAT = D3DFORMAT(116u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_CxV8U8: D3DFORMAT = D3DFORMAT(117u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A1: D3DFORMAT = D3DFORMAT(118u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_A2B10G10R10_XR_BIAS: D3DFORMAT = D3DFORMAT(119u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_BINARYBUFFER: D3DFORMAT = D3DFORMAT(199u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFMT_FORCE_DWORD: D3DFORMAT = D3DFORMAT(2147483647u32);
impl ::core::marker::Copy for D3DFORMAT {}
impl ::core::clone::Clone for D3DFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DFORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DFORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFVFCAPS_PSIZE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFVF_LASTBETA_D3DCOLOR: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFVF_LASTBETA_UBYTE4: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFVF_PSIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DFVF_XYZW: u32 = 16386u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DGAMMARAMP {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}
impl ::core::marker::Copy for D3DGAMMARAMP {}
impl ::core::clone::Clone for D3DGAMMARAMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DGAMMARAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DGAMMARAMP").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DGAMMARAMP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DGAMMARAMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DGAMMARAMP>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DGAMMARAMP {}
impl ::core::default::Default for D3DGAMMARAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DGETDATA_FLUSH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DINDEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Size: u32,
}
impl ::core::marker::Copy for D3DINDEXBUFFER_DESC {}
impl ::core::clone::Clone for D3DINDEXBUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DINDEXBUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DINDEXBUFFER_DESC").field("Format", &self.Format).field("Type", &self.Type).field("Usage", &self.Usage).field("Pool", &self.Pool).field("Size", &self.Size).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DINDEXBUFFER_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DINDEXBUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DINDEXBUFFER_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DINDEXBUFFER_DESC {}
impl ::core::default::Default for D3DINDEXBUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DISSUE_BEGIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DISSUE_END: u32 = 1u32;
pub const D3DKEYEXCHANGE_DXVA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43d3775c_38e5_4924_8d86_d3fccf153e9b);
pub const D3DKEYEXCHANGE_RSAES_OAEP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1949895_d72a_4a1d_8e5d_ed857d171520);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3DLIGHT9 {
    pub Type: D3DLIGHTTYPE,
    pub Diffuse: D3DCOLORVALUE,
    pub Specular: D3DCOLORVALUE,
    pub Ambient: D3DCOLORVALUE,
    pub Position: super::Direct3D::D3DVECTOR,
    pub Direction: super::Direct3D::D3DVECTOR,
    pub Range: f32,
    pub Falloff: f32,
    pub Attenuation0: f32,
    pub Attenuation1: f32,
    pub Attenuation2: f32,
    pub Theta: f32,
    pub Phi: f32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3DLIGHT9 {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3DLIGHT9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3DLIGHT9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DLIGHT9").field("Type", &self.Type).field("Diffuse", &self.Diffuse).field("Specular", &self.Specular).field("Ambient", &self.Ambient).field("Position", &self.Position).field("Direction", &self.Direction).field("Range", &self.Range).field("Falloff", &self.Falloff).field("Attenuation0", &self.Attenuation0).field("Attenuation1", &self.Attenuation1).field("Attenuation2", &self.Attenuation2).field("Theta", &self.Theta).field("Phi", &self.Phi).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3DLIGHT9 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3DLIGHT9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DLIGHT9>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3DLIGHT9 {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3DLIGHT9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DLIGHTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLIGHT_POINT: D3DLIGHTTYPE = D3DLIGHTTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLIGHT_SPOT: D3DLIGHTTYPE = D3DLIGHTTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLIGHT_DIRECTIONAL: D3DLIGHTTYPE = D3DLIGHTTYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLIGHT_FORCE_DWORD: D3DLIGHTTYPE = D3DLIGHTTYPE(2147483647i32);
impl ::core::marker::Copy for D3DLIGHTTYPE {}
impl ::core::clone::Clone for D3DLIGHTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DLIGHTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DLIGHTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DLIGHTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DLIGHTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLINECAPS_ALPHACMP: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLINECAPS_ANTIALIAS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLINECAPS_BLEND: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLINECAPS_FOG: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLINECAPS_TEXTURE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLINECAPS_ZTEST: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DLOCKED_BOX {
    pub RowPitch: i32,
    pub SlicePitch: i32,
    pub pBits: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3DLOCKED_BOX {}
impl ::core::clone::Clone for D3DLOCKED_BOX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DLOCKED_BOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DLOCKED_BOX").field("RowPitch", &self.RowPitch).field("SlicePitch", &self.SlicePitch).field("pBits", &self.pBits).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DLOCKED_BOX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DLOCKED_BOX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DLOCKED_BOX>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DLOCKED_BOX {}
impl ::core::default::Default for D3DLOCKED_BOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DLOCKED_RECT {
    pub Pitch: i32,
    pub pBits: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3DLOCKED_RECT {}
impl ::core::clone::Clone for D3DLOCKED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DLOCKED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DLOCKED_RECT").field("Pitch", &self.Pitch).field("pBits", &self.pBits).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DLOCKED_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DLOCKED_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DLOCKED_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DLOCKED_RECT {}
impl ::core::default::Default for D3DLOCKED_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLOCK_DISCARD: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLOCK_DONOTWAIT: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLOCK_NOOVERWRITE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLOCK_NOSYSLOCK: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLOCK_NO_DIRTY_UPDATE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DLOCK_READONLY: i32 = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DMATERIAL9 {
    pub Diffuse: D3DCOLORVALUE,
    pub Ambient: D3DCOLORVALUE,
    pub Specular: D3DCOLORVALUE,
    pub Emissive: D3DCOLORVALUE,
    pub Power: f32,
}
impl ::core::marker::Copy for D3DMATERIAL9 {}
impl ::core::clone::Clone for D3DMATERIAL9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DMATERIAL9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DMATERIAL9").field("Diffuse", &self.Diffuse).field("Ambient", &self.Ambient).field("Specular", &self.Specular).field("Emissive", &self.Emissive).field("Power", &self.Power).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DMATERIAL9 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DMATERIAL9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DMATERIAL9>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DMATERIAL9 {}
impl ::core::default::Default for D3DMATERIAL9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DMATERIALCOLORSOURCE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMCS_MATERIAL: D3DMATERIALCOLORSOURCE = D3DMATERIALCOLORSOURCE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMCS_COLOR1: D3DMATERIALCOLORSOURCE = D3DMATERIALCOLORSOURCE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMCS_COLOR2: D3DMATERIALCOLORSOURCE = D3DMATERIALCOLORSOURCE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMCS_FORCE_DWORD: D3DMATERIALCOLORSOURCE = D3DMATERIALCOLORSOURCE(2147483647i32);
impl ::core::marker::Copy for D3DMATERIALCOLORSOURCE {}
impl ::core::clone::Clone for D3DMATERIALCOLORSOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DMATERIALCOLORSOURCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DMATERIALCOLORSOURCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DMATERIALCOLORSOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DMATERIALCOLORSOURCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMAX30SHADERINSTRUCTIONS: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: u64,
    pub SizeOfInefficientAllocation: u64,
    pub LevelOfEfficiency: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for D3DMEMORYPRESSURE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for D3DMEMORYPRESSURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for D3DMEMORYPRESSURE {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for D3DMEMORYPRESSURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DMEMORYPRESSURE>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for D3DMEMORYPRESSURE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for D3DMEMORYPRESSURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(target_arch = "x86")]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: u64,
    pub SizeOfInefficientAllocation: u64,
    pub LevelOfEfficiency: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for D3DMEMORYPRESSURE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for D3DMEMORYPRESSURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for D3DMEMORYPRESSURE {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for D3DMEMORYPRESSURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DMEMORYPRESSURE>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for D3DMEMORYPRESSURE {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for D3DMEMORYPRESSURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMIN30SHADERINSTRUCTIONS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DMULTISAMPLE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_NONE: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_NONMASKABLE: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_2_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_3_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_4_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_5_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_6_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_7_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_8_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_9_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_10_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_11_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_12_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_13_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_14_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_15_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_16_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMULTISAMPLE_FORCE_DWORD: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(2147483647i32);
impl ::core::marker::Copy for D3DMULTISAMPLE_TYPE {}
impl ::core::clone::Clone for D3DMULTISAMPLE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DMULTISAMPLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DMULTISAMPLE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DMULTISAMPLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DMULTISAMPLE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DOVERLAYCAPS_FULLRANGERGB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DOVERLAYCAPS_LIMITEDRANGERGB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DOVERLAYCAPS_STRETCHX: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DOVERLAYCAPS_STRETCHY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DOVERLAYCAPS_YCbCr_BT601: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DOVERLAYCAPS_YCbCr_BT601_xvYCC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DOVERLAYCAPS_YCbCr_BT709: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DOVERLAYCAPS_YCbCr_BT709_xvYCC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DPATCHEDGESTYLE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPATCHEDGE_DISCRETE: D3DPATCHEDGESTYLE = D3DPATCHEDGESTYLE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPATCHEDGE_CONTINUOUS: D3DPATCHEDGESTYLE = D3DPATCHEDGESTYLE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPATCHEDGE_FORCE_DWORD: D3DPATCHEDGESTYLE = D3DPATCHEDGESTYLE(2147483647i32);
impl ::core::marker::Copy for D3DPATCHEDGESTYLE {}
impl ::core::clone::Clone for D3DPATCHEDGESTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DPATCHEDGESTYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DPATCHEDGESTYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DPATCHEDGESTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DPATCHEDGESTYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPBLENDCAPS_BLENDFACTOR: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPBLENDCAPS_INVSRCCOLOR2: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPBLENDCAPS_SRCCOLOR2: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[inline]
pub unsafe fn D3DPERF_BeginEvent<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(col: u32, wszname: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DPERF_BeginEvent(col: u32, wszname: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(D3DPERF_BeginEvent(::core::mem::transmute(col), wszname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[inline]
pub unsafe fn D3DPERF_EndEvent() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DPERF_EndEvent() -> i32;
        }
        ::core::mem::transmute(D3DPERF_EndEvent())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[inline]
pub unsafe fn D3DPERF_GetStatus() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DPERF_GetStatus() -> u32;
        }
        ::core::mem::transmute(D3DPERF_GetStatus())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn D3DPERF_QueryRepeatFrame() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DPERF_QueryRepeatFrame() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(D3DPERF_QueryRepeatFrame())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[inline]
pub unsafe fn D3DPERF_SetMarker<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(col: u32, wszname: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DPERF_SetMarker(col: u32, wszname: ::windows::core::PCWSTR);
        }
        D3DPERF_SetMarker(::core::mem::transmute(col), wszname.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[inline]
pub unsafe fn D3DPERF_SetOptions(dwoptions: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DPERF_SetOptions(dwoptions: u32);
        }
        D3DPERF_SetOptions(::core::mem::transmute(dwoptions))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[inline]
pub unsafe fn D3DPERF_SetRegion<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(col: u32, wszname: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DPERF_SetRegion(col: u32, wszname: ::windows::core::PCWSTR);
        }
        D3DPERF_SetRegion(::core::mem::transmute(col), wszname.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_BLENDOP: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_CLIPPLANESCALEDPOINTS: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_CLIPTLVERTS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_COLORWRITEENABLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_FOGANDSPECULARALPHA: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_FOGVERTEXCLAMPED: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_INDEPENDENTWRITEMASKS: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_NULLREFERENCE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_PERSTAGECONSTANT: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_POSTBLENDSRGBCONVERT: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_SEPARATEALPHABLEND: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPMISCCAPS_TSSARGTEMP: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DPOOL(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPOOL_DEFAULT: D3DPOOL = D3DPOOL(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPOOL_MANAGED: D3DPOOL = D3DPOOL(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPOOL_SYSTEMMEM: D3DPOOL = D3DPOOL(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPOOL_SCRATCH: D3DPOOL = D3DPOOL(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPOOL_FORCE_DWORD: D3DPOOL = D3DPOOL(2147483647u32);
impl ::core::marker::Copy for D3DPOOL {}
impl ::core::clone::Clone for D3DPOOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DPOOL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DPOOL {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DPOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DPOOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRASTERCAPS_COLORPERSPECTIVE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRASTERCAPS_DEPTHBIAS: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRASTERCAPS_MULTISAMPLE_TOGGLE: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRASTERCAPS_SCISSORTEST: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_DEVICECLIP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_LOCKABLE_BACKBUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_NOAUTOROTATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_OVERLAY_LIMITEDRGB: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_BT709: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_RESTRICTED_CONTENT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_UNPRUNEDMODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENTFLAG_VIDEO: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct D3DPRESENTSTATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for D3DPRESENTSTATS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for D3DPRESENTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for D3DPRESENTSTATS {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for D3DPRESENTSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DPRESENTSTATS>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for D3DPRESENTSTATS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for D3DPRESENTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(target_arch = "x86")]
pub struct D3DPRESENTSTATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for D3DPRESENTSTATS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for D3DPRESENTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for D3DPRESENTSTATS {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for D3DPRESENTSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DPRESENTSTATS>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for D3DPRESENTSTATS {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for D3DPRESENTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_BACK_BUFFERS_MAX: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_BACK_BUFFERS_MAX_EX: i32 = 30i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_DONOTFLIP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_DONOTWAIT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_FLIPRESTART: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_FORCEIMMEDIATE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_HIDEOVERLAY: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_INTERVAL_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_INTERVAL_FOUR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_INTERVAL_IMMEDIATE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_INTERVAL_ONE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_INTERVAL_THREE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_INTERVAL_TWO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_LINEAR_CONTENT: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DPRESENT_PARAMETERS {
    pub BackBufferWidth: u32,
    pub BackBufferHeight: u32,
    pub BackBufferFormat: D3DFORMAT,
    pub BackBufferCount: u32,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: u32,
    pub SwapEffect: D3DSWAPEFFECT,
    pub hDeviceWindow: super::super::Foundation::HWND,
    pub Windowed: super::super::Foundation::BOOL,
    pub EnableAutoDepthStencil: super::super::Foundation::BOOL,
    pub AutoDepthStencilFormat: D3DFORMAT,
    pub Flags: u32,
    pub FullScreen_RefreshRateInHz: u32,
    pub PresentationInterval: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DPRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DPRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DPRESENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DPRESENT_PARAMETERS")
            .field("BackBufferWidth", &self.BackBufferWidth)
            .field("BackBufferHeight", &self.BackBufferHeight)
            .field("BackBufferFormat", &self.BackBufferFormat)
            .field("BackBufferCount", &self.BackBufferCount)
            .field("MultiSampleType", &self.MultiSampleType)
            .field("MultiSampleQuality", &self.MultiSampleQuality)
            .field("SwapEffect", &self.SwapEffect)
            .field("hDeviceWindow", &self.hDeviceWindow)
            .field("Windowed", &self.Windowed)
            .field("EnableAutoDepthStencil", &self.EnableAutoDepthStencil)
            .field("AutoDepthStencilFormat", &self.AutoDepthStencilFormat)
            .field("Flags", &self.Flags)
            .field("FullScreen_RefreshRateInHz", &self.FullScreen_RefreshRateInHz)
            .field("PresentationInterval", &self.PresentationInterval)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DPRESENT_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DPRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DPRESENT_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DPRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DPRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_RATE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_UPDATECOLORKEY: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_UPDATEOVERLAYONLY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DPRIMITIVETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPT_POINTLIST: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPT_LINELIST: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPT_LINESTRIP: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPT_TRIANGLELIST: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPT_TRIANGLESTRIP: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPT_TRIANGLEFAN: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPT_FORCE_DWORD: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(2147483647i32);
impl ::core::marker::Copy for D3DPRIMITIVETYPE {}
impl ::core::clone::Clone for D3DPRIMITIVETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DPRIMITIVETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DPRIMITIVETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DPRIMITIVETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DPRIMITIVETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20CAPS_ARBITRARYSWIZZLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20CAPS_GRADIENTINSTRUCTIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20CAPS_NODEPENDENTREADLIMIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20CAPS_NOTEXINSTRUCTIONLIMIT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20CAPS_PREDICATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20_MAX_NUMINSTRUCTIONSLOTS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20_MAX_NUMTEMPS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20_MIN_NUMINSTRUCTIONSLOTS: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20_MIN_NUMTEMPS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DPSHADERCAPS2_0 {
    pub Caps: u32,
    pub DynamicFlowControlDepth: i32,
    pub NumTemps: i32,
    pub StaticFlowControlDepth: i32,
    pub NumInstructionSlots: i32,
}
impl ::core::marker::Copy for D3DPSHADERCAPS2_0 {}
impl ::core::clone::Clone for D3DPSHADERCAPS2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DPSHADERCAPS2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DPSHADERCAPS2_0").field("Caps", &self.Caps).field("DynamicFlowControlDepth", &self.DynamicFlowControlDepth).field("NumTemps", &self.NumTemps).field("StaticFlowControlDepth", &self.StaticFlowControlDepth).field("NumInstructionSlots", &self.NumInstructionSlots).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DPSHADERCAPS2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DPSHADERCAPS2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DPSHADERCAPS2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DPSHADERCAPS2_0 {}
impl ::core::default::Default for D3DPSHADERCAPS2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTADDRESSCAPS_MIRRORONCE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTEXTURECAPS_CUBEMAP_POW2: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTEXTURECAPS_MIPCUBEMAP: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTEXTURECAPS_MIPMAP: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTEXTURECAPS_MIPVOLUMEMAP: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTEXTURECAPS_NOPROJECTEDBUMPENV: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTEXTURECAPS_VOLUMEMAP: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTEXTURECAPS_VOLUMEMAP_POW2: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTFILTERCAPS_CONVOLUTIONMONO: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTFILTERCAPS_MAGFGAUSSIANQUAD: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTFILTERCAPS_MINFGAUSSIANQUAD: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DPTFILTERCAPS_MINFPYRAMIDALQUAD: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DQUERYTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_VCACHE: D3DQUERYTYPE = D3DQUERYTYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_RESOURCEMANAGER: D3DQUERYTYPE = D3DQUERYTYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_VERTEXSTATS: D3DQUERYTYPE = D3DQUERYTYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_EVENT: D3DQUERYTYPE = D3DQUERYTYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_OCCLUSION: D3DQUERYTYPE = D3DQUERYTYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_TIMESTAMP: D3DQUERYTYPE = D3DQUERYTYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_TIMESTAMPDISJOINT: D3DQUERYTYPE = D3DQUERYTYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_TIMESTAMPFREQ: D3DQUERYTYPE = D3DQUERYTYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_PIPELINETIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_INTERFACETIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_VERTEXTIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_PIXELTIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_BANDWIDTHTIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_CACHEUTILIZATION: D3DQUERYTYPE = D3DQUERYTYPE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DQUERYTYPE_MEMORYPRESSURE: D3DQUERYTYPE = D3DQUERYTYPE(19i32);
impl ::core::marker::Copy for D3DQUERYTYPE {}
impl ::core::clone::Clone for D3DQUERYTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DQUERYTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DQUERYTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DQUERYTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DQUERYTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DRANGE {
    pub Offset: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for D3DRANGE {}
impl ::core::clone::Clone for D3DRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRANGE").field("Offset", &self.Offset).field("Size", &self.Size).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DRANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DRANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DRANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DRANGE {}
impl ::core::default::Default for D3DRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DRASTER_STATUS {
    pub InVBlank: super::super::Foundation::BOOL,
    pub ScanLine: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DRASTER_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DRASTER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DRASTER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRASTER_STATUS").field("InVBlank", &self.InVBlank).field("ScanLine", &self.ScanLine).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DRASTER_STATUS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DRASTER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DRASTER_STATUS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DRASTER_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DRASTER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DRECT {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}
impl ::core::marker::Copy for D3DRECT {}
impl ::core::clone::Clone for D3DRECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRECT").field("x1", &self.x1).field("y1", &self.y1).field("x2", &self.x2).field("y2", &self.y2).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DRECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DRECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DRECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DRECT {}
impl ::core::default::Default for D3DRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DRECTPATCH_INFO {
    pub StartVertexOffsetWidth: u32,
    pub StartVertexOffsetHeight: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
impl ::core::marker::Copy for D3DRECTPATCH_INFO {}
impl ::core::clone::Clone for D3DRECTPATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DRECTPATCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRECTPATCH_INFO").field("StartVertexOffsetWidth", &self.StartVertexOffsetWidth).field("StartVertexOffsetHeight", &self.StartVertexOffsetHeight).field("Width", &self.Width).field("Height", &self.Height).field("Stride", &self.Stride).field("Basis", &self.Basis).field("Degree", &self.Degree).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DRECTPATCH_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DRECTPATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DRECTPATCH_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DRECTPATCH_INFO {}
impl ::core::default::Default for D3DRECTPATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DRENDERSTATETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ZENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FILLMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SHADEMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ZWRITEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ALPHATESTENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_LASTPIXEL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SRCBLEND: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_DESTBLEND: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_CULLMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ZFUNC: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ALPHAREF: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ALPHAFUNC: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_DITHERENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ALPHABLENDENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FOGENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SPECULARENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FOGCOLOR: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FOGTABLEMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FOGSTART: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FOGEND: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FOGDENSITY: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_RANGEFOGENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_STENCILENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(52i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_STENCILFAIL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(53i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_STENCILZFAIL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(54i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_STENCILPASS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(55i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_STENCILFUNC: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(56i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_STENCILREF: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(57i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_STENCILMASK: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(58i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_STENCILWRITEMASK: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(59i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_TEXTUREFACTOR: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(60i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP0: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP1: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(129i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP2: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(130i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP3: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(131i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP4: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(132i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP5: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(133i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP6: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(134i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP7: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(135i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_CLIPPING: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(136i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_LIGHTING: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(137i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_AMBIENT: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(139i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FOGVERTEXMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(140i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_COLORVERTEX: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(141i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_LOCALVIEWER: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(142i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_NORMALIZENORMALS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(143i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_DIFFUSEMATERIALSOURCE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(145i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SPECULARMATERIALSOURCE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(146i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_AMBIENTMATERIALSOURCE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(147i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_EMISSIVEMATERIALSOURCE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(148i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_VERTEXBLEND: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(151i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_CLIPPLANEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(152i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POINTSIZE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(154i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POINTSIZE_MIN: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(155i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POINTSPRITEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(156i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POINTSCALEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(157i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POINTSCALE_A: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(158i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POINTSCALE_B: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(159i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POINTSCALE_C: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(160i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_MULTISAMPLEANTIALIAS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(161i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_MULTISAMPLEMASK: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(162i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_PATCHEDGESTYLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(163i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_DEBUGMONITORTOKEN: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(165i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POINTSIZE_MAX: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(166i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_INDEXEDVERTEXBLENDENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(167i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_COLORWRITEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(168i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_TWEENFACTOR: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(170i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_BLENDOP: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(171i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_POSITIONDEGREE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(172i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_NORMALDEGREE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(173i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SCISSORTESTENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(174i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SLOPESCALEDEPTHBIAS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(175i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ANTIALIASEDLINEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(176i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_MINTESSELLATIONLEVEL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(178i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_MAXTESSELLATIONLEVEL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(179i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ADAPTIVETESS_X: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(180i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ADAPTIVETESS_Y: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(181i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ADAPTIVETESS_Z: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(182i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ADAPTIVETESS_W: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(183i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_ENABLEADAPTIVETESSELLATION: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(184i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_TWOSIDEDSTENCILMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(185i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_CCW_STENCILFAIL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(186i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_CCW_STENCILZFAIL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(187i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_CCW_STENCILPASS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(188i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_CCW_STENCILFUNC: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(189i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_COLORWRITEENABLE1: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(190i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_COLORWRITEENABLE2: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(191i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_COLORWRITEENABLE3: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(192i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_BLENDFACTOR: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(193i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SRGBWRITEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(194i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_DEPTHBIAS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(195i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP8: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(198i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP9: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(199i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP10: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(200i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP11: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(201i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP12: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(202i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP13: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(203i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP14: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(204i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_WRAP15: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(205i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SEPARATEALPHABLENDENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(206i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_SRCBLENDALPHA: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(207i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_DESTBLENDALPHA: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(208i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_BLENDOPALPHA: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(209i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRS_FORCE_DWORD: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(2147483647i32);
impl ::core::marker::Copy for D3DRENDERSTATETYPE {}
impl ::core::clone::Clone for D3DRENDERSTATETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DRENDERSTATETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DRENDERSTATETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DRENDERSTATETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DRENDERSTATETYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DRESOURCESTATS {
    pub bThrashing: super::super::Foundation::BOOL,
    pub ApproxBytesDownloaded: u32,
    pub NumEvicts: u32,
    pub NumVidCreates: u32,
    pub LastPri: u32,
    pub NumUsed: u32,
    pub NumUsedInVidMem: u32,
    pub WorkingSet: u32,
    pub WorkingSetBytes: u32,
    pub TotalManaged: u32,
    pub TotalBytes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DRESOURCESTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DRESOURCESTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DRESOURCESTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRESOURCESTATS")
            .field("bThrashing", &self.bThrashing)
            .field("ApproxBytesDownloaded", &self.ApproxBytesDownloaded)
            .field("NumEvicts", &self.NumEvicts)
            .field("NumVidCreates", &self.NumVidCreates)
            .field("LastPri", &self.LastPri)
            .field("NumUsed", &self.NumUsed)
            .field("NumUsedInVidMem", &self.NumUsedInVidMem)
            .field("WorkingSet", &self.WorkingSet)
            .field("WorkingSetBytes", &self.WorkingSetBytes)
            .field("TotalManaged", &self.TotalManaged)
            .field("TotalBytes", &self.TotalBytes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3DRESOURCESTATS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DRESOURCESTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DRESOURCESTATS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DRESOURCESTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DRESOURCESTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DRESOURCETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPE_SURFACE: D3DRESOURCETYPE = D3DRESOURCETYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPE_VOLUME: D3DRESOURCETYPE = D3DRESOURCETYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPE_TEXTURE: D3DRESOURCETYPE = D3DRESOURCETYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPE_VOLUMETEXTURE: D3DRESOURCETYPE = D3DRESOURCETYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPE_CUBETEXTURE: D3DRESOURCETYPE = D3DRESOURCETYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPE_VERTEXBUFFER: D3DRESOURCETYPE = D3DRESOURCETYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPE_INDEXBUFFER: D3DRESOURCETYPE = D3DRESOURCETYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPE_FORCE_DWORD: D3DRESOURCETYPE = D3DRESOURCETYPE(2147483647i32);
impl ::core::marker::Copy for D3DRESOURCETYPE {}
impl ::core::clone::Clone for D3DRESOURCETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DRESOURCETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DRESOURCETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DRESOURCETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DRESOURCETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DRTYPECOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSAMPLERSTATETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_ADDRESSU: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_ADDRESSV: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_ADDRESSW: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_BORDERCOLOR: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_MAGFILTER: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_MINFILTER: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_MIPFILTER: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_MIPMAPLODBIAS: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_MAXMIPLEVEL: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_MAXANISOTROPY: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_SRGBTEXTURE: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_ELEMENTINDEX: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_DMAPOFFSET: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSAMP_FORCE_DWORD: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(2147483647i32);
impl ::core::marker::Copy for D3DSAMPLERSTATETYPE {}
impl ::core::clone::Clone for D3DSAMPLERSTATETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSAMPLERSTATETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSAMPLERSTATETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSAMPLERSTATETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSAMPLERSTATETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSAMPLER_TEXTURE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTT_UNKNOWN: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTT_2D: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(268435456i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTT_CUBE: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(402653184i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTT_VOLUME: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(536870912i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTT_FORCE_DWORD: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(2147483647i32);
impl ::core::marker::Copy for D3DSAMPLER_TEXTURE_TYPE {}
impl ::core::clone::Clone for D3DSAMPLER_TEXTURE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSAMPLER_TEXTURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSAMPLER_TEXTURE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSAMPLER_TEXTURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSAMPLER_TEXTURE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSCANLINEORDERING(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSCANLINEORDERING_UNKNOWN: D3DSCANLINEORDERING = D3DSCANLINEORDERING(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSCANLINEORDERING_PROGRESSIVE: D3DSCANLINEORDERING = D3DSCANLINEORDERING(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSCANLINEORDERING_INTERLACED: D3DSCANLINEORDERING = D3DSCANLINEORDERING(2i32);
impl ::core::marker::Copy for D3DSCANLINEORDERING {}
impl ::core::clone::Clone for D3DSCANLINEORDERING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSCANLINEORDERING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSCANLINEORDERING {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSCANLINEORDERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSCANLINEORDERING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSGR_CALIBRATE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSGR_NO_CALIBRATION: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSHADEMODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADE_FLAT: D3DSHADEMODE = D3DSHADEMODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADE_GOURAUD: D3DSHADEMODE = D3DSHADEMODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADE_PHONG: D3DSHADEMODE = D3DSHADEMODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADE_FORCE_DWORD: D3DSHADEMODE = D3DSHADEMODE(2147483647i32);
impl ::core::marker::Copy for D3DSHADEMODE {}
impl ::core::clone::Clone for D3DSHADEMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSHADEMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSHADEMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSHADEMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADEMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADER_ADDRESSMODE_SHIFT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSHADER_ADDRESSMODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADER_ADDRMODE_ABSOLUTE: D3DSHADER_ADDRESSMODE_TYPE = D3DSHADER_ADDRESSMODE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADER_ADDRMODE_RELATIVE: D3DSHADER_ADDRESSMODE_TYPE = D3DSHADER_ADDRESSMODE_TYPE(8192i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADER_ADDRMODE_FORCE_DWORD: D3DSHADER_ADDRESSMODE_TYPE = D3DSHADER_ADDRESSMODE_TYPE(2147483647i32);
impl ::core::marker::Copy for D3DSHADER_ADDRESSMODE_TYPE {}
impl ::core::clone::Clone for D3DSHADER_ADDRESSMODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSHADER_ADDRESSMODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSHADER_ADDRESSMODE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSHADER_ADDRESSMODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_ADDRESSMODE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSHADER_COMPARISON(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPC_RESERVED0: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPC_GT: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPC_EQ: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPC_GE: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPC_LT: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPC_NE: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPC_LE: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPC_RESERVED1: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(7i32);
impl ::core::marker::Copy for D3DSHADER_COMPARISON {}
impl ::core::clone::Clone for D3DSHADER_COMPARISON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSHADER_COMPARISON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSHADER_COMPARISON {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSHADER_COMPARISON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_COMPARISON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSHADER_COMPARISON_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSHADER_INSTRUCTION_OPCODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_NOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_MOV: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_ADD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_SUB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_MAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_MUL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_RCP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_RSQ: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DP3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DP4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_MIN: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_MAX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_SLT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_SGE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_EXP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_LOG: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_LIT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DST: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_LRP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_FRC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_M4x4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_M4x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_M3x4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_M3x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_M3x2: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_CALL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_CALLNZ: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_LOOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_RET: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_ENDLOOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_LABEL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DCL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_POW: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_CRS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_SGN: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_ABS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_NRM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_SINCOS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_REP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_ENDREP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_IF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_IFC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_ELSE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_ENDIF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_BREAK: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_BREAKC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_MOVA: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DEFB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DEFI: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXCOORD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXKILL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(65i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(66i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXBEM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(67i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXBEML: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(68i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXREG2AR: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(69i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXREG2GB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(70i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXM3x2PAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(71i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXM3x2TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(72i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXM3x3PAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(73i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXM3x3TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(74i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_RESERVED0: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(75i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXM3x3SPEC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(76i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXM3x3VSPEC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(77i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_EXPP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(78i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_LOGP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(79i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_CND: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(80i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DEF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(81i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXREG2RGB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(82i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXDP3TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(83i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXM3x2DEPTH: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(84i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXDP3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(85i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXM3x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(86i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXDEPTH: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(87i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_CMP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(88i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_BEM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(89i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DP2ADD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(90i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DSX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(91i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_DSY: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(92i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXLDD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(93i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_SETP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(94i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_TEXLDL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(95i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_BREAKP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(96i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_PHASE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(65533i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_COMMENT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(65534i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_END: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(65535i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSIO_FORCE_DWORD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(2147483647i32);
impl ::core::marker::Copy for D3DSHADER_INSTRUCTION_OPCODE_TYPE {}
impl ::core::clone::Clone for D3DSHADER_INSTRUCTION_OPCODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSHADER_INSTRUCTION_OPCODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSHADER_INSTRUCTION_OPCODE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSHADER_INSTRUCTION_OPCODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_INSTRUCTION_OPCODE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSHADER_MIN_PRECISION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMP_DEFAULT: D3DSHADER_MIN_PRECISION = D3DSHADER_MIN_PRECISION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMP_16: D3DSHADER_MIN_PRECISION = D3DSHADER_MIN_PRECISION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DMP_2_8: D3DSHADER_MIN_PRECISION = D3DSHADER_MIN_PRECISION(2i32);
impl ::core::marker::Copy for D3DSHADER_MIN_PRECISION {}
impl ::core::clone::Clone for D3DSHADER_MIN_PRECISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSHADER_MIN_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSHADER_MIN_PRECISION {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSHADER_MIN_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_MIN_PRECISION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSHADER_MISCTYPE_OFFSETS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSMO_POSITION: D3DSHADER_MISCTYPE_OFFSETS = D3DSHADER_MISCTYPE_OFFSETS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSMO_FACE: D3DSHADER_MISCTYPE_OFFSETS = D3DSHADER_MISCTYPE_OFFSETS(1i32);
impl ::core::marker::Copy for D3DSHADER_MISCTYPE_OFFSETS {}
impl ::core::clone::Clone for D3DSHADER_MISCTYPE_OFFSETS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSHADER_MISCTYPE_OFFSETS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSHADER_MISCTYPE_OFFSETS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSHADER_MISCTYPE_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_MISCTYPE_OFFSETS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSHADER_PARAM_REGISTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_TEMP: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_INPUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_CONST: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_ADDR: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_TEXTURE: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_RASTOUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_ATTROUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_TEXCRDOUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_OUTPUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_CONSTINT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_COLOROUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_DEPTHOUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_SAMPLER: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_CONST2: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_CONST3: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_CONST4: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_CONSTBOOL: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_LOOP: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_TEMPFLOAT16: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_MISCTYPE: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_LABEL: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_PREDICATE: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPR_FORCE_DWORD: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(2147483647i32);
impl ::core::marker::Copy for D3DSHADER_PARAM_REGISTER_TYPE {}
impl ::core::clone::Clone for D3DSHADER_PARAM_REGISTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSHADER_PARAM_REGISTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSHADER_PARAM_REGISTER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSHADER_PARAM_REGISTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_PARAM_REGISTER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSHADER_PARAM_SRCMOD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_NONE: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_NEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(16777216i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_BIAS: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(33554432i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_BIASNEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(50331648i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_SIGN: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(67108864i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_SIGNNEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(83886080i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_COMP: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(100663296i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_X2: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(117440512i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_X2NEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(134217728i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_DZ: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(150994944i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_DW: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(167772160i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_ABS: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(184549376i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_ABSNEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(201326592i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_NOT: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(218103808i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPSM_FORCE_DWORD: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(2147483647i32);
impl ::core::marker::Copy for D3DSHADER_PARAM_SRCMOD_TYPE {}
impl ::core::clone::Clone for D3DSHADER_PARAM_SRCMOD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSHADER_PARAM_SRCMOD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSHADER_PARAM_SRCMOD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSHADER_PARAM_SRCMOD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_PARAM_SRCMOD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSI_COISSUE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSI_COMMENTSIZE_MASK: u32 = 2147418112u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSI_COMMENTSIZE_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSI_INSTLENGTH_MASK: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSI_INSTLENGTH_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSI_OPCODE_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSPD_IUNKNOWN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_DCL_USAGEINDEX_MASK: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_DCL_USAGEINDEX_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_DCL_USAGE_MASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_DCL_USAGE_SHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_DSTMOD_MASK: u32 = 15728640u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_DSTMOD_SHIFT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_DSTSHIFT_MASK: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_DSTSHIFT_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_MIN_PRECISION_MASK: u32 = 49152u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_MIN_PRECISION_SHIFT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_OPCODESPECIFICCONTROL_MASK: u32 = 16711680u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_OPCODESPECIFICCONTROL_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_REGNUM_MASK: u32 = 2047u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_REGTYPE_MASK: u32 = 1879048192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_REGTYPE_MASK2: u32 = 6144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_REGTYPE_SHIFT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_REGTYPE_SHIFT2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_SRCMOD_MASK: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_SRCMOD_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_SWIZZLE_MASK: u32 = 16711680u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_SWIZZLE_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_TEXTURETYPE_MASK: u32 = 2013265920u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_TEXTURETYPE_SHIFT: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_WRITEMASK_0: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_WRITEMASK_1: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_WRITEMASK_2: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_WRITEMASK_3: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSP_WRITEMASK_ALL: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSTATEBLOCKTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSBT_ALL: D3DSTATEBLOCKTYPE = D3DSTATEBLOCKTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSBT_PIXELSTATE: D3DSTATEBLOCKTYPE = D3DSTATEBLOCKTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSBT_VERTEXSTATE: D3DSTATEBLOCKTYPE = D3DSTATEBLOCKTYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSBT_FORCE_DWORD: D3DSTATEBLOCKTYPE = D3DSTATEBLOCKTYPE(2147483647i32);
impl ::core::marker::Copy for D3DSTATEBLOCKTYPE {}
impl ::core::clone::Clone for D3DSTATEBLOCKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSTATEBLOCKTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSTATEBLOCKTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSTATEBLOCKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSTATEBLOCKTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILCAPS_TWOSIDED: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSTENCILOP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_KEEP: D3DSTENCILOP = D3DSTENCILOP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_ZERO: D3DSTENCILOP = D3DSTENCILOP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_REPLACE: D3DSTENCILOP = D3DSTENCILOP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_INCRSAT: D3DSTENCILOP = D3DSTENCILOP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_DECRSAT: D3DSTENCILOP = D3DSTENCILOP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_INVERT: D3DSTENCILOP = D3DSTENCILOP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_INCR: D3DSTENCILOP = D3DSTENCILOP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_DECR: D3DSTENCILOP = D3DSTENCILOP(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTENCILOP_FORCE_DWORD: D3DSTENCILOP = D3DSTENCILOP(2147483647u32);
impl ::core::marker::Copy for D3DSTENCILOP {}
impl ::core::clone::Clone for D3DSTENCILOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSTENCILOP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSTENCILOP {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSTENCILOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSTENCILOP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTREAMSOURCE_INDEXEDDATA: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSTREAMSOURCE_INSTANCEDATA: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DSURFACE_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: u32,
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for D3DSURFACE_DESC {}
impl ::core::clone::Clone for D3DSURFACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DSURFACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DSURFACE_DESC").field("Format", &self.Format).field("Type", &self.Type).field("Usage", &self.Usage).field("Pool", &self.Pool).field("MultiSampleType", &self.MultiSampleType).field("MultiSampleQuality", &self.MultiSampleQuality).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DSURFACE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DSURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DSURFACE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DSURFACE_DESC {}
impl ::core::default::Default for D3DSURFACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DSWAPEFFECT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSWAPEFFECT_DISCARD: D3DSWAPEFFECT = D3DSWAPEFFECT(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSWAPEFFECT_FLIP: D3DSWAPEFFECT = D3DSWAPEFFECT(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSWAPEFFECT_COPY: D3DSWAPEFFECT = D3DSWAPEFFECT(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSWAPEFFECT_OVERLAY: D3DSWAPEFFECT = D3DSWAPEFFECT(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSWAPEFFECT_FLIPEX: D3DSWAPEFFECT = D3DSWAPEFFECT(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSWAPEFFECT_FORCE_DWORD: D3DSWAPEFFECT = D3DSWAPEFFECT(2147483647u32);
impl ::core::marker::Copy for D3DSWAPEFFECT {}
impl ::core::clone::Clone for D3DSWAPEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DSWAPEFFECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DSWAPEFFECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DSWAPEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSWAPEFFECT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTA_CONSTANT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTA_TEMP: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXOPCAPS_LERP: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXOPCAPS_MULTIPLYADD: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DTEXTUREADDRESS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTADDRESS_WRAP: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTADDRESS_MIRROR: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTADDRESS_CLAMP: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTADDRESS_BORDER: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTADDRESS_MIRRORONCE: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTADDRESS_FORCE_DWORD: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(2147483647i32);
impl ::core::marker::Copy for D3DTEXTUREADDRESS {}
impl ::core::clone::Clone for D3DTEXTUREADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DTEXTUREADDRESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DTEXTUREADDRESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DTEXTUREADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTUREADDRESS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DTEXTUREFILTERTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXF_NONE: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXF_POINT: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXF_LINEAR: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXF_ANISOTROPIC: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXF_PYRAMIDALQUAD: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXF_GAUSSIANQUAD: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXF_CONVOLUTIONMONO: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTEXF_FORCE_DWORD: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(2147483647i32);
impl ::core::marker::Copy for D3DTEXTUREFILTERTYPE {}
impl ::core::clone::Clone for D3DTEXTUREFILTERTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DTEXTUREFILTERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DTEXTUREFILTERTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DTEXTUREFILTERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTUREFILTERTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DTEXTUREOP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_DISABLE: D3DTEXTUREOP = D3DTEXTUREOP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_SELECTARG1: D3DTEXTUREOP = D3DTEXTUREOP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_SELECTARG2: D3DTEXTUREOP = D3DTEXTUREOP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_MODULATE: D3DTEXTUREOP = D3DTEXTUREOP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_MODULATE2X: D3DTEXTUREOP = D3DTEXTUREOP(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_MODULATE4X: D3DTEXTUREOP = D3DTEXTUREOP(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_ADD: D3DTEXTUREOP = D3DTEXTUREOP(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_ADDSIGNED: D3DTEXTUREOP = D3DTEXTUREOP(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_ADDSIGNED2X: D3DTEXTUREOP = D3DTEXTUREOP(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_SUBTRACT: D3DTEXTUREOP = D3DTEXTUREOP(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_ADDSMOOTH: D3DTEXTUREOP = D3DTEXTUREOP(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_BLENDDIFFUSEALPHA: D3DTEXTUREOP = D3DTEXTUREOP(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_BLENDTEXTUREALPHA: D3DTEXTUREOP = D3DTEXTUREOP(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_BLENDFACTORALPHA: D3DTEXTUREOP = D3DTEXTUREOP(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_BLENDTEXTUREALPHAPM: D3DTEXTUREOP = D3DTEXTUREOP(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_BLENDCURRENTALPHA: D3DTEXTUREOP = D3DTEXTUREOP(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_PREMODULATE: D3DTEXTUREOP = D3DTEXTUREOP(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_MODULATEALPHA_ADDCOLOR: D3DTEXTUREOP = D3DTEXTUREOP(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_MODULATECOLOR_ADDALPHA: D3DTEXTUREOP = D3DTEXTUREOP(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_MODULATEINVALPHA_ADDCOLOR: D3DTEXTUREOP = D3DTEXTUREOP(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_MODULATEINVCOLOR_ADDALPHA: D3DTEXTUREOP = D3DTEXTUREOP(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_BUMPENVMAP: D3DTEXTUREOP = D3DTEXTUREOP(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_BUMPENVMAPLUMINANCE: D3DTEXTUREOP = D3DTEXTUREOP(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_DOTPRODUCT3: D3DTEXTUREOP = D3DTEXTUREOP(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_MULTIPLYADD: D3DTEXTUREOP = D3DTEXTUREOP(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_LERP: D3DTEXTUREOP = D3DTEXTUREOP(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTOP_FORCE_DWORD: D3DTEXTUREOP = D3DTEXTUREOP(2147483647i32);
impl ::core::marker::Copy for D3DTEXTUREOP {}
impl ::core::clone::Clone for D3DTEXTUREOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DTEXTUREOP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DTEXTUREOP {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DTEXTUREOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTUREOP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DTEXTURESTAGESTATETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_COLOROP: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_COLORARG1: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_COLORARG2: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_ALPHAOP: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_ALPHAARG1: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_ALPHAARG2: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_BUMPENVMAT00: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_BUMPENVMAT01: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_BUMPENVMAT10: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_BUMPENVMAT11: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_TEXCOORDINDEX: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_BUMPENVLSCALE: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_BUMPENVLOFFSET: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_TEXTURETRANSFORMFLAGS: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_COLORARG0: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_ALPHAARG0: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_RESULTARG: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_CONSTANT: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_FORCE_DWORD: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(2147483647i32);
impl ::core::marker::Copy for D3DTEXTURESTAGESTATETYPE {}
impl ::core::clone::Clone for D3DTEXTURESTAGESTATETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DTEXTURESTAGESTATETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DTEXTURESTAGESTATETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DTEXTURESTAGESTATETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTURESTAGESTATETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DTEXTURETRANSFORMFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTTFF_DISABLE: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTTFF_COUNT1: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTTFF_COUNT2: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTTFF_COUNT3: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTTFF_COUNT4: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTTFF_PROJECTED: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTTFF_FORCE_DWORD: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(2147483647i32);
impl ::core::marker::Copy for D3DTEXTURETRANSFORMFLAGS {}
impl ::core::clone::Clone for D3DTEXTURETRANSFORMFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DTEXTURETRANSFORMFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DTEXTURETRANSFORMFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DTEXTURETRANSFORMFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTURETRANSFORMFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DTRANSFORMSTATETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_VIEW: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_PROJECTION: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_TEXTURE0: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_TEXTURE1: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_TEXTURE2: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_TEXTURE3: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_TEXTURE4: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_TEXTURE5: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_TEXTURE6: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_TEXTURE7: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTS_FORCE_DWORD: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(2147483647i32);
impl ::core::marker::Copy for D3DTRANSFORMSTATETYPE {}
impl ::core::clone::Clone for D3DTRANSFORMSTATETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DTRANSFORMSTATETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DTRANSFORMSTATETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DTRANSFORMSTATETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTRANSFORMSTATETYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DTRIPATCH_INFO {
    pub StartVertexOffset: u32,
    pub NumVertices: u32,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
impl ::core::marker::Copy for D3DTRIPATCH_INFO {}
impl ::core::clone::Clone for D3DTRIPATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DTRIPATCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DTRIPATCH_INFO").field("StartVertexOffset", &self.StartVertexOffset).field("NumVertices", &self.NumVertices).field("Basis", &self.Basis).field("Degree", &self.Degree).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DTRIPATCH_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DTRIPATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DTRIPATCH_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DTRIPATCH_INFO {}
impl ::core::default::Default for D3DTRIPATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DTSS_TCI_SPHEREMAP: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_AUTOGENMIPMAP: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_DEPTHSTENCIL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_DMAP: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_DONOTCLIP: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_DYNAMIC: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_NONSECURE: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_NPATCHES: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_POINTS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_QUERY_FILTER: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_QUERY_LEGACYBUMPMAP: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_QUERY_SRGBREAD: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_QUERY_SRGBWRITE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_QUERY_VERTEXTEXTURE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_QUERY_WRAPANDMIP: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_RENDERTARGET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_RESTRICTED_CONTENT: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_RTPATCHES: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_SOFTWAREPROCESSING: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_TEXTAPI: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DUSAGE_WRITEONLY: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DVERTEXBLENDFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVBF_DISABLE: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVBF_1WEIGHTS: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVBF_2WEIGHTS: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVBF_3WEIGHTS: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVBF_TWEENING: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(255i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVBF_0WEIGHTS: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVBF_FORCE_DWORD: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(2147483647i32);
impl ::core::marker::Copy for D3DVERTEXBLENDFLAGS {}
impl ::core::clone::Clone for D3DVERTEXBLENDFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DVERTEXBLENDFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DVERTEXBLENDFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DVERTEXBLENDFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DVERTEXBLENDFLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DVERTEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Size: u32,
    pub FVF: u32,
}
impl ::core::marker::Copy for D3DVERTEXBUFFER_DESC {}
impl ::core::clone::Clone for D3DVERTEXBUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DVERTEXBUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVERTEXBUFFER_DESC").field("Format", &self.Format).field("Type", &self.Type).field("Usage", &self.Usage).field("Pool", &self.Pool).field("Size", &self.Size).field("FVF", &self.FVF).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DVERTEXBUFFER_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DVERTEXBUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DVERTEXBUFFER_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DVERTEXBUFFER_DESC {}
impl ::core::default::Default for D3DVERTEXBUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DVERTEXELEMENT9 {
    pub Stream: u16,
    pub Offset: u16,
    pub Type: u8,
    pub Method: u8,
    pub Usage: u8,
    pub UsageIndex: u8,
}
impl ::core::marker::Copy for D3DVERTEXELEMENT9 {}
impl ::core::clone::Clone for D3DVERTEXELEMENT9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DVERTEXELEMENT9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVERTEXELEMENT9").field("Stream", &self.Stream).field("Offset", &self.Offset).field("Type", &self.Type).field("Method", &self.Method).field("Usage", &self.Usage).field("UsageIndex", &self.UsageIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DVERTEXELEMENT9 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DVERTEXELEMENT9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DVERTEXELEMENT9>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DVERTEXELEMENT9 {}
impl ::core::default::Default for D3DVERTEXELEMENT9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVERTEXTEXTURESAMPLER0: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVERTEXTEXTURESAMPLER1: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVERTEXTEXTURESAMPLER2: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVERTEXTEXTURESAMPLER3: u32 = 260u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DVIEWPORT9 {
    pub X: u32,
    pub Y: u32,
    pub Width: u32,
    pub Height: u32,
    pub MinZ: f32,
    pub MaxZ: f32,
}
impl ::core::marker::Copy for D3DVIEWPORT9 {}
impl ::core::clone::Clone for D3DVIEWPORT9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DVIEWPORT9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVIEWPORT9").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).field("MinZ", &self.MinZ).field("MaxZ", &self.MaxZ).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DVIEWPORT9 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DVIEWPORT9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DVIEWPORT9>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DVIEWPORT9 {}
impl ::core::default::Default for D3DVIEWPORT9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DVOLUME_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
impl ::core::marker::Copy for D3DVOLUME_DESC {}
impl ::core::clone::Clone for D3DVOLUME_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DVOLUME_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVOLUME_DESC").field("Format", &self.Format).field("Type", &self.Type).field("Usage", &self.Usage).field("Pool", &self.Pool).field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DVOLUME_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DVOLUME_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DVOLUME_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DVOLUME_DESC {}
impl ::core::default::Default for D3DVOLUME_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS20CAPS_PREDICATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS20_MAX_NUMTEMPS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS20_MIN_NUMTEMPS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3DVSHADERCAPS2_0 {
    pub Caps: u32,
    pub DynamicFlowControlDepth: i32,
    pub NumTemps: i32,
    pub StaticFlowControlDepth: i32,
}
impl ::core::marker::Copy for D3DVSHADERCAPS2_0 {}
impl ::core::clone::Clone for D3DVSHADERCAPS2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3DVSHADERCAPS2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVSHADERCAPS2_0").field("Caps", &self.Caps).field("DynamicFlowControlDepth", &self.DynamicFlowControlDepth).field("NumTemps", &self.NumTemps).field("StaticFlowControlDepth", &self.StaticFlowControlDepth).finish()
    }
}
unsafe impl ::windows::core::Abi for D3DVSHADERCAPS2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3DVSHADERCAPS2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3DVSHADERCAPS2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3DVSHADERCAPS2_0 {}
impl ::core::default::Default for D3DVSHADERCAPS2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS_ADDRESSMODE_SHIFT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DVS_ADDRESSMODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS_ADDRMODE_ABSOLUTE: D3DVS_ADDRESSMODE_TYPE = D3DVS_ADDRESSMODE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS_ADDRMODE_RELATIVE: D3DVS_ADDRESSMODE_TYPE = D3DVS_ADDRESSMODE_TYPE(8192i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS_ADDRMODE_FORCE_DWORD: D3DVS_ADDRESSMODE_TYPE = D3DVS_ADDRESSMODE_TYPE(2147483647i32);
impl ::core::marker::Copy for D3DVS_ADDRESSMODE_TYPE {}
impl ::core::clone::Clone for D3DVS_ADDRESSMODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DVS_ADDRESSMODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DVS_ADDRESSMODE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DVS_ADDRESSMODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DVS_ADDRESSMODE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DVS_RASTOUT_OFFSETS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSRO_POSITION: D3DVS_RASTOUT_OFFSETS = D3DVS_RASTOUT_OFFSETS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSRO_FOG: D3DVS_RASTOUT_OFFSETS = D3DVS_RASTOUT_OFFSETS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSRO_POINT_SIZE: D3DVS_RASTOUT_OFFSETS = D3DVS_RASTOUT_OFFSETS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DSRO_FORCE_DWORD: D3DVS_RASTOUT_OFFSETS = D3DVS_RASTOUT_OFFSETS(2147483647i32);
impl ::core::marker::Copy for D3DVS_RASTOUT_OFFSETS {}
impl ::core::clone::Clone for D3DVS_RASTOUT_OFFSETS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DVS_RASTOUT_OFFSETS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DVS_RASTOUT_OFFSETS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DVS_RASTOUT_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DVS_RASTOUT_OFFSETS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS_SWIZZLE_MASK: u32 = 16711680u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVS_SWIZZLE_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVTXPCAPS_NO_TEXGEN_NONLOCALVIEWER: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVTXPCAPS_TEXGEN_SPHEREMAP: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DVTXPCAPS_TWEENING: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DWRAP_W: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct D3DZBUFFERTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DZB_FALSE: D3DZBUFFERTYPE = D3DZBUFFERTYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DZB_TRUE: D3DZBUFFERTYPE = D3DZBUFFERTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DZB_USEW: D3DZBUFFERTYPE = D3DZBUFFERTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3DZB_FORCE_DWORD: D3DZBUFFERTYPE = D3DZBUFFERTYPE(2147483647i32);
impl ::core::marker::Copy for D3DZBUFFERTYPE {}
impl ::core::clone::Clone for D3DZBUFFERTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DZBUFFERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DZBUFFERTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DZBUFFERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DZBUFFERTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D_MAX_SIMULTANEOUS_RENDERTARGETS: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub struct D3D_OMAC {
    pub Omac: [u8; 16],
}
impl ::core::marker::Copy for D3D_OMAC {}
impl ::core::clone::Clone for D3D_OMAC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D_OMAC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D_OMAC").field("Omac", &self.Omac).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D_OMAC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D_OMAC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D_OMAC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D_OMAC {}
impl ::core::default::Default for D3D_OMAC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D_OMAC_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const D3D_SDK_VERSION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[inline]
pub unsafe fn Direct3DCreate9(sdkversion: u32) -> ::core::option::Option<IDirect3D9> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9(sdkversion: u32) -> ::core::option::Option<IDirect3D9>;
        }
        ::core::mem::transmute(Direct3DCreate9(::core::mem::transmute(sdkversion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[inline]
pub unsafe fn Direct3DCreate9Ex(sdkversion: u32) -> ::windows::core::Result<IDirect3D9Ex> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9Ex(sdkversion: u32, param1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        Direct3DCreate9Ex(::core::mem::transmute(sdkversion), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3D9Ex>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3D9(::windows::core::IUnknown);
impl IDirect3D9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn RegisterSoftwareDevice(&self, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterSoftwareDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinitializefunction)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetAdapterCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAdapterIdentifier)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(flags), ::core::mem::transmute(pidentifier)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetAdapterModeCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(format)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EnumAdapterModes(&self, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAdapterModes)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(format), ::core::mem::transmute(mode), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAdapterDisplayMode(&self, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAdapterDisplayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceType<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckDeviceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devtype), ::core::mem::transmute(adapterformat), ::core::mem::transmute(backbufferformat), bwindowed.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CheckDeviceFormat(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckDeviceFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(adapterformat), ::core::mem::transmute(usage), ::core::mem::transmute(rtype), ::core::mem::transmute(checkformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceMultiSampleType<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: Param3, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckDeviceMultiSampleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(surfaceformat), windowed.into_param().abi(), ::core::mem::transmute(multisampletype), ::core::mem::transmute(pqualitylevels)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckDepthStencilMatch)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(adapterformat), ::core::mem::transmute(rendertargetformat), ::core::mem::transmute(depthstencilformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckDeviceFormatConversion)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(sourceformat), ::core::mem::transmute(targetformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDeviceCaps(&self, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceCaps)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(pcaps)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetAdapterMonitor(&self, adapter: u32) -> super::Gdi::HMONITOR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetAdapterMonitor)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDevice<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: Param2, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), hfocuswindow.into_param().abi(), ::core::mem::transmute(behaviorflags), ::core::mem::transmute(ppresentationparameters), ::core::mem::transmute(ppreturneddeviceinterface)).ok()
    }
}
impl ::core::convert::From<IDirect3D9> for ::windows::core::IUnknown {
    fn from(value: IDirect3D9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3D9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3D9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3D9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3D9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3D9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3D9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3D9 {}
impl ::core::fmt::Debug for IDirect3D9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3D9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3D9 {
    type Vtable = IDirect3D9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81bdcbca_64d4_426d_ae8d_ad0147f4275c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterSoftwareDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAdapterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAdapterIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAdapterIdentifier: usize,
    pub GetAdapterModeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT) -> u32,
    pub EnumAdapterModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT,
    pub GetAdapterDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckDeviceType: usize,
    pub CheckDeviceFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckDeviceMultiSampleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckDeviceMultiSampleType: usize,
    pub CheckDepthStencilMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::HRESULT,
    pub CheckDeviceFormatConversion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::HRESULT,
    pub GetDeviceCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetAdapterMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32) -> super::Gdi::HMONITOR,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetAdapterMonitor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDevice: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3D9Ex(::windows::core::IUnknown);
impl IDirect3D9Ex {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn RegisterSoftwareDevice(&self, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RegisterSoftwareDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinitializefunction)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetAdapterCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetAdapterIdentifier)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(flags), ::core::mem::transmute(pidentifier)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetAdapterModeCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(format)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EnumAdapterModes(&self, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EnumAdapterModes)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(format), ::core::mem::transmute(mode), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAdapterDisplayMode(&self, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetAdapterDisplayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceType<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CheckDeviceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devtype), ::core::mem::transmute(adapterformat), ::core::mem::transmute(backbufferformat), bwindowed.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CheckDeviceFormat(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CheckDeviceFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(adapterformat), ::core::mem::transmute(usage), ::core::mem::transmute(rtype), ::core::mem::transmute(checkformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceMultiSampleType<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: Param3, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CheckDeviceMultiSampleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(surfaceformat), windowed.into_param().abi(), ::core::mem::transmute(multisampletype), ::core::mem::transmute(pqualitylevels)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CheckDepthStencilMatch)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(adapterformat), ::core::mem::transmute(rendertargetformat), ::core::mem::transmute(depthstencilformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CheckDeviceFormatConversion)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(sourceformat), ::core::mem::transmute(targetformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDeviceCaps(&self, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDeviceCaps)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), ::core::mem::transmute(pcaps)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetAdapterMonitor(&self, adapter: u32) -> super::Gdi::HMONITOR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetAdapterMonitor)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDevice<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: Param2, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), hfocuswindow.into_param().abi(), ::core::mem::transmute(behaviorflags), ::core::mem::transmute(ppresentationparameters), ::core::mem::transmute(ppreturneddeviceinterface)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAdapterModeCountEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetAdapterModeCountEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(pfilter)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EnumAdapterModesEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAdapterModesEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(pfilter), ::core::mem::transmute(mode), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAdapterDisplayModeEx(&self, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAdapterDisplayModeEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(pmode), ::core::mem::transmute(protation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDeviceEx<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: Param2, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9Ex>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateDeviceEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(devicetype), hfocuswindow.into_param().abi(), ::core::mem::transmute(behaviorflags), ::core::mem::transmute(ppresentationparameters), ::core::mem::transmute(pfullscreendisplaymode), ::core::mem::transmute(ppreturneddeviceinterface)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLUID(&self, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAdapterLUID)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(pluid)).ok()
    }
}
impl ::core::convert::From<IDirect3D9Ex> for ::windows::core::IUnknown {
    fn from(value: IDirect3D9Ex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3D9Ex> for ::windows::core::IUnknown {
    fn from(value: &IDirect3D9Ex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3D9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3D9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3D9Ex> for IDirect3D9 {
    fn from(value: IDirect3D9Ex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3D9Ex> for IDirect3D9 {
    fn from(value: &IDirect3D9Ex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3D9> for IDirect3D9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3D9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3D9> for &'a IDirect3D9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3D9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3D9Ex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3D9Ex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3D9Ex {}
impl ::core::fmt::Debug for IDirect3D9Ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3D9Ex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3D9Ex {
    type Vtable = IDirect3D9Ex_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02177241_69fc_400c_8ff1_93a44df6861d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9Ex_Vtbl {
    pub base: IDirect3D9_Vtbl,
    pub GetAdapterModeCountEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32,
    pub EnumAdapterModesEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT,
    pub GetAdapterDisplayModeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDeviceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDeviceEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAdapterLUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAdapterLUID: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DBaseTexture9(::windows::core::IUnknown);
impl IDirect3DBaseTexture9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(prioritynew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetPriority)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Interface::vtable(self).base.PreLoad)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).SetLOD)(::core::mem::transmute_copy(self), ::core::mem::transmute(lodnew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLOD(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetLOD)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLevelCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetLevelCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAutoGenFilterType)(::core::mem::transmute_copy(self), ::core::mem::transmute(filtertype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetAutoGenFilterType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GenerateMipSubLevels(&self) {
        (::windows::core::Interface::vtable(self).GenerateMipSubLevels)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IDirect3DBaseTexture9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DBaseTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DBaseTexture9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DBaseTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DBaseTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DBaseTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DBaseTexture9> for IDirect3DResource9 {
    fn from(value: IDirect3DBaseTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DBaseTexture9> for IDirect3DResource9 {
    fn from(value: &IDirect3DBaseTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for IDirect3DBaseTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for &'a IDirect3DBaseTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DBaseTexture9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DBaseTexture9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DBaseTexture9 {}
impl ::core::fmt::Debug for IDirect3DBaseTexture9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DBaseTexture9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DBaseTexture9 {
    type Vtable = IDirect3DBaseTexture9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x580ca87e_1d3c_4d54_991d_b7d3e3c298ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DBaseTexture9_Vtbl {
    pub base: IDirect3DResource9_Vtbl,
    pub SetLOD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lodnew: u32) -> u32,
    pub GetLOD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub SetAutoGenFilterType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT,
    pub GetAutoGenFilterType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3DTEXTUREFILTERTYPE,
    pub GenerateMipSubLevels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DCubeTexture9(::windows::core::IUnknown);
impl IDirect3DCubeTexture9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(prioritynew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetPriority)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Interface::vtable(self).base.base.PreLoad)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.SetLOD)(::core::mem::transmute_copy(self), ::core::mem::transmute(lodnew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLOD(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetLOD)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLevelCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetLevelCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAutoGenFilterType)(::core::mem::transmute_copy(self), ::core::mem::transmute(filtertype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetAutoGenFilterType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GenerateMipSubLevels(&self) {
        (::windows::core::Interface::vtable(self).base.GenerateMipSubLevels)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLevelDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetCubeMapSurface(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCubeMapSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(facetype), ::core::mem::transmute(level), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(facetype), ::core::mem::transmute(level), ::core::mem::transmute(plockedrect), ::core::mem::transmute(prect), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn UnlockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(facetype), ::core::mem::transmute(level)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirtyRect(&self, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDirtyRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(facetype), ::core::mem::transmute(pdirtyrect)).ok()
    }
}
impl ::core::convert::From<IDirect3DCubeTexture9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DCubeTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DCubeTexture9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DCubeTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DCubeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DCubeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DCubeTexture9> for IDirect3DResource9 {
    fn from(value: IDirect3DCubeTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DCubeTexture9> for IDirect3DResource9 {
    fn from(value: &IDirect3DCubeTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for IDirect3DCubeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for &'a IDirect3DCubeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DCubeTexture9> for IDirect3DBaseTexture9 {
    fn from(value: IDirect3DCubeTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DCubeTexture9> for IDirect3DBaseTexture9 {
    fn from(value: &IDirect3DCubeTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DBaseTexture9> for IDirect3DCubeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DBaseTexture9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DBaseTexture9> for &'a IDirect3DCubeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DBaseTexture9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DCubeTexture9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DCubeTexture9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DCubeTexture9 {}
impl ::core::fmt::Debug for IDirect3DCubeTexture9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DCubeTexture9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DCubeTexture9 {
    type Vtable = IDirect3DCubeTexture9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfff32f81_d953_473a_9223_93d652aba93f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DCubeTexture9_Vtbl {
    pub base: IDirect3DBaseTexture9_Vtbl,
    pub GetLevelDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT,
    pub GetCubeMapSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LockRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockRect: usize,
    pub UnlockRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDirtyRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDirtyRect: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DDevice9(::windows::core::IUnknown);
impl IDirect3DDevice9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn TestCooperativeLevel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TestCooperativeLevel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAvailableTextureMem(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetAvailableTextureMem)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EvictManagedResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EvictManagedResources)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDirect3D(&self) -> ::windows::core::Result<IDirect3D9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDirect3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3D9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceCaps)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcaps)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDisplayMode(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCreationParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(pparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetCursorProperties<'a, Param2: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCursorProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(xhotspot), ::core::mem::transmute(yhotspot), pcursorbitmap.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetCursorPosition(&self, x: i32, y: i32, flags: u32) {
        (::windows::core::Interface::vtable(self).SetCursorPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(flags))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowCursor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).ShowCursor)(::core::mem::transmute_copy(self), bshow.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::core::option::Option<IDirect3DSwapChain9>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateAdditionalSwapChain)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresentationparameters), ::core::mem::transmute(pswapchain)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetSwapChain(&self, iswapchain: u32) -> ::windows::core::Result<IDirect3DSwapChain9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSwapChain)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSwapChain9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetNumberOfSwapChains(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNumberOfSwapChains)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresentationparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Present<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: Param2, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(psourcerect), ::core::mem::transmute(pdestrect), hdestwindowoverride.into_param().abi(), ::core::mem::transmute(pdirtyregion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBackBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(ibackbuffer), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRasterStatus(&self, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRasterStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(prasterstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDialogBoxMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benabledialogs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDialogBoxMode)(::core::mem::transmute_copy(self), benabledialogs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
        (::windows::core::Interface::vtable(self).SetGammaRamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(flags), ::core::mem::transmute(pramp))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
        (::windows::core::Interface::vtable(self).GetGammaRamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(pramp))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::core::option::Option<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(levels), ::core::mem::transmute(usage), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(pptexture), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::core::option::Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateVolumeTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(depth), ::core::mem::transmute(levels), ::core::mem::transmute(usage), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppvolumetexture), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::core::option::Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateCubeTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(edgelength), ::core::mem::transmute(levels), ::core::mem::transmute(usage), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppcubetexture), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::core::option::Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateVertexBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(length), ::core::mem::transmute(usage), ::core::mem::transmute(fvf), ::core::mem::transmute(pool), ::core::mem::transmute(ppvertexbuffer), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateIndexBuffer(&self, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::core::option::Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateIndexBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(length), ::core::mem::transmute(usage), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppindexbuffer), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRenderTarget<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: Param5, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateRenderTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(multisample), ::core::mem::transmute(multisamplequality), lockable.into_param().abi(), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilSurface<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: Param5, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateDepthStencilSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(multisample), ::core::mem::transmute(multisamplequality), discard.into_param().abi(), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateSurface<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>, Param2: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, psourcesurface: Param0, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: Param2, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateSurface)(::core::mem::transmute_copy(self), psourcesurface.into_param().abi(), ::core::mem::transmute(psourcerect), pdestinationsurface.into_param().abi(), ::core::mem::transmute(pdestpoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn UpdateTexture<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DBaseTexture9>, Param1: ::windows::core::IntoParam<'a, IDirect3DBaseTexture9>>(&self, psourcetexture: Param0, pdestinationtexture: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateTexture)(::core::mem::transmute_copy(self), psourcetexture.into_param().abi(), pdestinationtexture.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetRenderTargetData<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>, Param1: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, prendertarget: Param0, pdestsurface: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRenderTargetData)(::core::mem::transmute_copy(self), prendertarget.into_param().abi(), pdestsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetFrontBufferData<'a, Param1: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, iswapchain: u32, pdestsurface: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrontBufferData)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), pdestsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StretchRect<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>, Param2: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, psourcesurface: Param0, psourcerect: *const super::super::Foundation::RECT, pdestsurface: Param2, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StretchRect)(::core::mem::transmute_copy(self), psourcesurface.into_param().abi(), ::core::mem::transmute(psourcerect), pdestsurface.into_param().abi(), ::core::mem::transmute(pdestrect), ::core::mem::transmute(filter)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ColorFill<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, psurface: Param0, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ColorFill)(::core::mem::transmute_copy(self), psurface.into_param().abi(), ::core::mem::transmute(prect), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateOffscreenPlainSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetRenderTarget<'a, Param1: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, rendertargetindex: u32, prendertarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRenderTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(rendertargetindex), prendertarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetRenderTarget(&self, rendertargetindex: u32) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRenderTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(rendertargetindex), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetDepthStencilSurface<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, pnewzstencil: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDepthStencilSurface)(::core::mem::transmute_copy(self), pnewzstencil.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDepthStencilSurface(&self) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDepthStencilSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn BeginScene(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginScene)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EndScene(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndScene)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Clear(&self, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(prects), ::core::mem::transmute(flags), ::core::mem::transmute(color), ::core::mem::transmute(z), ::core::mem::transmute(stencil)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MultiplyTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0), ::core::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(pviewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(pviewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaterial)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmaterial)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMaterial)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmaterial)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLight)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLight)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LightEnable<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, index: u32, enable: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LightEnable)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), enable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLightEnable(&self, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLightEnable)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(penable)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetClipPlane(&self, index: u32, pplane: *const f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClipPlane)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pplane)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClipPlane)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pplane)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRenderState)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRenderState)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE) -> ::windows::core::Result<IDirect3DStateBlock9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStateBlock)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DStateBlock9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn BeginStateBlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginStateBlock)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EndStateBlock(&self) -> ::windows::core::Result<IDirect3DStateBlock9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EndStateBlock)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DStateBlock9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClipStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclipstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClipStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclipstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetTexture(&self, stage: u32) -> ::windows::core::Result<IDirect3DBaseTexture9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(stage), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DBaseTexture9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetTexture<'a, Param1: ::windows::core::IntoParam<'a, IDirect3DBaseTexture9>>(&self, stage: u32, ptexture: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(stage), ptexture.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTextureStageState)(::core::mem::transmute_copy(self), ::core::mem::transmute(stage), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTextureStageState)(::core::mem::transmute_copy(self), ::core::mem::transmute(stage), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSamplerState)(::core::mem::transmute_copy(self), ::core::mem::transmute(sampler), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSamplerState)(::core::mem::transmute_copy(self), ::core::mem::transmute(sampler), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn ValidateDevice(&self, pnumpasses: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ValidateDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnumpasses)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPaletteEntries)(::core::mem::transmute_copy(self), ::core::mem::transmute(palettenumber), ::core::mem::transmute(pentries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetPaletteEntries(&self, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPaletteEntries)(::core::mem::transmute_copy(self), ::core::mem::transmute(palettenumber), ::core::mem::transmute(pentries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetCurrentTexturePalette(&self, palettenumber: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCurrentTexturePalette)(::core::mem::transmute_copy(self), ::core::mem::transmute(palettenumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentTexturePalette)(::core::mem::transmute_copy(self), ::core::mem::transmute(palettenumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetScissorRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScissorRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScissorRect(&self, prect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScissorRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoftwareVertexProcessing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bsoftware: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSoftwareVertexProcessing)(::core::mem::transmute_copy(self), bsoftware.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSoftwareVertexProcessing(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetSoftwareVertexProcessing)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetNPatchMode(&self, nsegments: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNPatchMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(nsegments)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetNPatchMode(&self) -> f32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNPatchMode)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DrawPrimitive)(::core::mem::transmute_copy(self), ::core::mem::transmute(primitivetype), ::core::mem::transmute(startvertex), ::core::mem::transmute(primitivecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DrawIndexedPrimitive)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0), ::core::mem::transmute(basevertexindex), ::core::mem::transmute(minvertexindex), ::core::mem::transmute(numvertices), ::core::mem::transmute(startindex), ::core::mem::transmute(primcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DrawPrimitiveUP)(::core::mem::transmute_copy(self), ::core::mem::transmute(primitivetype), ::core::mem::transmute(primitivecount), ::core::mem::transmute(pvertexstreamzerodata), ::core::mem::transmute(vertexstreamzerostride)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DrawIndexedPrimitiveUP)(::core::mem::transmute_copy(self), ::core::mem::transmute(primitivetype), ::core::mem::transmute(minvertexindex), ::core::mem::transmute(numvertices), ::core::mem::transmute(primitivecount), ::core::mem::transmute(pindexdata), ::core::mem::transmute(indexdataformat), ::core::mem::transmute(pvertexstreamzerodata), ::core::mem::transmute(vertexstreamzerostride)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn ProcessVertices<'a, Param3: ::windows::core::IntoParam<'a, IDirect3DVertexBuffer9>, Param4: ::windows::core::IntoParam<'a, IDirect3DVertexDeclaration9>>(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: Param3, pvertexdecl: Param4, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessVertices)(::core::mem::transmute_copy(self), ::core::mem::transmute(srcstartindex), ::core::mem::transmute(destindex), ::core::mem::transmute(vertexcount), pdestbuffer.into_param().abi(), pvertexdecl.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9) -> ::windows::core::Result<IDirect3DVertexDeclaration9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVertexDeclaration)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvertexelements), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVertexDeclaration9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetVertexDeclaration<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DVertexDeclaration9>>(&self, pdecl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVertexDeclaration)(::core::mem::transmute_copy(self), pdecl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVertexDeclaration(&self) -> ::windows::core::Result<IDirect3DVertexDeclaration9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVertexDeclaration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVertexDeclaration9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetFVF(&self, fvf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFVF)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvf)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetFVF(&self, pfvf: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFVF)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfvf)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreateVertexShader(&self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DVertexShader9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVertexShader)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfunction), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVertexShader9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetVertexShader<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DVertexShader9>>(&self, pshader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVertexShader)(::core::mem::transmute_copy(self), pshader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVertexShader(&self) -> ::windows::core::Result<IDirect3DVertexShader9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVertexShader)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVertexShader9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVertexShaderConstantF)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4fcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVertexShaderConstantF)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4fcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVertexShaderConstantI)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4icount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVertexShaderConstantI)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4icount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVertexShaderConstantB)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(boolcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVertexShaderConstantB)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(boolcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetStreamSource<'a, Param1: ::windows::core::IntoParam<'a, IDirect3DVertexBuffer9>>(&self, streamnumber: u32, pstreamdata: Param1, offsetinbytes: u32, stride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamnumber), pstreamdata.into_param().abi(), ::core::mem::transmute(offsetinbytes), ::core::mem::transmute(stride)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: *mut ::core::option::Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamnumber), ::core::mem::transmute(ppstreamdata), ::core::mem::transmute(poffsetinbytes), ::core::mem::transmute(pstride)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamSourceFreq)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamnumber), ::core::mem::transmute(setting)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetStreamSourceFreq(&self, streamnumber: u32, psetting: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamSourceFreq)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamnumber), ::core::mem::transmute(psetting)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetIndices<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DIndexBuffer9>>(&self, pindexdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIndices)(::core::mem::transmute_copy(self), pindexdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetIndices(&self) -> ::windows::core::Result<IDirect3DIndexBuffer9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIndices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DIndexBuffer9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreatePixelShader(&self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DPixelShader9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePixelShader)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfunction), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DPixelShader9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPixelShader<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DPixelShader9>>(&self, pshader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPixelShader)(::core::mem::transmute_copy(self), pshader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPixelShader(&self) -> ::windows::core::Result<IDirect3DPixelShader9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPixelShader)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DPixelShader9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPixelShaderConstantF)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4fcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelShaderConstantF)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4fcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPixelShaderConstantI)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4icount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelShaderConstantI)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4icount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPixelShaderConstantB)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(boolcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelShaderConstantB)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(boolcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DrawRectPatch)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(pnumsegs), ::core::mem::transmute(prectpatchinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DrawTriPatch)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(pnumsegs), ::core::mem::transmute(ptripatchinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DeletePatch(&self, handle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePatch)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreateQuery(&self, r#type: D3DQUERYTYPE) -> ::windows::core::Result<IDirect3DQuery9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DQuery9>(result__)
    }
}
impl ::core::convert::From<IDirect3DDevice9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DDevice9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDevice9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DDevice9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DDevice9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DDevice9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DDevice9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice9 {}
impl ::core::fmt::Debug for IDirect3DDevice9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DDevice9 {
    type Vtable = IDirect3DDevice9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0223b96_bf7a_43fd_92bd_a43b0d82b9eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub TestCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAvailableTextureMem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub EvictManagedResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDirect3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppd3d9: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDeviceCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcaps: *mut D3DCAPS9) -> ::windows::core::HRESULT,
    pub GetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCreationParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCreationParameters: usize,
    pub SetCursorProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCursorPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, flags: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub ShowCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowCursor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateAdditionalSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateAdditionalSwapChain: usize,
    pub GetSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32, pswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumberOfSwapChains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Reset: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Present: usize,
    pub GetBackBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRasterStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRasterStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDialogBoxMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabledialogs: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDialogBoxMode: usize,
    pub SetGammaRamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP),
    pub GetGammaRamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32, pramp: *mut D3DGAMMARAMP),
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTexture: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVolumeTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVolumeTexture: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateCubeTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateCubeTexture: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVertexBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVertexBuffer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateIndexBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateIndexBuffer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRenderTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDepthStencilSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDepthStencilSurface: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcesurface: ::windows::core::RawPtr, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: ::windows::core::RawPtr, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateSurface: usize,
    pub UpdateTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcetexture: ::windows::core::RawPtr, pdestinationtexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRenderTargetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFrontBufferData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StretchRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcesurface: ::windows::core::RawPtr, psourcerect: *const super::super::Foundation::RECT, pdestsurface: ::windows::core::RawPtr, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StretchRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ColorFill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psurface: ::windows::core::RawPtr, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ColorFill: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateOffscreenPlainSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateOffscreenPlainSurface: usize,
    pub SetRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendertargetindex: u32, prendertarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDepthStencilSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewzstencil: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDepthStencilSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppzstencilsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BeginScene: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndScene: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetTransform: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetTransform: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub MultiplyTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    MultiplyTransform: usize,
    pub SetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> ::windows::core::HRESULT,
    pub GetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> ::windows::core::HRESULT,
    pub GetMaterial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetLight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetLight: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetLight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetLight: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LightEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LightEnable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLightEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLightEnable: usize,
    pub SetClipPlane: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pplane: *const f32) -> ::windows::core::HRESULT,
    pub GetClipPlane: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pplane: *mut f32) -> ::windows::core::HRESULT,
    pub SetRenderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::HRESULT,
    pub GetRenderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT,
    pub CreateStateBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BeginStateBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndStateBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetClipStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::HRESULT,
    pub GetClipStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::HRESULT,
    pub GetTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stage: u32, pptexture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stage: u32, ptexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTextureStageState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT,
    pub SetTextureStageState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::HRESULT,
    pub GetSamplerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::HRESULT,
    pub SetSamplerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::HRESULT,
    pub ValidateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumpasses: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetPaletteEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetPaletteEntries: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetPaletteEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetPaletteEntries: usize,
    pub SetCurrentTexturePalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palettenumber: u32) -> ::windows::core::HRESULT,
    pub GetCurrentTexturePalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palettenumber: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetScissorRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetScissorRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScissorRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScissorRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoftwareVertexProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsoftware: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoftwareVertexProcessing: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSoftwareVertexProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSoftwareVertexProcessing: usize,
    pub SetNPatchMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nsegments: f32) -> ::windows::core::HRESULT,
    pub GetNPatchMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub DrawPrimitive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::HRESULT,
    pub DrawIndexedPrimitive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::HRESULT,
    pub DrawPrimitiveUP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT,
    pub DrawIndexedPrimitiveUP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::HRESULT,
    pub ProcessVertices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: ::windows::core::RawPtr, pvertexdecl: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub CreateVertexDeclaration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetVertexDeclaration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdecl: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetVertexDeclaration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdecl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetFVF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvf: u32) -> ::windows::core::HRESULT,
    pub GetFVF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvf: *mut u32) -> ::windows::core::HRESULT,
    pub CreateVertexShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetVertexShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetVertexShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetVertexShaderConstantF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT,
    pub GetVertexShaderConstantF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT,
    pub SetVertexShaderConstantI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT,
    pub GetVertexShaderConstantI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVertexShaderConstantB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVertexShaderConstantB: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVertexShaderConstantB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVertexShaderConstantB: usize,
    pub SetStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamnumber: u32, pstreamdata: ::windows::core::RawPtr, offsetinbytes: u32, stride: u32) -> ::windows::core::HRESULT,
    pub GetStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut ::windows::core::RawPtr, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::HRESULT,
    pub SetStreamSourceFreq: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamnumber: u32, setting: u32) -> ::windows::core::HRESULT,
    pub GetStreamSourceFreq: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> ::windows::core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindexdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppindexdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPixelShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPixelShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPixelShaderConstantF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::HRESULT,
    pub GetPixelShaderConstantF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::HRESULT,
    pub SetPixelShaderConstantI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::HRESULT,
    pub GetPixelShaderConstantI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPixelShaderConstantB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPixelShaderConstantB: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPixelShaderConstantB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPixelShaderConstantB: usize,
    pub DrawRectPatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::HRESULT,
    pub DrawTriPatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::HRESULT,
    pub DeletePatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: u32) -> ::windows::core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DDevice9Ex(::windows::core::IUnknown);
impl IDirect3DDevice9Ex {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn TestCooperativeLevel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.TestCooperativeLevel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAvailableTextureMem(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetAvailableTextureMem)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EvictManagedResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EvictManagedResources)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDirect3D(&self) -> ::windows::core::Result<IDirect3D9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDirect3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3D9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDeviceCaps)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcaps)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDisplayMode(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDisplayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCreationParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(pparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetCursorProperties<'a, Param2: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCursorProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(xhotspot), ::core::mem::transmute(yhotspot), pcursorbitmap.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetCursorPosition(&self, x: i32, y: i32, flags: u32) {
        (::windows::core::Interface::vtable(self).base.SetCursorPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(flags))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowCursor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.ShowCursor)(::core::mem::transmute_copy(self), bshow.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::core::option::Option<IDirect3DSwapChain9>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateAdditionalSwapChain)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresentationparameters), ::core::mem::transmute(pswapchain)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetSwapChain(&self, iswapchain: u32) -> ::windows::core::Result<IDirect3DSwapChain9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSwapChain)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSwapChain9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetNumberOfSwapChains(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetNumberOfSwapChains)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Reset)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresentationparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Present<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: Param2, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(psourcerect), ::core::mem::transmute(pdestrect), hdestwindowoverride.into_param().abi(), ::core::mem::transmute(pdirtyregion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBackBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(ibackbuffer), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRasterStatus(&self, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetRasterStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(prasterstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDialogBoxMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benabledialogs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDialogBoxMode)(::core::mem::transmute_copy(self), benabledialogs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
        (::windows::core::Interface::vtable(self).base.SetGammaRamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(flags), ::core::mem::transmute(pramp))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
        (::windows::core::Interface::vtable(self).base.GetGammaRamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(pramp))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::core::option::Option<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(levels), ::core::mem::transmute(usage), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(pptexture), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::core::option::Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateVolumeTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(depth), ::core::mem::transmute(levels), ::core::mem::transmute(usage), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppvolumetexture), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::core::option::Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateCubeTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(edgelength), ::core::mem::transmute(levels), ::core::mem::transmute(usage), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppcubetexture), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::core::option::Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateVertexBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(length), ::core::mem::transmute(usage), ::core::mem::transmute(fvf), ::core::mem::transmute(pool), ::core::mem::transmute(ppvertexbuffer), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateIndexBuffer(&self, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::core::option::Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateIndexBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(length), ::core::mem::transmute(usage), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppindexbuffer), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRenderTarget<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: Param5, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateRenderTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(multisample), ::core::mem::transmute(multisamplequality), lockable.into_param().abi(), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilSurface<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: Param5, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateDepthStencilSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(multisample), ::core::mem::transmute(multisamplequality), discard.into_param().abi(), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateSurface<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>, Param2: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, psourcesurface: Param0, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: Param2, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.UpdateSurface)(::core::mem::transmute_copy(self), psourcesurface.into_param().abi(), ::core::mem::transmute(psourcerect), pdestinationsurface.into_param().abi(), ::core::mem::transmute(pdestpoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn UpdateTexture<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DBaseTexture9>, Param1: ::windows::core::IntoParam<'a, IDirect3DBaseTexture9>>(&self, psourcetexture: Param0, pdestinationtexture: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.UpdateTexture)(::core::mem::transmute_copy(self), psourcetexture.into_param().abi(), pdestinationtexture.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetRenderTargetData<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>, Param1: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, prendertarget: Param0, pdestsurface: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetRenderTargetData)(::core::mem::transmute_copy(self), prendertarget.into_param().abi(), pdestsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetFrontBufferData<'a, Param1: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, iswapchain: u32, pdestsurface: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetFrontBufferData)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), pdestsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StretchRect<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>, Param2: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, psourcesurface: Param0, psourcerect: *const super::super::Foundation::RECT, pdestsurface: Param2, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StretchRect)(::core::mem::transmute_copy(self), psourcesurface.into_param().abi(), ::core::mem::transmute(psourcerect), pdestsurface.into_param().abi(), ::core::mem::transmute(pdestrect), ::core::mem::transmute(filter)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ColorFill<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, psurface: Param0, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ColorFill)(::core::mem::transmute_copy(self), psurface.into_param().abi(), ::core::mem::transmute(prect), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateOffscreenPlainSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetRenderTarget<'a, Param1: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, rendertargetindex: u32, prendertarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRenderTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(rendertargetindex), prendertarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetRenderTarget(&self, rendertargetindex: u32) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRenderTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(rendertargetindex), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetDepthStencilSurface<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, pnewzstencil: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDepthStencilSurface)(::core::mem::transmute_copy(self), pnewzstencil.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDepthStencilSurface(&self) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDepthStencilSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn BeginScene(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginScene)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EndScene(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EndScene)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Clear(&self, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Clear)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(prects), ::core::mem::transmute(flags), ::core::mem::transmute(color), ::core::mem::transmute(z), ::core::mem::transmute(stencil)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.MultiplyTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0), ::core::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(pviewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetViewport)(::core::mem::transmute_copy(self), ::core::mem::transmute(pviewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMaterial)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmaterial)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetMaterial)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmaterial)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLight)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetLight)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LightEnable<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, index: u32, enable: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.LightEnable)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), enable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLightEnable(&self, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetLightEnable)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(penable)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetClipPlane(&self, index: u32, pplane: *const f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClipPlane)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pplane)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetClipPlane)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pplane)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRenderState)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetRenderState)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE) -> ::windows::core::Result<IDirect3DStateBlock9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateStateBlock)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DStateBlock9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn BeginStateBlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginStateBlock)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn EndStateBlock(&self) -> ::windows::core::Result<IDirect3DStateBlock9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EndStateBlock)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DStateBlock9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClipStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclipstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetClipStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclipstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetTexture(&self, stage: u32) -> ::windows::core::Result<IDirect3DBaseTexture9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(stage), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DBaseTexture9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetTexture<'a, Param1: ::windows::core::IntoParam<'a, IDirect3DBaseTexture9>>(&self, stage: u32, ptexture: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTexture)(::core::mem::transmute_copy(self), ::core::mem::transmute(stage), ptexture.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetTextureStageState)(::core::mem::transmute_copy(self), ::core::mem::transmute(stage), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTextureStageState)(::core::mem::transmute_copy(self), ::core::mem::transmute(stage), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSamplerState)(::core::mem::transmute_copy(self), ::core::mem::transmute(sampler), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSamplerState)(::core::mem::transmute_copy(self), ::core::mem::transmute(sampler), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn ValidateDevice(&self, pnumpasses: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ValidateDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnumpasses)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPaletteEntries)(::core::mem::transmute_copy(self), ::core::mem::transmute(palettenumber), ::core::mem::transmute(pentries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetPaletteEntries(&self, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPaletteEntries)(::core::mem::transmute_copy(self), ::core::mem::transmute(palettenumber), ::core::mem::transmute(pentries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetCurrentTexturePalette(&self, palettenumber: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCurrentTexturePalette)(::core::mem::transmute_copy(self), ::core::mem::transmute(palettenumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCurrentTexturePalette)(::core::mem::transmute_copy(self), ::core::mem::transmute(palettenumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetScissorRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetScissorRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScissorRect(&self, prect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetScissorRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoftwareVertexProcessing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bsoftware: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSoftwareVertexProcessing)(::core::mem::transmute_copy(self), bsoftware.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSoftwareVertexProcessing(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetSoftwareVertexProcessing)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetNPatchMode(&self, nsegments: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetNPatchMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(nsegments)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetNPatchMode(&self) -> f32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetNPatchMode)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DrawPrimitive)(::core::mem::transmute_copy(self), ::core::mem::transmute(primitivetype), ::core::mem::transmute(startvertex), ::core::mem::transmute(primitivecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DrawIndexedPrimitive)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0), ::core::mem::transmute(basevertexindex), ::core::mem::transmute(minvertexindex), ::core::mem::transmute(numvertices), ::core::mem::transmute(startindex), ::core::mem::transmute(primcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DrawPrimitiveUP)(::core::mem::transmute_copy(self), ::core::mem::transmute(primitivetype), ::core::mem::transmute(primitivecount), ::core::mem::transmute(pvertexstreamzerodata), ::core::mem::transmute(vertexstreamzerostride)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DrawIndexedPrimitiveUP)(::core::mem::transmute_copy(self), ::core::mem::transmute(primitivetype), ::core::mem::transmute(minvertexindex), ::core::mem::transmute(numvertices), ::core::mem::transmute(primitivecount), ::core::mem::transmute(pindexdata), ::core::mem::transmute(indexdataformat), ::core::mem::transmute(pvertexstreamzerodata), ::core::mem::transmute(vertexstreamzerostride)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn ProcessVertices<'a, Param3: ::windows::core::IntoParam<'a, IDirect3DVertexBuffer9>, Param4: ::windows::core::IntoParam<'a, IDirect3DVertexDeclaration9>>(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: Param3, pvertexdecl: Param4, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ProcessVertices)(::core::mem::transmute_copy(self), ::core::mem::transmute(srcstartindex), ::core::mem::transmute(destindex), ::core::mem::transmute(vertexcount), pdestbuffer.into_param().abi(), pvertexdecl.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9) -> ::windows::core::Result<IDirect3DVertexDeclaration9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateVertexDeclaration)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvertexelements), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVertexDeclaration9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetVertexDeclaration<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DVertexDeclaration9>>(&self, pdecl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetVertexDeclaration)(::core::mem::transmute_copy(self), pdecl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVertexDeclaration(&self) -> ::windows::core::Result<IDirect3DVertexDeclaration9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetVertexDeclaration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVertexDeclaration9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetFVF(&self, fvf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFVF)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvf)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetFVF(&self, pfvf: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetFVF)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfvf)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreateVertexShader(&self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DVertexShader9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateVertexShader)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfunction), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVertexShader9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetVertexShader<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DVertexShader9>>(&self, pshader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetVertexShader)(::core::mem::transmute_copy(self), pshader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVertexShader(&self) -> ::windows::core::Result<IDirect3DVertexShader9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetVertexShader)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVertexShader9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetVertexShaderConstantF)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4fcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetVertexShaderConstantF)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4fcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetVertexShaderConstantI)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4icount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetVertexShaderConstantI)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4icount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetVertexShaderConstantB)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(boolcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetVertexShaderConstantB)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(boolcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetStreamSource<'a, Param1: ::windows::core::IntoParam<'a, IDirect3DVertexBuffer9>>(&self, streamnumber: u32, pstreamdata: Param1, offsetinbytes: u32, stride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetStreamSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamnumber), pstreamdata.into_param().abi(), ::core::mem::transmute(offsetinbytes), ::core::mem::transmute(stride)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: *mut ::core::option::Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetStreamSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamnumber), ::core::mem::transmute(ppstreamdata), ::core::mem::transmute(poffsetinbytes), ::core::mem::transmute(pstride)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetStreamSourceFreq)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamnumber), ::core::mem::transmute(setting)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetStreamSourceFreq(&self, streamnumber: u32, psetting: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetStreamSourceFreq)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamnumber), ::core::mem::transmute(psetting)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetIndices<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DIndexBuffer9>>(&self, pindexdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIndices)(::core::mem::transmute_copy(self), pindexdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetIndices(&self) -> ::windows::core::Result<IDirect3DIndexBuffer9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetIndices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DIndexBuffer9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreatePixelShader(&self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DPixelShader9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePixelShader)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfunction), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DPixelShader9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPixelShader<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DPixelShader9>>(&self, pshader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPixelShader)(::core::mem::transmute_copy(self), pshader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPixelShader(&self) -> ::windows::core::Result<IDirect3DPixelShader9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPixelShader)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DPixelShader9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPixelShaderConstantF)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4fcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPixelShaderConstantF)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4fcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPixelShaderConstantI)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4icount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPixelShaderConstantI)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(vector4icount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPixelShaderConstantB)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(boolcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPixelShaderConstantB)(::core::mem::transmute_copy(self), ::core::mem::transmute(startregister), ::core::mem::transmute(pconstantdata), ::core::mem::transmute(boolcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DrawRectPatch)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(pnumsegs), ::core::mem::transmute(prectpatchinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DrawTriPatch)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(pnumsegs), ::core::mem::transmute(ptripatchinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn DeletePatch(&self, handle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DeletePatch)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CreateQuery(&self, r#type: D3DQUERYTYPE) -> ::windows::core::Result<IDirect3DQuery9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DQuery9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetConvolutionMonoKernel(&self, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConvolutionMonoKernel)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(rows), ::core::mem::transmute(columns)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn ComposeRects<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>, Param1: ::windows::core::IntoParam<'a, IDirect3DSurface9>, Param2: ::windows::core::IntoParam<'a, IDirect3DVertexBuffer9>, Param4: ::windows::core::IntoParam<'a, IDirect3DVertexBuffer9>>(&self, psrc: Param0, pdst: Param1, psrcrectdescs: Param2, numrects: u32, pdstrectdescs: Param4, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ComposeRects)(::core::mem::transmute_copy(self), psrc.into_param().abi(), pdst.into_param().abi(), psrcrectdescs.into_param().abi(), ::core::mem::transmute(numrects), pdstrectdescs.into_param().abi(), ::core::mem::transmute(operation), ::core::mem::transmute(xoffset), ::core::mem::transmute(yoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn PresentEx<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: Param2, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PresentEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(psourcerect), ::core::mem::transmute(pdestrect), hdestwindowoverride.into_param().abi(), ::core::mem::transmute(pdirtyregion), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn WaitForVBlank(&self, iswapchain: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForVBlank)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn CheckResourceResidency(&self, presourcearray: *mut ::core::option::Option<IDirect3DResource9>, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckResourceResidency)(::core::mem::transmute_copy(self), ::core::mem::transmute(presourcearray), ::core::mem::transmute(numresources)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmaxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hdestinationwindow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckDeviceState)(::core::mem::transmute_copy(self), hdestinationwindow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRenderTargetEx<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: Param5, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateRenderTargetEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(multisample), ::core::mem::transmute(multisamplequality), lockable.into_param().abi(), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle), ::core::mem::transmute(usage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOffscreenPlainSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateOffscreenPlainSurfaceEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(pool), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle), ::core::mem::transmute(usage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilSurfaceEx<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: Param5, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateDepthStencilSurfaceEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(multisample), ::core::mem::transmute(multisamplequality), discard.into_param().abi(), ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle), ::core::mem::transmute(usage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResetEx(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresentationparameters), ::core::mem::transmute(pfullscreendisplaymode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDisplayModeEx(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayModeEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(iswapchain), ::core::mem::transmute(pmode), ::core::mem::transmute(protation)).ok()
    }
}
impl ::core::convert::From<IDirect3DDevice9Ex> for ::windows::core::IUnknown {
    fn from(value: IDirect3DDevice9Ex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDevice9Ex> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DDevice9Ex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DDevice9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DDevice9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DDevice9Ex> for IDirect3DDevice9 {
    fn from(value: IDirect3DDevice9Ex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDevice9Ex> for IDirect3DDevice9 {
    fn from(value: &IDirect3DDevice9Ex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DDevice9> for IDirect3DDevice9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DDevice9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DDevice9> for &'a IDirect3DDevice9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DDevice9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DDevice9Ex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice9Ex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice9Ex {}
impl ::core::fmt::Debug for IDirect3DDevice9Ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice9Ex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DDevice9Ex {
    type Vtable = IDirect3DDevice9Ex_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb18b10ce_2649_405a_870f_95f777d4313a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9Ex_Vtbl {
    pub base: IDirect3DDevice9_Vtbl,
    pub SetConvolutionMonoKernel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows::core::HRESULT,
    pub ComposeRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr, psrcrectdescs: ::windows::core::RawPtr, numrects: u32, pdstrectdescs: ::windows::core::RawPtr, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub PresentEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    PresentEx: usize,
    pub GetGPUThreadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT,
    pub WaitForVBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32) -> ::windows::core::HRESULT,
    pub CheckResourceResidency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcearray: *mut ::windows::core::RawPtr, numresources: u32) -> ::windows::core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdestinationwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckDeviceState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRenderTargetEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRenderTargetEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateOffscreenPlainSurfaceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateOffscreenPlainSurfaceEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDepthStencilSurfaceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::windows::core::RawPtr, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDepthStencilSurfaceEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResetEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResetEx: usize,
    pub GetDisplayModeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DIndexBuffer9(::windows::core::IUnknown);
impl IDirect3DIndexBuffer9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(prioritynew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetPriority)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Interface::vtable(self).base.PreLoad)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lock)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsettolock), ::core::mem::transmute(sizetolock), ::core::mem::transmute(ppbdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
}
impl ::core::convert::From<IDirect3DIndexBuffer9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DIndexBuffer9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DIndexBuffer9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DIndexBuffer9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DIndexBuffer9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DIndexBuffer9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DIndexBuffer9> for IDirect3DResource9 {
    fn from(value: IDirect3DIndexBuffer9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DIndexBuffer9> for IDirect3DResource9 {
    fn from(value: &IDirect3DIndexBuffer9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for IDirect3DIndexBuffer9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for &'a IDirect3DIndexBuffer9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DIndexBuffer9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DIndexBuffer9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DIndexBuffer9 {}
impl ::core::fmt::Debug for IDirect3DIndexBuffer9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DIndexBuffer9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DIndexBuffer9 {
    type Vtable = IDirect3DIndexBuffer9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c9dd65e_d3f7_4529_acee_785830acde35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DIndexBuffer9_Vtbl {
    pub base: IDirect3DResource9_Vtbl,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DPixelShader9(::windows::core::IUnknown);
impl IDirect3DPixelShader9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetFunction(&self, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunction)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0), ::core::mem::transmute(psizeofdata)).ok()
    }
}
impl ::core::convert::From<IDirect3DPixelShader9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DPixelShader9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DPixelShader9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DPixelShader9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DPixelShader9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DPixelShader9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DPixelShader9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DPixelShader9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DPixelShader9 {}
impl ::core::fmt::Debug for IDirect3DPixelShader9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DPixelShader9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DPixelShader9 {
    type Vtable = IDirect3DPixelShader9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d3bdbdc_5b02_4415_b852_ce5e8bccb289);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DPixelShader9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DQuery9(::windows::core::IUnknown);
impl IDirect3DQuery9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DQUERYTYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDataSize(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetDataSize)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Issue(&self, dwissueflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Issue)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwissueflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwgetdataflags)).ok()
    }
}
impl ::core::convert::From<IDirect3DQuery9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DQuery9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DQuery9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DQuery9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DQuery9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DQuery9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DQuery9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DQuery9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DQuery9 {}
impl ::core::fmt::Debug for IDirect3DQuery9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DQuery9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DQuery9 {
    type Vtable = IDirect3DQuery9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9771460_a695_4f26_bbd3_27b840b541cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DQuery9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3DQUERYTYPE,
    pub GetDataSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Issue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwissueflags: u32) -> ::windows::core::HRESULT,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DResource9(::windows::core::IUnknown);
impl IDirect3DResource9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(prioritynew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetPriority)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Interface::vtable(self).PreLoad)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDirect3DResource9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DResource9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DResource9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DResource9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DResource9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DResource9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DResource9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DResource9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DResource9 {}
impl ::core::fmt::Debug for IDirect3DResource9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DResource9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DResource9 {
    type Vtable = IDirect3DResource9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05eec05d_8f7d_4362_b999_d1baf357c704);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DResource9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prioritynew: u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3DRESOURCETYPE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DStateBlock9(::windows::core::IUnknown);
impl IDirect3DStateBlock9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Capture(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Capture)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Apply)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDirect3DStateBlock9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DStateBlock9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DStateBlock9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DStateBlock9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DStateBlock9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DStateBlock9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DStateBlock9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DStateBlock9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DStateBlock9 {}
impl ::core::fmt::Debug for IDirect3DStateBlock9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DStateBlock9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DStateBlock9 {
    type Vtable = IDirect3DStateBlock9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb07c4fe5_310d_4ba8_a23c_4f0f206f218b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DStateBlock9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Capture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Apply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DSurface9(::windows::core::IUnknown);
impl IDirect3DSurface9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(prioritynew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetPriority)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Interface::vtable(self).base.PreLoad)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetContainer(&self, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContainer)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppcontainer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockRect(&self, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(plockedrect), ::core::mem::transmute(prect), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn UnlockRect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockRect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, phdc: *mut super::Gdi::HDC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(phdc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(&self, hdc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseDC)(::core::mem::transmute_copy(self), hdc.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDirect3DSurface9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DSurface9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DSurface9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DSurface9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DSurface9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DSurface9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DSurface9> for IDirect3DResource9 {
    fn from(value: IDirect3DSurface9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DSurface9> for IDirect3DResource9 {
    fn from(value: &IDirect3DSurface9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for IDirect3DSurface9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for &'a IDirect3DSurface9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DSurface9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DSurface9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSurface9 {}
impl ::core::fmt::Debug for IDirect3DSurface9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSurface9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DSurface9 {
    type Vtable = IDirect3DSurface9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cfbaf3a_9ff6_429a_99b3_a2796af8b89b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSurface9_Vtbl {
    pub base: IDirect3DResource9_Vtbl,
    pub GetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LockRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockRect: usize,
    pub UnlockRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DSwapChain9(::windows::core::IUnknown);
impl IDirect3DSwapChain9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Present<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: Param2, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(psourcerect), ::core::mem::transmute(pdestrect), hdestwindowoverride.into_param().abi(), ::core::mem::transmute(pdirtyregion), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetFrontBufferData<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, pdestsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrontBufferData)(::core::mem::transmute_copy(self), pdestsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetBackBuffer(&self, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBackBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(ibackbuffer), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRasterStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(prasterstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPresentParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresentationparameters)).ok()
    }
}
impl ::core::convert::From<IDirect3DSwapChain9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DSwapChain9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DSwapChain9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DSwapChain9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DSwapChain9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DSwapChain9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DSwapChain9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DSwapChain9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSwapChain9 {}
impl ::core::fmt::Debug for IDirect3DSwapChain9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSwapChain9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DSwapChain9 {
    type Vtable = IDirect3DSwapChain9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x794950f2_adfc_458a_905e_10a10b0b503b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSwapChain9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Present: usize,
    pub GetFrontBufferData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetBackBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRasterStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRasterStatus: usize,
    pub GetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPresentParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPresentParameters: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DSwapChain9Ex(::windows::core::IUnknown);
impl IDirect3DSwapChain9Ex {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Present<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: Param2, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(psourcerect), ::core::mem::transmute(pdestrect), hdestwindowoverride.into_param().abi(), ::core::mem::transmute(pdirtyregion), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetFrontBufferData<'a, Param0: ::windows::core::IntoParam<'a, IDirect3DSurface9>>(&self, pdestsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetFrontBufferData)(::core::mem::transmute_copy(self), pdestsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetBackBuffer(&self, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBackBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(ibackbuffer), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetRasterStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(prasterstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDisplayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPresentParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresentationparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLastPresentCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(plastpresentcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPresentStats(&self, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPresentStats)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresentationstatistics)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDisplayModeEx(&self, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayModeEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmode), ::core::mem::transmute(protation)).ok()
    }
}
impl ::core::convert::From<IDirect3DSwapChain9Ex> for ::windows::core::IUnknown {
    fn from(value: IDirect3DSwapChain9Ex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DSwapChain9Ex> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DSwapChain9Ex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DSwapChain9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DSwapChain9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DSwapChain9Ex> for IDirect3DSwapChain9 {
    fn from(value: IDirect3DSwapChain9Ex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DSwapChain9Ex> for IDirect3DSwapChain9 {
    fn from(value: &IDirect3DSwapChain9Ex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DSwapChain9> for IDirect3DSwapChain9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DSwapChain9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DSwapChain9> for &'a IDirect3DSwapChain9Ex {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DSwapChain9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DSwapChain9Ex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DSwapChain9Ex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSwapChain9Ex {}
impl ::core::fmt::Debug for IDirect3DSwapChain9Ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSwapChain9Ex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DSwapChain9Ex {
    type Vtable = IDirect3DSwapChain9Ex_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91886caf_1c3d_4d2e_a0ab_3e4c7d8d3303);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSwapChain9Ex_Vtbl {
    pub base: IDirect3DSwapChain9_Vtbl,
    pub GetLastPresentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetPresentStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows::core::HRESULT,
    pub GetDisplayModeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DTexture9(::windows::core::IUnknown);
impl IDirect3DTexture9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(prioritynew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetPriority)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Interface::vtable(self).base.base.PreLoad)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.SetLOD)(::core::mem::transmute_copy(self), ::core::mem::transmute(lodnew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLOD(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetLOD)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLevelCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetLevelCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAutoGenFilterType)(::core::mem::transmute_copy(self), ::core::mem::transmute(filtertype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetAutoGenFilterType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GenerateMipSubLevels(&self) {
        (::windows::core::Interface::vtable(self).base.GenerateMipSubLevels)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLevelDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetSurfaceLevel(&self, level: u32) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSurfaceLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DSurface9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockRect(&self, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), ::core::mem::transmute(plockedrect), ::core::mem::transmute(prect), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn UnlockRect(&self, level: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(level)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirtyRect(&self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDirtyRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdirtyrect)).ok()
    }
}
impl ::core::convert::From<IDirect3DTexture9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DTexture9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DTexture9> for IDirect3DResource9 {
    fn from(value: IDirect3DTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DTexture9> for IDirect3DResource9 {
    fn from(value: &IDirect3DTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for IDirect3DTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for &'a IDirect3DTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DTexture9> for IDirect3DBaseTexture9 {
    fn from(value: IDirect3DTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DTexture9> for IDirect3DBaseTexture9 {
    fn from(value: &IDirect3DTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DBaseTexture9> for IDirect3DTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DBaseTexture9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DBaseTexture9> for &'a IDirect3DTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DBaseTexture9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DTexture9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DTexture9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DTexture9 {}
impl ::core::fmt::Debug for IDirect3DTexture9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DTexture9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DTexture9 {
    type Vtable = IDirect3DTexture9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85c31227_3de5_4f00_9b3a_f11ac38c18b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DTexture9_Vtbl {
    pub base: IDirect3DBaseTexture9_Vtbl,
    pub GetLevelDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows::core::HRESULT,
    pub GetSurfaceLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, ppsurfacelevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LockRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockRect: usize,
    pub UnlockRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDirtyRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDirtyRect: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DVertexBuffer9(::windows::core::IUnknown);
impl IDirect3DVertexBuffer9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(prioritynew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetPriority)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Interface::vtable(self).base.PreLoad)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lock)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsettolock), ::core::mem::transmute(sizetolock), ::core::mem::transmute(ppbdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
}
impl ::core::convert::From<IDirect3DVertexBuffer9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DVertexBuffer9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DVertexBuffer9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DVertexBuffer9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DVertexBuffer9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DVertexBuffer9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DVertexBuffer9> for IDirect3DResource9 {
    fn from(value: IDirect3DVertexBuffer9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DVertexBuffer9> for IDirect3DResource9 {
    fn from(value: &IDirect3DVertexBuffer9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for IDirect3DVertexBuffer9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for &'a IDirect3DVertexBuffer9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DVertexBuffer9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DVertexBuffer9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVertexBuffer9 {}
impl ::core::fmt::Debug for IDirect3DVertexBuffer9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVertexBuffer9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DVertexBuffer9 {
    type Vtable = IDirect3DVertexBuffer9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb64bb1b5_fd70_4df6_bf91_19d0a12455e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexBuffer9_Vtbl {
    pub base: IDirect3DResource9_Vtbl,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DVertexDeclaration9(::windows::core::IUnknown);
impl IDirect3DVertexDeclaration9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDeclaration(&self, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeclaration)(::core::mem::transmute_copy(self), ::core::mem::transmute(pelement), ::core::mem::transmute(pnumelements)).ok()
    }
}
impl ::core::convert::From<IDirect3DVertexDeclaration9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DVertexDeclaration9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DVertexDeclaration9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DVertexDeclaration9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DVertexDeclaration9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DVertexDeclaration9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DVertexDeclaration9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DVertexDeclaration9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVertexDeclaration9 {}
impl ::core::fmt::Debug for IDirect3DVertexDeclaration9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVertexDeclaration9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DVertexDeclaration9 {
    type Vtable = IDirect3DVertexDeclaration9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd13c59c_36fa_4098_a8fb_c7ed39dc8546);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexDeclaration9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDeclaration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DVertexShader9(::windows::core::IUnknown);
impl IDirect3DVertexShader9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetFunction(&self, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunction)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0), ::core::mem::transmute(psizeofdata)).ok()
    }
}
impl ::core::convert::From<IDirect3DVertexShader9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DVertexShader9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DVertexShader9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DVertexShader9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DVertexShader9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DVertexShader9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DVertexShader9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DVertexShader9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVertexShader9 {}
impl ::core::fmt::Debug for IDirect3DVertexShader9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVertexShader9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DVertexShader9 {
    type Vtable = IDirect3DVertexShader9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefc5557e_6265_4613_8a94_43857889eb36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexShader9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DVolume9(::windows::core::IUnknown);
impl IDirect3DVolume9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetContainer(&self, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContainer)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppcontainer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn LockBox(&self, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(plockedvolume), ::core::mem::transmute(pbox), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn UnlockBox(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockBox)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDirect3DVolume9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DVolume9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DVolume9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DVolume9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DVolume9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DVolume9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DVolume9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DVolume9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVolume9 {}
impl ::core::fmt::Debug for IDirect3DVolume9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVolume9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DVolume9 {
    type Vtable = IDirect3DVolume9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24f416e6_1f67_4aa7_b88e_d33f6f3128a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVolume9_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT,
    pub LockBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT,
    pub UnlockBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
#[repr(transparent)]
pub struct IDirect3DVolumeTexture9(::windows::core::IUnknown);
impl IDirect3DVolumeTexture9 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DDevice9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(sizeofdata), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid), ::core::mem::transmute(pdata), ::core::mem::transmute(psizeofdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.FreePrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(prioritynew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetPriority)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Interface::vtable(self).base.base.PreLoad)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.SetLOD)(::core::mem::transmute_copy(self), ::core::mem::transmute(lodnew)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLOD(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetLOD)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLevelCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetLevelCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAutoGenFilterType)(::core::mem::transmute_copy(self), ::core::mem::transmute(filtertype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetAutoGenFilterType)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GenerateMipSubLevels(&self) {
        (::windows::core::Interface::vtable(self).base.GenerateMipSubLevels)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLevelDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn GetVolumeLevel(&self, level: u32) -> ::windows::core::Result<IDirect3DVolume9> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVolumeLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), ::core::mem::transmute(&mut result__)).from_abi::<IDirect3DVolume9>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn LockBox(&self, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), ::core::mem::transmute(plockedvolume), ::core::mem::transmute(pbox), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn UnlockBox(&self, level: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(level)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
    pub unsafe fn AddDirtyBox(&self, pdirtybox: *const D3DBOX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDirtyBox)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdirtybox)).ok()
    }
}
impl ::core::convert::From<IDirect3DVolumeTexture9> for ::windows::core::IUnknown {
    fn from(value: IDirect3DVolumeTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DVolumeTexture9> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DVolumeTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DVolumeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DVolumeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DVolumeTexture9> for IDirect3DResource9 {
    fn from(value: IDirect3DVolumeTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DVolumeTexture9> for IDirect3DResource9 {
    fn from(value: &IDirect3DVolumeTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for IDirect3DVolumeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DResource9> for &'a IDirect3DVolumeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DResource9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DVolumeTexture9> for IDirect3DBaseTexture9 {
    fn from(value: IDirect3DVolumeTexture9) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DVolumeTexture9> for IDirect3DBaseTexture9 {
    fn from(value: &IDirect3DVolumeTexture9) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DBaseTexture9> for IDirect3DVolumeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DBaseTexture9> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirect3DBaseTexture9> for &'a IDirect3DVolumeTexture9 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirect3DBaseTexture9> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DVolumeTexture9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DVolumeTexture9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVolumeTexture9 {}
impl ::core::fmt::Debug for IDirect3DVolumeTexture9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVolumeTexture9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DVolumeTexture9 {
    type Vtable = IDirect3DVolumeTexture9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2518526c_e789_4111_a7b9_47ef328d13e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVolumeTexture9_Vtbl {
    pub base: IDirect3DBaseTexture9_Vtbl,
    pub GetLevelDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows::core::HRESULT,
    pub GetVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, ppvolumelevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LockBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows::core::HRESULT,
    pub UnlockBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT,
    pub AddDirtyBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirtybox: *const D3DBOX) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const MAXD3DDECLLENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const MAXD3DDECLUSAGEINDEX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const MAX_DEVICE_IDENTIFIER_STRING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9\"`*"]
pub const _FACD3D: u32 = 2166u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
