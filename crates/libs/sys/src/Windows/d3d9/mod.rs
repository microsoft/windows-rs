#[cfg(feature = "dsound")]
windows_link::link!("d3d9.dll" "system" fn D3DPERF_BeginEvent(col : super::dsound::D3DCOLOR, wszname : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("d3d9.dll" "system" fn D3DPERF_EndEvent() -> i32);
windows_link::link!("d3d9.dll" "system" fn D3DPERF_GetStatus() -> u32);
windows_link::link!("d3d9.dll" "system" fn D3DPERF_QueryRepeatFrame() -> windows_sys::core::BOOL);
#[cfg(feature = "dsound")]
windows_link::link!("d3d9.dll" "system" fn D3DPERF_SetMarker(col : super::dsound::D3DCOLOR, wszname : windows_sys::core::PCWSTR));
windows_link::link!("d3d9.dll" "system" fn D3DPERF_SetOptions(dwoptions : u32));
#[cfg(feature = "dsound")]
windows_link::link!("d3d9.dll" "system" fn D3DPERF_SetRegion(col : super::dsound::D3DCOLOR, wszname : windows_sys::core::PCWSTR));
windows_link::link!("d3d9.dll" "system" fn Direct3DCreate9(sdkversion : u32) -> *mut core::ffi::c_void);
windows_link::link!("d3d9.dll" "system" fn Direct3DCreate9Ex(sdkversion : u32, param1 : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const D3D9_RESOURCE_PRIORITY_HIGH: u32 = 2684354560;
pub const D3D9_RESOURCE_PRIORITY_LOW: u32 = 1342177280;
pub const D3D9_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200;
pub const D3D9_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640;
pub const D3D9_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920;
pub const D3D9b_SDK_VERSION: u32 = 31;
pub const D3DADAPTER_DEFAULT: u32 = 0;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [i8; 512],
    pub Description: [i8; 512],
    pub DeviceName: [i8; 32],
    pub DriverVersion: i64,
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DeviceIdentifier: windows_sys::core::GUID,
    pub WHQLLevel: u32,
}
#[cfg(target_arch = "x86")]
impl Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [i8; 512],
    pub Description: [i8; 512],
    pub DeviceName: [i8; 32],
    pub DriverVersion: i64,
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DeviceIdentifier: windows_sys::core::GUID,
    pub WHQLLevel: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct D3DAES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
pub type D3DAUTHENTICATEDCHANNELTYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub DXVA2DecodeHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub StartSequenceQuery: u32,
    pub StartSequenceConfigure: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub Protections: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub ProcessIdentiferType: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::winnt::HANDLE,
    pub AllowAccess: windows_sys::core::BOOL,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub EncryptionGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_sys::core::HRESULT,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DAUTHENTICATEDCHANNEL_D3D9: D3DAUTHENTICATEDCHANNELTYPE = 1;
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_HARDWARE: D3DAUTHENTICATEDCHANNELTYPE = 3;
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_SOFTWARE: D3DAUTHENTICATEDCHANNELTYPE = 2;
pub type D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0,
}
impl Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DXVA2DecodeHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DXVA2DecodeHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumEncryptionGuids: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub EncryptionGuidIndex: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub BusType: D3DBUSTYPE,
    pub bAccessibleInContiguousBlocks: windows_sys::core::BOOL,
    pub bAccessibleInNonContiguousBlocks: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub NumOutputIDs: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProtectionFlags: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumRestrictedSharedResourceProcesses: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub ProcessIndex: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProcessIndex: u32,
    pub ProcessIdentifer: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumUnrestrictedProtectedSharedResources: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    pub QueryType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    pub omac: D3D_OMAC,
    pub QueryType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_sys::core::HRESULT,
}
#[cfg(feature = "winnt")]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DAUTHENTICATEDCONFIGURE_CRYPTOSESSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6346cc54_2cfc_4ad4_8224_d15837de7700);
pub const D3DAUTHENTICATEDCONFIGURE_ENCRYPTIONWHENACCESSIBLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41fff286_6ae0_4d43_9d55_a46e9efd158a);
pub const D3DAUTHENTICATEDCONFIGURE_INITIALIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06114bdb_3523_470a_8dca_fbc2845154f0);
pub const D3DAUTHENTICATEDCONFIGURE_PROTECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50455658_3f47_4362_bf99_bfdfcde9ed29);
pub const D3DAUTHENTICATEDCONFIGURE_SHAREDRESOURCE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0772d047_1b40_48e8_9ca6_b5f510de9f01);
pub const D3DAUTHENTICATEDQUERY_ACCESSIBILITYATTRIBUTES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6214d9d2_432c_4abb_9fce_216eea269e3b);
pub const D3DAUTHENTICATEDQUERY_CHANNELTYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc1b18a5_b1fb_42ab_bd94_b5828b4bf7be);
pub const D3DAUTHENTICATEDQUERY_CRYPTOSESSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2634499e_d018_4d74_ac17_7f724059528d);
pub const D3DAUTHENTICATEDQUERY_CURRENTENCRYPTIONWHENACCESSIBLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec1791c7_dad3_4f15_9ec3_faa93d60d4f0);
pub const D3DAUTHENTICATEDQUERY_DEVICEHANDLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec1c539d_8cff_4e2a_bcc4_f5692f99f480);
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf83a5958_e986_4bda_beb0_411f6a7a01b7);
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUIDCOUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb30f7066_203c_4b07_93fc_ceaafd61241e);
pub const D3DAUTHENTICATEDQUERY_OUTPUTID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x839ddca3_9b4e_41e4_b053_892bd2a11ee7);
pub const D3DAUTHENTICATEDQUERY_OUTPUTIDCOUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2c042b5e_8c07_46d5_aabe_8f75cbad4c31);
pub const D3DAUTHENTICATEDQUERY_PROTECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa84eb584_c495_48aa_b94d_8bd2d6fbce05);
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x649bbadb_f0f4_4639_a15b_24393fc3abac);
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESSCOUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0db207b3_9450_46a6_82de_1b96d44f9cf2);
pub const D3DAUTHENTICATEDQUERY_UNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x012f0bd6_e662_4474_befd_aa53e5143c6d);
pub type D3DBACKBUFFER_TYPE = i32;
pub const D3DBACKBUFFER_TYPE_FORCE_DWORD: D3DBACKBUFFER_TYPE = 2147483647;
pub const D3DBACKBUFFER_TYPE_LEFT: D3DBACKBUFFER_TYPE = 1;
pub const D3DBACKBUFFER_TYPE_MONO: D3DBACKBUFFER_TYPE = 0;
pub const D3DBACKBUFFER_TYPE_RIGHT: D3DBACKBUFFER_TYPE = 2;
pub type D3DBASISTYPE = i32;
pub const D3DBASIS_BEZIER: D3DBASISTYPE = 0;
pub const D3DBASIS_BSPLINE: D3DBASISTYPE = 1;
pub const D3DBASIS_CATMULL_ROM: D3DBASISTYPE = 2;
pub const D3DBASIS_FORCE_DWORD: D3DBASISTYPE = 2147483647;
pub type D3DBLEND = i32;
pub type D3DBLENDOP = i32;
pub const D3DBLENDOP_ADD: D3DBLENDOP = 1;
pub const D3DBLENDOP_FORCE_DWORD: D3DBLENDOP = 2147483647;
pub const D3DBLENDOP_MAX: D3DBLENDOP = 5;
pub const D3DBLENDOP_MIN: D3DBLENDOP = 4;
pub const D3DBLENDOP_REVSUBTRACT: D3DBLENDOP = 3;
pub const D3DBLENDOP_SUBTRACT: D3DBLENDOP = 2;
pub const D3DBLEND_BLENDFACTOR: D3DBLEND = 14;
pub const D3DBLEND_BOTHINVSRCALPHA: D3DBLEND = 13;
pub const D3DBLEND_BOTHSRCALPHA: D3DBLEND = 12;
pub const D3DBLEND_DESTALPHA: D3DBLEND = 7;
pub const D3DBLEND_DESTCOLOR: D3DBLEND = 9;
pub const D3DBLEND_FORCE_DWORD: D3DBLEND = 2147483647;
pub const D3DBLEND_INVBLENDFACTOR: D3DBLEND = 15;
pub const D3DBLEND_INVDESTALPHA: D3DBLEND = 8;
pub const D3DBLEND_INVDESTCOLOR: D3DBLEND = 10;
pub const D3DBLEND_INVSRCALPHA: D3DBLEND = 6;
pub const D3DBLEND_INVSRCCOLOR: D3DBLEND = 4;
pub const D3DBLEND_INVSRCCOLOR2: D3DBLEND = 17;
pub const D3DBLEND_ONE: D3DBLEND = 2;
pub const D3DBLEND_SRCALPHA: D3DBLEND = 5;
pub const D3DBLEND_SRCALPHASAT: D3DBLEND = 11;
pub const D3DBLEND_SRCCOLOR: D3DBLEND = 3;
pub const D3DBLEND_SRCCOLOR2: D3DBLEND = 16;
pub const D3DBLEND_ZERO: D3DBLEND = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DBOX {
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub Front: u32,
    pub Back: u32,
}
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR: D3DBUSTYPE = 262144;
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: D3DBUSTYPE = 327680;
pub const D3DBUSIMPL_MODIFIER_INSIDE_OF_CHIPSET: D3DBUSTYPE = 65536;
pub const D3DBUSIMPL_MODIFIER_NON_STANDARD: D3DBUSTYPE = -2147483648;
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: D3DBUSTYPE = 131072;
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: D3DBUSTYPE = 196608;
pub type D3DBUSTYPE = i32;
pub const D3DBUSTYPE_AGP: D3DBUSTYPE = 4;
pub const D3DBUSTYPE_OTHER: D3DBUSTYPE = 0;
pub const D3DBUSTYPE_PCI: D3DBUSTYPE = 1;
pub const D3DBUSTYPE_PCIEXPRESS: D3DBUSTYPE = 3;
pub const D3DBUSTYPE_PCIX: D3DBUSTYPE = 2;
pub const D3DCAPS2_CANAUTOGENMIPMAP: u32 = 1073741824;
pub const D3DCAPS2_CANCALIBRATEGAMMA: u32 = 1048576;
pub const D3DCAPS2_CANMANAGERESOURCE: u32 = 268435456;
pub const D3DCAPS2_CANSHARERESOURCE: u32 = 2147483648;
pub const D3DCAPS2_DYNAMICTEXTURES: u32 = 536870912;
pub const D3DCAPS2_FULLSCREENGAMMA: u32 = 131072;
pub const D3DCAPS2_RESERVED: u32 = 33554432;
pub const D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD: u32 = 32;
pub const D3DCAPS3_COPY_TO_SYSTEMMEM: u32 = 512;
pub const D3DCAPS3_COPY_TO_VIDMEM: u32 = 256;
pub const D3DCAPS3_DXVAHD: u32 = 1024;
pub const D3DCAPS3_DXVAHD_LIMITED: u32 = 2048;
pub const D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION: u32 = 128;
pub const D3DCAPS3_RESERVED: u32 = 2147483679;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const D3DCAPS_OVERLAY: u32 = 2048;
pub const D3DCAPS_READ_SCANLINE: u32 = 131072;
pub const D3DCLEAR_STENCIL: u32 = 4;
pub const D3DCLEAR_TARGET: u32 = 1;
pub const D3DCLEAR_ZBUFFER: u32 = 2;
pub const D3DCLIPPLANE0: u32 = 1;
pub const D3DCLIPPLANE1: u32 = 2;
pub const D3DCLIPPLANE2: u32 = 4;
pub const D3DCLIPPLANE3: u32 = 8;
pub const D3DCLIPPLANE4: u32 = 16;
pub const D3DCLIPPLANE5: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DCLIPSTATUS9 {
    pub ClipUnion: u32,
    pub ClipIntersection: u32,
}
pub type D3DCMPFUNC = i32;
pub const D3DCMP_ALWAYS: D3DCMPFUNC = 8;
pub const D3DCMP_EQUAL: D3DCMPFUNC = 3;
pub const D3DCMP_FORCE_DWORD: D3DCMPFUNC = 2147483647;
pub const D3DCMP_GREATER: D3DCMPFUNC = 5;
pub const D3DCMP_GREATEREQUAL: D3DCMPFUNC = 7;
pub const D3DCMP_LESS: D3DCMPFUNC = 2;
pub const D3DCMP_LESSEQUAL: D3DCMPFUNC = 4;
pub const D3DCMP_NEVER: D3DCMPFUNC = 1;
pub const D3DCMP_NOTEQUAL: D3DCMPFUNC = 6;
pub const D3DCOLORWRITEENABLE_ALPHA: u32 = 8;
pub const D3DCOLORWRITEENABLE_BLUE: u32 = 4;
pub const D3DCOLORWRITEENABLE_GREEN: u32 = 2;
pub const D3DCOLORWRITEENABLE_RED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DCOMPOSERECTDESC {
    pub X: u16,
    pub Y: u16,
    pub Width: u16,
    pub Height: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DCOMPOSERECTDESTINATION {
    pub SrcRectIndex: u16,
    pub Reserved: u16,
    pub X: i16,
    pub Y: i16,
}
pub type D3DCOMPOSERECTSOP = i32;
pub const D3DCOMPOSERECTS_AND: D3DCOMPOSERECTSOP = 3;
pub const D3DCOMPOSERECTS_COPY: D3DCOMPOSERECTSOP = 1;
pub const D3DCOMPOSERECTS_FORCE_DWORD: D3DCOMPOSERECTSOP = 2147483647;
pub const D3DCOMPOSERECTS_MAXNUMRECTS: u32 = 65535;
pub const D3DCOMPOSERECTS_NEG: D3DCOMPOSERECTSOP = 4;
pub const D3DCOMPOSERECTS_OR: D3DCOMPOSERECTSOP = 2;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct D3DCONTENTPROTECTIONCAPS {
    pub Caps: u32,
    pub KeyExchangeType: windows_sys::core::GUID,
    pub BufferAlignmentStart: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct D3DCONTENTPROTECTIONCAPS {
    pub Caps: u32,
    pub KeyExchangeType: windows_sys::core::GUID,
    pub BufferAlignmentStart: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
pub const D3DCONVOLUTIONMONO_MAXHEIGHT: u32 = 7;
pub const D3DCONVOLUTIONMONO_MAXWIDTH: u32 = 7;
pub const D3DCPCAPS_CONTENTKEY: u32 = 16;
pub const D3DCPCAPS_ENCRYPTEDREADBACK: u32 = 64;
pub const D3DCPCAPS_ENCRYPTEDREADBACKKEY: u32 = 128;
pub const D3DCPCAPS_ENCRYPTSLICEDATAONLY: u32 = 512;
pub const D3DCPCAPS_FRESHENSESSIONKEY: u32 = 32;
pub const D3DCPCAPS_HARDWARE: u32 = 2;
pub const D3DCPCAPS_PARTIALDECRYPTION: u32 = 8;
pub const D3DCPCAPS_PROTECTIONALWAYSON: u32 = 4;
pub const D3DCPCAPS_SEQUENTIAL_CTR_IV: u32 = 256;
pub const D3DCPCAPS_SOFTWARE: u32 = 1;
pub const D3DCREATE_ADAPTERGROUP_DEVICE: u32 = 512;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: u32 = 256;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: u32 = 1024;
pub const D3DCREATE_DISABLE_PRINTSCREEN: u32 = 32768;
pub const D3DCREATE_DISABLE_PSGP_THREADING: u32 = 8192;
pub const D3DCREATE_ENABLE_PRESENTSTATS: u32 = 16384;
pub const D3DCREATE_FPU_PRESERVE: u32 = 2;
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: u32 = 64;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: u32 = 128;
pub const D3DCREATE_MULTITHREADED: u32 = 4;
pub const D3DCREATE_NOWINDOWCHANGES: u32 = 2048;
pub const D3DCREATE_PUREDEVICE: u32 = 16;
pub const D3DCREATE_SCREENSAVER: u32 = 268435456;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: u32 = 32;
pub const D3DCRYPTOTYPE_AES128_CTR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9b6bd711_4f74_41c9_9e7b_0be2d7d93b4f);
pub const D3DCRYPTOTYPE_PROPRIETARY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab4e9afd_1d1c_46e6_a72f_0869917b0de8);
pub const D3DCS_ALL: u32 = 4095;
pub const D3DCS_BACK: u32 = 32;
pub const D3DCS_BOTTOM: u32 = 8;
pub const D3DCS_FRONT: u32 = 16;
pub const D3DCS_LEFT: u32 = 1;
pub const D3DCS_PLANE0: u32 = 64;
pub const D3DCS_PLANE1: u32 = 128;
pub const D3DCS_PLANE2: u32 = 256;
pub const D3DCS_PLANE3: u32 = 512;
pub const D3DCS_PLANE4: u32 = 1024;
pub const D3DCS_PLANE5: u32 = 2048;
pub const D3DCS_RIGHT: u32 = 2;
pub const D3DCS_TOP: u32 = 4;
pub type D3DCUBEMAP_FACES = i32;
pub const D3DCUBEMAP_FACE_FORCE_DWORD: D3DCUBEMAP_FACES = 2147483647;
pub const D3DCUBEMAP_FACE_NEGATIVE_X: D3DCUBEMAP_FACES = 1;
pub const D3DCUBEMAP_FACE_NEGATIVE_Y: D3DCUBEMAP_FACES = 3;
pub const D3DCUBEMAP_FACE_NEGATIVE_Z: D3DCUBEMAP_FACES = 5;
pub const D3DCUBEMAP_FACE_POSITIVE_X: D3DCUBEMAP_FACES = 0;
pub const D3DCUBEMAP_FACE_POSITIVE_Y: D3DCUBEMAP_FACES = 2;
pub const D3DCUBEMAP_FACE_POSITIVE_Z: D3DCUBEMAP_FACES = 4;
pub type D3DCULL = i32;
pub const D3DCULL_CCW: D3DCULL = 3;
pub const D3DCULL_CW: D3DCULL = 2;
pub const D3DCULL_FORCE_DWORD: D3DCULL = 2147483647;
pub const D3DCULL_NONE: D3DCULL = 1;
pub const D3DCURSORCAPS_COLOR: u32 = 1;
pub const D3DCURSORCAPS_LOWRES: u32 = 2;
pub const D3DCURSOR_IMMEDIATE_UPDATE: u32 = 1;
pub type D3DDEBUGMONITORTOKENS = i32;
pub type D3DDECLMETHOD = i32;
pub const D3DDECLMETHOD_CROSSUV: D3DDECLMETHOD = 3;
pub const D3DDECLMETHOD_DEFAULT: D3DDECLMETHOD = 0;
pub const D3DDECLMETHOD_LOOKUP: D3DDECLMETHOD = 5;
pub const D3DDECLMETHOD_LOOKUPPRESAMPLED: D3DDECLMETHOD = 6;
pub const D3DDECLMETHOD_PARTIALU: D3DDECLMETHOD = 1;
pub const D3DDECLMETHOD_PARTIALV: D3DDECLMETHOD = 2;
pub const D3DDECLMETHOD_UV: D3DDECLMETHOD = 4;
pub type D3DDECLTYPE = i32;
pub const D3DDECLTYPE_D3DCOLOR: D3DDECLTYPE = 4;
pub const D3DDECLTYPE_DEC3N: D3DDECLTYPE = 14;
pub const D3DDECLTYPE_FLOAT1: D3DDECLTYPE = 0;
pub const D3DDECLTYPE_FLOAT16_2: D3DDECLTYPE = 15;
pub const D3DDECLTYPE_FLOAT16_4: D3DDECLTYPE = 16;
pub const D3DDECLTYPE_FLOAT2: D3DDECLTYPE = 1;
pub const D3DDECLTYPE_FLOAT3: D3DDECLTYPE = 2;
pub const D3DDECLTYPE_FLOAT4: D3DDECLTYPE = 3;
pub const D3DDECLTYPE_SHORT2: D3DDECLTYPE = 6;
pub const D3DDECLTYPE_SHORT2N: D3DDECLTYPE = 9;
pub const D3DDECLTYPE_SHORT4: D3DDECLTYPE = 7;
pub const D3DDECLTYPE_SHORT4N: D3DDECLTYPE = 10;
pub const D3DDECLTYPE_UBYTE4: D3DDECLTYPE = 5;
pub const D3DDECLTYPE_UBYTE4N: D3DDECLTYPE = 8;
pub const D3DDECLTYPE_UDEC3: D3DDECLTYPE = 13;
pub const D3DDECLTYPE_UNUSED: D3DDECLTYPE = 17;
pub const D3DDECLTYPE_USHORT2N: D3DDECLTYPE = 11;
pub const D3DDECLTYPE_USHORT4N: D3DDECLTYPE = 12;
pub type D3DDECLUSAGE = i32;
pub const D3DDECLUSAGE_BINORMAL: D3DDECLUSAGE = 7;
pub const D3DDECLUSAGE_BLENDINDICES: D3DDECLUSAGE = 2;
pub const D3DDECLUSAGE_BLENDWEIGHT: D3DDECLUSAGE = 1;
pub const D3DDECLUSAGE_COLOR: D3DDECLUSAGE = 10;
pub const D3DDECLUSAGE_DEPTH: D3DDECLUSAGE = 12;
pub const D3DDECLUSAGE_FOG: D3DDECLUSAGE = 11;
pub const D3DDECLUSAGE_NORMAL: D3DDECLUSAGE = 3;
pub const D3DDECLUSAGE_POSITION: D3DDECLUSAGE = 0;
pub const D3DDECLUSAGE_POSITIONT: D3DDECLUSAGE = 9;
pub const D3DDECLUSAGE_PSIZE: D3DDECLUSAGE = 4;
pub const D3DDECLUSAGE_SAMPLE: D3DDECLUSAGE = 13;
pub const D3DDECLUSAGE_TANGENT: D3DDECLUSAGE = 6;
pub const D3DDECLUSAGE_TESSFACTOR: D3DDECLUSAGE = 8;
pub const D3DDECLUSAGE_TEXCOORD: D3DDECLUSAGE = 5;
pub type D3DDEGREETYPE = i32;
pub const D3DDEGREE_CUBIC: D3DDEGREETYPE = 3;
pub const D3DDEGREE_FORCE_DWORD: D3DDEGREETYPE = 2147483647;
pub const D3DDEGREE_LINEAR: D3DDEGREETYPE = 1;
pub const D3DDEGREE_QUADRATIC: D3DDEGREETYPE = 2;
pub const D3DDEGREE_QUINTIC: D3DDEGREETYPE = 5;
pub const D3DDEVCAPS2_ADAPTIVETESSNPATCH: u32 = 8;
pub const D3DDEVCAPS2_ADAPTIVETESSRTPATCH: u32 = 4;
pub const D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES: u32 = 16;
pub const D3DDEVCAPS2_DMAPNPATCH: u32 = 2;
pub const D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH: u32 = 32;
pub const D3DDEVCAPS2_STREAMOFFSET: u32 = 1;
pub const D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET: u32 = 64;
pub const D3DDEVCAPS_CANBLTSYSTONONLOCAL: u32 = 131072;
pub const D3DDEVCAPS_CANRENDERAFTERFLIP: u32 = 2048;
pub const D3DDEVCAPS_DRAWPRIMITIVES2: u32 = 8192;
pub const D3DDEVCAPS_DRAWPRIMITIVES2EX: u32 = 32768;
pub const D3DDEVCAPS_DRAWPRIMTLVERTEX: u32 = 1024;
pub const D3DDEVCAPS_EXECUTESYSTEMMEMORY: u32 = 16;
pub const D3DDEVCAPS_EXECUTEVIDEOMEMORY: u32 = 32;
pub const D3DDEVCAPS_HWRASTERIZATION: u32 = 524288;
pub const D3DDEVCAPS_HWTRANSFORMANDLIGHT: u32 = 65536;
pub const D3DDEVCAPS_NPATCHES: u32 = 16777216;
pub const D3DDEVCAPS_PUREDEVICE: u32 = 1048576;
pub const D3DDEVCAPS_QUINTICRTPATCHES: u32 = 2097152;
pub const D3DDEVCAPS_RTPATCHES: u32 = 4194304;
pub const D3DDEVCAPS_RTPATCHHANDLEZERO: u32 = 8388608;
pub const D3DDEVCAPS_SEPARATETEXTUREMEMORIES: u32 = 16384;
pub const D3DDEVCAPS_TEXTURENONLOCALVIDMEM: u32 = 4096;
pub const D3DDEVCAPS_TEXTURESYSTEMMEMORY: u32 = 256;
pub const D3DDEVCAPS_TEXTUREVIDEOMEMORY: u32 = 512;
pub const D3DDEVCAPS_TLVERTEXSYSTEMMEMORY: u32 = 64;
pub const D3DDEVCAPS_TLVERTEXVIDEOMEMORY: u32 = 128;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct D3DDEVICE_CREATION_PARAMETERS {
    pub AdapterOrdinal: u32,
    pub DeviceType: D3DDEVTYPE,
    pub hFocusWindow: super::windef::HWND,
    pub BehaviorFlags: u32,
}
#[cfg(feature = "windef")]
impl Default for D3DDEVICE_CREATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    pub MaxBandwidthUtilized: f32,
    pub FrontEndUploadMemoryUtilizedPercent: f32,
    pub VertexRateUtilizedPercent: f32,
    pub TriangleSetupRateUtilizedPercent: f32,
    pub FillRateUtilizedPercent: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9CACHEUTILIZATION {
    pub TextureCacheHitRate: f32,
    pub PostTransformVertexCacheHitRate: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9INTERFACETIMINGS {
    pub WaitingForGPUToUseApplicationResourceTimePercent: f32,
    pub WaitingForGPUToAcceptMoreCommandsTimePercent: f32,
    pub WaitingForGPUToStayWithinLatencyTimePercent: f32,
    pub WaitingForGPUExclusiveResourceTimePercent: f32,
    pub WaitingForGPUOtherTimePercent: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9PIPELINETIMINGS {
    pub VertexProcessingTimePercent: f32,
    pub PixelProcessingTimePercent: f32,
    pub OtherGPUProcessingTimePercent: f32,
    pub GPUIdleTimePercent: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9STAGETIMINGS {
    pub MemoryProcessingPercent: f32,
    pub ComputationProcessingPercent: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3DVERTEXSTATS {
    pub NumRenderedTriangles: u32,
    pub NumExtraClippingTriangles: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDEVINFO_RESOURCEMANAGER {
    pub stats: [D3DRESOURCESTATS; 8],
}
impl Default for D3DDEVINFO_RESOURCEMANAGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_VCACHE {
    pub Pattern: u32,
    pub OptMethod: u32,
    pub CacheSize: u32,
    pub MagicNumber: u32,
}
pub type D3DDEVTYPE = i32;
pub const D3DDEVTYPE_FORCE_DWORD: D3DDEVTYPE = 2147483647;
pub const D3DDEVTYPE_HAL: D3DDEVTYPE = 1;
pub const D3DDEVTYPE_NULLREF: D3DDEVTYPE = 4;
pub const D3DDEVTYPE_REF: D3DDEVTYPE = 2;
pub const D3DDEVTYPE_SW: D3DDEVTYPE = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDISPLAYMODE {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: u32,
    pub Format: D3DFORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDISPLAYMODEEX {
    pub Size: u32,
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: u32,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDISPLAYMODEFILTER {
    pub Size: u32,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
pub type D3DDISPLAYROTATION = i32;
pub const D3DDISPLAYROTATION_180: D3DDISPLAYROTATION = 3;
pub const D3DDISPLAYROTATION_270: D3DDISPLAYROTATION = 4;
pub const D3DDISPLAYROTATION_90: D3DDISPLAYROTATION = 2;
pub const D3DDISPLAYROTATION_IDENTITY: D3DDISPLAYROTATION = 1;
pub const D3DDMAPSAMPLER: u32 = 256;
pub const D3DDMT_DISABLE: D3DDEBUGMONITORTOKENS = 1;
pub const D3DDMT_ENABLE: D3DDEBUGMONITORTOKENS = 0;
pub const D3DDMT_FORCE_DWORD: D3DDEBUGMONITORTOKENS = 2147483647;
pub const D3DDP_MAXTEXCOORD: u32 = 8;
pub const D3DDTCAPS_DEC3N: u32 = 128;
pub const D3DDTCAPS_FLOAT16_2: u32 = 256;
pub const D3DDTCAPS_FLOAT16_4: u32 = 512;
pub const D3DDTCAPS_SHORT2N: u32 = 4;
pub const D3DDTCAPS_SHORT4N: u32 = 8;
pub const D3DDTCAPS_UBYTE4: u32 = 1;
pub const D3DDTCAPS_UBYTE4N: u32 = 2;
pub const D3DDTCAPS_UDEC3: u32 = 64;
pub const D3DDTCAPS_USHORT2N: u32 = 16;
pub const D3DDTCAPS_USHORT4N: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: u32,
    pub NumBytesInSkipPattern: u32,
    pub NumBytesInEncryptPattern: u32,
}
pub const D3DENUM_NO_DRIVERVERSION: u32 = 4;
pub const D3DENUM_WHQL_LEVEL: u32 = 2;
pub const D3DERR_CANNOTPROTECTCONTENT: i32 = -2005530499;
pub const D3DERR_CONFLICTINGRENDERSTATE: i32 = -2005530591;
pub const D3DERR_CONFLICTINGTEXTUREFILTER: i32 = -2005530594;
pub const D3DERR_CONFLICTINGTEXTUREPALETTE: i32 = -2005530586;
pub const D3DERR_DEVICEHUNG: i32 = -2005530508;
pub const D3DERR_DEVICELOST: i32 = -2005530520;
pub const D3DERR_DEVICENOTRESET: i32 = -2005530519;
pub const D3DERR_DEVICEREMOVED: i32 = -2005530512;
pub const D3DERR_DRIVERINTERNALERROR: i32 = -2005530585;
pub const D3DERR_DRIVERINVALIDCALL: i32 = -2005530515;
pub const D3DERR_INVALIDCALL: i32 = -2005530516;
pub const D3DERR_INVALIDDEVICE: i32 = -2005530517;
pub const D3DERR_MOREDATA: i32 = -2005530521;
pub const D3DERR_NOTAVAILABLE: i32 = -2005530518;
pub const D3DERR_NOTFOUND: i32 = -2005530522;
pub const D3DERR_OUTOFVIDEOMEMORY: i32 = -2005532292;
pub const D3DERR_PRESENT_STATISTICS_DISJOINT: i32 = -2005530492;
pub const D3DERR_TOOMANYOPERATIONS: i32 = -2005530595;
pub const D3DERR_UNSUPPORTEDALPHAARG: i32 = -2005530596;
pub const D3DERR_UNSUPPORTEDALPHAOPERATION: i32 = -2005530597;
pub const D3DERR_UNSUPPORTEDCOLORARG: i32 = -2005530598;
pub const D3DERR_UNSUPPORTEDCOLOROPERATION: i32 = -2005530599;
pub const D3DERR_UNSUPPORTEDCRYPTO: i32 = -2005530498;
pub const D3DERR_UNSUPPORTEDFACTORVALUE: i32 = -2005530593;
pub const D3DERR_UNSUPPORTEDOVERLAY: i32 = -2005530501;
pub const D3DERR_UNSUPPORTEDOVERLAYFORMAT: i32 = -2005530500;
pub const D3DERR_UNSUPPORTEDTEXTUREFILTER: i32 = -2005530590;
pub const D3DERR_WASSTILLDRAWING: i32 = -2005532132;
pub const D3DERR_WRONGTEXTUREFORMAT: i32 = -2005530600;
pub type D3DFILLMODE = i32;
pub const D3DFILL_FORCE_DWORD: D3DFILLMODE = 2147483647;
pub const D3DFILL_POINT: D3DFILLMODE = 1;
pub const D3DFILL_SOLID: D3DFILLMODE = 3;
pub const D3DFILL_WIREFRAME: D3DFILLMODE = 2;
pub const D3DFMT_A1: D3DFORMAT = 118;
pub const D3DFMT_A16B16G16R16: D3DFORMAT = 36;
pub const D3DFMT_A16B16G16R16F: D3DFORMAT = 113;
pub const D3DFMT_A1R5G5B5: D3DFORMAT = 25;
pub const D3DFMT_A1_SURFACE_MAXHEIGHT: u32 = 2048;
pub const D3DFMT_A1_SURFACE_MAXWIDTH: u32 = 8192;
pub const D3DFMT_A2B10G10R10: D3DFORMAT = 31;
pub const D3DFMT_A2B10G10R10_XR_BIAS: D3DFORMAT = 119;
pub const D3DFMT_A2R10G10B10: D3DFORMAT = 35;
pub const D3DFMT_A2W10V10U10: D3DFORMAT = 67;
pub const D3DFMT_A32B32G32R32F: D3DFORMAT = 116;
pub const D3DFMT_A4L4: D3DFORMAT = 52;
pub const D3DFMT_A4R4G4B4: D3DFORMAT = 26;
pub const D3DFMT_A8: D3DFORMAT = 28;
pub const D3DFMT_A8B8G8R8: D3DFORMAT = 32;
pub const D3DFMT_A8L8: D3DFORMAT = 51;
pub const D3DFMT_A8P8: D3DFORMAT = 40;
pub const D3DFMT_A8R3G3B2: D3DFORMAT = 29;
pub const D3DFMT_A8R8G8B8: D3DFORMAT = 21;
pub const D3DFMT_BINARYBUFFER: D3DFORMAT = 199;
pub const D3DFMT_CxV8U8: D3DFORMAT = 117;
pub const D3DFMT_D15S1: D3DFORMAT = 73;
pub const D3DFMT_D16: D3DFORMAT = 80;
pub const D3DFMT_D16_LOCKABLE: D3DFORMAT = 70;
pub const D3DFMT_D24FS8: D3DFORMAT = 83;
pub const D3DFMT_D24S8: D3DFORMAT = 75;
pub const D3DFMT_D24X4S4: D3DFORMAT = 79;
pub const D3DFMT_D24X8: D3DFORMAT = 77;
pub const D3DFMT_D32: D3DFORMAT = 71;
pub const D3DFMT_D32F_LOCKABLE: D3DFORMAT = 82;
pub const D3DFMT_D32_LOCKABLE: D3DFORMAT = 84;
pub const D3DFMT_DXT1: D3DFORMAT = 827611204;
pub const D3DFMT_DXT2: D3DFORMAT = 844388420;
pub const D3DFMT_DXT3: D3DFORMAT = 861165636;
pub const D3DFMT_DXT4: D3DFORMAT = 877942852;
pub const D3DFMT_DXT5: D3DFORMAT = 894720068;
pub const D3DFMT_FORCE_DWORD: D3DFORMAT = 2147483647;
pub const D3DFMT_G16R16: D3DFORMAT = 34;
pub const D3DFMT_G16R16F: D3DFORMAT = 112;
pub const D3DFMT_G32R32F: D3DFORMAT = 115;
pub const D3DFMT_G8R8_G8B8: D3DFORMAT = 1111970375;
pub const D3DFMT_INDEX16: D3DFORMAT = 101;
pub const D3DFMT_INDEX32: D3DFORMAT = 102;
pub const D3DFMT_L16: D3DFORMAT = 81;
pub const D3DFMT_L6V5U5: D3DFORMAT = 61;
pub const D3DFMT_L8: D3DFORMAT = 50;
pub const D3DFMT_MULTI2_ARGB8: D3DFORMAT = 827606349;
pub const D3DFMT_P8: D3DFORMAT = 41;
pub const D3DFMT_Q16W16V16U16: D3DFORMAT = 110;
pub const D3DFMT_Q8W8V8U8: D3DFORMAT = 63;
pub const D3DFMT_R16F: D3DFORMAT = 111;
pub const D3DFMT_R32F: D3DFORMAT = 114;
pub const D3DFMT_R3G3B2: D3DFORMAT = 27;
pub const D3DFMT_R5G6B5: D3DFORMAT = 23;
pub const D3DFMT_R8G8B8: D3DFORMAT = 20;
pub const D3DFMT_R8G8_B8G8: D3DFORMAT = 1195525970;
pub const D3DFMT_S8_LOCKABLE: D3DFORMAT = 85;
pub const D3DFMT_UNKNOWN: D3DFORMAT = 0;
pub const D3DFMT_UYVY: D3DFORMAT = 1498831189;
pub const D3DFMT_V16U16: D3DFORMAT = 64;
pub const D3DFMT_V8U8: D3DFORMAT = 60;
pub const D3DFMT_VERTEXDATA: D3DFORMAT = 100;
pub const D3DFMT_X1R5G5B5: D3DFORMAT = 24;
pub const D3DFMT_X4R4G4B4: D3DFORMAT = 30;
pub const D3DFMT_X8B8G8R8: D3DFORMAT = 33;
pub const D3DFMT_X8L8V8U8: D3DFORMAT = 62;
pub const D3DFMT_X8R8G8B8: D3DFORMAT = 22;
pub const D3DFMT_YUY2: D3DFORMAT = 844715353;
pub type D3DFOGMODE = i32;
pub const D3DFOG_EXP: D3DFOGMODE = 1;
pub const D3DFOG_EXP2: D3DFOGMODE = 2;
pub const D3DFOG_FORCE_DWORD: D3DFOGMODE = 2147483647;
pub const D3DFOG_LINEAR: D3DFOGMODE = 3;
pub const D3DFOG_NONE: D3DFOGMODE = 0;
pub type D3DFORMAT = i32;
pub const D3DFVFCAPS_DONOTSTRIPELEMENTS: u32 = 524288;
pub const D3DFVFCAPS_PSIZE: u32 = 1048576;
pub const D3DFVFCAPS_TEXCOORDCOUNTMASK: u32 = 65535;
pub const D3DFVF_DIFFUSE: u32 = 64;
pub const D3DFVF_LASTBETA_D3DCOLOR: u32 = 32768;
pub const D3DFVF_LASTBETA_UBYTE4: u32 = 4096;
pub const D3DFVF_NORMAL: u32 = 16;
pub const D3DFVF_POSITION_MASK: u32 = 16398;
pub const D3DFVF_PSIZE: u32 = 32;
pub const D3DFVF_RESERVED0: u32 = 1;
pub const D3DFVF_RESERVED2: u32 = 24576;
pub const D3DFVF_SPECULAR: u32 = 128;
pub const D3DFVF_TEX0: u32 = 0;
pub const D3DFVF_TEX1: u32 = 256;
pub const D3DFVF_TEX2: u32 = 512;
pub const D3DFVF_TEX3: u32 = 768;
pub const D3DFVF_TEX4: u32 = 1024;
pub const D3DFVF_TEX5: u32 = 1280;
pub const D3DFVF_TEX6: u32 = 1536;
pub const D3DFVF_TEX7: u32 = 1792;
pub const D3DFVF_TEX8: u32 = 2048;
pub const D3DFVF_TEXCOUNT_MASK: u32 = 3840;
pub const D3DFVF_TEXCOUNT_SHIFT: u32 = 8;
pub const D3DFVF_TEXTUREFORMAT1: u32 = 3;
pub const D3DFVF_TEXTUREFORMAT2: u32 = 0;
pub const D3DFVF_TEXTUREFORMAT3: u32 = 1;
pub const D3DFVF_TEXTUREFORMAT4: u32 = 2;
pub const D3DFVF_XYZ: u32 = 2;
pub const D3DFVF_XYZB1: u32 = 6;
pub const D3DFVF_XYZB2: u32 = 8;
pub const D3DFVF_XYZB3: u32 = 10;
pub const D3DFVF_XYZB4: u32 = 12;
pub const D3DFVF_XYZB5: u32 = 14;
pub const D3DFVF_XYZRHW: u32 = 4;
pub const D3DFVF_XYZW: u32 = 16386;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DGAMMARAMP {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}
impl Default for D3DGAMMARAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DGETDATA_FLUSH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DINDEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Size: u32,
}
pub const D3DISSUE_BEGIN: u32 = 2;
pub const D3DISSUE_END: u32 = 1;
pub const D3DKEYEXCHANGE_DXVA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43d3775c_38e5_4924_8d86_d3fccf153e9b);
pub const D3DKEYEXCHANGE_RSAES_OAEP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc1949895_d72a_4a1d_8e5d_ed857d171520);
#[repr(C)]
#[cfg(all(feature = "dsound", feature = "dxgi"))]
#[derive(Clone, Copy, Default)]
pub struct D3DLIGHT9 {
    pub Type: D3DLIGHTTYPE,
    pub Diffuse: super::dxgi::D3DCOLORVALUE,
    pub Specular: super::dxgi::D3DCOLORVALUE,
    pub Ambient: super::dxgi::D3DCOLORVALUE,
    pub Position: super::dsound::D3DVECTOR,
    pub Direction: super::dsound::D3DVECTOR,
    pub Range: f32,
    pub Falloff: f32,
    pub Attenuation0: f32,
    pub Attenuation1: f32,
    pub Attenuation2: f32,
    pub Theta: f32,
    pub Phi: f32,
}
pub type D3DLIGHTTYPE = i32;
pub const D3DLIGHT_DIRECTIONAL: D3DLIGHTTYPE = 3;
pub const D3DLIGHT_FORCE_DWORD: D3DLIGHTTYPE = 2147483647;
pub const D3DLIGHT_POINT: D3DLIGHTTYPE = 1;
pub const D3DLIGHT_SPOT: D3DLIGHTTYPE = 2;
pub const D3DLINECAPS_ALPHACMP: u32 = 8;
pub const D3DLINECAPS_ANTIALIAS: u32 = 32;
pub const D3DLINECAPS_BLEND: u32 = 4;
pub const D3DLINECAPS_FOG: u32 = 16;
pub const D3DLINECAPS_TEXTURE: u32 = 1;
pub const D3DLINECAPS_ZTEST: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DLOCKED_BOX {
    pub RowPitch: i32,
    pub SlicePitch: i32,
    pub pBits: *mut core::ffi::c_void,
}
impl Default for D3DLOCKED_BOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DLOCKED_RECT {
    pub Pitch: i32,
    pub pBits: *mut core::ffi::c_void,
}
impl Default for D3DLOCKED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DLOCK_DISCARD: u32 = 8192;
pub const D3DLOCK_DONOTWAIT: u32 = 16384;
pub const D3DLOCK_NOOVERWRITE: u32 = 4096;
pub const D3DLOCK_NOSYSLOCK: u32 = 2048;
pub const D3DLOCK_NO_DIRTY_UPDATE: u32 = 32768;
pub const D3DLOCK_READONLY: u32 = 16;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3DMATERIAL9 {
    pub Diffuse: super::dxgi::D3DCOLORVALUE,
    pub Ambient: super::dxgi::D3DCOLORVALUE,
    pub Specular: super::dxgi::D3DCOLORVALUE,
    pub Emissive: super::dxgi::D3DCOLORVALUE,
    pub Power: f32,
}
pub type D3DMATERIALCOLORSOURCE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DMATRIX {
    pub Anonymous: D3DMATRIX_0,
}
impl Default for D3DMATRIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATRIX_0 {
    pub Anonymous: D3DMATRIX_0_0,
    pub m: [[f32; 4]; 4],
}
impl Default for D3DMATRIX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DMATRIX_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
pub const D3DMAX30SHADERINSTRUCTIONS: u32 = 32768;
pub const D3DMAXUSERCLIPPLANES: u32 = 32;
pub const D3DMCS_COLOR1: D3DMATERIALCOLORSOURCE = 1;
pub const D3DMCS_COLOR2: D3DMATERIALCOLORSOURCE = 2;
pub const D3DMCS_FORCE_DWORD: D3DMATERIALCOLORSOURCE = 2147483647;
pub const D3DMCS_MATERIAL: D3DMATERIALCOLORSOURCE = 0;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: u64,
    pub SizeOfInefficientAllocation: u64,
    pub LevelOfEfficiency: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: u64,
    pub SizeOfInefficientAllocation: u64,
    pub LevelOfEfficiency: u32,
}
pub const D3DMIN30SHADERINSTRUCTIONS: u32 = 512;
pub const D3DMP_16: D3DSHADER_MIN_PRECISION = 1;
pub const D3DMP_2_8: D3DSHADER_MIN_PRECISION = 2;
pub const D3DMP_DEFAULT: D3DSHADER_MIN_PRECISION = 0;
pub const D3DMULTISAMPLE_10_SAMPLES: D3DMULTISAMPLE_TYPE = 10;
pub const D3DMULTISAMPLE_11_SAMPLES: D3DMULTISAMPLE_TYPE = 11;
pub const D3DMULTISAMPLE_12_SAMPLES: D3DMULTISAMPLE_TYPE = 12;
pub const D3DMULTISAMPLE_13_SAMPLES: D3DMULTISAMPLE_TYPE = 13;
pub const D3DMULTISAMPLE_14_SAMPLES: D3DMULTISAMPLE_TYPE = 14;
pub const D3DMULTISAMPLE_15_SAMPLES: D3DMULTISAMPLE_TYPE = 15;
pub const D3DMULTISAMPLE_16_SAMPLES: D3DMULTISAMPLE_TYPE = 16;
pub const D3DMULTISAMPLE_2_SAMPLES: D3DMULTISAMPLE_TYPE = 2;
pub const D3DMULTISAMPLE_3_SAMPLES: D3DMULTISAMPLE_TYPE = 3;
pub const D3DMULTISAMPLE_4_SAMPLES: D3DMULTISAMPLE_TYPE = 4;
pub const D3DMULTISAMPLE_5_SAMPLES: D3DMULTISAMPLE_TYPE = 5;
pub const D3DMULTISAMPLE_6_SAMPLES: D3DMULTISAMPLE_TYPE = 6;
pub const D3DMULTISAMPLE_7_SAMPLES: D3DMULTISAMPLE_TYPE = 7;
pub const D3DMULTISAMPLE_8_SAMPLES: D3DMULTISAMPLE_TYPE = 8;
pub const D3DMULTISAMPLE_9_SAMPLES: D3DMULTISAMPLE_TYPE = 9;
pub const D3DMULTISAMPLE_FORCE_DWORD: D3DMULTISAMPLE_TYPE = 2147483647;
pub const D3DMULTISAMPLE_NONE: D3DMULTISAMPLE_TYPE = 0;
pub const D3DMULTISAMPLE_NONMASKABLE: D3DMULTISAMPLE_TYPE = 1;
pub type D3DMULTISAMPLE_TYPE = i32;
pub const D3DOK_NOAUTOGEN: u32 = 141953135;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DOVERLAYCAPS {
    pub Caps: u32,
    pub MaxOverlayDisplayWidth: u32,
    pub MaxOverlayDisplayHeight: u32,
}
pub const D3DOVERLAYCAPS_FULLRANGERGB: u32 = 1;
pub const D3DOVERLAYCAPS_LIMITEDRANGERGB: u32 = 2;
pub const D3DOVERLAYCAPS_STRETCHX: u32 = 64;
pub const D3DOVERLAYCAPS_STRETCHY: u32 = 128;
pub const D3DOVERLAYCAPS_YCbCr_BT601: u32 = 4;
pub const D3DOVERLAYCAPS_YCbCr_BT601_xvYCC: u32 = 16;
pub const D3DOVERLAYCAPS_YCbCr_BT709: u32 = 8;
pub const D3DOVERLAYCAPS_YCbCr_BT709_xvYCC: u32 = 32;
pub type D3DPATCHEDGESTYLE = i32;
pub const D3DPATCHEDGE_CONTINUOUS: D3DPATCHEDGESTYLE = 1;
pub const D3DPATCHEDGE_DISCRETE: D3DPATCHEDGESTYLE = 0;
pub const D3DPATCHEDGE_FORCE_DWORD: D3DPATCHEDGESTYLE = 2147483647;
pub const D3DPBLENDCAPS_BLENDFACTOR: u32 = 8192;
pub const D3DPBLENDCAPS_BOTHINVSRCALPHA: u32 = 4096;
pub const D3DPBLENDCAPS_BOTHSRCALPHA: u32 = 2048;
pub const D3DPBLENDCAPS_DESTALPHA: u32 = 64;
pub const D3DPBLENDCAPS_DESTCOLOR: u32 = 256;
pub const D3DPBLENDCAPS_INVDESTALPHA: u32 = 128;
pub const D3DPBLENDCAPS_INVDESTCOLOR: u32 = 512;
pub const D3DPBLENDCAPS_INVSRCALPHA: u32 = 32;
pub const D3DPBLENDCAPS_INVSRCCOLOR: u32 = 8;
pub const D3DPBLENDCAPS_INVSRCCOLOR2: u32 = 32768;
pub const D3DPBLENDCAPS_ONE: u32 = 2;
pub const D3DPBLENDCAPS_SRCALPHA: u32 = 16;
pub const D3DPBLENDCAPS_SRCALPHASAT: u32 = 1024;
pub const D3DPBLENDCAPS_SRCCOLOR: u32 = 4;
pub const D3DPBLENDCAPS_SRCCOLOR2: u32 = 16384;
pub const D3DPBLENDCAPS_ZERO: u32 = 1;
pub const D3DPCMPCAPS_ALWAYS: u32 = 128;
pub const D3DPCMPCAPS_EQUAL: u32 = 4;
pub const D3DPCMPCAPS_GREATER: u32 = 16;
pub const D3DPCMPCAPS_GREATEREQUAL: u32 = 64;
pub const D3DPCMPCAPS_LESS: u32 = 2;
pub const D3DPCMPCAPS_LESSEQUAL: u32 = 8;
pub const D3DPCMPCAPS_NEVER: u32 = 1;
pub const D3DPCMPCAPS_NOTEQUAL: u32 = 32;
pub const D3DPMISCCAPS_BLENDOP: u32 = 2048;
pub const D3DPMISCCAPS_CLIPPLANESCALEDPOINTS: u32 = 256;
pub const D3DPMISCCAPS_CLIPTLVERTS: u32 = 512;
pub const D3DPMISCCAPS_COLORWRITEENABLE: u32 = 128;
pub const D3DPMISCCAPS_CULLCCW: u32 = 64;
pub const D3DPMISCCAPS_CULLCW: u32 = 32;
pub const D3DPMISCCAPS_CULLNONE: u32 = 16;
pub const D3DPMISCCAPS_FOGANDSPECULARALPHA: u32 = 65536;
pub const D3DPMISCCAPS_FOGVERTEXCLAMPED: u32 = 1048576;
pub const D3DPMISCCAPS_INDEPENDENTWRITEMASKS: u32 = 16384;
pub const D3DPMISCCAPS_MASKZ: u32 = 2;
pub const D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS: u32 = 262144;
pub const D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING: u32 = 524288;
pub const D3DPMISCCAPS_NULLREFERENCE: u32 = 4096;
pub const D3DPMISCCAPS_PERSTAGECONSTANT: u32 = 32768;
pub const D3DPMISCCAPS_POSTBLENDSRGBCONVERT: u32 = 2097152;
pub const D3DPMISCCAPS_SEPARATEALPHABLEND: u32 = 131072;
pub const D3DPMISCCAPS_TSSARGTEMP: u32 = 1024;
pub type D3DPOOL = i32;
pub const D3DPOOL_DEFAULT: D3DPOOL = 0;
pub const D3DPOOL_FORCE_DWORD: D3DPOOL = 2147483647;
pub const D3DPOOL_MANAGED: D3DPOOL = 1;
pub const D3DPOOL_SCRATCH: D3DPOOL = 3;
pub const D3DPOOL_SYSTEMMEM: D3DPOOL = 2;
pub const D3DPRASTERCAPS_ANISOTROPY: u32 = 131072;
pub const D3DPRASTERCAPS_COLORPERSPECTIVE: u32 = 4194304;
pub const D3DPRASTERCAPS_DEPTHBIAS: u32 = 67108864;
pub const D3DPRASTERCAPS_DITHER: u32 = 1;
pub const D3DPRASTERCAPS_FOGRANGE: u32 = 65536;
pub const D3DPRASTERCAPS_FOGTABLE: u32 = 256;
pub const D3DPRASTERCAPS_FOGVERTEX: u32 = 128;
pub const D3DPRASTERCAPS_MIPMAPLODBIAS: u32 = 8192;
pub const D3DPRASTERCAPS_MULTISAMPLE_TOGGLE: u32 = 134217728;
pub const D3DPRASTERCAPS_SCISSORTEST: u32 = 16777216;
pub const D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS: u32 = 33554432;
pub const D3DPRASTERCAPS_WBUFFER: u32 = 262144;
pub const D3DPRASTERCAPS_WFOG: u32 = 1048576;
pub const D3DPRASTERCAPS_ZBUFFERLESSHSR: u32 = 32768;
pub const D3DPRASTERCAPS_ZFOG: u32 = 2097152;
pub const D3DPRASTERCAPS_ZTEST: u32 = 16;
pub const D3DPRESENTFLAG_DEVICECLIP: u32 = 4;
pub const D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL: u32 = 2;
pub const D3DPRESENTFLAG_LOCKABLE_BACKBUFFER: u32 = 1;
pub const D3DPRESENTFLAG_NOAUTOROTATE: u32 = 32;
pub const D3DPRESENTFLAG_OVERLAY_LIMITEDRGB: u32 = 128;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_BT709: u32 = 256;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC: u32 = 512;
pub const D3DPRESENTFLAG_RESTRICTED_CONTENT: u32 = 1024;
pub const D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER: u32 = 2048;
pub const D3DPRESENTFLAG_UNPRUNEDMODE: u32 = 64;
pub const D3DPRESENTFLAG_VIDEO: u32 = 16;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct D3DPRESENTSTATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct D3DPRESENTSTATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
pub const D3DPRESENT_BACK_BUFFERS_MAX: u32 = 3;
pub const D3DPRESENT_BACK_BUFFERS_MAX_EX: u32 = 30;
pub const D3DPRESENT_DONOTFLIP: u32 = 4;
pub const D3DPRESENT_DONOTWAIT: u32 = 1;
pub const D3DPRESENT_FLIPRESTART: u32 = 8;
pub const D3DPRESENT_FORCEIMMEDIATE: u32 = 256;
pub const D3DPRESENT_HIDEOVERLAY: u32 = 64;
pub const D3DPRESENT_INTERVAL_DEFAULT: u32 = 0;
pub const D3DPRESENT_INTERVAL_FOUR: u32 = 8;
pub const D3DPRESENT_INTERVAL_IMMEDIATE: u32 = 2147483648;
pub const D3DPRESENT_INTERVAL_ONE: u32 = 1;
pub const D3DPRESENT_INTERVAL_THREE: u32 = 4;
pub const D3DPRESENT_INTERVAL_TWO: u32 = 2;
pub const D3DPRESENT_LINEAR_CONTENT: u32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct D3DPRESENT_PARAMETERS {
    pub BackBufferWidth: u32,
    pub BackBufferHeight: u32,
    pub BackBufferFormat: D3DFORMAT,
    pub BackBufferCount: u32,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: u32,
    pub SwapEffect: D3DSWAPEFFECT,
    pub hDeviceWindow: super::windef::HWND,
    pub Windowed: windows_sys::core::BOOL,
    pub EnableAutoDepthStencil: windows_sys::core::BOOL,
    pub AutoDepthStencilFormat: D3DFORMAT,
    pub Flags: u32,
    pub FullScreen_RefreshRateInHz: u32,
    pub PresentationInterval: u32,
}
#[cfg(feature = "windef")]
impl Default for D3DPRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DPRESENT_RATE_DEFAULT: u32 = 0;
pub const D3DPRESENT_UPDATECOLORKEY: u32 = 128;
pub const D3DPRESENT_UPDATEOVERLAYONLY: u32 = 32;
pub const D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR: u32 = 16;
pub type D3DPRIMITIVETYPE = i32;
pub const D3DPS20CAPS_ARBITRARYSWIZZLE: u32 = 1;
pub const D3DPS20CAPS_GRADIENTINSTRUCTIONS: u32 = 2;
pub const D3DPS20CAPS_NODEPENDENTREADLIMIT: u32 = 8;
pub const D3DPS20CAPS_NOTEXINSTRUCTIONLIMIT: u32 = 16;
pub const D3DPS20CAPS_PREDICATION: u32 = 4;
pub const D3DPS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24;
pub const D3DPS20_MAX_NUMINSTRUCTIONSLOTS: u32 = 512;
pub const D3DPS20_MAX_NUMTEMPS: u32 = 32;
pub const D3DPS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4;
pub const D3DPS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0;
pub const D3DPS20_MIN_NUMINSTRUCTIONSLOTS: u32 = 96;
pub const D3DPS20_MIN_NUMTEMPS: u32 = 12;
pub const D3DPS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 0;
pub const D3DPSHADECAPS_ALPHAGOURAUDBLEND: u32 = 16384;
pub const D3DPSHADECAPS_COLORGOURAUDRGB: u32 = 8;
pub const D3DPSHADECAPS_FOGGOURAUD: u32 = 524288;
pub const D3DPSHADECAPS_SPECULARGOURAUDRGB: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DPSHADERCAPS2_0 {
    pub Caps: u32,
    pub DynamicFlowControlDepth: i32,
    pub NumTemps: i32,
    pub StaticFlowControlDepth: i32,
    pub NumInstructionSlots: i32,
}
pub const D3DPTADDRESSCAPS_BORDER: u32 = 8;
pub const D3DPTADDRESSCAPS_CLAMP: u32 = 4;
pub const D3DPTADDRESSCAPS_INDEPENDENTUV: u32 = 16;
pub const D3DPTADDRESSCAPS_MIRROR: u32 = 2;
pub const D3DPTADDRESSCAPS_MIRRORONCE: u32 = 32;
pub const D3DPTADDRESSCAPS_WRAP: u32 = 1;
pub const D3DPTEXTURECAPS_ALPHA: u32 = 4;
pub const D3DPTEXTURECAPS_ALPHAPALETTE: u32 = 128;
pub const D3DPTEXTURECAPS_CUBEMAP: u32 = 2048;
pub const D3DPTEXTURECAPS_CUBEMAP_POW2: u32 = 131072;
pub const D3DPTEXTURECAPS_MIPCUBEMAP: u32 = 65536;
pub const D3DPTEXTURECAPS_MIPMAP: u32 = 16384;
pub const D3DPTEXTURECAPS_MIPVOLUMEMAP: u32 = 32768;
pub const D3DPTEXTURECAPS_NONPOW2CONDITIONAL: u32 = 256;
pub const D3DPTEXTURECAPS_NOPROJECTEDBUMPENV: u32 = 2097152;
pub const D3DPTEXTURECAPS_PERSPECTIVE: u32 = 1;
pub const D3DPTEXTURECAPS_POW2: u32 = 2;
pub const D3DPTEXTURECAPS_PROJECTED: u32 = 1024;
pub const D3DPTEXTURECAPS_SQUAREONLY: u32 = 32;
pub const D3DPTEXTURECAPS_TEXREPEATNOTSCALEDBYSIZE: u32 = 64;
pub const D3DPTEXTURECAPS_VOLUMEMAP: u32 = 8192;
pub const D3DPTEXTURECAPS_VOLUMEMAP_POW2: u32 = 262144;
pub const D3DPTFILTERCAPS_CONVOLUTIONMONO: u32 = 262144;
pub const D3DPTFILTERCAPS_MAGFANISOTROPIC: u32 = 67108864;
pub const D3DPTFILTERCAPS_MAGFGAUSSIANQUAD: u32 = 268435456;
pub const D3DPTFILTERCAPS_MAGFLINEAR: u32 = 33554432;
pub const D3DPTFILTERCAPS_MAGFPOINT: u32 = 16777216;
pub const D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD: u32 = 134217728;
pub const D3DPTFILTERCAPS_MINFANISOTROPIC: u32 = 1024;
pub const D3DPTFILTERCAPS_MINFGAUSSIANQUAD: u32 = 4096;
pub const D3DPTFILTERCAPS_MINFLINEAR: u32 = 512;
pub const D3DPTFILTERCAPS_MINFPOINT: u32 = 256;
pub const D3DPTFILTERCAPS_MINFPYRAMIDALQUAD: u32 = 2048;
pub const D3DPTFILTERCAPS_MIPFLINEAR: u32 = 131072;
pub const D3DPTFILTERCAPS_MIPFPOINT: u32 = 65536;
pub const D3DPT_FORCE_DWORD: D3DPRIMITIVETYPE = 2147483647;
pub const D3DPT_LINELIST: D3DPRIMITIVETYPE = 2;
pub const D3DPT_LINESTRIP: D3DPRIMITIVETYPE = 3;
pub const D3DPT_POINTLIST: D3DPRIMITIVETYPE = 1;
pub const D3DPT_TRIANGLEFAN: D3DPRIMITIVETYPE = 6;
pub const D3DPT_TRIANGLELIST: D3DPRIMITIVETYPE = 4;
pub const D3DPT_TRIANGLESTRIP: D3DPRIMITIVETYPE = 5;
pub const D3DPV_DONOTCOPYDATA: u32 = 1;
pub type D3DQUERYTYPE = i32;
pub const D3DQUERYTYPE_BANDWIDTHTIMINGS: D3DQUERYTYPE = 17;
pub const D3DQUERYTYPE_CACHEUTILIZATION: D3DQUERYTYPE = 18;
pub const D3DQUERYTYPE_EVENT: D3DQUERYTYPE = 8;
pub const D3DQUERYTYPE_INTERFACETIMINGS: D3DQUERYTYPE = 14;
pub const D3DQUERYTYPE_MEMORYPRESSURE: D3DQUERYTYPE = 19;
pub const D3DQUERYTYPE_OCCLUSION: D3DQUERYTYPE = 9;
pub const D3DQUERYTYPE_PIPELINETIMINGS: D3DQUERYTYPE = 13;
pub const D3DQUERYTYPE_PIXELTIMINGS: D3DQUERYTYPE = 16;
pub const D3DQUERYTYPE_RESOURCEMANAGER: D3DQUERYTYPE = 5;
pub const D3DQUERYTYPE_TIMESTAMP: D3DQUERYTYPE = 10;
pub const D3DQUERYTYPE_TIMESTAMPDISJOINT: D3DQUERYTYPE = 11;
pub const D3DQUERYTYPE_TIMESTAMPFREQ: D3DQUERYTYPE = 12;
pub const D3DQUERYTYPE_VCACHE: D3DQUERYTYPE = 4;
pub const D3DQUERYTYPE_VERTEXSTATS: D3DQUERYTYPE = 6;
pub const D3DQUERYTYPE_VERTEXTIMINGS: D3DQUERYTYPE = 15;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DRANGE {
    pub Offset: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DRASTER_STATUS {
    pub InVBlank: windows_sys::core::BOOL,
    pub ScanLine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DRECT {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DRECTPATCH_INFO {
    pub StartVertexOffsetWidth: u32,
    pub StartVertexOffsetHeight: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
pub type D3DRENDERSTATETYPE = i32;
pub const D3DRENDERSTATE_WRAPBIAS: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DRESOURCESTATS {
    pub bThrashing: windows_sys::core::BOOL,
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
pub type D3DRESOURCETYPE = i32;
pub const D3DRS_ADAPTIVETESS_W: D3DRENDERSTATETYPE = 183;
pub const D3DRS_ADAPTIVETESS_X: D3DRENDERSTATETYPE = 180;
pub const D3DRS_ADAPTIVETESS_Y: D3DRENDERSTATETYPE = 181;
pub const D3DRS_ADAPTIVETESS_Z: D3DRENDERSTATETYPE = 182;
pub const D3DRS_ALPHABLENDENABLE: D3DRENDERSTATETYPE = 27;
pub const D3DRS_ALPHAFUNC: D3DRENDERSTATETYPE = 25;
pub const D3DRS_ALPHAREF: D3DRENDERSTATETYPE = 24;
pub const D3DRS_ALPHATESTENABLE: D3DRENDERSTATETYPE = 15;
pub const D3DRS_AMBIENT: D3DRENDERSTATETYPE = 139;
pub const D3DRS_AMBIENTMATERIALSOURCE: D3DRENDERSTATETYPE = 147;
pub const D3DRS_ANTIALIASEDLINEENABLE: D3DRENDERSTATETYPE = 176;
pub const D3DRS_BLENDFACTOR: D3DRENDERSTATETYPE = 193;
pub const D3DRS_BLENDOP: D3DRENDERSTATETYPE = 171;
pub const D3DRS_BLENDOPALPHA: D3DRENDERSTATETYPE = 209;
pub const D3DRS_CCW_STENCILFAIL: D3DRENDERSTATETYPE = 186;
pub const D3DRS_CCW_STENCILFUNC: D3DRENDERSTATETYPE = 189;
pub const D3DRS_CCW_STENCILPASS: D3DRENDERSTATETYPE = 188;
pub const D3DRS_CCW_STENCILZFAIL: D3DRENDERSTATETYPE = 187;
pub const D3DRS_CLIPPING: D3DRENDERSTATETYPE = 136;
pub const D3DRS_CLIPPLANEENABLE: D3DRENDERSTATETYPE = 152;
pub const D3DRS_COLORVERTEX: D3DRENDERSTATETYPE = 141;
pub const D3DRS_COLORWRITEENABLE: D3DRENDERSTATETYPE = 168;
pub const D3DRS_COLORWRITEENABLE1: D3DRENDERSTATETYPE = 190;
pub const D3DRS_COLORWRITEENABLE2: D3DRENDERSTATETYPE = 191;
pub const D3DRS_COLORWRITEENABLE3: D3DRENDERSTATETYPE = 192;
pub const D3DRS_CULLMODE: D3DRENDERSTATETYPE = 22;
pub const D3DRS_DEBUGMONITORTOKEN: D3DRENDERSTATETYPE = 165;
pub const D3DRS_DEPTHBIAS: D3DRENDERSTATETYPE = 195;
pub const D3DRS_DESTBLEND: D3DRENDERSTATETYPE = 20;
pub const D3DRS_DESTBLENDALPHA: D3DRENDERSTATETYPE = 208;
pub const D3DRS_DIFFUSEMATERIALSOURCE: D3DRENDERSTATETYPE = 145;
pub const D3DRS_DITHERENABLE: D3DRENDERSTATETYPE = 26;
pub const D3DRS_EMISSIVEMATERIALSOURCE: D3DRENDERSTATETYPE = 148;
pub const D3DRS_ENABLEADAPTIVETESSELLATION: D3DRENDERSTATETYPE = 184;
pub const D3DRS_FILLMODE: D3DRENDERSTATETYPE = 8;
pub const D3DRS_FOGCOLOR: D3DRENDERSTATETYPE = 34;
pub const D3DRS_FOGDENSITY: D3DRENDERSTATETYPE = 38;
pub const D3DRS_FOGENABLE: D3DRENDERSTATETYPE = 28;
pub const D3DRS_FOGEND: D3DRENDERSTATETYPE = 37;
pub const D3DRS_FOGSTART: D3DRENDERSTATETYPE = 36;
pub const D3DRS_FOGTABLEMODE: D3DRENDERSTATETYPE = 35;
pub const D3DRS_FOGVERTEXMODE: D3DRENDERSTATETYPE = 140;
pub const D3DRS_FORCE_DWORD: D3DRENDERSTATETYPE = 2147483647;
pub const D3DRS_INDEXEDVERTEXBLENDENABLE: D3DRENDERSTATETYPE = 167;
pub const D3DRS_LASTPIXEL: D3DRENDERSTATETYPE = 16;
pub const D3DRS_LIGHTING: D3DRENDERSTATETYPE = 137;
pub const D3DRS_LOCALVIEWER: D3DRENDERSTATETYPE = 142;
pub const D3DRS_MAXTESSELLATIONLEVEL: D3DRENDERSTATETYPE = 179;
pub const D3DRS_MINTESSELLATIONLEVEL: D3DRENDERSTATETYPE = 178;
pub const D3DRS_MULTISAMPLEANTIALIAS: D3DRENDERSTATETYPE = 161;
pub const D3DRS_MULTISAMPLEMASK: D3DRENDERSTATETYPE = 162;
pub const D3DRS_NORMALDEGREE: D3DRENDERSTATETYPE = 173;
pub const D3DRS_NORMALIZENORMALS: D3DRENDERSTATETYPE = 143;
pub const D3DRS_PATCHEDGESTYLE: D3DRENDERSTATETYPE = 163;
pub const D3DRS_POINTSCALEENABLE: D3DRENDERSTATETYPE = 157;
pub const D3DRS_POINTSCALE_A: D3DRENDERSTATETYPE = 158;
pub const D3DRS_POINTSCALE_B: D3DRENDERSTATETYPE = 159;
pub const D3DRS_POINTSCALE_C: D3DRENDERSTATETYPE = 160;
pub const D3DRS_POINTSIZE: D3DRENDERSTATETYPE = 154;
pub const D3DRS_POINTSIZE_MAX: D3DRENDERSTATETYPE = 166;
pub const D3DRS_POINTSIZE_MIN: D3DRENDERSTATETYPE = 155;
pub const D3DRS_POINTSPRITEENABLE: D3DRENDERSTATETYPE = 156;
pub const D3DRS_POSITIONDEGREE: D3DRENDERSTATETYPE = 172;
pub const D3DRS_RANGEFOGENABLE: D3DRENDERSTATETYPE = 48;
pub const D3DRS_SCISSORTESTENABLE: D3DRENDERSTATETYPE = 174;
pub const D3DRS_SEPARATEALPHABLENDENABLE: D3DRENDERSTATETYPE = 206;
pub const D3DRS_SHADEMODE: D3DRENDERSTATETYPE = 9;
pub const D3DRS_SLOPESCALEDEPTHBIAS: D3DRENDERSTATETYPE = 175;
pub const D3DRS_SPECULARENABLE: D3DRENDERSTATETYPE = 29;
pub const D3DRS_SPECULARMATERIALSOURCE: D3DRENDERSTATETYPE = 146;
pub const D3DRS_SRCBLEND: D3DRENDERSTATETYPE = 19;
pub const D3DRS_SRCBLENDALPHA: D3DRENDERSTATETYPE = 207;
pub const D3DRS_SRGBWRITEENABLE: D3DRENDERSTATETYPE = 194;
pub const D3DRS_STENCILENABLE: D3DRENDERSTATETYPE = 52;
pub const D3DRS_STENCILFAIL: D3DRENDERSTATETYPE = 53;
pub const D3DRS_STENCILFUNC: D3DRENDERSTATETYPE = 56;
pub const D3DRS_STENCILMASK: D3DRENDERSTATETYPE = 58;
pub const D3DRS_STENCILPASS: D3DRENDERSTATETYPE = 55;
pub const D3DRS_STENCILREF: D3DRENDERSTATETYPE = 57;
pub const D3DRS_STENCILWRITEMASK: D3DRENDERSTATETYPE = 59;
pub const D3DRS_STENCILZFAIL: D3DRENDERSTATETYPE = 54;
pub const D3DRS_TEXTUREFACTOR: D3DRENDERSTATETYPE = 60;
pub const D3DRS_TWEENFACTOR: D3DRENDERSTATETYPE = 170;
pub const D3DRS_TWOSIDEDSTENCILMODE: D3DRENDERSTATETYPE = 185;
pub const D3DRS_VERTEXBLEND: D3DRENDERSTATETYPE = 151;
pub const D3DRS_WRAP0: D3DRENDERSTATETYPE = 128;
pub const D3DRS_WRAP1: D3DRENDERSTATETYPE = 129;
pub const D3DRS_WRAP10: D3DRENDERSTATETYPE = 200;
pub const D3DRS_WRAP11: D3DRENDERSTATETYPE = 201;
pub const D3DRS_WRAP12: D3DRENDERSTATETYPE = 202;
pub const D3DRS_WRAP13: D3DRENDERSTATETYPE = 203;
pub const D3DRS_WRAP14: D3DRENDERSTATETYPE = 204;
pub const D3DRS_WRAP15: D3DRENDERSTATETYPE = 205;
pub const D3DRS_WRAP2: D3DRENDERSTATETYPE = 130;
pub const D3DRS_WRAP3: D3DRENDERSTATETYPE = 131;
pub const D3DRS_WRAP4: D3DRENDERSTATETYPE = 132;
pub const D3DRS_WRAP5: D3DRENDERSTATETYPE = 133;
pub const D3DRS_WRAP6: D3DRENDERSTATETYPE = 134;
pub const D3DRS_WRAP7: D3DRENDERSTATETYPE = 135;
pub const D3DRS_WRAP8: D3DRENDERSTATETYPE = 198;
pub const D3DRS_WRAP9: D3DRENDERSTATETYPE = 199;
pub const D3DRS_ZENABLE: D3DRENDERSTATETYPE = 7;
pub const D3DRS_ZFUNC: D3DRENDERSTATETYPE = 23;
pub const D3DRS_ZWRITEENABLE: D3DRENDERSTATETYPE = 14;
pub const D3DRTYPECOUNT: u32 = 8;
pub const D3DRTYPE_CUBETEXTURE: D3DRESOURCETYPE = 5;
pub const D3DRTYPE_FORCE_DWORD: D3DRESOURCETYPE = 2147483647;
pub const D3DRTYPE_INDEXBUFFER: D3DRESOURCETYPE = 7;
pub const D3DRTYPE_SURFACE: D3DRESOURCETYPE = 1;
pub const D3DRTYPE_TEXTURE: D3DRESOURCETYPE = 3;
pub const D3DRTYPE_VERTEXBUFFER: D3DRESOURCETYPE = 6;
pub const D3DRTYPE_VOLUME: D3DRESOURCETYPE = 2;
pub const D3DRTYPE_VOLUMETEXTURE: D3DRESOURCETYPE = 4;
pub type D3DSAMPLERSTATETYPE = i32;
pub type D3DSAMPLER_TEXTURE_TYPE = i32;
pub const D3DSAMP_ADDRESSU: D3DSAMPLERSTATETYPE = 1;
pub const D3DSAMP_ADDRESSV: D3DSAMPLERSTATETYPE = 2;
pub const D3DSAMP_ADDRESSW: D3DSAMPLERSTATETYPE = 3;
pub const D3DSAMP_BORDERCOLOR: D3DSAMPLERSTATETYPE = 4;
pub const D3DSAMP_DMAPOFFSET: D3DSAMPLERSTATETYPE = 13;
pub const D3DSAMP_ELEMENTINDEX: D3DSAMPLERSTATETYPE = 12;
pub const D3DSAMP_FORCE_DWORD: D3DSAMPLERSTATETYPE = 2147483647;
pub const D3DSAMP_MAGFILTER: D3DSAMPLERSTATETYPE = 5;
pub const D3DSAMP_MAXANISOTROPY: D3DSAMPLERSTATETYPE = 10;
pub const D3DSAMP_MAXMIPLEVEL: D3DSAMPLERSTATETYPE = 9;
pub const D3DSAMP_MINFILTER: D3DSAMPLERSTATETYPE = 6;
pub const D3DSAMP_MIPFILTER: D3DSAMPLERSTATETYPE = 7;
pub const D3DSAMP_MIPMAPLODBIAS: D3DSAMPLERSTATETYPE = 8;
pub const D3DSAMP_SRGBTEXTURE: D3DSAMPLERSTATETYPE = 11;
pub const D3DSBT_ALL: D3DSTATEBLOCKTYPE = 1;
pub const D3DSBT_FORCE_DWORD: D3DSTATEBLOCKTYPE = 2147483647;
pub const D3DSBT_PIXELSTATE: D3DSTATEBLOCKTYPE = 2;
pub const D3DSBT_VERTEXSTATE: D3DSTATEBLOCKTYPE = 3;
pub type D3DSCANLINEORDERING = i32;
pub const D3DSCANLINEORDERING_INTERLACED: D3DSCANLINEORDERING = 2;
pub const D3DSCANLINEORDERING_PROGRESSIVE: D3DSCANLINEORDERING = 1;
pub const D3DSCANLINEORDERING_UNKNOWN: D3DSCANLINEORDERING = 0;
pub const D3DSGR_CALIBRATE: u32 = 1;
pub const D3DSGR_NO_CALIBRATION: u32 = 0;
pub type D3DSHADEMODE = i32;
pub const D3DSHADER_ADDRESSMODE_MASK: u32 = 8192;
pub const D3DSHADER_ADDRESSMODE_SHIFT: u32 = 13;
pub type D3DSHADER_ADDRESSMODE_TYPE = i32;
pub const D3DSHADER_ADDRMODE_ABSOLUTE: D3DSHADER_ADDRESSMODE_TYPE = 0;
pub const D3DSHADER_ADDRMODE_FORCE_DWORD: D3DSHADER_ADDRESSMODE_TYPE = 2147483647;
pub const D3DSHADER_ADDRMODE_RELATIVE: D3DSHADER_ADDRESSMODE_TYPE = 8192;
pub type D3DSHADER_COMPARISON = i32;
pub const D3DSHADER_COMPARISON_MASK: u32 = 458752;
pub const D3DSHADER_COMPARISON_SHIFT: u32 = 16;
pub type D3DSHADER_INSTRUCTION_OPCODE_TYPE = i32;
pub const D3DSHADER_INSTRUCTION_PREDICATED: u32 = 268435456;
pub type D3DSHADER_MIN_PRECISION = i32;
pub type D3DSHADER_MISCTYPE_OFFSETS = i32;
pub type D3DSHADER_PARAM_REGISTER_TYPE = i32;
pub type D3DSHADER_PARAM_SRCMOD_TYPE = i32;
pub const D3DSHADE_FLAT: D3DSHADEMODE = 1;
pub const D3DSHADE_FORCE_DWORD: D3DSHADEMODE = 2147483647;
pub const D3DSHADE_GOURAUD: D3DSHADEMODE = 2;
pub const D3DSHADE_PHONG: D3DSHADEMODE = 3;
pub const D3DSIO_ABS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 35;
pub const D3DSIO_ADD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 2;
pub const D3DSIO_BEM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 89;
pub const D3DSIO_BREAK: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 44;
pub const D3DSIO_BREAKC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 45;
pub const D3DSIO_BREAKP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 96;
pub const D3DSIO_CALL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 25;
pub const D3DSIO_CALLNZ: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 26;
pub const D3DSIO_CMP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 88;
pub const D3DSIO_CND: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 80;
pub const D3DSIO_COMMENT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 65534;
pub const D3DSIO_CRS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 33;
pub const D3DSIO_DCL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 31;
pub const D3DSIO_DEF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 81;
pub const D3DSIO_DEFB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 47;
pub const D3DSIO_DEFI: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 48;
pub const D3DSIO_DP2ADD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 90;
pub const D3DSIO_DP3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 8;
pub const D3DSIO_DP4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 9;
pub const D3DSIO_DST: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 17;
pub const D3DSIO_DSX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 91;
pub const D3DSIO_DSY: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 92;
pub const D3DSIO_ELSE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 42;
pub const D3DSIO_END: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 65535;
pub const D3DSIO_ENDIF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 43;
pub const D3DSIO_ENDLOOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 29;
pub const D3DSIO_ENDREP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 39;
pub const D3DSIO_EXP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 14;
pub const D3DSIO_EXPP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 78;
pub const D3DSIO_FORCE_DWORD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 2147483647;
pub const D3DSIO_FRC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 19;
pub const D3DSIO_IF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 40;
pub const D3DSIO_IFC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 41;
pub const D3DSIO_LABEL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 30;
pub const D3DSIO_LIT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 16;
pub const D3DSIO_LOG: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 15;
pub const D3DSIO_LOGP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 79;
pub const D3DSIO_LOOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 27;
pub const D3DSIO_LRP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 18;
pub const D3DSIO_M3x2: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 24;
pub const D3DSIO_M3x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 23;
pub const D3DSIO_M3x4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 22;
pub const D3DSIO_M4x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 21;
pub const D3DSIO_M4x4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 20;
pub const D3DSIO_MAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 4;
pub const D3DSIO_MAX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 11;
pub const D3DSIO_MIN: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 10;
pub const D3DSIO_MOV: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 1;
pub const D3DSIO_MOVA: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 46;
pub const D3DSIO_MUL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 5;
pub const D3DSIO_NOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 0;
pub const D3DSIO_NRM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 36;
pub const D3DSIO_PHASE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 65533;
pub const D3DSIO_POW: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 32;
pub const D3DSIO_RCP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 6;
pub const D3DSIO_REP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 38;
pub const D3DSIO_RESERVED0: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 75;
pub const D3DSIO_RET: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 28;
pub const D3DSIO_RSQ: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 7;
pub const D3DSIO_SETP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 94;
pub const D3DSIO_SGE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 13;
pub const D3DSIO_SGN: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 34;
pub const D3DSIO_SINCOS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 37;
pub const D3DSIO_SLT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 12;
pub const D3DSIO_SUB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 3;
pub const D3DSIO_TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 66;
pub const D3DSIO_TEXBEM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 67;
pub const D3DSIO_TEXBEML: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 68;
pub const D3DSIO_TEXCOORD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 64;
pub const D3DSIO_TEXDEPTH: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 87;
pub const D3DSIO_TEXDP3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 85;
pub const D3DSIO_TEXDP3TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 83;
pub const D3DSIO_TEXKILL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 65;
pub const D3DSIO_TEXLDD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 93;
pub const D3DSIO_TEXLDL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 95;
pub const D3DSIO_TEXM3x2DEPTH: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 84;
pub const D3DSIO_TEXM3x2PAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 71;
pub const D3DSIO_TEXM3x2TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 72;
pub const D3DSIO_TEXM3x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 86;
pub const D3DSIO_TEXM3x3PAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 73;
pub const D3DSIO_TEXM3x3SPEC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 76;
pub const D3DSIO_TEXM3x3TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 74;
pub const D3DSIO_TEXM3x3VSPEC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 77;
pub const D3DSIO_TEXREG2AR: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 69;
pub const D3DSIO_TEXREG2GB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 70;
pub const D3DSIO_TEXREG2RGB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = 82;
pub const D3DSI_COISSUE: u32 = 1073741824;
pub const D3DSI_COMMENTSIZE_MASK: u32 = 2147418112;
pub const D3DSI_COMMENTSIZE_SHIFT: u32 = 16;
pub const D3DSI_INSTLENGTH_MASK: u32 = 251658240;
pub const D3DSI_INSTLENGTH_SHIFT: u32 = 24;
pub const D3DSI_OPCODE_MASK: u32 = 65535;
pub const D3DSI_TEXLD_BIAS: u32 = 131072;
pub const D3DSI_TEXLD_PROJECT: u32 = 65536;
pub const D3DSMO_FACE: D3DSHADER_MISCTYPE_OFFSETS = 1;
pub const D3DSMO_POSITION: D3DSHADER_MISCTYPE_OFFSETS = 0;
pub const D3DSPC_EQ: D3DSHADER_COMPARISON = 2;
pub const D3DSPC_GE: D3DSHADER_COMPARISON = 3;
pub const D3DSPC_GT: D3DSHADER_COMPARISON = 1;
pub const D3DSPC_LE: D3DSHADER_COMPARISON = 6;
pub const D3DSPC_LT: D3DSHADER_COMPARISON = 4;
pub const D3DSPC_NE: D3DSHADER_COMPARISON = 5;
pub const D3DSPC_RESERVED0: D3DSHADER_COMPARISON = 0;
pub const D3DSPC_RESERVED1: D3DSHADER_COMPARISON = 7;
pub const D3DSPDM_MSAMPCENTROID: u32 = 4194304;
pub const D3DSPDM_NONE: u32 = 0;
pub const D3DSPDM_PARTIALPRECISION: u32 = 2097152;
pub const D3DSPDM_SATURATE: u32 = 1048576;
pub const D3DSPD_IUNKNOWN: u32 = 1;
pub const D3DSPR_ADDR: D3DSHADER_PARAM_REGISTER_TYPE = 3;
pub const D3DSPR_ATTROUT: D3DSHADER_PARAM_REGISTER_TYPE = 5;
pub const D3DSPR_COLOROUT: D3DSHADER_PARAM_REGISTER_TYPE = 8;
pub const D3DSPR_CONST: D3DSHADER_PARAM_REGISTER_TYPE = 2;
pub const D3DSPR_CONST2: D3DSHADER_PARAM_REGISTER_TYPE = 11;
pub const D3DSPR_CONST3: D3DSHADER_PARAM_REGISTER_TYPE = 12;
pub const D3DSPR_CONST4: D3DSHADER_PARAM_REGISTER_TYPE = 13;
pub const D3DSPR_CONSTBOOL: D3DSHADER_PARAM_REGISTER_TYPE = 14;
pub const D3DSPR_CONSTINT: D3DSHADER_PARAM_REGISTER_TYPE = 7;
pub const D3DSPR_DEPTHOUT: D3DSHADER_PARAM_REGISTER_TYPE = 9;
pub const D3DSPR_FORCE_DWORD: D3DSHADER_PARAM_REGISTER_TYPE = 2147483647;
pub const D3DSPR_INPUT: D3DSHADER_PARAM_REGISTER_TYPE = 1;
pub const D3DSPR_LABEL: D3DSHADER_PARAM_REGISTER_TYPE = 18;
pub const D3DSPR_LOOP: D3DSHADER_PARAM_REGISTER_TYPE = 15;
pub const D3DSPR_MISCTYPE: D3DSHADER_PARAM_REGISTER_TYPE = 17;
pub const D3DSPR_OUTPUT: D3DSHADER_PARAM_REGISTER_TYPE = 6;
pub const D3DSPR_PREDICATE: D3DSHADER_PARAM_REGISTER_TYPE = 19;
pub const D3DSPR_RASTOUT: D3DSHADER_PARAM_REGISTER_TYPE = 4;
pub const D3DSPR_SAMPLER: D3DSHADER_PARAM_REGISTER_TYPE = 10;
pub const D3DSPR_TEMP: D3DSHADER_PARAM_REGISTER_TYPE = 0;
pub const D3DSPR_TEMPFLOAT16: D3DSHADER_PARAM_REGISTER_TYPE = 16;
pub const D3DSPR_TEXCRDOUT: D3DSHADER_PARAM_REGISTER_TYPE = 6;
pub const D3DSPR_TEXTURE: D3DSHADER_PARAM_REGISTER_TYPE = 3;
pub const D3DSPSM_ABS: D3DSHADER_PARAM_SRCMOD_TYPE = 184549376;
pub const D3DSPSM_ABSNEG: D3DSHADER_PARAM_SRCMOD_TYPE = 201326592;
pub const D3DSPSM_BIAS: D3DSHADER_PARAM_SRCMOD_TYPE = 33554432;
pub const D3DSPSM_BIASNEG: D3DSHADER_PARAM_SRCMOD_TYPE = 50331648;
pub const D3DSPSM_COMP: D3DSHADER_PARAM_SRCMOD_TYPE = 100663296;
pub const D3DSPSM_DW: D3DSHADER_PARAM_SRCMOD_TYPE = 167772160;
pub const D3DSPSM_DZ: D3DSHADER_PARAM_SRCMOD_TYPE = 150994944;
pub const D3DSPSM_FORCE_DWORD: D3DSHADER_PARAM_SRCMOD_TYPE = 2147483647;
pub const D3DSPSM_NEG: D3DSHADER_PARAM_SRCMOD_TYPE = 16777216;
pub const D3DSPSM_NONE: D3DSHADER_PARAM_SRCMOD_TYPE = 0;
pub const D3DSPSM_NOT: D3DSHADER_PARAM_SRCMOD_TYPE = 218103808;
pub const D3DSPSM_SIGN: D3DSHADER_PARAM_SRCMOD_TYPE = 67108864;
pub const D3DSPSM_SIGNNEG: D3DSHADER_PARAM_SRCMOD_TYPE = 83886080;
pub const D3DSPSM_X2: D3DSHADER_PARAM_SRCMOD_TYPE = 117440512;
pub const D3DSPSM_X2NEG: D3DSHADER_PARAM_SRCMOD_TYPE = 134217728;
pub const D3DSP_DCL_USAGEINDEX_MASK: u32 = 983040;
pub const D3DSP_DCL_USAGEINDEX_SHIFT: u32 = 16;
pub const D3DSP_DCL_USAGE_MASK: u32 = 15;
pub const D3DSP_DCL_USAGE_SHIFT: u32 = 0;
pub const D3DSP_DSTMOD_MASK: u32 = 15728640;
pub const D3DSP_DSTMOD_SHIFT: u32 = 20;
pub const D3DSP_DSTSHIFT_MASK: u32 = 251658240;
pub const D3DSP_DSTSHIFT_SHIFT: u32 = 24;
pub const D3DSP_MIN_PRECISION_MASK: u32 = 49152;
pub const D3DSP_MIN_PRECISION_SHIFT: u32 = 14;
pub const D3DSP_NOSWIZZLE: u32 = 14942208;
pub const D3DSP_OPCODESPECIFICCONTROL_MASK: u32 = 16711680;
pub const D3DSP_OPCODESPECIFICCONTROL_SHIFT: u32 = 16;
pub const D3DSP_REGNUM_MASK: u32 = 2047;
pub const D3DSP_REGTYPE_MASK: u32 = 1879048192;
pub const D3DSP_REGTYPE_MASK2: u32 = 6144;
pub const D3DSP_REGTYPE_SHIFT: u32 = 28;
pub const D3DSP_REGTYPE_SHIFT2: u32 = 8;
pub const D3DSP_REPLICATEALPHA: u32 = 16711680;
pub const D3DSP_REPLICATEBLUE: u32 = 11141120;
pub const D3DSP_REPLICATEGREEN: u32 = 5570560;
pub const D3DSP_REPLICATERED: u32 = 0;
pub const D3DSP_SRCMOD_MASK: u32 = 251658240;
pub const D3DSP_SRCMOD_SHIFT: u32 = 24;
pub const D3DSP_SWIZZLE_MASK: u32 = 16711680;
pub const D3DSP_SWIZZLE_SHIFT: u32 = 16;
pub const D3DSP_TEXTURETYPE_MASK: u32 = 2013265920;
pub const D3DSP_TEXTURETYPE_SHIFT: u32 = 27;
pub const D3DSP_WRITEMASK_0: u32 = 65536;
pub const D3DSP_WRITEMASK_1: u32 = 131072;
pub const D3DSP_WRITEMASK_2: u32 = 262144;
pub const D3DSP_WRITEMASK_3: u32 = 524288;
pub const D3DSP_WRITEMASK_ALL: u32 = 983040;
pub const D3DSRO_FOG: D3DVS_RASTOUT_OFFSETS = 1;
pub const D3DSRO_FORCE_DWORD: D3DVS_RASTOUT_OFFSETS = 2147483647;
pub const D3DSRO_POINT_SIZE: D3DVS_RASTOUT_OFFSETS = 2;
pub const D3DSRO_POSITION: D3DVS_RASTOUT_OFFSETS = 0;
pub type D3DSTATEBLOCKTYPE = i32;
pub const D3DSTENCILCAPS_DECR: u32 = 128;
pub const D3DSTENCILCAPS_DECRSAT: u32 = 16;
pub const D3DSTENCILCAPS_INCR: u32 = 64;
pub const D3DSTENCILCAPS_INCRSAT: u32 = 8;
pub const D3DSTENCILCAPS_INVERT: u32 = 32;
pub const D3DSTENCILCAPS_KEEP: u32 = 1;
pub const D3DSTENCILCAPS_REPLACE: u32 = 4;
pub const D3DSTENCILCAPS_TWOSIDED: u32 = 256;
pub const D3DSTENCILCAPS_ZERO: u32 = 2;
pub type D3DSTENCILOP = i32;
pub const D3DSTENCILOP_DECR: D3DSTENCILOP = 8;
pub const D3DSTENCILOP_DECRSAT: D3DSTENCILOP = 5;
pub const D3DSTENCILOP_FORCE_DWORD: D3DSTENCILOP = 2147483647;
pub const D3DSTENCILOP_INCR: D3DSTENCILOP = 7;
pub const D3DSTENCILOP_INCRSAT: D3DSTENCILOP = 4;
pub const D3DSTENCILOP_INVERT: D3DSTENCILOP = 6;
pub const D3DSTENCILOP_KEEP: D3DSTENCILOP = 1;
pub const D3DSTENCILOP_REPLACE: D3DSTENCILOP = 3;
pub const D3DSTENCILOP_ZERO: D3DSTENCILOP = 2;
pub const D3DSTREAMSOURCE_INDEXEDDATA: u32 = 1073741824;
pub const D3DSTREAMSOURCE_INSTANCEDATA: i32 = -2147483648;
pub const D3DSTT_2D: D3DSAMPLER_TEXTURE_TYPE = 268435456;
pub const D3DSTT_CUBE: D3DSAMPLER_TEXTURE_TYPE = 402653184;
pub const D3DSTT_FORCE_DWORD: D3DSAMPLER_TEXTURE_TYPE = 2147483647;
pub const D3DSTT_UNKNOWN: D3DSAMPLER_TEXTURE_TYPE = 0;
pub const D3DSTT_VOLUME: D3DSAMPLER_TEXTURE_TYPE = 536870912;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub type D3DSWAPEFFECT = i32;
pub const D3DSWAPEFFECT_COPY: D3DSWAPEFFECT = 3;
pub const D3DSWAPEFFECT_DISCARD: D3DSWAPEFFECT = 1;
pub const D3DSWAPEFFECT_FLIP: D3DSWAPEFFECT = 2;
pub const D3DSWAPEFFECT_FLIPEX: D3DSWAPEFFECT = 5;
pub const D3DSWAPEFFECT_FORCE_DWORD: D3DSWAPEFFECT = 2147483647;
pub const D3DSWAPEFFECT_OVERLAY: D3DSWAPEFFECT = 4;
pub const D3DTADDRESS_BORDER: D3DTEXTUREADDRESS = 4;
pub const D3DTADDRESS_CLAMP: D3DTEXTUREADDRESS = 3;
pub const D3DTADDRESS_FORCE_DWORD: D3DTEXTUREADDRESS = 2147483647;
pub const D3DTADDRESS_MIRROR: D3DTEXTUREADDRESS = 2;
pub const D3DTADDRESS_MIRRORONCE: D3DTEXTUREADDRESS = 5;
pub const D3DTADDRESS_WRAP: D3DTEXTUREADDRESS = 1;
pub const D3DTA_ALPHAREPLICATE: u32 = 32;
pub const D3DTA_COMPLEMENT: u32 = 16;
pub const D3DTA_CONSTANT: u32 = 6;
pub const D3DTA_CURRENT: u32 = 1;
pub const D3DTA_DIFFUSE: u32 = 0;
pub const D3DTA_SELECTMASK: u32 = 15;
pub const D3DTA_SPECULAR: u32 = 4;
pub const D3DTA_TEMP: u32 = 5;
pub const D3DTA_TEXTURE: u32 = 2;
pub const D3DTA_TFACTOR: u32 = 3;
pub const D3DTEXF_ANISOTROPIC: D3DTEXTUREFILTERTYPE = 3;
pub const D3DTEXF_CONVOLUTIONMONO: D3DTEXTUREFILTERTYPE = 8;
pub const D3DTEXF_FORCE_DWORD: D3DTEXTUREFILTERTYPE = 2147483647;
pub const D3DTEXF_GAUSSIANQUAD: D3DTEXTUREFILTERTYPE = 7;
pub const D3DTEXF_LINEAR: D3DTEXTUREFILTERTYPE = 2;
pub const D3DTEXF_NONE: D3DTEXTUREFILTERTYPE = 0;
pub const D3DTEXF_POINT: D3DTEXTUREFILTERTYPE = 1;
pub const D3DTEXF_PYRAMIDALQUAD: D3DTEXTUREFILTERTYPE = 6;
pub const D3DTEXOPCAPS_ADD: u32 = 64;
pub const D3DTEXOPCAPS_ADDSIGNED: u32 = 128;
pub const D3DTEXOPCAPS_ADDSIGNED2X: u32 = 256;
pub const D3DTEXOPCAPS_ADDSMOOTH: u32 = 1024;
pub const D3DTEXOPCAPS_BLENDCURRENTALPHA: u32 = 32768;
pub const D3DTEXOPCAPS_BLENDDIFFUSEALPHA: u32 = 2048;
pub const D3DTEXOPCAPS_BLENDFACTORALPHA: u32 = 8192;
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHA: u32 = 4096;
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHAPM: u32 = 16384;
pub const D3DTEXOPCAPS_BUMPENVMAP: u32 = 2097152;
pub const D3DTEXOPCAPS_BUMPENVMAPLUMINANCE: u32 = 4194304;
pub const D3DTEXOPCAPS_DISABLE: u32 = 1;
pub const D3DTEXOPCAPS_DOTPRODUCT3: u32 = 8388608;
pub const D3DTEXOPCAPS_LERP: u32 = 33554432;
pub const D3DTEXOPCAPS_MODULATE: u32 = 8;
pub const D3DTEXOPCAPS_MODULATE2X: u32 = 16;
pub const D3DTEXOPCAPS_MODULATE4X: u32 = 32;
pub const D3DTEXOPCAPS_MODULATEALPHA_ADDCOLOR: u32 = 131072;
pub const D3DTEXOPCAPS_MODULATECOLOR_ADDALPHA: u32 = 262144;
pub const D3DTEXOPCAPS_MODULATEINVALPHA_ADDCOLOR: u32 = 524288;
pub const D3DTEXOPCAPS_MODULATEINVCOLOR_ADDALPHA: u32 = 1048576;
pub const D3DTEXOPCAPS_MULTIPLYADD: u32 = 16777216;
pub const D3DTEXOPCAPS_PREMODULATE: u32 = 65536;
pub const D3DTEXOPCAPS_SELECTARG1: u32 = 2;
pub const D3DTEXOPCAPS_SELECTARG2: u32 = 4;
pub const D3DTEXOPCAPS_SUBTRACT: u32 = 512;
pub type D3DTEXTUREADDRESS = i32;
pub type D3DTEXTUREFILTERTYPE = i32;
pub type D3DTEXTUREOP = i32;
pub type D3DTEXTURESTAGESTATETYPE = i32;
pub type D3DTEXTURETRANSFORMFLAGS = i32;
pub const D3DTOP_ADD: D3DTEXTUREOP = 7;
pub const D3DTOP_ADDSIGNED: D3DTEXTUREOP = 8;
pub const D3DTOP_ADDSIGNED2X: D3DTEXTUREOP = 9;
pub const D3DTOP_ADDSMOOTH: D3DTEXTUREOP = 11;
pub const D3DTOP_BLENDCURRENTALPHA: D3DTEXTUREOP = 16;
pub const D3DTOP_BLENDDIFFUSEALPHA: D3DTEXTUREOP = 12;
pub const D3DTOP_BLENDFACTORALPHA: D3DTEXTUREOP = 14;
pub const D3DTOP_BLENDTEXTUREALPHA: D3DTEXTUREOP = 13;
pub const D3DTOP_BLENDTEXTUREALPHAPM: D3DTEXTUREOP = 15;
pub const D3DTOP_BUMPENVMAP: D3DTEXTUREOP = 22;
pub const D3DTOP_BUMPENVMAPLUMINANCE: D3DTEXTUREOP = 23;
pub const D3DTOP_DISABLE: D3DTEXTUREOP = 1;
pub const D3DTOP_DOTPRODUCT3: D3DTEXTUREOP = 24;
pub const D3DTOP_FORCE_DWORD: D3DTEXTUREOP = 2147483647;
pub const D3DTOP_LERP: D3DTEXTUREOP = 26;
pub const D3DTOP_MODULATE: D3DTEXTUREOP = 4;
pub const D3DTOP_MODULATE2X: D3DTEXTUREOP = 5;
pub const D3DTOP_MODULATE4X: D3DTEXTUREOP = 6;
pub const D3DTOP_MODULATEALPHA_ADDCOLOR: D3DTEXTUREOP = 18;
pub const D3DTOP_MODULATECOLOR_ADDALPHA: D3DTEXTUREOP = 19;
pub const D3DTOP_MODULATEINVALPHA_ADDCOLOR: D3DTEXTUREOP = 20;
pub const D3DTOP_MODULATEINVCOLOR_ADDALPHA: D3DTEXTUREOP = 21;
pub const D3DTOP_MULTIPLYADD: D3DTEXTUREOP = 25;
pub const D3DTOP_PREMODULATE: D3DTEXTUREOP = 17;
pub const D3DTOP_SELECTARG1: D3DTEXTUREOP = 2;
pub const D3DTOP_SELECTARG2: D3DTEXTUREOP = 3;
pub const D3DTOP_SUBTRACT: D3DTEXTUREOP = 10;
pub type D3DTRANSFORMSTATETYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DTRIPATCH_INFO {
    pub StartVertexOffset: u32,
    pub NumVertices: u32,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
pub const D3DTSS_ALPHAARG0: D3DTEXTURESTAGESTATETYPE = 27;
pub const D3DTSS_ALPHAARG1: D3DTEXTURESTAGESTATETYPE = 5;
pub const D3DTSS_ALPHAARG2: D3DTEXTURESTAGESTATETYPE = 6;
pub const D3DTSS_ALPHAOP: D3DTEXTURESTAGESTATETYPE = 4;
pub const D3DTSS_BUMPENVLOFFSET: D3DTEXTURESTAGESTATETYPE = 23;
pub const D3DTSS_BUMPENVLSCALE: D3DTEXTURESTAGESTATETYPE = 22;
pub const D3DTSS_BUMPENVMAT00: D3DTEXTURESTAGESTATETYPE = 7;
pub const D3DTSS_BUMPENVMAT01: D3DTEXTURESTAGESTATETYPE = 8;
pub const D3DTSS_BUMPENVMAT10: D3DTEXTURESTAGESTATETYPE = 9;
pub const D3DTSS_BUMPENVMAT11: D3DTEXTURESTAGESTATETYPE = 10;
pub const D3DTSS_COLORARG0: D3DTEXTURESTAGESTATETYPE = 26;
pub const D3DTSS_COLORARG1: D3DTEXTURESTAGESTATETYPE = 2;
pub const D3DTSS_COLORARG2: D3DTEXTURESTAGESTATETYPE = 3;
pub const D3DTSS_COLOROP: D3DTEXTURESTAGESTATETYPE = 1;
pub const D3DTSS_CONSTANT: D3DTEXTURESTAGESTATETYPE = 32;
pub const D3DTSS_FORCE_DWORD: D3DTEXTURESTAGESTATETYPE = 2147483647;
pub const D3DTSS_RESULTARG: D3DTEXTURESTAGESTATETYPE = 28;
pub const D3DTSS_TCI_CAMERASPACENORMAL: u32 = 65536;
pub const D3DTSS_TCI_CAMERASPACEPOSITION: u32 = 131072;
pub const D3DTSS_TCI_CAMERASPACEREFLECTIONVECTOR: u32 = 196608;
pub const D3DTSS_TCI_PASSTHRU: u32 = 0;
pub const D3DTSS_TCI_SPHEREMAP: u32 = 262144;
pub const D3DTSS_TEXCOORDINDEX: D3DTEXTURESTAGESTATETYPE = 11;
pub const D3DTSS_TEXTURETRANSFORMFLAGS: D3DTEXTURESTAGESTATETYPE = 24;
pub const D3DTS_FORCE_DWORD: D3DTRANSFORMSTATETYPE = 2147483647;
pub const D3DTS_PROJECTION: D3DTRANSFORMSTATETYPE = 3;
pub const D3DTS_TEXTURE0: D3DTRANSFORMSTATETYPE = 16;
pub const D3DTS_TEXTURE1: D3DTRANSFORMSTATETYPE = 17;
pub const D3DTS_TEXTURE2: D3DTRANSFORMSTATETYPE = 18;
pub const D3DTS_TEXTURE3: D3DTRANSFORMSTATETYPE = 19;
pub const D3DTS_TEXTURE4: D3DTRANSFORMSTATETYPE = 20;
pub const D3DTS_TEXTURE5: D3DTRANSFORMSTATETYPE = 21;
pub const D3DTS_TEXTURE6: D3DTRANSFORMSTATETYPE = 22;
pub const D3DTS_TEXTURE7: D3DTRANSFORMSTATETYPE = 23;
pub const D3DTS_VIEW: D3DTRANSFORMSTATETYPE = 2;
pub const D3DTS_WORLD: u32 = 256;
pub const D3DTS_WORLD1: u32 = 257;
pub const D3DTS_WORLD2: u32 = 258;
pub const D3DTS_WORLD3: u32 = 259;
pub const D3DTTFF_COUNT1: D3DTEXTURETRANSFORMFLAGS = 1;
pub const D3DTTFF_COUNT2: D3DTEXTURETRANSFORMFLAGS = 2;
pub const D3DTTFF_COUNT3: D3DTEXTURETRANSFORMFLAGS = 3;
pub const D3DTTFF_COUNT4: D3DTEXTURETRANSFORMFLAGS = 4;
pub const D3DTTFF_DISABLE: D3DTEXTURETRANSFORMFLAGS = 0;
pub const D3DTTFF_FORCE_DWORD: D3DTEXTURETRANSFORMFLAGS = 2147483647;
pub const D3DTTFF_PROJECTED: D3DTEXTURETRANSFORMFLAGS = 256;
pub const D3DUSAGE_AUTOGENMIPMAP: u32 = 1024;
pub const D3DUSAGE_DEPTHSTENCIL: u32 = 2;
pub const D3DUSAGE_DMAP: u32 = 16384;
pub const D3DUSAGE_DONOTCLIP: u32 = 32;
pub const D3DUSAGE_DYNAMIC: u32 = 512;
pub const D3DUSAGE_NONSECURE: u32 = 8388608;
pub const D3DUSAGE_NPATCHES: u32 = 256;
pub const D3DUSAGE_POINTS: u32 = 64;
pub const D3DUSAGE_QUERY_FILTER: u32 = 131072;
pub const D3DUSAGE_QUERY_LEGACYBUMPMAP: u32 = 32768;
pub const D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING: u32 = 524288;
pub const D3DUSAGE_QUERY_SRGBREAD: u32 = 65536;
pub const D3DUSAGE_QUERY_SRGBWRITE: u32 = 262144;
pub const D3DUSAGE_QUERY_VERTEXTEXTURE: u32 = 1048576;
pub const D3DUSAGE_QUERY_WRAPANDMIP: u32 = 2097152;
pub const D3DUSAGE_RENDERTARGET: u32 = 1;
pub const D3DUSAGE_RESTRICTED_CONTENT: u32 = 2048;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE: u32 = 8192;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER: u32 = 4096;
pub const D3DUSAGE_RTPATCHES: u32 = 128;
pub const D3DUSAGE_SOFTWAREPROCESSING: u32 = 16;
pub const D3DUSAGE_TEXTAPI: u32 = 268435456;
pub const D3DUSAGE_WRITEONLY: u32 = 8;
pub const D3DVBF_0WEIGHTS: D3DVERTEXBLENDFLAGS = 256;
pub const D3DVBF_1WEIGHTS: D3DVERTEXBLENDFLAGS = 1;
pub const D3DVBF_2WEIGHTS: D3DVERTEXBLENDFLAGS = 2;
pub const D3DVBF_3WEIGHTS: D3DVERTEXBLENDFLAGS = 3;
pub const D3DVBF_DISABLE: D3DVERTEXBLENDFLAGS = 0;
pub const D3DVBF_FORCE_DWORD: D3DVERTEXBLENDFLAGS = 2147483647;
pub const D3DVBF_TWEENING: D3DVERTEXBLENDFLAGS = 255;
pub type D3DVERTEXBLENDFLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DVERTEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Size: u32,
    pub FVF: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DVERTEXELEMENT9 {
    pub Stream: u16,
    pub Offset: u16,
    pub Type: u8,
    pub Method: u8,
    pub Usage: u8,
    pub UsageIndex: u8,
}
pub const D3DVERTEXTEXTURESAMPLER0: u32 = 257;
pub const D3DVERTEXTEXTURESAMPLER1: u32 = 258;
pub const D3DVERTEXTEXTURESAMPLER2: u32 = 259;
pub const D3DVERTEXTEXTURESAMPLER3: u32 = 260;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DVIEWPORT9 {
    pub X: u32,
    pub Y: u32,
    pub Width: u32,
    pub Height: u32,
    pub MinZ: f32,
    pub MaxZ: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DVOLUME_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
pub const D3DVS20CAPS_PREDICATION: u32 = 1;
pub const D3DVS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24;
pub const D3DVS20_MAX_NUMTEMPS: u32 = 32;
pub const D3DVS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4;
pub const D3DVS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0;
pub const D3DVS20_MIN_NUMTEMPS: u32 = 12;
pub const D3DVS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DVSHADERCAPS2_0 {
    pub Caps: u32,
    pub DynamicFlowControlDepth: i32,
    pub NumTemps: i32,
    pub StaticFlowControlDepth: i32,
}
pub const D3DVS_ADDRESSMODE_MASK: u32 = 8192;
pub const D3DVS_ADDRESSMODE_SHIFT: u32 = 13;
pub type D3DVS_ADDRESSMODE_TYPE = i32;
pub const D3DVS_ADDRMODE_ABSOLUTE: D3DVS_ADDRESSMODE_TYPE = 0;
pub const D3DVS_ADDRMODE_FORCE_DWORD: D3DVS_ADDRESSMODE_TYPE = 2147483647;
pub const D3DVS_ADDRMODE_RELATIVE: D3DVS_ADDRESSMODE_TYPE = 8192;
pub const D3DVS_NOSWIZZLE: u32 = 14942208;
pub type D3DVS_RASTOUT_OFFSETS = i32;
pub const D3DVS_SWIZZLE_MASK: u32 = 16711680;
pub const D3DVS_SWIZZLE_SHIFT: u32 = 16;
pub const D3DVS_W_W: u32 = 12582912;
pub const D3DVS_W_X: u32 = 0;
pub const D3DVS_W_Y: u32 = 4194304;
pub const D3DVS_W_Z: u32 = 8388608;
pub const D3DVS_X_W: u32 = 196608;
pub const D3DVS_X_X: u32 = 0;
pub const D3DVS_X_Y: u32 = 65536;
pub const D3DVS_X_Z: u32 = 131072;
pub const D3DVS_Y_W: u32 = 786432;
pub const D3DVS_Y_X: u32 = 0;
pub const D3DVS_Y_Y: u32 = 262144;
pub const D3DVS_Y_Z: u32 = 524288;
pub const D3DVS_Z_W: u32 = 3145728;
pub const D3DVS_Z_X: u32 = 0;
pub const D3DVS_Z_Y: u32 = 1048576;
pub const D3DVS_Z_Z: u32 = 2097152;
pub const D3DVTXPCAPS_DIRECTIONALLIGHTS: u32 = 8;
pub const D3DVTXPCAPS_LOCALVIEWER: u32 = 32;
pub const D3DVTXPCAPS_MATERIALSOURCE7: u32 = 2;
pub const D3DVTXPCAPS_NO_TEXGEN_NONLOCALVIEWER: u32 = 512;
pub const D3DVTXPCAPS_POSITIONALLIGHTS: u32 = 16;
pub const D3DVTXPCAPS_TEXGEN: u32 = 1;
pub const D3DVTXPCAPS_TEXGEN_SPHEREMAP: u32 = 256;
pub const D3DVTXPCAPS_TWEENING: u32 = 64;
pub const D3DWRAPCOORD_0: u32 = 1;
pub const D3DWRAPCOORD_1: u32 = 2;
pub const D3DWRAPCOORD_2: u32 = 4;
pub const D3DWRAPCOORD_3: u32 = 8;
pub const D3DWRAP_U: u32 = 1;
pub const D3DWRAP_V: u32 = 2;
pub const D3DWRAP_W: u32 = 4;
pub type D3DZBUFFERTYPE = i32;
pub const D3DZB_FALSE: D3DZBUFFERTYPE = 0;
pub const D3DZB_FORCE_DWORD: D3DZBUFFERTYPE = 2147483647;
pub const D3DZB_TRUE: D3DZBUFFERTYPE = 1;
pub const D3DZB_USEW: D3DZBUFFERTYPE = 2;
pub const D3D_MAX_SIMULTANEOUS_RENDERTARGETS: u32 = 4;
pub const D3D_OK: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D_OMAC {
    pub Omac: [u8; 16],
}
impl Default for D3D_OMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D_OMAC_SIZE: u32 = 16;
pub const D3D_SDK_VERSION: u32 = 32;
pub const DIRECT3D_VERSION: u32 = 2304;
pub const IID_HelperName: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe4a36723_fdfe_4b22_b146_3c04c07f4cc8);
pub type LPD3DDEVINFO_D3DVERTEXSTATS = *mut D3DDEVINFO_D3DVERTEXSTATS;
pub type LPD3DDEVINFO_RESOURCEMANAGER = *mut D3DDEVINFO_RESOURCEMANAGER;
pub type LPD3DDEVINFO_VCACHE = *mut D3DDEVINFO_VCACHE;
pub type LPD3DVERTEXELEMENT9 = *mut D3DVERTEXELEMENT9;
pub const MAXD3DDECLLENGTH: u32 = 64;
pub const MAXD3DDECLMETHOD: u32 = 6;
pub const MAXD3DDECLTYPE: u32 = 17;
pub const MAXD3DDECLUSAGE: u32 = 13;
pub const MAXD3DDECLUSAGEINDEX: u32 = 15;
pub const MAX_DEVICE_IDENTIFIER_STRING: u32 = 512;
pub const PROCESSIDTYPE_DWM: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = 1;
pub const PROCESSIDTYPE_HANDLE: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = 2;
pub const PROCESSIDTYPE_UNKNOWN: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = 0;
pub const S_NOT_RESIDENT: u32 = 141953141;
pub const S_PRESENT_MODE_CHANGED: u32 = 141953143;
pub const S_PRESENT_OCCLUDED: u32 = 141953144;
pub const S_RESIDENT_IN_SHARED_MEMORY: u32 = 141953142;
