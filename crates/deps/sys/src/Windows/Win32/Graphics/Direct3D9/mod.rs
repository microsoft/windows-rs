#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_BeginEvent(col: u32, wszname: super::super::Foundation::PWSTR) -> i32;
    pub fn D3DPERF_EndEvent() -> i32;
    pub fn D3DPERF_GetStatus() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_QueryRepeatFrame() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_SetMarker(col: u32, wszname: super::super::Foundation::PWSTR);
    pub fn D3DPERF_SetOptions(dwoptions: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_SetRegion(col: u32, wszname: super::super::Foundation::PWSTR);
    pub fn Direct3DCreate9(sdkversion: u32) -> IDirect3D9;
    pub fn Direct3DCreate9Ex(sdkversion: u32, param1: *mut IDirect3D9Ex) -> ::windows_sys::core::HRESULT;
}
pub const D3D9_RESOURCE_PRIORITY_HIGH: u32 = 2684354560u32;
pub const D3D9_RESOURCE_PRIORITY_LOW: u32 = 1342177280u32;
pub const D3D9_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200u32;
pub const D3D9_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640u32;
pub const D3D9_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920u32;
pub const D3D9b_SDK_VERSION: u32 = 31u32;
pub const D3DADAPTER_DEFAULT: u32 = 0u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
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
    pub DeviceIdentifier: ::windows_sys::core::GUID,
    pub WHQLLevel: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DADAPTER_IDENTIFIER9 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DADAPTER_IDENTIFIER9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "x86",))]
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
    pub DeviceIdentifier: ::windows_sys::core::GUID,
    pub WHQLLevel: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DADAPTER_IDENTIFIER9 {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DADAPTER_IDENTIFIER9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct D3DAES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for D3DAES_CTR_IV {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for D3DAES_CTR_IV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "x86",))]
pub struct D3DAES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for D3DAES_CTR_IV {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for D3DAES_CTR_IV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3DAUTHENTICATEDCHANNEL_D3D9: i32 = 1i32;
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_SOFTWARE: i32 = 2i32;
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_HARDWARE: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub EncryptionGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: ::windows_sys::core::GUID,
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: ::windows_sys::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROCESSIDTYPE_UNKNOWN: i32 = 0i32;
pub const PROCESSIDTYPE_DWM: i32 = 1i32;
pub const PROCESSIDTYPE_HANDLE: i32 = 2i32;
#[repr(C)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0,
}
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {}
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {}
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    pub QueryType: ::windows_sys::core::GUID,
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    pub omac: D3D_OMAC,
    pub QueryType: ::windows_sys::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3DAUTHENTICATEDCONFIGURE_CRYPTOSESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1665584212, data2: 11516, data3: 19156, data4: [130, 36, 209, 88, 55, 222, 119, 0] };
pub const D3DAUTHENTICATEDCONFIGURE_ENCRYPTIONWHENACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1107292806,
    data2: 27360,
    data3: 19779,
    data4: [157, 85, 164, 110, 158, 253, 21, 138],
};
pub const D3DAUTHENTICATEDCONFIGURE_INITIALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 101796827,
    data2: 13603,
    data3: 18186,
    data4: [141, 202, 251, 194, 132, 81, 84, 240],
};
pub const D3DAUTHENTICATEDCONFIGURE_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1346721368,
    data2: 16199,
    data3: 17250,
    data4: [191, 153, 191, 223, 205, 233, 237, 41],
};
pub const D3DAUTHENTICATEDCONFIGURE_SHAREDRESOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 124964935, data2: 6976, data3: 18664, data4: [156, 166, 181, 245, 16, 222, 159, 1] };
pub const D3DAUTHENTICATEDQUERY_ACCESSIBILITYATTRIBUTES: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1645533650,
    data2: 17196,
    data3: 19131,
    data4: [159, 206, 33, 110, 234, 38, 158, 59],
};
pub const D3DAUTHENTICATEDQUERY_CHANNELTYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3155892389,
    data2: 45563,
    data3: 17067,
    data4: [189, 148, 181, 130, 139, 75, 247, 190],
};
pub const D3DAUTHENTICATEDQUERY_CRYPTOSESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 640960926, data2: 53272, data3: 19828, data4: [172, 23, 127, 114, 64, 89, 82, 141] };
pub const D3DAUTHENTICATEDQUERY_CURRENTENCRYPTIONWHENACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3960967623,
    data2: 56019,
    data3: 20245,
    data4: [158, 195, 250, 169, 61, 96, 212, 240],
};
pub const D3DAUTHENTICATEDQUERY_DEVICEHANDLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3961279389,
    data2: 36095,
    data3: 20010,
    data4: [188, 196, 245, 105, 47, 153, 244, 128],
};
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4164573528, data2: 59782, data3: 19418, data4: [190, 176, 65, 31, 106, 122, 1, 183] };
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUIDCOUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3004133478, data2: 8252, data3: 19207, data4: [147, 252, 206, 170, 253, 97, 36, 30] };
pub const D3DAUTHENTICATEDQUERY_OUTPUTID: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2208160931,
    data2: 39758,
    data3: 16868,
    data4: [176, 83, 137, 43, 210, 161, 30, 231],
};
pub const D3DAUTHENTICATEDQUERY_OUTPUTIDCOUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 738470750,
    data2: 35847,
    data3: 18133,
    data4: [170, 190, 143, 117, 203, 173, 76, 49],
};
pub const D3DAUTHENTICATEDQUERY_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2823730564,
    data2: 50325,
    data3: 18602,
    data4: [185, 77, 139, 210, 214, 251, 206, 5],
};
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1687927515, data2: 61684, data3: 17977, data4: [161, 91, 36, 57, 63, 195, 171, 172] };
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESSCOUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 229771187,
    data2: 37968,
    data3: 18086,
    data4: [130, 222, 27, 150, 212, 79, 156, 242],
};
pub const D3DAUTHENTICATEDQUERY_UNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 19860438, data2: 58978, data3: 17524, data4: [190, 253, 170, 83, 229, 20, 60, 109] };
pub const D3DBACKBUFFER_TYPE_MONO: u32 = 0u32;
pub const D3DBACKBUFFER_TYPE_LEFT: u32 = 1u32;
pub const D3DBACKBUFFER_TYPE_RIGHT: u32 = 2u32;
pub const D3DBACKBUFFER_TYPE_FORCE_DWORD: u32 = 2147483647u32;
pub const D3DBASIS_BEZIER: i32 = 0i32;
pub const D3DBASIS_BSPLINE: i32 = 1i32;
pub const D3DBASIS_CATMULL_ROM: i32 = 2i32;
pub const D3DBASIS_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DBLEND_ZERO: u32 = 1u32;
pub const D3DBLEND_ONE: u32 = 2u32;
pub const D3DBLEND_SRCCOLOR: u32 = 3u32;
pub const D3DBLEND_INVSRCCOLOR: u32 = 4u32;
pub const D3DBLEND_SRCALPHA: u32 = 5u32;
pub const D3DBLEND_INVSRCALPHA: u32 = 6u32;
pub const D3DBLEND_DESTALPHA: u32 = 7u32;
pub const D3DBLEND_INVDESTALPHA: u32 = 8u32;
pub const D3DBLEND_DESTCOLOR: u32 = 9u32;
pub const D3DBLEND_INVDESTCOLOR: u32 = 10u32;
pub const D3DBLEND_SRCALPHASAT: u32 = 11u32;
pub const D3DBLEND_BOTHSRCALPHA: u32 = 12u32;
pub const D3DBLEND_BOTHINVSRCALPHA: u32 = 13u32;
pub const D3DBLEND_BLENDFACTOR: u32 = 14u32;
pub const D3DBLEND_INVBLENDFACTOR: u32 = 15u32;
pub const D3DBLEND_SRCCOLOR2: u32 = 16u32;
pub const D3DBLEND_INVSRCCOLOR2: u32 = 17u32;
pub const D3DBLEND_FORCE_DWORD: u32 = 2147483647u32;
pub const D3DBLENDOP_ADD: u32 = 1u32;
pub const D3DBLENDOP_SUBTRACT: u32 = 2u32;
pub const D3DBLENDOP_REVSUBTRACT: u32 = 3u32;
pub const D3DBLENDOP_MIN: u32 = 4u32;
pub const D3DBLENDOP_MAX: u32 = 5u32;
pub const D3DBLENDOP_FORCE_DWORD: u32 = 2147483647u32;
#[repr(C)]
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
pub const D3DBUSTYPE_OTHER: i32 = 0i32;
pub const D3DBUSTYPE_PCI: i32 = 1i32;
pub const D3DBUSTYPE_PCIX: i32 = 2i32;
pub const D3DBUSTYPE_PCIEXPRESS: i32 = 3i32;
pub const D3DBUSTYPE_AGP: i32 = 4i32;
pub const D3DBUSIMPL_MODIFIER_INSIDE_OF_CHIPSET: i32 = 65536i32;
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: i32 = 131072i32;
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: i32 = 196608i32;
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR: i32 = 262144i32;
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: i32 = 327680i32;
pub const D3DBUSIMPL_MODIFIER_NON_STANDARD: i32 = -2147483648i32;
pub const D3DCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824i32;
pub const D3DCAPS2_CANCALIBRATEGAMMA: i32 = 1048576i32;
pub const D3DCAPS2_CANMANAGERESOURCE: i32 = 268435456i32;
pub const D3DCAPS2_CANSHARERESOURCE: i32 = -2147483648i32;
pub const D3DCAPS2_DYNAMICTEXTURES: i32 = 536870912i32;
pub const D3DCAPS2_FULLSCREENGAMMA: i32 = 131072i32;
pub const D3DCAPS2_RESERVED: i32 = 33554432i32;
pub const D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD: i32 = 32i32;
pub const D3DCAPS3_COPY_TO_SYSTEMMEM: i32 = 512i32;
pub const D3DCAPS3_COPY_TO_VIDMEM: i32 = 256i32;
pub const D3DCAPS3_DXVAHD: i32 = 1024i32;
pub const D3DCAPS3_DXVAHD_LIMITED: i32 = 2048i32;
pub const D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION: i32 = 128i32;
pub const D3DCAPS3_RESERVED: i32 = -2147483617i32;
#[repr(C)]
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
pub const D3DCAPS_OVERLAY: i32 = 2048i32;
pub const D3DCAPS_READ_SCANLINE: i32 = 131072i32;
#[repr(C)]
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
pub const D3DCMP_NEVER: i32 = 1i32;
pub const D3DCMP_LESS: i32 = 2i32;
pub const D3DCMP_EQUAL: i32 = 3i32;
pub const D3DCMP_LESSEQUAL: i32 = 4i32;
pub const D3DCMP_GREATER: i32 = 5i32;
pub const D3DCMP_NOTEQUAL: i32 = 6i32;
pub const D3DCMP_GREATEREQUAL: i32 = 7i32;
pub const D3DCMP_ALWAYS: i32 = 8i32;
pub const D3DCMP_FORCE_DWORD: i32 = 2147483647i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const D3DCOMPOSERECTS_COPY: i32 = 1i32;
pub const D3DCOMPOSERECTS_OR: i32 = 2i32;
pub const D3DCOMPOSERECTS_AND: i32 = 3i32;
pub const D3DCOMPOSERECTS_NEG: i32 = 4i32;
pub const D3DCOMPOSERECTS_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DCOMPOSERECTS_MAXNUMRECTS: u32 = 65535u32;
pub const D3DCONVOLUTIONMONO_MAXHEIGHT: u32 = 7u32;
pub const D3DCONVOLUTIONMONO_MAXWIDTH: u32 = 7u32;
pub const D3DCPCAPS_CONTENTKEY: u32 = 16u32;
pub const D3DCPCAPS_ENCRYPTEDREADBACK: u32 = 64u32;
pub const D3DCPCAPS_ENCRYPTEDREADBACKKEY: u32 = 128u32;
pub const D3DCPCAPS_ENCRYPTSLICEDATAONLY: u32 = 512u32;
pub const D3DCPCAPS_FRESHENSESSIONKEY: u32 = 32u32;
pub const D3DCPCAPS_HARDWARE: u32 = 2u32;
pub const D3DCPCAPS_PARTIALDECRYPTION: u32 = 8u32;
pub const D3DCPCAPS_PROTECTIONALWAYSON: u32 = 4u32;
pub const D3DCPCAPS_SEQUENTIAL_CTR_IV: u32 = 256u32;
pub const D3DCPCAPS_SOFTWARE: u32 = 1u32;
pub const D3DCREATE_ADAPTERGROUP_DEVICE: i32 = 512i32;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: i32 = 256i32;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: i32 = 1024i32;
pub const D3DCREATE_DISABLE_PRINTSCREEN: i32 = 32768i32;
pub const D3DCREATE_DISABLE_PSGP_THREADING: i32 = 8192i32;
pub const D3DCREATE_ENABLE_PRESENTSTATS: i32 = 16384i32;
pub const D3DCREATE_FPU_PRESERVE: i32 = 2i32;
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: i32 = 64i32;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: i32 = 128i32;
pub const D3DCREATE_MULTITHREADED: i32 = 4i32;
pub const D3DCREATE_NOWINDOWCHANGES: i32 = 2048i32;
pub const D3DCREATE_PUREDEVICE: i32 = 16i32;
pub const D3DCREATE_SCREENSAVER: i32 = 268435456i32;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: i32 = 32i32;
pub const D3DCRYPTOTYPE_AES128_CTR: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2607535889,
    data2: 20340,
    data3: 16841,
    data4: [158, 123, 11, 226, 215, 217, 59, 79],
};
pub const D3DCRYPTOTYPE_PROPRIETARY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2874055421, data2: 7452, data3: 18150, data4: [167, 47, 8, 105, 145, 123, 13, 232] };
pub const D3DCS_BACK: i32 = 32i32;
pub const D3DCS_BOTTOM: i32 = 8i32;
pub const D3DCS_FRONT: i32 = 16i32;
pub const D3DCS_LEFT: i32 = 1i32;
pub const D3DCS_PLANE0: i32 = 64i32;
pub const D3DCS_PLANE1: i32 = 128i32;
pub const D3DCS_PLANE2: i32 = 256i32;
pub const D3DCS_PLANE3: i32 = 512i32;
pub const D3DCS_PLANE4: i32 = 1024i32;
pub const D3DCS_PLANE5: i32 = 2048i32;
pub const D3DCS_RIGHT: i32 = 2i32;
pub const D3DCS_TOP: i32 = 4i32;
pub const D3DCUBEMAP_FACE_POSITIVE_X: i32 = 0i32;
pub const D3DCUBEMAP_FACE_NEGATIVE_X: i32 = 1i32;
pub const D3DCUBEMAP_FACE_POSITIVE_Y: i32 = 2i32;
pub const D3DCUBEMAP_FACE_NEGATIVE_Y: i32 = 3i32;
pub const D3DCUBEMAP_FACE_POSITIVE_Z: i32 = 4i32;
pub const D3DCUBEMAP_FACE_NEGATIVE_Z: i32 = 5i32;
pub const D3DCUBEMAP_FACE_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DCULL_NONE: u32 = 1u32;
pub const D3DCULL_CW: u32 = 2u32;
pub const D3DCULL_CCW: u32 = 3u32;
pub const D3DCULL_FORCE_DWORD: u32 = 2147483647u32;
pub const D3DCURSORCAPS_COLOR: i32 = 1i32;
pub const D3DCURSORCAPS_LOWRES: i32 = 2i32;
pub const D3DCURSOR_IMMEDIATE_UPDATE: i32 = 1i32;
pub const D3DDMT_ENABLE: i32 = 0i32;
pub const D3DDMT_DISABLE: i32 = 1i32;
pub const D3DDMT_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DDECLMETHOD_DEFAULT: i32 = 0i32;
pub const D3DDECLMETHOD_PARTIALU: i32 = 1i32;
pub const D3DDECLMETHOD_PARTIALV: i32 = 2i32;
pub const D3DDECLMETHOD_CROSSUV: i32 = 3i32;
pub const D3DDECLMETHOD_UV: i32 = 4i32;
pub const D3DDECLMETHOD_LOOKUP: i32 = 5i32;
pub const D3DDECLMETHOD_LOOKUPPRESAMPLED: i32 = 6i32;
pub const D3DDECLTYPE_FLOAT1: i32 = 0i32;
pub const D3DDECLTYPE_FLOAT2: i32 = 1i32;
pub const D3DDECLTYPE_FLOAT3: i32 = 2i32;
pub const D3DDECLTYPE_FLOAT4: i32 = 3i32;
pub const D3DDECLTYPE_D3DCOLOR: i32 = 4i32;
pub const D3DDECLTYPE_UBYTE4: i32 = 5i32;
pub const D3DDECLTYPE_SHORT2: i32 = 6i32;
pub const D3DDECLTYPE_SHORT4: i32 = 7i32;
pub const D3DDECLTYPE_UBYTE4N: i32 = 8i32;
pub const D3DDECLTYPE_SHORT2N: i32 = 9i32;
pub const D3DDECLTYPE_SHORT4N: i32 = 10i32;
pub const D3DDECLTYPE_USHORT2N: i32 = 11i32;
pub const D3DDECLTYPE_USHORT4N: i32 = 12i32;
pub const D3DDECLTYPE_UDEC3: i32 = 13i32;
pub const D3DDECLTYPE_DEC3N: i32 = 14i32;
pub const D3DDECLTYPE_FLOAT16_2: i32 = 15i32;
pub const D3DDECLTYPE_FLOAT16_4: i32 = 16i32;
pub const D3DDECLTYPE_UNUSED: i32 = 17i32;
pub const D3DDECLUSAGE_POSITION: i32 = 0i32;
pub const D3DDECLUSAGE_BLENDWEIGHT: i32 = 1i32;
pub const D3DDECLUSAGE_BLENDINDICES: i32 = 2i32;
pub const D3DDECLUSAGE_NORMAL: i32 = 3i32;
pub const D3DDECLUSAGE_PSIZE: i32 = 4i32;
pub const D3DDECLUSAGE_TEXCOORD: i32 = 5i32;
pub const D3DDECLUSAGE_TANGENT: i32 = 6i32;
pub const D3DDECLUSAGE_BINORMAL: i32 = 7i32;
pub const D3DDECLUSAGE_TESSFACTOR: i32 = 8i32;
pub const D3DDECLUSAGE_POSITIONT: i32 = 9i32;
pub const D3DDECLUSAGE_COLOR: i32 = 10i32;
pub const D3DDECLUSAGE_FOG: i32 = 11i32;
pub const D3DDECLUSAGE_DEPTH: i32 = 12i32;
pub const D3DDECLUSAGE_SAMPLE: i32 = 13i32;
pub const D3DDEGREE_LINEAR: i32 = 1i32;
pub const D3DDEGREE_QUADRATIC: i32 = 2i32;
pub const D3DDEGREE_CUBIC: i32 = 3i32;
pub const D3DDEGREE_QUINTIC: i32 = 5i32;
pub const D3DDEGREE_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DDEVCAPS2_ADAPTIVETESSNPATCH: i32 = 8i32;
pub const D3DDEVCAPS2_ADAPTIVETESSRTPATCH: i32 = 4i32;
pub const D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES: i32 = 16i32;
pub const D3DDEVCAPS2_DMAPNPATCH: i32 = 2i32;
pub const D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH: i32 = 32i32;
pub const D3DDEVCAPS2_STREAMOFFSET: i32 = 1i32;
pub const D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET: i32 = 64i32;
pub const D3DDEVCAPS_NPATCHES: i32 = 16777216i32;
pub const D3DDEVCAPS_PUREDEVICE: i32 = 1048576i32;
pub const D3DDEVCAPS_QUINTICRTPATCHES: i32 = 2097152i32;
pub const D3DDEVCAPS_RTPATCHES: i32 = 4194304i32;
pub const D3DDEVCAPS_RTPATCHHANDLEZERO: i32 = 8388608i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const D3DDEVTYPE_HAL: u32 = 1u32;
pub const D3DDEVTYPE_REF: u32 = 2u32;
pub const D3DDEVTYPE_SW: u32 = 3u32;
pub const D3DDEVTYPE_NULLREF: u32 = 4u32;
pub const D3DDEVTYPE_FORCE_DWORD: u32 = 2147483647u32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const D3DDISPLAYROTATION_IDENTITY: i32 = 1i32;
pub const D3DDISPLAYROTATION_90: i32 = 2i32;
pub const D3DDISPLAYROTATION_180: i32 = 3i32;
pub const D3DDISPLAYROTATION_270: i32 = 4i32;
pub const D3DDMAPSAMPLER: u32 = 256u32;
pub const D3DDTCAPS_DEC3N: i32 = 128i32;
pub const D3DDTCAPS_FLOAT16_2: i32 = 256i32;
pub const D3DDTCAPS_FLOAT16_4: i32 = 512i32;
pub const D3DDTCAPS_SHORT2N: i32 = 4i32;
pub const D3DDTCAPS_SHORT4N: i32 = 8i32;
pub const D3DDTCAPS_UBYTE4: i32 = 1i32;
pub const D3DDTCAPS_UBYTE4N: i32 = 2i32;
pub const D3DDTCAPS_UDEC3: i32 = 64i32;
pub const D3DDTCAPS_USHORT2N: i32 = 16i32;
pub const D3DDTCAPS_USHORT4N: i32 = 32i32;
#[repr(C)]
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
pub const D3DENUM_NO_DRIVERVERSION: i32 = 4i32;
pub const D3DENUM_WHQL_LEVEL: i32 = 2i32;
pub const D3DFILL_POINT: i32 = 1i32;
pub const D3DFILL_WIREFRAME: i32 = 2i32;
pub const D3DFILL_SOLID: i32 = 3i32;
pub const D3DFILL_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DFMT_A1_SURFACE_MAXHEIGHT: u32 = 2048u32;
pub const D3DFMT_A1_SURFACE_MAXWIDTH: u32 = 8192u32;
pub const D3DFOG_NONE: i32 = 0i32;
pub const D3DFOG_EXP: i32 = 1i32;
pub const D3DFOG_EXP2: i32 = 2i32;
pub const D3DFOG_LINEAR: i32 = 3i32;
pub const D3DFOG_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DFMT_UNKNOWN: u32 = 0u32;
pub const D3DFMT_R8G8B8: u32 = 20u32;
pub const D3DFMT_A8R8G8B8: u32 = 21u32;
pub const D3DFMT_X8R8G8B8: u32 = 22u32;
pub const D3DFMT_R5G6B5: u32 = 23u32;
pub const D3DFMT_X1R5G5B5: u32 = 24u32;
pub const D3DFMT_A1R5G5B5: u32 = 25u32;
pub const D3DFMT_A4R4G4B4: u32 = 26u32;
pub const D3DFMT_R3G3B2: u32 = 27u32;
pub const D3DFMT_A8: u32 = 28u32;
pub const D3DFMT_A8R3G3B2: u32 = 29u32;
pub const D3DFMT_X4R4G4B4: u32 = 30u32;
pub const D3DFMT_A2B10G10R10: u32 = 31u32;
pub const D3DFMT_A8B8G8R8: u32 = 32u32;
pub const D3DFMT_X8B8G8R8: u32 = 33u32;
pub const D3DFMT_G16R16: u32 = 34u32;
pub const D3DFMT_A2R10G10B10: u32 = 35u32;
pub const D3DFMT_A16B16G16R16: u32 = 36u32;
pub const D3DFMT_A8P8: u32 = 40u32;
pub const D3DFMT_P8: u32 = 41u32;
pub const D3DFMT_L8: u32 = 50u32;
pub const D3DFMT_A8L8: u32 = 51u32;
pub const D3DFMT_A4L4: u32 = 52u32;
pub const D3DFMT_V8U8: u32 = 60u32;
pub const D3DFMT_L6V5U5: u32 = 61u32;
pub const D3DFMT_X8L8V8U8: u32 = 62u32;
pub const D3DFMT_Q8W8V8U8: u32 = 63u32;
pub const D3DFMT_V16U16: u32 = 64u32;
pub const D3DFMT_A2W10V10U10: u32 = 67u32;
pub const D3DFMT_UYVY: u32 = 1498831189u32;
pub const D3DFMT_R8G8_B8G8: u32 = 1195525970u32;
pub const D3DFMT_YUY2: u32 = 844715353u32;
pub const D3DFMT_G8R8_G8B8: u32 = 1111970375u32;
pub const D3DFMT_DXT1: u32 = 827611204u32;
pub const D3DFMT_DXT2: u32 = 844388420u32;
pub const D3DFMT_DXT3: u32 = 861165636u32;
pub const D3DFMT_DXT4: u32 = 877942852u32;
pub const D3DFMT_DXT5: u32 = 894720068u32;
pub const D3DFMT_D16_LOCKABLE: u32 = 70u32;
pub const D3DFMT_D32: u32 = 71u32;
pub const D3DFMT_D15S1: u32 = 73u32;
pub const D3DFMT_D24S8: u32 = 75u32;
pub const D3DFMT_D24X8: u32 = 77u32;
pub const D3DFMT_D24X4S4: u32 = 79u32;
pub const D3DFMT_D16: u32 = 80u32;
pub const D3DFMT_D32F_LOCKABLE: u32 = 82u32;
pub const D3DFMT_D24FS8: u32 = 83u32;
pub const D3DFMT_D32_LOCKABLE: u32 = 84u32;
pub const D3DFMT_S8_LOCKABLE: u32 = 85u32;
pub const D3DFMT_L16: u32 = 81u32;
pub const D3DFMT_VERTEXDATA: u32 = 100u32;
pub const D3DFMT_INDEX16: u32 = 101u32;
pub const D3DFMT_INDEX32: u32 = 102u32;
pub const D3DFMT_Q16W16V16U16: u32 = 110u32;
pub const D3DFMT_MULTI2_ARGB8: u32 = 827606349u32;
pub const D3DFMT_R16F: u32 = 111u32;
pub const D3DFMT_G16R16F: u32 = 112u32;
pub const D3DFMT_A16B16G16R16F: u32 = 113u32;
pub const D3DFMT_R32F: u32 = 114u32;
pub const D3DFMT_G32R32F: u32 = 115u32;
pub const D3DFMT_A32B32G32R32F: u32 = 116u32;
pub const D3DFMT_CxV8U8: u32 = 117u32;
pub const D3DFMT_A1: u32 = 118u32;
pub const D3DFMT_A2B10G10R10_XR_BIAS: u32 = 119u32;
pub const D3DFMT_BINARYBUFFER: u32 = 199u32;
pub const D3DFMT_FORCE_DWORD: u32 = 2147483647u32;
pub const D3DFVFCAPS_PSIZE: i32 = 1048576i32;
pub const D3DFVF_LASTBETA_D3DCOLOR: u32 = 32768u32;
pub const D3DFVF_LASTBETA_UBYTE4: u32 = 4096u32;
pub const D3DFVF_PSIZE: u32 = 32u32;
pub const D3DFVF_XYZW: u32 = 16386u32;
#[repr(C)]
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
pub const D3DGETDATA_FLUSH: u32 = 1u32;
#[repr(C)]
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
pub const D3DISSUE_BEGIN: u32 = 2u32;
pub const D3DISSUE_END: u32 = 1u32;
pub const D3DKEYEXCHANGE_DXVA: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1137932124,
    data2: 14565,
    data3: 18724,
    data4: [141, 134, 211, 252, 207, 21, 62, 155],
};
pub const D3DKEYEXCHANGE_RSAES_OAEP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3247741077, data2: 55082, data3: 18973, data4: [142, 93, 237, 133, 125, 23, 21, 32] };
#[repr(C)]
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
pub const D3DLIGHT_POINT: i32 = 1i32;
pub const D3DLIGHT_SPOT: i32 = 2i32;
pub const D3DLIGHT_DIRECTIONAL: i32 = 3i32;
pub const D3DLIGHT_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DLINECAPS_ALPHACMP: i32 = 8i32;
pub const D3DLINECAPS_ANTIALIAS: i32 = 32i32;
pub const D3DLINECAPS_BLEND: i32 = 4i32;
pub const D3DLINECAPS_FOG: i32 = 16i32;
pub const D3DLINECAPS_TEXTURE: i32 = 1i32;
pub const D3DLINECAPS_ZTEST: i32 = 2i32;
#[repr(C)]
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
#[repr(C)]
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
pub const D3DLOCK_DISCARD: i32 = 8192i32;
pub const D3DLOCK_DONOTWAIT: i32 = 16384i32;
pub const D3DLOCK_NOOVERWRITE: i32 = 4096i32;
pub const D3DLOCK_NOSYSLOCK: i32 = 2048i32;
pub const D3DLOCK_NO_DIRTY_UPDATE: i32 = 32768i32;
pub const D3DLOCK_READONLY: i32 = 16i32;
#[repr(C)]
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
pub const D3DMCS_MATERIAL: i32 = 0i32;
pub const D3DMCS_COLOR1: i32 = 1i32;
pub const D3DMCS_COLOR2: i32 = 2i32;
pub const D3DMCS_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DMAX30SHADERINSTRUCTIONS: u32 = 32768u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: u64,
    pub SizeOfInefficientAllocation: u64,
    pub LevelOfEfficiency: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for D3DMEMORYPRESSURE {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for D3DMEMORYPRESSURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "x86",))]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: u64,
    pub SizeOfInefficientAllocation: u64,
    pub LevelOfEfficiency: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for D3DMEMORYPRESSURE {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for D3DMEMORYPRESSURE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3DMIN30SHADERINSTRUCTIONS: u32 = 512u32;
pub const D3DMULTISAMPLE_NONE: i32 = 0i32;
pub const D3DMULTISAMPLE_NONMASKABLE: i32 = 1i32;
pub const D3DMULTISAMPLE_2_SAMPLES: i32 = 2i32;
pub const D3DMULTISAMPLE_3_SAMPLES: i32 = 3i32;
pub const D3DMULTISAMPLE_4_SAMPLES: i32 = 4i32;
pub const D3DMULTISAMPLE_5_SAMPLES: i32 = 5i32;
pub const D3DMULTISAMPLE_6_SAMPLES: i32 = 6i32;
pub const D3DMULTISAMPLE_7_SAMPLES: i32 = 7i32;
pub const D3DMULTISAMPLE_8_SAMPLES: i32 = 8i32;
pub const D3DMULTISAMPLE_9_SAMPLES: i32 = 9i32;
pub const D3DMULTISAMPLE_10_SAMPLES: i32 = 10i32;
pub const D3DMULTISAMPLE_11_SAMPLES: i32 = 11i32;
pub const D3DMULTISAMPLE_12_SAMPLES: i32 = 12i32;
pub const D3DMULTISAMPLE_13_SAMPLES: i32 = 13i32;
pub const D3DMULTISAMPLE_14_SAMPLES: i32 = 14i32;
pub const D3DMULTISAMPLE_15_SAMPLES: i32 = 15i32;
pub const D3DMULTISAMPLE_16_SAMPLES: i32 = 16i32;
pub const D3DMULTISAMPLE_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DOVERLAYCAPS_FULLRANGERGB: u32 = 1u32;
pub const D3DOVERLAYCAPS_LIMITEDRANGERGB: u32 = 2u32;
pub const D3DOVERLAYCAPS_STRETCHX: u32 = 64u32;
pub const D3DOVERLAYCAPS_STRETCHY: u32 = 128u32;
pub const D3DOVERLAYCAPS_YCbCr_BT601: u32 = 4u32;
pub const D3DOVERLAYCAPS_YCbCr_BT601_xvYCC: u32 = 16u32;
pub const D3DOVERLAYCAPS_YCbCr_BT709: u32 = 8u32;
pub const D3DOVERLAYCAPS_YCbCr_BT709_xvYCC: u32 = 32u32;
pub const D3DPATCHEDGE_DISCRETE: i32 = 0i32;
pub const D3DPATCHEDGE_CONTINUOUS: i32 = 1i32;
pub const D3DPATCHEDGE_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DPBLENDCAPS_BLENDFACTOR: i32 = 8192i32;
pub const D3DPBLENDCAPS_INVSRCCOLOR2: i32 = 32768i32;
pub const D3DPBLENDCAPS_SRCCOLOR2: i32 = 16384i32;
pub const D3DPMISCCAPS_BLENDOP: i32 = 2048i32;
pub const D3DPMISCCAPS_CLIPPLANESCALEDPOINTS: i32 = 256i32;
pub const D3DPMISCCAPS_CLIPTLVERTS: i32 = 512i32;
pub const D3DPMISCCAPS_COLORWRITEENABLE: i32 = 128i32;
pub const D3DPMISCCAPS_FOGANDSPECULARALPHA: i32 = 65536i32;
pub const D3DPMISCCAPS_FOGVERTEXCLAMPED: i32 = 1048576i32;
pub const D3DPMISCCAPS_INDEPENDENTWRITEMASKS: i32 = 16384i32;
pub const D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS: i32 = 262144i32;
pub const D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING: i32 = 524288i32;
pub const D3DPMISCCAPS_NULLREFERENCE: i32 = 4096i32;
pub const D3DPMISCCAPS_PERSTAGECONSTANT: i32 = 32768i32;
pub const D3DPMISCCAPS_POSTBLENDSRGBCONVERT: i32 = 2097152i32;
pub const D3DPMISCCAPS_SEPARATEALPHABLEND: i32 = 131072i32;
pub const D3DPMISCCAPS_TSSARGTEMP: i32 = 1024i32;
pub const D3DPOOL_DEFAULT: u32 = 0u32;
pub const D3DPOOL_MANAGED: u32 = 1u32;
pub const D3DPOOL_SYSTEMMEM: u32 = 2u32;
pub const D3DPOOL_SCRATCH: u32 = 3u32;
pub const D3DPOOL_FORCE_DWORD: u32 = 2147483647u32;
pub const D3DPRASTERCAPS_COLORPERSPECTIVE: i32 = 4194304i32;
pub const D3DPRASTERCAPS_DEPTHBIAS: i32 = 67108864i32;
pub const D3DPRASTERCAPS_MULTISAMPLE_TOGGLE: i32 = 134217728i32;
pub const D3DPRASTERCAPS_SCISSORTEST: i32 = 16777216i32;
pub const D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS: i32 = 33554432i32;
pub const D3DPRESENTFLAG_DEVICECLIP: u32 = 4u32;
pub const D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL: u32 = 2u32;
pub const D3DPRESENTFLAG_LOCKABLE_BACKBUFFER: u32 = 1u32;
pub const D3DPRESENTFLAG_NOAUTOROTATE: u32 = 32u32;
pub const D3DPRESENTFLAG_OVERLAY_LIMITEDRGB: u32 = 128u32;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_BT709: u32 = 256u32;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC: u32 = 512u32;
pub const D3DPRESENTFLAG_RESTRICTED_CONTENT: u32 = 1024u32;
pub const D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER: u32 = 2048u32;
pub const D3DPRESENTFLAG_UNPRUNEDMODE: u32 = 64u32;
pub const D3DPRESENTFLAG_VIDEO: u32 = 16u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct D3DPRESENTSTATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for D3DPRESENTSTATS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for D3DPRESENTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "x86",))]
pub struct D3DPRESENTSTATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for D3DPRESENTSTATS {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for D3DPRESENTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3DPRESENT_BACK_BUFFERS_MAX: i32 = 3i32;
pub const D3DPRESENT_BACK_BUFFERS_MAX_EX: i32 = 30i32;
pub const D3DPRESENT_DONOTFLIP: i32 = 4i32;
pub const D3DPRESENT_DONOTWAIT: i32 = 1i32;
pub const D3DPRESENT_FLIPRESTART: i32 = 8i32;
pub const D3DPRESENT_FORCEIMMEDIATE: i32 = 256i32;
pub const D3DPRESENT_HIDEOVERLAY: i32 = 64i32;
pub const D3DPRESENT_INTERVAL_DEFAULT: i32 = 0i32;
pub const D3DPRESENT_INTERVAL_FOUR: i32 = 8i32;
pub const D3DPRESENT_INTERVAL_IMMEDIATE: i32 = -2147483648i32;
pub const D3DPRESENT_INTERVAL_ONE: i32 = 1i32;
pub const D3DPRESENT_INTERVAL_THREE: i32 = 4i32;
pub const D3DPRESENT_INTERVAL_TWO: i32 = 2i32;
pub const D3DPRESENT_LINEAR_CONTENT: i32 = 2i32;
#[repr(C)]
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
pub const D3DPRESENT_RATE_DEFAULT: u32 = 0u32;
pub const D3DPRESENT_UPDATECOLORKEY: i32 = 128i32;
pub const D3DPRESENT_UPDATEOVERLAYONLY: i32 = 32i32;
pub const D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR: i32 = 16i32;
pub const D3DPT_POINTLIST: i32 = 1i32;
pub const D3DPT_LINELIST: i32 = 2i32;
pub const D3DPT_LINESTRIP: i32 = 3i32;
pub const D3DPT_TRIANGLELIST: i32 = 4i32;
pub const D3DPT_TRIANGLESTRIP: i32 = 5i32;
pub const D3DPT_TRIANGLEFAN: i32 = 6i32;
pub const D3DPT_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DPS20CAPS_ARBITRARYSWIZZLE: u32 = 1u32;
pub const D3DPS20CAPS_GRADIENTINSTRUCTIONS: u32 = 2u32;
pub const D3DPS20CAPS_NODEPENDENTREADLIMIT: u32 = 8u32;
pub const D3DPS20CAPS_NOTEXINSTRUCTIONLIMIT: u32 = 16u32;
pub const D3DPS20CAPS_PREDICATION: u32 = 4u32;
pub const D3DPS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24u32;
pub const D3DPS20_MAX_NUMINSTRUCTIONSLOTS: u32 = 512u32;
pub const D3DPS20_MAX_NUMTEMPS: u32 = 32u32;
pub const D3DPS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4u32;
pub const D3DPS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0u32;
pub const D3DPS20_MIN_NUMINSTRUCTIONSLOTS: u32 = 96u32;
pub const D3DPS20_MIN_NUMTEMPS: u32 = 12u32;
pub const D3DPS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 0u32;
#[repr(C)]
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
pub const D3DPTADDRESSCAPS_MIRRORONCE: i32 = 32i32;
pub const D3DPTEXTURECAPS_CUBEMAP_POW2: i32 = 131072i32;
pub const D3DPTEXTURECAPS_MIPCUBEMAP: i32 = 65536i32;
pub const D3DPTEXTURECAPS_MIPMAP: i32 = 16384i32;
pub const D3DPTEXTURECAPS_MIPVOLUMEMAP: i32 = 32768i32;
pub const D3DPTEXTURECAPS_NOPROJECTEDBUMPENV: i32 = 2097152i32;
pub const D3DPTEXTURECAPS_VOLUMEMAP: i32 = 8192i32;
pub const D3DPTEXTURECAPS_VOLUMEMAP_POW2: i32 = 262144i32;
pub const D3DPTFILTERCAPS_CONVOLUTIONMONO: i32 = 262144i32;
pub const D3DPTFILTERCAPS_MAGFGAUSSIANQUAD: i32 = 268435456i32;
pub const D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD: i32 = 134217728i32;
pub const D3DPTFILTERCAPS_MINFGAUSSIANQUAD: i32 = 4096i32;
pub const D3DPTFILTERCAPS_MINFPYRAMIDALQUAD: i32 = 2048i32;
pub const D3DQUERYTYPE_VCACHE: i32 = 4i32;
pub const D3DQUERYTYPE_RESOURCEMANAGER: i32 = 5i32;
pub const D3DQUERYTYPE_VERTEXSTATS: i32 = 6i32;
pub const D3DQUERYTYPE_EVENT: i32 = 8i32;
pub const D3DQUERYTYPE_OCCLUSION: i32 = 9i32;
pub const D3DQUERYTYPE_TIMESTAMP: i32 = 10i32;
pub const D3DQUERYTYPE_TIMESTAMPDISJOINT: i32 = 11i32;
pub const D3DQUERYTYPE_TIMESTAMPFREQ: i32 = 12i32;
pub const D3DQUERYTYPE_PIPELINETIMINGS: i32 = 13i32;
pub const D3DQUERYTYPE_INTERFACETIMINGS: i32 = 14i32;
pub const D3DQUERYTYPE_VERTEXTIMINGS: i32 = 15i32;
pub const D3DQUERYTYPE_PIXELTIMINGS: i32 = 16i32;
pub const D3DQUERYTYPE_BANDWIDTHTIMINGS: i32 = 17i32;
pub const D3DQUERYTYPE_CACHEUTILIZATION: i32 = 18i32;
pub const D3DQUERYTYPE_MEMORYPRESSURE: i32 = 19i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const D3DRS_ZENABLE: i32 = 7i32;
pub const D3DRS_FILLMODE: i32 = 8i32;
pub const D3DRS_SHADEMODE: i32 = 9i32;
pub const D3DRS_ZWRITEENABLE: i32 = 14i32;
pub const D3DRS_ALPHATESTENABLE: i32 = 15i32;
pub const D3DRS_LASTPIXEL: i32 = 16i32;
pub const D3DRS_SRCBLEND: i32 = 19i32;
pub const D3DRS_DESTBLEND: i32 = 20i32;
pub const D3DRS_CULLMODE: i32 = 22i32;
pub const D3DRS_ZFUNC: i32 = 23i32;
pub const D3DRS_ALPHAREF: i32 = 24i32;
pub const D3DRS_ALPHAFUNC: i32 = 25i32;
pub const D3DRS_DITHERENABLE: i32 = 26i32;
pub const D3DRS_ALPHABLENDENABLE: i32 = 27i32;
pub const D3DRS_FOGENABLE: i32 = 28i32;
pub const D3DRS_SPECULARENABLE: i32 = 29i32;
pub const D3DRS_FOGCOLOR: i32 = 34i32;
pub const D3DRS_FOGTABLEMODE: i32 = 35i32;
pub const D3DRS_FOGSTART: i32 = 36i32;
pub const D3DRS_FOGEND: i32 = 37i32;
pub const D3DRS_FOGDENSITY: i32 = 38i32;
pub const D3DRS_RANGEFOGENABLE: i32 = 48i32;
pub const D3DRS_STENCILENABLE: i32 = 52i32;
pub const D3DRS_STENCILFAIL: i32 = 53i32;
pub const D3DRS_STENCILZFAIL: i32 = 54i32;
pub const D3DRS_STENCILPASS: i32 = 55i32;
pub const D3DRS_STENCILFUNC: i32 = 56i32;
pub const D3DRS_STENCILREF: i32 = 57i32;
pub const D3DRS_STENCILMASK: i32 = 58i32;
pub const D3DRS_STENCILWRITEMASK: i32 = 59i32;
pub const D3DRS_TEXTUREFACTOR: i32 = 60i32;
pub const D3DRS_WRAP0: i32 = 128i32;
pub const D3DRS_WRAP1: i32 = 129i32;
pub const D3DRS_WRAP2: i32 = 130i32;
pub const D3DRS_WRAP3: i32 = 131i32;
pub const D3DRS_WRAP4: i32 = 132i32;
pub const D3DRS_WRAP5: i32 = 133i32;
pub const D3DRS_WRAP6: i32 = 134i32;
pub const D3DRS_WRAP7: i32 = 135i32;
pub const D3DRS_CLIPPING: i32 = 136i32;
pub const D3DRS_LIGHTING: i32 = 137i32;
pub const D3DRS_AMBIENT: i32 = 139i32;
pub const D3DRS_FOGVERTEXMODE: i32 = 140i32;
pub const D3DRS_COLORVERTEX: i32 = 141i32;
pub const D3DRS_LOCALVIEWER: i32 = 142i32;
pub const D3DRS_NORMALIZENORMALS: i32 = 143i32;
pub const D3DRS_DIFFUSEMATERIALSOURCE: i32 = 145i32;
pub const D3DRS_SPECULARMATERIALSOURCE: i32 = 146i32;
pub const D3DRS_AMBIENTMATERIALSOURCE: i32 = 147i32;
pub const D3DRS_EMISSIVEMATERIALSOURCE: i32 = 148i32;
pub const D3DRS_VERTEXBLEND: i32 = 151i32;
pub const D3DRS_CLIPPLANEENABLE: i32 = 152i32;
pub const D3DRS_POINTSIZE: i32 = 154i32;
pub const D3DRS_POINTSIZE_MIN: i32 = 155i32;
pub const D3DRS_POINTSPRITEENABLE: i32 = 156i32;
pub const D3DRS_POINTSCALEENABLE: i32 = 157i32;
pub const D3DRS_POINTSCALE_A: i32 = 158i32;
pub const D3DRS_POINTSCALE_B: i32 = 159i32;
pub const D3DRS_POINTSCALE_C: i32 = 160i32;
pub const D3DRS_MULTISAMPLEANTIALIAS: i32 = 161i32;
pub const D3DRS_MULTISAMPLEMASK: i32 = 162i32;
pub const D3DRS_PATCHEDGESTYLE: i32 = 163i32;
pub const D3DRS_DEBUGMONITORTOKEN: i32 = 165i32;
pub const D3DRS_POINTSIZE_MAX: i32 = 166i32;
pub const D3DRS_INDEXEDVERTEXBLENDENABLE: i32 = 167i32;
pub const D3DRS_COLORWRITEENABLE: i32 = 168i32;
pub const D3DRS_TWEENFACTOR: i32 = 170i32;
pub const D3DRS_BLENDOP: i32 = 171i32;
pub const D3DRS_POSITIONDEGREE: i32 = 172i32;
pub const D3DRS_NORMALDEGREE: i32 = 173i32;
pub const D3DRS_SCISSORTESTENABLE: i32 = 174i32;
pub const D3DRS_SLOPESCALEDEPTHBIAS: i32 = 175i32;
pub const D3DRS_ANTIALIASEDLINEENABLE: i32 = 176i32;
pub const D3DRS_MINTESSELLATIONLEVEL: i32 = 178i32;
pub const D3DRS_MAXTESSELLATIONLEVEL: i32 = 179i32;
pub const D3DRS_ADAPTIVETESS_X: i32 = 180i32;
pub const D3DRS_ADAPTIVETESS_Y: i32 = 181i32;
pub const D3DRS_ADAPTIVETESS_Z: i32 = 182i32;
pub const D3DRS_ADAPTIVETESS_W: i32 = 183i32;
pub const D3DRS_ENABLEADAPTIVETESSELLATION: i32 = 184i32;
pub const D3DRS_TWOSIDEDSTENCILMODE: i32 = 185i32;
pub const D3DRS_CCW_STENCILFAIL: i32 = 186i32;
pub const D3DRS_CCW_STENCILZFAIL: i32 = 187i32;
pub const D3DRS_CCW_STENCILPASS: i32 = 188i32;
pub const D3DRS_CCW_STENCILFUNC: i32 = 189i32;
pub const D3DRS_COLORWRITEENABLE1: i32 = 190i32;
pub const D3DRS_COLORWRITEENABLE2: i32 = 191i32;
pub const D3DRS_COLORWRITEENABLE3: i32 = 192i32;
pub const D3DRS_BLENDFACTOR: i32 = 193i32;
pub const D3DRS_SRGBWRITEENABLE: i32 = 194i32;
pub const D3DRS_DEPTHBIAS: i32 = 195i32;
pub const D3DRS_WRAP8: i32 = 198i32;
pub const D3DRS_WRAP9: i32 = 199i32;
pub const D3DRS_WRAP10: i32 = 200i32;
pub const D3DRS_WRAP11: i32 = 201i32;
pub const D3DRS_WRAP12: i32 = 202i32;
pub const D3DRS_WRAP13: i32 = 203i32;
pub const D3DRS_WRAP14: i32 = 204i32;
pub const D3DRS_WRAP15: i32 = 205i32;
pub const D3DRS_SEPARATEALPHABLENDENABLE: i32 = 206i32;
pub const D3DRS_SRCBLENDALPHA: i32 = 207i32;
pub const D3DRS_DESTBLENDALPHA: i32 = 208i32;
pub const D3DRS_BLENDOPALPHA: i32 = 209i32;
pub const D3DRS_FORCE_DWORD: i32 = 2147483647i32;
#[repr(C)]
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
pub const D3DRTYPE_SURFACE: i32 = 1i32;
pub const D3DRTYPE_VOLUME: i32 = 2i32;
pub const D3DRTYPE_TEXTURE: i32 = 3i32;
pub const D3DRTYPE_VOLUMETEXTURE: i32 = 4i32;
pub const D3DRTYPE_CUBETEXTURE: i32 = 5i32;
pub const D3DRTYPE_VERTEXBUFFER: i32 = 6i32;
pub const D3DRTYPE_INDEXBUFFER: i32 = 7i32;
pub const D3DRTYPE_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DRTYPECOUNT: u32 = 8u32;
pub const D3DSAMP_ADDRESSU: i32 = 1i32;
pub const D3DSAMP_ADDRESSV: i32 = 2i32;
pub const D3DSAMP_ADDRESSW: i32 = 3i32;
pub const D3DSAMP_BORDERCOLOR: i32 = 4i32;
pub const D3DSAMP_MAGFILTER: i32 = 5i32;
pub const D3DSAMP_MINFILTER: i32 = 6i32;
pub const D3DSAMP_MIPFILTER: i32 = 7i32;
pub const D3DSAMP_MIPMAPLODBIAS: i32 = 8i32;
pub const D3DSAMP_MAXMIPLEVEL: i32 = 9i32;
pub const D3DSAMP_MAXANISOTROPY: i32 = 10i32;
pub const D3DSAMP_SRGBTEXTURE: i32 = 11i32;
pub const D3DSAMP_ELEMENTINDEX: i32 = 12i32;
pub const D3DSAMP_DMAPOFFSET: i32 = 13i32;
pub const D3DSAMP_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DSTT_UNKNOWN: i32 = 0i32;
pub const D3DSTT_2D: i32 = 268435456i32;
pub const D3DSTT_CUBE: i32 = 402653184i32;
pub const D3DSTT_VOLUME: i32 = 536870912i32;
pub const D3DSTT_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DSCANLINEORDERING_UNKNOWN: i32 = 0i32;
pub const D3DSCANLINEORDERING_PROGRESSIVE: i32 = 1i32;
pub const D3DSCANLINEORDERING_INTERLACED: i32 = 2i32;
pub const D3DSGR_CALIBRATE: i32 = 1i32;
pub const D3DSGR_NO_CALIBRATION: i32 = 0i32;
pub const D3DSHADE_FLAT: i32 = 1i32;
pub const D3DSHADE_GOURAUD: i32 = 2i32;
pub const D3DSHADE_PHONG: i32 = 3i32;
pub const D3DSHADE_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DSHADER_ADDRESSMODE_SHIFT: u32 = 13u32;
pub const D3DSHADER_ADDRMODE_ABSOLUTE: i32 = 0i32;
pub const D3DSHADER_ADDRMODE_RELATIVE: i32 = 8192i32;
pub const D3DSHADER_ADDRMODE_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DSPC_RESERVED0: i32 = 0i32;
pub const D3DSPC_GT: i32 = 1i32;
pub const D3DSPC_EQ: i32 = 2i32;
pub const D3DSPC_GE: i32 = 3i32;
pub const D3DSPC_LT: i32 = 4i32;
pub const D3DSPC_NE: i32 = 5i32;
pub const D3DSPC_LE: i32 = 6i32;
pub const D3DSPC_RESERVED1: i32 = 7i32;
pub const D3DSHADER_COMPARISON_SHIFT: u32 = 16u32;
pub const D3DSIO_NOP: i32 = 0i32;
pub const D3DSIO_MOV: i32 = 1i32;
pub const D3DSIO_ADD: i32 = 2i32;
pub const D3DSIO_SUB: i32 = 3i32;
pub const D3DSIO_MAD: i32 = 4i32;
pub const D3DSIO_MUL: i32 = 5i32;
pub const D3DSIO_RCP: i32 = 6i32;
pub const D3DSIO_RSQ: i32 = 7i32;
pub const D3DSIO_DP3: i32 = 8i32;
pub const D3DSIO_DP4: i32 = 9i32;
pub const D3DSIO_MIN: i32 = 10i32;
pub const D3DSIO_MAX: i32 = 11i32;
pub const D3DSIO_SLT: i32 = 12i32;
pub const D3DSIO_SGE: i32 = 13i32;
pub const D3DSIO_EXP: i32 = 14i32;
pub const D3DSIO_LOG: i32 = 15i32;
pub const D3DSIO_LIT: i32 = 16i32;
pub const D3DSIO_DST: i32 = 17i32;
pub const D3DSIO_LRP: i32 = 18i32;
pub const D3DSIO_FRC: i32 = 19i32;
pub const D3DSIO_M4x4: i32 = 20i32;
pub const D3DSIO_M4x3: i32 = 21i32;
pub const D3DSIO_M3x4: i32 = 22i32;
pub const D3DSIO_M3x3: i32 = 23i32;
pub const D3DSIO_M3x2: i32 = 24i32;
pub const D3DSIO_CALL: i32 = 25i32;
pub const D3DSIO_CALLNZ: i32 = 26i32;
pub const D3DSIO_LOOP: i32 = 27i32;
pub const D3DSIO_RET: i32 = 28i32;
pub const D3DSIO_ENDLOOP: i32 = 29i32;
pub const D3DSIO_LABEL: i32 = 30i32;
pub const D3DSIO_DCL: i32 = 31i32;
pub const D3DSIO_POW: i32 = 32i32;
pub const D3DSIO_CRS: i32 = 33i32;
pub const D3DSIO_SGN: i32 = 34i32;
pub const D3DSIO_ABS: i32 = 35i32;
pub const D3DSIO_NRM: i32 = 36i32;
pub const D3DSIO_SINCOS: i32 = 37i32;
pub const D3DSIO_REP: i32 = 38i32;
pub const D3DSIO_ENDREP: i32 = 39i32;
pub const D3DSIO_IF: i32 = 40i32;
pub const D3DSIO_IFC: i32 = 41i32;
pub const D3DSIO_ELSE: i32 = 42i32;
pub const D3DSIO_ENDIF: i32 = 43i32;
pub const D3DSIO_BREAK: i32 = 44i32;
pub const D3DSIO_BREAKC: i32 = 45i32;
pub const D3DSIO_MOVA: i32 = 46i32;
pub const D3DSIO_DEFB: i32 = 47i32;
pub const D3DSIO_DEFI: i32 = 48i32;
pub const D3DSIO_TEXCOORD: i32 = 64i32;
pub const D3DSIO_TEXKILL: i32 = 65i32;
pub const D3DSIO_TEX: i32 = 66i32;
pub const D3DSIO_TEXBEM: i32 = 67i32;
pub const D3DSIO_TEXBEML: i32 = 68i32;
pub const D3DSIO_TEXREG2AR: i32 = 69i32;
pub const D3DSIO_TEXREG2GB: i32 = 70i32;
pub const D3DSIO_TEXM3x2PAD: i32 = 71i32;
pub const D3DSIO_TEXM3x2TEX: i32 = 72i32;
pub const D3DSIO_TEXM3x3PAD: i32 = 73i32;
pub const D3DSIO_TEXM3x3TEX: i32 = 74i32;
pub const D3DSIO_RESERVED0: i32 = 75i32;
pub const D3DSIO_TEXM3x3SPEC: i32 = 76i32;
pub const D3DSIO_TEXM3x3VSPEC: i32 = 77i32;
pub const D3DSIO_EXPP: i32 = 78i32;
pub const D3DSIO_LOGP: i32 = 79i32;
pub const D3DSIO_CND: i32 = 80i32;
pub const D3DSIO_DEF: i32 = 81i32;
pub const D3DSIO_TEXREG2RGB: i32 = 82i32;
pub const D3DSIO_TEXDP3TEX: i32 = 83i32;
pub const D3DSIO_TEXM3x2DEPTH: i32 = 84i32;
pub const D3DSIO_TEXDP3: i32 = 85i32;
pub const D3DSIO_TEXM3x3: i32 = 86i32;
pub const D3DSIO_TEXDEPTH: i32 = 87i32;
pub const D3DSIO_CMP: i32 = 88i32;
pub const D3DSIO_BEM: i32 = 89i32;
pub const D3DSIO_DP2ADD: i32 = 90i32;
pub const D3DSIO_DSX: i32 = 91i32;
pub const D3DSIO_DSY: i32 = 92i32;
pub const D3DSIO_TEXLDD: i32 = 93i32;
pub const D3DSIO_SETP: i32 = 94i32;
pub const D3DSIO_TEXLDL: i32 = 95i32;
pub const D3DSIO_BREAKP: i32 = 96i32;
pub const D3DSIO_PHASE: i32 = 65533i32;
pub const D3DSIO_COMMENT: i32 = 65534i32;
pub const D3DSIO_END: i32 = 65535i32;
pub const D3DSIO_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DMP_DEFAULT: i32 = 0i32;
pub const D3DMP_16: i32 = 1i32;
pub const D3DMP_2_8: i32 = 2i32;
pub const D3DSMO_POSITION: i32 = 0i32;
pub const D3DSMO_FACE: i32 = 1i32;
pub const D3DSPR_TEMP: i32 = 0i32;
pub const D3DSPR_INPUT: i32 = 1i32;
pub const D3DSPR_CONST: i32 = 2i32;
pub const D3DSPR_ADDR: i32 = 3i32;
pub const D3DSPR_TEXTURE: i32 = 3i32;
pub const D3DSPR_RASTOUT: i32 = 4i32;
pub const D3DSPR_ATTROUT: i32 = 5i32;
pub const D3DSPR_TEXCRDOUT: i32 = 6i32;
pub const D3DSPR_OUTPUT: i32 = 6i32;
pub const D3DSPR_CONSTINT: i32 = 7i32;
pub const D3DSPR_COLOROUT: i32 = 8i32;
pub const D3DSPR_DEPTHOUT: i32 = 9i32;
pub const D3DSPR_SAMPLER: i32 = 10i32;
pub const D3DSPR_CONST2: i32 = 11i32;
pub const D3DSPR_CONST3: i32 = 12i32;
pub const D3DSPR_CONST4: i32 = 13i32;
pub const D3DSPR_CONSTBOOL: i32 = 14i32;
pub const D3DSPR_LOOP: i32 = 15i32;
pub const D3DSPR_TEMPFLOAT16: i32 = 16i32;
pub const D3DSPR_MISCTYPE: i32 = 17i32;
pub const D3DSPR_LABEL: i32 = 18i32;
pub const D3DSPR_PREDICATE: i32 = 19i32;
pub const D3DSPR_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DSPSM_NONE: i32 = 0i32;
pub const D3DSPSM_NEG: i32 = 16777216i32;
pub const D3DSPSM_BIAS: i32 = 33554432i32;
pub const D3DSPSM_BIASNEG: i32 = 50331648i32;
pub const D3DSPSM_SIGN: i32 = 67108864i32;
pub const D3DSPSM_SIGNNEG: i32 = 83886080i32;
pub const D3DSPSM_COMP: i32 = 100663296i32;
pub const D3DSPSM_X2: i32 = 117440512i32;
pub const D3DSPSM_X2NEG: i32 = 134217728i32;
pub const D3DSPSM_DZ: i32 = 150994944i32;
pub const D3DSPSM_DW: i32 = 167772160i32;
pub const D3DSPSM_ABS: i32 = 184549376i32;
pub const D3DSPSM_ABSNEG: i32 = 201326592i32;
pub const D3DSPSM_NOT: i32 = 218103808i32;
pub const D3DSPSM_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DSI_COISSUE: u32 = 1073741824u32;
pub const D3DSI_COMMENTSIZE_MASK: u32 = 2147418112u32;
pub const D3DSI_COMMENTSIZE_SHIFT: u32 = 16u32;
pub const D3DSI_INSTLENGTH_MASK: u32 = 251658240u32;
pub const D3DSI_INSTLENGTH_SHIFT: u32 = 24u32;
pub const D3DSI_OPCODE_MASK: u32 = 65535u32;
pub const D3DSPD_IUNKNOWN: i32 = 1i32;
pub const D3DSP_DCL_USAGEINDEX_MASK: u32 = 983040u32;
pub const D3DSP_DCL_USAGEINDEX_SHIFT: u32 = 16u32;
pub const D3DSP_DCL_USAGE_MASK: u32 = 15u32;
pub const D3DSP_DCL_USAGE_SHIFT: u32 = 0u32;
pub const D3DSP_DSTMOD_MASK: u32 = 15728640u32;
pub const D3DSP_DSTMOD_SHIFT: u32 = 20u32;
pub const D3DSP_DSTSHIFT_MASK: u32 = 251658240u32;
pub const D3DSP_DSTSHIFT_SHIFT: u32 = 24u32;
pub const D3DSP_MIN_PRECISION_MASK: u32 = 49152u32;
pub const D3DSP_MIN_PRECISION_SHIFT: u32 = 14u32;
pub const D3DSP_OPCODESPECIFICCONTROL_MASK: u32 = 16711680u32;
pub const D3DSP_OPCODESPECIFICCONTROL_SHIFT: u32 = 16u32;
pub const D3DSP_REGNUM_MASK: u32 = 2047u32;
pub const D3DSP_REGTYPE_MASK: u32 = 1879048192u32;
pub const D3DSP_REGTYPE_MASK2: u32 = 6144u32;
pub const D3DSP_REGTYPE_SHIFT: u32 = 28u32;
pub const D3DSP_REGTYPE_SHIFT2: u32 = 8u32;
pub const D3DSP_SRCMOD_MASK: u32 = 251658240u32;
pub const D3DSP_SRCMOD_SHIFT: u32 = 24u32;
pub const D3DSP_SWIZZLE_MASK: u32 = 16711680u32;
pub const D3DSP_SWIZZLE_SHIFT: u32 = 16u32;
pub const D3DSP_TEXTURETYPE_MASK: u32 = 2013265920u32;
pub const D3DSP_TEXTURETYPE_SHIFT: u32 = 27u32;
pub const D3DSP_WRITEMASK_0: u32 = 65536u32;
pub const D3DSP_WRITEMASK_1: u32 = 131072u32;
pub const D3DSP_WRITEMASK_2: u32 = 262144u32;
pub const D3DSP_WRITEMASK_3: u32 = 524288u32;
pub const D3DSP_WRITEMASK_ALL: u32 = 983040u32;
pub const D3DSBT_ALL: i32 = 1i32;
pub const D3DSBT_PIXELSTATE: i32 = 2i32;
pub const D3DSBT_VERTEXSTATE: i32 = 3i32;
pub const D3DSBT_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DSTENCILCAPS_TWOSIDED: i32 = 256i32;
pub const D3DSTENCILOP_KEEP: u32 = 1u32;
pub const D3DSTENCILOP_ZERO: u32 = 2u32;
pub const D3DSTENCILOP_REPLACE: u32 = 3u32;
pub const D3DSTENCILOP_INCRSAT: u32 = 4u32;
pub const D3DSTENCILOP_DECRSAT: u32 = 5u32;
pub const D3DSTENCILOP_INVERT: u32 = 6u32;
pub const D3DSTENCILOP_INCR: u32 = 7u32;
pub const D3DSTENCILOP_DECR: u32 = 8u32;
pub const D3DSTENCILOP_FORCE_DWORD: u32 = 2147483647u32;
pub const D3DSTREAMSOURCE_INDEXEDDATA: u32 = 1073741824u32;
pub const D3DSTREAMSOURCE_INSTANCEDATA: u32 = 2147483648u32;
#[repr(C)]
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
pub const D3DSWAPEFFECT_DISCARD: u32 = 1u32;
pub const D3DSWAPEFFECT_FLIP: u32 = 2u32;
pub const D3DSWAPEFFECT_COPY: u32 = 3u32;
pub const D3DSWAPEFFECT_OVERLAY: u32 = 4u32;
pub const D3DSWAPEFFECT_FLIPEX: u32 = 5u32;
pub const D3DSWAPEFFECT_FORCE_DWORD: u32 = 2147483647u32;
pub const D3DTA_CONSTANT: u32 = 6u32;
pub const D3DTA_TEMP: u32 = 5u32;
pub const D3DTEXOPCAPS_LERP: i32 = 33554432i32;
pub const D3DTEXOPCAPS_MULTIPLYADD: i32 = 16777216i32;
pub const D3DTADDRESS_WRAP: i32 = 1i32;
pub const D3DTADDRESS_MIRROR: i32 = 2i32;
pub const D3DTADDRESS_CLAMP: i32 = 3i32;
pub const D3DTADDRESS_BORDER: i32 = 4i32;
pub const D3DTADDRESS_MIRRORONCE: i32 = 5i32;
pub const D3DTADDRESS_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DTEXF_NONE: i32 = 0i32;
pub const D3DTEXF_POINT: i32 = 1i32;
pub const D3DTEXF_LINEAR: i32 = 2i32;
pub const D3DTEXF_ANISOTROPIC: i32 = 3i32;
pub const D3DTEXF_PYRAMIDALQUAD: i32 = 6i32;
pub const D3DTEXF_GAUSSIANQUAD: i32 = 7i32;
pub const D3DTEXF_CONVOLUTIONMONO: i32 = 8i32;
pub const D3DTEXF_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DTOP_DISABLE: i32 = 1i32;
pub const D3DTOP_SELECTARG1: i32 = 2i32;
pub const D3DTOP_SELECTARG2: i32 = 3i32;
pub const D3DTOP_MODULATE: i32 = 4i32;
pub const D3DTOP_MODULATE2X: i32 = 5i32;
pub const D3DTOP_MODULATE4X: i32 = 6i32;
pub const D3DTOP_ADD: i32 = 7i32;
pub const D3DTOP_ADDSIGNED: i32 = 8i32;
pub const D3DTOP_ADDSIGNED2X: i32 = 9i32;
pub const D3DTOP_SUBTRACT: i32 = 10i32;
pub const D3DTOP_ADDSMOOTH: i32 = 11i32;
pub const D3DTOP_BLENDDIFFUSEALPHA: i32 = 12i32;
pub const D3DTOP_BLENDTEXTUREALPHA: i32 = 13i32;
pub const D3DTOP_BLENDFACTORALPHA: i32 = 14i32;
pub const D3DTOP_BLENDTEXTUREALPHAPM: i32 = 15i32;
pub const D3DTOP_BLENDCURRENTALPHA: i32 = 16i32;
pub const D3DTOP_PREMODULATE: i32 = 17i32;
pub const D3DTOP_MODULATEALPHA_ADDCOLOR: i32 = 18i32;
pub const D3DTOP_MODULATECOLOR_ADDALPHA: i32 = 19i32;
pub const D3DTOP_MODULATEINVALPHA_ADDCOLOR: i32 = 20i32;
pub const D3DTOP_MODULATEINVCOLOR_ADDALPHA: i32 = 21i32;
pub const D3DTOP_BUMPENVMAP: i32 = 22i32;
pub const D3DTOP_BUMPENVMAPLUMINANCE: i32 = 23i32;
pub const D3DTOP_DOTPRODUCT3: i32 = 24i32;
pub const D3DTOP_MULTIPLYADD: i32 = 25i32;
pub const D3DTOP_LERP: i32 = 26i32;
pub const D3DTOP_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DTSS_COLOROP: i32 = 1i32;
pub const D3DTSS_COLORARG1: i32 = 2i32;
pub const D3DTSS_COLORARG2: i32 = 3i32;
pub const D3DTSS_ALPHAOP: i32 = 4i32;
pub const D3DTSS_ALPHAARG1: i32 = 5i32;
pub const D3DTSS_ALPHAARG2: i32 = 6i32;
pub const D3DTSS_BUMPENVMAT00: i32 = 7i32;
pub const D3DTSS_BUMPENVMAT01: i32 = 8i32;
pub const D3DTSS_BUMPENVMAT10: i32 = 9i32;
pub const D3DTSS_BUMPENVMAT11: i32 = 10i32;
pub const D3DTSS_TEXCOORDINDEX: i32 = 11i32;
pub const D3DTSS_BUMPENVLSCALE: i32 = 22i32;
pub const D3DTSS_BUMPENVLOFFSET: i32 = 23i32;
pub const D3DTSS_TEXTURETRANSFORMFLAGS: i32 = 24i32;
pub const D3DTSS_COLORARG0: i32 = 26i32;
pub const D3DTSS_ALPHAARG0: i32 = 27i32;
pub const D3DTSS_RESULTARG: i32 = 28i32;
pub const D3DTSS_CONSTANT: i32 = 32i32;
pub const D3DTSS_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DTTFF_DISABLE: i32 = 0i32;
pub const D3DTTFF_COUNT1: i32 = 1i32;
pub const D3DTTFF_COUNT2: i32 = 2i32;
pub const D3DTTFF_COUNT3: i32 = 3i32;
pub const D3DTTFF_COUNT4: i32 = 4i32;
pub const D3DTTFF_PROJECTED: i32 = 256i32;
pub const D3DTTFF_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DTS_VIEW: i32 = 2i32;
pub const D3DTS_PROJECTION: i32 = 3i32;
pub const D3DTS_TEXTURE0: i32 = 16i32;
pub const D3DTS_TEXTURE1: i32 = 17i32;
pub const D3DTS_TEXTURE2: i32 = 18i32;
pub const D3DTS_TEXTURE3: i32 = 19i32;
pub const D3DTS_TEXTURE4: i32 = 20i32;
pub const D3DTS_TEXTURE5: i32 = 21i32;
pub const D3DTS_TEXTURE6: i32 = 22i32;
pub const D3DTS_TEXTURE7: i32 = 23i32;
pub const D3DTS_FORCE_DWORD: i32 = 2147483647i32;
#[repr(C)]
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
pub const D3DTSS_TCI_SPHEREMAP: u32 = 262144u32;
pub const D3DUSAGE_AUTOGENMIPMAP: i32 = 1024i32;
pub const D3DUSAGE_DEPTHSTENCIL: i32 = 2i32;
pub const D3DUSAGE_DMAP: i32 = 16384i32;
pub const D3DUSAGE_DONOTCLIP: i32 = 32i32;
pub const D3DUSAGE_DYNAMIC: i32 = 512i32;
pub const D3DUSAGE_NONSECURE: i32 = 8388608i32;
pub const D3DUSAGE_NPATCHES: i32 = 256i32;
pub const D3DUSAGE_POINTS: i32 = 64i32;
pub const D3DUSAGE_QUERY_FILTER: i32 = 131072i32;
pub const D3DUSAGE_QUERY_LEGACYBUMPMAP: i32 = 32768i32;
pub const D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING: i32 = 524288i32;
pub const D3DUSAGE_QUERY_SRGBREAD: i32 = 65536i32;
pub const D3DUSAGE_QUERY_SRGBWRITE: i32 = 262144i32;
pub const D3DUSAGE_QUERY_VERTEXTEXTURE: i32 = 1048576i32;
pub const D3DUSAGE_QUERY_WRAPANDMIP: i32 = 2097152i32;
pub const D3DUSAGE_RENDERTARGET: i32 = 1i32;
pub const D3DUSAGE_RESTRICTED_CONTENT: i32 = 2048i32;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE: i32 = 8192i32;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER: i32 = 4096i32;
pub const D3DUSAGE_RTPATCHES: i32 = 128i32;
pub const D3DUSAGE_SOFTWAREPROCESSING: i32 = 16i32;
pub const D3DUSAGE_TEXTAPI: i32 = 268435456i32;
pub const D3DUSAGE_WRITEONLY: i32 = 8i32;
pub const D3DVBF_DISABLE: i32 = 0i32;
pub const D3DVBF_1WEIGHTS: i32 = 1i32;
pub const D3DVBF_2WEIGHTS: i32 = 2i32;
pub const D3DVBF_3WEIGHTS: i32 = 3i32;
pub const D3DVBF_TWEENING: i32 = 255i32;
pub const D3DVBF_0WEIGHTS: i32 = 256i32;
pub const D3DVBF_FORCE_DWORD: i32 = 2147483647i32;
#[repr(C)]
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
#[repr(C)]
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
pub const D3DVERTEXTEXTURESAMPLER0: u32 = 257u32;
pub const D3DVERTEXTEXTURESAMPLER1: u32 = 258u32;
pub const D3DVERTEXTEXTURESAMPLER2: u32 = 259u32;
pub const D3DVERTEXTEXTURESAMPLER3: u32 = 260u32;
#[repr(C)]
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
#[repr(C)]
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
pub const D3DVS20CAPS_PREDICATION: u32 = 1u32;
pub const D3DVS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24u32;
pub const D3DVS20_MAX_NUMTEMPS: u32 = 32u32;
pub const D3DVS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4u32;
pub const D3DVS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0u32;
pub const D3DVS20_MIN_NUMTEMPS: u32 = 12u32;
pub const D3DVS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 1u32;
#[repr(C)]
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
pub const D3DVS_ADDRESSMODE_SHIFT: u32 = 13u32;
pub const D3DVS_ADDRMODE_ABSOLUTE: i32 = 0i32;
pub const D3DVS_ADDRMODE_RELATIVE: i32 = 8192i32;
pub const D3DVS_ADDRMODE_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DSRO_POSITION: i32 = 0i32;
pub const D3DSRO_FOG: i32 = 1i32;
pub const D3DSRO_POINT_SIZE: i32 = 2i32;
pub const D3DSRO_FORCE_DWORD: i32 = 2147483647i32;
pub const D3DVS_SWIZZLE_MASK: u32 = 16711680u32;
pub const D3DVS_SWIZZLE_SHIFT: u32 = 16u32;
pub const D3DVTXPCAPS_NO_TEXGEN_NONLOCALVIEWER: i32 = 512i32;
pub const D3DVTXPCAPS_TEXGEN_SPHEREMAP: i32 = 256i32;
pub const D3DVTXPCAPS_TWEENING: i32 = 64i32;
pub const D3DWRAP_W: i32 = 4i32;
pub const D3DZB_FALSE: i32 = 0i32;
pub const D3DZB_TRUE: i32 = 1i32;
pub const D3DZB_USEW: i32 = 2i32;
pub const D3DZB_FORCE_DWORD: i32 = 2147483647i32;
pub const D3D_MAX_SIMULTANEOUS_RENDERTARGETS: u32 = 4u32;
#[repr(C)]
pub struct D3D_OMAC {
    pub Omac: [u8; 16],
}
impl ::core::marker::Copy for D3D_OMAC {}
impl ::core::clone::Clone for D3D_OMAC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D_OMAC_SIZE: u32 = 16u32;
pub const D3D_SDK_VERSION: u32 = 32u32;
#[repr(transparent)]
pub struct IDirect3D9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3D9 {}
impl ::core::clone::Clone for IDirect3D9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3D9Ex(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3D9Ex {}
impl ::core::clone::Clone for IDirect3D9Ex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DBaseTexture9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DBaseTexture9 {}
impl ::core::clone::Clone for IDirect3DBaseTexture9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DCubeTexture9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DCubeTexture9 {}
impl ::core::clone::Clone for IDirect3DCubeTexture9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DDevice9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DDevice9 {}
impl ::core::clone::Clone for IDirect3DDevice9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DDevice9Ex(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DDevice9Ex {}
impl ::core::clone::Clone for IDirect3DDevice9Ex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DIndexBuffer9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DIndexBuffer9 {}
impl ::core::clone::Clone for IDirect3DIndexBuffer9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DPixelShader9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DPixelShader9 {}
impl ::core::clone::Clone for IDirect3DPixelShader9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DQuery9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DQuery9 {}
impl ::core::clone::Clone for IDirect3DQuery9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DResource9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DResource9 {}
impl ::core::clone::Clone for IDirect3DResource9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DStateBlock9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DStateBlock9 {}
impl ::core::clone::Clone for IDirect3DStateBlock9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DSurface9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DSurface9 {}
impl ::core::clone::Clone for IDirect3DSurface9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DSwapChain9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DSwapChain9 {}
impl ::core::clone::Clone for IDirect3DSwapChain9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DSwapChain9Ex(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DSwapChain9Ex {}
impl ::core::clone::Clone for IDirect3DSwapChain9Ex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DTexture9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DTexture9 {}
impl ::core::clone::Clone for IDirect3DTexture9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DVertexBuffer9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DVertexBuffer9 {}
impl ::core::clone::Clone for IDirect3DVertexBuffer9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DVertexDeclaration9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DVertexDeclaration9 {}
impl ::core::clone::Clone for IDirect3DVertexDeclaration9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DVertexShader9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DVertexShader9 {}
impl ::core::clone::Clone for IDirect3DVertexShader9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DVolume9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DVolume9 {}
impl ::core::clone::Clone for IDirect3DVolume9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3DVolumeTexture9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DVolumeTexture9 {}
impl ::core::clone::Clone for IDirect3DVolumeTexture9 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAXD3DDECLLENGTH: u32 = 64u32;
pub const MAXD3DDECLUSAGEINDEX: u32 = 15u32;
pub const MAX_DEVICE_IDENTIFIER_STRING: u32 = 512u32;
pub const _FACD3D: u32 = 2166u32;
