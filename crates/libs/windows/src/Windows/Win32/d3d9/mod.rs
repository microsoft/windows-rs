#[cfg(feature = "dsound")]
#[inline]
pub unsafe fn D3DPERF_BeginEvent(col: super::D3DCOLOR, wszname: &windows_core::PCWSTR) -> i32 {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_BeginEvent(col : super::D3DCOLOR, wszname : windows_core::PCWSTR) -> i32);
    unsafe { D3DPERF_BeginEvent(col, core::mem::transmute_copy(wszname)) }
}
#[inline]
pub unsafe fn D3DPERF_EndEvent() -> i32 {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_EndEvent() -> i32);
    unsafe { D3DPERF_EndEvent() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn D3DPERF_GetStatus() -> super::DWORD {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_GetStatus() -> super::DWORD);
    unsafe { D3DPERF_GetStatus() }
}
#[inline]
pub unsafe fn D3DPERF_QueryRepeatFrame() -> windows_core::BOOL {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_QueryRepeatFrame() -> windows_core::BOOL);
    unsafe { D3DPERF_QueryRepeatFrame() }
}
#[cfg(feature = "dsound")]
#[inline]
pub unsafe fn D3DPERF_SetMarker(col: super::D3DCOLOR, wszname: &windows_core::PCWSTR) {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetMarker(col : super::D3DCOLOR, wszname : windows_core::PCWSTR));
    unsafe { D3DPERF_SetMarker(col, core::mem::transmute_copy(wszname)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn D3DPERF_SetOptions(dwoptions: super::DWORD) {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetOptions(dwoptions : super::DWORD));
    unsafe { D3DPERF_SetOptions(dwoptions) }
}
#[cfg(feature = "dsound")]
#[inline]
pub unsafe fn D3DPERF_SetRegion(col: super::D3DCOLOR, wszname: &windows_core::PCWSTR) {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetRegion(col : super::D3DCOLOR, wszname : windows_core::PCWSTR));
    unsafe { D3DPERF_SetRegion(col, core::mem::transmute_copy(wszname)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn Direct3DCreate9(sdkversion: super::UINT) -> Option<IDirect3D9> {
    windows_core::link!("d3d9.dll" "system" fn Direct3DCreate9(sdkversion : super::UINT) -> Option < IDirect3D9 >);
    unsafe { Direct3DCreate9(sdkversion) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn Direct3DCreate9Ex(sdkversion: super::UINT, param1: *mut Option<IDirect3D9Ex>) -> windows_core::HRESULT {
    windows_core::link!("d3d9.dll" "system" fn Direct3DCreate9Ex(sdkversion : super::UINT, param1 : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { Direct3DCreate9Ex(sdkversion, param1 as _) }
}
pub const D3D9_RESOURCE_PRIORITY_HIGH: u32 = 2684354560;
pub const D3D9_RESOURCE_PRIORITY_LOW: i32 = 1342177280;
pub const D3D9_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200;
pub const D3D9_RESOURCE_PRIORITY_MINIMUM: i32 = 671088640;
pub const D3D9_RESOURCE_PRIORITY_NORMAL: i32 = 2013265920;
pub const D3D9b_SDK_VERSION: i32 = 31;
pub const D3DADAPTER_DEFAULT: i32 = 0;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [i8; 512],
    pub Description: [i8; 512],
    pub DeviceName: [i8; 32],
    pub DriverVersion: super::LARGE_INTEGER,
    pub VendorId: super::DWORD,
    pub DeviceId: super::DWORD,
    pub SubSysId: super::DWORD,
    pub Revision: super::DWORD,
    pub DeviceIdentifier: windows_core::GUID,
    pub WHQLLevel: super::DWORD,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [i8; 512],
    pub Description: [i8; 512],
    pub DeviceName: [i8; 32],
    pub DriverVersion: super::LARGE_INTEGER,
    pub VendorId: super::DWORD,
    pub DeviceId: super::DWORD,
    pub SubSysId: super::DWORD,
    pub Revision: super::DWORD,
    pub DeviceIdentifier: windows_core::GUID,
    pub WHQLLevel: super::DWORD,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DAES_CTR_IV {
    pub IV: super::UINT64,
    pub Count: super::UINT64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DAES_CTR_IV {
    pub IV: super::UINT64,
    pub Count: super::UINT64,
}
pub type D3DAUTHENTICATEDCHANNELTYPE = i32;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub DXVA2DecodeHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub DeviceHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub DXVA2DecodeHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub DeviceHandle: super::HANDLE,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub StartSequenceQuery: super::UINT,
    pub StartSequenceConfigure: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub StartSequenceQuery: super::UINT,
    pub StartSequenceConfigure: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub Protections: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub Protections: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub ProcessIdentiferType: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::HANDLE,
    pub AllowAccess: windows_core::BOOL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub ProcessIdentiferType: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::HANDLE,
    pub AllowAccess: windows_core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::HANDLE,
    pub SequenceNumber: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::HANDLE,
    pub SequenceNumber: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::HANDLE,
    pub SequenceNumber: super::UINT,
    pub ReturnCode: windows_core::HRESULT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::HANDLE,
    pub SequenceNumber: super::UINT,
    pub ReturnCode: windows_core::HRESULT,
}
pub const D3DAUTHENTICATEDCHANNEL_D3D9: D3DAUTHENTICATEDCHANNELTYPE = 1;
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_HARDWARE: D3DAUTHENTICATEDCHANNELTYPE = 3;
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_SOFTWARE: D3DAUTHENTICATEDCHANNELTYPE = 2;
pub type D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = i32;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0,
    pub Value: super::UINT,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    pub fn ProtectionEnabled(&self) -> super::UINT {
        (self._bitfield) & 1
    }
    pub fn set_ProtectionEnabled(&mut self, value: super::UINT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn OverlayOrFullscreenRequired(&self) -> super::UINT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_OverlayOrFullscreenRequired(&mut self, value: super::UINT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::UINT {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::UINT) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0,
    pub Value: super::UINT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    pub fn ProtectionEnabled(&self) -> super::UINT {
        (self._bitfield) & 1
    }
    pub fn set_ProtectionEnabled(&mut self, value: super::UINT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn OverlayOrFullscreenRequired(&self) -> super::UINT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_OverlayOrFullscreenRequired(&mut self, value: super::UINT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::UINT {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::UINT) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DXVA2DecodeHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DXVA2DecodeHandle: super::HANDLE,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DXVA2DecodeHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub DeviceHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DXVA2DecodeHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub DeviceHandle: super::HANDLE,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::HANDLE,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumEncryptionGuids: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumEncryptionGuids: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub EncryptionGuidIndex: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub EncryptionGuidIndex: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuidIndex: super::UINT,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuidIndex: super::UINT,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub BusType: D3DBUSTYPE,
    pub bAccessibleInContiguousBlocks: windows_core::BOOL,
    pub bAccessibleInNonContiguousBlocks: windows_core::BOOL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub BusType: D3DBUSTYPE,
    pub bAccessibleInContiguousBlocks: windows_core::BOOL,
    pub bAccessibleInNonContiguousBlocks: windows_core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub NumOutputIDs: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub NumOutputIDs: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub OutputIDIndex: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub OutputIDIndex: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub OutputIDIndex: super::UINT,
    pub OutputID: super::UINT64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::HANDLE,
    pub CryptoSessionHandle: super::HANDLE,
    pub OutputIDIndex: super::UINT,
    pub OutputID: super::UINT64,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProtectionFlags: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProtectionFlags: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumRestrictedSharedResourceProcesses: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumRestrictedSharedResourceProcesses: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub ProcessIndex: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub ProcessIndex: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProcessIndex: super::UINT,
    pub ProcessIdentifer: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProcessIndex: super::UINT,
    pub ProcessIdentifer: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::HANDLE,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumUnrestrictedProtectedSharedResources: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumUnrestrictedProtectedSharedResources: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    pub QueryType: windows_core::GUID,
    pub hChannel: super::HANDLE,
    pub SequenceNumber: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    pub QueryType: windows_core::GUID,
    pub hChannel: super::HANDLE,
    pub SequenceNumber: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    pub omac: D3D_OMAC,
    pub QueryType: windows_core::GUID,
    pub hChannel: super::HANDLE,
    pub SequenceNumber: super::UINT,
    pub ReturnCode: windows_core::HRESULT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    pub omac: D3D_OMAC,
    pub QueryType: windows_core::GUID,
    pub hChannel: super::HANDLE,
    pub SequenceNumber: super::UINT,
    pub ReturnCode: windows_core::HRESULT,
}
pub const D3DAUTHENTICATEDCONFIGURE_CRYPTOSESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6346cc54_2cfc_4ad4_8224_d15837de7700);
pub const D3DAUTHENTICATEDCONFIGURE_ENCRYPTIONWHENACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0x41fff286_6ae0_4d43_9d55_a46e9efd158a);
pub const D3DAUTHENTICATEDCONFIGURE_INITIALIZE: windows_core::GUID = windows_core::GUID::from_u128(0x06114bdb_3523_470a_8dca_fbc2845154f0);
pub const D3DAUTHENTICATEDCONFIGURE_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0x50455658_3f47_4362_bf99_bfdfcde9ed29);
pub const D3DAUTHENTICATEDCONFIGURE_SHAREDRESOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x0772d047_1b40_48e8_9ca6_b5f510de9f01);
pub const D3DAUTHENTICATEDQUERY_ACCESSIBILITYATTRIBUTES: windows_core::GUID = windows_core::GUID::from_u128(0x6214d9d2_432c_4abb_9fce_216eea269e3b);
pub const D3DAUTHENTICATEDQUERY_CHANNELTYPE: windows_core::GUID = windows_core::GUID::from_u128(0xbc1b18a5_b1fb_42ab_bd94_b5828b4bf7be);
pub const D3DAUTHENTICATEDQUERY_CRYPTOSESSION: windows_core::GUID = windows_core::GUID::from_u128(0x2634499e_d018_4d74_ac17_7f724059528d);
pub const D3DAUTHENTICATEDQUERY_CURRENTENCRYPTIONWHENACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0xec1791c7_dad3_4f15_9ec3_faa93d60d4f0);
pub const D3DAUTHENTICATEDQUERY_DEVICEHANDLE: windows_core::GUID = windows_core::GUID::from_u128(0xec1c539d_8cff_4e2a_bcc4_f5692f99f480);
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUID: windows_core::GUID = windows_core::GUID::from_u128(0xf83a5958_e986_4bda_beb0_411f6a7a01b7);
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUIDCOUNT: windows_core::GUID = windows_core::GUID::from_u128(0xb30f7066_203c_4b07_93fc_ceaafd61241e);
pub const D3DAUTHENTICATEDQUERY_OUTPUTID: windows_core::GUID = windows_core::GUID::from_u128(0x839ddca3_9b4e_41e4_b053_892bd2a11ee7);
pub const D3DAUTHENTICATEDQUERY_OUTPUTIDCOUNT: windows_core::GUID = windows_core::GUID::from_u128(0x2c042b5e_8c07_46d5_aabe_8f75cbad4c31);
pub const D3DAUTHENTICATEDQUERY_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0xa84eb584_c495_48aa_b94d_8bd2d6fbce05);
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESS: windows_core::GUID = windows_core::GUID::from_u128(0x649bbadb_f0f4_4639_a15b_24393fc3abac);
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESSCOUNT: windows_core::GUID = windows_core::GUID::from_u128(0x0db207b3_9450_46a6_82de_1b96d44f9cf2);
pub const D3DAUTHENTICATEDQUERY_UNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT: windows_core::GUID = windows_core::GUID::from_u128(0x012f0bd6_e662_4474_befd_aa53e5143c6d);
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
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DBOX {
    pub Left: super::UINT,
    pub Top: super::UINT,
    pub Right: super::UINT,
    pub Bottom: super::UINT,
    pub Front: super::UINT,
    pub Back: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DBOX {
    pub Left: super::UINT,
    pub Top: super::UINT,
    pub Right: super::UINT,
    pub Bottom: super::UINT,
    pub Front: super::UINT,
    pub Back: super::UINT,
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
pub const D3DCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824;
pub const D3DCAPS2_CANCALIBRATEGAMMA: i32 = 1048576;
pub const D3DCAPS2_CANMANAGERESOURCE: i32 = 268435456;
pub const D3DCAPS2_CANSHARERESOURCE: u32 = 2147483648;
pub const D3DCAPS2_DYNAMICTEXTURES: i32 = 536870912;
pub const D3DCAPS2_FULLSCREENGAMMA: i32 = 131072;
pub const D3DCAPS2_RESERVED: i32 = 33554432;
pub const D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD: i32 = 32;
pub const D3DCAPS3_COPY_TO_SYSTEMMEM: i32 = 512;
pub const D3DCAPS3_COPY_TO_VIDMEM: i32 = 256;
pub const D3DCAPS3_DXVAHD: i32 = 1024;
pub const D3DCAPS3_DXVAHD_LIMITED: i32 = 2048;
pub const D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION: i32 = 128;
pub const D3DCAPS3_RESERVED: u32 = 2147483679;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DCAPS9 {
    pub DeviceType: D3DDEVTYPE,
    pub AdapterOrdinal: super::UINT,
    pub Caps: super::DWORD,
    pub Caps2: super::DWORD,
    pub Caps3: super::DWORD,
    pub PresentationIntervals: super::DWORD,
    pub CursorCaps: super::DWORD,
    pub DevCaps: super::DWORD,
    pub PrimitiveMiscCaps: super::DWORD,
    pub RasterCaps: super::DWORD,
    pub ZCmpCaps: super::DWORD,
    pub SrcBlendCaps: super::DWORD,
    pub DestBlendCaps: super::DWORD,
    pub AlphaCmpCaps: super::DWORD,
    pub ShadeCaps: super::DWORD,
    pub TextureCaps: super::DWORD,
    pub TextureFilterCaps: super::DWORD,
    pub CubeTextureFilterCaps: super::DWORD,
    pub VolumeTextureFilterCaps: super::DWORD,
    pub TextureAddressCaps: super::DWORD,
    pub VolumeTextureAddressCaps: super::DWORD,
    pub LineCaps: super::DWORD,
    pub MaxTextureWidth: super::DWORD,
    pub MaxTextureHeight: super::DWORD,
    pub MaxVolumeExtent: super::DWORD,
    pub MaxTextureRepeat: super::DWORD,
    pub MaxTextureAspectRatio: super::DWORD,
    pub MaxAnisotropy: super::DWORD,
    pub MaxVertexW: f32,
    pub GuardBandLeft: f32,
    pub GuardBandTop: f32,
    pub GuardBandRight: f32,
    pub GuardBandBottom: f32,
    pub ExtentsAdjust: f32,
    pub StencilCaps: super::DWORD,
    pub FVFCaps: super::DWORD,
    pub TextureOpCaps: super::DWORD,
    pub MaxTextureBlendStages: super::DWORD,
    pub MaxSimultaneousTextures: super::DWORD,
    pub VertexProcessingCaps: super::DWORD,
    pub MaxActiveLights: super::DWORD,
    pub MaxUserClipPlanes: super::DWORD,
    pub MaxVertexBlendMatrices: super::DWORD,
    pub MaxVertexBlendMatrixIndex: super::DWORD,
    pub MaxPointSize: f32,
    pub MaxPrimitiveCount: super::DWORD,
    pub MaxVertexIndex: super::DWORD,
    pub MaxStreams: super::DWORD,
    pub MaxStreamStride: super::DWORD,
    pub VertexShaderVersion: super::DWORD,
    pub MaxVertexShaderConst: super::DWORD,
    pub PixelShaderVersion: super::DWORD,
    pub PixelShader1xMaxValue: f32,
    pub DevCaps2: super::DWORD,
    pub MaxNpatchTessellationLevel: f32,
    pub Reserved5: super::DWORD,
    pub MasterAdapterOrdinal: super::UINT,
    pub AdapterOrdinalInGroup: super::UINT,
    pub NumberOfAdaptersInGroup: super::UINT,
    pub DeclTypes: super::DWORD,
    pub NumSimultaneousRTs: super::DWORD,
    pub StretchRectFilterCaps: super::DWORD,
    pub VS20Caps: D3DVSHADERCAPS2_0,
    pub PS20Caps: D3DPSHADERCAPS2_0,
    pub VertexTextureFilterCaps: super::DWORD,
    pub MaxVShaderInstructionsExecuted: super::DWORD,
    pub MaxPShaderInstructionsExecuted: super::DWORD,
    pub MaxVertexShader30InstructionSlots: super::DWORD,
    pub MaxPixelShader30InstructionSlots: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DCAPS9 {
    pub DeviceType: D3DDEVTYPE,
    pub AdapterOrdinal: super::UINT,
    pub Caps: super::DWORD,
    pub Caps2: super::DWORD,
    pub Caps3: super::DWORD,
    pub PresentationIntervals: super::DWORD,
    pub CursorCaps: super::DWORD,
    pub DevCaps: super::DWORD,
    pub PrimitiveMiscCaps: super::DWORD,
    pub RasterCaps: super::DWORD,
    pub ZCmpCaps: super::DWORD,
    pub SrcBlendCaps: super::DWORD,
    pub DestBlendCaps: super::DWORD,
    pub AlphaCmpCaps: super::DWORD,
    pub ShadeCaps: super::DWORD,
    pub TextureCaps: super::DWORD,
    pub TextureFilterCaps: super::DWORD,
    pub CubeTextureFilterCaps: super::DWORD,
    pub VolumeTextureFilterCaps: super::DWORD,
    pub TextureAddressCaps: super::DWORD,
    pub VolumeTextureAddressCaps: super::DWORD,
    pub LineCaps: super::DWORD,
    pub MaxTextureWidth: super::DWORD,
    pub MaxTextureHeight: super::DWORD,
    pub MaxVolumeExtent: super::DWORD,
    pub MaxTextureRepeat: super::DWORD,
    pub MaxTextureAspectRatio: super::DWORD,
    pub MaxAnisotropy: super::DWORD,
    pub MaxVertexW: f32,
    pub GuardBandLeft: f32,
    pub GuardBandTop: f32,
    pub GuardBandRight: f32,
    pub GuardBandBottom: f32,
    pub ExtentsAdjust: f32,
    pub StencilCaps: super::DWORD,
    pub FVFCaps: super::DWORD,
    pub TextureOpCaps: super::DWORD,
    pub MaxTextureBlendStages: super::DWORD,
    pub MaxSimultaneousTextures: super::DWORD,
    pub VertexProcessingCaps: super::DWORD,
    pub MaxActiveLights: super::DWORD,
    pub MaxUserClipPlanes: super::DWORD,
    pub MaxVertexBlendMatrices: super::DWORD,
    pub MaxVertexBlendMatrixIndex: super::DWORD,
    pub MaxPointSize: f32,
    pub MaxPrimitiveCount: super::DWORD,
    pub MaxVertexIndex: super::DWORD,
    pub MaxStreams: super::DWORD,
    pub MaxStreamStride: super::DWORD,
    pub VertexShaderVersion: super::DWORD,
    pub MaxVertexShaderConst: super::DWORD,
    pub PixelShaderVersion: super::DWORD,
    pub PixelShader1xMaxValue: f32,
    pub DevCaps2: super::DWORD,
    pub MaxNpatchTessellationLevel: f32,
    pub Reserved5: super::DWORD,
    pub MasterAdapterOrdinal: super::UINT,
    pub AdapterOrdinalInGroup: super::UINT,
    pub NumberOfAdaptersInGroup: super::UINT,
    pub DeclTypes: super::DWORD,
    pub NumSimultaneousRTs: super::DWORD,
    pub StretchRectFilterCaps: super::DWORD,
    pub VS20Caps: D3DVSHADERCAPS2_0,
    pub PS20Caps: D3DPSHADERCAPS2_0,
    pub VertexTextureFilterCaps: super::DWORD,
    pub MaxVShaderInstructionsExecuted: super::DWORD,
    pub MaxPShaderInstructionsExecuted: super::DWORD,
    pub MaxVertexShader30InstructionSlots: super::DWORD,
    pub MaxPixelShader30InstructionSlots: super::DWORD,
}
pub const D3DCAPS_OVERLAY: i32 = 2048;
pub const D3DCAPS_READ_SCANLINE: i32 = 131072;
pub const D3DCLEAR_STENCIL: i32 = 4;
pub const D3DCLEAR_TARGET: i32 = 1;
pub const D3DCLEAR_ZBUFFER: i32 = 2;
pub const D3DCLIPPLANE0: i32 = 1;
pub const D3DCLIPPLANE1: i32 = 2;
pub const D3DCLIPPLANE2: i32 = 4;
pub const D3DCLIPPLANE3: i32 = 8;
pub const D3DCLIPPLANE4: i32 = 16;
pub const D3DCLIPPLANE5: i32 = 32;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DCLIPSTATUS9 {
    pub ClipUnion: super::DWORD,
    pub ClipIntersection: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DCLIPSTATUS9 {
    pub ClipUnion: super::DWORD,
    pub ClipIntersection: super::DWORD,
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
pub const D3DCOLORWRITEENABLE_ALPHA: i32 = 8;
pub const D3DCOLORWRITEENABLE_BLUE: i32 = 4;
pub const D3DCOLORWRITEENABLE_GREEN: i32 = 2;
pub const D3DCOLORWRITEENABLE_RED: i32 = 1;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DCOMPOSERECTDESC {
    pub X: super::USHORT,
    pub Y: super::USHORT,
    pub Width: super::USHORT,
    pub Height: super::USHORT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DCOMPOSERECTDESC {
    pub X: super::USHORT,
    pub Y: super::USHORT,
    pub Width: super::USHORT,
    pub Height: super::USHORT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DCOMPOSERECTDESTINATION {
    pub SrcRectIndex: super::USHORT,
    pub Reserved: super::USHORT,
    pub X: super::SHORT,
    pub Y: super::SHORT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DCOMPOSERECTDESTINATION {
    pub SrcRectIndex: super::USHORT,
    pub Reserved: super::USHORT,
    pub X: super::SHORT,
    pub Y: super::SHORT,
}
pub type D3DCOMPOSERECTSOP = i32;
pub const D3DCOMPOSERECTS_AND: D3DCOMPOSERECTSOP = 3;
pub const D3DCOMPOSERECTS_COPY: D3DCOMPOSERECTSOP = 1;
pub const D3DCOMPOSERECTS_FORCE_DWORD: D3DCOMPOSERECTSOP = 2147483647;
pub const D3DCOMPOSERECTS_MAXNUMRECTS: i32 = 65535;
pub const D3DCOMPOSERECTS_NEG: D3DCOMPOSERECTSOP = 4;
pub const D3DCOMPOSERECTS_OR: D3DCOMPOSERECTSOP = 2;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DCONTENTPROTECTIONCAPS {
    pub Caps: super::DWORD,
    pub KeyExchangeType: windows_core::GUID,
    pub BufferAlignmentStart: super::UINT,
    pub BlockAlignmentSize: super::UINT,
    pub ProtectedMemorySize: super::ULONGLONG,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DCONTENTPROTECTIONCAPS {
    pub Caps: super::DWORD,
    pub KeyExchangeType: windows_core::GUID,
    pub BufferAlignmentStart: super::UINT,
    pub BlockAlignmentSize: super::UINT,
    pub ProtectedMemorySize: super::ULONGLONG,
}
pub const D3DCONVOLUTIONMONO_MAXHEIGHT: i32 = 7;
pub const D3DCONVOLUTIONMONO_MAXWIDTH: i32 = 7;
pub const D3DCPCAPS_CONTENTKEY: i32 = 16;
pub const D3DCPCAPS_ENCRYPTEDREADBACK: i32 = 64;
pub const D3DCPCAPS_ENCRYPTEDREADBACKKEY: i32 = 128;
pub const D3DCPCAPS_ENCRYPTSLICEDATAONLY: i32 = 512;
pub const D3DCPCAPS_FRESHENSESSIONKEY: i32 = 32;
pub const D3DCPCAPS_HARDWARE: i32 = 2;
pub const D3DCPCAPS_PARTIALDECRYPTION: i32 = 8;
pub const D3DCPCAPS_PROTECTIONALWAYSON: i32 = 4;
pub const D3DCPCAPS_SEQUENTIAL_CTR_IV: i32 = 256;
pub const D3DCPCAPS_SOFTWARE: i32 = 1;
pub const D3DCREATE_ADAPTERGROUP_DEVICE: i32 = 512;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: i32 = 256;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: i32 = 1024;
pub const D3DCREATE_DISABLE_PRINTSCREEN: i32 = 32768;
pub const D3DCREATE_DISABLE_PSGP_THREADING: i32 = 8192;
pub const D3DCREATE_ENABLE_PRESENTSTATS: i32 = 16384;
pub const D3DCREATE_FPU_PRESERVE: i32 = 2;
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: i32 = 64;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: i32 = 128;
pub const D3DCREATE_MULTITHREADED: i32 = 4;
pub const D3DCREATE_NOWINDOWCHANGES: i32 = 2048;
pub const D3DCREATE_PUREDEVICE: i32 = 16;
pub const D3DCREATE_SCREENSAVER: i32 = 268435456;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: i32 = 32;
pub const D3DCRYPTOTYPE_AES128_CTR: windows_core::GUID = windows_core::GUID::from_u128(0x9b6bd711_4f74_41c9_9e7b_0be2d7d93b4f);
pub const D3DCRYPTOTYPE_PROPRIETARY: windows_core::GUID = windows_core::GUID::from_u128(0xab4e9afd_1d1c_46e6_a72f_0869917b0de8);
pub const D3DCS_ALL: i32 = 4095;
pub const D3DCS_BACK: i32 = 32;
pub const D3DCS_BOTTOM: i32 = 8;
pub const D3DCS_FRONT: i32 = 16;
pub const D3DCS_LEFT: i32 = 1;
pub const D3DCS_PLANE0: i32 = 64;
pub const D3DCS_PLANE1: i32 = 128;
pub const D3DCS_PLANE2: i32 = 256;
pub const D3DCS_PLANE3: i32 = 512;
pub const D3DCS_PLANE4: i32 = 1024;
pub const D3DCS_PLANE5: i32 = 2048;
pub const D3DCS_RIGHT: i32 = 2;
pub const D3DCS_TOP: i32 = 4;
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
pub const D3DCURSORCAPS_COLOR: i32 = 1;
pub const D3DCURSORCAPS_LOWRES: i32 = 2;
pub const D3DCURSOR_IMMEDIATE_UPDATE: i32 = 1;
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
pub const D3DDEVCAPS2_ADAPTIVETESSNPATCH: i32 = 8;
pub const D3DDEVCAPS2_ADAPTIVETESSRTPATCH: i32 = 4;
pub const D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES: i32 = 16;
pub const D3DDEVCAPS2_DMAPNPATCH: i32 = 2;
pub const D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH: i32 = 32;
pub const D3DDEVCAPS2_STREAMOFFSET: i32 = 1;
pub const D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET: i32 = 64;
pub const D3DDEVCAPS_CANBLTSYSTONONLOCAL: i32 = 131072;
pub const D3DDEVCAPS_CANRENDERAFTERFLIP: i32 = 2048;
pub const D3DDEVCAPS_DRAWPRIMITIVES2: i32 = 8192;
pub const D3DDEVCAPS_DRAWPRIMITIVES2EX: i32 = 32768;
pub const D3DDEVCAPS_DRAWPRIMTLVERTEX: i32 = 1024;
pub const D3DDEVCAPS_EXECUTESYSTEMMEMORY: i32 = 16;
pub const D3DDEVCAPS_EXECUTEVIDEOMEMORY: i32 = 32;
pub const D3DDEVCAPS_HWRASTERIZATION: i32 = 524288;
pub const D3DDEVCAPS_HWTRANSFORMANDLIGHT: i32 = 65536;
pub const D3DDEVCAPS_NPATCHES: i32 = 16777216;
pub const D3DDEVCAPS_PUREDEVICE: i32 = 1048576;
pub const D3DDEVCAPS_QUINTICRTPATCHES: i32 = 2097152;
pub const D3DDEVCAPS_RTPATCHES: i32 = 4194304;
pub const D3DDEVCAPS_RTPATCHHANDLEZERO: i32 = 8388608;
pub const D3DDEVCAPS_SEPARATETEXTUREMEMORIES: i32 = 16384;
pub const D3DDEVCAPS_TEXTURENONLOCALVIDMEM: i32 = 4096;
pub const D3DDEVCAPS_TEXTURESYSTEMMEMORY: i32 = 256;
pub const D3DDEVCAPS_TEXTUREVIDEOMEMORY: i32 = 512;
pub const D3DDEVCAPS_TLVERTEXSYSTEMMEMORY: i32 = 64;
pub const D3DDEVCAPS_TLVERTEXVIDEOMEMORY: i32 = 128;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVICE_CREATION_PARAMETERS {
    pub AdapterOrdinal: super::UINT,
    pub DeviceType: D3DDEVTYPE,
    pub hFocusWindow: super::HWND,
    pub BehaviorFlags: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDEVICE_CREATION_PARAMETERS {
    pub AdapterOrdinal: super::UINT,
    pub DeviceType: D3DDEVTYPE,
    pub hFocusWindow: super::HWND,
    pub BehaviorFlags: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    pub MaxBandwidthUtilized: super::FLOAT,
    pub FrontEndUploadMemoryUtilizedPercent: super::FLOAT,
    pub VertexRateUtilizedPercent: super::FLOAT,
    pub TriangleSetupRateUtilizedPercent: super::FLOAT,
    pub FillRateUtilizedPercent: super::FLOAT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    pub MaxBandwidthUtilized: super::FLOAT,
    pub FrontEndUploadMemoryUtilizedPercent: super::FLOAT,
    pub VertexRateUtilizedPercent: super::FLOAT,
    pub TriangleSetupRateUtilizedPercent: super::FLOAT,
    pub FillRateUtilizedPercent: super::FLOAT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9CACHEUTILIZATION {
    pub TextureCacheHitRate: super::FLOAT,
    pub PostTransformVertexCacheHitRate: super::FLOAT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9CACHEUTILIZATION {
    pub TextureCacheHitRate: super::FLOAT,
    pub PostTransformVertexCacheHitRate: super::FLOAT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9INTERFACETIMINGS {
    pub WaitingForGPUToUseApplicationResourceTimePercent: super::FLOAT,
    pub WaitingForGPUToAcceptMoreCommandsTimePercent: super::FLOAT,
    pub WaitingForGPUToStayWithinLatencyTimePercent: super::FLOAT,
    pub WaitingForGPUExclusiveResourceTimePercent: super::FLOAT,
    pub WaitingForGPUOtherTimePercent: super::FLOAT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9INTERFACETIMINGS {
    pub WaitingForGPUToUseApplicationResourceTimePercent: super::FLOAT,
    pub WaitingForGPUToAcceptMoreCommandsTimePercent: super::FLOAT,
    pub WaitingForGPUToStayWithinLatencyTimePercent: super::FLOAT,
    pub WaitingForGPUExclusiveResourceTimePercent: super::FLOAT,
    pub WaitingForGPUOtherTimePercent: super::FLOAT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9PIPELINETIMINGS {
    pub VertexProcessingTimePercent: super::FLOAT,
    pub PixelProcessingTimePercent: super::FLOAT,
    pub OtherGPUProcessingTimePercent: super::FLOAT,
    pub GPUIdleTimePercent: super::FLOAT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9PIPELINETIMINGS {
    pub VertexProcessingTimePercent: super::FLOAT,
    pub PixelProcessingTimePercent: super::FLOAT,
    pub OtherGPUProcessingTimePercent: super::FLOAT,
    pub GPUIdleTimePercent: super::FLOAT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3D9STAGETIMINGS {
    pub MemoryProcessingPercent: super::FLOAT,
    pub ComputationProcessingPercent: super::FLOAT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9STAGETIMINGS {
    pub MemoryProcessingPercent: super::FLOAT,
    pub ComputationProcessingPercent: super::FLOAT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_D3DVERTEXSTATS {
    pub NumRenderedTriangles: super::DWORD,
    pub NumExtraClippingTriangles: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDEVINFO_D3DVERTEXSTATS {
    pub NumRenderedTriangles: super::DWORD,
    pub NumExtraClippingTriangles: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct D3DDEVINFO_RESOURCEMANAGER {
    pub stats: [D3DRESOURCESTATS; 8],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for D3DDEVINFO_RESOURCEMANAGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct D3DDEVINFO_RESOURCEMANAGER {
    pub stats: [D3DRESOURCESTATS; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for D3DDEVINFO_RESOURCEMANAGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DDEVINFO_VCACHE {
    pub Pattern: super::DWORD,
    pub OptMethod: super::DWORD,
    pub CacheSize: super::DWORD,
    pub MagicNumber: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDEVINFO_VCACHE {
    pub Pattern: super::DWORD,
    pub OptMethod: super::DWORD,
    pub CacheSize: super::DWORD,
    pub MagicNumber: super::DWORD,
}
pub type D3DDEVTYPE = i32;
pub const D3DDEVTYPE_FORCE_DWORD: D3DDEVTYPE = 2147483647;
pub const D3DDEVTYPE_HAL: D3DDEVTYPE = 1;
pub const D3DDEVTYPE_NULLREF: D3DDEVTYPE = 4;
pub const D3DDEVTYPE_REF: D3DDEVTYPE = 2;
pub const D3DDEVTYPE_SW: D3DDEVTYPE = 3;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DDISPLAYMODE {
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub RefreshRate: super::UINT,
    pub Format: D3DFORMAT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDISPLAYMODE {
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub RefreshRate: super::UINT,
    pub Format: D3DFORMAT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DDISPLAYMODEEX {
    pub Size: super::UINT,
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub RefreshRate: super::UINT,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDISPLAYMODEEX {
    pub Size: super::UINT,
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub RefreshRate: super::UINT,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DDISPLAYMODEFILTER {
    pub Size: super::UINT,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDISPLAYMODEFILTER {
    pub Size: super::UINT,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
pub type D3DDISPLAYROTATION = i32;
pub const D3DDISPLAYROTATION_180: D3DDISPLAYROTATION = 3;
pub const D3DDISPLAYROTATION_270: D3DDISPLAYROTATION = 4;
pub const D3DDISPLAYROTATION_90: D3DDISPLAYROTATION = 2;
pub const D3DDISPLAYROTATION_IDENTITY: D3DDISPLAYROTATION = 1;
pub const D3DDMAPSAMPLER: i32 = 256;
pub const D3DDMT_DISABLE: D3DDEBUGMONITORTOKENS = 1;
pub const D3DDMT_ENABLE: D3DDEBUGMONITORTOKENS = 0;
pub const D3DDMT_FORCE_DWORD: D3DDEBUGMONITORTOKENS = 2147483647;
pub const D3DDP_MAXTEXCOORD: i32 = 8;
pub const D3DDTCAPS_DEC3N: i32 = 128;
pub const D3DDTCAPS_FLOAT16_2: i32 = 256;
pub const D3DDTCAPS_FLOAT16_4: i32 = 512;
pub const D3DDTCAPS_SHORT2N: i32 = 4;
pub const D3DDTCAPS_SHORT4N: i32 = 8;
pub const D3DDTCAPS_UBYTE4: i32 = 1;
pub const D3DDTCAPS_UBYTE4N: i32 = 2;
pub const D3DDTCAPS_UDEC3: i32 = 64;
pub const D3DDTCAPS_USHORT2N: i32 = 16;
pub const D3DDTCAPS_USHORT4N: i32 = 32;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: super::UINT,
    pub NumBytesInSkipPattern: super::UINT,
    pub NumBytesInEncryptPattern: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: super::UINT,
    pub NumBytesInSkipPattern: super::UINT,
    pub NumBytesInEncryptPattern: super::UINT,
}
pub const D3DENUM_NO_DRIVERVERSION: i32 = 4;
pub const D3DENUM_WHQL_LEVEL: i32 = 2;
pub const D3DERR_CANNOTPROTECTCONTENT: windows_core::HRESULT = windows_core::HRESULT(0x8876087D_u32 as _);
pub const D3DERR_CONFLICTINGRENDERSTATE: windows_core::HRESULT = windows_core::HRESULT(0x88760821_u32 as _);
pub const D3DERR_CONFLICTINGTEXTUREFILTER: windows_core::HRESULT = windows_core::HRESULT(0x8876081E_u32 as _);
pub const D3DERR_CONFLICTINGTEXTUREPALETTE: windows_core::HRESULT = windows_core::HRESULT(0x88760826_u32 as _);
pub const D3DERR_DEVICEHUNG: windows_core::HRESULT = windows_core::HRESULT(0x88760874_u32 as _);
pub const D3DERR_DEVICELOST: windows_core::HRESULT = windows_core::HRESULT(0x88760868_u32 as _);
pub const D3DERR_DEVICENOTRESET: windows_core::HRESULT = windows_core::HRESULT(0x88760869_u32 as _);
pub const D3DERR_DEVICEREMOVED: windows_core::HRESULT = windows_core::HRESULT(0x88760870_u32 as _);
pub const D3DERR_DRIVERINTERNALERROR: windows_core::HRESULT = windows_core::HRESULT(0x88760827_u32 as _);
pub const D3DERR_DRIVERINVALIDCALL: windows_core::HRESULT = windows_core::HRESULT(0x8876086D_u32 as _);
pub const D3DERR_INVALIDCALL: windows_core::HRESULT = windows_core::HRESULT(0x8876086C_u32 as _);
pub const D3DERR_INVALIDDEVICE: windows_core::HRESULT = windows_core::HRESULT(0x8876086B_u32 as _);
pub const D3DERR_MOREDATA: windows_core::HRESULT = windows_core::HRESULT(0x88760867_u32 as _);
pub const D3DERR_NOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8876086A_u32 as _);
pub const D3DERR_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x88760866_u32 as _);
pub const D3DERR_OUTOFVIDEOMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8876017C_u32 as _);
pub const D3DERR_PRESENT_STATISTICS_DISJOINT: windows_core::HRESULT = windows_core::HRESULT(0x88760884_u32 as _);
pub const D3DERR_TOOMANYOPERATIONS: windows_core::HRESULT = windows_core::HRESULT(0x8876081D_u32 as _);
pub const D3DERR_UNSUPPORTEDALPHAARG: windows_core::HRESULT = windows_core::HRESULT(0x8876081C_u32 as _);
pub const D3DERR_UNSUPPORTEDALPHAOPERATION: windows_core::HRESULT = windows_core::HRESULT(0x8876081B_u32 as _);
pub const D3DERR_UNSUPPORTEDCOLORARG: windows_core::HRESULT = windows_core::HRESULT(0x8876081A_u32 as _);
pub const D3DERR_UNSUPPORTEDCOLOROPERATION: windows_core::HRESULT = windows_core::HRESULT(0x88760819_u32 as _);
pub const D3DERR_UNSUPPORTEDCRYPTO: windows_core::HRESULT = windows_core::HRESULT(0x8876087E_u32 as _);
pub const D3DERR_UNSUPPORTEDFACTORVALUE: windows_core::HRESULT = windows_core::HRESULT(0x8876081F_u32 as _);
pub const D3DERR_UNSUPPORTEDOVERLAY: windows_core::HRESULT = windows_core::HRESULT(0x8876087B_u32 as _);
pub const D3DERR_UNSUPPORTEDOVERLAYFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8876087C_u32 as _);
pub const D3DERR_UNSUPPORTEDTEXTUREFILTER: windows_core::HRESULT = windows_core::HRESULT(0x88760822_u32 as _);
pub const D3DERR_WASSTILLDRAWING: windows_core::HRESULT = windows_core::HRESULT(0x8876021C_u32 as _);
pub const D3DERR_WRONGTEXTUREFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x88760818_u32 as _);
pub type D3DFILLMODE = i32;
pub const D3DFILL_FORCE_DWORD: D3DFILLMODE = 2147483647;
pub const D3DFILL_POINT: D3DFILLMODE = 1;
pub const D3DFILL_SOLID: D3DFILLMODE = 3;
pub const D3DFILL_WIREFRAME: D3DFILLMODE = 2;
pub const D3DFMT_A1: D3DFORMAT = 118;
pub const D3DFMT_A16B16G16R16: D3DFORMAT = 36;
pub const D3DFMT_A16B16G16R16F: D3DFORMAT = 113;
pub const D3DFMT_A1R5G5B5: D3DFORMAT = 25;
pub const D3DFMT_A1_SURFACE_MAXHEIGHT: i32 = 2048;
pub const D3DFMT_A1_SURFACE_MAXWIDTH: i32 = 8192;
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
pub const D3DFVFCAPS_DONOTSTRIPELEMENTS: i32 = 524288;
pub const D3DFVFCAPS_PSIZE: i32 = 1048576;
pub const D3DFVFCAPS_TEXCOORDCOUNTMASK: i32 = 65535;
pub const D3DFVF_DIFFUSE: i32 = 64;
pub const D3DFVF_LASTBETA_D3DCOLOR: i32 = 32768;
pub const D3DFVF_LASTBETA_UBYTE4: i32 = 4096;
pub const D3DFVF_NORMAL: i32 = 16;
pub const D3DFVF_POSITION_MASK: i32 = 16398;
pub const D3DFVF_PSIZE: i32 = 32;
pub const D3DFVF_RESERVED0: i32 = 1;
pub const D3DFVF_RESERVED2: i32 = 24576;
pub const D3DFVF_SPECULAR: i32 = 128;
pub const D3DFVF_TEX0: i32 = 0;
pub const D3DFVF_TEX1: i32 = 256;
pub const D3DFVF_TEX2: i32 = 512;
pub const D3DFVF_TEX3: i32 = 768;
pub const D3DFVF_TEX4: i32 = 1024;
pub const D3DFVF_TEX5: i32 = 1280;
pub const D3DFVF_TEX6: i32 = 1536;
pub const D3DFVF_TEX7: i32 = 1792;
pub const D3DFVF_TEX8: i32 = 2048;
pub const D3DFVF_TEXCOUNT_MASK: i32 = 3840;
pub const D3DFVF_TEXCOUNT_SHIFT: i32 = 8;
pub const D3DFVF_TEXTUREFORMAT1: i32 = 3;
pub const D3DFVF_TEXTUREFORMAT2: i32 = 0;
pub const D3DFVF_TEXTUREFORMAT3: i32 = 1;
pub const D3DFVF_TEXTUREFORMAT4: i32 = 2;
pub const D3DFVF_XYZ: i32 = 2;
pub const D3DFVF_XYZB1: i32 = 6;
pub const D3DFVF_XYZB2: i32 = 8;
pub const D3DFVF_XYZB3: i32 = 10;
pub const D3DFVF_XYZB4: i32 = 12;
pub const D3DFVF_XYZB5: i32 = 14;
pub const D3DFVF_XYZRHW: i32 = 4;
pub const D3DFVF_XYZW: i32 = 16386;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct D3DGAMMARAMP {
    pub red: [super::WORD; 256],
    pub green: [super::WORD; 256],
    pub blue: [super::WORD; 256],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for D3DGAMMARAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3DGAMMARAMP {
    pub red: [super::WORD; 256],
    pub green: [super::WORD; 256],
    pub blue: [super::WORD; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for D3DGAMMARAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DGETDATA_FLUSH: i32 = 1;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DINDEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: super::DWORD,
    pub Pool: D3DPOOL,
    pub Size: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DINDEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: super::DWORD,
    pub Pool: D3DPOOL,
    pub Size: super::UINT,
}
pub const D3DISSUE_BEGIN: i32 = 2;
pub const D3DISSUE_END: i32 = 1;
pub const D3DKEYEXCHANGE_DXVA: windows_core::GUID = windows_core::GUID::from_u128(0x43d3775c_38e5_4924_8d86_d3fccf153e9b);
pub const D3DKEYEXCHANGE_RSAES_OAEP: windows_core::GUID = windows_core::GUID::from_u128(0xc1949895_d72a_4a1d_8e5d_ed857d171520);
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "dsound", feature = "dxgi"))]
#[derive(Clone, Copy, Default)]
pub struct D3DLIGHT9 {
    pub Type: D3DLIGHTTYPE,
    pub Diffuse: super::D3DCOLORVALUE,
    pub Specular: super::D3DCOLORVALUE,
    pub Ambient: super::D3DCOLORVALUE,
    pub Position: super::D3DVECTOR,
    pub Direction: super::D3DVECTOR,
    pub Range: f32,
    pub Falloff: f32,
    pub Attenuation0: f32,
    pub Attenuation1: f32,
    pub Attenuation2: f32,
    pub Theta: f32,
    pub Phi: f32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "dsound", feature = "dxgi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DLIGHT9 {
    pub Type: D3DLIGHTTYPE,
    pub Diffuse: super::D3DCOLORVALUE,
    pub Specular: super::D3DCOLORVALUE,
    pub Ambient: super::D3DCOLORVALUE,
    pub Position: super::D3DVECTOR,
    pub Direction: super::D3DVECTOR,
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
pub const D3DLINECAPS_ALPHACMP: i32 = 8;
pub const D3DLINECAPS_ANTIALIAS: i32 = 32;
pub const D3DLINECAPS_BLEND: i32 = 4;
pub const D3DLINECAPS_FOG: i32 = 16;
pub const D3DLINECAPS_TEXTURE: i32 = 1;
pub const D3DLINECAPS_ZTEST: i32 = 2;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DLOCKED_BOX {
    pub RowPitch: super::INT,
    pub SlicePitch: super::INT,
    pub pBits: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DLOCKED_BOX {
    pub RowPitch: super::INT,
    pub SlicePitch: super::INT,
    pub pBits: *mut core::ffi::c_void,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DLOCKED_RECT {
    pub Pitch: super::INT,
    pub pBits: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DLOCKED_RECT {
    pub Pitch: super::INT,
    pub pBits: *mut core::ffi::c_void,
}
pub const D3DLOCK_DISCARD: i32 = 8192;
pub const D3DLOCK_DONOTWAIT: i32 = 16384;
pub const D3DLOCK_NOOVERWRITE: i32 = 4096;
pub const D3DLOCK_NOSYSLOCK: i32 = 2048;
pub const D3DLOCK_NO_DIRTY_UPDATE: i32 = 32768;
pub const D3DLOCK_READONLY: i32 = 16;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3DMATERIAL9 {
    pub Diffuse: super::D3DCOLORVALUE,
    pub Ambient: super::D3DCOLORVALUE,
    pub Specular: super::D3DCOLORVALUE,
    pub Emissive: super::D3DCOLORVALUE,
    pub Power: f32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DMATERIAL9 {
    pub Diffuse: super::D3DCOLORVALUE,
    pub Ambient: super::D3DCOLORVALUE,
    pub Specular: super::D3DCOLORVALUE,
    pub Emissive: super::D3DCOLORVALUE,
    pub Power: f32,
}
pub type D3DMATERIALCOLORSOURCE = i32;
pub const D3DMAX30SHADERINSTRUCTIONS: i32 = 32768;
pub const D3DMAXUSERCLIPPLANES: i32 = 32;
pub const D3DMCS_COLOR1: D3DMATERIALCOLORSOURCE = 1;
pub const D3DMCS_COLOR2: D3DMATERIALCOLORSOURCE = 2;
pub const D3DMCS_FORCE_DWORD: D3DMATERIALCOLORSOURCE = 2147483647;
pub const D3DMCS_MATERIAL: D3DMATERIALCOLORSOURCE = 0;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: super::UINT64,
    pub SizeOfInefficientAllocation: super::UINT64,
    pub LevelOfEfficiency: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: super::UINT64,
    pub SizeOfInefficientAllocation: super::UINT64,
    pub LevelOfEfficiency: super::DWORD,
}
pub const D3DMIN30SHADERINSTRUCTIONS: i32 = 512;
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
pub const D3DOK_NOAUTOGEN: windows_core::HRESULT = windows_core::HRESULT(0x876086F_u32 as _);
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DOVERLAYCAPS {
    pub Caps: super::UINT,
    pub MaxOverlayDisplayWidth: super::UINT,
    pub MaxOverlayDisplayHeight: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DOVERLAYCAPS {
    pub Caps: super::UINT,
    pub MaxOverlayDisplayWidth: super::UINT,
    pub MaxOverlayDisplayHeight: super::UINT,
}
pub const D3DOVERLAYCAPS_FULLRANGERGB: i32 = 1;
pub const D3DOVERLAYCAPS_LIMITEDRANGERGB: i32 = 2;
pub const D3DOVERLAYCAPS_STRETCHX: i32 = 64;
pub const D3DOVERLAYCAPS_STRETCHY: i32 = 128;
pub const D3DOVERLAYCAPS_YCbCr_BT601: i32 = 4;
pub const D3DOVERLAYCAPS_YCbCr_BT601_xvYCC: i32 = 16;
pub const D3DOVERLAYCAPS_YCbCr_BT709: i32 = 8;
pub const D3DOVERLAYCAPS_YCbCr_BT709_xvYCC: i32 = 32;
pub type D3DPATCHEDGESTYLE = i32;
pub const D3DPATCHEDGE_CONTINUOUS: D3DPATCHEDGESTYLE = 1;
pub const D3DPATCHEDGE_DISCRETE: D3DPATCHEDGESTYLE = 0;
pub const D3DPATCHEDGE_FORCE_DWORD: D3DPATCHEDGESTYLE = 2147483647;
pub const D3DPBLENDCAPS_BLENDFACTOR: i32 = 8192;
pub const D3DPBLENDCAPS_BOTHINVSRCALPHA: i32 = 4096;
pub const D3DPBLENDCAPS_BOTHSRCALPHA: i32 = 2048;
pub const D3DPBLENDCAPS_DESTALPHA: i32 = 64;
pub const D3DPBLENDCAPS_DESTCOLOR: i32 = 256;
pub const D3DPBLENDCAPS_INVDESTALPHA: i32 = 128;
pub const D3DPBLENDCAPS_INVDESTCOLOR: i32 = 512;
pub const D3DPBLENDCAPS_INVSRCALPHA: i32 = 32;
pub const D3DPBLENDCAPS_INVSRCCOLOR: i32 = 8;
pub const D3DPBLENDCAPS_INVSRCCOLOR2: i32 = 32768;
pub const D3DPBLENDCAPS_ONE: i32 = 2;
pub const D3DPBLENDCAPS_SRCALPHA: i32 = 16;
pub const D3DPBLENDCAPS_SRCALPHASAT: i32 = 1024;
pub const D3DPBLENDCAPS_SRCCOLOR: i32 = 4;
pub const D3DPBLENDCAPS_SRCCOLOR2: i32 = 16384;
pub const D3DPBLENDCAPS_ZERO: i32 = 1;
pub const D3DPCMPCAPS_ALWAYS: i32 = 128;
pub const D3DPCMPCAPS_EQUAL: i32 = 4;
pub const D3DPCMPCAPS_GREATER: i32 = 16;
pub const D3DPCMPCAPS_GREATEREQUAL: i32 = 64;
pub const D3DPCMPCAPS_LESS: i32 = 2;
pub const D3DPCMPCAPS_LESSEQUAL: i32 = 8;
pub const D3DPCMPCAPS_NEVER: i32 = 1;
pub const D3DPCMPCAPS_NOTEQUAL: i32 = 32;
pub const D3DPMISCCAPS_BLENDOP: i32 = 2048;
pub const D3DPMISCCAPS_CLIPPLANESCALEDPOINTS: i32 = 256;
pub const D3DPMISCCAPS_CLIPTLVERTS: i32 = 512;
pub const D3DPMISCCAPS_COLORWRITEENABLE: i32 = 128;
pub const D3DPMISCCAPS_CULLCCW: i32 = 64;
pub const D3DPMISCCAPS_CULLCW: i32 = 32;
pub const D3DPMISCCAPS_CULLNONE: i32 = 16;
pub const D3DPMISCCAPS_FOGANDSPECULARALPHA: i32 = 65536;
pub const D3DPMISCCAPS_FOGVERTEXCLAMPED: i32 = 1048576;
pub const D3DPMISCCAPS_INDEPENDENTWRITEMASKS: i32 = 16384;
pub const D3DPMISCCAPS_MASKZ: i32 = 2;
pub const D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS: i32 = 262144;
pub const D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING: i32 = 524288;
pub const D3DPMISCCAPS_NULLREFERENCE: i32 = 4096;
pub const D3DPMISCCAPS_PERSTAGECONSTANT: i32 = 32768;
pub const D3DPMISCCAPS_POSTBLENDSRGBCONVERT: i32 = 2097152;
pub const D3DPMISCCAPS_SEPARATEALPHABLEND: i32 = 131072;
pub const D3DPMISCCAPS_TSSARGTEMP: i32 = 1024;
pub type D3DPOOL = i32;
pub const D3DPOOL_DEFAULT: D3DPOOL = 0;
pub const D3DPOOL_FORCE_DWORD: D3DPOOL = 2147483647;
pub const D3DPOOL_MANAGED: D3DPOOL = 1;
pub const D3DPOOL_SCRATCH: D3DPOOL = 3;
pub const D3DPOOL_SYSTEMMEM: D3DPOOL = 2;
pub const D3DPRASTERCAPS_ANISOTROPY: i32 = 131072;
pub const D3DPRASTERCAPS_COLORPERSPECTIVE: i32 = 4194304;
pub const D3DPRASTERCAPS_DEPTHBIAS: i32 = 67108864;
pub const D3DPRASTERCAPS_DITHER: i32 = 1;
pub const D3DPRASTERCAPS_FOGRANGE: i32 = 65536;
pub const D3DPRASTERCAPS_FOGTABLE: i32 = 256;
pub const D3DPRASTERCAPS_FOGVERTEX: i32 = 128;
pub const D3DPRASTERCAPS_MIPMAPLODBIAS: i32 = 8192;
pub const D3DPRASTERCAPS_MULTISAMPLE_TOGGLE: i32 = 134217728;
pub const D3DPRASTERCAPS_SCISSORTEST: i32 = 16777216;
pub const D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS: i32 = 33554432;
pub const D3DPRASTERCAPS_WBUFFER: i32 = 262144;
pub const D3DPRASTERCAPS_WFOG: i32 = 1048576;
pub const D3DPRASTERCAPS_ZBUFFERLESSHSR: i32 = 32768;
pub const D3DPRASTERCAPS_ZFOG: i32 = 2097152;
pub const D3DPRASTERCAPS_ZTEST: i32 = 16;
pub const D3DPRESENTFLAG_DEVICECLIP: i32 = 4;
pub const D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL: i32 = 2;
pub const D3DPRESENTFLAG_LOCKABLE_BACKBUFFER: i32 = 1;
pub const D3DPRESENTFLAG_NOAUTOROTATE: i32 = 32;
pub const D3DPRESENTFLAG_OVERLAY_LIMITEDRGB: i32 = 128;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_BT709: i32 = 256;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC: i32 = 512;
pub const D3DPRESENTFLAG_RESTRICTED_CONTENT: i32 = 1024;
pub const D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER: i32 = 2048;
pub const D3DPRESENTFLAG_UNPRUNEDMODE: i32 = 64;
pub const D3DPRESENTFLAG_VIDEO: i32 = 16;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DPRESENTSTATS {
    pub PresentCount: super::UINT,
    pub PresentRefreshCount: super::UINT,
    pub SyncRefreshCount: super::UINT,
    pub SyncQPCTime: super::LARGE_INTEGER,
    pub SyncGPUTime: super::LARGE_INTEGER,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for D3DPRESENTSTATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DPRESENTSTATS {
    pub PresentCount: super::UINT,
    pub PresentRefreshCount: super::UINT,
    pub SyncRefreshCount: super::UINT,
    pub SyncQPCTime: super::LARGE_INTEGER,
    pub SyncGPUTime: super::LARGE_INTEGER,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for D3DPRESENTSTATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DPRESENT_BACK_BUFFERS_MAX: i32 = 3;
pub const D3DPRESENT_BACK_BUFFERS_MAX_EX: i32 = 30;
pub const D3DPRESENT_DONOTFLIP: i32 = 4;
pub const D3DPRESENT_DONOTWAIT: i32 = 1;
pub const D3DPRESENT_FLIPRESTART: i32 = 8;
pub const D3DPRESENT_FORCEIMMEDIATE: i32 = 256;
pub const D3DPRESENT_HIDEOVERLAY: i32 = 64;
pub const D3DPRESENT_INTERVAL_DEFAULT: i32 = 0;
pub const D3DPRESENT_INTERVAL_FOUR: i32 = 8;
pub const D3DPRESENT_INTERVAL_IMMEDIATE: u32 = 2147483648;
pub const D3DPRESENT_INTERVAL_ONE: i32 = 1;
pub const D3DPRESENT_INTERVAL_THREE: i32 = 4;
pub const D3DPRESENT_INTERVAL_TWO: i32 = 2;
pub const D3DPRESENT_LINEAR_CONTENT: i32 = 2;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct D3DPRESENT_PARAMETERS {
    pub BackBufferWidth: super::UINT,
    pub BackBufferHeight: super::UINT,
    pub BackBufferFormat: D3DFORMAT,
    pub BackBufferCount: super::UINT,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: super::DWORD,
    pub SwapEffect: D3DSWAPEFFECT,
    pub hDeviceWindow: super::HWND,
    pub Windowed: windows_core::BOOL,
    pub EnableAutoDepthStencil: windows_core::BOOL,
    pub AutoDepthStencilFormat: D3DFORMAT,
    pub Flags: super::DWORD,
    pub FullScreen_RefreshRateInHz: super::UINT,
    pub PresentationInterval: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DPRESENT_PARAMETERS {
    pub BackBufferWidth: super::UINT,
    pub BackBufferHeight: super::UINT,
    pub BackBufferFormat: D3DFORMAT,
    pub BackBufferCount: super::UINT,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: super::DWORD,
    pub SwapEffect: D3DSWAPEFFECT,
    pub hDeviceWindow: super::HWND,
    pub Windowed: windows_core::BOOL,
    pub EnableAutoDepthStencil: windows_core::BOOL,
    pub AutoDepthStencilFormat: D3DFORMAT,
    pub Flags: super::DWORD,
    pub FullScreen_RefreshRateInHz: super::UINT,
    pub PresentationInterval: super::UINT,
}
pub const D3DPRESENT_RATE_DEFAULT: i32 = 0;
pub const D3DPRESENT_UPDATECOLORKEY: i32 = 128;
pub const D3DPRESENT_UPDATEOVERLAYONLY: i32 = 32;
pub const D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR: i32 = 16;
pub type D3DPRIMITIVETYPE = i32;
pub const D3DPS20CAPS_ARBITRARYSWIZZLE: i32 = 1;
pub const D3DPS20CAPS_GRADIENTINSTRUCTIONS: i32 = 2;
pub const D3DPS20CAPS_NODEPENDENTREADLIMIT: i32 = 8;
pub const D3DPS20CAPS_NOTEXINSTRUCTIONLIMIT: i32 = 16;
pub const D3DPS20CAPS_PREDICATION: i32 = 4;
pub const D3DPS20_MAX_DYNAMICFLOWCONTROLDEPTH: i32 = 24;
pub const D3DPS20_MAX_NUMINSTRUCTIONSLOTS: i32 = 512;
pub const D3DPS20_MAX_NUMTEMPS: i32 = 32;
pub const D3DPS20_MAX_STATICFLOWCONTROLDEPTH: i32 = 4;
pub const D3DPS20_MIN_DYNAMICFLOWCONTROLDEPTH: i32 = 0;
pub const D3DPS20_MIN_NUMINSTRUCTIONSLOTS: i32 = 96;
pub const D3DPS20_MIN_NUMTEMPS: i32 = 12;
pub const D3DPS20_MIN_STATICFLOWCONTROLDEPTH: i32 = 0;
pub const D3DPSHADECAPS_ALPHAGOURAUDBLEND: i32 = 16384;
pub const D3DPSHADECAPS_COLORGOURAUDRGB: i32 = 8;
pub const D3DPSHADECAPS_FOGGOURAUD: i32 = 524288;
pub const D3DPSHADECAPS_SPECULARGOURAUDRGB: i32 = 512;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DPSHADERCAPS2_0 {
    pub Caps: super::DWORD,
    pub DynamicFlowControlDepth: super::INT,
    pub NumTemps: super::INT,
    pub StaticFlowControlDepth: super::INT,
    pub NumInstructionSlots: super::INT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DPSHADERCAPS2_0 {
    pub Caps: super::DWORD,
    pub DynamicFlowControlDepth: super::INT,
    pub NumTemps: super::INT,
    pub StaticFlowControlDepth: super::INT,
    pub NumInstructionSlots: super::INT,
}
pub const D3DPTADDRESSCAPS_BORDER: i32 = 8;
pub const D3DPTADDRESSCAPS_CLAMP: i32 = 4;
pub const D3DPTADDRESSCAPS_INDEPENDENTUV: i32 = 16;
pub const D3DPTADDRESSCAPS_MIRROR: i32 = 2;
pub const D3DPTADDRESSCAPS_MIRRORONCE: i32 = 32;
pub const D3DPTADDRESSCAPS_WRAP: i32 = 1;
pub const D3DPTEXTURECAPS_ALPHA: i32 = 4;
pub const D3DPTEXTURECAPS_ALPHAPALETTE: i32 = 128;
pub const D3DPTEXTURECAPS_CUBEMAP: i32 = 2048;
pub const D3DPTEXTURECAPS_CUBEMAP_POW2: i32 = 131072;
pub const D3DPTEXTURECAPS_MIPCUBEMAP: i32 = 65536;
pub const D3DPTEXTURECAPS_MIPMAP: i32 = 16384;
pub const D3DPTEXTURECAPS_MIPVOLUMEMAP: i32 = 32768;
pub const D3DPTEXTURECAPS_NONPOW2CONDITIONAL: i32 = 256;
pub const D3DPTEXTURECAPS_NOPROJECTEDBUMPENV: i32 = 2097152;
pub const D3DPTEXTURECAPS_PERSPECTIVE: i32 = 1;
pub const D3DPTEXTURECAPS_POW2: i32 = 2;
pub const D3DPTEXTURECAPS_PROJECTED: i32 = 1024;
pub const D3DPTEXTURECAPS_SQUAREONLY: i32 = 32;
pub const D3DPTEXTURECAPS_TEXREPEATNOTSCALEDBYSIZE: i32 = 64;
pub const D3DPTEXTURECAPS_VOLUMEMAP: i32 = 8192;
pub const D3DPTEXTURECAPS_VOLUMEMAP_POW2: i32 = 262144;
pub const D3DPTFILTERCAPS_CONVOLUTIONMONO: i32 = 262144;
pub const D3DPTFILTERCAPS_MAGFANISOTROPIC: i32 = 67108864;
pub const D3DPTFILTERCAPS_MAGFGAUSSIANQUAD: i32 = 268435456;
pub const D3DPTFILTERCAPS_MAGFLINEAR: i32 = 33554432;
pub const D3DPTFILTERCAPS_MAGFPOINT: i32 = 16777216;
pub const D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD: i32 = 134217728;
pub const D3DPTFILTERCAPS_MINFANISOTROPIC: i32 = 1024;
pub const D3DPTFILTERCAPS_MINFGAUSSIANQUAD: i32 = 4096;
pub const D3DPTFILTERCAPS_MINFLINEAR: i32 = 512;
pub const D3DPTFILTERCAPS_MINFPOINT: i32 = 256;
pub const D3DPTFILTERCAPS_MINFPYRAMIDALQUAD: i32 = 2048;
pub const D3DPTFILTERCAPS_MIPFLINEAR: i32 = 131072;
pub const D3DPTFILTERCAPS_MIPFPOINT: i32 = 65536;
pub const D3DPT_FORCE_DWORD: D3DPRIMITIVETYPE = 2147483647;
pub const D3DPT_LINELIST: D3DPRIMITIVETYPE = 2;
pub const D3DPT_LINESTRIP: D3DPRIMITIVETYPE = 3;
pub const D3DPT_POINTLIST: D3DPRIMITIVETYPE = 1;
pub const D3DPT_TRIANGLEFAN: D3DPRIMITIVETYPE = 6;
pub const D3DPT_TRIANGLELIST: D3DPRIMITIVETYPE = 4;
pub const D3DPT_TRIANGLESTRIP: D3DPRIMITIVETYPE = 5;
pub const D3DPV_DONOTCOPYDATA: i32 = 1;
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
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DRANGE {
    pub Offset: super::UINT,
    pub Size: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DRANGE {
    pub Offset: super::UINT,
    pub Size: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DRASTER_STATUS {
    pub InVBlank: windows_core::BOOL,
    pub ScanLine: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DRASTER_STATUS {
    pub InVBlank: windows_core::BOOL,
    pub ScanLine: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DRECT {
    pub x1: super::LONG,
    pub y1: super::LONG,
    pub x2: super::LONG,
    pub y2: super::LONG,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DRECT {
    pub x1: super::LONG,
    pub y1: super::LONG,
    pub x2: super::LONG,
    pub y2: super::LONG,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DRECTPATCH_INFO {
    pub StartVertexOffsetWidth: super::UINT,
    pub StartVertexOffsetHeight: super::UINT,
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Stride: super::UINT,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DRECTPATCH_INFO {
    pub StartVertexOffsetWidth: super::UINT,
    pub StartVertexOffsetHeight: super::UINT,
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Stride: super::UINT,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
pub type D3DRENDERSTATETYPE = i32;
pub const D3DRENDERSTATE_WRAPBIAS: u32 = 128;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DRESOURCESTATS {
    pub bThrashing: windows_core::BOOL,
    pub ApproxBytesDownloaded: super::DWORD,
    pub NumEvicts: super::DWORD,
    pub NumVidCreates: super::DWORD,
    pub LastPri: super::DWORD,
    pub NumUsed: super::DWORD,
    pub NumUsedInVidMem: super::DWORD,
    pub WorkingSet: super::DWORD,
    pub WorkingSetBytes: super::DWORD,
    pub TotalManaged: super::DWORD,
    pub TotalBytes: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DRESOURCESTATS {
    pub bThrashing: windows_core::BOOL,
    pub ApproxBytesDownloaded: super::DWORD,
    pub NumEvicts: super::DWORD,
    pub NumVidCreates: super::DWORD,
    pub LastPri: super::DWORD,
    pub NumUsed: super::DWORD,
    pub NumUsedInVidMem: super::DWORD,
    pub WorkingSet: super::DWORD,
    pub WorkingSetBytes: super::DWORD,
    pub TotalManaged: super::DWORD,
    pub TotalBytes: super::DWORD,
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
pub const D3DRTYPECOUNT: i32 = 8;
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
pub const D3DSGR_CALIBRATE: i32 = 1;
pub const D3DSGR_NO_CALIBRATION: i32 = 0;
pub type D3DSHADEMODE = i32;
pub const D3DSHADER_ADDRESSMODE_MASK: i32 = 8192;
pub const D3DSHADER_ADDRESSMODE_SHIFT: i32 = 13;
pub type D3DSHADER_ADDRESSMODE_TYPE = i32;
pub const D3DSHADER_ADDRMODE_ABSOLUTE: D3DSHADER_ADDRESSMODE_TYPE = 0;
pub const D3DSHADER_ADDRMODE_FORCE_DWORD: D3DSHADER_ADDRESSMODE_TYPE = 2147483647;
pub const D3DSHADER_ADDRMODE_RELATIVE: D3DSHADER_ADDRESSMODE_TYPE = 8192;
pub type D3DSHADER_COMPARISON = i32;
pub const D3DSHADER_COMPARISON_MASK: i32 = 458752;
pub const D3DSHADER_COMPARISON_SHIFT: i32 = 16;
pub type D3DSHADER_INSTRUCTION_OPCODE_TYPE = i32;
pub const D3DSHADER_INSTRUCTION_PREDICATED: i32 = 268435456;
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
pub const D3DSI_COISSUE: i32 = 1073741824;
pub const D3DSI_COMMENTSIZE_MASK: i32 = 2147418112;
pub const D3DSI_COMMENTSIZE_SHIFT: i32 = 16;
pub const D3DSI_INSTLENGTH_MASK: i32 = 251658240;
pub const D3DSI_INSTLENGTH_SHIFT: i32 = 24;
pub const D3DSI_OPCODE_MASK: i32 = 65535;
pub const D3DSI_TEXLD_BIAS: i32 = 131072;
pub const D3DSI_TEXLD_PROJECT: i32 = 65536;
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
pub const D3DSPDM_MSAMPCENTROID: i32 = 4194304;
pub const D3DSPDM_NONE: i32 = 0;
pub const D3DSPDM_PARTIALPRECISION: i32 = 2097152;
pub const D3DSPDM_SATURATE: i32 = 1048576;
pub const D3DSPD_IUNKNOWN: i32 = 1;
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
pub const D3DSP_DCL_USAGEINDEX_MASK: i32 = 983040;
pub const D3DSP_DCL_USAGEINDEX_SHIFT: i32 = 16;
pub const D3DSP_DCL_USAGE_MASK: i32 = 15;
pub const D3DSP_DCL_USAGE_SHIFT: i32 = 0;
pub const D3DSP_DSTMOD_MASK: i32 = 15728640;
pub const D3DSP_DSTMOD_SHIFT: i32 = 20;
pub const D3DSP_DSTSHIFT_MASK: i32 = 251658240;
pub const D3DSP_DSTSHIFT_SHIFT: i32 = 24;
pub const D3DSP_MIN_PRECISION_MASK: i32 = 49152;
pub const D3DSP_MIN_PRECISION_SHIFT: i32 = 14;
pub const D3DSP_NOSWIZZLE: i32 = 14942208;
pub const D3DSP_OPCODESPECIFICCONTROL_MASK: i32 = 16711680;
pub const D3DSP_OPCODESPECIFICCONTROL_SHIFT: i32 = 16;
pub const D3DSP_REGNUM_MASK: i32 = 2047;
pub const D3DSP_REGTYPE_MASK: i32 = 1879048192;
pub const D3DSP_REGTYPE_MASK2: i32 = 6144;
pub const D3DSP_REGTYPE_SHIFT: i32 = 28;
pub const D3DSP_REGTYPE_SHIFT2: i32 = 8;
pub const D3DSP_REPLICATEALPHA: i32 = 16711680;
pub const D3DSP_REPLICATEBLUE: i32 = 11141120;
pub const D3DSP_REPLICATEGREEN: i32 = 5570560;
pub const D3DSP_REPLICATERED: i32 = 0;
pub const D3DSP_SRCMOD_MASK: i32 = 251658240;
pub const D3DSP_SRCMOD_SHIFT: i32 = 24;
pub const D3DSP_SWIZZLE_MASK: i32 = 16711680;
pub const D3DSP_SWIZZLE_SHIFT: i32 = 16;
pub const D3DSP_TEXTURETYPE_MASK: i32 = 2013265920;
pub const D3DSP_TEXTURETYPE_SHIFT: i32 = 27;
pub const D3DSP_WRITEMASK_0: i32 = 65536;
pub const D3DSP_WRITEMASK_1: i32 = 131072;
pub const D3DSP_WRITEMASK_2: i32 = 262144;
pub const D3DSP_WRITEMASK_3: i32 = 524288;
pub const D3DSP_WRITEMASK_ALL: i32 = 983040;
pub const D3DSRO_FOG: D3DVS_RASTOUT_OFFSETS = 1;
pub const D3DSRO_FORCE_DWORD: D3DVS_RASTOUT_OFFSETS = 2147483647;
pub const D3DSRO_POINT_SIZE: D3DVS_RASTOUT_OFFSETS = 2;
pub const D3DSRO_POSITION: D3DVS_RASTOUT_OFFSETS = 0;
pub type D3DSTATEBLOCKTYPE = i32;
pub const D3DSTENCILCAPS_DECR: i32 = 128;
pub const D3DSTENCILCAPS_DECRSAT: i32 = 16;
pub const D3DSTENCILCAPS_INCR: i32 = 64;
pub const D3DSTENCILCAPS_INCRSAT: i32 = 8;
pub const D3DSTENCILCAPS_INVERT: i32 = 32;
pub const D3DSTENCILCAPS_KEEP: i32 = 1;
pub const D3DSTENCILCAPS_REPLACE: i32 = 4;
pub const D3DSTENCILCAPS_TWOSIDED: i32 = 256;
pub const D3DSTENCILCAPS_ZERO: i32 = 2;
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
pub const D3DSTREAMSOURCE_INDEXEDDATA: i32 = 1073741824;
pub const D3DSTREAMSOURCE_INSTANCEDATA: i32 = -2147483648;
pub const D3DSTT_2D: D3DSAMPLER_TEXTURE_TYPE = 268435456;
pub const D3DSTT_CUBE: D3DSAMPLER_TEXTURE_TYPE = 402653184;
pub const D3DSTT_FORCE_DWORD: D3DSAMPLER_TEXTURE_TYPE = 2147483647;
pub const D3DSTT_UNKNOWN: D3DSAMPLER_TEXTURE_TYPE = 0;
pub const D3DSTT_VOLUME: D3DSAMPLER_TEXTURE_TYPE = 536870912;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DSURFACE_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: super::DWORD,
    pub Pool: D3DPOOL,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: super::DWORD,
    pub Width: super::UINT,
    pub Height: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSURFACE_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: super::DWORD,
    pub Pool: D3DPOOL,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: super::DWORD,
    pub Width: super::UINT,
    pub Height: super::UINT,
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
pub const D3DTA_ALPHAREPLICATE: i32 = 32;
pub const D3DTA_COMPLEMENT: i32 = 16;
pub const D3DTA_CONSTANT: i32 = 6;
pub const D3DTA_CURRENT: i32 = 1;
pub const D3DTA_DIFFUSE: i32 = 0;
pub const D3DTA_SELECTMASK: i32 = 15;
pub const D3DTA_SPECULAR: i32 = 4;
pub const D3DTA_TEMP: i32 = 5;
pub const D3DTA_TEXTURE: i32 = 2;
pub const D3DTA_TFACTOR: i32 = 3;
pub const D3DTEXF_ANISOTROPIC: D3DTEXTUREFILTERTYPE = 3;
pub const D3DTEXF_CONVOLUTIONMONO: D3DTEXTUREFILTERTYPE = 8;
pub const D3DTEXF_FORCE_DWORD: D3DTEXTUREFILTERTYPE = 2147483647;
pub const D3DTEXF_GAUSSIANQUAD: D3DTEXTUREFILTERTYPE = 7;
pub const D3DTEXF_LINEAR: D3DTEXTUREFILTERTYPE = 2;
pub const D3DTEXF_NONE: D3DTEXTUREFILTERTYPE = 0;
pub const D3DTEXF_POINT: D3DTEXTUREFILTERTYPE = 1;
pub const D3DTEXF_PYRAMIDALQUAD: D3DTEXTUREFILTERTYPE = 6;
pub const D3DTEXOPCAPS_ADD: i32 = 64;
pub const D3DTEXOPCAPS_ADDSIGNED: i32 = 128;
pub const D3DTEXOPCAPS_ADDSIGNED2X: i32 = 256;
pub const D3DTEXOPCAPS_ADDSMOOTH: i32 = 1024;
pub const D3DTEXOPCAPS_BLENDCURRENTALPHA: i32 = 32768;
pub const D3DTEXOPCAPS_BLENDDIFFUSEALPHA: i32 = 2048;
pub const D3DTEXOPCAPS_BLENDFACTORALPHA: i32 = 8192;
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHA: i32 = 4096;
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHAPM: i32 = 16384;
pub const D3DTEXOPCAPS_BUMPENVMAP: i32 = 2097152;
pub const D3DTEXOPCAPS_BUMPENVMAPLUMINANCE: i32 = 4194304;
pub const D3DTEXOPCAPS_DISABLE: i32 = 1;
pub const D3DTEXOPCAPS_DOTPRODUCT3: i32 = 8388608;
pub const D3DTEXOPCAPS_LERP: i32 = 33554432;
pub const D3DTEXOPCAPS_MODULATE: i32 = 8;
pub const D3DTEXOPCAPS_MODULATE2X: i32 = 16;
pub const D3DTEXOPCAPS_MODULATE4X: i32 = 32;
pub const D3DTEXOPCAPS_MODULATEALPHA_ADDCOLOR: i32 = 131072;
pub const D3DTEXOPCAPS_MODULATECOLOR_ADDALPHA: i32 = 262144;
pub const D3DTEXOPCAPS_MODULATEINVALPHA_ADDCOLOR: i32 = 524288;
pub const D3DTEXOPCAPS_MODULATEINVCOLOR_ADDALPHA: i32 = 1048576;
pub const D3DTEXOPCAPS_MULTIPLYADD: i32 = 16777216;
pub const D3DTEXOPCAPS_PREMODULATE: i32 = 65536;
pub const D3DTEXOPCAPS_SELECTARG1: i32 = 2;
pub const D3DTEXOPCAPS_SELECTARG2: i32 = 4;
pub const D3DTEXOPCAPS_SUBTRACT: i32 = 512;
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
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DTRIPATCH_INFO {
    pub StartVertexOffset: super::UINT,
    pub NumVertices: super::UINT,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTRIPATCH_INFO {
    pub StartVertexOffset: super::UINT,
    pub NumVertices: super::UINT,
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
pub const D3DTSS_TCI_CAMERASPACENORMAL: i32 = 65536;
pub const D3DTSS_TCI_CAMERASPACEPOSITION: i32 = 131072;
pub const D3DTSS_TCI_CAMERASPACEREFLECTIONVECTOR: i32 = 196608;
pub const D3DTSS_TCI_PASSTHRU: i32 = 0;
pub const D3DTSS_TCI_SPHEREMAP: i32 = 262144;
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
pub const D3DTS_WORLD: D3DTRANSFORMSTATETYPE = 256;
pub const D3DTS_WORLD1: D3DTRANSFORMSTATETYPE = 257;
pub const D3DTS_WORLD2: D3DTRANSFORMSTATETYPE = 258;
pub const D3DTS_WORLD3: D3DTRANSFORMSTATETYPE = 259;
pub const D3DTTFF_COUNT1: D3DTEXTURETRANSFORMFLAGS = 1;
pub const D3DTTFF_COUNT2: D3DTEXTURETRANSFORMFLAGS = 2;
pub const D3DTTFF_COUNT3: D3DTEXTURETRANSFORMFLAGS = 3;
pub const D3DTTFF_COUNT4: D3DTEXTURETRANSFORMFLAGS = 4;
pub const D3DTTFF_DISABLE: D3DTEXTURETRANSFORMFLAGS = 0;
pub const D3DTTFF_FORCE_DWORD: D3DTEXTURETRANSFORMFLAGS = 2147483647;
pub const D3DTTFF_PROJECTED: D3DTEXTURETRANSFORMFLAGS = 256;
pub const D3DUSAGE_AUTOGENMIPMAP: i32 = 1024;
pub const D3DUSAGE_DEPTHSTENCIL: i32 = 2;
pub const D3DUSAGE_DMAP: i32 = 16384;
pub const D3DUSAGE_DONOTCLIP: i32 = 32;
pub const D3DUSAGE_DYNAMIC: i32 = 512;
pub const D3DUSAGE_NONSECURE: i32 = 8388608;
pub const D3DUSAGE_NPATCHES: i32 = 256;
pub const D3DUSAGE_POINTS: i32 = 64;
pub const D3DUSAGE_QUERY_FILTER: i32 = 131072;
pub const D3DUSAGE_QUERY_LEGACYBUMPMAP: i32 = 32768;
pub const D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING: i32 = 524288;
pub const D3DUSAGE_QUERY_SRGBREAD: i32 = 65536;
pub const D3DUSAGE_QUERY_SRGBWRITE: i32 = 262144;
pub const D3DUSAGE_QUERY_VERTEXTEXTURE: i32 = 1048576;
pub const D3DUSAGE_QUERY_WRAPANDMIP: i32 = 2097152;
pub const D3DUSAGE_RENDERTARGET: i32 = 1;
pub const D3DUSAGE_RESTRICTED_CONTENT: i32 = 2048;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE: i32 = 8192;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER: i32 = 4096;
pub const D3DUSAGE_RTPATCHES: i32 = 128;
pub const D3DUSAGE_SOFTWAREPROCESSING: i32 = 16;
pub const D3DUSAGE_TEXTAPI: i32 = 268435456;
pub const D3DUSAGE_WRITEONLY: i32 = 8;
pub const D3DVBF_0WEIGHTS: D3DVERTEXBLENDFLAGS = 256;
pub const D3DVBF_1WEIGHTS: D3DVERTEXBLENDFLAGS = 1;
pub const D3DVBF_2WEIGHTS: D3DVERTEXBLENDFLAGS = 2;
pub const D3DVBF_3WEIGHTS: D3DVERTEXBLENDFLAGS = 3;
pub const D3DVBF_DISABLE: D3DVERTEXBLENDFLAGS = 0;
pub const D3DVBF_FORCE_DWORD: D3DVERTEXBLENDFLAGS = 2147483647;
pub const D3DVBF_TWEENING: D3DVERTEXBLENDFLAGS = 255;
pub type D3DVERTEXBLENDFLAGS = i32;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DVERTEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: super::DWORD,
    pub Pool: D3DPOOL,
    pub Size: super::UINT,
    pub FVF: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DVERTEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: super::DWORD,
    pub Pool: D3DPOOL,
    pub Size: super::UINT,
    pub FVF: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DVERTEXELEMENT9 {
    pub Stream: super::WORD,
    pub Offset: super::WORD,
    pub Type: super::BYTE,
    pub Method: super::BYTE,
    pub Usage: super::BYTE,
    pub UsageIndex: super::BYTE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DVERTEXELEMENT9 {
    pub Stream: super::WORD,
    pub Offset: super::WORD,
    pub Type: super::BYTE,
    pub Method: super::BYTE,
    pub Usage: super::BYTE,
    pub UsageIndex: super::BYTE,
}
pub const D3DVERTEXTEXTURESAMPLER0: i32 = 257;
pub const D3DVERTEXTEXTURESAMPLER1: i32 = 258;
pub const D3DVERTEXTEXTURESAMPLER2: i32 = 259;
pub const D3DVERTEXTEXTURESAMPLER3: i32 = 260;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DVIEWPORT9 {
    pub X: super::DWORD,
    pub Y: super::DWORD,
    pub Width: super::DWORD,
    pub Height: super::DWORD,
    pub MinZ: f32,
    pub MaxZ: f32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVIEWPORT9 {
    pub X: super::DWORD,
    pub Y: super::DWORD,
    pub Width: super::DWORD,
    pub Height: super::DWORD,
    pub MinZ: f32,
    pub MaxZ: f32,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DVOLUME_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: super::DWORD,
    pub Pool: D3DPOOL,
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Depth: super::UINT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DVOLUME_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: super::DWORD,
    pub Pool: D3DPOOL,
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Depth: super::UINT,
}
pub const D3DVS20CAPS_PREDICATION: i32 = 1;
pub const D3DVS20_MAX_DYNAMICFLOWCONTROLDEPTH: i32 = 24;
pub const D3DVS20_MAX_NUMTEMPS: i32 = 32;
pub const D3DVS20_MAX_STATICFLOWCONTROLDEPTH: i32 = 4;
pub const D3DVS20_MIN_DYNAMICFLOWCONTROLDEPTH: i32 = 0;
pub const D3DVS20_MIN_NUMTEMPS: i32 = 12;
pub const D3DVS20_MIN_STATICFLOWCONTROLDEPTH: i32 = 1;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct D3DVSHADERCAPS2_0 {
    pub Caps: super::DWORD,
    pub DynamicFlowControlDepth: super::INT,
    pub NumTemps: super::INT,
    pub StaticFlowControlDepth: super::INT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DVSHADERCAPS2_0 {
    pub Caps: super::DWORD,
    pub DynamicFlowControlDepth: super::INT,
    pub NumTemps: super::INT,
    pub StaticFlowControlDepth: super::INT,
}
pub const D3DVS_ADDRESSMODE_MASK: i32 = 8192;
pub const D3DVS_ADDRESSMODE_SHIFT: i32 = 13;
pub type D3DVS_ADDRESSMODE_TYPE = i32;
pub const D3DVS_ADDRMODE_ABSOLUTE: D3DVS_ADDRESSMODE_TYPE = 0;
pub const D3DVS_ADDRMODE_FORCE_DWORD: D3DVS_ADDRESSMODE_TYPE = 2147483647;
pub const D3DVS_ADDRMODE_RELATIVE: D3DVS_ADDRESSMODE_TYPE = 8192;
pub const D3DVS_NOSWIZZLE: i32 = 14942208;
pub type D3DVS_RASTOUT_OFFSETS = i32;
pub const D3DVS_SWIZZLE_MASK: i32 = 16711680;
pub const D3DVS_SWIZZLE_SHIFT: i32 = 16;
pub const D3DVS_W_W: i32 = 12582912;
pub const D3DVS_W_X: i32 = 0;
pub const D3DVS_W_Y: i32 = 4194304;
pub const D3DVS_W_Z: i32 = 8388608;
pub const D3DVS_X_W: i32 = 196608;
pub const D3DVS_X_X: i32 = 0;
pub const D3DVS_X_Y: i32 = 65536;
pub const D3DVS_X_Z: i32 = 131072;
pub const D3DVS_Y_W: i32 = 786432;
pub const D3DVS_Y_X: i32 = 0;
pub const D3DVS_Y_Y: i32 = 262144;
pub const D3DVS_Y_Z: i32 = 524288;
pub const D3DVS_Z_W: i32 = 3145728;
pub const D3DVS_Z_X: i32 = 0;
pub const D3DVS_Z_Y: i32 = 1048576;
pub const D3DVS_Z_Z: i32 = 2097152;
pub const D3DVTXPCAPS_DIRECTIONALLIGHTS: i32 = 8;
pub const D3DVTXPCAPS_LOCALVIEWER: i32 = 32;
pub const D3DVTXPCAPS_MATERIALSOURCE7: i32 = 2;
pub const D3DVTXPCAPS_NO_TEXGEN_NONLOCALVIEWER: i32 = 512;
pub const D3DVTXPCAPS_POSITIONALLIGHTS: i32 = 16;
pub const D3DVTXPCAPS_TEXGEN: i32 = 1;
pub const D3DVTXPCAPS_TEXGEN_SPHEREMAP: i32 = 256;
pub const D3DVTXPCAPS_TWEENING: i32 = 64;
pub const D3DWRAPCOORD_0: i32 = 1;
pub const D3DWRAPCOORD_1: i32 = 2;
pub const D3DWRAPCOORD_2: i32 = 4;
pub const D3DWRAPCOORD_3: i32 = 8;
pub const D3DWRAP_U: i32 = 1;
pub const D3DWRAP_V: i32 = 2;
pub const D3DWRAP_W: i32 = 4;
pub type D3DZBUFFERTYPE = i32;
pub const D3DZB_FALSE: D3DZBUFFERTYPE = 0;
pub const D3DZB_FORCE_DWORD: D3DZBUFFERTYPE = 2147483647;
pub const D3DZB_TRUE: D3DZBUFFERTYPE = 1;
pub const D3DZB_USEW: D3DZBUFFERTYPE = 2;
pub const D3D_MAX_SIMULTANEOUS_RENDERTARGETS: i32 = 4;
pub const D3D_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct D3D_OMAC {
    pub Omac: [super::BYTE; 16],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for D3D_OMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D_OMAC {
    pub Omac: [super::BYTE; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for D3D_OMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D_OMAC_SIZE: i32 = 16;
pub const D3D_SDK_VERSION: i32 = 32;
pub const DIRECT3D_VERSION: i32 = 2304;
windows_core::imp::define_interface!(IDirect3D9, IDirect3D9_Vtbl, 0x81bdcbca_64d4_426d_ae8d_ad0147f4275c);
windows_core::imp::interface_hierarchy!(IDirect3D9, windows_core::IUnknown);
impl IDirect3D9 {
    pub unsafe fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterSoftwareDevice)(windows_core::Interface::as_raw(self), pinitializefunction as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAdapterCount(&self) -> super::UINT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetAdapterIdentifier(&self, adapter: super::UINT, flags: super::DWORD, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterIdentifier)(windows_core::Interface::as_raw(self), adapter, flags, pidentifier as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAdapterModeCount(&self, adapter: super::UINT, format: D3DFORMAT) -> super::UINT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterModeCount)(windows_core::Interface::as_raw(self), adapter, format) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumAdapterModes(&self, adapter: super::UINT, format: D3DFORMAT, mode: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterModes)(windows_core::Interface::as_raw(self), adapter, format, mode, pmode as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAdapterDisplayMode(&self, adapter: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterDisplayMode)(windows_core::Interface::as_raw(self), adapter, pmode as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CheckDeviceType(&self, adapter: super::UINT, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceType)(windows_core::Interface::as_raw(self), adapter, devtype, adapterformat, backbufferformat, bwindowed) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CheckDeviceFormat(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: super::DWORD, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceFormat)(windows_core::Interface::as_raw(self), adapter, devicetype, adapterformat, usage, rtype, checkformat) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CheckDeviceMultiSampleType(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: windows_core::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceMultiSampleType)(windows_core::Interface::as_raw(self), adapter, devicetype, surfaceformat, windowed, multisampletype, pqualitylevels as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CheckDepthStencilMatch(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDepthStencilMatch)(windows_core::Interface::as_raw(self), adapter, devicetype, adapterformat, rendertargetformat, depthstencilformat) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CheckDeviceFormatConversion(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceFormatConversion)(windows_core::Interface::as_raw(self), adapter, devicetype, sourceformat, targetformat) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDeviceCaps(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceCaps)(windows_core::Interface::as_raw(self), adapter, devicetype, pcaps as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetAdapterMonitor(&self, adapter: super::UINT) -> super::HMONITOR {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterMonitor)(windows_core::Interface::as_raw(self), adapter) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn CreateDevice(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, hfocuswindow: super::HWND, behaviorflags: super::DWORD, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), adapter, devicetype, hfocuswindow as _, behaviorflags, ppresentationparameters as _, ppreturneddeviceinterface as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterSoftwareDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetAdapterCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::UINT,
    #[cfg(not(feature = "shared"))]
    GetAdapterCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetAdapterIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::DWORD, *mut D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetAdapterIdentifier: usize,
    #[cfg(feature = "shared")]
    pub GetAdapterModeCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DFORMAT) -> super::UINT,
    #[cfg(not(feature = "shared"))]
    GetAdapterModeCount: usize,
    #[cfg(feature = "shared")]
    pub EnumAdapterModes: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DFORMAT, super::UINT, *mut D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumAdapterModes: usize,
    #[cfg(feature = "shared")]
    pub GetAdapterDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAdapterDisplayMode: usize,
    #[cfg(feature = "shared")]
    pub CheckDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, D3DFORMAT, D3DFORMAT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CheckDeviceType: usize,
    #[cfg(feature = "shared")]
    pub CheckDeviceFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, D3DFORMAT, super::DWORD, D3DRESOURCETYPE, D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CheckDeviceFormat: usize,
    #[cfg(feature = "shared")]
    pub CheckDeviceMultiSampleType: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, D3DFORMAT, windows_core::BOOL, D3DMULTISAMPLE_TYPE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CheckDeviceMultiSampleType: usize,
    #[cfg(feature = "shared")]
    pub CheckDepthStencilMatch: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, D3DFORMAT, D3DFORMAT, D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CheckDepthStencilMatch: usize,
    #[cfg(feature = "shared")]
    pub CheckDeviceFormatConversion: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, D3DFORMAT, D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CheckDeviceFormatConversion: usize,
    #[cfg(feature = "shared")]
    pub GetDeviceCaps: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, *mut D3DCAPS9) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDeviceCaps: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetAdapterMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> super::HMONITOR,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetAdapterMonitor: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, super::HWND, super::DWORD, *mut D3DPRESENT_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    CreateDevice: usize,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub trait IDirect3D9_Impl: windows_core::IUnknownImpl {
    fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterCount(&self) -> super::UINT;
    fn GetAdapterIdentifier(&self, adapter: super::UINT, flags: super::DWORD, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> windows_core::Result<()>;
    fn GetAdapterModeCount(&self, adapter: super::UINT, format: D3DFORMAT) -> super::UINT;
    fn EnumAdapterModes(&self, adapter: super::UINT, format: D3DFORMAT, mode: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetAdapterDisplayMode(&self, adapter: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn CheckDeviceType(&self, adapter: super::UINT, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::Result<()>;
    fn CheckDeviceFormat(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: super::DWORD, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceMultiSampleType(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: windows_core::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut super::DWORD) -> windows_core::Result<()>;
    fn CheckDepthStencilMatch(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceFormatConversion(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> windows_core::Result<()>;
    fn GetDeviceCaps(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> windows_core::Result<()>;
    fn GetAdapterMonitor(&self, adapter: super::UINT) -> super::HMONITOR;
    fn CreateDevice(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, hfocuswindow: super::HWND, behaviorflags: super::DWORD, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl IDirect3D9_Vtbl {
    pub const fn new<Identity: IDirect3D9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterSoftwareDevice<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitializefunction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::RegisterSoftwareDevice(this, core::mem::transmute_copy(&pinitializefunction)).into()
            }
        }
        unsafe extern "system" fn GetAdapterCount<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::UINT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterCount(this)
            }
        }
        unsafe extern "system" fn GetAdapterIdentifier<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, flags: super::DWORD, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterIdentifier(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pidentifier)).into()
            }
        }
        unsafe extern "system" fn GetAdapterModeCount<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, format: D3DFORMAT) -> super::UINT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterModeCount(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format))
            }
        }
        unsafe extern "system" fn EnumAdapterModes<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, format: D3DFORMAT, mode: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::EnumAdapterModes(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterDisplayMode(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceType<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devtype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&backbufferformat), core::mem::transmute_copy(&bwindowed)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceFormat<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: super::DWORD, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceFormat(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&rtype), core::mem::transmute_copy(&checkformat)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: windows_core::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceMultiSampleType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&surfaceformat), core::mem::transmute_copy(&windowed), core::mem::transmute_copy(&multisampletype), core::mem::transmute_copy(&pqualitylevels)).into()
            }
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDepthStencilMatch(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&depthstencilformat)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceFormatConversion(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&sourceformat), core::mem::transmute_copy(&targetformat)).into()
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetAdapterMonitor<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT) -> super::HMONITOR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterMonitor(this, core::mem::transmute_copy(&adapter))
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devicetype: D3DDEVTYPE, hfocuswindow: super::HWND, behaviorflags: super::DWORD, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CreateDevice(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterSoftwareDevice: RegisterSoftwareDevice::<Identity, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, OFFSET>,
            GetAdapterIdentifier: GetAdapterIdentifier::<Identity, OFFSET>,
            GetAdapterModeCount: GetAdapterModeCount::<Identity, OFFSET>,
            EnumAdapterModes: EnumAdapterModes::<Identity, OFFSET>,
            GetAdapterDisplayMode: GetAdapterDisplayMode::<Identity, OFFSET>,
            CheckDeviceType: CheckDeviceType::<Identity, OFFSET>,
            CheckDeviceFormat: CheckDeviceFormat::<Identity, OFFSET>,
            CheckDeviceMultiSampleType: CheckDeviceMultiSampleType::<Identity, OFFSET>,
            CheckDepthStencilMatch: CheckDepthStencilMatch::<Identity, OFFSET>,
            CheckDeviceFormatConversion: CheckDeviceFormatConversion::<Identity, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, OFFSET>,
            GetAdapterMonitor: GetAdapterMonitor::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3D9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IDirect3D9 {}
windows_core::imp::define_interface!(IDirect3D9Ex, IDirect3D9Ex_Vtbl, 0x02177241_69fc_400c_8ff1_93a44df6861d);
impl core::ops::Deref for IDirect3D9Ex {
    type Target = IDirect3D9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3D9Ex, windows_core::IUnknown, IDirect3D9);
impl IDirect3D9Ex {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAdapterModeCountEx(&self, adapter: super::UINT, pfilter: *const D3DDISPLAYMODEFILTER) -> super::UINT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterModeCountEx)(windows_core::Interface::as_raw(self), adapter, pfilter as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumAdapterModesEx(&self, adapter: super::UINT, pfilter: *const D3DDISPLAYMODEFILTER, mode: super::UINT, pmode: *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterModesEx)(windows_core::Interface::as_raw(self), adapter, pfilter as _, mode, pmode as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAdapterDisplayModeEx(&self, adapter: super::UINT, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterDisplayModeEx)(windows_core::Interface::as_raw(self), adapter, pmode as _, protation as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn CreateDeviceEx(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, hfocuswindow: super::HWND, behaviorflags: super::DWORD, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9Ex>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDeviceEx)(windows_core::Interface::as_raw(self), adapter, devicetype, hfocuswindow as _, behaviorflags, ppresentationparameters as _, pfullscreendisplaymode as _, ppreturneddeviceinterface as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetAdapterLUID(&self, adapter: super::UINT, pluid: *mut super::LUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterLUID)(windows_core::Interface::as_raw(self), adapter, pluid as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9Ex_Vtbl {
    pub base__: IDirect3D9_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAdapterModeCountEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const D3DDISPLAYMODEFILTER) -> super::UINT,
    #[cfg(not(feature = "shared"))]
    GetAdapterModeCountEx: usize,
    #[cfg(feature = "shared")]
    pub EnumAdapterModesEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const D3DDISPLAYMODEFILTER, super::UINT, *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumAdapterModesEx: usize,
    #[cfg(feature = "shared")]
    pub GetAdapterDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DDISPLAYMODEEX, *mut D3DDISPLAYROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAdapterDisplayModeEx: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub CreateDeviceEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, super::HWND, super::DWORD, *mut D3DPRESENT_PARAMETERS, *mut D3DDISPLAYMODEEX, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    CreateDeviceEx: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetAdapterLUID: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::LUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetAdapterLUID: usize,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub trait IDirect3D9Ex_Impl: IDirect3D9_Impl {
    fn GetAdapterModeCountEx(&self, adapter: super::UINT, pfilter: *const D3DDISPLAYMODEFILTER) -> super::UINT;
    fn EnumAdapterModesEx(&self, adapter: super::UINT, pfilter: *const D3DDISPLAYMODEFILTER, mode: super::UINT, pmode: *mut D3DDISPLAYMODEEX) -> windows_core::Result<()>;
    fn GetAdapterDisplayModeEx(&self, adapter: super::UINT, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
    fn CreateDeviceEx(&self, adapter: super::UINT, devicetype: D3DDEVTYPE, hfocuswindow: super::HWND, behaviorflags: super::DWORD, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9Ex>) -> windows_core::Result<()>;
    fn GetAdapterLUID(&self, adapter: super::UINT, pluid: *mut super::LUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl IDirect3D9Ex_Vtbl {
    pub const fn new<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdapterModeCountEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, pfilter: *const D3DDISPLAYMODEFILTER) -> super::UINT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterModeCountEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pfilter))
            }
        }
        unsafe extern "system" fn EnumAdapterModesEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, pfilter: *const D3DDISPLAYMODEFILTER, mode: super::UINT, pmode: *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::EnumAdapterModesEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterDisplayModeEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        unsafe extern "system" fn CreateDeviceEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devicetype: D3DDEVTYPE, hfocuswindow: super::HWND, behaviorflags: super::DWORD, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::CreateDeviceEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pfullscreendisplaymode), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
            }
        }
        unsafe extern "system" fn GetAdapterLUID<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, pluid: *mut super::LUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterLUID(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pluid)).into()
            }
        }
        Self {
            base__: IDirect3D9_Vtbl::new::<Identity, OFFSET>(),
            GetAdapterModeCountEx: GetAdapterModeCountEx::<Identity, OFFSET>,
            EnumAdapterModesEx: EnumAdapterModesEx::<Identity, OFFSET>,
            GetAdapterDisplayModeEx: GetAdapterDisplayModeEx::<Identity, OFFSET>,
            CreateDeviceEx: CreateDeviceEx::<Identity, OFFSET>,
            GetAdapterLUID: GetAdapterLUID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3D9Ex as windows_core::Interface>::IID || iid == &<IDirect3D9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IDirect3D9Ex {}
windows_core::imp::define_interface!(IDirect3D9ExOverlayExtension, IDirect3D9ExOverlayExtension_Vtbl, 0x187aeb13_aaf5_4c59_876d_e059088c0df8);
windows_core::imp::interface_hierarchy!(IDirect3D9ExOverlayExtension, windows_core::IUnknown);
impl IDirect3D9ExOverlayExtension {
    #[cfg(feature = "shared")]
    pub unsafe fn CheckDeviceOverlayType(&self, adapter: super::UINT, devtype: D3DDEVTYPE, overlaywidth: super::UINT, overlayheight: super::UINT, overlayformat: D3DFORMAT, pdisplaymode: *mut D3DDISPLAYMODEEX, displayrotation: D3DDISPLAYROTATION, poverlaycaps: *mut D3DOVERLAYCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceOverlayType)(windows_core::Interface::as_raw(self), adapter, devtype, overlaywidth, overlayheight, overlayformat, pdisplaymode as _, displayrotation, poverlaycaps as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9ExOverlayExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub CheckDeviceOverlayType: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DDEVTYPE, super::UINT, super::UINT, D3DFORMAT, *mut D3DDISPLAYMODEEX, D3DDISPLAYROTATION, *mut D3DOVERLAYCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CheckDeviceOverlayType: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3D9ExOverlayExtension_Impl: windows_core::IUnknownImpl {
    fn CheckDeviceOverlayType(&self, adapter: super::UINT, devtype: D3DDEVTYPE, overlaywidth: super::UINT, overlayheight: super::UINT, overlayformat: D3DFORMAT, pdisplaymode: *mut D3DDISPLAYMODEEX, displayrotation: D3DDISPLAYROTATION, poverlaycaps: *mut D3DOVERLAYCAPS) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3D9ExOverlayExtension_Vtbl {
    pub const fn new<Identity: IDirect3D9ExOverlayExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckDeviceOverlayType<Identity: IDirect3D9ExOverlayExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: super::UINT, devtype: D3DDEVTYPE, overlaywidth: super::UINT, overlayheight: super::UINT, overlayformat: D3DFORMAT, pdisplaymode: *mut D3DDISPLAYMODEEX, displayrotation: D3DDISPLAYROTATION, poverlaycaps: *mut D3DOVERLAYCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9ExOverlayExtension_Impl::CheckDeviceOverlayType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devtype), core::mem::transmute_copy(&overlaywidth), core::mem::transmute_copy(&overlayheight), core::mem::transmute_copy(&overlayformat), core::mem::transmute_copy(&pdisplaymode), core::mem::transmute_copy(&displayrotation), core::mem::transmute_copy(&poverlaycaps)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CheckDeviceOverlayType: CheckDeviceOverlayType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3D9ExOverlayExtension as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3D9ExOverlayExtension {}
windows_core::imp::define_interface!(IDirect3DAuthenticatedChannel9, IDirect3DAuthenticatedChannel9_Vtbl, 0xff24beee_da21_4beb_98b5_d2f899f98af9);
windows_core::imp::interface_hierarchy!(IDirect3DAuthenticatedChannel9, windows_core::IUnknown);
impl IDirect3DAuthenticatedChannel9 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCertificateSize(&self, pcertificatesize: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCertificateSize)(windows_core::Interface::as_raw(self), pcertificatesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCertificate(&self, certifactesize: super::UINT, ppcertificate: *mut super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), certifactesize, ppcertificate as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn NegotiateKeyExchange(&self, datasize: super::UINT, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NegotiateKeyExchange)(windows_core::Interface::as_raw(self), datasize, pdata as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Query(&self, inputsize: super::UINT, pinput: *const core::ffi::c_void, outputsize: super::UINT, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), inputsize, pinput as _, outputsize, poutput as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Configure(&self, inputsize: super::UINT, pinput: *const core::ffi::c_void, poutput: *mut D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), inputsize, pinput as _, poutput as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DAuthenticatedChannel9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCertificateSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCertificateSize: usize,
    #[cfg(feature = "shared")]
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCertificate: usize,
    #[cfg(feature = "shared")]
    pub NegotiateKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    NegotiateKeyExchange: usize,
    #[cfg(feature = "shared")]
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const core::ffi::c_void, super::UINT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Query: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const core::ffi::c_void, *mut D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Configure: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IDirect3DAuthenticatedChannel9_Impl: windows_core::IUnknownImpl {
    fn GetCertificateSize(&self, pcertificatesize: *mut super::UINT) -> windows_core::Result<()>;
    fn GetCertificate(&self, certifactesize: super::UINT, ppcertificate: *mut super::BYTE) -> windows_core::Result<()>;
    fn NegotiateKeyExchange(&self, datasize: super::UINT, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Query(&self, inputsize: super::UINT, pinput: *const core::ffi::c_void, outputsize: super::UINT, poutput: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Configure(&self, inputsize: super::UINT, pinput: *const core::ffi::c_void, poutput: *mut D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IDirect3DAuthenticatedChannel9_Vtbl {
    pub const fn new<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCertificateSize<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertificatesize: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::GetCertificateSize(this, core::mem::transmute_copy(&pcertificatesize)).into()
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certifactesize: super::UINT, ppcertificate: *mut super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::GetCertificate(this, core::mem::transmute_copy(&certifactesize), core::mem::transmute_copy(&ppcertificate)).into()
            }
        }
        unsafe extern "system" fn NegotiateKeyExchange<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datasize: super::UINT, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::NegotiateKeyExchange(this, core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputsize: super::UINT, pinput: *const core::ffi::c_void, outputsize: super::UINT, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::Query(this, core::mem::transmute_copy(&inputsize), core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&outputsize), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn Configure<Identity: IDirect3DAuthenticatedChannel9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputsize: super::UINT, pinput: *const core::ffi::c_void, poutput: *mut D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DAuthenticatedChannel9_Impl::Configure(this, core::mem::transmute_copy(&inputsize), core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCertificateSize: GetCertificateSize::<Identity, OFFSET>,
            GetCertificate: GetCertificate::<Identity, OFFSET>,
            NegotiateKeyExchange: NegotiateKeyExchange::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DAuthenticatedChannel9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IDirect3DAuthenticatedChannel9 {}
windows_core::imp::define_interface!(IDirect3DBaseTexture9, IDirect3DBaseTexture9_Vtbl, 0x580ca87e_1d3c_4d54_991d_b7d3e3c298ce);
impl core::ops::Deref for IDirect3DBaseTexture9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DBaseTexture9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DBaseTexture9 {
    #[cfg(feature = "shared")]
    pub unsafe fn SetLOD(&self, lodnew: super::DWORD) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).SetLOD)(windows_core::Interface::as_raw(self), lodnew) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLOD(&self) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).GetLOD)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLevelCount(&self) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).GetLevelCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoGenFilterType)(windows_core::Interface::as_raw(self), filtertype) }
    }
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetAutoGenFilterType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMipSubLevels)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DBaseTexture9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    #[cfg(feature = "shared")]
    pub SetLOD: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    SetLOD: usize,
    #[cfg(feature = "shared")]
    pub GetLOD: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    GetLOD: usize,
    #[cfg(feature = "shared")]
    pub GetLevelCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    GetLevelCount: usize,
    pub SetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    pub GetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3DTEXTUREFILTERTYPE,
    pub GenerateMipSubLevels: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "shared")]
pub trait IDirect3DBaseTexture9_Impl: IDirect3DResource9_Impl {
    fn SetLOD(&self, lodnew: super::DWORD) -> super::DWORD;
    fn GetLOD(&self) -> super::DWORD;
    fn GetLevelCount(&self) -> super::DWORD;
    fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&self);
}
#[cfg(feature = "shared")]
impl IDirect3DBaseTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLOD<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lodnew: super::DWORD) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::SetLOD(this, core::mem::transmute_copy(&lodnew))
            }
        }
        unsafe extern "system" fn GetLOD<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetLOD(this)
            }
        }
        unsafe extern "system" fn GetLevelCount<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetLevelCount(this)
            }
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::SetAutoGenFilterType(this, core::mem::transmute_copy(&filtertype)).into()
            }
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DTEXTUREFILTERTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetAutoGenFilterType(this)
            }
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GenerateMipSubLevels(this);
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            SetLOD: SetLOD::<Identity, OFFSET>,
            GetLOD: GetLOD::<Identity, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DBaseTexture9 {}
windows_core::imp::define_interface!(IDirect3DCryptoSession9, IDirect3DCryptoSession9_Vtbl, 0xfa0ab799_7a9c_48ca_8c5b_237e71a54434);
windows_core::imp::interface_hierarchy!(IDirect3DCryptoSession9, windows_core::IUnknown);
impl IDirect3DCryptoSession9 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCertificateSize(&self, pcertificatesize: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCertificateSize)(windows_core::Interface::as_raw(self), pcertificatesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCertificate(&self, certifactesize: super::UINT, ppcertificate: *mut super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), certifactesize, ppcertificate as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn NegotiateKeyExchange(&self, datasize: super::UINT, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NegotiateKeyExchange)(windows_core::Interface::as_raw(self), datasize, pdata as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EncryptionBlt(&self, psrcsurface: &Option<IDirect3DSurface9>, pdstsurface: &Option<IDirect3DSurface9>, dstsurfacesize: super::UINT, piv: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EncryptionBlt)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psrcsurface), core::mem::transmute_copy(pdstsurface), dstsurfacesize, piv as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DecryptionBlt(&self, psrcsurface: &Option<IDirect3DSurface9>, pdstsurface: &Option<IDirect3DSurface9>, srcsurfacesize: super::UINT, pencryptedblockinfo: *mut D3DENCRYPTED_BLOCK_INFO, pcontentkey: *mut core::ffi::c_void, piv: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecryptionBlt)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psrcsurface), core::mem::transmute_copy(pdstsurface), srcsurfacesize, pencryptedblockinfo as _, pcontentkey as _, piv as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSurfacePitch(&self, psrcsurface: &Option<IDirect3DSurface9>, psurfacepitch: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSurfacePitch)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psrcsurface), psurfacepitch as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn StartSessionKeyRefresh(&self, prandomnumber: *mut core::ffi::c_void, randomnumbersize: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartSessionKeyRefresh)(windows_core::Interface::as_raw(self), prandomnumber as _, randomnumbersize) }
    }
    pub unsafe fn FinishSessionKeyRefresh(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FinishSessionKeyRefresh)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEncryptionBltKey(&self, preadbackkey: *mut core::ffi::c_void, keysize: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEncryptionBltKey)(windows_core::Interface::as_raw(self), preadbackkey as _, keysize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DCryptoSession9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCertificateSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCertificateSize: usize,
    #[cfg(feature = "shared")]
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCertificate: usize,
    #[cfg(feature = "shared")]
    pub NegotiateKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    NegotiateKeyExchange: usize,
    #[cfg(feature = "shared")]
    pub EncryptionBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EncryptionBlt: usize,
    #[cfg(feature = "shared")]
    pub DecryptionBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT, *mut D3DENCRYPTED_BLOCK_INFO, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DecryptionBlt: usize,
    #[cfg(feature = "shared")]
    pub GetSurfacePitch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSurfacePitch: usize,
    #[cfg(feature = "shared")]
    pub StartSessionKeyRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    StartSessionKeyRefresh: usize,
    pub FinishSessionKeyRefresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetEncryptionBltKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEncryptionBltKey: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3DCryptoSession9_Impl: windows_core::IUnknownImpl {
    fn GetCertificateSize(&self, pcertificatesize: *mut super::UINT) -> windows_core::Result<()>;
    fn GetCertificate(&self, certifactesize: super::UINT, ppcertificate: *mut super::BYTE) -> windows_core::Result<()>;
    fn NegotiateKeyExchange(&self, datasize: super::UINT, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EncryptionBlt(&self, psrcsurface: Option<IDirect3DSurface9>, pdstsurface: Option<IDirect3DSurface9>, dstsurfacesize: super::UINT, piv: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DecryptionBlt(&self, psrcsurface: Option<IDirect3DSurface9>, pdstsurface: Option<IDirect3DSurface9>, srcsurfacesize: super::UINT, pencryptedblockinfo: *mut D3DENCRYPTED_BLOCK_INFO, pcontentkey: *mut core::ffi::c_void, piv: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSurfacePitch(&self, psrcsurface: Option<IDirect3DSurface9>, psurfacepitch: *mut super::UINT) -> windows_core::Result<()>;
    fn StartSessionKeyRefresh(&self, prandomnumber: *mut core::ffi::c_void, randomnumbersize: super::UINT) -> windows_core::Result<()>;
    fn FinishSessionKeyRefresh(&self) -> windows_core::Result<()>;
    fn GetEncryptionBltKey(&self, preadbackkey: *mut core::ffi::c_void, keysize: super::UINT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3DCryptoSession9_Vtbl {
    pub const fn new<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCertificateSize<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertificatesize: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::GetCertificateSize(this, core::mem::transmute_copy(&pcertificatesize)).into()
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certifactesize: super::UINT, ppcertificate: *mut super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::GetCertificate(this, core::mem::transmute_copy(&certifactesize), core::mem::transmute_copy(&ppcertificate)).into()
            }
        }
        unsafe extern "system" fn NegotiateKeyExchange<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datasize: super::UINT, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::NegotiateKeyExchange(this, core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn EncryptionBlt<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, pdstsurface: *mut core::ffi::c_void, dstsurfacesize: super::UINT, piv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::EncryptionBlt(this, core::mem::transmute(&psrcsurface), core::mem::transmute(&pdstsurface), core::mem::transmute_copy(&dstsurfacesize), core::mem::transmute_copy(&piv)).into()
            }
        }
        unsafe extern "system" fn DecryptionBlt<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, pdstsurface: *mut core::ffi::c_void, srcsurfacesize: super::UINT, pencryptedblockinfo: *mut D3DENCRYPTED_BLOCK_INFO, pcontentkey: *mut core::ffi::c_void, piv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::DecryptionBlt(this, core::mem::transmute(&psrcsurface), core::mem::transmute(&pdstsurface), core::mem::transmute_copy(&srcsurfacesize), core::mem::transmute_copy(&pencryptedblockinfo), core::mem::transmute_copy(&pcontentkey), core::mem::transmute_copy(&piv)).into()
            }
        }
        unsafe extern "system" fn GetSurfacePitch<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, psurfacepitch: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::GetSurfacePitch(this, core::mem::transmute(&psrcsurface), core::mem::transmute_copy(&psurfacepitch)).into()
            }
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomnumber: *mut core::ffi::c_void, randomnumbersize: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::StartSessionKeyRefresh(this, core::mem::transmute_copy(&prandomnumber), core::mem::transmute_copy(&randomnumbersize)).into()
            }
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::FinishSessionKeyRefresh(this).into()
            }
        }
        unsafe extern "system" fn GetEncryptionBltKey<Identity: IDirect3DCryptoSession9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preadbackkey: *mut core::ffi::c_void, keysize: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCryptoSession9_Impl::GetEncryptionBltKey(this, core::mem::transmute_copy(&preadbackkey), core::mem::transmute_copy(&keysize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCertificateSize: GetCertificateSize::<Identity, OFFSET>,
            GetCertificate: GetCertificate::<Identity, OFFSET>,
            NegotiateKeyExchange: NegotiateKeyExchange::<Identity, OFFSET>,
            EncryptionBlt: EncryptionBlt::<Identity, OFFSET>,
            DecryptionBlt: DecryptionBlt::<Identity, OFFSET>,
            GetSurfacePitch: GetSurfacePitch::<Identity, OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Identity, OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Identity, OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DCryptoSession9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DCryptoSession9 {}
windows_core::imp::define_interface!(IDirect3DCubeTexture9, IDirect3DCubeTexture9_Vtbl, 0xfff32f81_d953_473a_9223_93d652aba93f);
impl core::ops::Deref for IDirect3DCubeTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DCubeTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DCubeTexture9 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLevelDesc(&self, level: super::UINT, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCubeMapSurface(&self, facetype: D3DCUBEMAP_FACES, level: super::UINT, ppcubemapsurface: *mut Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCubeMapSurface)(windows_core::Interface::as_raw(self), facetype, level, ppcubemapsurface as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn LockRect(&self, facetype: D3DCUBEMAP_FACES, level: super::UINT, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), facetype, level, plockedrect as _, prect as _, flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UnlockRect(&self, facetype: D3DCUBEMAP_FACES, level: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self), facetype, level) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn AddDirtyRect(&self, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyRect)(windows_core::Interface::as_raw(self), facetype, pdirtyrect as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DCubeTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DSURFACE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLevelDesc: usize,
    #[cfg(feature = "shared")]
    pub GetCubeMapSurface: unsafe extern "system" fn(*mut core::ffi::c_void, D3DCUBEMAP_FACES, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCubeMapSurface: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, D3DCUBEMAP_FACES, super::UINT, *mut D3DLOCKED_RECT, *const super::RECT, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    LockRect: usize,
    #[cfg(feature = "shared")]
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void, D3DCUBEMAP_FACES, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UnlockRect: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub AddDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, D3DCUBEMAP_FACES, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    AddDirtyRect: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IDirect3DCubeTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: super::UINT, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn GetCubeMapSurface(&self, facetype: D3DCUBEMAP_FACES, level: super::UINT, ppcubemapsurface: *mut Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn LockRect(&self, facetype: D3DCUBEMAP_FACES, level: super::UINT, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::Result<()>;
    fn UnlockRect(&self, facetype: D3DCUBEMAP_FACES, level: super::UINT) -> windows_core::Result<()>;
    fn AddDirtyRect(&self, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IDirect3DCubeTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetCubeMapSurface<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: super::UINT, ppcubemapsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetCubeMapSurface(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level), core::mem::transmute_copy(&ppcubemapsurface)).into()
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: super::UINT, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::LockRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::UnlockRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::AddDirtyRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetCubeMapSurface: GetCubeMapSurface::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DCubeTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IDirect3DCubeTexture9 {}
windows_core::imp::define_interface!(IDirect3DDevice9, IDirect3DDevice9_Vtbl, 0xd0223b96_bf7a_43fd_92bd_a43b0d82b9eb);
windows_core::imp::interface_hierarchy!(IDirect3DDevice9, windows_core::IUnknown);
impl IDirect3DDevice9 {
    pub unsafe fn TestCooperativeLevel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestCooperativeLevel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAvailableTextureMem(&self) -> super::UINT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableTextureMem)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EvictManagedResources(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EvictManagedResources)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDirect3D(&self, ppd3d9: *mut Option<IDirect3D9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDirect3D)(windows_core::Interface::as_raw(self), ppd3d9 as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceCaps)(windows_core::Interface::as_raw(self), pcaps as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDisplayMode(&self, iswapchain: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), iswapchain, pmode as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCreationParameters)(windows_core::Interface::as_raw(self), pparameters as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCursorProperties(&self, xhotspot: super::UINT, yhotspot: super::UINT, pcursorbitmap: &Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCursorProperties)(windows_core::Interface::as_raw(self), xhotspot, yhotspot, core::mem::transmute_copy(pcursorbitmap)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCursorPosition(&self, x: i32, y: i32, flags: super::DWORD) {
        unsafe {
            (windows_core::Interface::vtable(self).SetCursorPosition)(windows_core::Interface::as_raw(self), x, y, flags);
        }
    }
    pub unsafe fn ShowCursor(&self, bshow: windows_core::BOOL) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).ShowCursor)(windows_core::Interface::as_raw(self), bshow) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut Option<IDirect3DSwapChain9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateAdditionalSwapChain)(windows_core::Interface::as_raw(self), ppresentationparameters as _, pswapchain as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSwapChain(&self, iswapchain: super::UINT, pswapchain: *mut Option<IDirect3DSwapChain9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSwapChain)(windows_core::Interface::as_raw(self), iswapchain, pswapchain as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetNumberOfSwapChains(&self) -> super::UINT {
        unsafe { (windows_core::Interface::vtable(self).GetNumberOfSwapChains)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), ppresentationparameters as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
    pub unsafe fn Present(&self, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), psourcerect as _, pdestrect as _, hdestwindowoverride as _, pdirtyregion as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBackBuffer(&self, iswapchain: super::UINT, ibackbuffer: super::UINT, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBackBuffer)(windows_core::Interface::as_raw(self), iswapchain, ibackbuffer, r#type, ppbackbuffer as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRasterStatus(&self, iswapchain: super::UINT, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRasterStatus)(windows_core::Interface::as_raw(self), iswapchain, prasterstatus as _) }
    }
    pub unsafe fn SetDialogBoxMode(&self, benabledialogs: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDialogBoxMode)(windows_core::Interface::as_raw(self), benabledialogs) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetGammaRamp(&self, iswapchain: super::UINT, flags: super::DWORD, pramp: *const D3DGAMMARAMP) {
        unsafe {
            (windows_core::Interface::vtable(self).SetGammaRamp)(windows_core::Interface::as_raw(self), iswapchain, flags, pramp as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetGammaRamp(&self, iswapchain: super::UINT, pramp: *mut D3DGAMMARAMP) {
        unsafe {
            (windows_core::Interface::vtable(self).GetGammaRamp)(windows_core::Interface::as_raw(self), iswapchain, pramp as _);
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateTexture(&self, width: super::UINT, height: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut Option<IDirect3DTexture9>, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture)(windows_core::Interface::as_raw(self), width, height, levels, usage, format, pool, pptexture as _, psharedhandle as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateVolumeTexture(&self, width: super::UINT, height: super::UINT, depth: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVolumeTexture)(windows_core::Interface::as_raw(self), width, height, depth, levels, usage, format, pool, ppvolumetexture as _, psharedhandle as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateCubeTexture(&self, edgelength: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateCubeTexture)(windows_core::Interface::as_raw(self), edgelength, levels, usage, format, pool, ppcubetexture as _, psharedhandle as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateVertexBuffer(&self, length: super::UINT, usage: super::DWORD, fvf: super::DWORD, pool: D3DPOOL, ppvertexbuffer: *mut Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexBuffer)(windows_core::Interface::as_raw(self), length, usage, fvf, pool, ppvertexbuffer as _, psharedhandle as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateIndexBuffer(&self, length: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateIndexBuffer)(windows_core::Interface::as_raw(self), length, usage, format, pool, ppindexbuffer as _, psharedhandle as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateRenderTarget(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, lockable: windows_core::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTarget)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, lockable, ppsurface as _, psharedhandle as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateDepthStencilSurface(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, discard: windows_core::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilSurface)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, discard, ppsurface as _, psharedhandle as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn UpdateSurface(&self, psourcesurface: &Option<IDirect3DSurface9>, psourcerect: *const super::RECT, pdestinationsurface: &Option<IDirect3DSurface9>, pdestpoint: *const super::POINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateSurface)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psourcesurface), psourcerect as _, core::mem::transmute_copy(pdestinationsurface), pdestpoint as _) }
    }
    pub unsafe fn UpdateTexture(&self, psourcetexture: &Option<IDirect3DBaseTexture9>, pdestinationtexture: &Option<IDirect3DBaseTexture9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateTexture)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psourcetexture), core::mem::transmute_copy(pdestinationtexture)) }
    }
    pub unsafe fn GetRenderTargetData(&self, prendertarget: &Option<IDirect3DSurface9>, pdestsurface: &Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRenderTargetData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(prendertarget), core::mem::transmute_copy(pdestsurface)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFrontBufferData(&self, iswapchain: super::UINT, pdestsurface: &Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrontBufferData)(windows_core::Interface::as_raw(self), iswapchain, core::mem::transmute_copy(pdestsurface)) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn StretchRect(&self, psourcesurface: &Option<IDirect3DSurface9>, psourcerect: *const super::RECT, pdestsurface: &Option<IDirect3DSurface9>, pdestrect: *const super::RECT, filter: D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StretchRect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psourcesurface), psourcerect as _, core::mem::transmute_copy(pdestsurface), pdestrect as _, filter) }
    }
    #[cfg(all(feature = "dsound", feature = "shared", feature = "windef"))]
    pub unsafe fn ColorFill(&self, psurface: &Option<IDirect3DSurface9>, prect: *const super::RECT, color: super::D3DCOLOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ColorFill)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psurface), prect as _, color) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateOffscreenPlainSurface(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateOffscreenPlainSurface)(windows_core::Interface::as_raw(self), width, height, format, pool, ppsurface as _, psharedhandle as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRenderTarget(&self, rendertargetindex: super::DWORD, prendertarget: &Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTarget)(windows_core::Interface::as_raw(self), rendertargetindex, core::mem::transmute_copy(prendertarget)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRenderTarget(&self, rendertargetindex: super::DWORD, pprendertarget: *mut Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRenderTarget)(windows_core::Interface::as_raw(self), rendertargetindex, pprendertarget as _) }
    }
    pub unsafe fn SetDepthStencilSurface(&self, pnewzstencil: &Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDepthStencilSurface)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pnewzstencil)) }
    }
    pub unsafe fn GetDepthStencilSurface(&self, ppzstencilsurface: *mut Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDepthStencilSurface)(windows_core::Interface::as_raw(self), ppzstencilsurface as _) }
    }
    pub unsafe fn BeginScene(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginScene)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndScene(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndScene)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "dsound", feature = "shared"))]
    pub unsafe fn Clear(&self, count: super::DWORD, prects: *const D3DRECT, flags: super::DWORD, color: super::D3DCOLOR, z: f32, stencil: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), count, prects as _, flags, color, z, stencil) }
    }
    pub unsafe fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), state, pmatrix as _) }
    }
    pub unsafe fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), state, pmatrix as _) }
    }
    pub unsafe fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MultiplyTransform)(windows_core::Interface::as_raw(self), param0, param1 as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetViewport)(windows_core::Interface::as_raw(self), pviewport as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetViewport)(windows_core::Interface::as_raw(self), pviewport as _) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaterial)(windows_core::Interface::as_raw(self), pmaterial as _) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaterial)(windows_core::Interface::as_raw(self), pmaterial as _) }
    }
    #[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared"))]
    pub unsafe fn SetLight(&self, index: super::DWORD, param1: *const D3DLIGHT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLight)(windows_core::Interface::as_raw(self), index, param1 as _) }
    }
    #[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared"))]
    pub unsafe fn GetLight(&self, index: super::DWORD, param1: *mut D3DLIGHT9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLight)(windows_core::Interface::as_raw(self), index, param1 as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn LightEnable(&self, index: super::DWORD, enable: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LightEnable)(windows_core::Interface::as_raw(self), index, enable) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLightEnable(&self, index: super::DWORD, penable: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLightEnable)(windows_core::Interface::as_raw(self), index, penable as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetClipPlane(&self, index: super::DWORD, pplane: *const f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipPlane)(windows_core::Interface::as_raw(self), index, pplane as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetClipPlane(&self, index: super::DWORD, pplane: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClipPlane)(windows_core::Interface::as_raw(self), index, pplane as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderState)(windows_core::Interface::as_raw(self), state, value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRenderState)(windows_core::Interface::as_raw(self), state, pvalue as _) }
    }
    pub unsafe fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut Option<IDirect3DStateBlock9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateStateBlock)(windows_core::Interface::as_raw(self), r#type, ppsb as _) }
    }
    pub unsafe fn BeginStateBlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginStateBlock)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndStateBlock(&self, ppsb: *mut Option<IDirect3DStateBlock9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndStateBlock)(windows_core::Interface::as_raw(self), ppsb as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipStatus)(windows_core::Interface::as_raw(self), pclipstatus as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClipStatus)(windows_core::Interface::as_raw(self), pclipstatus as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTexture(&self, stage: super::DWORD, pptexture: *mut Option<IDirect3DBaseTexture9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTexture)(windows_core::Interface::as_raw(self), stage, pptexture as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetTexture(&self, stage: super::DWORD, ptexture: &Option<IDirect3DBaseTexture9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTexture)(windows_core::Interface::as_raw(self), stage, core::mem::transmute_copy(ptexture)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTextureStageState(&self, stage: super::DWORD, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextureStageState)(windows_core::Interface::as_raw(self), stage, r#type, pvalue as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetTextureStageState(&self, stage: super::DWORD, r#type: D3DTEXTURESTAGESTATETYPE, value: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextureStageState)(windows_core::Interface::as_raw(self), stage, r#type, value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSamplerState(&self, sampler: super::DWORD, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSamplerState)(windows_core::Interface::as_raw(self), sampler, r#type, pvalue as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSamplerState(&self, sampler: super::DWORD, r#type: D3DSAMPLERSTATETYPE, value: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSamplerState)(windows_core::Interface::as_raw(self), sampler, r#type, value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ValidateDevice(&self, pnumpasses: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ValidateDevice)(windows_core::Interface::as_raw(self), pnumpasses as _) }
    }
    #[cfg(all(feature = "shared", feature = "wingdi"))]
    pub unsafe fn SetPaletteEntries(&self, palettenumber: super::UINT, pentries: *const super::PALETTEENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPaletteEntries)(windows_core::Interface::as_raw(self), palettenumber, pentries as _) }
    }
    #[cfg(all(feature = "shared", feature = "wingdi"))]
    pub unsafe fn GetPaletteEntries(&self, palettenumber: super::UINT, pentries: *mut super::PALETTEENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPaletteEntries)(windows_core::Interface::as_raw(self), palettenumber, pentries as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCurrentTexturePalette(&self, palettenumber: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTexturePalette)(windows_core::Interface::as_raw(self), palettenumber) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCurrentTexturePalette(&self, palettenumber: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentTexturePalette)(windows_core::Interface::as_raw(self), palettenumber as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn SetScissorRect(&self, prect: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScissorRect)(windows_core::Interface::as_raw(self), prect as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetScissorRect(&self, prect: *mut super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScissorRect)(windows_core::Interface::as_raw(self), prect as _) }
    }
    pub unsafe fn SetSoftwareVertexProcessing(&self, bsoftware: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSoftwareVertexProcessing)(windows_core::Interface::as_raw(self), bsoftware) }
    }
    pub unsafe fn GetSoftwareVertexProcessing(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetSoftwareVertexProcessing)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetNPatchMode(&self, nsegments: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNPatchMode)(windows_core::Interface::as_raw(self), nsegments) }
    }
    pub unsafe fn GetNPatchMode(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetNPatchMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: super::UINT, primitivecount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawPrimitive)(windows_core::Interface::as_raw(self), primitivetype, startvertex, primitivecount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: super::INT, minvertexindex: super::UINT, numvertices: super::UINT, startindex: super::UINT, primcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedPrimitive)(windows_core::Interface::as_raw(self), param0, basevertexindex, minvertexindex, numvertices, startindex, primcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: super::UINT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawPrimitiveUP)(windows_core::Interface::as_raw(self), primitivetype, primitivecount, pvertexstreamzerodata as _, vertexstreamzerostride) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: super::UINT, numvertices: super::UINT, primitivecount: super::UINT, pindexdata: *const core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedPrimitiveUP)(windows_core::Interface::as_raw(self), primitivetype, minvertexindex, numvertices, primitivecount, pindexdata as _, indexdataformat, pvertexstreamzerodata as _, vertexstreamzerostride) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ProcessVertices(&self, srcstartindex: super::UINT, destindex: super::UINT, vertexcount: super::UINT, pdestbuffer: &Option<IDirect3DVertexBuffer9>, pvertexdecl: &Option<IDirect3DVertexDeclaration9>, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessVertices)(windows_core::Interface::as_raw(self), srcstartindex, destindex, vertexcount, core::mem::transmute_copy(pdestbuffer), core::mem::transmute_copy(pvertexdecl), flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut Option<IDirect3DVertexDeclaration9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexDeclaration)(windows_core::Interface::as_raw(self), pvertexelements as _, ppdecl as _) }
    }
    pub unsafe fn SetVertexDeclaration(&self, pdecl: &Option<IDirect3DVertexDeclaration9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexDeclaration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pdecl)) }
    }
    pub unsafe fn GetVertexDeclaration(&self, ppdecl: *mut Option<IDirect3DVertexDeclaration9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexDeclaration)(windows_core::Interface::as_raw(self), ppdecl as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFVF(&self, fvf: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFVF)(windows_core::Interface::as_raw(self), fvf) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFVF(&self, pfvf: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFVF)(windows_core::Interface::as_raw(self), pfvf as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateVertexShader(&self, pfunction: *const super::DWORD, ppshader: *mut Option<IDirect3DVertexShader9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), pfunction as _, ppshader as _) }
    }
    pub unsafe fn SetVertexShader(&self, pshader: &Option<IDirect3DVertexShader9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pshader)) }
    }
    pub unsafe fn GetVertexShader(&self, ppshader: *mut Option<IDirect3DVertexShader9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShader)(windows_core::Interface::as_raw(self), ppshader as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVertexShaderConstantF(&self, startregister: super::UINT, pconstantdata: *const f32, vector4fcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetVertexShaderConstantF(&self, startregister: super::UINT, pconstantdata: *mut f32, vector4fcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVertexShaderConstantI(&self, startregister: super::UINT, pconstantdata: *const i32, vector4icount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetVertexShaderConstantI(&self, startregister: super::UINT, pconstantdata: *mut i32, vector4icount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVertexShaderConstantB(&self, startregister: super::UINT, pconstantdata: *const windows_core::BOOL, boolcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetVertexShaderConstantB(&self, startregister: super::UINT, pconstantdata: *mut windows_core::BOOL, boolcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetStreamSource(&self, streamnumber: super::UINT, pstreamdata: &Option<IDirect3DVertexBuffer9>, offsetinbytes: super::UINT, stride: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSource)(windows_core::Interface::as_raw(self), streamnumber, core::mem::transmute_copy(pstreamdata), offsetinbytes, stride) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamSource(&self, streamnumber: super::UINT, ppstreamdata: *mut Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut super::UINT, pstride: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamSource)(windows_core::Interface::as_raw(self), streamnumber, ppstreamdata as _, poffsetinbytes as _, pstride as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetStreamSourceFreq(&self, streamnumber: super::UINT, setting: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSourceFreq)(windows_core::Interface::as_raw(self), streamnumber, setting) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamSourceFreq(&self, streamnumber: super::UINT, psetting: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamSourceFreq)(windows_core::Interface::as_raw(self), streamnumber, psetting as _) }
    }
    pub unsafe fn SetIndices(&self, pindexdata: &Option<IDirect3DIndexBuffer9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIndices)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pindexdata)) }
    }
    pub unsafe fn GetIndices(&self, ppindexdata: *mut Option<IDirect3DIndexBuffer9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIndices)(windows_core::Interface::as_raw(self), ppindexdata as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreatePixelShader(&self, pfunction: *const super::DWORD, ppshader: *mut Option<IDirect3DPixelShader9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), pfunction as _, ppshader as _) }
    }
    pub unsafe fn SetPixelShader(&self, pshader: &Option<IDirect3DPixelShader9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pshader)) }
    }
    pub unsafe fn GetPixelShader(&self, ppshader: *mut Option<IDirect3DPixelShader9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShader)(windows_core::Interface::as_raw(self), ppshader as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPixelShaderConstantF(&self, startregister: super::UINT, pconstantdata: *const f32, vector4fcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPixelShaderConstantF(&self, startregister: super::UINT, pconstantdata: *mut f32, vector4fcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPixelShaderConstantI(&self, startregister: super::UINT, pconstantdata: *const i32, vector4icount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPixelShaderConstantI(&self, startregister: super::UINT, pconstantdata: *mut i32, vector4icount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPixelShaderConstantB(&self, startregister: super::UINT, pconstantdata: *const windows_core::BOOL, boolcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPixelShaderConstantB(&self, startregister: super::UINT, pconstantdata: *mut windows_core::BOOL, boolcount: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DrawRectPatch(&self, handle: super::UINT, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawRectPatch)(windows_core::Interface::as_raw(self), handle, pnumsegs as _, prectpatchinfo as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DrawTriPatch(&self, handle: super::UINT, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawTriPatch)(windows_core::Interface::as_raw(self), handle, pnumsegs as _, ptripatchinfo as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeletePatch(&self, handle: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeletePatch)(windows_core::Interface::as_raw(self), handle) }
    }
    pub unsafe fn CreateQuery(&self, r#type: D3DQUERYTYPE, ppquery: *mut Option<IDirect3DQuery9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), r#type, ppquery as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TestCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetAvailableTextureMem: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::UINT,
    #[cfg(not(feature = "shared"))]
    GetAvailableTextureMem: usize,
    pub EvictManagedResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDirect3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetDeviceCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DCAPS9) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDeviceCaps: usize,
    #[cfg(feature = "shared")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDisplayMode: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetCreationParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetCreationParameters: usize,
    #[cfg(feature = "shared")]
    pub SetCursorProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetCursorProperties: usize,
    #[cfg(feature = "shared")]
    pub SetCursorPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::DWORD),
    #[cfg(not(feature = "shared"))]
    SetCursorPosition: usize,
    pub ShowCursor: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub CreateAdditionalSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENT_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    CreateAdditionalSwapChain: usize,
    #[cfg(feature = "shared")]
    pub GetSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSwapChain: usize,
    #[cfg(feature = "shared")]
    pub GetNumberOfSwapChains: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::UINT,
    #[cfg(not(feature = "shared"))]
    GetNumberOfSwapChains: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    Reset: usize,
    #[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT, *const super::RECT, super::HWND, *const super::RGNDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef", feature = "wingdi")))]
    Present: usize,
    #[cfg(feature = "shared")]
    pub GetBackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, D3DBACKBUFFER_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBackBuffer: usize,
    #[cfg(feature = "shared")]
    pub GetRasterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DRASTER_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRasterStatus: usize,
    pub SetDialogBoxMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::DWORD, *const D3DGAMMARAMP),
    #[cfg(not(feature = "shared"))]
    SetGammaRamp: usize,
    #[cfg(feature = "shared")]
    pub GetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DGAMMARAMP),
    #[cfg(not(feature = "shared"))]
    GetGammaRamp: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateTexture: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, super::UINT, super::DWORD, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateTexture: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateVolumeTexture: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, super::UINT, super::UINT, super::DWORD, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateVolumeTexture: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateCubeTexture: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, super::DWORD, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateCubeTexture: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateVertexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::DWORD, super::DWORD, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateVertexBuffer: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::DWORD, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateIndexBuffer: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, D3DFORMAT, D3DMULTISAMPLE_TYPE, super::DWORD, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateRenderTarget: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, D3DFORMAT, D3DMULTISAMPLE_TYPE, super::DWORD, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateDepthStencilSurface: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub UpdateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::RECT, *mut core::ffi::c_void, *const super::POINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    UpdateSurface: usize,
    pub UpdateTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTargetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFrontBufferData: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFrontBufferData: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub StretchRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::RECT, *mut core::ffi::c_void, *const super::RECT, D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    StretchRect: usize,
    #[cfg(all(feature = "dsound", feature = "shared", feature = "windef"))]
    pub ColorFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::RECT, super::D3DCOLOR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "dsound", feature = "shared", feature = "windef")))]
    ColorFill: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateOffscreenPlainSurface: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateOffscreenPlainSurface: usize,
    #[cfg(feature = "shared")]
    pub SetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRenderTarget: usize,
    #[cfg(feature = "shared")]
    pub GetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRenderTarget: usize,
    pub SetDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginScene: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndScene: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "dsound", feature = "shared"))]
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const D3DRECT, super::DWORD, super::D3DCOLOR, f32, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "dsound", feature = "shared")))]
    Clear: usize,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, D3DTRANSFORMSTATETYPE, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, D3DTRANSFORMSTATETYPE, *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub MultiplyTransform: unsafe extern "system" fn(*mut core::ffi::c_void, D3DTRANSFORMSTATETYPE, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DVIEWPORT9) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetViewport: usize,
    #[cfg(feature = "shared")]
    pub GetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVIEWPORT9) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetViewport: usize,
    #[cfg(feature = "dxgi")]
    pub SetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DMATERIAL9) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    SetMaterial: usize,
    #[cfg(feature = "dxgi")]
    pub GetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DMATERIAL9) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetMaterial: usize,
    #[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared"))]
    pub SetLight: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const D3DLIGHT9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "dsound", feature = "dxgi", feature = "shared")))]
    SetLight: usize,
    #[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared"))]
    pub GetLight: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut D3DLIGHT9) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "dsound", feature = "dxgi", feature = "shared")))]
    GetLight: usize,
    #[cfg(feature = "shared")]
    pub LightEnable: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LightEnable: usize,
    #[cfg(feature = "shared")]
    pub GetLightEnable: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLightEnable: usize,
    #[cfg(feature = "shared")]
    pub SetClipPlane: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetClipPlane: usize,
    #[cfg(feature = "shared")]
    pub GetClipPlane: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetClipPlane: usize,
    #[cfg(feature = "shared")]
    pub SetRenderState: unsafe extern "system" fn(*mut core::ffi::c_void, D3DRENDERSTATETYPE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRenderState: usize,
    #[cfg(feature = "shared")]
    pub GetRenderState: unsafe extern "system" fn(*mut core::ffi::c_void, D3DRENDERSTATETYPE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRenderState: usize,
    pub CreateStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, D3DSTATEBLOCKTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetClipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DCLIPSTATUS9) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetClipStatus: usize,
    #[cfg(feature = "shared")]
    pub GetClipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DCLIPSTATUS9) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetClipStatus: usize,
    #[cfg(feature = "shared")]
    pub GetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTexture: usize,
    #[cfg(feature = "shared")]
    pub SetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetTexture: usize,
    #[cfg(feature = "shared")]
    pub GetTextureStageState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, D3DTEXTURESTAGESTATETYPE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTextureStageState: usize,
    #[cfg(feature = "shared")]
    pub SetTextureStageState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, D3DTEXTURESTAGESTATETYPE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetTextureStageState: usize,
    #[cfg(feature = "shared")]
    pub GetSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, D3DSAMPLERSTATETYPE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSamplerState: usize,
    #[cfg(feature = "shared")]
    pub SetSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, D3DSAMPLERSTATETYPE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSamplerState: usize,
    #[cfg(feature = "shared")]
    pub ValidateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ValidateDevice: usize,
    #[cfg(all(feature = "shared", feature = "wingdi"))]
    pub SetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const super::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wingdi")))]
    SetPaletteEntries: usize,
    #[cfg(all(feature = "shared", feature = "wingdi"))]
    pub GetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wingdi")))]
    GetPaletteEntries: usize,
    #[cfg(feature = "shared")]
    pub SetCurrentTexturePalette: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetCurrentTexturePalette: usize,
    #[cfg(feature = "shared")]
    pub GetCurrentTexturePalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCurrentTexturePalette: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub SetScissorRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    SetScissorRect: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetScissorRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetScissorRect: usize,
    pub SetSoftwareVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSoftwareVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetNPatchMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetNPatchMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    #[cfg(feature = "shared")]
    pub DrawPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, D3DPRIMITIVETYPE, super::UINT, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DrawPrimitive: usize,
    #[cfg(feature = "shared")]
    pub DrawIndexedPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, D3DPRIMITIVETYPE, super::INT, super::UINT, super::UINT, super::UINT, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DrawIndexedPrimitive: usize,
    #[cfg(feature = "shared")]
    pub DrawPrimitiveUP: unsafe extern "system" fn(*mut core::ffi::c_void, D3DPRIMITIVETYPE, super::UINT, *const core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DrawPrimitiveUP: usize,
    #[cfg(feature = "shared")]
    pub DrawIndexedPrimitiveUP: unsafe extern "system" fn(*mut core::ffi::c_void, D3DPRIMITIVETYPE, super::UINT, super::UINT, super::UINT, *const core::ffi::c_void, D3DFORMAT, *const core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DrawIndexedPrimitiveUP: usize,
    #[cfg(feature = "shared")]
    pub ProcessVertices: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, super::UINT, *mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ProcessVertices: usize,
    #[cfg(feature = "shared")]
    pub CreateVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DVERTEXELEMENT9, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateVertexDeclaration: usize,
    pub SetVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetFVF: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFVF: usize,
    #[cfg(feature = "shared")]
    pub GetFVF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFVF: usize,
    #[cfg(feature = "shared")]
    pub CreateVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateVertexShader: usize,
    pub SetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetVertexShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const f32, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVertexShaderConstantF: usize,
    #[cfg(feature = "shared")]
    pub GetVertexShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut f32, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetVertexShaderConstantF: usize,
    #[cfg(feature = "shared")]
    pub SetVertexShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const i32, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVertexShaderConstantI: usize,
    #[cfg(feature = "shared")]
    pub GetVertexShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut i32, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetVertexShaderConstantI: usize,
    #[cfg(feature = "shared")]
    pub SetVertexShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const windows_core::BOOL, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVertexShaderConstantB: usize,
    #[cfg(feature = "shared")]
    pub GetVertexShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut windows_core::BOOL, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetVertexShaderConstantB: usize,
    #[cfg(feature = "shared")]
    pub SetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut core::ffi::c_void, super::UINT, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetStreamSource: usize,
    #[cfg(feature = "shared")]
    pub GetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void, *mut super::UINT, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamSource: usize,
    #[cfg(feature = "shared")]
    pub SetStreamSourceFreq: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetStreamSourceFreq: usize,
    #[cfg(feature = "shared")]
    pub GetStreamSourceFreq: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamSourceFreq: usize,
    pub SetIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub CreatePixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreatePixelShader: usize,
    pub SetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetPixelShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const f32, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPixelShaderConstantF: usize,
    #[cfg(feature = "shared")]
    pub GetPixelShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut f32, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPixelShaderConstantF: usize,
    #[cfg(feature = "shared")]
    pub SetPixelShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const i32, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPixelShaderConstantI: usize,
    #[cfg(feature = "shared")]
    pub GetPixelShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut i32, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPixelShaderConstantI: usize,
    #[cfg(feature = "shared")]
    pub SetPixelShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const windows_core::BOOL, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPixelShaderConstantB: usize,
    #[cfg(feature = "shared")]
    pub GetPixelShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut windows_core::BOOL, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPixelShaderConstantB: usize,
    #[cfg(feature = "shared")]
    pub DrawRectPatch: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const f32, *const D3DRECTPATCH_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DrawRectPatch: usize,
    #[cfg(feature = "shared")]
    pub DrawTriPatch: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const f32, *const D3DTRIPATCH_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DrawTriPatch: usize,
    #[cfg(feature = "shared")]
    pub DeletePatch: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeletePatch: usize,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, D3DQUERYTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub trait IDirect3DDevice9_Impl: windows_core::IUnknownImpl {
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetAvailableTextureMem(&self) -> super::UINT;
    fn EvictManagedResources(&self) -> windows_core::Result<()>;
    fn GetDirect3D(&self, ppd3d9: *mut Option<IDirect3D9>) -> windows_core::Result<()>;
    fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, iswapchain: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::Result<()>;
    fn SetCursorProperties(&self, xhotspot: super::UINT, yhotspot: super::UINT, pcursorbitmap: Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn SetCursorPosition(&self, x: i32, y: i32, flags: super::DWORD);
    fn ShowCursor(&self, bshow: windows_core::BOOL) -> windows_core::BOOL;
    fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut Option<IDirect3DSwapChain9>) -> windows_core::Result<()>;
    fn GetSwapChain(&self, iswapchain: super::UINT, pswapchain: *mut Option<IDirect3DSwapChain9>) -> windows_core::Result<()>;
    fn GetNumberOfSwapChains(&self) -> super::UINT;
    fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
    fn Present(&self, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, iswapchain: super::UINT, ibackbuffer: super::UINT, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetRasterStatus(&self, iswapchain: super::UINT, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::Result<()>;
    fn SetDialogBoxMode(&self, benabledialogs: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetGammaRamp(&self, iswapchain: super::UINT, flags: super::DWORD, pramp: *const D3DGAMMARAMP);
    fn GetGammaRamp(&self, iswapchain: super::UINT, pramp: *mut D3DGAMMARAMP);
    fn CreateTexture(&self, width: super::UINT, height: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut Option<IDirect3DTexture9>, psharedhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn CreateVolumeTexture(&self, width: super::UINT, height: super::UINT, depth: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn CreateCubeTexture(&self, edgelength: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn CreateVertexBuffer(&self, length: super::UINT, usage: super::DWORD, fvf: super::DWORD, pool: D3DPOOL, ppvertexbuffer: *mut Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn CreateIndexBuffer(&self, length: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn CreateRenderTarget(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, lockable: windows_core::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn CreateDepthStencilSurface(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, discard: windows_core::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn UpdateSurface(&self, psourcesurface: Option<IDirect3DSurface9>, psourcerect: *const super::RECT, pdestinationsurface: Option<IDirect3DSurface9>, pdestpoint: *const super::POINT) -> windows_core::Result<()>;
    fn UpdateTexture(&self, psourcetexture: Option<IDirect3DBaseTexture9>, pdestinationtexture: Option<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetRenderTargetData(&self, prendertarget: Option<IDirect3DSurface9>, pdestsurface: Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, iswapchain: super::UINT, pdestsurface: Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn StretchRect(&self, psourcesurface: Option<IDirect3DSurface9>, psourcerect: *const super::RECT, pdestsurface: Option<IDirect3DSurface9>, pdestrect: *const super::RECT, filter: D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn ColorFill(&self, psurface: Option<IDirect3DSurface9>, prect: *const super::RECT, color: super::D3DCOLOR) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurface(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn SetRenderTarget(&self, rendertargetindex: super::DWORD, prendertarget: Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetRenderTarget(&self, rendertargetindex: super::DWORD, pprendertarget: *mut Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn SetDepthStencilSurface(&self, pnewzstencil: Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetDepthStencilSurface(&self, ppzstencilsurface: *mut Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn BeginScene(&self) -> windows_core::Result<()>;
    fn EndScene(&self) -> windows_core::Result<()>;
    fn Clear(&self, count: super::DWORD, prects: *const D3DRECT, flags: super::DWORD, color: super::D3DCOLOR, z: f32, stencil: super::DWORD) -> windows_core::Result<()>;
    fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> windows_core::Result<()>;
    fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> windows_core::Result<()>;
    fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> windows_core::Result<()>;
    fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> windows_core::Result<()>;
    fn SetLight(&self, index: super::DWORD, param1: *const D3DLIGHT9) -> windows_core::Result<()>;
    fn GetLight(&self, index: super::DWORD, param1: *mut D3DLIGHT9) -> windows_core::Result<()>;
    fn LightEnable(&self, index: super::DWORD, enable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLightEnable(&self, index: super::DWORD, penable: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetClipPlane(&self, index: super::DWORD, pplane: *const f32) -> windows_core::Result<()>;
    fn GetClipPlane(&self, index: super::DWORD, pplane: *mut f32) -> windows_core::Result<()>;
    fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: super::DWORD) -> windows_core::Result<()>;
    fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut super::DWORD) -> windows_core::Result<()>;
    fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut Option<IDirect3DStateBlock9>) -> windows_core::Result<()>;
    fn BeginStateBlock(&self) -> windows_core::Result<()>;
    fn EndStateBlock(&self, ppsb: *mut Option<IDirect3DStateBlock9>) -> windows_core::Result<()>;
    fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> windows_core::Result<()>;
    fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> windows_core::Result<()>;
    fn GetTexture(&self, stage: super::DWORD, pptexture: *mut Option<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn SetTexture(&self, stage: super::DWORD, ptexture: Option<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetTextureStageState(&self, stage: super::DWORD, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetTextureStageState(&self, stage: super::DWORD, r#type: D3DTEXTURESTAGESTATETYPE, value: super::DWORD) -> windows_core::Result<()>;
    fn GetSamplerState(&self, sampler: super::DWORD, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetSamplerState(&self, sampler: super::DWORD, r#type: D3DSAMPLERSTATETYPE, value: super::DWORD) -> windows_core::Result<()>;
    fn ValidateDevice(&self, pnumpasses: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetPaletteEntries(&self, palettenumber: super::UINT, pentries: *const super::PALETTEENTRY) -> windows_core::Result<()>;
    fn GetPaletteEntries(&self, palettenumber: super::UINT, pentries: *mut super::PALETTEENTRY) -> windows_core::Result<()>;
    fn SetCurrentTexturePalette(&self, palettenumber: super::UINT) -> windows_core::Result<()>;
    fn GetCurrentTexturePalette(&self, palettenumber: *mut super::UINT) -> windows_core::Result<()>;
    fn SetScissorRect(&self, prect: *const super::RECT) -> windows_core::Result<()>;
    fn GetScissorRect(&self, prect: *mut super::RECT) -> windows_core::Result<()>;
    fn SetSoftwareVertexProcessing(&self, bsoftware: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSoftwareVertexProcessing(&self) -> windows_core::BOOL;
    fn SetNPatchMode(&self, nsegments: f32) -> windows_core::Result<()>;
    fn GetNPatchMode(&self) -> f32;
    fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: super::UINT, primitivecount: super::UINT) -> windows_core::Result<()>;
    fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: super::INT, minvertexindex: super::UINT, numvertices: super::UINT, startindex: super::UINT, primcount: super::UINT) -> windows_core::Result<()>;
    fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: super::UINT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: super::UINT) -> windows_core::Result<()>;
    fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: super::UINT, numvertices: super::UINT, primitivecount: super::UINT, pindexdata: *const core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: super::UINT) -> windows_core::Result<()>;
    fn ProcessVertices(&self, srcstartindex: super::UINT, destindex: super::UINT, vertexcount: super::UINT, pdestbuffer: Option<IDirect3DVertexBuffer9>, pvertexdecl: Option<IDirect3DVertexDeclaration9>, flags: super::DWORD) -> windows_core::Result<()>;
    fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut Option<IDirect3DVertexDeclaration9>) -> windows_core::Result<()>;
    fn SetVertexDeclaration(&self, pdecl: Option<IDirect3DVertexDeclaration9>) -> windows_core::Result<()>;
    fn GetVertexDeclaration(&self, ppdecl: *mut Option<IDirect3DVertexDeclaration9>) -> windows_core::Result<()>;
    fn SetFVF(&self, fvf: super::DWORD) -> windows_core::Result<()>;
    fn GetFVF(&self, pfvf: *mut super::DWORD) -> windows_core::Result<()>;
    fn CreateVertexShader(&self, pfunction: *const super::DWORD, ppshader: *mut Option<IDirect3DVertexShader9>) -> windows_core::Result<()>;
    fn SetVertexShader(&self, pshader: Option<IDirect3DVertexShader9>) -> windows_core::Result<()>;
    fn GetVertexShader(&self, ppshader: *mut Option<IDirect3DVertexShader9>) -> windows_core::Result<()>;
    fn SetVertexShaderConstantF(&self, startregister: super::UINT, pconstantdata: *const f32, vector4fcount: super::UINT) -> windows_core::Result<()>;
    fn GetVertexShaderConstantF(&self, startregister: super::UINT, pconstantdata: *mut f32, vector4fcount: super::UINT) -> windows_core::Result<()>;
    fn SetVertexShaderConstantI(&self, startregister: super::UINT, pconstantdata: *const i32, vector4icount: super::UINT) -> windows_core::Result<()>;
    fn GetVertexShaderConstantI(&self, startregister: super::UINT, pconstantdata: *mut i32, vector4icount: super::UINT) -> windows_core::Result<()>;
    fn SetVertexShaderConstantB(&self, startregister: super::UINT, pconstantdata: *const windows_core::BOOL, boolcount: super::UINT) -> windows_core::Result<()>;
    fn GetVertexShaderConstantB(&self, startregister: super::UINT, pconstantdata: *mut windows_core::BOOL, boolcount: super::UINT) -> windows_core::Result<()>;
    fn SetStreamSource(&self, streamnumber: super::UINT, pstreamdata: Option<IDirect3DVertexBuffer9>, offsetinbytes: super::UINT, stride: super::UINT) -> windows_core::Result<()>;
    fn GetStreamSource(&self, streamnumber: super::UINT, ppstreamdata: *mut Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut super::UINT, pstride: *mut super::UINT) -> windows_core::Result<()>;
    fn SetStreamSourceFreq(&self, streamnumber: super::UINT, setting: super::UINT) -> windows_core::Result<()>;
    fn GetStreamSourceFreq(&self, streamnumber: super::UINT, psetting: *mut super::UINT) -> windows_core::Result<()>;
    fn SetIndices(&self, pindexdata: Option<IDirect3DIndexBuffer9>) -> windows_core::Result<()>;
    fn GetIndices(&self, ppindexdata: *mut Option<IDirect3DIndexBuffer9>) -> windows_core::Result<()>;
    fn CreatePixelShader(&self, pfunction: *const super::DWORD, ppshader: *mut Option<IDirect3DPixelShader9>) -> windows_core::Result<()>;
    fn SetPixelShader(&self, pshader: Option<IDirect3DPixelShader9>) -> windows_core::Result<()>;
    fn GetPixelShader(&self, ppshader: *mut Option<IDirect3DPixelShader9>) -> windows_core::Result<()>;
    fn SetPixelShaderConstantF(&self, startregister: super::UINT, pconstantdata: *const f32, vector4fcount: super::UINT) -> windows_core::Result<()>;
    fn GetPixelShaderConstantF(&self, startregister: super::UINT, pconstantdata: *mut f32, vector4fcount: super::UINT) -> windows_core::Result<()>;
    fn SetPixelShaderConstantI(&self, startregister: super::UINT, pconstantdata: *const i32, vector4icount: super::UINT) -> windows_core::Result<()>;
    fn GetPixelShaderConstantI(&self, startregister: super::UINT, pconstantdata: *mut i32, vector4icount: super::UINT) -> windows_core::Result<()>;
    fn SetPixelShaderConstantB(&self, startregister: super::UINT, pconstantdata: *const windows_core::BOOL, boolcount: super::UINT) -> windows_core::Result<()>;
    fn GetPixelShaderConstantB(&self, startregister: super::UINT, pconstantdata: *mut windows_core::BOOL, boolcount: super::UINT) -> windows_core::Result<()>;
    fn DrawRectPatch(&self, handle: super::UINT, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> windows_core::Result<()>;
    fn DrawTriPatch(&self, handle: super::UINT, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> windows_core::Result<()>;
    fn DeletePatch(&self, handle: super::UINT) -> windows_core::Result<()>;
    fn CreateQuery(&self, r#type: D3DQUERYTYPE, ppquery: *mut Option<IDirect3DQuery9>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl IDirect3DDevice9_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TestCooperativeLevel<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::TestCooperativeLevel(this).into()
            }
        }
        unsafe extern "system" fn GetAvailableTextureMem<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::UINT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetAvailableTextureMem(this)
            }
        }
        unsafe extern "system" fn EvictManagedResources<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::EvictManagedResources(this).into()
            }
        }
        unsafe extern "system" fn GetDirect3D<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppd3d9: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetDirect3D(this, core::mem::transmute_copy(&ppd3d9)).into()
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut D3DCAPS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetDisplayMode(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetCreationParameters<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetCreationParameters(this, core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn SetCursorProperties<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xhotspot: super::UINT, yhotspot: super::UINT, pcursorbitmap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCursorProperties(this, core::mem::transmute_copy(&xhotspot), core::mem::transmute_copy(&yhotspot), core::mem::transmute(&pcursorbitmap)).into()
            }
        }
        unsafe extern "system" fn SetCursorPosition<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, flags: super::DWORD) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCursorPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&flags));
            }
        }
        unsafe extern "system" fn ShowCursor<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshow: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ShowCursor(this, core::mem::transmute_copy(&bshow))
            }
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateAdditionalSwapChain(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pswapchain)).into()
            }
        }
        unsafe extern "system" fn GetSwapChain<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetSwapChain(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pswapchain)).into()
            }
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::UINT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetNumberOfSwapChains(this)
            }
        }
        unsafe extern "system" fn Reset<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Reset(this, core::mem::transmute_copy(&ppresentationparameters)).into()
            }
        }
        unsafe extern "system" fn Present<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion)).into()
            }
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT, ibackbuffer: super::UINT, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetBackBuffer(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&ppbackbuffer)).into()
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetRasterStatus(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&prasterstatus)).into()
            }
        }
        unsafe extern "system" fn SetDialogBoxMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabledialogs: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetDialogBoxMode(this, core::mem::transmute_copy(&benabledialogs)).into()
            }
        }
        unsafe extern "system" fn SetGammaRamp<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT, flags: super::DWORD, pramp: *const D3DGAMMARAMP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pramp));
            }
        }
        unsafe extern "system" fn GetGammaRamp<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT, pramp: *mut D3DGAMMARAMP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pramp));
            }
        }
        unsafe extern "system" fn CreateTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&pptexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateVolumeTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, depth: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateVolumeTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvolumetexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateCubeTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edgelength: super::UINT, levels: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateCubeTexture(this, core::mem::transmute_copy(&edgelength), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppcubetexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: super::UINT, usage: super::DWORD, fvf: super::DWORD, pool: D3DPOOL, ppvertexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateVertexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&fvf), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvertexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateIndexBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: super::UINT, usage: super::DWORD, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateIndexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppindexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, lockable: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateRenderTarget(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, discard: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateDepthStencilSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn UpdateSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::RECT, pdestinationsurface: *mut core::ffi::c_void, pdestpoint: *const super::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::UpdateSurface(this, core::mem::transmute(&psourcesurface), core::mem::transmute_copy(&psourcerect), core::mem::transmute(&pdestinationsurface), core::mem::transmute_copy(&pdestpoint)).into()
            }
        }
        unsafe extern "system" fn UpdateTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetexture: *mut core::ffi::c_void, pdestinationtexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::UpdateTexture(this, core::mem::transmute(&psourcetexture), core::mem::transmute(&pdestinationtexture)).into()
            }
        }
        unsafe extern "system" fn GetRenderTargetData<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetRenderTargetData(this, core::mem::transmute(&prendertarget), core::mem::transmute(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetFrontBufferData(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn StretchRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::RECT, pdestsurface: *mut core::ffi::c_void, pdestrect: *const super::RECT, filter: D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::StretchRect(this, core::mem::transmute(&psourcesurface), core::mem::transmute_copy(&psourcerect), core::mem::transmute(&pdestsurface), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&filter)).into()
            }
        }
        unsafe extern "system" fn ColorFill<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psurface: *mut core::ffi::c_void, prect: *const super::RECT, color: super::D3DCOLOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ColorFill(this, core::mem::transmute(&psurface), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateOffscreenPlainSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn SetRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: super::DWORD, prendertarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex), core::mem::transmute(&prendertarget)).into()
            }
        }
        unsafe extern "system" fn GetRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: super::DWORD, pprendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex), core::mem::transmute_copy(&pprendertarget)).into()
            }
        }
        unsafe extern "system" fn SetDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewzstencil: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetDepthStencilSurface(this, core::mem::transmute(&pnewzstencil)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppzstencilsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetDepthStencilSurface(this, core::mem::transmute_copy(&ppzstencilsurface)).into()
            }
        }
        unsafe extern "system" fn BeginScene<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::BeginScene(this).into()
            }
        }
        unsafe extern "system" fn EndScene<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::EndScene(this).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: super::DWORD, prects: *const D3DRECT, flags: super::DWORD, color: super::D3DCOLOR, z: f32, stencil: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Clear(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prects), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&color), core::mem::transmute_copy(&z), core::mem::transmute_copy(&stencil)).into()
            }
        }
        unsafe extern "system" fn SetTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn MultiplyTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::MultiplyTransform(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetViewport<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetViewport(this, core::mem::transmute_copy(&pviewport)).into()
            }
        }
        unsafe extern "system" fn GetViewport<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetViewport(this, core::mem::transmute_copy(&pviewport)).into()
            }
        }
        unsafe extern "system" fn SetMaterial<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
            }
        }
        unsafe extern "system" fn GetMaterial<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
            }
        }
        unsafe extern "system" fn SetLight<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::DWORD, param1: *const D3DLIGHT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetLight<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::DWORD, param1: *mut D3DLIGHT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn LightEnable<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::DWORD, enable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::LightEnable(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&enable)).into()
            }
        }
        unsafe extern "system" fn GetLightEnable<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::DWORD, penable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetLightEnable(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&penable)).into()
            }
        }
        unsafe extern "system" fn SetClipPlane<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::DWORD, pplane: *const f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetClipPlane(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pplane)).into()
            }
        }
        unsafe extern "system" fn GetClipPlane<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::DWORD, pplane: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetClipPlane(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pplane)).into()
            }
        }
        unsafe extern "system" fn SetRenderState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DRENDERSTATETYPE, value: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetRenderState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetRenderState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetRenderState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn CreateStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateStateBlock(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&ppsb)).into()
            }
        }
        unsafe extern "system" fn BeginStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::BeginStateBlock(this).into()
            }
        }
        unsafe extern "system" fn EndStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::EndStateBlock(this, core::mem::transmute_copy(&ppsb)).into()
            }
        }
        unsafe extern "system" fn SetClipStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetClipStatus(this, core::mem::transmute_copy(&pclipstatus)).into()
            }
        }
        unsafe extern "system" fn GetClipStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetClipStatus(this, core::mem::transmute_copy(&pclipstatus)).into()
            }
        }
        unsafe extern "system" fn GetTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: super::DWORD, pptexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetTexture(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&pptexture)).into()
            }
        }
        unsafe extern "system" fn SetTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: super::DWORD, ptexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTexture(this, core::mem::transmute_copy(&stage), core::mem::transmute(&ptexture)).into()
            }
        }
        unsafe extern "system" fn GetTextureStageState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: super::DWORD, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetTextureStageState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: super::DWORD, r#type: D3DTEXTURESTAGESTATETYPE, value: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSamplerState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: super::DWORD, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetSamplerState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: super::DWORD, r#type: D3DSAMPLERSTATETYPE, value: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ValidateDevice<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpasses: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ValidateDevice(this, core::mem::transmute_copy(&pnumpasses)).into()
            }
        }
        unsafe extern "system" fn SetPaletteEntries<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: super::UINT, pentries: *const super::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPaletteEntries(this, core::mem::transmute_copy(&palettenumber), core::mem::transmute_copy(&pentries)).into()
            }
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: super::UINT, pentries: *mut super::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPaletteEntries(this, core::mem::transmute_copy(&palettenumber), core::mem::transmute_copy(&pentries)).into()
            }
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCurrentTexturePalette(this, core::mem::transmute_copy(&palettenumber)).into()
            }
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetCurrentTexturePalette(this, core::mem::transmute_copy(&palettenumber)).into()
            }
        }
        unsafe extern "system" fn SetScissorRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetScissorRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn GetScissorRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetScissorRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsoftware: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetSoftwareVertexProcessing(this, core::mem::transmute_copy(&bsoftware)).into()
            }
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetSoftwareVertexProcessing(this)
            }
        }
        unsafe extern "system" fn SetNPatchMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsegments: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetNPatchMode(this, core::mem::transmute_copy(&nsegments)).into()
            }
        }
        unsafe extern "system" fn GetNPatchMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetNPatchMode(this)
            }
        }
        unsafe extern "system" fn DrawPrimitive<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: super::UINT, primitivecount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawPrimitive(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&startvertex), core::mem::transmute_copy(&primitivecount)).into()
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: super::INT, minvertexindex: super::UINT, numvertices: super::UINT, startindex: super::UINT, primcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawIndexedPrimitive(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&basevertexindex), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&primcount)).into()
            }
        }
        unsafe extern "system" fn DrawPrimitiveUP<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: super::UINT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: super::UINT, numvertices: super::UINT, primitivecount: super::UINT, pindexdata: *const core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawIndexedPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pindexdata), core::mem::transmute_copy(&indexdataformat), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
            }
        }
        unsafe extern "system" fn ProcessVertices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcstartindex: super::UINT, destindex: super::UINT, vertexcount: super::UINT, pdestbuffer: *mut core::ffi::c_void, pvertexdecl: *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ProcessVertices(this, core::mem::transmute_copy(&srcstartindex), core::mem::transmute_copy(&destindex), core::mem::transmute_copy(&vertexcount), core::mem::transmute(&pdestbuffer), core::mem::transmute(&pvertexdecl), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn CreateVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateVertexDeclaration(this, core::mem::transmute_copy(&pvertexelements), core::mem::transmute_copy(&ppdecl)).into()
            }
        }
        unsafe extern "system" fn SetVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexDeclaration(this, core::mem::transmute(&pdecl)).into()
            }
        }
        unsafe extern "system" fn GetVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexDeclaration(this, core::mem::transmute_copy(&ppdecl)).into()
            }
        }
        unsafe extern "system" fn SetFVF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvf: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetFVF(this, core::mem::transmute_copy(&fvf)).into()
            }
        }
        unsafe extern "system" fn GetFVF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvf: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetFVF(this, core::mem::transmute_copy(&pfvf)).into()
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const super::DWORD, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pfunction), core::mem::transmute_copy(&ppshader)).into()
            }
        }
        unsafe extern "system" fn SetVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShader(this, core::mem::transmute(&pshader)).into()
            }
        }
        unsafe extern "system" fn GetVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShader(this, core::mem::transmute_copy(&ppshader)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *const f32, vector4fcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *mut f32, vector4fcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *const i32, vector4icount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *mut i32, vector4icount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *const windows_core::BOOL, boolcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *mut windows_core::BOOL, boolcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn SetStreamSource<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: super::UINT, pstreamdata: *mut core::ffi::c_void, offsetinbytes: super::UINT, stride: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute(&pstreamdata), core::mem::transmute_copy(&offsetinbytes), core::mem::transmute_copy(&stride)).into()
            }
        }
        unsafe extern "system" fn GetStreamSource<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: super::UINT, ppstreamdata: *mut *mut core::ffi::c_void, poffsetinbytes: *mut super::UINT, pstride: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&ppstreamdata), core::mem::transmute_copy(&poffsetinbytes), core::mem::transmute_copy(&pstride)).into()
            }
        }
        unsafe extern "system" fn SetStreamSourceFreq<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: super::UINT, setting: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&setting)).into()
            }
        }
        unsafe extern "system" fn GetStreamSourceFreq<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: super::UINT, psetting: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&psetting)).into()
            }
        }
        unsafe extern "system" fn SetIndices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetIndices(this, core::mem::transmute(&pindexdata)).into()
            }
        }
        unsafe extern "system" fn GetIndices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppindexdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetIndices(this, core::mem::transmute_copy(&ppindexdata)).into()
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const super::DWORD, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pfunction), core::mem::transmute_copy(&ppshader)).into()
            }
        }
        unsafe extern "system" fn SetPixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShader(this, core::mem::transmute(&pshader)).into()
            }
        }
        unsafe extern "system" fn GetPixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShader(this, core::mem::transmute_copy(&ppshader)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *const f32, vector4fcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *mut f32, vector4fcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *const i32, vector4icount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *mut i32, vector4icount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *const windows_core::BOOL, boolcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: super::UINT, pconstantdata: *mut windows_core::BOOL, boolcount: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn DrawRectPatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::UINT, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawRectPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&prectpatchinfo)).into()
            }
        }
        unsafe extern "system" fn DrawTriPatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::UINT, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawTriPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&ptripatchinfo)).into()
            }
        }
        unsafe extern "system" fn DeletePatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DeletePatch(this, core::mem::transmute_copy(&handle)).into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateQuery(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&ppquery)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TestCooperativeLevel: TestCooperativeLevel::<Identity, OFFSET>,
            GetAvailableTextureMem: GetAvailableTextureMem::<Identity, OFFSET>,
            EvictManagedResources: EvictManagedResources::<Identity, OFFSET>,
            GetDirect3D: GetDirect3D::<Identity, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetCreationParameters: GetCreationParameters::<Identity, OFFSET>,
            SetCursorProperties: SetCursorProperties::<Identity, OFFSET>,
            SetCursorPosition: SetCursorPosition::<Identity, OFFSET>,
            ShowCursor: ShowCursor::<Identity, OFFSET>,
            CreateAdditionalSwapChain: CreateAdditionalSwapChain::<Identity, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, OFFSET>,
            GetNumberOfSwapChains: GetNumberOfSwapChains::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Present: Present::<Identity, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, OFFSET>,
            SetDialogBoxMode: SetDialogBoxMode::<Identity, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, OFFSET>,
            GetGammaRamp: GetGammaRamp::<Identity, OFFSET>,
            CreateTexture: CreateTexture::<Identity, OFFSET>,
            CreateVolumeTexture: CreateVolumeTexture::<Identity, OFFSET>,
            CreateCubeTexture: CreateCubeTexture::<Identity, OFFSET>,
            CreateVertexBuffer: CreateVertexBuffer::<Identity, OFFSET>,
            CreateIndexBuffer: CreateIndexBuffer::<Identity, OFFSET>,
            CreateRenderTarget: CreateRenderTarget::<Identity, OFFSET>,
            CreateDepthStencilSurface: CreateDepthStencilSurface::<Identity, OFFSET>,
            UpdateSurface: UpdateSurface::<Identity, OFFSET>,
            UpdateTexture: UpdateTexture::<Identity, OFFSET>,
            GetRenderTargetData: GetRenderTargetData::<Identity, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, OFFSET>,
            StretchRect: StretchRect::<Identity, OFFSET>,
            ColorFill: ColorFill::<Identity, OFFSET>,
            CreateOffscreenPlainSurface: CreateOffscreenPlainSurface::<Identity, OFFSET>,
            SetRenderTarget: SetRenderTarget::<Identity, OFFSET>,
            GetRenderTarget: GetRenderTarget::<Identity, OFFSET>,
            SetDepthStencilSurface: SetDepthStencilSurface::<Identity, OFFSET>,
            GetDepthStencilSurface: GetDepthStencilSurface::<Identity, OFFSET>,
            BeginScene: BeginScene::<Identity, OFFSET>,
            EndScene: EndScene::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            MultiplyTransform: MultiplyTransform::<Identity, OFFSET>,
            SetViewport: SetViewport::<Identity, OFFSET>,
            GetViewport: GetViewport::<Identity, OFFSET>,
            SetMaterial: SetMaterial::<Identity, OFFSET>,
            GetMaterial: GetMaterial::<Identity, OFFSET>,
            SetLight: SetLight::<Identity, OFFSET>,
            GetLight: GetLight::<Identity, OFFSET>,
            LightEnable: LightEnable::<Identity, OFFSET>,
            GetLightEnable: GetLightEnable::<Identity, OFFSET>,
            SetClipPlane: SetClipPlane::<Identity, OFFSET>,
            GetClipPlane: GetClipPlane::<Identity, OFFSET>,
            SetRenderState: SetRenderState::<Identity, OFFSET>,
            GetRenderState: GetRenderState::<Identity, OFFSET>,
            CreateStateBlock: CreateStateBlock::<Identity, OFFSET>,
            BeginStateBlock: BeginStateBlock::<Identity, OFFSET>,
            EndStateBlock: EndStateBlock::<Identity, OFFSET>,
            SetClipStatus: SetClipStatus::<Identity, OFFSET>,
            GetClipStatus: GetClipStatus::<Identity, OFFSET>,
            GetTexture: GetTexture::<Identity, OFFSET>,
            SetTexture: SetTexture::<Identity, OFFSET>,
            GetTextureStageState: GetTextureStageState::<Identity, OFFSET>,
            SetTextureStageState: SetTextureStageState::<Identity, OFFSET>,
            GetSamplerState: GetSamplerState::<Identity, OFFSET>,
            SetSamplerState: SetSamplerState::<Identity, OFFSET>,
            ValidateDevice: ValidateDevice::<Identity, OFFSET>,
            SetPaletteEntries: SetPaletteEntries::<Identity, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, OFFSET>,
            SetCurrentTexturePalette: SetCurrentTexturePalette::<Identity, OFFSET>,
            GetCurrentTexturePalette: GetCurrentTexturePalette::<Identity, OFFSET>,
            SetScissorRect: SetScissorRect::<Identity, OFFSET>,
            GetScissorRect: GetScissorRect::<Identity, OFFSET>,
            SetSoftwareVertexProcessing: SetSoftwareVertexProcessing::<Identity, OFFSET>,
            GetSoftwareVertexProcessing: GetSoftwareVertexProcessing::<Identity, OFFSET>,
            SetNPatchMode: SetNPatchMode::<Identity, OFFSET>,
            GetNPatchMode: GetNPatchMode::<Identity, OFFSET>,
            DrawPrimitive: DrawPrimitive::<Identity, OFFSET>,
            DrawIndexedPrimitive: DrawIndexedPrimitive::<Identity, OFFSET>,
            DrawPrimitiveUP: DrawPrimitiveUP::<Identity, OFFSET>,
            DrawIndexedPrimitiveUP: DrawIndexedPrimitiveUP::<Identity, OFFSET>,
            ProcessVertices: ProcessVertices::<Identity, OFFSET>,
            CreateVertexDeclaration: CreateVertexDeclaration::<Identity, OFFSET>,
            SetVertexDeclaration: SetVertexDeclaration::<Identity, OFFSET>,
            GetVertexDeclaration: GetVertexDeclaration::<Identity, OFFSET>,
            SetFVF: SetFVF::<Identity, OFFSET>,
            GetFVF: GetFVF::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            SetVertexShader: SetVertexShader::<Identity, OFFSET>,
            GetVertexShader: GetVertexShader::<Identity, OFFSET>,
            SetVertexShaderConstantF: SetVertexShaderConstantF::<Identity, OFFSET>,
            GetVertexShaderConstantF: GetVertexShaderConstantF::<Identity, OFFSET>,
            SetVertexShaderConstantI: SetVertexShaderConstantI::<Identity, OFFSET>,
            GetVertexShaderConstantI: GetVertexShaderConstantI::<Identity, OFFSET>,
            SetVertexShaderConstantB: SetVertexShaderConstantB::<Identity, OFFSET>,
            GetVertexShaderConstantB: GetVertexShaderConstantB::<Identity, OFFSET>,
            SetStreamSource: SetStreamSource::<Identity, OFFSET>,
            GetStreamSource: GetStreamSource::<Identity, OFFSET>,
            SetStreamSourceFreq: SetStreamSourceFreq::<Identity, OFFSET>,
            GetStreamSourceFreq: GetStreamSourceFreq::<Identity, OFFSET>,
            SetIndices: SetIndices::<Identity, OFFSET>,
            GetIndices: GetIndices::<Identity, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            SetPixelShader: SetPixelShader::<Identity, OFFSET>,
            GetPixelShader: GetPixelShader::<Identity, OFFSET>,
            SetPixelShaderConstantF: SetPixelShaderConstantF::<Identity, OFFSET>,
            GetPixelShaderConstantF: GetPixelShaderConstantF::<Identity, OFFSET>,
            SetPixelShaderConstantI: SetPixelShaderConstantI::<Identity, OFFSET>,
            GetPixelShaderConstantI: GetPixelShaderConstantI::<Identity, OFFSET>,
            SetPixelShaderConstantB: SetPixelShaderConstantB::<Identity, OFFSET>,
            GetPixelShaderConstantB: GetPixelShaderConstantB::<Identity, OFFSET>,
            DrawRectPatch: DrawRectPatch::<Identity, OFFSET>,
            DrawTriPatch: DrawTriPatch::<Identity, OFFSET>,
            DeletePatch: DeletePatch::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl windows_core::RuntimeName for IDirect3DDevice9 {}
windows_core::imp::define_interface!(IDirect3DDevice9Ex, IDirect3DDevice9Ex_Vtbl, 0xb18b10ce_2649_405a_870f_95f777d4313a);
impl core::ops::Deref for IDirect3DDevice9Ex {
    type Target = IDirect3DDevice9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DDevice9Ex, windows_core::IUnknown, IDirect3DDevice9);
impl IDirect3DDevice9Ex {
    #[cfg(feature = "shared")]
    pub unsafe fn SetConvolutionMonoKernel(&self, width: super::UINT, height: super::UINT, rows: *mut f32, columns: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConvolutionMonoKernel)(windows_core::Interface::as_raw(self), width, height, rows as _, columns as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ComposeRects(&self, psrc: &Option<IDirect3DSurface9>, pdst: &Option<IDirect3DSurface9>, psrcrectdescs: &Option<IDirect3DVertexBuffer9>, numrects: super::UINT, pdstrectdescs: &Option<IDirect3DVertexBuffer9>, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ComposeRects)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psrc), core::mem::transmute_copy(pdst), core::mem::transmute_copy(psrcrectdescs), numrects, core::mem::transmute_copy(pdstrectdescs), operation, xoffset, yoffset) }
    }
    #[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
    pub unsafe fn PresentEx(&self, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PresentEx)(windows_core::Interface::as_raw(self), psourcerect as _, pdestrect as _, hdestwindowoverride as _, pdirtyregion as _, dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetGPUThreadPriority(&self, ppriority: *mut super::INT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGPUThreadPriority)(windows_core::Interface::as_raw(self), ppriority as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetGPUThreadPriority(&self, priority: super::INT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGPUThreadPriority)(windows_core::Interface::as_raw(self), priority) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn WaitForVBlank(&self, iswapchain: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForVBlank)(windows_core::Interface::as_raw(self), iswapchain) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CheckResourceResidency(&self, presourcearray: *mut Option<IDirect3DResource9>, numresources: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckResourceResidency)(windows_core::Interface::as_raw(self), presourcearray as _, numresources) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), pmaxlatency as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn CheckDeviceState(&self, hdestinationwindow: super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceState)(windows_core::Interface::as_raw(self), hdestinationwindow as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateRenderTargetEx(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, lockable: windows_core::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTargetEx)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, lockable, ppsurface as _, psharedhandle as _, usage) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateOffscreenPlainSurfaceEx(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateOffscreenPlainSurfaceEx)(windows_core::Interface::as_raw(self), width, height, format, pool, ppsurface as _, psharedhandle as _, usage) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateDepthStencilSurfaceEx(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, discard: windows_core::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilSurfaceEx)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, discard, ppsurface as _, psharedhandle as _, usage) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn ResetEx(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetEx)(windows_core::Interface::as_raw(self), ppresentationparameters as _, pfullscreendisplaymode as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDisplayModeEx(&self, iswapchain: super::UINT, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeEx)(windows_core::Interface::as_raw(self), iswapchain, pmode as _, protation as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9Ex_Vtbl {
    pub base__: IDirect3DDevice9_Vtbl,
    #[cfg(feature = "shared")]
    pub SetConvolutionMonoKernel: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, *mut f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetConvolutionMonoKernel: usize,
    #[cfg(feature = "shared")]
    pub ComposeRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT, *mut core::ffi::c_void, D3DCOMPOSERECTSOP, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ComposeRects: usize,
    #[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
    pub PresentEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT, *const super::RECT, super::HWND, *const super::RGNDATA, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef", feature = "wingdi")))]
    PresentEx: usize,
    #[cfg(feature = "shared")]
    pub GetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::INT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetGPUThreadPriority: usize,
    #[cfg(feature = "shared")]
    pub SetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, super::INT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetGPUThreadPriority: usize,
    #[cfg(feature = "shared")]
    pub WaitForVBlank: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WaitForVBlank: usize,
    #[cfg(feature = "shared")]
    pub CheckResourceResidency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CheckResourceResidency: usize,
    #[cfg(feature = "shared")]
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMaximumFrameLatency: usize,
    #[cfg(feature = "shared")]
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaximumFrameLatency: usize,
    #[cfg(feature = "windef")]
    pub CheckDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CheckDeviceState: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateRenderTargetEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, D3DFORMAT, D3DMULTISAMPLE_TYPE, super::DWORD, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateRenderTargetEx: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateOffscreenPlainSurfaceEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateOffscreenPlainSurfaceEx: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateDepthStencilSurfaceEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, D3DFORMAT, D3DMULTISAMPLE_TYPE, super::DWORD, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::HANDLE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateDepthStencilSurfaceEx: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub ResetEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENT_PARAMETERS, *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    ResetEx: usize,
    #[cfg(feature = "shared")]
    pub GetDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DDISPLAYMODEEX, *mut D3DDISPLAYROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDisplayModeEx: usize,
}
#[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub trait IDirect3DDevice9Ex_Impl: IDirect3DDevice9_Impl {
    fn SetConvolutionMonoKernel(&self, width: super::UINT, height: super::UINT, rows: *mut f32, columns: *mut f32) -> windows_core::Result<()>;
    fn ComposeRects(&self, psrc: Option<IDirect3DSurface9>, pdst: Option<IDirect3DSurface9>, psrcrectdescs: Option<IDirect3DVertexBuffer9>, numrects: super::UINT, pdstrectdescs: Option<IDirect3DVertexBuffer9>, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::Result<()>;
    fn PresentEx(&self, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self, ppriority: *mut super::INT) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: super::INT) -> windows_core::Result<()>;
    fn WaitForVBlank(&self, iswapchain: super::UINT) -> windows_core::Result<()>;
    fn CheckResourceResidency(&self, presourcearray: *mut Option<IDirect3DResource9>, numresources: super::UINT32) -> windows_core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: super::UINT) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self, pmaxlatency: *mut super::UINT) -> windows_core::Result<()>;
    fn CheckDeviceState(&self, hdestinationwindow: super::HWND) -> windows_core::Result<()>;
    fn CreateRenderTargetEx(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, lockable: windows_core::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurfaceEx(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::Result<()>;
    fn CreateDepthStencilSurfaceEx(&self, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, discard: windows_core::BOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::Result<()>;
    fn ResetEx(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> windows_core::Result<()>;
    fn GetDisplayModeEx(&self, iswapchain: super::UINT, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl IDirect3DDevice9Ex_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetConvolutionMonoKernel<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, rows: *mut f32, columns: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetConvolutionMonoKernel(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&rows), core::mem::transmute_copy(&columns)).into()
            }
        }
        unsafe extern "system" fn ComposeRects<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *mut core::ffi::c_void, pdst: *mut core::ffi::c_void, psrcrectdescs: *mut core::ffi::c_void, numrects: super::UINT, pdstrectdescs: *mut core::ffi::c_void, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ComposeRects(this, core::mem::transmute(&psrc), core::mem::transmute(&pdst), core::mem::transmute(&psrcrectdescs), core::mem::transmute_copy(&numrects), core::mem::transmute(&pdstrectdescs), core::mem::transmute_copy(&operation), core::mem::transmute_copy(&xoffset), core::mem::transmute_copy(&yoffset)).into()
            }
        }
        unsafe extern "system" fn PresentEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::PresentEx(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut super::INT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetGPUThreadPriority(this, core::mem::transmute_copy(&ppriority)).into()
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: super::INT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn WaitForVBlank<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::WaitForVBlank(this, core::mem::transmute_copy(&iswapchain)).into()
            }
        }
        unsafe extern "system" fn CheckResourceResidency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcearray: *mut *mut core::ffi::c_void, numresources: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CheckResourceResidency(this, core::mem::transmute_copy(&presourcearray), core::mem::transmute_copy(&numresources)).into()
            }
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetMaximumFrameLatency(this, core::mem::transmute_copy(&pmaxlatency)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdestinationwindow: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CheckDeviceState(this, core::mem::transmute_copy(&hdestinationwindow)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, lockable: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateRenderTargetEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateOffscreenPlainSurfaceEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: super::UINT, height: super::UINT, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: super::DWORD, discard: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::HANDLE, usage: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateDepthStencilSurfaceEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn ResetEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ResetEx(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pfullscreendisplaymode)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: super::UINT, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetDisplayModeEx(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        Self {
            base__: IDirect3DDevice9_Vtbl::new::<Identity, OFFSET>(),
            SetConvolutionMonoKernel: SetConvolutionMonoKernel::<Identity, OFFSET>,
            ComposeRects: ComposeRects::<Identity, OFFSET>,
            PresentEx: PresentEx::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, OFFSET>,
            CheckResourceResidency: CheckResourceResidency::<Identity, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, OFFSET>,
            CreateRenderTargetEx: CreateRenderTargetEx::<Identity, OFFSET>,
            CreateOffscreenPlainSurfaceEx: CreateOffscreenPlainSurfaceEx::<Identity, OFFSET>,
            CreateDepthStencilSurfaceEx: CreateDepthStencilSurfaceEx::<Identity, OFFSET>,
            ResetEx: ResetEx::<Identity, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9Ex as windows_core::Interface>::IID || iid == &<IDirect3DDevice9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "dsound", feature = "dxgi", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl windows_core::RuntimeName for IDirect3DDevice9Ex {}
windows_core::imp::define_interface!(IDirect3DDevice9Video, IDirect3DDevice9Video_Vtbl, 0x26dc4561_a1ee_4ae7_96da_118a36c0ec95);
windows_core::imp::interface_hierarchy!(IDirect3DDevice9Video, windows_core::IUnknown);
impl IDirect3DDevice9Video {
    #[cfg(feature = "shared")]
    pub unsafe fn GetContentProtectionCaps(&self, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, pcaps: *mut D3DCONTENTPROTECTIONCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContentProtectionCaps)(windows_core::Interface::as_raw(self), pcryptotype as _, pdecodeprofile as _, pcaps as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn CreateAuthenticatedChannel(&self, channeltype: D3DAUTHENTICATEDCHANNELTYPE, ppauthenticatedchannel: *mut Option<IDirect3DAuthenticatedChannel9>, pchannelhandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateAuthenticatedChannel)(windows_core::Interface::as_raw(self), channeltype, ppauthenticatedchannel as _, pchannelhandle as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn CreateCryptoSession(&self, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, ppcryptosession: *mut Option<IDirect3DCryptoSession9>, pcryptohandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateCryptoSession)(windows_core::Interface::as_raw(self), pcryptotype as _, pdecodeprofile as _, ppcryptosession as _, pcryptohandle as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9Video_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetContentProtectionCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut D3DCONTENTPROTECTIONCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetContentProtectionCaps: usize,
    #[cfg(feature = "winnt")]
    pub CreateAuthenticatedChannel: unsafe extern "system" fn(*mut core::ffi::c_void, D3DAUTHENTICATEDCHANNELTYPE, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    CreateAuthenticatedChannel: usize,
    #[cfg(feature = "winnt")]
    pub CreateCryptoSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    CreateCryptoSession: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IDirect3DDevice9Video_Impl: windows_core::IUnknownImpl {
    fn GetContentProtectionCaps(&self, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, pcaps: *mut D3DCONTENTPROTECTIONCAPS) -> windows_core::Result<()>;
    fn CreateAuthenticatedChannel(&self, channeltype: D3DAUTHENTICATEDCHANNELTYPE, ppauthenticatedchannel: *mut Option<IDirect3DAuthenticatedChannel9>, pchannelhandle: *mut super::HANDLE) -> windows_core::Result<()>;
    fn CreateCryptoSession(&self, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, ppcryptosession: *mut Option<IDirect3DCryptoSession9>, pcryptohandle: *mut super::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IDirect3DDevice9Video_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContentProtectionCaps<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, pcaps: *mut D3DCONTENTPROTECTIONCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::GetContentProtectionCaps(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecodeprofile), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channeltype: D3DAUTHENTICATEDCHANNELTYPE, ppauthenticatedchannel: *mut *mut core::ffi::c_void, pchannelhandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::CreateAuthenticatedChannel(this, core::mem::transmute_copy(&channeltype), core::mem::transmute_copy(&ppauthenticatedchannel), core::mem::transmute_copy(&pchannelhandle)).into()
            }
        }
        unsafe extern "system" fn CreateCryptoSession<Identity: IDirect3DDevice9Video_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecodeprofile: *const windows_core::GUID, ppcryptosession: *mut *mut core::ffi::c_void, pcryptohandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Video_Impl::CreateCryptoSession(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecodeprofile), core::mem::transmute_copy(&ppcryptosession), core::mem::transmute_copy(&pcryptohandle)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContentProtectionCaps: GetContentProtectionCaps::<Identity, OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Identity, OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9Video as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IDirect3DDevice9Video {}
windows_core::imp::define_interface!(IDirect3DIndexBuffer9, IDirect3DIndexBuffer9_Vtbl, 0x7c9dd65e_d3f7_4529_acee_785830acde35);
impl core::ops::Deref for IDirect3DIndexBuffer9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DIndexBuffer9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DIndexBuffer9 {
    #[cfg(feature = "shared")]
    pub unsafe fn Lock(&self, offsettolock: super::UINT, sizetolock: super::UINT, ppbdata: *mut *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), offsettolock, sizetolock, ppbdata as _, flags) }
    }
    pub unsafe fn Unlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DINDEXBUFFER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DIndexBuffer9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    #[cfg(feature = "shared")]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, *mut *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Lock: usize,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DINDEXBUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDesc: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3DIndexBuffer9_Impl: IDirect3DResource9_Impl {
    fn Lock(&self, offsettolock: super::UINT, sizetolock: super::UINT, ppbdata: *mut *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DINDEXBUFFER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3DIndexBuffer9_Vtbl {
    pub const fn new<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsettolock: super::UINT, sizetolock: super::UINT, ppbdata: *mut *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::Lock(this, core::mem::transmute_copy(&offsettolock), core::mem::transmute_copy(&sizetolock), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::Unlock(this).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DIndexBuffer9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DIndexBuffer9 {}
windows_core::imp::define_interface!(IDirect3DPixelShader9, IDirect3DPixelShader9_Vtbl, 0x6d3bdbdc_5b02_4415_b852_ce5e8bccb289);
windows_core::imp::interface_hierarchy!(IDirect3DPixelShader9, windows_core::IUnknown);
impl IDirect3DPixelShader9 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), ppdevice as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFunction)(windows_core::Interface::as_raw(self), param0 as _, psizeofdata as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DPixelShader9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFunction: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3DPixelShader9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut super::UINT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3DPixelShader9_Vtbl {
    pub const fn new<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DPixelShader9_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        unsafe extern "system" fn GetFunction<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, psizeofdata: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DPixelShader9_Impl::GetFunction(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DPixelShader9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DPixelShader9 {}
windows_core::imp::define_interface!(IDirect3DQuery9, IDirect3DQuery9_Vtbl, 0xd9771460_a695_4f26_bbd3_27b840b541cc);
windows_core::imp::interface_hierarchy!(IDirect3DQuery9, windows_core::IUnknown);
impl IDirect3DQuery9 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), ppdevice as _) }
    }
    pub unsafe fn GetType(&self) -> D3DQUERYTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDataSize(&self) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Issue(&self, dwissueflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Issue)(windows_core::Interface::as_raw(self), dwissueflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetData(&self, pdata: *mut core::ffi::c_void, dwsize: super::DWORD, dwgetdataflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pdata as _, dwsize, dwgetdataflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DQuery9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3DQUERYTYPE,
    #[cfg(feature = "shared")]
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    GetDataSize: usize,
    #[cfg(feature = "shared")]
    pub Issue: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Issue: usize,
    #[cfg(feature = "shared")]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetData: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3DQuery9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn GetType(&self) -> D3DQUERYTYPE;
    fn GetDataSize(&self) -> super::DWORD;
    fn Issue(&self, dwissueflags: super::DWORD) -> windows_core::Result<()>;
    fn GetData(&self, pdata: *mut core::ffi::c_void, dwsize: super::DWORD, dwgetdataflags: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3DQuery9_Vtbl {
    pub const fn new<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DQUERYTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDataSize<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetDataSize(this)
            }
        }
        unsafe extern "system" fn Issue<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwissueflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::Issue(this, core::mem::transmute_copy(&dwissueflags)).into()
            }
        }
        unsafe extern "system" fn GetData<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, dwsize: super::DWORD, dwgetdataflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwgetdataflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetDataSize: GetDataSize::<Identity, OFFSET>,
            Issue: Issue::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DQuery9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DQuery9 {}
windows_core::imp::define_interface!(IDirect3DResource9, IDirect3DResource9_Vtbl, 0x05eec05d_8f7d_4362_b999_d1baf357c704);
windows_core::imp::interface_hierarchy!(IDirect3DResource9, windows_core::IUnknown);
impl IDirect3DResource9 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), ppdevice as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: super::DWORD, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid as _, pdata as _, sizeofdata, flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid as _, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPriority(&self, prioritynew: super::DWORD) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPriority(&self) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DResource9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPrivateData: usize,
    #[cfg(feature = "shared")]
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPrivateData: usize,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    SetPriority: usize,
    #[cfg(feature = "shared")]
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    GetPriority: usize,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3DRESOURCETYPE,
}
#[cfg(feature = "shared")]
pub trait IDirect3DResource9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: super::DWORD, flags: super::DWORD) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut super::DWORD) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: super::DWORD) -> super::DWORD;
    fn GetPriority(&self) -> super::DWORD;
    fn PreLoad(&self);
    fn GetType(&self) -> D3DRESOURCETYPE;
}
#[cfg(feature = "shared")]
impl IDirect3DResource9_Vtbl {
    pub const fn new<Identity: IDirect3DResource9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: super::DWORD, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: super::DWORD) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::PreLoad(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetType(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DResource9 {}
windows_core::imp::define_interface!(IDirect3DStateBlock9, IDirect3DStateBlock9_Vtbl, 0xb07c4fe5_310d_4ba8_a23c_4f0f206f218b);
windows_core::imp::interface_hierarchy!(IDirect3DStateBlock9, windows_core::IUnknown);
impl IDirect3DStateBlock9 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), ppdevice as _) }
    }
    pub unsafe fn Capture(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Capture)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Apply(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DStateBlock9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Capture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirect3DStateBlock9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn Capture(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
}
impl IDirect3DStateBlock9_Vtbl {
    pub const fn new<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        unsafe extern "system" fn Capture<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::Capture(this).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::Apply(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            Capture: Capture::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DStateBlock9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DStateBlock9 {}
windows_core::imp::define_interface!(IDirect3DSurface9, IDirect3DSurface9_Vtbl, 0x0cfbaf3a_9ff6_429a_99b3_a2796af8b89b);
impl core::ops::Deref for IDirect3DSurface9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DSurface9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DSurface9 {
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetContainer(&self, riid: *const super::IID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContainer)(windows_core::Interface::as_raw(self), riid as _, ppcontainer as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn LockRect(&self, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), plockedrect as _, prect as _, flags) }
    }
    pub unsafe fn UnlockRect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDC(&self, phdc: *mut super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), phdc as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseDC(&self, hdc: super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), hdc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSurface9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    #[cfg(feature = "guiddef")]
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetContainer: usize,
    #[cfg(feature = "shared")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DSURFACE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDesc: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DLOCKED_RECT, *const super::RECT, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    LockRect: usize,
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDC: usize,
    #[cfg(feature = "windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseDC: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef"))]
pub trait IDirect3DSurface9_Impl: IDirect3DResource9_Impl {
    fn GetContainer(&self, riid: *const super::IID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn LockRect(&self, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::Result<()>;
    fn UnlockRect(&self) -> windows_core::Result<()>;
    fn GetDC(&self, phdc: *mut super::HDC) -> windows_core::Result<()>;
    fn ReleaseDC(&self, hdc: super::HDC) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef"))]
impl IDirect3DSurface9_Vtbl {
    pub const fn new<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContainer<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetContainer(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcontainer)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::LockRect(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::UnlockRect(this).into()
            }
        }
        unsafe extern "system" fn GetDC<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetDC(this, core::mem::transmute_copy(&phdc)).into()
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::ReleaseDC(this, core::mem::transmute_copy(&hdc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            GetContainer: GetContainer::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSurface9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IDirect3DSurface9 {}
windows_core::imp::define_interface!(IDirect3DSwapChain9, IDirect3DSwapChain9_Vtbl, 0x794950f2_adfc_458a_905e_10a10b0b503b);
windows_core::imp::interface_hierarchy!(IDirect3DSwapChain9, windows_core::IUnknown);
impl IDirect3DSwapChain9 {
    #[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
    pub unsafe fn Present(&self, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), psourcerect as _, pdestrect as _, hdestwindowoverride as _, pdirtyregion as _, dwflags) }
    }
    pub unsafe fn GetFrontBufferData(&self, pdestsurface: &Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrontBufferData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pdestsurface)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBackBuffer(&self, ibackbuffer: super::UINT, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBackBuffer)(windows_core::Interface::as_raw(self), ibackbuffer, r#type, ppbackbuffer as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRasterStatus)(windows_core::Interface::as_raw(self), prasterstatus as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), pmode as _) }
    }
    pub unsafe fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), ppdevice as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPresentParameters)(windows_core::Interface::as_raw(self), ppresentationparameters as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSwapChain9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT, *const super::RECT, super::HWND, *const super::RGNDATA, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef", feature = "wingdi")))]
    Present: usize,
    pub GetFrontBufferData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetBackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, D3DBACKBUFFER_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBackBuffer: usize,
    #[cfg(feature = "shared")]
    pub GetRasterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DRASTER_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRasterStatus: usize,
    #[cfg(feature = "shared")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DDISPLAYMODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDisplayMode: usize,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetPresentParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetPresentParameters: usize,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub trait IDirect3DSwapChain9_Impl: windows_core::IUnknownImpl {
    fn Present(&self, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, pdestsurface: Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, ibackbuffer: super::UINT, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
impl IDirect3DSwapChain9_Vtbl {
    pub const fn new<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Present<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::RECT, pdestrect: *const super::RECT, hdestwindowoverride: super::HWND, pdirtyregion: *const super::RGNDATA, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetFrontBufferData(this, core::mem::transmute(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ibackbuffer: super::UINT, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetBackBuffer(this, core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&ppbackbuffer)).into()
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetRasterStatus(this, core::mem::transmute_copy(&prasterstatus)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetDisplayMode(this, core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        unsafe extern "system" fn GetPresentParameters<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetPresentParameters(this, core::mem::transmute_copy(&ppresentationparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPresentParameters: GetPresentParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IDirect3DSwapChain9 {}
windows_core::imp::define_interface!(IDirect3DSwapChain9Ex, IDirect3DSwapChain9Ex_Vtbl, 0x91886caf_1c3d_4d2e_a0ab_3e4c7d8d3303);
impl core::ops::Deref for IDirect3DSwapChain9Ex {
    type Target = IDirect3DSwapChain9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DSwapChain9Ex, windows_core::IUnknown, IDirect3DSwapChain9);
impl IDirect3DSwapChain9Ex {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLastPresentCount(&self, plastpresentcount: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLastPresentCount)(windows_core::Interface::as_raw(self), plastpresentcount as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetPresentStats(&self, ppresentationstatistics: *mut D3DPRESENTSTATS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPresentStats)(windows_core::Interface::as_raw(self), ppresentationstatistics as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDisplayModeEx(&self, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeEx)(windows_core::Interface::as_raw(self), pmode as _, protation as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSwapChain9Ex_Vtbl {
    pub base__: IDirect3DSwapChain9_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLastPresentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLastPresentCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetPresentStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENTSTATS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetPresentStats: usize,
    #[cfg(feature = "shared")]
    pub GetDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DDISPLAYMODEEX, *mut D3DDISPLAYROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDisplayModeEx: usize,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub trait IDirect3DSwapChain9Ex_Impl: IDirect3DSwapChain9_Impl {
    fn GetLastPresentCount(&self, plastpresentcount: *mut super::UINT) -> windows_core::Result<()>;
    fn GetPresentStats(&self, ppresentationstatistics: *mut D3DPRESENTSTATS) -> windows_core::Result<()>;
    fn GetDisplayModeEx(&self, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl IDirect3DSwapChain9Ex_Vtbl {
    pub const fn new<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLastPresentCount<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetLastPresentCount(this, core::mem::transmute_copy(&plastpresentcount)).into()
            }
        }
        unsafe extern "system" fn GetPresentStats<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetPresentStats(this, core::mem::transmute_copy(&ppresentationstatistics)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetDisplayModeEx(this, core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        Self {
            base__: IDirect3DSwapChain9_Vtbl::new::<Identity, OFFSET>(),
            GetLastPresentCount: GetLastPresentCount::<Identity, OFFSET>,
            GetPresentStats: GetPresentStats::<Identity, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9Ex as windows_core::Interface>::IID || iid == &<IDirect3DSwapChain9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl windows_core::RuntimeName for IDirect3DSwapChain9Ex {}
windows_core::imp::define_interface!(IDirect3DTexture9, IDirect3DTexture9_Vtbl, 0x85c31227_3de5_4f00_9b3a_f11ac38c18b5);
impl core::ops::Deref for IDirect3DTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DTexture9 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLevelDesc(&self, level: super::UINT, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSurfaceLevel(&self, level: super::UINT, ppsurfacelevel: *mut Option<IDirect3DSurface9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSurfaceLevel)(windows_core::Interface::as_raw(self), level, ppsurfacelevel as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn LockRect(&self, level: super::UINT, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), level, plockedrect as _, prect as _, flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UnlockRect(&self, level: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self), level) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn AddDirtyRect(&self, pdirtyrect: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyRect)(windows_core::Interface::as_raw(self), pdirtyrect as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DSURFACE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLevelDesc: usize,
    #[cfg(feature = "shared")]
    pub GetSurfaceLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSurfaceLevel: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DLOCKED_RECT, *const super::RECT, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    LockRect: usize,
    #[cfg(feature = "shared")]
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UnlockRect: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub AddDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    AddDirtyRect: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IDirect3DTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: super::UINT, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn GetSurfaceLevel(&self, level: super::UINT, ppsurfacelevel: *mut Option<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn LockRect(&self, level: super::UINT, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::Result<()>;
    fn UnlockRect(&self, level: super::UINT) -> windows_core::Result<()>;
    fn AddDirtyRect(&self, pdirtyrect: *const super::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IDirect3DTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceLevel<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT, ppsurfacelevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetSurfaceLevel(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&ppsurfacelevel)).into()
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::RECT, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::LockRect(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::UnlockRect(this, core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::AddDirtyRect(this, core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetSurfaceLevel: GetSurfaceLevel::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IDirect3DTexture9 {}
windows_core::imp::define_interface!(IDirect3DVertexBuffer9, IDirect3DVertexBuffer9_Vtbl, 0xb64bb1b5_fd70_4df6_bf91_19d0a12455e3);
impl core::ops::Deref for IDirect3DVertexBuffer9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DVertexBuffer9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DVertexBuffer9 {
    #[cfg(feature = "shared")]
    pub unsafe fn Lock(&self, offsettolock: super::UINT, sizetolock: super::UINT, ppbdata: *mut *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), offsettolock, sizetolock, ppbdata as _, flags) }
    }
    pub unsafe fn Unlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexBuffer9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    #[cfg(feature = "shared")]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::UINT, *mut *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Lock: usize,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDesc: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3DVertexBuffer9_Impl: IDirect3DResource9_Impl {
    fn Lock(&self, offsettolock: super::UINT, sizetolock: super::UINT, ppbdata: *mut *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DVERTEXBUFFER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3DVertexBuffer9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsettolock: super::UINT, sizetolock: super::UINT, ppbdata: *mut *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::Lock(this, core::mem::transmute_copy(&offsettolock), core::mem::transmute_copy(&sizetolock), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::Unlock(this).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexBuffer9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DVertexBuffer9 {}
windows_core::imp::define_interface!(IDirect3DVertexDeclaration9, IDirect3DVertexDeclaration9_Vtbl, 0xdd13c59c_36fa_4098_a8fb_c7ed39dc8546);
windows_core::imp::interface_hierarchy!(IDirect3DVertexDeclaration9, windows_core::IUnknown);
impl IDirect3DVertexDeclaration9 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), ppdevice as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDeclaration(&self, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeclaration)(windows_core::Interface::as_raw(self), pelement as _, pnumelements as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexDeclaration9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVERTEXELEMENT9, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDeclaration: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3DVertexDeclaration9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn GetDeclaration(&self, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut super::UINT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3DVertexDeclaration9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexDeclaration9_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        unsafe extern "system" fn GetDeclaration<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexDeclaration9_Impl::GetDeclaration(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&pnumelements)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetDeclaration: GetDeclaration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexDeclaration9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DVertexDeclaration9 {}
windows_core::imp::define_interface!(IDirect3DVertexShader9, IDirect3DVertexShader9_Vtbl, 0xefc5557e_6265_4613_8a94_43857889eb36);
windows_core::imp::interface_hierarchy!(IDirect3DVertexShader9, windows_core::IUnknown);
impl IDirect3DVertexShader9 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), ppdevice as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFunction)(windows_core::Interface::as_raw(self), param0 as _, psizeofdata as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexShader9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFunction: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3DVertexShader9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut super::UINT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3DVertexShader9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexShader9_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        unsafe extern "system" fn GetFunction<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, psizeofdata: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexShader9_Impl::GetFunction(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexShader9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DVertexShader9 {}
windows_core::imp::define_interface!(IDirect3DVolume9, IDirect3DVolume9_Vtbl, 0x24f416e6_1f67_4aa7_b88e_d33f6f3128a1);
windows_core::imp::interface_hierarchy!(IDirect3DVolume9, windows_core::IUnknown);
impl IDirect3DVolume9 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), ppdevice as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: super::DWORD, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid as _, pdata as _, sizeofdata, flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid as _, pdata as _, psizeofdata as _) }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid as _) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetContainer(&self, riid: *const super::IID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContainer)(windows_core::Interface::as_raw(self), riid as _, ppcontainer as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DVOLUME_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn LockBox(&self, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockBox)(windows_core::Interface::as_raw(self), plockedvolume as _, pbox as _, flags) }
    }
    pub unsafe fn UnlockBox(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockBox)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVolume9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPrivateData: usize,
    #[cfg(feature = "shared")]
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPrivateData: usize,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "guiddef")]
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetContainer: usize,
    #[cfg(feature = "shared")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVOLUME_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDesc: usize,
    #[cfg(feature = "shared")]
    pub LockBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DLOCKED_BOX, *const D3DBOX, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LockBox: usize,
    pub UnlockBox: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IDirect3DVolume9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: super::DWORD, flags: super::DWORD) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut super::DWORD) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetContainer(&self, riid: *const super::IID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DVOLUME_DESC) -> windows_core::Result<()>;
    fn LockBox(&self, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: super::DWORD) -> windows_core::Result<()>;
    fn UnlockBox(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IDirect3DVolume9_Vtbl {
    pub const fn new<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: super::DWORD, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn GetContainer<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetContainer(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcontainer)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn LockBox<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::LockBox(this, core::mem::transmute_copy(&plockedvolume), core::mem::transmute_copy(&pbox), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockBox<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::UnlockBox(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            GetContainer: GetContainer::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            LockBox: LockBox::<Identity, OFFSET>,
            UnlockBox: UnlockBox::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVolume9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IDirect3DVolume9 {}
windows_core::imp::define_interface!(IDirect3DVolumeTexture9, IDirect3DVolumeTexture9_Vtbl, 0x2518526c_e789_4111_a7b9_47ef328d13e6);
impl core::ops::Deref for IDirect3DVolumeTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DVolumeTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DVolumeTexture9 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLevelDesc(&self, level: super::UINT, pdesc: *mut D3DVOLUME_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetVolumeLevel(&self, level: super::UINT, ppvolumelevel: *mut Option<IDirect3DVolume9>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVolumeLevel)(windows_core::Interface::as_raw(self), level, ppvolumelevel as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn LockBox(&self, level: super::UINT, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockBox)(windows_core::Interface::as_raw(self), level, plockedvolume as _, pbox as _, flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UnlockBox(&self, level: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockBox)(windows_core::Interface::as_raw(self), level) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddDirtyBox(&self, pdirtybox: *const D3DBOX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyBox)(windows_core::Interface::as_raw(self), pdirtybox as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVolumeTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DVOLUME_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLevelDesc: usize,
    #[cfg(feature = "shared")]
    pub GetVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetVolumeLevel: usize,
    #[cfg(feature = "shared")]
    pub LockBox: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut D3DLOCKED_BOX, *const D3DBOX, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LockBox: usize,
    #[cfg(feature = "shared")]
    pub UnlockBox: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UnlockBox: usize,
    #[cfg(feature = "shared")]
    pub AddDirtyBox: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DBOX) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddDirtyBox: usize,
}
#[cfg(feature = "shared")]
pub trait IDirect3DVolumeTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: super::UINT, pdesc: *mut D3DVOLUME_DESC) -> windows_core::Result<()>;
    fn GetVolumeLevel(&self, level: super::UINT, ppvolumelevel: *mut Option<IDirect3DVolume9>) -> windows_core::Result<()>;
    fn LockBox(&self, level: super::UINT, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: super::DWORD) -> windows_core::Result<()>;
    fn UnlockBox(&self, level: super::UINT) -> windows_core::Result<()>;
    fn AddDirtyBox(&self, pdirtybox: *const D3DBOX) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirect3DVolumeTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT, pdesc: *mut D3DVOLUME_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT, ppvolumelevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetVolumeLevel(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&ppvolumelevel)).into()
            }
        }
        unsafe extern "system" fn LockBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::LockBox(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedvolume), core::mem::transmute_copy(&pbox), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::UnlockBox(this, core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtybox: *const D3DBOX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::AddDirtyBox(this, core::mem::transmute_copy(&pdirtybox)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, OFFSET>,
            LockBox: LockBox::<Identity, OFFSET>,
            UnlockBox: UnlockBox::<Identity, OFFSET>,
            AddDirtyBox: AddDirtyBox::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVolumeTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirect3DVolumeTexture9 {}
pub const IID_HelperName: windows_core::GUID = windows_core::GUID::from_u128(0xe4a36723_fdfe_4b22_b146_3c04c07f4cc8);
pub const IID_IDirect3D9: windows_core::GUID = windows_core::GUID::from_u128(0x81bdcbca_64d4_426d_ae8d_ad0147f4275c);
pub const IID_IDirect3D9Ex: windows_core::GUID = windows_core::GUID::from_u128(0x02177241_69fc_400c_8ff1_93a44df6861d);
pub const IID_IDirect3D9ExOverlayExtension: windows_core::GUID = windows_core::GUID::from_u128(0x187aeb13_aaf5_4c59_876d_e059088c0df8);
pub const IID_IDirect3DAuthenticatedChannel9: windows_core::GUID = windows_core::GUID::from_u128(0xff24beee_da21_4beb_98b5_d2f899f98af9);
pub const IID_IDirect3DBaseTexture9: windows_core::GUID = windows_core::GUID::from_u128(0x580ca87e_1d3c_4d54_991d_b7d3e3c298ce);
pub const IID_IDirect3DCryptoSession9: windows_core::GUID = windows_core::GUID::from_u128(0xfa0ab799_7a9c_48ca_8c5b_237e71a54434);
pub const IID_IDirect3DCubeTexture9: windows_core::GUID = windows_core::GUID::from_u128(0xfff32f81_d953_473a_9223_93d652aba93f);
pub const IID_IDirect3DDevice9: windows_core::GUID = windows_core::GUID::from_u128(0xd0223b96_bf7a_43fd_92bd_a43b0d82b9eb);
pub const IID_IDirect3DDevice9Ex: windows_core::GUID = windows_core::GUID::from_u128(0xb18b10ce_2649_405a_870f_95f777d4313a);
pub const IID_IDirect3DDevice9Video: windows_core::GUID = windows_core::GUID::from_u128(0x26dc4561_a1ee_4ae7_96da_118a36c0ec95);
pub const IID_IDirect3DIndexBuffer9: windows_core::GUID = windows_core::GUID::from_u128(0x7c9dd65e_d3f7_4529_acee_785830acde35);
pub const IID_IDirect3DPixelShader9: windows_core::GUID = windows_core::GUID::from_u128(0x6d3bdbdc_5b02_4415_b852_ce5e8bccb289);
pub const IID_IDirect3DQuery9: windows_core::GUID = windows_core::GUID::from_u128(0xd9771460_a695_4f26_bbd3_27b840b541cc);
pub const IID_IDirect3DResource9: windows_core::GUID = windows_core::GUID::from_u128(0x05eec05d_8f7d_4362_b999_d1baf357c704);
pub const IID_IDirect3DStateBlock9: windows_core::GUID = windows_core::GUID::from_u128(0xb07c4fe5_310d_4ba8_a23c_4f0f206f218b);
pub const IID_IDirect3DSurface9: windows_core::GUID = windows_core::GUID::from_u128(0x0cfbaf3a_9ff6_429a_99b3_a2796af8b89b);
pub const IID_IDirect3DSwapChain9: windows_core::GUID = windows_core::GUID::from_u128(0x794950f2_adfc_458a_905e_10a10b0b503b);
pub const IID_IDirect3DSwapChain9Ex: windows_core::GUID = windows_core::GUID::from_u128(0x91886caf_1c3d_4d2e_a0ab_3e4c7d8d3303);
pub const IID_IDirect3DTexture9: windows_core::GUID = windows_core::GUID::from_u128(0x85c31227_3de5_4f00_9b3a_f11ac38c18b5);
pub const IID_IDirect3DVertexBuffer9: windows_core::GUID = windows_core::GUID::from_u128(0xb64bb1b5_fd70_4df6_bf91_19d0a12455e3);
pub const IID_IDirect3DVertexDeclaration9: windows_core::GUID = windows_core::GUID::from_u128(0xdd13c59c_36fa_4098_a8fb_c7ed39dc8546);
pub const IID_IDirect3DVertexShader9: windows_core::GUID = windows_core::GUID::from_u128(0xefc5557e_6265_4613_8a94_43857889eb36);
pub const IID_IDirect3DVolume9: windows_core::GUID = windows_core::GUID::from_u128(0x24f416e6_1f67_4aa7_b88e_d33f6f3128a1);
pub const IID_IDirect3DVolumeTexture9: windows_core::GUID = windows_core::GUID::from_u128(0x2518526c_e789_4111_a7b9_47ef328d13e6);
#[cfg(feature = "shared")]
pub type LPD3DDEVINFO_D3DVERTEXSTATS = *mut D3DDEVINFO_D3DVERTEXSTATS;
#[cfg(feature = "shared")]
pub type LPD3DDEVINFO_RESOURCEMANAGER = *mut D3DDEVINFO_RESOURCEMANAGER;
#[cfg(feature = "shared")]
pub type LPD3DDEVINFO_VCACHE = *mut D3DDEVINFO_VCACHE;
#[cfg(feature = "shared")]
pub type LPD3DVERTEXELEMENT9 = *mut D3DVERTEXELEMENT9;
pub type LPDIRECT3D9 = IDirect3D9;
pub type LPDIRECT3D9EX = IDirect3D9Ex;
pub type LPDIRECT3D9EXOVERLAYEXTENSION = IDirect3D9ExOverlayExtension;
pub type LPDIRECT3DAUTHENTICATEDCHANNEL9 = IDirect3DAuthenticatedChannel9;
pub type LPDIRECT3DBASETEXTURE9 = IDirect3DBaseTexture9;
pub type LPDIRECT3DCRYPTOSESSION9 = IDirect3DCryptoSession9;
pub type LPDIRECT3DCUBETEXTURE9 = IDirect3DCubeTexture9;
pub type LPDIRECT3DDEVICE9 = IDirect3DDevice9;
pub type LPDIRECT3DDEVICE9EX = IDirect3DDevice9Ex;
pub type LPDIRECT3DDEVICE9VIDEO = IDirect3DDevice9Video;
pub type LPDIRECT3DINDEXBUFFER9 = IDirect3DIndexBuffer9;
pub type LPDIRECT3DPIXELSHADER9 = IDirect3DPixelShader9;
pub type LPDIRECT3DQUERY9 = IDirect3DQuery9;
pub type LPDIRECT3DRESOURCE9 = IDirect3DResource9;
pub type LPDIRECT3DSTATEBLOCK9 = IDirect3DStateBlock9;
pub type LPDIRECT3DSURFACE9 = IDirect3DSurface9;
pub type LPDIRECT3DSWAPCHAIN9 = IDirect3DSwapChain9;
pub type LPDIRECT3DSWAPCHAIN9EX = IDirect3DSwapChain9Ex;
pub type LPDIRECT3DTEXTURE9 = IDirect3DTexture9;
pub type LPDIRECT3DVERTEXBUFFER9 = IDirect3DVertexBuffer9;
pub type LPDIRECT3DVERTEXDECLARATION9 = IDirect3DVertexDeclaration9;
pub type LPDIRECT3DVERTEXSHADER9 = IDirect3DVertexShader9;
pub type LPDIRECT3DVOLUME9 = IDirect3DVolume9;
pub type LPDIRECT3DVOLUMETEXTURE9 = IDirect3DVolumeTexture9;
pub const MAXD3DDECLLENGTH: i32 = 64;
pub const MAXD3DDECLMETHOD: D3DDECLMETHOD = 6;
pub const MAXD3DDECLTYPE: D3DDECLTYPE = 17;
pub const MAXD3DDECLUSAGE: D3DDECLUSAGE = 13;
pub const MAXD3DDECLUSAGEINDEX: i32 = 15;
pub const MAX_DEVICE_IDENTIFIER_STRING: i32 = 512;
pub type PDIRECT3D9 = IDirect3D9;
pub type PDIRECT3D9EX = IDirect3D9Ex;
pub type PDIRECT3D9EXOVERLAYEXTENSION = IDirect3D9ExOverlayExtension;
pub type PDIRECT3DAUTHENTICATEDCHANNEL9 = IDirect3DAuthenticatedChannel9;
pub type PDIRECT3DBASETEXTURE9 = IDirect3DBaseTexture9;
pub type PDIRECT3DCRYPTOSESSION9 = IDirect3DCryptoSession9;
pub type PDIRECT3DCUBETEXTURE9 = IDirect3DCubeTexture9;
pub type PDIRECT3DDEVICE9 = IDirect3DDevice9;
pub type PDIRECT3DDEVICE9EX = IDirect3DDevice9Ex;
pub type PDIRECT3DDEVICE9VIDEO = IDirect3DDevice9Video;
pub type PDIRECT3DINDEXBUFFER9 = IDirect3DIndexBuffer9;
pub type PDIRECT3DPIXELSHADER9 = IDirect3DPixelShader9;
pub type PDIRECT3DQUERY9 = IDirect3DQuery9;
pub type PDIRECT3DRESOURCE9 = IDirect3DResource9;
pub type PDIRECT3DSTATEBLOCK9 = IDirect3DStateBlock9;
pub type PDIRECT3DSURFACE9 = IDirect3DSurface9;
pub type PDIRECT3DSWAPCHAIN9 = IDirect3DSwapChain9;
pub type PDIRECT3DSWAPCHAIN9EX = IDirect3DSwapChain9Ex;
pub type PDIRECT3DTEXTURE9 = IDirect3DTexture9;
pub type PDIRECT3DVERTEXBUFFER9 = IDirect3DVertexBuffer9;
pub type PDIRECT3DVERTEXDECLARATION9 = IDirect3DVertexDeclaration9;
pub type PDIRECT3DVERTEXSHADER9 = IDirect3DVertexShader9;
pub type PDIRECT3DVOLUME9 = IDirect3DVolume9;
pub type PDIRECT3DVOLUMETEXTURE9 = IDirect3DVolumeTexture9;
pub const PROCESSIDTYPE_DWM: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = 1;
pub const PROCESSIDTYPE_HANDLE: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = 2;
pub const PROCESSIDTYPE_UNKNOWN: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = 0;
pub const S_NOT_RESIDENT: windows_core::HRESULT = windows_core::HRESULT(0x8760875_u32 as _);
pub const S_PRESENT_MODE_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x8760877_u32 as _);
pub const S_PRESENT_OCCLUDED: windows_core::HRESULT = windows_core::HRESULT(0x8760878_u32 as _);
pub const S_RESIDENT_IN_SHARED_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8760876_u32 as _);
pub const _FACD3D: i32 = 2166;
