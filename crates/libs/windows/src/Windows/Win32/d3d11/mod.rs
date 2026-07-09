#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn D3D11CreateDevice<P0>(padapter: P0, drivertype: super::d3dcommon::D3D_DRIVER_TYPE, software: super::minwindef::HMODULE, flags: u32, pfeaturelevels: Option<&[super::d3dcommon::D3D_FEATURE_LEVEL]>, sdkversion: u32, ppdevice: Option<*mut Option<ID3D11Device>>, pfeaturelevel: Option<*mut super::d3dcommon::D3D_FEATURE_LEVEL>, ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d11.dll" "system" fn D3D11CreateDevice(padapter : *mut core::ffi::c_void, drivertype : super::d3dcommon::D3D_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, pfeaturelevels : *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut super::d3dcommon::D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D11CreateDevice(padapter.param().abi(), drivertype, software, flags, core::mem::transmute(pfeaturelevels.map_or(core::ptr::null(), |slice| slice.as_ptr())), pfeaturelevels.map_or(0, |slice| slice.len().try_into().unwrap()), sdkversion, ppdevice.unwrap_or(core::mem::zeroed()) as _, pfeaturelevel.unwrap_or(core::mem::zeroed()) as _, ppimmediatecontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
#[inline]
pub unsafe fn D3D11CreateDeviceAndSwapChain<P0>(padapter: P0, drivertype: super::d3dcommon::D3D_DRIVER_TYPE, software: super::minwindef::HMODULE, flags: u32, pfeaturelevels: Option<&[super::d3dcommon::D3D_FEATURE_LEVEL]>, sdkversion: u32, pswapchaindesc: Option<*const super::dxgi::DXGI_SWAP_CHAIN_DESC>, ppswapchain: Option<*mut Option<super::dxgi::IDXGISwapChain>>, ppdevice: Option<*mut Option<ID3D11Device>>, pfeaturelevel: Option<*mut super::d3dcommon::D3D_FEATURE_LEVEL>, ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d11.dll" "system" fn D3D11CreateDeviceAndSwapChain(padapter : *mut core::ffi::c_void, drivertype : super::d3dcommon::D3D_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, pfeaturelevels : *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, pswapchaindesc : *const super::dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut super::d3dcommon::D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D11CreateDeviceAndSwapChain(padapter.param().abi(), drivertype, software, flags, core::mem::transmute(pfeaturelevels.map_or(core::ptr::null(), |slice| slice.as_ptr())), pfeaturelevels.map_or(0, |slice| slice.len().try_into().unwrap()), sdkversion, pswapchaindesc.unwrap_or(core::mem::zeroed()) as _, ppswapchain.unwrap_or(core::mem::zeroed()) as _, ppdevice.unwrap_or(core::mem::zeroed()) as _, pfeaturelevel.unwrap_or(core::mem::zeroed()) as _, ppimmediatecontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct APP_DEPRECATED_HRESULT(pub windows_core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_BLEND_DESC {
    pub Base: D3D11_BLEND_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_BOX {
    pub Base: D3D11_BOX,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_BUFFER_DESC {
    pub Base: D3D11_BUFFER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_COUNTER_DESC {
    pub Base: D3D11_COUNTER_DESC,
}
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_DEFAULT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_DEPTH_STENCIL_DESC {
    pub Base: D3D11_DEPTH_STENCIL_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct CD3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Base: D3D11_DEPTH_STENCIL_VIEW_DESC,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for CD3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_QUERY_DESC {
    pub Base: D3D11_QUERY_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_RASTERIZER_DESC {
    pub Base: D3D11_RASTERIZER_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_RECT {
    pub Base: D3D11_RECT,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct CD3D11_RENDER_TARGET_VIEW_DESC {
    pub Base: D3D11_RENDER_TARGET_VIEW_DESC,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for CD3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_SAMPLER_DESC {
    pub Base: D3D11_SAMPLER_DESC,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct CD3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Base: D3D11_SHADER_RESOURCE_VIEW_DESC,
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for CD3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_TEXTURE1D_DESC {
    pub Base: D3D11_TEXTURE1D_DESC,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_TEXTURE2D_DESC {
    pub Base: D3D11_TEXTURE2D_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_TEXTURE3D_DESC {
    pub Base: D3D11_TEXTURE3D_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct CD3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Base: D3D11_UNORDERED_ACCESS_VIEW_DESC,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for CD3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_VIDEO_DEFAULT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_VIEWPORT {
    pub Base: D3D11_VIEWPORT,
}
pub const D3D11_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535;
pub const D3D11_1_UAV_SLOT_COUNT: u32 = 64;
pub const D3D11_2_TILED_RESOURCE_TILE_SIZE_IN_BYTES: u32 = 65536;
pub const D3D11_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295;
pub const D3D11_4_VIDEO_DECODER_HISTOGRAM_OFFSET_ALIGNMENT: u32 = 256;
pub const D3D11_4_VIDEO_DECODER_MAX_HISTOGRAM_COMPONENTS: u32 = 4;
pub const D3D11_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
pub const D3D11_ANISOTROPIC_FILTERING_BIT: u32 = 64;
pub const D3D11_APPEND_ALIGNED_ELEMENT: u32 = 4294967295;
pub const D3D11_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9;
pub const D3D11_ASYNC_GETDATA_DONOTFLUSH: D3D11_ASYNC_GETDATA_FLAG = 1;
pub type D3D11_ASYNC_GETDATA_FLAG = i32;
pub const D3D11_AUTHENTICATED_CHANNEL_D3D11: D3D11_AUTHENTICATED_CHANNEL_TYPE = 1;
pub const D3D11_AUTHENTICATED_CHANNEL_DRIVER_HARDWARE: D3D11_AUTHENTICATED_CHANNEL_TYPE = 3;
pub const D3D11_AUTHENTICATED_CHANNEL_DRIVER_SOFTWARE: D3D11_AUTHENTICATED_CHANNEL_TYPE = 2;
pub type D3D11_AUTHENTICATED_CHANNEL_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub EncryptionGuid: windows_core::GUID,
}
pub const D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6346cc54_2cfc_4ad4_8224_d15837de7700);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub DecoderHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub DeviceHandle: super::winnt::HANDLE,
}
pub const D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0x41fff286_6ae0_4d43_9d55_a46e9efd158a);
pub const D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE: windows_core::GUID = windows_core::GUID::from_u128(0x06114bdb_3523_470a_8dca_fbc2845154f0);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub StartSequenceQuery: u32,
    pub StartSequenceConfigure: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_core::HRESULT,
}
pub const D3D11_AUTHENTICATED_CONFIGURE_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0x50455658_3f47_4362_bf99_bfdfcde9ed29);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub Protections: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x0772d047_1b40_48e8_9ca6_b5f510de9f01);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub ProcessType: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::winnt::HANDLE,
    pub AllowAccess: windows_core::BOOL,
}
pub type D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    pub Flags: D3D11_AUTHENTICATED_PROTECTION_FLAGS_0,
    pub Value: u32,
}
impl Default for D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_PROTECTION_FLAGS_0 {
    pub _bitfield: u32,
}
pub const D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES: windows_core::GUID = windows_core::GUID::from_u128(0x6214d9d2_432c_4abb_9fce_216eea269e3b);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub EncryptionGuidIndex: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub BusType: D3D11_BUS_TYPE,
    pub AccessibleInContiguousBlocks: windows_core::BOOL,
    pub AccessibleInNonContiguousBlocks: windows_core::BOOL,
}
pub const D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xbc1b18a5_b1fb_42ab_bd94_b5828b4bf7be);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ChannelType: D3D11_AUTHENTICATED_CHANNEL_TYPE,
}
pub const D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION: windows_core::GUID = windows_core::GUID::from_u128(0x2634499e_d018_4d74_ac17_7f724059528d);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DecoderHandle: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DecoderHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuid: windows_core::GUID,
}
pub const D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0xec1791c7_dad3_4f15_9ec3_faa93d60d4f0);
pub const D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE: windows_core::GUID = windows_core::GUID::from_u128(0xec1c539d_8cff_4e2a_bcc4_f5692f99f480);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
}
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf83a5958_e986_4bda_beb0_411f6a7a01b7);
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0xb30f7066_203c_4b07_93fc_ceaafd61241e);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_INPUT {
    pub QueryType: windows_core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT {
    pub omac: D3D11_OMAC,
    pub QueryType: windows_core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_core::HRESULT,
}
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID: windows_core::GUID = windows_core::GUID::from_u128(0x839ddca3_9b4e_41e4_b053_892bd2a11ee7);
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x2c042b5e_8c07_46d5_aabe_8f75cbad4c31);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
pub const D3D11_AUTHENTICATED_QUERY_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0xa84eb584_c495_48aa_b94d_8bd2d6fbce05);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProtectionFlags: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS: windows_core::GUID = windows_core::GUID::from_u128(0x649bbadb_f0f4_4639_a15b_24393fc3abac);
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x0db207b3_9450_46a6_82de_1b96d44f9cf2);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub RestrictedSharedResourceProcessCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub ProcessIndex: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProcessIndex: u32,
    pub ProcessIdentifier: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::winnt::HANDLE,
}
pub const D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x012f0bd6_e662_4474_befd_aa53e5143c6d);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub UnrestrictedProtectedSharedResourceCount: u32,
}
pub const D3D11_BIND_CONSTANT_BUFFER: D3D11_BIND_FLAG = 4;
pub const D3D11_BIND_DECODER: D3D11_BIND_FLAG = 512;
pub const D3D11_BIND_DEPTH_STENCIL: D3D11_BIND_FLAG = 64;
pub type D3D11_BIND_FLAG = i32;
pub const D3D11_BIND_INDEX_BUFFER: D3D11_BIND_FLAG = 2;
pub const D3D11_BIND_RENDER_TARGET: D3D11_BIND_FLAG = 32;
pub const D3D11_BIND_SHADER_RESOURCE: D3D11_BIND_FLAG = 8;
pub const D3D11_BIND_STREAM_OUTPUT: D3D11_BIND_FLAG = 16;
pub const D3D11_BIND_UNORDERED_ACCESS: D3D11_BIND_FLAG = 128;
pub const D3D11_BIND_VERTEX_BUFFER: D3D11_BIND_FLAG = 1;
pub const D3D11_BIND_VIDEO_ENCODER: D3D11_BIND_FLAG = 1024;
pub type D3D11_BLEND = i32;
pub const D3D11_BLEND_BLEND_FACTOR: D3D11_BLEND = 14;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_BLEND_DESC {
    pub AlphaToCoverageEnable: windows_core::BOOL,
    pub IndependentBlendEnable: windows_core::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}
impl Default for D3D11_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_BLEND_DEST_ALPHA: D3D11_BLEND = 7;
pub const D3D11_BLEND_DEST_COLOR: D3D11_BLEND = 9;
pub const D3D11_BLEND_INV_BLEND_FACTOR: D3D11_BLEND = 15;
pub const D3D11_BLEND_INV_DEST_ALPHA: D3D11_BLEND = 8;
pub const D3D11_BLEND_INV_DEST_COLOR: D3D11_BLEND = 10;
pub const D3D11_BLEND_INV_SRC1_ALPHA: D3D11_BLEND = 19;
pub const D3D11_BLEND_INV_SRC1_COLOR: D3D11_BLEND = 17;
pub const D3D11_BLEND_INV_SRC_ALPHA: D3D11_BLEND = 6;
pub const D3D11_BLEND_INV_SRC_COLOR: D3D11_BLEND = 4;
pub const D3D11_BLEND_ONE: D3D11_BLEND = 2;
pub type D3D11_BLEND_OP = i32;
pub const D3D11_BLEND_OP_ADD: D3D11_BLEND_OP = 1;
pub const D3D11_BLEND_OP_MAX: D3D11_BLEND_OP = 5;
pub const D3D11_BLEND_OP_MIN: D3D11_BLEND_OP = 4;
pub const D3D11_BLEND_OP_REV_SUBTRACT: D3D11_BLEND_OP = 3;
pub const D3D11_BLEND_OP_SUBTRACT: D3D11_BLEND_OP = 2;
pub const D3D11_BLEND_SRC1_ALPHA: D3D11_BLEND = 18;
pub const D3D11_BLEND_SRC1_COLOR: D3D11_BLEND = 16;
pub const D3D11_BLEND_SRC_ALPHA: D3D11_BLEND = 5;
pub const D3D11_BLEND_SRC_ALPHA_SAT: D3D11_BLEND = 11;
pub const D3D11_BLEND_SRC_COLOR: D3D11_BLEND = 3;
pub const D3D11_BLEND_ZERO: D3D11_BLEND = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_BUFFEREX_SRV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
pub type D3D11_BUFFEREX_SRV_FLAG = i32;
pub const D3D11_BUFFEREX_SRV_FLAG_RAW: D3D11_BUFFEREX_SRV_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub StructureByteStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BUFFER_RTV {
    pub Anonymous: D3D11_BUFFER_RTV_0,
    pub Anonymous2: D3D11_BUFFER_RTV_1,
}
impl Default for D3D11_BUFFER_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D11_BUFFER_RTV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D11_BUFFER_RTV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BUFFER_SRV {
    pub Anonymous: D3D11_BUFFER_SRV_0,
    pub Anonymous2: D3D11_BUFFER_SRV_1,
}
impl Default for D3D11_BUFFER_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D11_BUFFER_SRV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D11_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_BUFFER_UAV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
pub type D3D11_BUFFER_UAV_FLAG = i32;
pub const D3D11_BUFFER_UAV_FLAG_APPEND: D3D11_BUFFER_UAV_FLAG = 2;
pub const D3D11_BUFFER_UAV_FLAG_COUNTER: D3D11_BUFFER_UAV_FLAG = 4;
pub const D3D11_BUFFER_UAV_FLAG_RAW: D3D11_BUFFER_UAV_FLAG = 1;
pub const D3D11_BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR: D3D11_BUS_TYPE = 262144;
pub const D3D11_BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: D3D11_BUS_TYPE = 327680;
pub const D3D11_BUS_IMPL_MODIFIER_INSIDE_OF_CHIPSET: D3D11_BUS_TYPE = 65536;
pub const D3D11_BUS_IMPL_MODIFIER_NON_STANDARD: D3D11_BUS_TYPE = -2147483648;
pub const D3D11_BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: D3D11_BUS_TYPE = 131072;
pub const D3D11_BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: D3D11_BUS_TYPE = 196608;
pub type D3D11_BUS_TYPE = i32;
pub const D3D11_BUS_TYPE_AGP: D3D11_BUS_TYPE = 4;
pub const D3D11_BUS_TYPE_OTHER: D3D11_BUS_TYPE = 0;
pub const D3D11_BUS_TYPE_PCI: D3D11_BUS_TYPE = 1;
pub const D3D11_BUS_TYPE_PCIEXPRESS: D3D11_BUS_TYPE = 3;
pub const D3D11_BUS_TYPE_PCIX: D3D11_BUS_TYPE = 2;
pub const D3D11_CENTER_MULTISAMPLE_PATTERN: D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_CLASS_INSTANCE_DESC {
    pub InstanceId: u32,
    pub InstanceIndex: u32,
    pub TypeId: u32,
    pub ConstantBuffer: u32,
    pub BaseConstantBufferOffset: u32,
    pub BaseTexture: u32,
    pub BaseSampler: u32,
    pub Created: windows_core::BOOL,
}
pub const D3D11_CLEAR_DEPTH: D3D11_CLEAR_FLAG = 1;
pub type D3D11_CLEAR_FLAG = i32;
pub const D3D11_CLEAR_STENCIL: D3D11_CLEAR_FLAG = 2;
pub const D3D11_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8;
pub const D3D11_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2;
pub type D3D11_COLOR_WRITE_ENABLE = i32;
pub const D3D11_COLOR_WRITE_ENABLE_ALL: D3D11_COLOR_WRITE_ENABLE = 15;
pub const D3D11_COLOR_WRITE_ENABLE_ALPHA: D3D11_COLOR_WRITE_ENABLE = 8;
pub const D3D11_COLOR_WRITE_ENABLE_BLUE: D3D11_COLOR_WRITE_ENABLE = 4;
pub const D3D11_COLOR_WRITE_ENABLE_GREEN: D3D11_COLOR_WRITE_ENABLE = 2;
pub const D3D11_COLOR_WRITE_ENABLE_RED: D3D11_COLOR_WRITE_ENABLE = 1;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: u32 = 16;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16;
pub const D3D11_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3;
pub const D3D11_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10;
pub const D3D11_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10;
pub const D3D11_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8;
pub const D3D11_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7;
pub const D3D11_COMPARISON_ALWAYS: D3D11_COMPARISON_FUNC = 8;
pub const D3D11_COMPARISON_EQUAL: D3D11_COMPARISON_FUNC = 3;
pub const D3D11_COMPARISON_FILTERING_BIT: u32 = 128;
pub type D3D11_COMPARISON_FUNC = i32;
pub const D3D11_COMPARISON_GREATER: D3D11_COMPARISON_FUNC = 5;
pub const D3D11_COMPARISON_GREATER_EQUAL: D3D11_COMPARISON_FUNC = 7;
pub const D3D11_COMPARISON_LESS: D3D11_COMPARISON_FUNC = 2;
pub const D3D11_COMPARISON_LESS_EQUAL: D3D11_COMPARISON_FUNC = 4;
pub const D3D11_COMPARISON_NEVER: D3D11_COMPARISON_FUNC = 1;
pub const D3D11_COMPARISON_NOT_EQUAL: D3D11_COMPARISON_FUNC = 6;
pub const D3D11_CONSERVATIVE_RASTERIZATION_NOT_SUPPORTED: D3D11_CONSERVATIVE_RASTERIZATION_TIER = 0;
pub type D3D11_CONSERVATIVE_RASTERIZATION_TIER = i32;
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_1: D3D11_CONSERVATIVE_RASTERIZATION_TIER = 1;
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_2: D3D11_CONSERVATIVE_RASTERIZATION_TIER = 2;
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_3: D3D11_CONSERVATIVE_RASTERIZATION_TIER = 3;
pub type D3D11_CONTENT_PROTECTION_CAPS = i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_CONTENT_KEY: D3D11_CONTENT_PROTECTION_CAPS = 16;
pub const D3D11_CONTENT_PROTECTION_CAPS_DECRYPTION_BLT: D3D11_CONTENT_PROTECTION_CAPS = 1024;
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK: D3D11_CONTENT_PROTECTION_CAPS = 64;
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK_KEY: D3D11_CONTENT_PROTECTION_CAPS = 128;
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPT_SLICEDATA_ONLY: D3D11_CONTENT_PROTECTION_CAPS = 512;
pub const D3D11_CONTENT_PROTECTION_CAPS_FRESHEN_SESSION_KEY: D3D11_CONTENT_PROTECTION_CAPS = 32;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE: D3D11_CONTENT_PROTECTION_CAPS = 2;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION: D3D11_CONTENT_PROTECTION_CAPS = 16384;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION_MULTI_THREADED: D3D11_CONTENT_PROTECTION_CAPS = 32768;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_PROTECTED_MEMORY_PAGEABLE: D3D11_CONTENT_PROTECTION_CAPS = 4096;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_PROTECT_UNCOMPRESSED: D3D11_CONTENT_PROTECTION_CAPS = 2048;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_TEARDOWN: D3D11_CONTENT_PROTECTION_CAPS = 8192;
pub const D3D11_CONTENT_PROTECTION_CAPS_PARTIAL_DECRYPTION: D3D11_CONTENT_PROTECTION_CAPS = 8;
pub const D3D11_CONTENT_PROTECTION_CAPS_PROTECTION_ALWAYS_ON: D3D11_CONTENT_PROTECTION_CAPS = 4;
pub const D3D11_CONTENT_PROTECTION_CAPS_SEQUENTIAL_CTR_IV: D3D11_CONTENT_PROTECTION_CAPS = 256;
pub const D3D11_CONTENT_PROTECTION_CAPS_SOFTWARE: D3D11_CONTENT_PROTECTION_CAPS = 1;
pub type D3D11_COUNTER = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_COUNTER_DESC {
    pub Counter: D3D11_COUNTER,
    pub MiscFlags: u32,
}
pub const D3D11_COUNTER_DEVICE_DEPENDENT_0: D3D11_COUNTER = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D11_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
pub type D3D11_COUNTER_TYPE = i32;
pub const D3D11_COUNTER_TYPE_FLOAT32: D3D11_COUNTER_TYPE = 0;
pub const D3D11_COUNTER_TYPE_UINT16: D3D11_COUNTER_TYPE = 1;
pub const D3D11_COUNTER_TYPE_UINT32: D3D11_COUNTER_TYPE = 2;
pub const D3D11_COUNTER_TYPE_UINT64: D3D11_COUNTER_TYPE = 3;
pub type D3D11_CPU_ACCESS_FLAG = i32;
pub const D3D11_CPU_ACCESS_READ: D3D11_CPU_ACCESS_FLAG = 131072;
pub const D3D11_CPU_ACCESS_WRITE: D3D11_CPU_ACCESS_FLAG = 65536;
pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 32;
pub const D3D11_CREATE_DEVICE_DEBUG: D3D11_CREATE_DEVICE_FLAG = 2;
pub const D3D11_CREATE_DEVICE_DEBUGGABLE: D3D11_CREATE_DEVICE_FLAG = 64;
pub const D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT: D3D11_CREATE_DEVICE_FLAG = 256;
pub type D3D11_CREATE_DEVICE_FLAG = i32;
pub const D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D11_CREATE_DEVICE_FLAG = 128;
pub const D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D11_CREATE_DEVICE_FLAG = 8;
pub const D3D11_CREATE_DEVICE_SINGLETHREADED: D3D11_CREATE_DEVICE_FLAG = 1;
pub const D3D11_CREATE_DEVICE_SWITCH_TO_REF: D3D11_CREATE_DEVICE_FLAG = 4;
pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 2048;
pub const D3D11_CRYPTO_TYPE_AES128_CTR: windows_core::GUID = windows_core::GUID::from_u128(0x9b6bd711_4f74_41c9_9e7b_0be2d7d93b4f);
pub const D3D11_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 256;
pub const D3D11_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: u32 = 64;
pub const D3D11_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 240;
pub const D3D11_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: u32 = 68;
pub const D3D11_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 224;
pub const D3D11_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: u32 = 72;
pub const D3D11_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 208;
pub const D3D11_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: u32 = 76;
pub const D3D11_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 192;
pub const D3D11_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: u32 = 84;
pub const D3D11_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 176;
pub const D3D11_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: u32 = 92;
pub const D3D11_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 160;
pub const D3D11_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: u32 = 100;
pub const D3D11_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 144;
pub const D3D11_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: u32 = 112;
pub const D3D11_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 128;
pub const D3D11_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: u32 = 128;
pub const D3D11_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 112;
pub const D3D11_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: u32 = 144;
pub const D3D11_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 96;
pub const D3D11_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: u32 = 168;
pub const D3D11_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 80;
pub const D3D11_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: u32 = 204;
pub const D3D11_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 64;
pub const D3D11_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: u32 = 256;
pub const D3D11_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 48;
pub const D3D11_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: u32 = 340;
pub const D3D11_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 32;
pub const D3D11_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: u32 = 512;
pub const D3D11_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 16;
pub const D3D11_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: u32 = 768;
pub const D3D11_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: u32 = 1;
pub const D3D11_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: u32 = 256;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 768;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_X: u32 = 768;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_Y: u32 = 768;
pub const D3D11_CS_4_X_UAV_REGISTER_COUNT: u32 = 1;
pub const D3D11_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: u32 = 65535;
pub const D3D11_CS_TGSM_REGISTER_COUNT: u32 = 8192;
pub const D3D11_CS_TGSM_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D11_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_CS_THREADGROUPID_REGISTER_COMPONENTS: u32 = 3;
pub const D3D11_CS_THREADGROUPID_REGISTER_COUNT: u32 = 1;
pub const D3D11_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: u32 = 1;
pub const D3D11_CS_THREADIDINGROUP_REGISTER_COMPONENTS: u32 = 3;
pub const D3D11_CS_THREADIDINGROUP_REGISTER_COUNT: u32 = 1;
pub const D3D11_CS_THREADID_REGISTER_COMPONENTS: u32 = 3;
pub const D3D11_CS_THREADID_REGISTER_COUNT: u32 = 1;
pub const D3D11_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 1024;
pub const D3D11_CS_THREAD_GROUP_MAX_X: u32 = 1024;
pub const D3D11_CS_THREAD_GROUP_MAX_Y: u32 = 1024;
pub const D3D11_CS_THREAD_GROUP_MAX_Z: u32 = 64;
pub const D3D11_CS_THREAD_GROUP_MIN_X: u32 = 1;
pub const D3D11_CS_THREAD_GROUP_MIN_Y: u32 = 1;
pub const D3D11_CS_THREAD_GROUP_MIN_Z: u32 = 1;
pub const D3D11_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: u32 = 16384;
pub const D3D11_CULL_BACK: D3D11_CULL_MODE = 3;
pub const D3D11_CULL_FRONT: D3D11_CULL_MODE = 2;
pub type D3D11_CULL_MODE = i32;
pub const D3D11_CULL_NONE: D3D11_CULL_MODE = 1;
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CBCS: windows_core::GUID = windows_core::GUID::from_u128(0x422d9319_9d21_4bb7_9371_faf5a82c3e04);
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CENC: windows_core::GUID = windows_core::GUID::from_u128(0xb0405235_c13d_44f2_9ae5_dd48e08e5b67);
pub const D3D11_DECODER_ENCRYPTION_HW_CENC: windows_core::GUID = windows_core::GUID::from_u128(0x89d6ac4f_09f2_4229_b2cd_37740a6dfd81);
pub const D3D11_DECODER_PROFILE_APV_VLD_400_10: windows_core::GUID = windows_core::GUID::from_u128(0x37148862_6bd6_4618_8293_777b686b0824);
pub const D3D11_DECODER_PROFILE_APV_VLD_422_10: windows_core::GUID = windows_core::GUID::from_u128(0x226a709d_ae12_44c5_ba21_164feeb7f9b6);
pub const D3D11_DECODER_PROFILE_APV_VLD_422_12: windows_core::GUID = windows_core::GUID::from_u128(0xf6f152ad_94e5_4bfa_9227_676cddeff42b);
pub const D3D11_DECODER_PROFILE_APV_VLD_4444_10: windows_core::GUID = windows_core::GUID::from_u128(0xc83799b9_9655_4b95_8008_56a322ce5d81);
pub const D3D11_DECODER_PROFILE_APV_VLD_4444_12: windows_core::GUID = windows_core::GUID::from_u128(0x6a763ee3_4d05_47fe_a429_723474b69d7c);
pub const D3D11_DECODER_PROFILE_APV_VLD_444_10: windows_core::GUID = windows_core::GUID::from_u128(0x6a4a8d7d_7610_469f_855f_39f13051c013);
pub const D3D11_DECODER_PROFILE_APV_VLD_444_12: windows_core::GUID = windows_core::GUID::from_u128(0xf1039a1c_e208_45c1_952c_040841b67667);
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0x17127009_a00f_4ce1_994e_bf4081f6f3f0);
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2_420: windows_core::GUID = windows_core::GUID::from_u128(0x2d80bed6_9cac_4835_9e91_327bbc4f9ee8);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE0: windows_core::GUID = windows_core::GUID::from_u128(0xb8be4ccb_cf53_46ba_8d59_d6b8a6da5d2a);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE1: windows_core::GUID = windows_core::GUID::from_u128(0x6936ff0f_45b1_4163_9cc1_646ef6946108);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0x0c5f2aa1_e541_4089_bb7b_98110a19d7c8);
pub const D3D11_DECODER_PROFILE_H264_IDCT_FGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be67_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_IDCT_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be66_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_FGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be65_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be64_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_FGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be69_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_MULTIVIEW_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const D3D11_DECODER_PROFILE_H264_VLD_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_PROGRESSIVE_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const D3D11_DECODER_PROFILE_H264_VLD_WITHFMOASO_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0xd5f04ff9_3418_45d8_9561_32a76aae2ddd);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN: windows_core::GUID = windows_core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10: windows_core::GUID = windows_core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10_422: windows_core::GUID = windows_core::GUID::from_u128(0x0bac4fe5_1532_4429_a854_f84de04953db);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10_444: windows_core::GUID = windows_core::GUID::from_u128(0x0dabeffa_4458_4602_bc03_0795659d617c);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10_EXT: windows_core::GUID = windows_core::GUID::from_u128(0x9cc55490_e37c_4932_8684_4920f9f6409c);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN12: windows_core::GUID = windows_core::GUID::from_u128(0x1a72925f_0c2c_4f15_96fb_b17d1473603f);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN12_422: windows_core::GUID = windows_core::GUID::from_u128(0x55bcac81_f311_4093_a7d0_1cbc0b849bee);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN12_444: windows_core::GUID = windows_core::GUID::from_u128(0x9798634d_fe9d_48e5_b4da_dbec45b3df01);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN16: windows_core::GUID = windows_core::GUID::from_u128(0xa4fbdbb0_a113_482b_a232_635cc0697f6d);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN_444: windows_core::GUID = windows_core::GUID::from_u128(0x4008018f_f537_4b36_98cf_61af8a2c1a33);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MONOCHROME: windows_core::GUID = windows_core::GUID::from_u128(0x0685b993_3d8c_43a0_8b28_d74c2d6899a4);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MONOCHROME10: windows_core::GUID = windows_core::GUID::from_u128(0x142a1d0f_69dd_4ec9_8591_b12ffcb91a29);
pub const D3D11_DECODER_PROFILE_JPEG_VLD_420: windows_core::GUID = windows_core::GUID::from_u128(0xcf782c83_bef5_4a2c_87cb_6019e7b175ac);
pub const D3D11_DECODER_PROFILE_JPEG_VLD_422: windows_core::GUID = windows_core::GUID::from_u128(0xf04df417_eee2_4067_a778_f35c15ab9721);
pub const D3D11_DECODER_PROFILE_JPEG_VLD_444: windows_core::GUID = windows_core::GUID::from_u128(0x4cd00e17_89ba_48ef_b9f9_edcb82713f65);
pub const D3D11_DECODER_PROFILE_MJPEG_VLD_420: windows_core::GUID = windows_core::GUID::from_u128(0x725cb506_0c29_43c4_9440_8e9397903a04);
pub const D3D11_DECODER_PROFILE_MJPEG_VLD_422: windows_core::GUID = windows_core::GUID::from_u128(0x5b77b9cd_1a35_4c30_9fd8_ef4b60c035dd);
pub const D3D11_DECODER_PROFILE_MJPEG_VLD_444: windows_core::GUID = windows_core::GUID::from_u128(0xd95161f9_0d44_47e6_bcf5_1bfbfb268f97);
pub const D3D11_DECODER_PROFILE_MJPEG_VLD_4444: windows_core::GUID = windows_core::GUID::from_u128(0xc91748d5_fd18_4aca_9db3_3a6634ab547d);
pub const D3D11_DECODER_PROFILE_MPEG1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x6f3ec719_3735_42cc_8063_65cc3cb36616);
pub const D3D11_DECODER_PROFILE_MPEG2_IDCT: windows_core::GUID = windows_core::GUID::from_u128(0xbf22ad00_03ea_4690_8077_473346209b7e);
pub const D3D11_DECODER_PROFILE_MPEG2_MOCOMP: windows_core::GUID = windows_core::GUID::from_u128(0xe6a9f44b_61b0_4563_9ea4_63d2a3c6fe66);
pub const D3D11_DECODER_PROFILE_MPEG2_VLD: windows_core::GUID = windows_core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const D3D11_DECODER_PROFILE_MPEG2and1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_GMC: windows_core::GUID = windows_core::GUID::from_u128(0xab998b5b_4258_44a9_9feb_94e597a6baae);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_NOGMC: windows_core::GUID = windows_core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_SIMPLE: windows_core::GUID = windows_core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const D3D11_DECODER_PROFILE_VC1_D2010: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_IDCT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea2_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_MOCOMP: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea1_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_POSTPROC: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea0_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VP8_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const D3D11_DECODER_PROFILE_VP9_VLD_10BIT_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
pub const D3D11_DECODER_PROFILE_VP9_VLD_PROFILE0: windows_core::GUID = windows_core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const D3D11_DECODER_PROFILE_WMV8_MOCOMP: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be81_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV8_POSTPROC: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be80_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_IDCT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be94_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_MOCOMP: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be91_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_POSTPROC: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be90_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DEFAULT_DEPTH_BIAS: u32 = 0;
pub const D3D11_DEFAULT_MAX_ANISOTROPY: u32 = 16;
pub const D3D11_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0;
pub const D3D11_DEFAULT_SAMPLE_MASK: u32 = 4294967295;
pub const D3D11_DEFAULT_SCISSOR_ENDX: u32 = 0;
pub const D3D11_DEFAULT_SCISSOR_ENDY: u32 = 0;
pub const D3D11_DEFAULT_SCISSOR_STARTX: u32 = 0;
pub const D3D11_DEFAULT_SCISSOR_STARTY: u32 = 0;
pub const D3D11_DEFAULT_STENCIL_READ_MASK: u32 = 255;
pub const D3D11_DEFAULT_STENCIL_REFERENCE: u32 = 0;
pub const D3D11_DEFAULT_STENCIL_WRITE_MASK: u32 = 255;
pub const D3D11_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0;
pub const D3D11_DEFAULT_VIEWPORT_HEIGHT: u32 = 0;
pub const D3D11_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0;
pub const D3D11_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0;
pub const D3D11_DEFAULT_VIEWPORT_WIDTH: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D11_STENCIL_OP,
    pub StencilDepthFailOp: D3D11_STENCIL_OP,
    pub StencilPassOp: D3D11_STENCIL_OP,
    pub StencilFunc: D3D11_COMPARISON_FUNC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: windows_core::BOOL,
    pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D11_COMPARISON_FUNC,
    pub StencilEnable: windows_core::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D11_DSV_DIMENSION,
    pub Flags: u32,
    pub Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D11_TEX1D_DSV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D11_TEX2D_DSV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D11_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_DEPTH_WRITE_MASK = i32;
pub const D3D11_DEPTH_WRITE_MASK_ALL: D3D11_DEPTH_WRITE_MASK = 1;
pub const D3D11_DEPTH_WRITE_MASK_ZERO: D3D11_DEPTH_WRITE_MASK = 0;
pub const D3D11_DEVICE_CONTEXT_DEFERRED: D3D11_DEVICE_CONTEXT_TYPE = 1;
pub const D3D11_DEVICE_CONTEXT_IMMEDIATE: D3D11_DEVICE_CONTEXT_TYPE = 0;
pub type D3D11_DEVICE_CONTEXT_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    pub IndexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartIndexLocation: u32,
    pub BaseVertexLocation: i32,
    pub StartInstanceLocation: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    pub VertexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartVertexLocation: u32,
    pub StartInstanceLocation: u32,
}
pub type D3D11_DSV_DIMENSION = i32;
pub const D3D11_DSV_DIMENSION_TEXTURE1D: D3D11_DSV_DIMENSION = 1;
pub const D3D11_DSV_DIMENSION_TEXTURE1DARRAY: D3D11_DSV_DIMENSION = 2;
pub const D3D11_DSV_DIMENSION_TEXTURE2D: D3D11_DSV_DIMENSION = 3;
pub const D3D11_DSV_DIMENSION_TEXTURE2DARRAY: D3D11_DSV_DIMENSION = 4;
pub const D3D11_DSV_DIMENSION_TEXTURE2DMS: D3D11_DSV_DIMENSION = 5;
pub const D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D11_DSV_DIMENSION = 6;
pub const D3D11_DSV_DIMENSION_UNKNOWN: D3D11_DSV_DIMENSION = 0;
pub type D3D11_DSV_FLAG = i32;
pub const D3D11_DSV_READ_ONLY_DEPTH: D3D11_DSV_FLAG = 1;
pub const D3D11_DSV_READ_ONLY_STENCIL: D3D11_DSV_FLAG = 2;
pub const D3D11_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: u32 = 32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: u32 = 3;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: u32 = 1;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_DS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_DS_OUTPUT_REGISTER_COUNT: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_ENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: u32,
    pub NumBytesInSkipPattern: u32,
    pub NumBytesInEncryptPattern: u32,
}
pub type D3D11_FEATURE = i32;
pub const D3D11_FEATURE_ARCHITECTURE_INFO: D3D11_FEATURE = 6;
pub const D3D11_FEATURE_D3D10_X_HARDWARE_OPTIONS: D3D11_FEATURE = 4;
pub const D3D11_FEATURE_D3D11_OPTIONS: D3D11_FEATURE = 5;
pub const D3D11_FEATURE_D3D11_OPTIONS1: D3D11_FEATURE = 10;
pub const D3D11_FEATURE_D3D11_OPTIONS2: D3D11_FEATURE = 14;
pub const D3D11_FEATURE_D3D11_OPTIONS3: D3D11_FEATURE = 15;
pub const D3D11_FEATURE_D3D11_OPTIONS4: D3D11_FEATURE = 17;
pub const D3D11_FEATURE_D3D11_OPTIONS5: D3D11_FEATURE = 19;
pub const D3D11_FEATURE_D3D11_OPTIONS6: D3D11_FEATURE = 21;
pub const D3D11_FEATURE_D3D9_OPTIONS: D3D11_FEATURE = 7;
pub const D3D11_FEATURE_D3D9_OPTIONS1: D3D11_FEATURE = 13;
pub const D3D11_FEATURE_D3D9_SHADOW_SUPPORT: D3D11_FEATURE = 9;
pub const D3D11_FEATURE_D3D9_SIMPLE_INSTANCING_SUPPORT: D3D11_FEATURE = 11;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    pub TileBasedDeferredRenderer: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    pub ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS {
    pub OutputMergerLogicOp: windows_core::BOOL,
    pub UAVOnlyRenderingForcedSampleCount: windows_core::BOOL,
    pub DiscardAPIsSeenByDriver: windows_core::BOOL,
    pub FlagsForUpdateAndCopySeenByDriver: windows_core::BOOL,
    pub ClearView: windows_core::BOOL,
    pub CopyWithOverlap: windows_core::BOOL,
    pub ConstantBufferPartialUpdate: windows_core::BOOL,
    pub ConstantBufferOffsetting: windows_core::BOOL,
    pub MapNoOverwriteOnDynamicConstantBuffer: windows_core::BOOL,
    pub MapNoOverwriteOnDynamicBufferSRV: windows_core::BOOL,
    pub MultisampleRTVWithForcedSampleCountOne: windows_core::BOOL,
    pub SAD4ShaderInstructions: windows_core::BOOL,
    pub ExtendedDoublesShaderInstructions: windows_core::BOOL,
    pub ExtendedResourceSharing: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
    pub MinMaxFiltering: windows_core::BOOL,
    pub ClearViewAlsoSupportsDepthOnlyFormats: windows_core::BOOL,
    pub MapOnDefaultBuffers: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    pub PSSpecifiedStencilRefSupported: windows_core::BOOL,
    pub TypedUAVLoadAdditionalFormats: windows_core::BOOL,
    pub ROVsSupported: windows_core::BOOL,
    pub ConservativeRasterizationTier: D3D11_CONSERVATIVE_RASTERIZATION_TIER,
    pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
    pub MapOnDefaultTextures: windows_core::BOOL,
    pub StandardSwizzle: windows_core::BOOL,
    pub UnifiedMemoryArchitecture: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    pub VPAndRTArrayIndexFromAnyShaderFeedingRasterizer: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    pub SharedResourceTier: D3D11_SHARED_RESOURCE_TIER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS6 {
    pub ShaderAccessRestrictedResourceTier: D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS {
    pub FullNonPow2TextureSupport: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    pub FullNonPow2TextureSupported: windows_core::BOOL,
    pub DepthAsTextureWithLessEqualComparisonFilterSupported: windows_core::BOOL,
    pub SimpleInstancingSupported: windows_core::BOOL,
    pub TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    pub SupportsDepthAsTextureWithLessEqualComparisonFilter: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    pub SimpleInstancingSupported: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_DISPLAYABLE {
    pub DisplayableTexture: windows_core::BOOL,
    pub SharedResourceTier: D3D11_SHARED_RESOURCE_TIER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_DOUBLES {
    pub DoublePrecisionFloatShaderOps: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    pub InFormat: super::dxgiformat::DXGI_FORMAT,
    pub OutFormatSupport: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    pub InFormat: super::dxgiformat::DXGI_FORMAT,
    pub OutFormatSupport2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub MaxGPUVirtualAddressBitsPerProcess: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_MARKER_SUPPORT {
    pub Profile: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_SHADER_CACHE {
    pub SupportFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    pub PixelShaderMinPrecision: u32,
    pub AllOtherShaderStagesMinPrecision: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_THREADING {
    pub DriverConcurrentCreates: windows_core::BOOL,
    pub DriverCommandLists: windows_core::BOOL,
}
pub const D3D11_FEATURE_DISPLAYABLE: D3D11_FEATURE = 20;
pub const D3D11_FEATURE_DOUBLES: D3D11_FEATURE = 1;
pub const D3D11_FEATURE_FORMAT_SUPPORT: D3D11_FEATURE = 2;
pub const D3D11_FEATURE_FORMAT_SUPPORT2: D3D11_FEATURE = 3;
pub const D3D11_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT: D3D11_FEATURE = 16;
pub const D3D11_FEATURE_MARKER_SUPPORT: D3D11_FEATURE = 12;
pub const D3D11_FEATURE_SHADER_CACHE: D3D11_FEATURE = 18;
pub const D3D11_FEATURE_SHADER_MIN_PRECISION_SUPPORT: D3D11_FEATURE = 8;
pub const D3D11_FEATURE_THREADING: D3D11_FEATURE = 0;
pub type D3D11_FILL_MODE = i32;
pub const D3D11_FILL_SOLID: D3D11_FILL_MODE = 3;
pub const D3D11_FILL_WIREFRAME: D3D11_FILL_MODE = 2;
pub type D3D11_FILTER = i32;
pub const D3D11_FILTER_ANISOTROPIC: D3D11_FILTER = 85;
pub const D3D11_FILTER_COMPARISON_ANISOTROPIC: D3D11_FILTER = 213;
pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = 144;
pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 145;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 148;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 149;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D11_FILTER = 128;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 129;
pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 132;
pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = 133;
pub const D3D11_FILTER_MAXIMUM_ANISOTROPIC: D3D11_FILTER = 469;
pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = 400;
pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 401;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 404;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 405;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_POINT: D3D11_FILTER = 384;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 385;
pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 388;
pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = 389;
pub const D3D11_FILTER_MINIMUM_ANISOTROPIC: D3D11_FILTER = 341;
pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = 272;
pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 273;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 276;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 277;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_POINT: D3D11_FILTER = 256;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 257;
pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 260;
pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = 261;
pub const D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = 16;
pub const D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 17;
pub const D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 20;
pub const D3D11_FILTER_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 21;
pub const D3D11_FILTER_MIN_MAG_MIP_POINT: D3D11_FILTER = 0;
pub const D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = 1;
pub const D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = 4;
pub const D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = 5;
pub type D3D11_FILTER_REDUCTION_TYPE = i32;
pub const D3D11_FILTER_REDUCTION_TYPE_COMPARISON: D3D11_FILTER_REDUCTION_TYPE = 1;
pub const D3D11_FILTER_REDUCTION_TYPE_MASK: u32 = 3;
pub const D3D11_FILTER_REDUCTION_TYPE_MAXIMUM: D3D11_FILTER_REDUCTION_TYPE = 3;
pub const D3D11_FILTER_REDUCTION_TYPE_MINIMUM: D3D11_FILTER_REDUCTION_TYPE = 2;
pub const D3D11_FILTER_REDUCTION_TYPE_SHIFT: u32 = 7;
pub const D3D11_FILTER_REDUCTION_TYPE_STANDARD: D3D11_FILTER_REDUCTION_TYPE = 0;
pub type D3D11_FILTER_TYPE = i32;
pub const D3D11_FILTER_TYPE_LINEAR: D3D11_FILTER_TYPE = 1;
pub const D3D11_FILTER_TYPE_MASK: u32 = 3;
pub const D3D11_FILTER_TYPE_POINT: D3D11_FILTER_TYPE = 0;
pub type D3D11_FORMAT_SUPPORT = i32;
pub type D3D11_FORMAT_SUPPORT2 = i32;
pub const D3D11_FORMAT_SUPPORT2_DISPLAYABLE: D3D11_FORMAT_SUPPORT2 = 65536;
pub const D3D11_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY: D3D11_FORMAT_SUPPORT2 = 16384;
pub const D3D11_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP: D3D11_FORMAT_SUPPORT2 = 256;
pub const D3D11_FORMAT_SUPPORT2_SHAREABLE: D3D11_FORMAT_SUPPORT2 = 1024;
pub const D3D11_FORMAT_SUPPORT2_TILED: D3D11_FORMAT_SUPPORT2 = 512;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_ADD: D3D11_FORMAT_SUPPORT2 = 1;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS: D3D11_FORMAT_SUPPORT2 = 2;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE: D3D11_FORMAT_SUPPORT2 = 4;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE: D3D11_FORMAT_SUPPORT2 = 8;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX: D3D11_FORMAT_SUPPORT2 = 16;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX: D3D11_FORMAT_SUPPORT2 = 32;
pub const D3D11_FORMAT_SUPPORT2_UAV_TYPED_LOAD: D3D11_FORMAT_SUPPORT2 = 64;
pub const D3D11_FORMAT_SUPPORT2_UAV_TYPED_STORE: D3D11_FORMAT_SUPPORT2 = 128;
pub const D3D11_FORMAT_SUPPORT_BACK_BUFFER_CAST: D3D11_FORMAT_SUPPORT = 16777216;
pub const D3D11_FORMAT_SUPPORT_BLENDABLE: D3D11_FORMAT_SUPPORT = 32768;
pub const D3D11_FORMAT_SUPPORT_BUFFER: D3D11_FORMAT_SUPPORT = 1;
pub const D3D11_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT: D3D11_FORMAT_SUPPORT = 1048576;
pub const D3D11_FORMAT_SUPPORT_CPU_LOCKABLE: D3D11_FORMAT_SUPPORT = 131072;
pub const D3D11_FORMAT_SUPPORT_DECODER_OUTPUT: D3D11_FORMAT_SUPPORT = 134217728;
pub const D3D11_FORMAT_SUPPORT_DEPTH_STENCIL: D3D11_FORMAT_SUPPORT = 65536;
pub const D3D11_FORMAT_SUPPORT_DISPLAY: D3D11_FORMAT_SUPPORT = 524288;
pub const D3D11_FORMAT_SUPPORT_IA_INDEX_BUFFER: D3D11_FORMAT_SUPPORT = 4;
pub const D3D11_FORMAT_SUPPORT_IA_VERTEX_BUFFER: D3D11_FORMAT_SUPPORT = 2;
pub const D3D11_FORMAT_SUPPORT_MIP: D3D11_FORMAT_SUPPORT = 4096;
pub const D3D11_FORMAT_SUPPORT_MIP_AUTOGEN: D3D11_FORMAT_SUPPORT = 8192;
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_LOAD: D3D11_FORMAT_SUPPORT = 4194304;
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET: D3D11_FORMAT_SUPPORT = 2097152;
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE: D3D11_FORMAT_SUPPORT = 262144;
pub const D3D11_FORMAT_SUPPORT_RENDER_TARGET: D3D11_FORMAT_SUPPORT = 16384;
pub const D3D11_FORMAT_SUPPORT_SHADER_GATHER: D3D11_FORMAT_SUPPORT = 8388608;
pub const D3D11_FORMAT_SUPPORT_SHADER_GATHER_COMPARISON: D3D11_FORMAT_SUPPORT = 67108864;
pub const D3D11_FORMAT_SUPPORT_SHADER_LOAD: D3D11_FORMAT_SUPPORT = 256;
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE: D3D11_FORMAT_SUPPORT = 512;
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON: D3D11_FORMAT_SUPPORT = 1024;
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT: D3D11_FORMAT_SUPPORT = 2048;
pub const D3D11_FORMAT_SUPPORT_SO_BUFFER: D3D11_FORMAT_SUPPORT = 8;
pub const D3D11_FORMAT_SUPPORT_TEXTURE1D: D3D11_FORMAT_SUPPORT = 16;
pub const D3D11_FORMAT_SUPPORT_TEXTURE2D: D3D11_FORMAT_SUPPORT = 32;
pub const D3D11_FORMAT_SUPPORT_TEXTURE3D: D3D11_FORMAT_SUPPORT = 64;
pub const D3D11_FORMAT_SUPPORT_TEXTURECUBE: D3D11_FORMAT_SUPPORT = 128;
pub const D3D11_FORMAT_SUPPORT_TYPED_UNORDERED_ACCESS_VIEW: D3D11_FORMAT_SUPPORT = 33554432;
pub const D3D11_FORMAT_SUPPORT_VIDEO_ENCODER: D3D11_FORMAT_SUPPORT = 1073741824;
pub const D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_INPUT: D3D11_FORMAT_SUPPORT = 536870912;
pub const D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_OUTPUT: D3D11_FORMAT_SUPPORT = 268435456;
pub const D3D11_GS_INPUT_INSTANCE_ID_READS_PER_INST: u32 = 2;
pub const D3D11_GS_INPUT_INSTANCE_ID_READ_PORTS: u32 = 1;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: u32 = 1;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_GS_INPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_GS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D11_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_GS_INPUT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_GS_INPUT_REGISTER_VERTICES: u32 = 32;
pub const D3D11_GS_MAX_INSTANCE_COUNT: u32 = 32;
pub const D3D11_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: u32 = 1024;
pub const D3D11_GS_OUTPUT_ELEMENTS: u32 = 32;
pub const D3D11_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_GS_OUTPUT_REGISTER_COUNT: u32 = 32;
pub const D3D11_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D11_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: u32 = 32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_HS_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: u32 = 1;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: u32 = 1;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295;
pub const D3D11_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: u32 = 1;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: u32 = 128;
pub const D3D11_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0;
pub const D3D11_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0;
pub const D3D11_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0;
pub const D3D11_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1;
pub const D3D11_IA_INSTANCE_ID_BIT_COUNT: u32 = 32;
pub const D3D11_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32;
pub const D3D11_IA_PATCH_MAX_CONTROL_POINT_COUNT: u32 = 32;
pub const D3D11_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32;
pub const D3D11_IA_VERTEX_ID_BIT_COUNT: u32 = 32;
pub const D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32;
pub const D3D11_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128;
pub const D3D11_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32;
pub type D3D11_INPUT_CLASSIFICATION = i32;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
pub const D3D11_INPUT_PER_INSTANCE_DATA: D3D11_INPUT_CLASSIFICATION = 1;
pub const D3D11_INPUT_PER_VERTEX_DATA: D3D11_INPUT_CLASSIFICATION = 0;
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295;
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295;
pub const D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295;
pub const D3D11_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295;
pub const D3D11_KEY_EXCHANGE_HW_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0xb1170d8a_628d_4da3_ad3b_82ddb08b4970);
pub const D3D11_KEY_EXCHANGE_RSAES_OAEP: windows_core::GUID = windows_core::GUID::from_u128(0xc1949895_d72a_4a1d_8e5d_ed857d171520);
pub const D3D11_MAG_FILTER_SHIFT: u32 = 2;
pub const D3D11_MAJOR_VERSION: u32 = 11;
pub type D3D11_MAP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl Default for D3D11_MAPPED_SUBRESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_MAP_FLAG = i32;
pub const D3D11_MAP_FLAG_DO_NOT_WAIT: D3D11_MAP_FLAG = 1048576;
pub const D3D11_MAP_READ: D3D11_MAP = 1;
pub const D3D11_MAP_READ_WRITE: D3D11_MAP = 3;
pub const D3D11_MAP_WRITE: D3D11_MAP = 2;
pub const D3D11_MAP_WRITE_DISCARD: D3D11_MAP = 4;
pub const D3D11_MAP_WRITE_NO_OVERWRITE: D3D11_MAP = 5;
pub const D3D11_MAX_MAXANISOTROPY: u32 = 16;
pub const D3D11_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32;
pub const D3D11_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17;
pub const D3D11_MINOR_VERSION: u32 = 0;
pub const D3D11_MIN_FILTER_SHIFT: u32 = 4;
pub const D3D11_MIN_MAXANISOTROPY: u32 = 0;
pub const D3D11_MIP_FILTER_SHIFT: u32 = 0;
pub const D3D11_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8;
pub const D3D11_MIP_LOD_RANGE_BIT_COUNT: u32 = 8;
pub const D3D11_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_OMAC {
    pub Omac: [u8; 16],
}
impl Default for D3D11_OMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15;
pub const D3D11_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16;
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_PRIMITIVE(pub super::d3dcommon::D3D_PRIMITIVE);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_PRIMITIVE_TOPOLOGY(pub super::d3dcommon::D3D_PRIMITIVE_TOPOLOGY);
pub const D3D11_PROCESSIDTYPE_DWM: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE = 1;
pub const D3D11_PROCESSIDTYPE_HANDLE: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE = 2;
pub const D3D11_PROCESSIDTYPE_UNKNOWN: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE = 0;
pub const D3D11_PS_CS_UAV_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_PS_CS_UAV_REGISTER_COUNT: u32 = 8;
pub const D3D11_PS_CS_UAV_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D11_PS_CS_UAV_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295;
pub const D3D11_PS_FRONTFACING_FALSE_VALUE: u32 = 0;
pub const D3D11_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295;
pub const D3D11_PS_INPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_PS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D11_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_PS_INPUT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1;
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_PS_OUTPUT_REGISTER_COUNT: u32 = 8;
pub type D3D11_QUERY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
    pub HSInvocations: u64,
    pub DSInvocations: u64,
    pub CSInvocations: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_QUERY_DESC {
    pub Query: D3D11_QUERY,
    pub MiscFlags: u32,
}
pub const D3D11_QUERY_EVENT: D3D11_QUERY = 0;
pub type D3D11_QUERY_MISC_FLAG = i32;
pub const D3D11_QUERY_MISC_PREDICATEHINT: D3D11_QUERY_MISC_FLAG = 1;
pub const D3D11_QUERY_OCCLUSION: D3D11_QUERY = 1;
pub const D3D11_QUERY_OCCLUSION_PREDICATE: D3D11_QUERY = 5;
pub const D3D11_QUERY_PIPELINE_STATISTICS: D3D11_QUERY = 4;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE: D3D11_QUERY = 7;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM0: D3D11_QUERY = 9;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM1: D3D11_QUERY = 11;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM2: D3D11_QUERY = 13;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM3: D3D11_QUERY = 15;
pub const D3D11_QUERY_SO_STATISTICS: D3D11_QUERY = 6;
pub const D3D11_QUERY_SO_STATISTICS_STREAM0: D3D11_QUERY = 8;
pub const D3D11_QUERY_SO_STATISTICS_STREAM1: D3D11_QUERY = 10;
pub const D3D11_QUERY_SO_STATISTICS_STREAM2: D3D11_QUERY = 12;
pub const D3D11_QUERY_SO_STATISTICS_STREAM3: D3D11_QUERY = 14;
pub const D3D11_QUERY_TIMESTAMP: D3D11_QUERY = 2;
pub const D3D11_QUERY_TIMESTAMP_DISJOINT: D3D11_QUERY = 3;
pub type D3D11_RAISE_FLAG = i32;
pub const D3D11_RAISE_FLAG_DRIVER_INTERNAL_ERROR: D3D11_RAISE_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: windows_core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_core::BOOL,
    pub ScissorEnable: windows_core::BOOL,
    pub MultisampleEnable: windows_core::BOOL,
    pub AntialiasedLineEnable: windows_core::BOOL,
}
pub const D3D11_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16;
#[cfg(feature = "Win32_windef")]
pub type D3D11_RECT = super::windef::RECT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: windows_core::BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub union D3D11_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture1D: D3D11_TEX1D_RTV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D11_TEX3D_RTV,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: u32 = 4096;
pub const D3D11_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27;
pub const D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096;
pub const D3D11_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: u32 = 4096;
pub const D3D11_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32;
pub const D3D11_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32;
pub const D3D11_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 16384;
pub const D3D11_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024;
pub const D3D11_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096;
pub const D3D11_REQ_MAXANISOTROPY: u32 = 16;
pub const D3D11_REQ_MIP_LEVELS: u32 = 15;
pub const D3D11_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048;
pub const D3D11_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: u32 = 4096;
pub const D3D11_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 16384;
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: u32 = 128;
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: u32 = 2048;
pub const D3D11_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: u32 = 20;
pub const D3D11_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: u32 = 4096;
pub const D3D11_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 2048;
pub const D3D11_REQ_TEXTURE1D_U_DIMENSION: u32 = 16384;
pub const D3D11_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 2048;
pub const D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 16384;
pub const D3D11_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048;
pub const D3D11_REQ_TEXTURECUBE_DIMENSION: u32 = 16384;
pub const D3D11_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0;
pub type D3D11_RESOURCE_DIMENSION = i32;
pub const D3D11_RESOURCE_DIMENSION_BUFFER: D3D11_RESOURCE_DIMENSION = 1;
pub const D3D11_RESOURCE_DIMENSION_TEXTURE1D: D3D11_RESOURCE_DIMENSION = 2;
pub const D3D11_RESOURCE_DIMENSION_TEXTURE2D: D3D11_RESOURCE_DIMENSION = 3;
pub const D3D11_RESOURCE_DIMENSION_TEXTURE3D: D3D11_RESOURCE_DIMENSION = 4;
pub const D3D11_RESOURCE_DIMENSION_UNKNOWN: D3D11_RESOURCE_DIMENSION = 0;
pub const D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS: D3D11_RESOURCE_MISC_FLAG = 32;
pub const D3D11_RESOURCE_MISC_BUFFER_STRUCTURED: D3D11_RESOURCE_MISC_FLAG = 64;
pub const D3D11_RESOURCE_MISC_DRAWINDIRECT_ARGS: D3D11_RESOURCE_MISC_FLAG = 16;
pub type D3D11_RESOURCE_MISC_FLAG = i32;
pub const D3D11_RESOURCE_MISC_GDI_COMPATIBLE: D3D11_RESOURCE_MISC_FLAG = 512;
pub const D3D11_RESOURCE_MISC_GENERATE_MIPS: D3D11_RESOURCE_MISC_FLAG = 1;
pub const D3D11_RESOURCE_MISC_GUARDED: D3D11_RESOURCE_MISC_FLAG = 32768;
pub const D3D11_RESOURCE_MISC_HW_PROTECTED: D3D11_RESOURCE_MISC_FLAG = 524288;
pub const D3D11_RESOURCE_MISC_NO_SHADER_ACCESS: D3D11_RESOURCE_MISC_FLAG = 4194304;
pub const D3D11_RESOURCE_MISC_RESOURCE_CLAMP: D3D11_RESOURCE_MISC_FLAG = 128;
pub const D3D11_RESOURCE_MISC_RESTRICTED_CONTENT: D3D11_RESOURCE_MISC_FLAG = 4096;
pub const D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE: D3D11_RESOURCE_MISC_FLAG = 8192;
pub const D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE_DRIVER: D3D11_RESOURCE_MISC_FLAG = 16384;
pub const D3D11_RESOURCE_MISC_SHARED: D3D11_RESOURCE_MISC_FLAG = 2;
pub const D3D11_RESOURCE_MISC_SHARED_DISPLAYABLE: D3D11_RESOURCE_MISC_FLAG = 1048576;
pub const D3D11_RESOURCE_MISC_SHARED_EXCLUSIVE_WRITER: D3D11_RESOURCE_MISC_FLAG = 2097152;
pub const D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX: D3D11_RESOURCE_MISC_FLAG = 256;
pub const D3D11_RESOURCE_MISC_SHARED_NTHANDLE: D3D11_RESOURCE_MISC_FLAG = 2048;
pub const D3D11_RESOURCE_MISC_TEXTURECUBE: D3D11_RESOURCE_MISC_FLAG = 4;
pub const D3D11_RESOURCE_MISC_TILED: D3D11_RESOURCE_MISC_FLAG = 262144;
pub const D3D11_RESOURCE_MISC_TILE_POOL: D3D11_RESOURCE_MISC_FLAG = 131072;
pub type D3D11_RTV_DIMENSION = i32;
pub const D3D11_RTV_DIMENSION_BUFFER: D3D11_RTV_DIMENSION = 1;
pub const D3D11_RTV_DIMENSION_TEXTURE1D: D3D11_RTV_DIMENSION = 2;
pub const D3D11_RTV_DIMENSION_TEXTURE1DARRAY: D3D11_RTV_DIMENSION = 3;
pub const D3D11_RTV_DIMENSION_TEXTURE2D: D3D11_RTV_DIMENSION = 4;
pub const D3D11_RTV_DIMENSION_TEXTURE2DARRAY: D3D11_RTV_DIMENSION = 5;
pub const D3D11_RTV_DIMENSION_TEXTURE2DMS: D3D11_RTV_DIMENSION = 6;
pub const D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D11_RTV_DIMENSION = 7;
pub const D3D11_RTV_DIMENSION_TEXTURE3D: D3D11_RTV_DIMENSION = 8;
pub const D3D11_RTV_DIMENSION_UNKNOWN: D3D11_RTV_DIMENSION = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_SAMPLER_DESC {
    pub Filter: D3D11_FILTER,
    pub AddressU: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D11_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D11_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl Default for D3D11_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_SDK_VERSION: u32 = 7;
pub type D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER = i32;
pub const D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER_0: D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER = 0;
pub const D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER_1: D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER = 1;
pub const D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = 2;
pub const D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = 1;
pub type D3D11_SHADER_CACHE_SUPPORT_FLAGS = i32;
pub const D3D11_SHADER_CACHE_SUPPORT_NONE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = 0;
pub const D3D11_SHADER_MAJOR_VERSION: u32 = 5;
pub const D3D11_SHADER_MAX_INSTANCES: u32 = 65535;
pub const D3D11_SHADER_MAX_INTERFACES: u32 = 253;
pub const D3D11_SHADER_MAX_INTERFACE_CALL_SITES: u32 = 4096;
pub const D3D11_SHADER_MAX_TYPES: u32 = 65535;
pub const D3D11_SHADER_MINOR_VERSION: u32 = 0;
pub const D3D11_SHADER_MIN_PRECISION_10_BIT: D3D11_SHADER_MIN_PRECISION_SUPPORT = 1;
pub const D3D11_SHADER_MIN_PRECISION_16_BIT: D3D11_SHADER_MIN_PRECISION_SUPPORT = 2;
pub type D3D11_SHADER_MIN_PRECISION_SUPPORT = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D11_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_SRV,
    pub Texture1D: D3D11_TEX1D_SRV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D11_TEX3D_SRV,
    pub TextureCube: D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: D3D11_BUFFEREX_SRV,
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_SHARED_RESOURCE_TIER = i32;
pub const D3D11_SHARED_RESOURCE_TIER_0: D3D11_SHARED_RESOURCE_TIER = 0;
pub const D3D11_SHARED_RESOURCE_TIER_1: D3D11_SHARED_RESOURCE_TIER = 1;
pub const D3D11_SHARED_RESOURCE_TIER_2: D3D11_SHARED_RESOURCE_TIER = 2;
pub const D3D11_SHARED_RESOURCE_TIER_3: D3D11_SHARED_RESOURCE_TIER = 3;
pub const D3D11_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0;
pub const D3D11_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5;
pub const D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8;
pub const D3D11_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048;
pub const D3D11_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 512;
pub const D3D11_SO_BUFFER_SLOT_COUNT: u32 = 4;
pub const D3D11_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
pub const D3D11_SO_NO_RASTERIZED_STREAM: u32 = 4294967295;
pub const D3D11_SO_OUTPUT_COMPONENT_COUNT: u32 = 128;
pub const D3D11_SO_STREAM_COUNT: u32 = 4;
pub const D3D11_SPEC_DATE_DAY: u32 = 16;
pub const D3D11_SPEC_DATE_MONTH: u32 = 5;
pub const D3D11_SPEC_DATE_YEAR: u32 = 2011;
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_SRV_DIMENSION(pub super::d3dcommon::D3D_SRV_DIMENSION);
pub const D3D11_STANDARD_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64;
pub const D3D11_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4;
pub const D3D11_STANDARD_MULTISAMPLE_PATTERN: D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -1;
pub type D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS = i32;
pub const D3D11_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128;
pub const D3D11_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32;
pub const D3D11_STANDARD_VECTOR_SIZE: u32 = 4;
pub const D3D11_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32;
pub const D3D11_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64;
pub type D3D11_STENCIL_OP = i32;
pub const D3D11_STENCIL_OP_DECR: D3D11_STENCIL_OP = 8;
pub const D3D11_STENCIL_OP_DECR_SAT: D3D11_STENCIL_OP = 5;
pub const D3D11_STENCIL_OP_INCR: D3D11_STENCIL_OP = 7;
pub const D3D11_STENCIL_OP_INCR_SAT: D3D11_STENCIL_OP = 4;
pub const D3D11_STENCIL_OP_INVERT: D3D11_STENCIL_OP = 6;
pub const D3D11_STENCIL_OP_KEEP: D3D11_STENCIL_OP = 1;
pub const D3D11_STENCIL_OP_REPLACE: D3D11_STENCIL_OP = 3;
pub const D3D11_STENCIL_OP_ZERO: D3D11_STENCIL_OP = 2;
pub const D3D11_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub pSysMem: *const core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl Default for D3D11_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8;
pub const D3D11_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64;
pub const D3D11_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64;
pub const D3D11_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63;
pub const D3D11_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64;
pub const D3D11_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2;
pub const D3D11_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1;
pub const D3D11_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX1D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX1D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX1D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_VPOV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_VDOV {
    pub ArraySlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_VPIV {
    pub MipSlice: u32,
    pub ArraySlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_VPOV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
pub const D3D11_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
pub type D3D11_TEXTURECUBE_FACE = i32;
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_X: D3D11_TEXTURECUBE_FACE = 1;
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_Y: D3D11_TEXTURECUBE_FACE = 3;
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_Z: D3D11_TEXTURECUBE_FACE = 5;
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_X: D3D11_TEXTURECUBE_FACE = 0;
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_Y: D3D11_TEXTURECUBE_FACE = 2;
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_Z: D3D11_TEXTURECUBE_FACE = 4;
pub const D3D11_TEXTURE_ADDRESS_BORDER: D3D11_TEXTURE_ADDRESS_MODE = 4;
pub const D3D11_TEXTURE_ADDRESS_CLAMP: D3D11_TEXTURE_ADDRESS_MODE = 3;
pub const D3D11_TEXTURE_ADDRESS_MIRROR: D3D11_TEXTURE_ADDRESS_MODE = 2;
pub const D3D11_TEXTURE_ADDRESS_MIRROR_ONCE: D3D11_TEXTURE_ADDRESS_MODE = 5;
pub type D3D11_TEXTURE_ADDRESS_MODE = i32;
pub const D3D11_TEXTURE_ADDRESS_WRAP: D3D11_TEXTURE_ADDRESS_MODE = 1;
pub const D3D11_TILED_RESOURCES_NOT_SUPPORTED: D3D11_TILED_RESOURCES_TIER = 0;
pub type D3D11_TILED_RESOURCES_TIER = i32;
pub const D3D11_TILED_RESOURCES_TIER_1: D3D11_TILED_RESOURCES_TIER = 1;
pub const D3D11_TILED_RESOURCES_TIER_2: D3D11_TILED_RESOURCES_TIER = 2;
pub const D3D11_TILED_RESOURCES_TIER_3: D3D11_TILED_RESOURCES_TIER = 3;
pub type D3D11_UAV_DIMENSION = i32;
pub const D3D11_UAV_DIMENSION_BUFFER: D3D11_UAV_DIMENSION = 1;
pub const D3D11_UAV_DIMENSION_TEXTURE1D: D3D11_UAV_DIMENSION = 2;
pub const D3D11_UAV_DIMENSION_TEXTURE1DARRAY: D3D11_UAV_DIMENSION = 3;
pub const D3D11_UAV_DIMENSION_TEXTURE2D: D3D11_UAV_DIMENSION = 4;
pub const D3D11_UAV_DIMENSION_TEXTURE2DARRAY: D3D11_UAV_DIMENSION = 5;
pub const D3D11_UAV_DIMENSION_TEXTURE3D: D3D11_UAV_DIMENSION = 8;
pub const D3D11_UAV_DIMENSION_UNKNOWN: D3D11_UAV_DIMENSION = 0;
pub const D3D11_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC_0,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV,
    pub Texture3D: D3D11_TEX3D_UAV,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_USAGE = i32;
pub const D3D11_USAGE_DEFAULT: D3D11_USAGE = 0;
pub const D3D11_USAGE_DYNAMIC: D3D11_USAGE = 2;
pub const D3D11_USAGE_IMMUTABLE: D3D11_USAGE = 1;
pub const D3D11_USAGE_STAGING: D3D11_USAGE = 3;
pub type D3D11_VDOV_DIMENSION = i32;
pub const D3D11_VDOV_DIMENSION_TEXTURE2D: D3D11_VDOV_DIMENSION = 1;
pub const D3D11_VDOV_DIMENSION_UNKNOWN: D3D11_VDOV_DIMENSION = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_COLOR {
    pub Anonymous: D3D11_VIDEO_COLOR_0,
}
impl Default for D3D11_VIDEO_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_COLOR_0 {
    pub YCbCr: D3D11_VIDEO_COLOR_YCbCrA,
    pub RGBA: D3D11_VIDEO_COLOR_RGBA,
}
impl Default for D3D11_VIDEO_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_COLOR_RGBA {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_COLOR_YCbCrA {
    pub Y: f32,
    pub Cb: f32,
    pub Cr: f32,
    pub A: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    pub Caps: u32,
    pub KeyExchangeTypeCount: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
pub const D3D11_VIDEO_DECODER_BUFFER_BITSTREAM: D3D11_VIDEO_DECODER_BUFFER_TYPE = 6;
pub const D3D11_VIDEO_DECODER_BUFFER_DEBLOCKING_CONTROL: D3D11_VIDEO_DECODER_BUFFER_TYPE = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub BufferIndex: u32,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub FirstMBaddress: u32,
    pub NumMBsInBuffer: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub ReservedBits: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub PartialEncryption: windows_core::BOOL,
    pub EncryptedBlockInfo: D3D11_ENCRYPTED_BLOCK_INFO,
}
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_VIDEO_DECODER_BUFFER_FILM_GRAIN: D3D11_VIDEO_DECODER_BUFFER_TYPE = 8;
pub const D3D11_VIDEO_DECODER_BUFFER_HUFFMAN_TABLE: D3D11_VIDEO_DECODER_BUFFER_TYPE = 9;
pub const D3D11_VIDEO_DECODER_BUFFER_INVERSE_QUANTIZATION_MATRIX: D3D11_VIDEO_DECODER_BUFFER_TYPE = 4;
pub const D3D11_VIDEO_DECODER_BUFFER_MACROBLOCK_CONTROL: D3D11_VIDEO_DECODER_BUFFER_TYPE = 1;
pub const D3D11_VIDEO_DECODER_BUFFER_MOTION_VECTOR: D3D11_VIDEO_DECODER_BUFFER_TYPE = 7;
pub const D3D11_VIDEO_DECODER_BUFFER_PICTURE_PARAMETERS: D3D11_VIDEO_DECODER_BUFFER_TYPE = 0;
pub const D3D11_VIDEO_DECODER_BUFFER_RESIDUAL_DIFFERENCE: D3D11_VIDEO_DECODER_BUFFER_TYPE = 2;
pub const D3D11_VIDEO_DECODER_BUFFER_SLICE_CONTROL: D3D11_VIDEO_DECODER_BUFFER_TYPE = 5;
pub type D3D11_VIDEO_DECODER_BUFFER_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_DECODER_CONFIG {
    pub guidConfigBitstreamEncryption: windows_core::GUID,
    pub guidConfigMBcontrolEncryption: windows_core::GUID,
    pub guidConfigResidDiffEncryption: windows_core::GUID,
    pub ConfigBitstreamRaw: u32,
    pub ConfigMBcontrolRasterOrder: u32,
    pub ConfigResidDiffHost: u32,
    pub ConfigSpatialResid8: u32,
    pub ConfigResid8Subtraction: u32,
    pub ConfigSpatialHost8or9Clipping: u32,
    pub ConfigSpatialResidInterleaved: u32,
    pub ConfigIntraResidUnsigned: u32,
    pub ConfigResidDiffAccelerator: u32,
    pub ConfigHostInverseScan: u32,
    pub ConfigSpecificIDCT: u32,
    pub Config4GroupedCoefs: u32,
    pub ConfigMinRenderTargetBuffCount: u16,
    pub ConfigDecoderSpecific: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_DECODER_DESC {
    pub Guid: windows_core::GUID,
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub OutputFormat: super::dxgiformat::DXGI_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIDEO_DECODER_EXTENSION {
    pub Function: u32,
    pub pPrivateInputData: *mut core::ffi::c_void,
    pub PrivateInputDataSize: u32,
    pub pPrivateOutputData: *mut core::ffi::c_void,
    pub PrivateOutputDataSize: u32,
    pub ResourceCount: u32,
    pub ppResourceList: *mut Option<ID3D11Resource>,
}
impl Default for D3D11_VIDEO_DECODER_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    pub DecodeProfile: windows_core::GUID,
    pub ViewDimension: D3D11_VDOV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0,
}
impl Default for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VDOV,
}
impl Default for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_FRAME_FORMAT = i32;
pub const D3D11_VIDEO_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST: D3D11_VIDEO_FRAME_FORMAT = 2;
pub const D3D11_VIDEO_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST: D3D11_VIDEO_FRAME_FORMAT = 1;
pub const D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE: D3D11_VIDEO_FRAME_FORMAT = 0;
pub type D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = i32;
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_BACKGROUND: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = 1;
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_DESTINATION: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = 2;
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_OPAQUE: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = 0;
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_SOURCE_STREAM: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = 3;
pub type D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_ANAMORPHIC_SCALING: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = 128;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_COLOR_CORRECTION: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = 8;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DENOISE: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = 1;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DERINGING: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = 2;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_EDGE_ENHANCEMENT: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = 4;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_FLESH_TONE_MAPPING: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = 16;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_IMAGE_STABILIZATION: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = 32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_SUPER_RESOLUTION: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CAPS {
    pub DeviceCaps: u32,
    pub FeatureCaps: u32,
    pub FilterCaps: u32,
    pub InputFormatCaps: u32,
    pub AutoStreamCaps: u32,
    pub StereoCaps: u32,
    pub RateConversionCapsCount: u32,
    pub MaxInputStreams: u32,
    pub MaxStreamStates: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgicommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    pub InputFrameFormat: D3D11_VIDEO_FRAME_FORMAT,
    pub InputFrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub InputWidth: u32,
    pub InputHeight: u32,
    pub OutputFrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
    pub Usage: D3D11_VIDEO_USAGE,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgicommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    pub CustomRate: super::dxgicommon::DXGI_RATIONAL,
    pub OutputFrames: u32,
    pub InputInterlaced: windows_core::BOOL,
    pub InputFramesOrFields: u32,
}
pub type D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = i32;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_LINEAR_SPACE: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = 1;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_NOMINAL_RANGE: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = 16;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_RGB_RANGE_CONVERSION: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = 4;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_YCbCr_MATRIX_CONVERSION: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = 8;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_xvYCC: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = 2;
pub type D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_FILL: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 1;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_PALETTE: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 8;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_STREAM: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 128;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_CONSTRICTION: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 2;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_LEGACY: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 16;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_LUMA_KEY: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 4;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_METADATA_HDR10: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 2048;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_MIRROR: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 512;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_PIXEL_ASPECT_RATIO: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 256;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ROTATION: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 64;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_SHADER_USAGE: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 1024;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_STEREO: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = 32;
pub type D3D11_VIDEO_PROCESSOR_FILTER = i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_ANAMORPHIC_SCALING: D3D11_VIDEO_PROCESSOR_FILTER = 6;
pub const D3D11_VIDEO_PROCESSOR_FILTER_BRIGHTNESS: D3D11_VIDEO_PROCESSOR_FILTER = 0;
pub type D3D11_VIDEO_PROCESSOR_FILTER_CAPS = i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_ANAMORPHIC_SCALING: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = 64;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_BRIGHTNESS: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = 1;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_CONTRAST: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = 2;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_EDGE_ENHANCEMENT: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = 32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_HUE: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = 4;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_NOISE_REDUCTION: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = 16;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_SATURATION: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = 8;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_STEREO_ADJUSTMENT: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = 128;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CONTRAST: D3D11_VIDEO_PROCESSOR_FILTER = 1;
pub const D3D11_VIDEO_PROCESSOR_FILTER_EDGE_ENHANCEMENT: D3D11_VIDEO_PROCESSOR_FILTER = 5;
pub const D3D11_VIDEO_PROCESSOR_FILTER_HUE: D3D11_VIDEO_PROCESSOR_FILTER = 2;
pub const D3D11_VIDEO_PROCESSOR_FILTER_NOISE_REDUCTION: D3D11_VIDEO_PROCESSOR_FILTER = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    pub Minimum: i32,
    pub Maximum: i32,
    pub Default: i32,
    pub Multiplier: f32,
}
pub const D3D11_VIDEO_PROCESSOR_FILTER_SATURATION: D3D11_VIDEO_PROCESSOR_FILTER = 3;
pub const D3D11_VIDEO_PROCESSOR_FILTER_STEREO_ADJUSTMENT: D3D11_VIDEO_PROCESSOR_FILTER = 7;
pub type D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = i32;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_PALETTE_INTERLACED: D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = 8;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_INTERLACED: D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = 1;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_LUMA_KEY: D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = 4;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_PROCAMP: D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = 2;
pub type D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT = i32;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_INPUT: D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT = 1;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_OUTPUT: D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    pub FourCC: u32,
    pub ViewDimension: D3D11_VPIV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0,
}
impl Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VPIV,
}
impl Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_22: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 2;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_222222222223: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 256;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_2224: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 4;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_2332: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 8;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_32: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 1;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_32322: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 16;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_55: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_64: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 64;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_87: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = 128;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_OTHER: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = -2147483648;
pub type D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE = i32;
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_0_255: D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE = 2;
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_16_235: D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE = 1;
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_UNDEFINED: D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE = 0;
pub type D3D11_VIDEO_PROCESSOR_OUTPUT_RATE = i32;
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_CUSTOM: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE = 2;
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_HALF: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE = 1;
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_NORMAL: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    pub ViewDimension: D3D11_VPOV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0,
}
impl Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VPOV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_VPOV,
}
impl Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = i32;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_ADAPTIVE: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = 4;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_BLEND: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = 1;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_BOB: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = 2;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_MOTION_COMPENSATION: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = 8;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_FRAME_RATE_CONVERSION: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = 32;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_INVERSE_TELECINE: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ProcessorCaps: u32,
    pub ITelecineCaps: u32,
    pub CustomRateCount: u32,
}
pub type D3D11_VIDEO_PROCESSOR_ROTATION = i32;
pub const D3D11_VIDEO_PROCESSOR_ROTATION_180: D3D11_VIDEO_PROCESSOR_ROTATION = 2;
pub const D3D11_VIDEO_PROCESSOR_ROTATION_270: D3D11_VIDEO_PROCESSOR_ROTATION = 3;
pub const D3D11_VIDEO_PROCESSOR_ROTATION_90: D3D11_VIDEO_PROCESSOR_ROTATION = 1;
pub const D3D11_VIDEO_PROCESSOR_ROTATION_IDENTITY: D3D11_VIDEO_PROCESSOR_ROTATION = 0;
pub type D3D11_VIDEO_PROCESSOR_STEREO_CAPS = i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_CHECKERBOARD: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = 8;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_COLUMN_INTERLEAVED: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = 4;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_FLIP_MODE: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = 16;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_MONO_OFFSET: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = 1;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_ROW_INTERLEAVED: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = 2;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_FRAME0: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE = 1;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_FRAME1: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE = 2;
pub type D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE = i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_NONE: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE = 0;
pub type D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_CHECKERBOARD: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = 7;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_COLUMN_INTERLEAVED: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = 6;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_HORIZONTAL: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = 1;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_MONO: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = 0;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_MONO_OFFSET: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = 4;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_ROW_INTERLEAVED: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = 5;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_SEPARATE: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = 3;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_VERTICAL: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = 2;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_STREAM {
    pub Enable: windows_core::BOOL,
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ppPastSurfaces: *mut Option<ID3D11VideoProcessorInputView>,
    pub pInputSurface: core::mem::ManuallyDrop<Option<ID3D11VideoProcessorInputView>>,
    pub ppFutureSurfaces: *mut Option<ID3D11VideoProcessorInputView>,
    pub ppPastSurfacesRight: *mut Option<ID3D11VideoProcessorInputView>,
    pub pInputSurfaceRight: core::mem::ManuallyDrop<Option<ID3D11VideoProcessorInputView>>,
    pub ppFutureSurfacesRight: *mut Option<ID3D11VideoProcessorInputView>,
}
impl Default for D3D11_VIDEO_PROCESSOR_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_USAGE = i32;
pub const D3D11_VIDEO_USAGE_OPTIMAL_QUALITY: D3D11_VIDEO_USAGE = 2;
pub const D3D11_VIDEO_USAGE_OPTIMAL_SPEED: D3D11_VIDEO_USAGE = 1;
pub const D3D11_VIDEO_USAGE_PLAYBACK_NORMAL: D3D11_VIDEO_USAGE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
pub const D3D11_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15;
pub const D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16;
pub const D3D11_VIEWPORT_BOUNDS_MAX: u32 = 32767;
pub const D3D11_VIEWPORT_BOUNDS_MIN: i32 = -32768;
pub type D3D11_VPIV_DIMENSION = i32;
pub const D3D11_VPIV_DIMENSION_TEXTURE2D: D3D11_VPIV_DIMENSION = 1;
pub const D3D11_VPIV_DIMENSION_UNKNOWN: D3D11_VPIV_DIMENSION = 0;
pub type D3D11_VPOV_DIMENSION = i32;
pub const D3D11_VPOV_DIMENSION_TEXTURE2D: D3D11_VPOV_DIMENSION = 1;
pub const D3D11_VPOV_DIMENSION_TEXTURE2DARRAY: D3D11_VPOV_DIMENSION = 2;
pub const D3D11_VPOV_DIMENSION_UNKNOWN: D3D11_VPOV_DIMENSION = 0;
pub const D3D11_VS_INPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_VS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D11_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D11_VS_INPUT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D11_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_VS_OUTPUT_REGISTER_COUNT: u32 = 32;
pub const D3D11_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10;
pub const D3D11_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25;
pub const D3D11_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25;
windows_core::imp::define_interface!(ID3D11Asynchronous, ID3D11Asynchronous_Vtbl, 0x4b35d0cd_1e15_4258_9c98_1b1333f6dd3b);
impl core::ops::Deref for ID3D11Asynchronous {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Asynchronous, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11Asynchronous {
    pub unsafe fn GetDataSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Asynchronous_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait ID3D11Asynchronous_Impl: ID3D11DeviceChild_Impl {
    fn GetDataSize(&self) -> u32;
}
impl ID3D11Asynchronous_Vtbl {
    pub const fn new<Identity: ID3D11Asynchronous_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDataSize<Identity: ID3D11Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Asynchronous_Impl::GetDataSize(this)
            }
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDataSize: GetDataSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Asynchronous as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Asynchronous {}
windows_core::imp::define_interface!(ID3D11AuthenticatedChannel, ID3D11AuthenticatedChannel_Vtbl, 0x3015a308_dcbd_47aa_a747_192486d14d4a);
impl core::ops::Deref for ID3D11AuthenticatedChannel {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11AuthenticatedChannel, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11AuthenticatedChannel {
    pub unsafe fn GetCertificateSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCertificate(&self, pcertificate: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), pcertificate.len().try_into().unwrap(), core::mem::transmute(pcertificate.as_ptr())) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetChannelHandle(&self) -> super::winnt::HANDLE {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelHandle)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11AuthenticatedChannel_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetCertificateSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub GetChannelHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE),
    #[cfg(not(feature = "Win32_winnt"))]
    GetChannelHandle: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait ID3D11AuthenticatedChannel_Impl: ID3D11DeviceChild_Impl {
    fn GetCertificateSize(&self) -> windows_core::Result<u32>;
    fn GetCertificate(&self, certificatesize: u32, pcertificate: *mut u8) -> windows_core::Result<()>;
    fn GetChannelHandle(&self, pchannelhandle: *mut super::winnt::HANDLE);
}
#[cfg(feature = "Win32_winnt")]
impl ID3D11AuthenticatedChannel_Vtbl {
    pub const fn new<Identity: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCertificateSize<Identity: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertificatesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11AuthenticatedChannel_Impl::GetCertificateSize(this) {
                    Ok(ok__) => {
                        pcertificatesize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11AuthenticatedChannel_Impl::GetCertificate(this, core::mem::transmute_copy(&certificatesize), core::mem::transmute_copy(&pcertificate)).into()
            }
        }
        unsafe extern "system" fn GetChannelHandle<Identity: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannelhandle: *mut super::winnt::HANDLE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11AuthenticatedChannel_Impl::GetChannelHandle(this, core::mem::transmute_copy(&pchannelhandle));
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetCertificateSize: GetCertificateSize::<Identity, OFFSET>,
            GetCertificate: GetCertificate::<Identity, OFFSET>,
            GetChannelHandle: GetChannelHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11AuthenticatedChannel as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for ID3D11AuthenticatedChannel {}
windows_core::imp::define_interface!(ID3D11BlendState, ID3D11BlendState_Vtbl, 0x75b68faa_347d_4159_8f45_a0640f01cd9a);
impl core::ops::Deref for ID3D11BlendState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11BlendState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11BlendState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_BLEND_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11BlendState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_BLEND_DESC),
}
pub trait ID3D11BlendState_Impl: ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_BLEND_DESC);
}
impl ID3D11BlendState_Vtbl {
    pub const fn new<Identity: ID3D11BlendState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11BlendState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11BlendState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11BlendState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11BlendState {}
windows_core::imp::define_interface!(ID3D11Buffer, ID3D11Buffer_Vtbl, 0x48570b85_d1ee_4fcd_a250_eb350722b037);
impl core::ops::Deref for ID3D11Buffer {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Buffer, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
impl ID3D11Buffer {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_BUFFER_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Buffer_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_BUFFER_DESC),
}
pub trait ID3D11Buffer_Impl: ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_BUFFER_DESC);
}
impl ID3D11Buffer_Vtbl {
    pub const fn new<Identity: ID3D11Buffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11Buffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_BUFFER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Buffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11Resource_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Buffer as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Buffer {}
windows_core::imp::define_interface!(ID3D11ClassInstance, ID3D11ClassInstance_Vtbl, 0xa6cd7faa_b0b7_4a2f_9436_8662a65797cb);
impl core::ops::Deref for ID3D11ClassInstance {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ClassInstance, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11ClassInstance {
    pub unsafe fn GetClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassLinkage)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
    pub unsafe fn GetInstanceName(&self, pinstancename: Option<windows_core::PSTR>, pbufferlength: *mut usize) {
        unsafe {
            (windows_core::Interface::vtable(self).GetInstanceName)(windows_core::Interface::as_raw(self), pinstancename.unwrap_or(core::mem::zeroed()) as _, pbufferlength as _);
        }
    }
    pub unsafe fn GetTypeName(&self, ptypename: Option<windows_core::PSTR>, pbufferlength: *mut usize) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTypeName)(windows_core::Interface::as_raw(self), ptypename.unwrap_or(core::mem::zeroed()) as _, pbufferlength as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ClassInstance_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetClassLinkage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_CLASS_INSTANCE_DESC),
    pub GetInstanceName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, *mut usize),
    pub GetTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, *mut usize),
}
pub trait ID3D11ClassInstance_Impl: ID3D11DeviceChild_Impl {
    fn GetClassLinkage(&self, pplinkage: windows_core::OutRef<ID3D11ClassLinkage>);
    fn GetDesc(&self, pdesc: *mut D3D11_CLASS_INSTANCE_DESC);
    fn GetInstanceName(&self, pinstancename: windows_core::PSTR, pbufferlength: *mut usize);
    fn GetTypeName(&self, ptypename: windows_core::PSTR, pbufferlength: *mut usize);
}
impl ID3D11ClassInstance_Vtbl {
    pub const fn new<Identity: ID3D11ClassInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassLinkage<Identity: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplinkage: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ClassInstance_Impl::GetClassLinkage(this, core::mem::transmute_copy(&pplinkage));
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ClassInstance_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        unsafe extern "system" fn GetInstanceName<Identity: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstancename: windows_core::PSTR, pbufferlength: *mut usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ClassInstance_Impl::GetInstanceName(this, core::mem::transmute_copy(&pinstancename), core::mem::transmute_copy(&pbufferlength));
            }
        }
        unsafe extern "system" fn GetTypeName<Identity: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypename: windows_core::PSTR, pbufferlength: *mut usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ClassInstance_Impl::GetTypeName(this, core::mem::transmute_copy(&ptypename), core::mem::transmute_copy(&pbufferlength));
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetClassLinkage: GetClassLinkage::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetInstanceName: GetInstanceName::<Identity, OFFSET>,
            GetTypeName: GetTypeName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ClassInstance as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11ClassInstance {}
windows_core::imp::define_interface!(ID3D11ClassLinkage, ID3D11ClassLinkage_Vtbl, 0xddf57cba_9543_46e4_a12b_f207a0fe7fed);
impl core::ops::Deref for ID3D11ClassLinkage {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ClassLinkage, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11ClassLinkage {
    pub unsafe fn GetClassInstance<P0>(&self, pclassinstancename: P0, instanceindex: u32) -> windows_core::Result<ID3D11ClassInstance>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassInstance)(windows_core::Interface::as_raw(self), pclassinstancename.param().abi(), instanceindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateClassInstance<P0>(&self, pclasstypename: P0, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32) -> windows_core::Result<ID3D11ClassInstance>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateClassInstance)(windows_core::Interface::as_raw(self), pclasstypename.param().abi(), constantbufferoffset, constantvectoroffset, textureoffset, sampleroffset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ClassLinkage_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetClassInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ID3D11ClassLinkage_Impl: ID3D11DeviceChild_Impl {
    fn GetClassInstance(&self, pclassinstancename: &windows_core::PCSTR, instanceindex: u32) -> windows_core::Result<ID3D11ClassInstance>;
    fn CreateClassInstance(&self, pclasstypename: &windows_core::PCSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32) -> windows_core::Result<ID3D11ClassInstance>;
}
impl ID3D11ClassLinkage_Vtbl {
    pub const fn new<Identity: ID3D11ClassLinkage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassInstance<Identity: ID3D11ClassLinkage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclassinstancename: windows_core::PCSTR, instanceindex: u32, ppinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11ClassLinkage_Impl::GetClassInstance(this, core::mem::transmute(&pclassinstancename), core::mem::transmute_copy(&instanceindex)) {
                    Ok(ok__) => {
                        ppinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateClassInstance<Identity: ID3D11ClassLinkage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclasstypename: windows_core::PCSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32, ppinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11ClassLinkage_Impl::CreateClassInstance(this, core::mem::transmute(&pclasstypename), core::mem::transmute_copy(&constantbufferoffset), core::mem::transmute_copy(&constantvectoroffset), core::mem::transmute_copy(&textureoffset), core::mem::transmute_copy(&sampleroffset)) {
                    Ok(ok__) => {
                        ppinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetClassInstance: GetClassInstance::<Identity, OFFSET>,
            CreateClassInstance: CreateClassInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ClassLinkage as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11ClassLinkage {}
windows_core::imp::define_interface!(ID3D11CommandList, ID3D11CommandList_Vtbl, 0xa24bc4d1_769e_43f7_8013_98ff566c18e2);
impl core::ops::Deref for ID3D11CommandList {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11CommandList, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11CommandList {
    pub unsafe fn GetContextFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetContextFlags)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11CommandList_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetContextFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait ID3D11CommandList_Impl: ID3D11DeviceChild_Impl {
    fn GetContextFlags(&self) -> u32;
}
impl ID3D11CommandList_Vtbl {
    pub const fn new<Identity: ID3D11CommandList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContextFlags<Identity: ID3D11CommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11CommandList_Impl::GetContextFlags(this)
            }
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetContextFlags: GetContextFlags::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11CommandList as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11CommandList {}
windows_core::imp::define_interface!(ID3D11ComputeShader, ID3D11ComputeShader_Vtbl, 0x4f5b196e_c2bd_495e_bd01_1fded38e4969);
impl core::ops::Deref for ID3D11ComputeShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ComputeShader, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ComputeShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11ComputeShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11ComputeShader_Vtbl {
    pub const fn new<Identity: ID3D11ComputeShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ComputeShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11ComputeShader {}
windows_core::imp::define_interface!(ID3D11Counter, ID3D11Counter_Vtbl, 0x6e8c49fb_a371_4770_b440_29086022b741);
impl core::ops::Deref for ID3D11Counter {
    type Target = ID3D11Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Counter, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Asynchronous);
impl ID3D11Counter {
    pub unsafe fn GetDesc(&self) -> D3D11_COUNTER_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Counter_Vtbl {
    pub base__: ID3D11Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_COUNTER_DESC),
}
pub trait ID3D11Counter_Impl: ID3D11Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_COUNTER_DESC);
}
impl ID3D11Counter_Vtbl {
    pub const fn new<Identity: ID3D11Counter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11Counter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_COUNTER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Counter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11Asynchronous_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Counter as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Asynchronous as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Counter {}
windows_core::imp::define_interface!(ID3D11CryptoSession, ID3D11CryptoSession_Vtbl, 0x9b32f9ad_bdcc_40a6_a39d_d5c865845720);
impl core::ops::Deref for ID3D11CryptoSession {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11CryptoSession, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11CryptoSession {
    pub unsafe fn GetCryptoType(&self) -> windows_core::GUID {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCryptoType)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetDecoderProfile(&self) -> windows_core::GUID {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDecoderProfile)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetCertificateSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCertificate(&self, pcertificate: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), pcertificate.len().try_into().unwrap(), core::mem::transmute(pcertificate.as_ptr())) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetCryptoSessionHandle(&self) -> super::winnt::HANDLE {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCryptoSessionHandle)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11CryptoSession_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetCryptoType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID),
    pub GetDecoderProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID),
    pub GetCertificateSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub GetCryptoSessionHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE),
    #[cfg(not(feature = "Win32_winnt"))]
    GetCryptoSessionHandle: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait ID3D11CryptoSession_Impl: ID3D11DeviceChild_Impl {
    fn GetCryptoType(&self, pcryptotype: *mut windows_core::GUID);
    fn GetDecoderProfile(&self, pdecoderprofile: *mut windows_core::GUID);
    fn GetCertificateSize(&self) -> windows_core::Result<u32>;
    fn GetCertificate(&self, certificatesize: u32, pcertificate: *mut u8) -> windows_core::Result<()>;
    fn GetCryptoSessionHandle(&self, pcryptosessionhandle: *mut super::winnt::HANDLE);
}
#[cfg(feature = "Win32_winnt")]
impl ID3D11CryptoSession_Vtbl {
    pub const fn new<Identity: ID3D11CryptoSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCryptoType<Identity: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *mut windows_core::GUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11CryptoSession_Impl::GetCryptoType(this, core::mem::transmute_copy(&pcryptotype));
            }
        }
        unsafe extern "system" fn GetDecoderProfile<Identity: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoderprofile: *mut windows_core::GUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11CryptoSession_Impl::GetDecoderProfile(this, core::mem::transmute_copy(&pdecoderprofile));
            }
        }
        unsafe extern "system" fn GetCertificateSize<Identity: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertificatesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11CryptoSession_Impl::GetCertificateSize(this) {
                    Ok(ok__) => {
                        pcertificatesize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11CryptoSession_Impl::GetCertificate(this, core::mem::transmute_copy(&certificatesize), core::mem::transmute_copy(&pcertificate)).into()
            }
        }
        unsafe extern "system" fn GetCryptoSessionHandle<Identity: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosessionhandle: *mut super::winnt::HANDLE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11CryptoSession_Impl::GetCryptoSessionHandle(this, core::mem::transmute_copy(&pcryptosessionhandle));
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetCryptoType: GetCryptoType::<Identity, OFFSET>,
            GetDecoderProfile: GetDecoderProfile::<Identity, OFFSET>,
            GetCertificateSize: GetCertificateSize::<Identity, OFFSET>,
            GetCertificate: GetCertificate::<Identity, OFFSET>,
            GetCryptoSessionHandle: GetCryptoSessionHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11CryptoSession as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for ID3D11CryptoSession {}
windows_core::imp::define_interface!(ID3D11DepthStencilState, ID3D11DepthStencilState_Vtbl, 0x03823efb_8d8f_4e1c_9aa2_f64bb2cbfdf1);
impl core::ops::Deref for ID3D11DepthStencilState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DepthStencilState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11DepthStencilState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DepthStencilState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_DEPTH_STENCIL_DESC),
}
pub trait ID3D11DepthStencilState_Impl: ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_DESC);
}
impl ID3D11DepthStencilState_Vtbl {
    pub const fn new<Identity: ID3D11DepthStencilState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11DepthStencilState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DepthStencilState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DepthStencilState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11DepthStencilState {}
windows_core::imp::define_interface!(ID3D11DepthStencilView, ID3D11DepthStencilView_Vtbl, 0x9fdac92a_1876_48c3_afad_25b94f84a9b6);
impl core::ops::Deref for ID3D11DepthStencilView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DepthStencilView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11DepthStencilView {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DepthStencilView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_DEPTH_STENCIL_VIEW_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D11DepthStencilView_Impl: ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D11DepthStencilView_Vtbl {
    pub const fn new<Identity: ID3D11DepthStencilView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11DepthStencilView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DepthStencilView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DepthStencilView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D11DepthStencilView {}
windows_core::imp::define_interface!(ID3D11Device, ID3D11Device_Vtbl, 0xdb6f6ddb_ac77_4e88_8253_819df9bbf140);
windows_core::imp::interface_hierarchy!(ID3D11Device, windows_core::IUnknown);
impl ID3D11Device {
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, ppbuffer: Option<*mut Option<ID3D11Buffer>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateBuffer)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, pptexture1d: Option<*mut Option<ID3D11Texture1D>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture1D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, pptexture1d.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: Option<*mut Option<ID3D11Texture2D>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture2D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, pptexture2d.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: Option<*mut Option<ID3D11Texture3D>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture3D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, pptexture3d.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>, ppsrview: Option<*mut Option<ID3D11ShaderResourceView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateShaderResourceView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppsrview.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateUnorderedAccessView<P0>(&self, presource: P0, pdesc: Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>, ppuaview: Option<*mut Option<ID3D11UnorderedAccessView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateUnorderedAccessView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppuaview.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: Option<*const D3D11_RENDER_TARGET_VIEW_DESC>, pprtview: Option<*mut Option<ID3D11RenderTargetView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTargetView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, pprtview.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: Option<*mut Option<ID3D11DepthStencilView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: Option<*mut Option<ID3D11InputLayout>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateInputLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len().try_into().unwrap(), core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len().try_into().unwrap(), ppinputlayout.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateVertexShader<P2>(&self, pshaderbytecode: &[u8], pclasslinkage: P2, ppvertexshader: Option<*mut Option<ID3D11VertexShader>>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), ppvertexshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateGeometryShader<P2>(&self, pshaderbytecode: &[u8], pclasslinkage: P2, ppgeometryshader: Option<*mut Option<ID3D11GeometryShader>>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateGeometryShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), ppgeometryshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<P7>(&self, pshaderbytecode: &[u8], psodeclaration: Option<&[D3D11_SO_DECLARATION_ENTRY]>, pbufferstrides: Option<&[u32]>, rasterizedstream: u32, pclasslinkage: P7, ppgeometryshader: Option<*mut Option<ID3D11GeometryShader>>) -> windows_core::HRESULT
    where
        P7: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateGeometryShaderWithStreamOutput)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                core::mem::transmute(psodeclaration.map_or(core::ptr::null(), |slice| slice.as_ptr())),
                psodeclaration.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(pbufferstrides.map_or(core::ptr::null(), |slice| slice.as_ptr())),
                pbufferstrides.map_or(0, |slice| slice.len().try_into().unwrap()),
                rasterizedstream,
                pclasslinkage.param().abi(),
                ppgeometryshader.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn CreatePixelShader<P2>(&self, pshaderbytecode: &[u8], pclasslinkage: P2, pppixelshader: Option<*mut Option<ID3D11PixelShader>>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), pppixelshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateHullShader<P2>(&self, pshaderbytecode: &[u8], pclasslinkage: P2, pphullshader: Option<*mut Option<ID3D11HullShader>>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateHullShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), pphullshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateDomainShader<P2>(&self, pshaderbytecode: &[u8], pclasslinkage: P2, ppdomainshader: Option<*mut Option<ID3D11DomainShader>>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDomainShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), ppdomainshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateComputeShader<P2>(&self, pshaderbytecode: &[u8], pclasslinkage: P2, ppcomputeshader: Option<*mut Option<ID3D11ComputeShader>>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateComputeShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), ppcomputeshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateClassLinkage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: Option<*mut Option<ID3D11BlendState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateBlendState)(windows_core::Interface::as_raw(self), pblendstatedesc, ppblendstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: Option<*mut Option<ID3D11DepthStencilState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencildesc, ppdepthstencilstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: Option<*mut Option<ID3D11RasterizerState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRasterizerState)(windows_core::Interface::as_raw(self), prasterizerdesc, pprasterizerstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: Option<*mut Option<ID3D11SamplerState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateSamplerState)(windows_core::Interface::as_raw(self), psamplerdesc, ppsamplerstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: Option<*mut Option<ID3D11Query>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), pquerydesc, ppquery.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: Option<*mut Option<ID3D11Predicate>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreatePredicate)(windows_core::Interface::as_raw(self), ppredicatedesc, pppredicate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: Option<*mut Option<ID3D11Counter>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateCounter)(windows_core::Interface::as_raw(self), pcounterdesc, ppcounter.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDeferredContext)(windows_core::Interface::as_raw(self), contextflags, ppdeferredcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OpenSharedResource<T>(&self, hresource: super::winnt::HANDLE, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenSharedResource)(windows_core::Interface::as_raw(self), hresource, &T::IID, result__ as *mut _ as *mut _).ok() }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CheckFormatSupport(&self, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckFormatSupport)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels)(windows_core::Interface::as_raw(self), format, samplecount, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCounterInfo)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: Option<windows_core::PSTR>, pnamelength: Option<*mut u32>, szunits: Option<windows_core::PSTR>, punitslength: Option<*mut u32>, szdescription: Option<windows_core::PSTR>, pdescriptionlength: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckCounter)(windows_core::Interface::as_raw(self), pdesc, ptype as _, pactivecounters as _, szname.unwrap_or(core::mem::zeroed()) as _, pnamelength.unwrap_or(core::mem::zeroed()) as _, szunits.unwrap_or(core::mem::zeroed()) as _, punitslength.unwrap_or(core::mem::zeroed()) as _, szdescription.unwrap_or(core::mem::zeroed()) as _, pdescriptionlength.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, pfeaturesupportdata as _, featuresupportdatasize) }
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize as _, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetFeatureLevel(&self) -> super::d3dcommon::D3D_FEATURE_LEVEL {
        unsafe { (windows_core::Interface::vtable(self).GetFeatureLevel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetImmediateContext(&self) -> windows_core::Result<ID3D11DeviceContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmediateContext)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExceptionMode)(windows_core::Interface::as_raw(self), raiseflags) }
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetExceptionMode)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Device_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateTexture1D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE1D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateTexture1D: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateTexture2D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateTexture2D: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateTexture3D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE3D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateTexture3D: usize,
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub CreateShaderResourceView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_SHADER_RESOURCE_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat")))]
    CreateShaderResourceView: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateUnorderedAccessView: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateRenderTargetView: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_DEPTH_STENCIL_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateDepthStencilView: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateInputLayout: usize,
    pub CreateVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryShaderWithStreamOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *const D3D11_SO_DECLARATION_ENTRY, u32, *const u32, u32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateHullShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDomainShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateComputeShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassLinkage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_BLEND_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_DEPTH_STENCIL_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRasterizerState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_RASTERIZER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_SAMPLER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_QUERY_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePredicate: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_QUERY_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_COUNTER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDeferredContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub OpenSharedResource: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OpenSharedResource: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CheckFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CheckFormatSupport: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CheckMultisampleQualityLevels: usize,
    pub CheckCounterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_COUNTER_INFO),
    pub CheckCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_COUNTER_DESC, *mut D3D11_COUNTER_TYPE, *mut u32, windows_core::PSTR, *mut u32, windows_core::PSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_FEATURE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3dcommon::D3D_FEATURE_LEVEL,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetFeatureLevel: usize,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImmediateContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait ID3D11Device_Impl: windows_core::IUnknownImpl {
    fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: windows_core::OutRef<ID3D11Buffer>) -> windows_core::Result<()>;
    fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: windows_core::OutRef<ID3D11Texture1D>) -> windows_core::Result<()>;
    fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: windows_core::OutRef<ID3D11Texture2D>) -> windows_core::Result<()>;
    fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: windows_core::OutRef<ID3D11Texture3D>) -> windows_core::Result<()>;
    fn CreateShaderResourceView(&self, presource: windows_core::Ref<ID3D11Resource>, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: windows_core::OutRef<ID3D11ShaderResourceView>) -> windows_core::Result<()>;
    fn CreateUnorderedAccessView(&self, presource: windows_core::Ref<ID3D11Resource>, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: windows_core::OutRef<ID3D11UnorderedAccessView>) -> windows_core::Result<()>;
    fn CreateRenderTargetView(&self, presource: windows_core::Ref<ID3D11Resource>, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: windows_core::OutRef<ID3D11RenderTargetView>) -> windows_core::Result<()>;
    fn CreateDepthStencilView(&self, presource: windows_core::Ref<ID3D11Resource>, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: windows_core::OutRef<ID3D11DepthStencilView>) -> windows_core::Result<()>;
    fn CreateInputLayout(&self, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: windows_core::OutRef<ID3D11InputLayout>) -> windows_core::Result<()>;
    fn CreateVertexShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>, ppvertexshader: windows_core::OutRef<ID3D11VertexShader>) -> windows_core::Result<()>;
    fn CreateGeometryShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>, ppgeometryshader: windows_core::OutRef<ID3D11GeometryShader>) -> windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>, ppgeometryshader: windows_core::OutRef<ID3D11GeometryShader>) -> windows_core::Result<()>;
    fn CreatePixelShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>, pppixelshader: windows_core::OutRef<ID3D11PixelShader>) -> windows_core::Result<()>;
    fn CreateHullShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>, pphullshader: windows_core::OutRef<ID3D11HullShader>) -> windows_core::Result<()>;
    fn CreateDomainShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>, ppdomainshader: windows_core::OutRef<ID3D11DomainShader>) -> windows_core::Result<()>;
    fn CreateComputeShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>, ppcomputeshader: windows_core::OutRef<ID3D11ComputeShader>) -> windows_core::Result<()>;
    fn CreateClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage>;
    fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: windows_core::OutRef<ID3D11BlendState>) -> windows_core::Result<()>;
    fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: windows_core::OutRef<ID3D11DepthStencilState>) -> windows_core::Result<()>;
    fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: windows_core::OutRef<ID3D11RasterizerState>) -> windows_core::Result<()>;
    fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: windows_core::OutRef<ID3D11SamplerState>) -> windows_core::Result<()>;
    fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: windows_core::OutRef<ID3D11Query>) -> windows_core::Result<()>;
    fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: windows_core::OutRef<ID3D11Predicate>) -> windows_core::Result<()>;
    fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: windows_core::OutRef<ID3D11Counter>) -> windows_core::Result<()>;
    fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: windows_core::OutRef<ID3D11DeviceContext>) -> windows_core::Result<()>;
    fn OpenSharedResource(&self, hresource: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CheckFormatSupport(&self, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(&self, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D11_COUNTER_INFO);
    fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::Result<()>;
    fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetFeatureLevel(&self) -> super::d3dcommon::D3D_FEATURE_LEVEL;
    fn GetCreationFlags(&self) -> u32;
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn GetImmediateContext(&self, ppimmediatecontext: windows_core::OutRef<ID3D11DeviceContext>);
    fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl ID3D11Device_Vtbl {
    pub const fn new<Identity: ID3D11Device_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBuffer<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateBuffer(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn CreateTexture1D<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture1D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture1d)).into()
            }
        }
        unsafe extern "system" fn CreateTexture2D<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture2D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture2d)).into()
            }
        }
        unsafe extern "system" fn CreateTexture3D<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture3D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture3d)).into()
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateShaderResourceView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppsrview)).into()
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateUnorderedAccessView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppuaview)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateRenderTargetView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pprtview)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDepthStencilView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppdepthstencilview)).into()
            }
        }
        unsafe extern "system" fn CreateInputLayout<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateInputLayout(this, core::mem::transmute_copy(&pinputelementdescs), core::mem::transmute_copy(&numelements), core::mem::transmute_copy(&pshaderbytecodewithinputsignature), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppinputlayout)).into()
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, ppvertexshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pclasslinkage), core::mem::transmute_copy(&ppvertexshader)).into()
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateGeometryShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pclasslinkage), core::mem::transmute_copy(&ppgeometryshader)).into()
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateGeometryShaderWithStreamOutput(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&psodeclaration), core::mem::transmute_copy(&numentries), core::mem::transmute_copy(&pbufferstrides), core::mem::transmute_copy(&numstrides), core::mem::transmute_copy(&rasterizedstream), core::mem::transmute_copy(&pclasslinkage), core::mem::transmute_copy(&ppgeometryshader)).into()
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, pppixelshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pclasslinkage), core::mem::transmute_copy(&pppixelshader)).into()
            }
        }
        unsafe extern "system" fn CreateHullShader<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, pphullshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateHullShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pclasslinkage), core::mem::transmute_copy(&pphullshader)).into()
            }
        }
        unsafe extern "system" fn CreateDomainShader<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, ppdomainshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDomainShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pclasslinkage), core::mem::transmute_copy(&ppdomainshader)).into()
            }
        }
        unsafe extern "system" fn CreateComputeShader<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, ppcomputeshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateComputeShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pclasslinkage), core::mem::transmute_copy(&ppcomputeshader)).into()
            }
        }
        unsafe extern "system" fn CreateClassLinkage<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplinkage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CreateClassLinkage(this) {
                    Ok(ok__) => {
                        pplinkage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlendState<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateBlendState(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDepthStencilState(this, core::mem::transmute_copy(&pdepthstencildesc), core::mem::transmute_copy(&ppdepthstencilstate)).into()
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateRasterizerState(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
            }
        }
        unsafe extern "system" fn CreateSamplerState<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateSamplerState(this, core::mem::transmute_copy(&psamplerdesc), core::mem::transmute_copy(&ppsamplerstate)).into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateQuery(this, core::mem::transmute_copy(&pquerydesc), core::mem::transmute_copy(&ppquery)).into()
            }
        }
        unsafe extern "system" fn CreatePredicate<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreatePredicate(this, core::mem::transmute_copy(&ppredicatedesc), core::mem::transmute_copy(&pppredicate)).into()
            }
        }
        unsafe extern "system" fn CreateCounter<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateCounter(this, core::mem::transmute_copy(&pcounterdesc), core::mem::transmute_copy(&ppcounter)).into()
            }
        }
        unsafe extern "system" fn CreateDeferredContext<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDeferredContext(this, core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&ppdeferredcontext)).into()
            }
        }
        unsafe extern "system" fn OpenSharedResource<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::OpenSharedResource(this, core::mem::transmute_copy(&hresource), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
            }
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, pformatsupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CheckFormatSupport(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pformatsupport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CheckMultisampleQualityLevels(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&samplecount)) {
                    Ok(ok__) => {
                        pnumqualitylevels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterinfo: *mut D3D11_COUNTER_INFO) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckCounterInfo(this, core::mem::transmute_copy(&pcounterinfo));
            }
        }
        unsafe extern "system" fn CheckCounter<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckCounter(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pactivecounters), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pnamelength), core::mem::transmute_copy(&szunits), core::mem::transmute_copy(&punitslength), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pdescriptionlength)).into()
            }
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: D3D11_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3dcommon::D3D_FEATURE_LEVEL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetFeatureLevel(this)
            }
        }
        unsafe extern "system" fn GetCreationFlags<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetCreationFlags(this)
            }
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetDeviceRemovedReason(this).into()
            }
        }
        unsafe extern "system" fn GetImmediateContext<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimmediatecontext: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetImmediateContext(this, core::mem::transmute_copy(&ppimmediatecontext));
            }
        }
        unsafe extern "system" fn SetExceptionMode<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, raiseflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetExceptionMode(this, core::mem::transmute_copy(&raiseflags)).into()
            }
        }
        unsafe extern "system" fn GetExceptionMode<Identity: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetExceptionMode(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBuffer: CreateBuffer::<Identity, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            CreateHullShader: CreateHullShader::<Identity, OFFSET>,
            CreateDomainShader: CreateDomainShader::<Identity, OFFSET>,
            CreateComputeShader: CreateComputeShader::<Identity, OFFSET>,
            CreateClassLinkage: CreateClassLinkage::<Identity, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, OFFSET>,
            CreateCounter: CreateCounter::<Identity, OFFSET>,
            CreateDeferredContext: CreateDeferredContext::<Identity, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, OFFSET>,
            CheckCounter: CheckCounter::<Identity, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, OFFSET>,
            GetImmediateContext: GetImmediateContext::<Identity, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11Device {}
windows_core::imp::define_interface!(ID3D11DeviceChild, ID3D11DeviceChild_Vtbl, 0x1841e5c8_16b0_489b_bcc8_44cfb0d5deae);
windows_core::imp::interface_hierarchy!(ID3D11DeviceChild, windows_core::IUnknown);
impl ID3D11DeviceChild {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D11Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize as _, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceChild_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ID3D11DeviceChild_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: windows_core::OutRef<ID3D11Device>);
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ID3D11DeviceChild_Vtbl {
    pub const fn new<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice));
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11DeviceChild {}
windows_core::imp::define_interface!(ID3D11DeviceContext, ID3D11DeviceContext_Vtbl, 0xc0bfa96c_e089_44fb_8eaf_26f8796190da);
impl core::ops::Deref for ID3D11DeviceContext {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11DeviceContext {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11PixelShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetShader)(windows_core::Interface::as_raw(self), ppixelshader.param().abi(), core::mem::transmute(ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11VertexShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetShader)(windows_core::Interface::as_raw(self), pvertexshader.param().abi(), core::mem::transmute(ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexed)(windows_core::Interface::as_raw(self), indexcount, startindexlocation, basevertexlocation);
        }
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), vertexcount, startvertexlocation);
        }
    }
    pub unsafe fn Map<P0>(&self, presource: P0, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: Option<*mut D3D11_MAPPED_SUBRESOURCE>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, maptype, mapflags, pmappedresource.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Unmap<P0>(&self, presource: P0, subresource: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource);
        }
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: windows_core::Param<ID3D11InputLayout>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IASetInputLayout)(windows_core::Interface::as_raw(self), pinputlayout.param().abi());
        }
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*const Option<ID3D11Buffer>>, pstrides: Option<*const u32>, poffsets: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).IASetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _, pstrides.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::dxgiformat::DXGI_FORMAT, offset: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IASetIndexBuffer)(windows_core::Interface::as_raw(self), pindexbuffer.param().abi(), format, offset);
        }
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexedInstanced)(windows_core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation);
        }
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInstanced)(windows_core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation);
        }
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11GeometryShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetShader)(windows_core::Interface::as_raw(self), pshader.param().abi(), core::mem::transmute(ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: D3D11_PRIMITIVE_TOPOLOGY) {
        unsafe {
            (windows_core::Interface::vtable(self).IASetPrimitiveTopology)(windows_core::Interface::as_raw(self), topology);
        }
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn Begin<P0>(&self, pasync: P0)
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self), pasync.param().abi());
        }
    }
    pub unsafe fn End<P0>(&self, pasync: P0)
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self), pasync.param().abi());
        }
    }
    pub unsafe fn GetData<P0>(&self, pasync: P0, pdata: Option<*mut core::ffi::c_void>, datasize: u32, getdataflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pasync.param().abi(), pdata.unwrap_or(core::mem::zeroed()) as _, datasize, getdataflags) }
    }
    pub unsafe fn SetPredication<P0>(&self, ppredicate: P0, predicatevalue: bool)
    where
        P0: windows_core::Param<ID3D11Predicate>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), ppredicate.param().abi(), predicatevalue.into());
        }
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn OMSetRenderTargets<P2>(&self, pprendertargetviews: Option<&[Option<ID3D11RenderTargetView>]>, pdepthstencilview: P2)
    where
        P2: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.map_or(core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.param().abi());
        }
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<P2>(&self, pprendertargetviews: Option<&[Option<ID3D11RenderTargetView>]>, pdepthstencilview: P2, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: Option<*const Option<ID3D11UnorderedAccessView>>, puavinitialcounts: Option<*const u32>)
    where
        P2: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetRenderTargetsAndUnorderedAccessViews)(windows_core::Interface::as_raw(self), pprendertargetviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.map_or(core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.param().abi(), uavstartslot, numuavs, ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _, puavinitialcounts.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: Option<&[f32; 4]>, samplemask: u32)
    where
        P0: windows_core::Param<ID3D11BlendState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetBlendState)(windows_core::Interface::as_raw(self), pblendstate.param().abi(), core::mem::transmute(blendfactor.map_or(core::ptr::null(), |slice| slice.as_ptr())), samplemask);
        }
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: windows_core::Param<ID3D11DepthStencilState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencilstate.param().abi(), stencilref);
        }
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: Option<*const Option<ID3D11Buffer>>, poffsets: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).SOSetTargets)(windows_core::Interface::as_raw(self), numbuffers, ppsotargets.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn DrawAuto(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawAuto)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn DrawIndexedInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexedInstancedIndirect)(windows_core::Interface::as_raw(self), pbufferforargs.param().abi(), alignedbyteoffsetforargs);
        }
    }
    pub unsafe fn DrawInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInstancedIndirect)(windows_core::Interface::as_raw(self), pbufferforargs.param().abi(), alignedbyteoffsetforargs);
        }
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Dispatch)(windows_core::Interface::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz);
        }
    }
    pub unsafe fn DispatchIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DispatchIndirect)(windows_core::Interface::as_raw(self), pbufferforargs.param().abi(), alignedbyteoffsetforargs);
        }
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: windows_core::Param<ID3D11RasterizerState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetState)(windows_core::Interface::as_raw(self), prasterizerstate.param().abi());
        }
    }
    pub unsafe fn RSSetViewports(&self, pviewports: Option<&[D3D11_VIEWPORT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetViewports)(windows_core::Interface::as_raw(self), pviewports.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviewports.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RSSetScissorRects(&self, prects: Option<&[D3D11_RECT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetScissorRects)(windows_core::Interface::as_raw(self), prects.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prects.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CopySubresourceRegion<P0, P5>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P5, srcsubresource: u32, psrcbox: Option<*const D3D11_BOX>)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P5: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopySubresourceRegion)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.param().abi(), srcsubresource, psrcbox.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyResource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), psrcresource.param().abi());
        }
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const D3D11_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, pdstbox.unwrap_or(core::mem::zeroed()) as _, psrcdata, srcrowpitch, srcdepthpitch);
        }
    }
    pub unsafe fn CopyStructureCount<P0, P2>(&self, pdstbuffer: P0, dstalignedbyteoffset: u32, psrcview: P2)
    where
        P0: windows_core::Param<ID3D11Buffer>,
        P2: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyStructureCount)(windows_core::Interface::as_raw(self), pdstbuffer.param().abi(), dstalignedbyteoffset, psrcview.param().abi());
        }
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: &[f32; 4])
    where
        P0: windows_core::Param<ID3D11RenderTargetView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearRenderTargetView)(windows_core::Interface::as_raw(self), prendertargetview.param().abi(), core::mem::transmute(colorrgba.as_ptr()));
        }
    }
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, punorderedaccessview: P0, values: &[u32; 4])
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearUnorderedAccessViewUint)(windows_core::Interface::as_raw(self), punorderedaccessview.param().abi(), core::mem::transmute(values.as_ptr()));
        }
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, punorderedaccessview: P0, values: &[f32; 4])
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearUnorderedAccessViewFloat)(windows_core::Interface::as_raw(self), punorderedaccessview.param().abi(), core::mem::transmute(values.as_ptr()));
        }
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearDepthStencilView)(windows_core::Interface::as_raw(self), pdepthstencilview.param().abi(), clearflags, depth, stencil);
        }
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: windows_core::Param<ID3D11ShaderResourceView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMips)(windows_core::Interface::as_raw(self), pshaderresourceview.param().abi());
        }
    }
    pub unsafe fn SetResourceMinLOD<P0>(&self, presource: P0, minlod: f32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetResourceMinLOD)(windows_core::Interface::as_raw(self), presource.param().abi(), minlod);
        }
    }
    pub unsafe fn GetResourceMinLOD<P0>(&self, presource: P0) -> f32
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResourceMinLOD)(windows_core::Interface::as_raw(self), presource.param().abi()) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn ResolveSubresource<P0, P2>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P2, srcsubresource: u32, format: super::dxgiformat::DXGI_FORMAT)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P2: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, psrcresource.param().abi(), srcsubresource, format);
        }
    }
    pub unsafe fn ExecuteCommandList<P0>(&self, pcommandlist: P0, restorecontextstate: bool)
    where
        P0: windows_core::Param<ID3D11CommandList>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ExecuteCommandList)(windows_core::Interface::as_raw(self), pcommandlist.param().abi(), restorecontextstate.into());
        }
    }
    pub unsafe fn HSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn HSSetShader<P0>(&self, phullshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11HullShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetShader)(windows_core::Interface::as_raw(self), phullshader.param().abi(), core::mem::transmute(ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
    pub unsafe fn HSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn HSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn DSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn DSSetShader<P0>(&self, pdomainshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11DomainShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetShader)(windows_core::Interface::as_raw(self), pdomainshader.param().abi(), core::mem::transmute(ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
    pub unsafe fn DSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn DSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: Option<*const Option<ID3D11UnorderedAccessView>>, puavinitialcounts: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetUnorderedAccessViews)(windows_core::Interface::as_raw(self), startslot, numuavs, ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _, puavinitialcounts.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn CSSetShader<P0>(&self, pcomputeshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11ComputeShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetShader)(windows_core::Interface::as_raw(self), pcomputeshader.param().abi(), core::mem::transmute(ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
    pub unsafe fn CSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut Option<ID3D11PixelShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pppixelshader), ppclassinstances.unwrap_or(core::mem::zeroed()) as _, pnumclassinstances.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut Option<ID3D11VertexShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(ppvertexshader), ppclassinstances.unwrap_or(core::mem::zeroed()) as _, pnumclassinstances.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn IAGetInputLayout(&self) -> windows_core::Result<ID3D11InputLayout> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetInputLayout)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*mut Option<ID3D11Buffer>>, pstrides: Option<*mut u32>, poffsets: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).IAGetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _, pstrides.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: *mut Option<ID3D11Buffer>, format: Option<*mut super::dxgiformat::DXGI_FORMAT>, offset: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).IAGetIndexBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(pindexbuffer), format.unwrap_or(core::mem::zeroed()) as _, offset.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut Option<ID3D11GeometryShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(ppgeometryshader), ppclassinstances.unwrap_or(core::mem::zeroed()) as _, pnumclassinstances.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> D3D11_PRIMITIVE_TOPOLOGY {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetPrimitiveTopology)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GetPredication(&self, pppredicate: *mut Option<ID3D11Predicate>, ppredicatevalue: Option<*mut windows_core::BOOL>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetPredication)(windows_core::Interface::as_raw(self), core::mem::transmute(pppredicate), ppredicatevalue.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: Option<&mut [Option<ID3D11RenderTargetView>]>, ppdepthstencilview: *mut Option<ID3D11DepthStencilView>) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(ppdepthstencilview));
        }
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(&self, pprendertargetviews: Option<&mut [Option<ID3D11RenderTargetView>]>, ppdepthstencilview: *mut Option<ID3D11DepthStencilView>, uavstartslot: u32, ppunorderedaccessviews: Option<&mut [Option<ID3D11UnorderedAccessView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetRenderTargetsAndUnorderedAccessViews)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(ppdepthstencilview), uavstartslot, ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: *mut Option<ID3D11BlendState>, blendfactor: Option<&mut [f32; 4]>, psamplemask: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetBlendState)(windows_core::Interface::as_raw(self), core::mem::transmute(ppblendstate), core::mem::transmute(blendfactor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psamplemask.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut Option<ID3D11DepthStencilState>, pstencilref: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetDepthStencilState)(windows_core::Interface::as_raw(self), core::mem::transmute(ppdepthstencilstate), pstencilref.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn SOGetTargets(&self, ppsotargets: Option<&mut [Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).SOGetTargets)(windows_core::Interface::as_raw(self), ppsotargets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsotargets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn RSGetState(&self) -> windows_core::Result<ID3D11RasterizerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RSGetState)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: Option<*mut D3D11_VIEWPORT>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetViewports)(windows_core::Interface::as_raw(self), pnumviewports as _, pviewports.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: Option<*mut D3D11_RECT>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetScissorRects)(windows_core::Interface::as_raw(self), pnumrects as _, prects.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn HSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn HSGetShader(&self, pphullshader: *mut Option<ID3D11HullShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pphullshader), ppclassinstances.unwrap_or(core::mem::zeroed()) as _, pnumclassinstances.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn HSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn HSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn DSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn DSGetShader(&self, ppdomainshader: *mut Option<ID3D11DomainShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(ppdomainshader), ppclassinstances.unwrap_or(core::mem::zeroed()) as _, pnumclassinstances.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn DSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn DSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CSGetUnorderedAccessViews(&self, startslot: u32, ppunorderedaccessviews: Option<&mut [Option<ID3D11UnorderedAccessView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetUnorderedAccessViews)(windows_core::Interface::as_raw(self), startslot, ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CSGetShader(&self, ppcomputeshader: *mut Option<ID3D11ComputeShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(ppcomputeshader), ppclassinstances.unwrap_or(core::mem::zeroed()) as _, pnumclassinstances.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn CSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn ClearState(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Flush(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetContextFlags)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FinishCommandList(&self, restoredeferredcontextstate: bool, ppcommandlist: Option<*mut Option<ID3D11CommandList>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FinishCommandList)(windows_core::Interface::as_raw(self), restoredeferredcontextstate.into(), ppcommandlist.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceContext_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub VSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub PSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub DrawIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32),
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_MAP, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub PSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub IASetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub IASetVertexBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    #[cfg(feature = "Win32_dxgiformat")]
    pub IASetIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, u32),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    IASetIndexBuffer: usize,
    pub DrawIndexedInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, i32, u32),
    pub DrawInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32),
    pub GSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_d3dcommon")]
    pub IASetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_d3dcommon"))]
    IASetPrimitiveTopology: usize,
    pub VSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL),
    pub GSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub OMSetRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OMSetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32),
    pub OMSetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32, u32),
    pub OMSetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub SOSetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *const u32),
    pub DrawAuto: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub DrawIndexedInstancedIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub DrawInstancedIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
    pub DispatchIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub RSSetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RSSetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D11_VIEWPORT),
    #[cfg(feature = "Win32_windef")]
    pub RSSetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D11_RECT),
    #[cfg(not(feature = "Win32_windef"))]
    RSSetScissorRects: usize,
    pub CopySubresourceRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, u32, *const D3D11_BOX),
    pub CopyResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub UpdateSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_BOX, *const core::ffi::c_void, u32, u32),
    pub CopyStructureCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void),
    pub ClearRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearUnorderedAccessViewUint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u32),
    pub ClearUnorderedAccessViewFloat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, f32, u8),
    pub GenerateMips: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetResourceMinLOD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32),
    pub GetResourceMinLOD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> f32,
    #[cfg(feature = "Win32_dxgiformat")]
    pub ResolveSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, super::dxgiformat::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    ResolveSubresource: usize,
    pub ExecuteCommandList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL),
    pub HSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub HSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub HSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub HSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub DSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetUnorderedAccessViews: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32),
    pub CSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub CSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub PSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub PSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub IAGetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub IAGetVertexBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    #[cfg(feature = "Win32_dxgiformat")]
    pub IAGetIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::dxgiformat::DXGI_FORMAT, *mut u32),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    IAGetIndexBuffer: usize,
    pub GSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    #[cfg(feature = "Win32_d3dcommon")]
    pub IAGetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_d3dcommon"))]
    IAGetPrimitiveTopology: usize,
    pub VSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL),
    pub GSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub OMGetRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub OMGetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub OMGetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut f32, *mut u32),
    pub OMGetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub SOGetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void),
    pub RSGetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub RSGetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D11_VIEWPORT),
    #[cfg(feature = "Win32_windef")]
    pub RSGetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D11_RECT),
    #[cfg(not(feature = "Win32_windef"))]
    RSGetScissorRects: usize,
    pub HSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub HSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub HSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub HSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub DSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetUnorderedAccessViews: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub CSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE,
    pub GetContextFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub FinishCommandList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
pub trait ID3D11DeviceContext_Impl: ID3D11DeviceChild_Impl {
    fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn PSSetShader(&self, ppixelshader: windows_core::Ref<ID3D11PixelShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn VSSetShader(&self, pvertexshader: windows_core::Ref<ID3D11VertexShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&self, vertexcount: u32, startvertexlocation: u32);
    fn Map(&self, presource: windows_core::Ref<ID3D11Resource>, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> windows_core::Result<()>;
    fn Unmap(&self, presource: windows_core::Ref<ID3D11Resource>, subresource: u32);
    fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn IASetInputLayout(&self, pinputlayout: windows_core::Ref<ID3D11InputLayout>);
    fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const Option<ID3D11Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&self, pindexbuffer: windows_core::Ref<ID3D11Buffer>, format: super::dxgiformat::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn GSSetShader(&self, pshader: windows_core::Ref<ID3D11GeometryShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn IASetPrimitiveTopology(&self, topology: D3D11_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn Begin(&self, pasync: windows_core::Ref<ID3D11Asynchronous>);
    fn End(&self, pasync: windows_core::Ref<ID3D11Asynchronous>);
    fn GetData(&self, pasync: windows_core::Ref<ID3D11Asynchronous>, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::Result<()>;
    fn SetPredication(&self, ppredicate: windows_core::Ref<ID3D11Predicate>, predicatevalue: windows_core::BOOL);
    fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn OMSetRenderTargets(&self, numviews: u32, pprendertargetviews: *const Option<ID3D11RenderTargetView>, pdepthstencilview: windows_core::Ref<ID3D11DepthStencilView>);
    fn OMSetRenderTargetsAndUnorderedAccessViews(&self, numrtvs: u32, pprendertargetviews: *const Option<ID3D11RenderTargetView>, pdepthstencilview: windows_core::Ref<ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn OMSetBlendState(&self, pblendstate: windows_core::Ref<ID3D11BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&self, pdepthstencilstate: windows_core::Ref<ID3D11DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const Option<ID3D11Buffer>, poffsets: *const u32);
    fn DrawAuto(&self);
    fn DrawIndexedInstancedIndirect(&self, pbufferforargs: windows_core::Ref<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn DrawInstancedIndirect(&self, pbufferforargs: windows_core::Ref<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn DispatchIndirect(&self, pbufferforargs: windows_core::Ref<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn RSSetState(&self, prasterizerstate: windows_core::Ref<ID3D11RasterizerState>);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D11_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const D3D11_RECT);
    fn CopySubresourceRegion(&self, pdstresource: windows_core::Ref<ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: windows_core::Ref<ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
    fn CopyResource(&self, pdstresource: windows_core::Ref<ID3D11Resource>, psrcresource: windows_core::Ref<ID3D11Resource>);
    fn UpdateSubresource(&self, pdstresource: windows_core::Ref<ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn CopyStructureCount(&self, pdstbuffer: windows_core::Ref<ID3D11Buffer>, dstalignedbyteoffset: u32, psrcview: windows_core::Ref<ID3D11UnorderedAccessView>);
    fn ClearRenderTargetView(&self, prendertargetview: windows_core::Ref<ID3D11RenderTargetView>, colorrgba: *const f32);
    fn ClearUnorderedAccessViewUint(&self, punorderedaccessview: windows_core::Ref<ID3D11UnorderedAccessView>, values: *const u32);
    fn ClearUnorderedAccessViewFloat(&self, punorderedaccessview: windows_core::Ref<ID3D11UnorderedAccessView>, values: *const f32);
    fn ClearDepthStencilView(&self, pdepthstencilview: windows_core::Ref<ID3D11DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&self, pshaderresourceview: windows_core::Ref<ID3D11ShaderResourceView>);
    fn SetResourceMinLOD(&self, presource: windows_core::Ref<ID3D11Resource>, minlod: f32);
    fn GetResourceMinLOD(&self, presource: windows_core::Ref<ID3D11Resource>) -> f32;
    fn ResolveSubresource(&self, pdstresource: windows_core::Ref<ID3D11Resource>, dstsubresource: u32, psrcresource: windows_core::Ref<ID3D11Resource>, srcsubresource: u32, format: super::dxgiformat::DXGI_FORMAT);
    fn ExecuteCommandList(&self, pcommandlist: windows_core::Ref<ID3D11CommandList>, restorecontextstate: windows_core::BOOL);
    fn HSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn HSSetShader(&self, phullshader: windows_core::Ref<ID3D11HullShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn HSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn HSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn DSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn DSSetShader(&self, pdomainshader: windows_core::Ref<ID3D11DomainShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn DSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn CSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn CSSetShader(&self, pcomputeshader: windows_core::Ref<ID3D11ComputeShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn CSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn CSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn PSGetShader(&self, pppixelshader: windows_core::OutRef<ID3D11PixelShader>, ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn VSGetShader(&self, ppvertexshader: windows_core::OutRef<ID3D11VertexShader>, ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn IAGetInputLayout(&self, ppinputlayout: windows_core::OutRef<ID3D11InputLayout>);
    fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: windows_core::OutRef<ID3D11Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&self, pindexbuffer: windows_core::OutRef<ID3D11Buffer>, format: *mut super::dxgiformat::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn GSGetShader(&self, ppgeometryshader: windows_core::OutRef<ID3D11GeometryShader>, ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn IAGetPrimitiveTopology(&self, ptopology: *mut D3D11_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn GetPredication(&self, pppredicate: windows_core::OutRef<ID3D11Predicate>, ppredicatevalue: *mut windows_core::BOOL);
    fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut Option<ID3D11RenderTargetView>, ppdepthstencilview: windows_core::OutRef<ID3D11DepthStencilView>);
    fn OMGetRenderTargetsAndUnorderedAccessViews(&self, numrtvs: u32, pprendertargetviews: *mut Option<ID3D11RenderTargetView>, ppdepthstencilview: windows_core::OutRef<ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut Option<ID3D11UnorderedAccessView>);
    fn OMGetBlendState(&self, ppblendstate: windows_core::OutRef<ID3D11BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&self, ppdepthstencilstate: windows_core::OutRef<ID3D11DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut Option<ID3D11Buffer>);
    fn RSGetState(&self, pprasterizerstate: windows_core::OutRef<ID3D11RasterizerState>);
    fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT);
    fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: *mut D3D11_RECT);
    fn HSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn HSGetShader(&self, pphullshader: windows_core::OutRef<ID3D11HullShader>, ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn HSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn HSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn DSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn DSGetShader(&self, ppdomainshader: windows_core::OutRef<ID3D11DomainShader>, ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn DSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn DSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn CSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn CSGetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut Option<ID3D11UnorderedAccessView>);
    fn CSGetShader(&self, ppcomputeshader: windows_core::OutRef<ID3D11ComputeShader>, ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn CSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn CSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn ClearState(&self);
    fn Flush(&self);
    fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE;
    fn GetContextFlags(&self) -> u32;
    fn FinishCommandList(&self, restoredeferredcontextstate: windows_core::BOOL, ppcommandlist: windows_core::OutRef<ID3D11CommandList>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl ID3D11DeviceContext_Vtbl {
    pub const fn new<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn PSSetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSSetShader(this, core::mem::transmute_copy(&ppixelshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances));
            }
        }
        unsafe extern "system" fn PSSetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn VSSetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSSetShader(this, core::mem::transmute_copy(&pvertexshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances));
            }
        }
        unsafe extern "system" fn DrawIndexed<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawIndexed(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation));
            }
        }
        unsafe extern "system" fn Draw<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Draw(this, core::mem::transmute_copy(&vertexcount), core::mem::transmute_copy(&startvertexlocation));
            }
        }
        unsafe extern "system" fn Map<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Map(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags), core::mem::transmute_copy(&pmappedresource)).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Unmap(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&subresource));
            }
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn IASetInputLayout<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputlayout: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IASetInputLayout(this, core::mem::transmute_copy(&pinputlayout));
            }
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut core::ffi::c_void, pstrides: *const u32, poffsets: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IASetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets));
            }
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, offset: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IASetIndexBuffer(this, core::mem::transmute_copy(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset));
            }
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawIndexedInstanced(this, core::mem::transmute_copy(&indexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation), core::mem::transmute_copy(&startinstancelocation));
            }
        }
        unsafe extern "system" fn DrawInstanced<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawInstanced(this, core::mem::transmute_copy(&vertexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startvertexlocation), core::mem::transmute_copy(&startinstancelocation));
            }
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn GSSetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSSetShader(this, core::mem::transmute_copy(&pshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances));
            }
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, topology: D3D11_PRIMITIVE_TOPOLOGY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IASetPrimitiveTopology(this, core::mem::transmute_copy(&topology));
            }
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn VSSetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn Begin<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Begin(this, core::mem::transmute_copy(&pasync));
            }
        }
        unsafe extern "system" fn End<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::End(this, core::mem::transmute_copy(&pasync));
            }
        }
        unsafe extern "system" fn GetData<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetData(this, core::mem::transmute_copy(&pasync), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&getdataflags)).into()
            }
        }
        unsafe extern "system" fn SetPredication<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicate: *mut core::ffi::c_void, predicatevalue: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::SetPredication(this, core::mem::transmute_copy(&ppredicate), core::mem::transmute_copy(&predicatevalue));
            }
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn GSSetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMSetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&pdepthstencilview));
            }
        }
        unsafe extern "system" fn OMSetRenderTargetsAndUnorderedAccessViews<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrtvs: u32, pprendertargetviews: *const *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const *mut core::ffi::c_void, puavinitialcounts: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMSetRenderTargetsAndUnorderedAccessViews(this, core::mem::transmute_copy(&numrtvs), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&pdepthstencilview), core::mem::transmute_copy(&uavstartslot), core::mem::transmute_copy(&numuavs), core::mem::transmute_copy(&ppunorderedaccessviews), core::mem::transmute_copy(&puavinitialcounts));
            }
        }
        unsafe extern "system" fn OMSetBlendState<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstate: *mut core::ffi::c_void, blendfactor: *const f32, samplemask: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMSetBlendState(this, core::mem::transmute_copy(&pblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&samplemask));
            }
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilstate: *mut core::ffi::c_void, stencilref: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMSetDepthStencilState(this, core::mem::transmute_copy(&pdepthstencilstate), core::mem::transmute_copy(&stencilref));
            }
        }
        unsafe extern "system" fn SOSetTargets<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut core::ffi::c_void, poffsets: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::SOSetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets), core::mem::transmute_copy(&poffsets));
            }
        }
        unsafe extern "system" fn DrawAuto<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawAuto(this);
            }
        }
        unsafe extern "system" fn DrawIndexedInstancedIndirect<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferforargs: *mut core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawIndexedInstancedIndirect(this, core::mem::transmute_copy(&pbufferforargs), core::mem::transmute_copy(&alignedbyteoffsetforargs));
            }
        }
        unsafe extern "system" fn DrawInstancedIndirect<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferforargs: *mut core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawInstancedIndirect(this, core::mem::transmute_copy(&pbufferforargs), core::mem::transmute_copy(&alignedbyteoffsetforargs));
            }
        }
        unsafe extern "system" fn Dispatch<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Dispatch(this, core::mem::transmute_copy(&threadgroupcountx), core::mem::transmute_copy(&threadgroupcounty), core::mem::transmute_copy(&threadgroupcountz));
            }
        }
        unsafe extern "system" fn DispatchIndirect<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferforargs: *mut core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DispatchIndirect(this, core::mem::transmute_copy(&pbufferforargs), core::mem::transmute_copy(&alignedbyteoffsetforargs));
            }
        }
        unsafe extern "system" fn RSSetState<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerstate: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSSetState(this, core::mem::transmute_copy(&prasterizerstate));
            }
        }
        unsafe extern "system" fn RSSetViewports<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: u32, pviewports: *const D3D11_VIEWPORT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSSetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports));
            }
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: u32, prects: *const D3D11_RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSSetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects));
            }
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CopySubresourceRegion(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), core::mem::transmute_copy(&dstz), core::mem::transmute_copy(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox));
            }
        }
        unsafe extern "system" fn CopyResource<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, psrcresource: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CopyResource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&psrcresource));
            }
        }
        unsafe extern "system" fn UpdateSubresource<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::UpdateSubresource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch));
            }
        }
        unsafe extern "system" fn CopyStructureCount<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstbuffer: *mut core::ffi::c_void, dstalignedbyteoffset: u32, psrcview: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CopyStructureCount(this, core::mem::transmute_copy(&pdstbuffer), core::mem::transmute_copy(&dstalignedbyteoffset), core::mem::transmute_copy(&psrcview));
            }
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertargetview: *mut core::ffi::c_void, colorrgba: *const f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearRenderTargetView(this, core::mem::transmute_copy(&prendertargetview), core::mem::transmute_copy(&colorrgba));
            }
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punorderedaccessview: *mut core::ffi::c_void, values: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearUnorderedAccessViewUint(this, core::mem::transmute_copy(&punorderedaccessview), core::mem::transmute_copy(&values));
            }
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punorderedaccessview: *mut core::ffi::c_void, values: *const f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearUnorderedAccessViewFloat(this, core::mem::transmute_copy(&punorderedaccessview), core::mem::transmute_copy(&values));
            }
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearDepthStencilView(this, core::mem::transmute_copy(&pdepthstencilview), core::mem::transmute_copy(&clearflags), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&stencil));
            }
        }
        unsafe extern "system" fn GenerateMips<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderresourceview: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GenerateMips(this, core::mem::transmute_copy(&pshaderresourceview));
            }
        }
        unsafe extern "system" fn SetResourceMinLOD<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, minlod: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::SetResourceMinLOD(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&minlod));
            }
        }
        unsafe extern "system" fn GetResourceMinLOD<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetResourceMinLOD(this, core::mem::transmute_copy(&presource))
            }
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, format: super::dxgiformat::DXGI_FORMAT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ResolveSubresource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&format));
            }
        }
        unsafe extern "system" fn ExecuteCommandList<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandlist: *mut core::ffi::c_void, restorecontextstate: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ExecuteCommandList(this, core::mem::transmute_copy(&pcommandlist), core::mem::transmute_copy(&restorecontextstate));
            }
        }
        unsafe extern "system" fn HSSetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn HSSetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phullshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSSetShader(this, core::mem::transmute_copy(&phullshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances));
            }
        }
        unsafe extern "system" fn HSSetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn HSSetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn DSSetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn DSSetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomainshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSSetShader(this, core::mem::transmute_copy(&pdomainshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances));
            }
        }
        unsafe extern "system" fn DSSetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn DSSetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn CSSetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn CSSetUnorderedAccessViews<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const *mut core::ffi::c_void, puavinitialcounts: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetUnorderedAccessViews(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numuavs), core::mem::transmute_copy(&ppunorderedaccessviews), core::mem::transmute_copy(&puavinitialcounts));
            }
        }
        unsafe extern "system" fn CSSetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomputeshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetShader(this, core::mem::transmute_copy(&pcomputeshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances));
            }
        }
        unsafe extern "system" fn CSSetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn CSSetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn PSGetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppixelshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSGetShader(this, core::mem::transmute_copy(&pppixelshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances));
            }
        }
        unsafe extern "system" fn PSGetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn VSGetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvertexshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSGetShader(this, core::mem::transmute_copy(&ppvertexshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances));
            }
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinputlayout: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IAGetInputLayout(this, core::mem::transmute_copy(&ppinputlayout));
            }
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IAGetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets));
            }
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut *mut core::ffi::c_void, format: *mut super::dxgiformat::DXGI_FORMAT, offset: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IAGetIndexBuffer(this, core::mem::transmute_copy(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset));
            }
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn GSGetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSGetShader(this, core::mem::transmute_copy(&ppgeometryshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances));
            }
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut D3D11_PRIMITIVE_TOPOLOGY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IAGetPrimitiveTopology(this, core::mem::transmute_copy(&ptopology));
            }
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn VSGetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn GetPredication<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppredicate: *mut *mut core::ffi::c_void, ppredicatevalue: *mut windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetPredication(this, core::mem::transmute_copy(&pppredicate), core::mem::transmute_copy(&ppredicatevalue));
            }
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn GSGetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut core::ffi::c_void, ppdepthstencilview: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMGetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&ppdepthstencilview));
            }
        }
        unsafe extern "system" fn OMGetRenderTargetsAndUnorderedAccessViews<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrtvs: u32, pprendertargetviews: *mut *mut core::ffi::c_void, ppdepthstencilview: *mut *mut core::ffi::c_void, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMGetRenderTargetsAndUnorderedAccessViews(this, core::mem::transmute_copy(&numrtvs), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&ppdepthstencilview), core::mem::transmute_copy(&uavstartslot), core::mem::transmute_copy(&numuavs), core::mem::transmute_copy(&ppunorderedaccessviews));
            }
        }
        unsafe extern "system" fn OMGetBlendState<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblendstate: *mut *mut core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMGetBlendState(this, core::mem::transmute_copy(&ppblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&psamplemask));
            }
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdepthstencilstate: *mut *mut core::ffi::c_void, pstencilref: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMGetDepthStencilState(this, core::mem::transmute_copy(&ppdepthstencilstate), core::mem::transmute_copy(&pstencilref));
            }
        }
        unsafe extern "system" fn SOGetTargets<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::SOGetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets));
            }
        }
        unsafe extern "system" fn RSGetState<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprasterizerstate: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSGetState(this, core::mem::transmute_copy(&pprasterizerstate));
            }
        }
        unsafe extern "system" fn RSGetViewports<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSGetViewports(this, core::mem::transmute_copy(&pnumviewports), core::mem::transmute_copy(&pviewports));
            }
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumrects: *mut u32, prects: *mut D3D11_RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSGetScissorRects(this, core::mem::transmute_copy(&pnumrects), core::mem::transmute_copy(&prects));
            }
        }
        unsafe extern "system" fn HSGetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn HSGetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphullshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSGetShader(this, core::mem::transmute_copy(&pphullshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances));
            }
        }
        unsafe extern "system" fn HSGetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn HSGetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn DSGetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn DSGetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdomainshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSGetShader(this, core::mem::transmute_copy(&ppdomainshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances));
            }
        }
        unsafe extern "system" fn DSGetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn DSGetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn CSGetShaderResources<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn CSGetUnorderedAccessViews<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetUnorderedAccessViews(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numuavs), core::mem::transmute_copy(&ppunorderedaccessviews));
            }
        }
        unsafe extern "system" fn CSGetShader<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomputeshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetShader(this, core::mem::transmute_copy(&ppcomputeshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances));
            }
        }
        unsafe extern "system" fn CSGetSamplers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn CSGetConstantBuffers<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn ClearState<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearState(this);
            }
        }
        unsafe extern "system" fn Flush<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Flush(this);
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetContextFlags<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetContextFlags(this)
            }
        }
        unsafe extern "system" fn FinishCommandList<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restoredeferredcontextstate: windows_core::BOOL, ppcommandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::FinishCommandList(this, core::mem::transmute_copy(&restoredeferredcontextstate), core::mem::transmute_copy(&ppcommandlist)).into()
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, OFFSET>,
            PSSetShader: PSSetShader::<Identity, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, OFFSET>,
            VSSetShader: VSSetShader::<Identity, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, OFFSET>,
            GSSetShader: GSSetShader::<Identity, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, OFFSET>,
            Begin: Begin::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            SetPredication: SetPredication::<Identity, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews: OMSetRenderTargetsAndUnorderedAccessViews::<Identity, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, OFFSET>,
            DrawAuto: DrawAuto::<Identity, OFFSET>,
            DrawIndexedInstancedIndirect: DrawIndexedInstancedIndirect::<Identity, OFFSET>,
            DrawInstancedIndirect: DrawInstancedIndirect::<Identity, OFFSET>,
            Dispatch: Dispatch::<Identity, OFFSET>,
            DispatchIndirect: DispatchIndirect::<Identity, OFFSET>,
            RSSetState: RSSetState::<Identity, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, OFFSET>,
            CopyResource: CopyResource::<Identity, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, OFFSET>,
            CopyStructureCount: CopyStructureCount::<Identity, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Identity, OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Identity, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, OFFSET>,
            GenerateMips: GenerateMips::<Identity, OFFSET>,
            SetResourceMinLOD: SetResourceMinLOD::<Identity, OFFSET>,
            GetResourceMinLOD: GetResourceMinLOD::<Identity, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, OFFSET>,
            ExecuteCommandList: ExecuteCommandList::<Identity, OFFSET>,
            HSSetShaderResources: HSSetShaderResources::<Identity, OFFSET>,
            HSSetShader: HSSetShader::<Identity, OFFSET>,
            HSSetSamplers: HSSetSamplers::<Identity, OFFSET>,
            HSSetConstantBuffers: HSSetConstantBuffers::<Identity, OFFSET>,
            DSSetShaderResources: DSSetShaderResources::<Identity, OFFSET>,
            DSSetShader: DSSetShader::<Identity, OFFSET>,
            DSSetSamplers: DSSetSamplers::<Identity, OFFSET>,
            DSSetConstantBuffers: DSSetConstantBuffers::<Identity, OFFSET>,
            CSSetShaderResources: CSSetShaderResources::<Identity, OFFSET>,
            CSSetUnorderedAccessViews: CSSetUnorderedAccessViews::<Identity, OFFSET>,
            CSSetShader: CSSetShader::<Identity, OFFSET>,
            CSSetSamplers: CSSetSamplers::<Identity, OFFSET>,
            CSSetConstantBuffers: CSSetConstantBuffers::<Identity, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, OFFSET>,
            PSGetShader: PSGetShader::<Identity, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, OFFSET>,
            VSGetShader: VSGetShader::<Identity, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, OFFSET>,
            GSGetShader: GSGetShader::<Identity, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, OFFSET>,
            GetPredication: GetPredication::<Identity, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews: OMGetRenderTargetsAndUnorderedAccessViews::<Identity, OFFSET>,
            OMGetBlendState: OMGetBlendState::<Identity, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, OFFSET>,
            RSGetState: RSGetState::<Identity, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, OFFSET>,
            HSGetShaderResources: HSGetShaderResources::<Identity, OFFSET>,
            HSGetShader: HSGetShader::<Identity, OFFSET>,
            HSGetSamplers: HSGetSamplers::<Identity, OFFSET>,
            HSGetConstantBuffers: HSGetConstantBuffers::<Identity, OFFSET>,
            DSGetShaderResources: DSGetShaderResources::<Identity, OFFSET>,
            DSGetShader: DSGetShader::<Identity, OFFSET>,
            DSGetSamplers: DSGetSamplers::<Identity, OFFSET>,
            DSGetConstantBuffers: DSGetConstantBuffers::<Identity, OFFSET>,
            CSGetShaderResources: CSGetShaderResources::<Identity, OFFSET>,
            CSGetUnorderedAccessViews: CSGetUnorderedAccessViews::<Identity, OFFSET>,
            CSGetShader: CSGetShader::<Identity, OFFSET>,
            CSGetSamplers: CSGetSamplers::<Identity, OFFSET>,
            CSGetConstantBuffers: CSGetConstantBuffers::<Identity, OFFSET>,
            ClearState: ClearState::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetContextFlags: GetContextFlags::<Identity, OFFSET>,
            FinishCommandList: FinishCommandList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D11DeviceContext {}
windows_core::imp::define_interface!(ID3D11DomainShader, ID3D11DomainShader_Vtbl, 0xf582c508_0f36_490c_9977_31eece268cfa);
impl core::ops::Deref for ID3D11DomainShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DomainShader, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DomainShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11DomainShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11DomainShader_Vtbl {
    pub const fn new<Identity: ID3D11DomainShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DomainShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11DomainShader {}
windows_core::imp::define_interface!(ID3D11GeometryShader, ID3D11GeometryShader_Vtbl, 0x38325b96_effb_4022_ba02_2e795b70275c);
impl core::ops::Deref for ID3D11GeometryShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11GeometryShader, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11GeometryShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11GeometryShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11GeometryShader_Vtbl {
    pub const fn new<Identity: ID3D11GeometryShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11GeometryShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11GeometryShader {}
windows_core::imp::define_interface!(ID3D11HullShader, ID3D11HullShader_Vtbl, 0x8e5c6061_628a_4c8e_8264_bbe45cb3d5dd);
impl core::ops::Deref for ID3D11HullShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11HullShader, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11HullShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11HullShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11HullShader_Vtbl {
    pub const fn new<Identity: ID3D11HullShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11HullShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11HullShader {}
windows_core::imp::define_interface!(ID3D11InputLayout, ID3D11InputLayout_Vtbl, 0xe4819ddc_4cf0_4025_bd26_5de82a3e07b7);
impl core::ops::Deref for ID3D11InputLayout {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11InputLayout, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11InputLayout_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11InputLayout_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11InputLayout_Vtbl {
    pub const fn new<Identity: ID3D11InputLayout_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11InputLayout as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11InputLayout {}
windows_core::imp::define_interface!(ID3D11PixelShader, ID3D11PixelShader_Vtbl, 0xea82e40d_51dc_4f33_93d4_db7c9125ae8c);
impl core::ops::Deref for ID3D11PixelShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11PixelShader, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11PixelShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11PixelShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11PixelShader_Vtbl {
    pub const fn new<Identity: ID3D11PixelShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11PixelShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11PixelShader {}
windows_core::imp::define_interface!(ID3D11Predicate, ID3D11Predicate_Vtbl, 0x9eb576dd_9f77_4d86_81aa_8bab5fe490e2);
impl core::ops::Deref for ID3D11Predicate {
    type Target = ID3D11Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Predicate, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Asynchronous, ID3D11Query);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Predicate_Vtbl {
    pub base__: ID3D11Query_Vtbl,
}
pub trait ID3D11Predicate_Impl: ID3D11Query_Impl {}
impl ID3D11Predicate_Vtbl {
    pub const fn new<Identity: ID3D11Predicate_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D11Query_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Predicate as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Asynchronous as windows_core::Interface>::IID || iid == &<ID3D11Query as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Predicate {}
windows_core::imp::define_interface!(ID3D11Query, ID3D11Query_Vtbl, 0xd6c00747_87b7_425e_b84d_44d108560afd);
impl core::ops::Deref for ID3D11Query {
    type Target = ID3D11Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Query, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Asynchronous);
impl ID3D11Query {
    pub unsafe fn GetDesc(&self) -> D3D11_QUERY_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Query_Vtbl {
    pub base__: ID3D11Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_QUERY_DESC),
}
pub trait ID3D11Query_Impl: ID3D11Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_QUERY_DESC);
}
impl ID3D11Query_Vtbl {
    pub const fn new<Identity: ID3D11Query_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11Query_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_QUERY_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Query_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11Asynchronous_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Query as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Asynchronous as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Query {}
windows_core::imp::define_interface!(ID3D11RasterizerState, ID3D11RasterizerState_Vtbl, 0x9bb4ab81_ab1a_4d8f_b506_fc04200b6ee7);
impl core::ops::Deref for ID3D11RasterizerState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RasterizerState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11RasterizerState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_RASTERIZER_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11RasterizerState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RASTERIZER_DESC),
}
pub trait ID3D11RasterizerState_Impl: ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_RASTERIZER_DESC);
}
impl ID3D11RasterizerState_Vtbl {
    pub const fn new<Identity: ID3D11RasterizerState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11RasterizerState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11RasterizerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RasterizerState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11RasterizerState {}
windows_core::imp::define_interface!(ID3D11RenderTargetView, ID3D11RenderTargetView_Vtbl, 0xdfdba067_0b8d_4865_875b_d7b4516cc164);
impl core::ops::Deref for ID3D11RenderTargetView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RenderTargetView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11RenderTargetView {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11RenderTargetView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RENDER_TARGET_VIEW_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D11RenderTargetView_Impl: ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D11RenderTargetView_Vtbl {
    pub const fn new<Identity: ID3D11RenderTargetView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11RenderTargetView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11RenderTargetView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D11RenderTargetView {}
windows_core::imp::define_interface!(ID3D11Resource, ID3D11Resource_Vtbl, 0xdc8e63f3_d12b_4952_b47b_5e45026a862d);
impl core::ops::Deref for ID3D11Resource {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Resource, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11Resource {
    pub unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetEvictionPriority)(windows_core::Interface::as_raw(self), evictionpriority);
        }
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetEvictionPriority)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Resource_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RESOURCE_DIMENSION),
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait ID3D11Resource_Impl: ID3D11DeviceChild_Impl {
    fn GetType(&self, presourcedimension: *mut D3D11_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&self, evictionpriority: u32);
    fn GetEvictionPriority(&self) -> u32;
}
impl ID3D11Resource_Vtbl {
    pub const fn new<Identity: ID3D11Resource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcedimension: *mut D3D11_RESOURCE_DIMENSION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Resource_Impl::GetType(this, core::mem::transmute_copy(&presourcedimension));
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Resource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority));
            }
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Resource_Impl::GetEvictionPriority(this)
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Resource as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Resource {}
windows_core::imp::define_interface!(ID3D11SamplerState, ID3D11SamplerState_Vtbl, 0xda6fea51_564c_4487_9810_f0d0f9b4e3a5);
impl core::ops::Deref for ID3D11SamplerState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11SamplerState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11SamplerState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SAMPLER_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11SamplerState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SAMPLER_DESC),
}
pub trait ID3D11SamplerState_Impl: ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SAMPLER_DESC);
}
impl ID3D11SamplerState_Vtbl {
    pub const fn new<Identity: ID3D11SamplerState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11SamplerState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SAMPLER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11SamplerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11SamplerState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11SamplerState {}
windows_core::imp::define_interface!(ID3D11ShaderResourceView, ID3D11ShaderResourceView_Vtbl, 0xb0e06fe0_8192_4e1a_b1ca_36d7414710b2);
impl core::ops::Deref for ID3D11ShaderResourceView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ShaderResourceView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11ShaderResourceView {
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ShaderResourceView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_RESOURCE_VIEW_DESC),
    #[cfg(not(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat")))]
    GetDesc: usize,
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11ShaderResourceView_Impl: ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl ID3D11ShaderResourceView_Vtbl {
    pub const fn new<Identity: ID3D11ShaderResourceView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11ShaderResourceView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderResourceView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11ShaderResourceView {}
windows_core::imp::define_interface!(ID3D11Texture1D, ID3D11Texture1D_Vtbl, 0xf8fb5c27_c6b3_4f75_a4c8_439af2ef564c);
impl core::ops::Deref for ID3D11Texture1D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Texture1D, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
impl ID3D11Texture1D {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE1D_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Texture1D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE1D_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D11Texture1D_Impl: ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D11Texture1D_Vtbl {
    pub const fn new<Identity: ID3D11Texture1D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11Texture1D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE1D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Texture1D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11Resource_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture1D as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D11Texture1D {}
windows_core::imp::define_interface!(ID3D11Texture2D, ID3D11Texture2D_Vtbl, 0x6f15aaf2_d208_4e89_9ab4_489535d34f9c);
impl core::ops::Deref for ID3D11Texture2D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Texture2D, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
impl ID3D11Texture2D {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Texture2D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE2D_DESC),
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    GetDesc: usize,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11Texture2D_Impl: ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC);
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D11Texture2D_Vtbl {
    pub const fn new<Identity: ID3D11Texture2D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11Texture2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Texture2D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11Resource_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture2D as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11Texture2D {}
windows_core::imp::define_interface!(ID3D11Texture3D, ID3D11Texture3D_Vtbl, 0x037e866e_f56d_4357_a8af_9dabbe6e250e);
impl core::ops::Deref for ID3D11Texture3D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Texture3D, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
impl ID3D11Texture3D {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE3D_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Texture3D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE3D_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D11Texture3D_Impl: ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D11Texture3D_Vtbl {
    pub const fn new<Identity: ID3D11Texture3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11Texture3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Texture3D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11Resource_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture3D as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D11Texture3D {}
windows_core::imp::define_interface!(ID3D11UnorderedAccessView, ID3D11UnorderedAccessView_Vtbl, 0x28acf509_7f5c_48f6_8611_f316010a6380);
impl core::ops::Deref for ID3D11UnorderedAccessView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11UnorderedAccessView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11UnorderedAccessView {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11UnorderedAccessView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_UNORDERED_ACCESS_VIEW_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D11UnorderedAccessView_Impl: ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D11UnorderedAccessView_Vtbl {
    pub const fn new<Identity: ID3D11UnorderedAccessView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11UnorderedAccessView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11UnorderedAccessView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D11UnorderedAccessView {}
windows_core::imp::define_interface!(ID3D11VertexShader, ID3D11VertexShader_Vtbl, 0x3b301d64_d678_4289_8897_22f8928b72f3);
impl core::ops::Deref for ID3D11VertexShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VertexShader, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VertexShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11VertexShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11VertexShader_Vtbl {
    pub const fn new<Identity: ID3D11VertexShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VertexShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VertexShader {}
windows_core::imp::define_interface!(ID3D11VideoContext, ID3D11VideoContext_Vtbl, 0x61f21c45_3c0e_4a74_9cea_67100d9ad5e4);
impl core::ops::Deref for ID3D11VideoContext {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoContext, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VideoContext {
    pub unsafe fn GetDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDecoderBuffer)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), r#type, pbuffersize as _, ppbuffer as _) }
    }
    pub unsafe fn ReleaseDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDecoderBuffer)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), r#type) }
    }
    pub unsafe fn DecoderBeginFrame<P0, P1>(&self, pdecoder: P0, pview: P1, contentkeysize: u32, pcontentkey: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
        P1: windows_core::Param<ID3D11VideoDecoderOutputView>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecoderBeginFrame)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pview.param().abi(), contentkeysize, pcontentkey.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DecoderEndFrame<P0>(&self, pdecoder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecoderEndFrame)(windows_core::Interface::as_raw(self), pdecoder.param().abi()) }
    }
    pub unsafe fn SubmitDecoderBuffers<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubmitDecoderBuffers)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pbufferdesc.len().try_into().unwrap(), core::mem::transmute(pbufferdesc.as_ptr())) }
    }
    pub unsafe fn DecoderExtension<P0>(&self, pdecoder: P0, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecoderExtension)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pextensiondata) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn VideoProcessorSetOutputTargetRect<P0>(&self, pvideoprocessor: P0, enable: bool, prect: Option<*const super::windef::RECT>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputTargetRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), enable.into(), prect.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn VideoProcessorSetOutputBackgroundColor<P0>(&self, pvideoprocessor: P0, ycbcr: bool, pcolor: *const D3D11_VIDEO_COLOR)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputBackgroundColor)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), ycbcr.into(), pcolor);
        }
    }
    pub unsafe fn VideoProcessorSetOutputColorSpace<P0>(&self, pvideoprocessor: P0, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputColorSpace)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pcolorspace);
        }
    }
    pub unsafe fn VideoProcessorSetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputAlphaFillMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), alphafillmode, streamindex);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn VideoProcessorSetOutputConstriction<P0>(&self, pvideoprocessor: P0, enable: bool, size: super::windef::SIZE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputConstriction)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), enable.into(), core::mem::transmute(size));
        }
    }
    pub unsafe fn VideoProcessorSetOutputStereoMode<P0>(&self, pvideoprocessor: P0, enable: bool)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputStereoMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), enable.into());
        }
    }
    pub unsafe fn VideoProcessorSetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe { (windows_core::Interface::vtable(self).VideoProcessorSetOutputExtension)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pextensionguid, datasize, pdata) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn VideoProcessorGetOutputTargetRect<P0>(&self, pvideoprocessor: P0, enabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputTargetRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), enabled as _, prect as _);
        }
    }
    pub unsafe fn VideoProcessorGetOutputBackgroundColor<P0>(&self, pvideoprocessor: P0, pycbcr: *mut windows_core::BOOL, pcolor: *mut D3D11_VIDEO_COLOR)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputBackgroundColor)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pycbcr as _, pcolor as _);
        }
    }
    pub unsafe fn VideoProcessorGetOutputColorSpace<P0>(&self, pvideoprocessor: P0) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputColorSpace)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), &mut result__);
            result__
        }
    }
    pub unsafe fn VideoProcessorGetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputAlphaFillMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), palphafillmode as _, pstreamindex as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn VideoProcessorGetOutputConstriction<P0>(&self, pvideoprocessor: P0, penabled: *mut windows_core::BOOL, psize: *mut super::windef::SIZE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputConstriction)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), penabled as _, psize as _);
        }
    }
    pub unsafe fn VideoProcessorGetOutputStereoMode<P0>(&self, pvideoprocessor: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputStereoMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), &mut result__);
            result__
        }
    }
    pub unsafe fn VideoProcessorGetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe { (windows_core::Interface::vtable(self).VideoProcessorGetOutputExtension)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pextensionguid, datasize, pdata as _) }
    }
    pub unsafe fn VideoProcessorSetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamFrameFormat)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, frameformat);
        }
    }
    pub unsafe fn VideoProcessorSetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamColorSpace)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pcolorspace);
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn VideoProcessorSetStreamOutputRate<P0>(&self, pvideoprocessor: P0, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: bool, pcustomrate: Option<*const super::dxgicommon::DXGI_RATIONAL>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamOutputRate)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, outputrate, repeatframe.into(), pcustomrate.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn VideoProcessorSetStreamSourceRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool, prect: Option<*const super::windef::RECT>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamSourceRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into(), prect.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn VideoProcessorSetStreamDestRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool, prect: Option<*const super::windef::RECT>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamDestRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into(), prect.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn VideoProcessorSetStreamAlpha<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool, alpha: f32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamAlpha)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into(), alpha);
        }
    }
    pub unsafe fn VideoProcessorSetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: Option<&[u32]>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamPalette)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pentries.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pentries.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn VideoProcessorSetStreamPixelAspectRatio<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool, psourceaspectratio: Option<*const super::dxgicommon::DXGI_RATIONAL>, pdestinationaspectratio: Option<*const super::dxgicommon::DXGI_RATIONAL>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamPixelAspectRatio)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into(), psourceaspectratio.unwrap_or(core::mem::zeroed()) as _, pdestinationaspectratio.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn VideoProcessorSetStreamLumaKey<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool, lower: f32, upper: f32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamLumaKey)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into(), lower, upper);
        }
    }
    pub unsafe fn VideoProcessorSetStreamStereoFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: bool, baseviewframe0: bool, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamStereoFormat)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into(), format, leftviewframe0.into(), baseviewframe0.into(), flipmode, monooffset);
        }
    }
    pub unsafe fn VideoProcessorSetStreamAutoProcessingMode<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamAutoProcessingMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into());
        }
    }
    pub unsafe fn VideoProcessorSetStreamFilter<P0>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: bool, level: i32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamFilter)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, filter, enable.into(), level);
        }
    }
    pub unsafe fn VideoProcessorSetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe { (windows_core::Interface::vtable(self).VideoProcessorSetStreamExtension)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pextensionguid, datasize, pdata) }
    }
    pub unsafe fn VideoProcessorGetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_FRAME_FORMAT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamFrameFormat)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, &mut result__);
            result__
        }
    }
    pub unsafe fn VideoProcessorGetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamColorSpace)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, &mut result__);
            result__
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn VideoProcessorGetStreamOutputRate<P0>(&self, pvideoprocessor: P0, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut windows_core::BOOL, pcustomrate: *mut super::dxgicommon::DXGI_RATIONAL)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamOutputRate)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, poutputrate as _, prepeatframe as _, pcustomrate as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn VideoProcessorGetStreamSourceRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamSourceRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled as _, prect as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn VideoProcessorGetStreamDestRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamDestRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled as _, prect as _);
        }
    }
    pub unsafe fn VideoProcessorGetStreamAlpha<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut windows_core::BOOL, palpha: *mut f32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamAlpha)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled as _, palpha as _);
        }
    }
    pub unsafe fn VideoProcessorGetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: &mut [u32])
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamPalette)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pentries.len().try_into().unwrap(), core::mem::transmute(pentries.as_ptr()));
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn VideoProcessorGetStreamPixelAspectRatio<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut windows_core::BOOL, psourceaspectratio: *mut super::dxgicommon::DXGI_RATIONAL, pdestinationaspectratio: *mut super::dxgicommon::DXGI_RATIONAL)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamPixelAspectRatio)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled as _, psourceaspectratio as _, pdestinationaspectratio as _);
        }
    }
    pub unsafe fn VideoProcessorGetStreamLumaKey<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut windows_core::BOOL, plower: *mut f32, pupper: *mut f32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamLumaKey)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled as _, plower as _, pupper as _);
        }
    }
    pub unsafe fn VideoProcessorGetStreamStereoFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut windows_core::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut windows_core::BOOL, pbaseviewframe0: *mut windows_core::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamStereoFormat)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penable as _, pformat as _, pleftviewframe0 as _, pbaseviewframe0 as _, pflipmode as _, monooffset as _);
        }
    }
    pub unsafe fn VideoProcessorGetStreamAutoProcessingMode<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> windows_core::BOOL
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamAutoProcessingMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, &mut result__);
            result__
        }
    }
    pub unsafe fn VideoProcessorGetStreamFilter<P0>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut windows_core::BOOL, plevel: *mut i32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamFilter)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, filter, penabled as _, plevel as _);
        }
    }
    pub unsafe fn VideoProcessorGetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe { (windows_core::Interface::vtable(self).VideoProcessorGetStreamExtension)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pextensionguid, datasize, pdata as _) }
    }
    pub unsafe fn VideoProcessorBlt<P0, P1>(&self, pvideoprocessor: P0, pview: P1, outputframe: u32, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<ID3D11VideoProcessorOutputView>,
    {
        unsafe { (windows_core::Interface::vtable(self).VideoProcessorBlt)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pview.param().abi(), outputframe, pstreams.len().try_into().unwrap(), core::mem::transmute(pstreams.as_ptr())) }
    }
    pub unsafe fn NegotiateCryptoSessionKeyExchange<P0>(&self, pcryptosession: P0, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        unsafe { (windows_core::Interface::vtable(self).NegotiateCryptoSessionKeyExchange)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), datasize, pdata as _) }
    }
    pub unsafe fn EncryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, ivsize: u32, piv: *mut core::ffi::c_void)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
        P1: windows_core::Param<ID3D11Texture2D>,
        P2: windows_core::Param<ID3D11Texture2D>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).EncryptionBlt)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), psrcsurface.param().abi(), pdstsurface.param().abi(), ivsize, piv as _);
        }
    }
    pub unsafe fn DecryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, pencryptedblockinfo: Option<*const D3D11_ENCRYPTED_BLOCK_INFO>, contentkeysize: u32, pcontentkey: Option<*const core::ffi::c_void>, ivsize: u32, piv: *mut core::ffi::c_void)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
        P1: windows_core::Param<ID3D11Texture2D>,
        P2: windows_core::Param<ID3D11Texture2D>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DecryptionBlt)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), psrcsurface.param().abi(), pdstsurface.param().abi(), pencryptedblockinfo.unwrap_or(core::mem::zeroed()) as _, contentkeysize, pcontentkey.unwrap_or(core::mem::zeroed()) as _, ivsize, piv as _);
        }
    }
    pub unsafe fn StartSessionKeyRefresh<P0>(&self, pcryptosession: P0, randomnumbersize: u32, prandomnumber: *mut core::ffi::c_void)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).StartSessionKeyRefresh)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), randomnumbersize, prandomnumber as _);
        }
    }
    pub unsafe fn FinishSessionKeyRefresh<P0>(&self, pcryptosession: P0)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FinishSessionKeyRefresh)(windows_core::Interface::as_raw(self), pcryptosession.param().abi());
        }
    }
    pub unsafe fn GetEncryptionBltKey<P0>(&self, pcryptosession: P0, keysize: u32, preadbackkey: *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetEncryptionBltKey)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), keysize, preadbackkey as _) }
    }
    pub unsafe fn NegotiateAuthenticatedChannelKeyExchange<P0>(&self, pchannel: P0, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        unsafe { (windows_core::Interface::vtable(self).NegotiateAuthenticatedChannelKeyExchange)(windows_core::Interface::as_raw(self), pchannel.param().abi(), datasize, pdata as _) }
    }
    pub unsafe fn QueryAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryAuthenticatedChannel)(windows_core::Interface::as_raw(self), pchannel.param().abi(), inputsize, pinput, outputsize, poutput as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn ConfigureAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConfigureAuthenticatedChannel)(windows_core::Interface::as_raw(self), pchannel.param().abi(), inputsize, pinput, poutput as _) }
    }
    pub unsafe fn VideoProcessorSetStreamRotation<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool, rotation: D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamRotation)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into(), rotation);
        }
    }
    pub unsafe fn VideoProcessorGetStreamRotation<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut windows_core::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamRotation)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penable as _, protation as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoContext_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDecoderBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D11_VIDEO_DECODER_BUFFER_TYPE, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseDecoderBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D11_VIDEO_DECODER_BUFFER_TYPE) -> windows_core::HRESULT,
    pub DecoderBeginFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub DecoderEndFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubmitDecoderBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> windows_core::HRESULT,
    pub DecoderExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_EXTENSION) -> APP_DEPRECATED_HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub VideoProcessorSetOutputTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *const super::windef::RECT),
    #[cfg(not(feature = "Win32_windef"))]
    VideoProcessorSetOutputTargetRect: usize,
    pub VideoProcessorSetOutputBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *const D3D11_VIDEO_COLOR),
    pub VideoProcessorSetOutputColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE),
    pub VideoProcessorSetOutputAlphaFillMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, u32),
    #[cfg(feature = "Win32_windef")]
    pub VideoProcessorSetOutputConstriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, super::windef::SIZE),
    #[cfg(not(feature = "Win32_windef"))]
    VideoProcessorSetOutputConstriction: usize,
    pub VideoProcessorSetOutputStereoMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL),
    pub VideoProcessorSetOutputExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> APP_DEPRECATED_HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub VideoProcessorGetOutputTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut super::windef::RECT),
    #[cfg(not(feature = "Win32_windef"))]
    VideoProcessorGetOutputTargetRect: usize,
    pub VideoProcessorGetOutputBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut D3D11_VIDEO_COLOR),
    pub VideoProcessorGetOutputColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE),
    pub VideoProcessorGetOutputAlphaFillMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, *mut u32),
    #[cfg(feature = "Win32_windef")]
    pub VideoProcessorGetOutputConstriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut super::windef::SIZE),
    #[cfg(not(feature = "Win32_windef"))]
    VideoProcessorGetOutputConstriction: usize,
    pub VideoProcessorGetOutputStereoMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL),
    pub VideoProcessorGetOutputExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, *mut core::ffi::c_void) -> APP_DEPRECATED_HRESULT,
    pub VideoProcessorSetStreamFrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_FRAME_FORMAT),
    pub VideoProcessorSetStreamColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE),
    #[cfg(feature = "Win32_dxgicommon")]
    pub VideoProcessorSetStreamOutputRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, windows_core::BOOL, *const super::dxgicommon::DXGI_RATIONAL),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    VideoProcessorSetStreamOutputRate: usize,
    #[cfg(feature = "Win32_windef")]
    pub VideoProcessorSetStreamSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, *const super::windef::RECT),
    #[cfg(not(feature = "Win32_windef"))]
    VideoProcessorSetStreamSourceRect: usize,
    #[cfg(feature = "Win32_windef")]
    pub VideoProcessorSetStreamDestRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, *const super::windef::RECT),
    #[cfg(not(feature = "Win32_windef"))]
    VideoProcessorSetStreamDestRect: usize,
    pub VideoProcessorSetStreamAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, f32),
    pub VideoProcessorSetStreamPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const u32),
    #[cfg(feature = "Win32_dxgicommon")]
    pub VideoProcessorSetStreamPixelAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, *const super::dxgicommon::DXGI_RATIONAL, *const super::dxgicommon::DXGI_RATIONAL),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    VideoProcessorSetStreamPixelAspectRatio: usize,
    pub VideoProcessorSetStreamLumaKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, f32, f32),
    pub VideoProcessorSetStreamStereoFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, windows_core::BOOL, windows_core::BOOL, D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, i32),
    pub VideoProcessorSetStreamAutoProcessingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL),
    pub VideoProcessorSetStreamFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_PROCESSOR_FILTER, windows_core::BOOL, i32),
    pub VideoProcessorSetStreamExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32, *const core::ffi::c_void) -> APP_DEPRECATED_HRESULT,
    pub VideoProcessorGetStreamFrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut D3D11_VIDEO_FRAME_FORMAT),
    pub VideoProcessorGetStreamColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE),
    #[cfg(feature = "Win32_dxgicommon")]
    pub VideoProcessorGetStreamOutputRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, *mut windows_core::BOOL, *mut super::dxgicommon::DXGI_RATIONAL),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    VideoProcessorGetStreamOutputRate: usize,
    #[cfg(feature = "Win32_windef")]
    pub VideoProcessorGetStreamSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut super::windef::RECT),
    #[cfg(not(feature = "Win32_windef"))]
    VideoProcessorGetStreamSourceRect: usize,
    #[cfg(feature = "Win32_windef")]
    pub VideoProcessorGetStreamDestRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut super::windef::RECT),
    #[cfg(not(feature = "Win32_windef"))]
    VideoProcessorGetStreamDestRect: usize,
    pub VideoProcessorGetStreamAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut f32),
    pub VideoProcessorGetStreamPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut u32),
    #[cfg(feature = "Win32_dxgicommon")]
    pub VideoProcessorGetStreamPixelAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut super::dxgicommon::DXGI_RATIONAL, *mut super::dxgicommon::DXGI_RATIONAL),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    VideoProcessorGetStreamPixelAspectRatio: usize,
    pub VideoProcessorGetStreamLumaKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut f32, *mut f32),
    pub VideoProcessorGetStreamStereoFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, *mut windows_core::BOOL, *mut windows_core::BOOL, *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, *mut i32),
    pub VideoProcessorGetStreamAutoProcessingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL),
    pub VideoProcessorGetStreamFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_PROCESSOR_FILTER, *mut windows_core::BOOL, *mut i32),
    pub VideoProcessorGetStreamExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32, *mut core::ffi::c_void) -> APP_DEPRECATED_HRESULT,
    pub VideoProcessorBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const D3D11_VIDEO_PROCESSOR_STREAM) -> windows_core::HRESULT,
    pub NegotiateCryptoSessionKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EncryptionBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void),
    pub DecryptionBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_ENCRYPTED_BLOCK_INFO, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void),
    pub StartSessionKeyRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void),
    pub FinishSessionKeyRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetEncryptionBltKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NegotiateAuthenticatedChannelKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAuthenticatedChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub ConfigureAuthenticatedChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    ConfigureAuthenticatedChannel: usize,
    pub VideoProcessorSetStreamRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, D3D11_VIDEO_PROCESSOR_ROTATION),
    pub VideoProcessorGetStreamRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut D3D11_VIDEO_PROCESSOR_ROTATION),
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait ID3D11VideoContext_Impl: ID3D11DeviceChild_Impl {
    fn GetDecoderBuffer(&self, pdecoder: windows_core::Ref<ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseDecoderBuffer(&self, pdecoder: windows_core::Ref<ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> windows_core::Result<()>;
    fn DecoderBeginFrame(&self, pdecoder: windows_core::Ref<ID3D11VideoDecoder>, pview: windows_core::Ref<ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn DecoderEndFrame(&self, pdecoder: windows_core::Ref<ID3D11VideoDecoder>) -> windows_core::Result<()>;
    fn SubmitDecoderBuffers(&self, pdecoder: windows_core::Ref<ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> windows_core::Result<()>;
    fn DecoderExtension(&self, pdecoder: windows_core::Ref<ID3D11VideoDecoder>, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorSetOutputTargetRect(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, enable: windows_core::BOOL, prect: *const super::windef::RECT);
    fn VideoProcessorSetOutputBackgroundColor(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, ycbcr: windows_core::BOOL, pcolor: *const D3D11_VIDEO_COLOR);
    fn VideoProcessorSetOutputColorSpace(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetOutputAlphaFillMode(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32);
    fn VideoProcessorSetOutputConstriction(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, enable: windows_core::BOOL, size: &super::windef::SIZE);
    fn VideoProcessorSetOutputStereoMode(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, enable: windows_core::BOOL);
    fn VideoProcessorSetOutputExtension(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorGetOutputTargetRect(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, enabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT);
    fn VideoProcessorGetOutputBackgroundColor(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, pycbcr: *mut windows_core::BOOL, pcolor: *mut D3D11_VIDEO_COLOR);
    fn VideoProcessorGetOutputColorSpace(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetOutputAlphaFillMode(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32);
    fn VideoProcessorGetOutputConstriction(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, penabled: *mut windows_core::BOOL, psize: *mut super::windef::SIZE);
    fn VideoProcessorGetOutputStereoMode(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, penabled: *mut windows_core::BOOL);
    fn VideoProcessorGetOutputExtension(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorSetStreamFrameFormat(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorSetStreamColorSpace(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetStreamOutputRate(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: windows_core::BOOL, pcustomrate: *const super::dxgicommon::DXGI_RATIONAL);
    fn VideoProcessorSetStreamSourceRect(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL, prect: *const super::windef::RECT);
    fn VideoProcessorSetStreamDestRect(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL, prect: *const super::windef::RECT);
    fn VideoProcessorSetStreamAlpha(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL, alpha: f32);
    fn VideoProcessorSetStreamPalette(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *const u32);
    fn VideoProcessorSetStreamPixelAspectRatio(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL, psourceaspectratio: *const super::dxgicommon::DXGI_RATIONAL, pdestinationaspectratio: *const super::dxgicommon::DXGI_RATIONAL);
    fn VideoProcessorSetStreamLumaKey(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL, lower: f32, upper: f32);
    fn VideoProcessorSetStreamStereoFormat(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: windows_core::BOOL, baseviewframe0: windows_core::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32);
    fn VideoProcessorSetStreamAutoProcessingMode(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL);
    fn VideoProcessorSetStreamFilter(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: windows_core::BOOL, level: i32);
    fn VideoProcessorSetStreamExtension(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorGetStreamFrameFormat(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorGetStreamColorSpace(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetStreamOutputRate(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut windows_core::BOOL, pcustomrate: *mut super::dxgicommon::DXGI_RATIONAL);
    fn VideoProcessorGetStreamSourceRect(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT);
    fn VideoProcessorGetStreamDestRect(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT);
    fn VideoProcessorGetStreamAlpha(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut windows_core::BOOL, palpha: *mut f32);
    fn VideoProcessorGetStreamPalette(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *mut u32);
    fn VideoProcessorGetStreamPixelAspectRatio(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut windows_core::BOOL, psourceaspectratio: *mut super::dxgicommon::DXGI_RATIONAL, pdestinationaspectratio: *mut super::dxgicommon::DXGI_RATIONAL);
    fn VideoProcessorGetStreamLumaKey(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut windows_core::BOOL, plower: *mut f32, pupper: *mut f32);
    fn VideoProcessorGetStreamStereoFormat(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, penable: *mut windows_core::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut windows_core::BOOL, pbaseviewframe0: *mut windows_core::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32);
    fn VideoProcessorGetStreamAutoProcessingMode(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut windows_core::BOOL);
    fn VideoProcessorGetStreamFilter(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut windows_core::BOOL, plevel: *mut i32);
    fn VideoProcessorGetStreamExtension(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorBlt(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, pview: windows_core::Ref<ID3D11VideoProcessorOutputView>, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> windows_core::Result<()>;
    fn NegotiateCryptoSessionKeyExchange(&self, pcryptosession: windows_core::Ref<ID3D11CryptoSession>, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EncryptionBlt(&self, pcryptosession: windows_core::Ref<ID3D11CryptoSession>, psrcsurface: windows_core::Ref<ID3D11Texture2D>, pdstsurface: windows_core::Ref<ID3D11Texture2D>, ivsize: u32, piv: *mut core::ffi::c_void);
    fn DecryptionBlt(&self, pcryptosession: windows_core::Ref<ID3D11CryptoSession>, psrcsurface: windows_core::Ref<ID3D11Texture2D>, pdstsurface: windows_core::Ref<ID3D11Texture2D>, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const core::ffi::c_void, ivsize: u32, piv: *mut core::ffi::c_void);
    fn StartSessionKeyRefresh(&self, pcryptosession: windows_core::Ref<ID3D11CryptoSession>, randomnumbersize: u32, prandomnumber: *mut core::ffi::c_void);
    fn FinishSessionKeyRefresh(&self, pcryptosession: windows_core::Ref<ID3D11CryptoSession>);
    fn GetEncryptionBltKey(&self, pcryptosession: windows_core::Ref<ID3D11CryptoSession>, keysize: u32, preadbackkey: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn NegotiateAuthenticatedChannelKeyExchange(&self, pchannel: windows_core::Ref<ID3D11AuthenticatedChannel>, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn QueryAuthenticatedChannel(&self, pchannel: windows_core::Ref<ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ConfigureAuthenticatedChannel(&self, pchannel: windows_core::Ref<ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> windows_core::Result<()>;
    fn VideoProcessorSetStreamRotation(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION);
    fn VideoProcessorGetStreamRotation(&self, pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>, streamindex: u32, penable: *mut windows_core::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION);
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_windef", feature = "Win32_winnt"))]
impl ID3D11VideoContext_Vtbl {
    pub const fn new<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDecoderBuffer<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::GetDecoderBuffer(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pbuffersize), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn ReleaseDecoderBuffer<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::ReleaseDecoderBuffer(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn DecoderBeginFrame<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, pview: *mut core::ffi::c_void, contentkeysize: u32, pcontentkey: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::DecoderBeginFrame(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&pview), core::mem::transmute_copy(&contentkeysize), core::mem::transmute_copy(&pcontentkey)).into()
            }
        }
        unsafe extern "system" fn DecoderEndFrame<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::DecoderEndFrame(this, core::mem::transmute_copy(&pdecoder)).into()
            }
        }
        unsafe extern "system" fn SubmitDecoderBuffers<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::SubmitDecoderBuffers(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&pbufferdesc)).into()
            }
        }
        unsafe extern "system" fn DecoderExtension<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::DecoderExtension(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&pextensiondata))
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputTargetRect<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, enable: windows_core::BOOL, prect: *const super::windef::RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputTargetRect(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&prect));
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputBackgroundColor<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, ycbcr: windows_core::BOOL, pcolor: *const D3D11_VIDEO_COLOR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputBackgroundColor(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&ycbcr), core::mem::transmute_copy(&pcolor));
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputColorSpace(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&pcolorspace));
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputAlphaFillMode<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputAlphaFillMode(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&alphafillmode), core::mem::transmute_copy(&streamindex));
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputConstriction<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, enable: windows_core::BOOL, size: super::windef::SIZE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputConstriction(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&enable), core::mem::transmute(&size));
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputStereoMode<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, enable: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputStereoMode(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&enable));
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputExtension<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputExtension(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&pextensionguid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata))
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputTargetRect<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, enabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputTargetRect(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&enabled), core::mem::transmute_copy(&prect));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputBackgroundColor<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pycbcr: *mut windows_core::BOOL, pcolor: *mut D3D11_VIDEO_COLOR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputBackgroundColor(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&pycbcr), core::mem::transmute_copy(&pcolor));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputColorSpace(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&pcolorspace));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputAlphaFillMode<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputAlphaFillMode(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&palphafillmode), core::mem::transmute_copy(&pstreamindex));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputConstriction<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, penabled: *mut windows_core::BOOL, psize: *mut super::windef::SIZE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputConstriction(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&psize));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputStereoMode<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, penabled: *mut windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputStereoMode(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&penabled));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputExtension<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputExtension(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&pextensionguid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata))
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamFrameFormat<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamFrameFormat(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&frameformat));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamColorSpace(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pcolorspace));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamOutputRate<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: windows_core::BOOL, pcustomrate: *const super::dxgicommon::DXGI_RATIONAL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamOutputRate(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&outputrate), core::mem::transmute_copy(&repeatframe), core::mem::transmute_copy(&pcustomrate));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamSourceRect<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL, prect: *const super::windef::RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamSourceRect(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&prect));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamDestRect<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL, prect: *const super::windef::RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamDestRect(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&prect));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamAlpha<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL, alpha: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamAlpha(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&alpha));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamPalette<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, count: u32, pentries: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamPalette(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&count), core::mem::transmute_copy(&pentries));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamPixelAspectRatio<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL, psourceaspectratio: *const super::dxgicommon::DXGI_RATIONAL, pdestinationaspectratio: *const super::dxgicommon::DXGI_RATIONAL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamPixelAspectRatio(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&psourceaspectratio), core::mem::transmute_copy(&pdestinationaspectratio));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamLumaKey<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL, lower: f32, upper: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamLumaKey(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&lower), core::mem::transmute_copy(&upper));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamStereoFormat<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: windows_core::BOOL, baseviewframe0: windows_core::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamStereoFormat(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&format), core::mem::transmute_copy(&leftviewframe0), core::mem::transmute_copy(&baseviewframe0), core::mem::transmute_copy(&flipmode), core::mem::transmute_copy(&monooffset));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamAutoProcessingMode<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamAutoProcessingMode(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamFilter<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: windows_core::BOOL, level: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamFilter(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&filter), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&level));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamExtension<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamExtension(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pextensionguid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata))
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamFrameFormat<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamFrameFormat(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pframeformat));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamColorSpace(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pcolorspace));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamOutputRate<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut windows_core::BOOL, pcustomrate: *mut super::dxgicommon::DXGI_RATIONAL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamOutputRate(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&poutputrate), core::mem::transmute_copy(&prepeatframe), core::mem::transmute_copy(&pcustomrate));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamSourceRect<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamSourceRect(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&prect));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamDestRect<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut windows_core::BOOL, prect: *mut super::windef::RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamDestRect(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&prect));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamAlpha<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut windows_core::BOOL, palpha: *mut f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamAlpha(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&palpha));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamPalette<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, count: u32, pentries: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamPalette(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&count), core::mem::transmute_copy(&pentries));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamPixelAspectRatio<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut windows_core::BOOL, psourceaspectratio: *mut super::dxgicommon::DXGI_RATIONAL, pdestinationaspectratio: *mut super::dxgicommon::DXGI_RATIONAL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamPixelAspectRatio(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&psourceaspectratio), core::mem::transmute_copy(&pdestinationaspectratio));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamLumaKey<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut windows_core::BOOL, plower: *mut f32, pupper: *mut f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamLumaKey(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&plower), core::mem::transmute_copy(&pupper));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamStereoFormat<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penable: *mut windows_core::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut windows_core::BOOL, pbaseviewframe0: *mut windows_core::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamStereoFormat(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penable), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pleftviewframe0), core::mem::transmute_copy(&pbaseviewframe0), core::mem::transmute_copy(&pflipmode), core::mem::transmute_copy(&monooffset));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamAutoProcessingMode<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamAutoProcessingMode(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamFilter<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut windows_core::BOOL, plevel: *mut i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamFilter(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&filter), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&plevel));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamExtension<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamExtension(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pextensionguid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata))
            }
        }
        unsafe extern "system" fn VideoProcessorBlt<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pview: *mut core::ffi::c_void, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorBlt(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&pview), core::mem::transmute_copy(&outputframe), core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreams)).into()
            }
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchange<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::NegotiateCryptoSessionKeyExchange(this, core::mem::transmute_copy(&pcryptosession), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn EncryptionBlt<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, pdstsurface: *mut core::ffi::c_void, ivsize: u32, piv: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::EncryptionBlt(this, core::mem::transmute_copy(&pcryptosession), core::mem::transmute_copy(&psrcsurface), core::mem::transmute_copy(&pdstsurface), core::mem::transmute_copy(&ivsize), core::mem::transmute_copy(&piv));
            }
        }
        unsafe extern "system" fn DecryptionBlt<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, pdstsurface: *mut core::ffi::c_void, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const core::ffi::c_void, ivsize: u32, piv: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::DecryptionBlt(this, core::mem::transmute_copy(&pcryptosession), core::mem::transmute_copy(&psrcsurface), core::mem::transmute_copy(&pdstsurface), core::mem::transmute_copy(&pencryptedblockinfo), core::mem::transmute_copy(&contentkeysize), core::mem::transmute_copy(&pcontentkey), core::mem::transmute_copy(&ivsize), core::mem::transmute_copy(&piv));
            }
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, randomnumbersize: u32, prandomnumber: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::StartSessionKeyRefresh(this, core::mem::transmute_copy(&pcryptosession), core::mem::transmute_copy(&randomnumbersize), core::mem::transmute_copy(&prandomnumber));
            }
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::FinishSessionKeyRefresh(this, core::mem::transmute_copy(&pcryptosession));
            }
        }
        unsafe extern "system" fn GetEncryptionBltKey<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, keysize: u32, preadbackkey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::GetEncryptionBltKey(this, core::mem::transmute_copy(&pcryptosession), core::mem::transmute_copy(&keysize), core::mem::transmute_copy(&preadbackkey)).into()
            }
        }
        unsafe extern "system" fn NegotiateAuthenticatedChannelKeyExchange<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::NegotiateAuthenticatedChannelKeyExchange(this, core::mem::transmute_copy(&pchannel), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn QueryAuthenticatedChannel<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::QueryAuthenticatedChannel(this, core::mem::transmute_copy(&pchannel), core::mem::transmute_copy(&inputsize), core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&outputsize), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn ConfigureAuthenticatedChannel<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::ConfigureAuthenticatedChannel(this, core::mem::transmute_copy(&pchannel), core::mem::transmute_copy(&inputsize), core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamRotation<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamRotation(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&rotation));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamRotation<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penable: *mut windows_core::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamRotation(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penable), core::mem::transmute_copy(&protation));
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetDecoderBuffer: GetDecoderBuffer::<Identity, OFFSET>,
            ReleaseDecoderBuffer: ReleaseDecoderBuffer::<Identity, OFFSET>,
            DecoderBeginFrame: DecoderBeginFrame::<Identity, OFFSET>,
            DecoderEndFrame: DecoderEndFrame::<Identity, OFFSET>,
            SubmitDecoderBuffers: SubmitDecoderBuffers::<Identity, OFFSET>,
            DecoderExtension: DecoderExtension::<Identity, OFFSET>,
            VideoProcessorSetOutputTargetRect: VideoProcessorSetOutputTargetRect::<Identity, OFFSET>,
            VideoProcessorSetOutputBackgroundColor: VideoProcessorSetOutputBackgroundColor::<Identity, OFFSET>,
            VideoProcessorSetOutputColorSpace: VideoProcessorSetOutputColorSpace::<Identity, OFFSET>,
            VideoProcessorSetOutputAlphaFillMode: VideoProcessorSetOutputAlphaFillMode::<Identity, OFFSET>,
            VideoProcessorSetOutputConstriction: VideoProcessorSetOutputConstriction::<Identity, OFFSET>,
            VideoProcessorSetOutputStereoMode: VideoProcessorSetOutputStereoMode::<Identity, OFFSET>,
            VideoProcessorSetOutputExtension: VideoProcessorSetOutputExtension::<Identity, OFFSET>,
            VideoProcessorGetOutputTargetRect: VideoProcessorGetOutputTargetRect::<Identity, OFFSET>,
            VideoProcessorGetOutputBackgroundColor: VideoProcessorGetOutputBackgroundColor::<Identity, OFFSET>,
            VideoProcessorGetOutputColorSpace: VideoProcessorGetOutputColorSpace::<Identity, OFFSET>,
            VideoProcessorGetOutputAlphaFillMode: VideoProcessorGetOutputAlphaFillMode::<Identity, OFFSET>,
            VideoProcessorGetOutputConstriction: VideoProcessorGetOutputConstriction::<Identity, OFFSET>,
            VideoProcessorGetOutputStereoMode: VideoProcessorGetOutputStereoMode::<Identity, OFFSET>,
            VideoProcessorGetOutputExtension: VideoProcessorGetOutputExtension::<Identity, OFFSET>,
            VideoProcessorSetStreamFrameFormat: VideoProcessorSetStreamFrameFormat::<Identity, OFFSET>,
            VideoProcessorSetStreamColorSpace: VideoProcessorSetStreamColorSpace::<Identity, OFFSET>,
            VideoProcessorSetStreamOutputRate: VideoProcessorSetStreamOutputRate::<Identity, OFFSET>,
            VideoProcessorSetStreamSourceRect: VideoProcessorSetStreamSourceRect::<Identity, OFFSET>,
            VideoProcessorSetStreamDestRect: VideoProcessorSetStreamDestRect::<Identity, OFFSET>,
            VideoProcessorSetStreamAlpha: VideoProcessorSetStreamAlpha::<Identity, OFFSET>,
            VideoProcessorSetStreamPalette: VideoProcessorSetStreamPalette::<Identity, OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio: VideoProcessorSetStreamPixelAspectRatio::<Identity, OFFSET>,
            VideoProcessorSetStreamLumaKey: VideoProcessorSetStreamLumaKey::<Identity, OFFSET>,
            VideoProcessorSetStreamStereoFormat: VideoProcessorSetStreamStereoFormat::<Identity, OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode: VideoProcessorSetStreamAutoProcessingMode::<Identity, OFFSET>,
            VideoProcessorSetStreamFilter: VideoProcessorSetStreamFilter::<Identity, OFFSET>,
            VideoProcessorSetStreamExtension: VideoProcessorSetStreamExtension::<Identity, OFFSET>,
            VideoProcessorGetStreamFrameFormat: VideoProcessorGetStreamFrameFormat::<Identity, OFFSET>,
            VideoProcessorGetStreamColorSpace: VideoProcessorGetStreamColorSpace::<Identity, OFFSET>,
            VideoProcessorGetStreamOutputRate: VideoProcessorGetStreamOutputRate::<Identity, OFFSET>,
            VideoProcessorGetStreamSourceRect: VideoProcessorGetStreamSourceRect::<Identity, OFFSET>,
            VideoProcessorGetStreamDestRect: VideoProcessorGetStreamDestRect::<Identity, OFFSET>,
            VideoProcessorGetStreamAlpha: VideoProcessorGetStreamAlpha::<Identity, OFFSET>,
            VideoProcessorGetStreamPalette: VideoProcessorGetStreamPalette::<Identity, OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio: VideoProcessorGetStreamPixelAspectRatio::<Identity, OFFSET>,
            VideoProcessorGetStreamLumaKey: VideoProcessorGetStreamLumaKey::<Identity, OFFSET>,
            VideoProcessorGetStreamStereoFormat: VideoProcessorGetStreamStereoFormat::<Identity, OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode: VideoProcessorGetStreamAutoProcessingMode::<Identity, OFFSET>,
            VideoProcessorGetStreamFilter: VideoProcessorGetStreamFilter::<Identity, OFFSET>,
            VideoProcessorGetStreamExtension: VideoProcessorGetStreamExtension::<Identity, OFFSET>,
            VideoProcessorBlt: VideoProcessorBlt::<Identity, OFFSET>,
            NegotiateCryptoSessionKeyExchange: NegotiateCryptoSessionKeyExchange::<Identity, OFFSET>,
            EncryptionBlt: EncryptionBlt::<Identity, OFFSET>,
            DecryptionBlt: DecryptionBlt::<Identity, OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Identity, OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Identity, OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Identity, OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange: NegotiateAuthenticatedChannelKeyExchange::<Identity, OFFSET>,
            QueryAuthenticatedChannel: QueryAuthenticatedChannel::<Identity, OFFSET>,
            ConfigureAuthenticatedChannel: ConfigureAuthenticatedChannel::<Identity, OFFSET>,
            VideoProcessorSetStreamRotation: VideoProcessorSetStreamRotation::<Identity, OFFSET>,
            VideoProcessorGetStreamRotation: VideoProcessorGetStreamRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11VideoContext {}
windows_core::imp::define_interface!(ID3D11VideoDecoder, ID3D11VideoDecoder_Vtbl, 0x3c9c5b51_995d_48d1_9b8d_fa5caeded65c);
impl core::ops::Deref for ID3D11VideoDecoder {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoDecoder, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VideoDecoder {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetCreationParameters(&self, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCreationParameters)(windows_core::Interface::as_raw(self), pvideodesc as _, pconfig as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDriverHandle(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDriverHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoDecoder_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetCreationParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_DECODER_DESC, *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetCreationParameters: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetDriverHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDriverHandle: usize,
}
#[cfg(all(feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait ID3D11VideoDecoder_Impl: ID3D11DeviceChild_Impl {
    fn GetCreationParameters(&self, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<()>;
    fn GetDriverHandle(&self) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(all(feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl ID3D11VideoDecoder_Vtbl {
    pub const fn new<Identity: ID3D11VideoDecoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCreationParameters<Identity: ID3D11VideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDecoder_Impl::GetCreationParameters(this, core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&pconfig)).into()
            }
        }
        unsafe extern "system" fn GetDriverHandle<Identity: ID3D11VideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdriverhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDecoder_Impl::GetDriverHandle(this) {
                    Ok(ok__) => {
                        pdriverhandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetCreationParameters: GetCreationParameters::<Identity, OFFSET>,
            GetDriverHandle: GetDriverHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDecoder as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11VideoDecoder {}
windows_core::imp::define_interface!(ID3D11VideoDecoderOutputView, ID3D11VideoDecoderOutputView_Vtbl, 0xc2931aea_2a85_4f20_860f_fba1fd256e18);
impl core::ops::Deref for ID3D11VideoDecoderOutputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoDecoderOutputView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11VideoDecoderOutputView {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoDecoderOutputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC),
}
pub trait ID3D11VideoDecoderOutputView_Impl: ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC);
}
impl ID3D11VideoDecoderOutputView_Vtbl {
    pub const fn new<Identity: ID3D11VideoDecoderOutputView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11VideoDecoderOutputView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDecoderOutputView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDecoderOutputView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VideoDecoderOutputView {}
windows_core::imp::define_interface!(ID3D11VideoDevice, ID3D11VideoDevice_Vtbl, 0x10ec4d5b_975a_4689_b9e4_d0aac30fe333);
windows_core::imp::interface_hierarchy!(ID3D11VideoDevice, windows_core::IUnknown);
impl ID3D11VideoDevice {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateVideoDecoder(&self, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<ID3D11VideoDecoder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVideoDecoder)(windows_core::Interface::as_raw(self), pvideodesc, pconfig, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateVideoProcessor<P0>(&self, penum: P0, rateconversionindex: u32) -> windows_core::Result<ID3D11VideoProcessor>
    where
        P0: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVideoProcessor)(windows_core::Interface::as_raw(self), penum.param().abi(), rateconversionindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAuthenticatedChannel(&self, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> windows_core::Result<ID3D11AuthenticatedChannel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAuthenticatedChannel)(windows_core::Interface::as_raw(self), channeltype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCryptoSession(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: Option<*const windows_core::GUID>, pkeyexchangetype: *const windows_core::GUID) -> windows_core::Result<ID3D11CryptoSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCryptoSession)(windows_core::Interface::as_raw(self), pcryptotype, pdecoderprofile.unwrap_or(core::mem::zeroed()) as _, pkeyexchangetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateVideoDecoderOutputView<P0>(&self, presource: P0, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: Option<*mut Option<ID3D11VideoDecoderOutputView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateVideoDecoderOutputView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc, ppvdovview.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateVideoProcessorInputView<P0, P1>(&self, presource: P0, penum: P1, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: Option<*mut Option<ID3D11VideoProcessorInputView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateVideoProcessorInputView)(windows_core::Interface::as_raw(self), presource.param().abi(), penum.param().abi(), pdesc, ppvpiview.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateVideoProcessorOutputView<P0, P1>(&self, presource: P0, penum: P1, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: Option<*mut Option<ID3D11VideoProcessorOutputView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateVideoProcessorOutputView)(windows_core::Interface::as_raw(self), presource.param().abi(), penum.param().abi(), pdesc, ppvpoview.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn CreateVideoProcessorEnumerator(&self, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::Result<ID3D11VideoProcessorEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVideoProcessorEnumerator)(windows_core::Interface::as_raw(self), pdesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetVideoDecoderProfileCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetVideoDecoderProfileCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetVideoDecoderProfile(&self, index: u32) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoDecoderProfile)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CheckVideoDecoderFormat(&self, pdecoderprofile: *const windows_core::GUID, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckVideoDecoderFormat)(windows_core::Interface::as_raw(self), pdecoderprofile, format, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetVideoDecoderConfigCount(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoDecoderConfigCount)(windows_core::Interface::as_raw(self), pdesc, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetVideoDecoderConfig(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoDecoderConfig)(windows_core::Interface::as_raw(self), pdesc, index, pconfig as _) }
    }
    pub unsafe fn GetContentProtectionCaps(&self, pcryptotype: Option<*const windows_core::GUID>, pdecoderprofile: Option<*const windows_core::GUID>, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContentProtectionCaps)(windows_core::Interface::as_raw(self), pcryptotype.unwrap_or(core::mem::zeroed()) as _, pdecoderprofile.unwrap_or(core::mem::zeroed()) as _, pcaps as _) }
    }
    pub unsafe fn CheckCryptoKeyExchange(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: Option<*const windows_core::GUID>, index: u32) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCryptoKeyExchange)(windows_core::Interface::as_raw(self), pcryptotype, pdecoderprofile.unwrap_or(core::mem::zeroed()) as _, index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateVideoDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_DESC, *const D3D11_VIDEO_DECODER_CONFIG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateVideoDecoder: usize,
    pub CreateVideoProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAuthenticatedChannel: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_AUTHENTICATED_CHANNEL_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCryptoSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVideoDecoderOutputView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVideoProcessorInputView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVideoProcessorOutputView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgicommon")]
    pub CreateVideoProcessorEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgicommon"))]
    CreateVideoProcessorEnumerator: usize,
    pub GetVideoDecoderProfileCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetVideoDecoderProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CheckVideoDecoderFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::dxgiformat::DXGI_FORMAT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CheckVideoDecoderFormat: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetVideoDecoderConfigCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_DESC, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetVideoDecoderConfigCount: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetVideoDecoderConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_DESC, u32, *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetVideoDecoderConfig: usize,
    pub GetContentProtectionCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> windows_core::HRESULT,
    pub CheckCryptoKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11VideoDevice_Impl: windows_core::IUnknownImpl {
    fn CreateVideoDecoder(&self, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<ID3D11VideoDecoder>;
    fn CreateVideoProcessor(&self, penum: windows_core::Ref<ID3D11VideoProcessorEnumerator>, rateconversionindex: u32) -> windows_core::Result<ID3D11VideoProcessor>;
    fn CreateAuthenticatedChannel(&self, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> windows_core::Result<ID3D11AuthenticatedChannel>;
    fn CreateCryptoSession(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pkeyexchangetype: *const windows_core::GUID) -> windows_core::Result<ID3D11CryptoSession>;
    fn CreateVideoDecoderOutputView(&self, presource: windows_core::Ref<ID3D11Resource>, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: windows_core::OutRef<ID3D11VideoDecoderOutputView>) -> windows_core::Result<()>;
    fn CreateVideoProcessorInputView(&self, presource: windows_core::Ref<ID3D11Resource>, penum: windows_core::Ref<ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: windows_core::OutRef<ID3D11VideoProcessorInputView>) -> windows_core::Result<()>;
    fn CreateVideoProcessorOutputView(&self, presource: windows_core::Ref<ID3D11Resource>, penum: windows_core::Ref<ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: windows_core::OutRef<ID3D11VideoProcessorOutputView>) -> windows_core::Result<()>;
    fn CreateVideoProcessorEnumerator(&self, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::Result<ID3D11VideoProcessorEnumerator>;
    fn GetVideoDecoderProfileCount(&self) -> u32;
    fn GetVideoDecoderProfile(&self, index: u32) -> windows_core::Result<windows_core::GUID>;
    fn CheckVideoDecoderFormat(&self, pdecoderprofile: *const windows_core::GUID, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<windows_core::BOOL>;
    fn GetVideoDecoderConfigCount(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> windows_core::Result<u32>;
    fn GetVideoDecoderConfig(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<()>;
    fn GetContentProtectionCaps(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> windows_core::Result<()>;
    fn CheckCryptoKeyExchange(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, index: u32) -> windows_core::Result<windows_core::GUID>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D11VideoDevice_Vtbl {
    pub const fn new<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVideoDecoder<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG, ppdecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateVideoDecoder(this, core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&pconfig)) {
                    Ok(ok__) => {
                        ppdecoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVideoProcessor<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penum: *mut core::ffi::c_void, rateconversionindex: u32, ppvideoprocessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateVideoProcessor(this, core::mem::transmute_copy(&penum), core::mem::transmute_copy(&rateconversionindex)) {
                    Ok(ok__) => {
                        ppvideoprocessor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE, ppauthenticatedchannel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateAuthenticatedChannel(this, core::mem::transmute_copy(&channeltype)) {
                    Ok(ok__) => {
                        ppauthenticatedchannel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCryptoSession<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pkeyexchangetype: *const windows_core::GUID, ppcryptosession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateCryptoSession(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&pkeyexchangetype)) {
                    Ok(ok__) => {
                        ppcryptosession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVideoDecoderOutputView<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::CreateVideoDecoderOutputView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppvdovview)).into()
            }
        }
        unsafe extern "system" fn CreateVideoProcessorInputView<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, penum: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::CreateVideoProcessorInputView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&penum), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppvpiview)).into()
            }
        }
        unsafe extern "system" fn CreateVideoProcessorOutputView<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, penum: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::CreateVideoProcessorOutputView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&penum), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppvpoview)).into()
            }
        }
        unsafe extern "system" fn CreateVideoProcessorEnumerator<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateVideoProcessorEnumerator(this, core::mem::transmute_copy(&pdesc)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfileCount<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::GetVideoDecoderProfileCount(this)
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfile<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pdecoderprofile: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::GetVideoDecoderProfile(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pdecoderprofile.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckVideoDecoderFormat<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoderprofile: *const windows_core::GUID, format: super::dxgiformat::DXGI_FORMAT, psupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CheckVideoDecoderFormat(this, core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        psupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfigCount<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::GetVideoDecoderConfigCount(this, core::mem::transmute_copy(&pdesc)) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfig<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::GetVideoDecoderConfig(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&index), core::mem::transmute_copy(&pconfig)).into()
            }
        }
        unsafe extern "system" fn GetContentProtectionCaps<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::GetContentProtectionCaps(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn CheckCryptoKeyExchange<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, index: u32, pkeyexchangetype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CheckCryptoKeyExchange(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pkeyexchangetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateVideoDecoder: CreateVideoDecoder::<Identity, OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Identity, OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Identity, OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Identity, OFFSET>,
            CreateVideoDecoderOutputView: CreateVideoDecoderOutputView::<Identity, OFFSET>,
            CreateVideoProcessorInputView: CreateVideoProcessorInputView::<Identity, OFFSET>,
            CreateVideoProcessorOutputView: CreateVideoProcessorOutputView::<Identity, OFFSET>,
            CreateVideoProcessorEnumerator: CreateVideoProcessorEnumerator::<Identity, OFFSET>,
            GetVideoDecoderProfileCount: GetVideoDecoderProfileCount::<Identity, OFFSET>,
            GetVideoDecoderProfile: GetVideoDecoderProfile::<Identity, OFFSET>,
            CheckVideoDecoderFormat: CheckVideoDecoderFormat::<Identity, OFFSET>,
            GetVideoDecoderConfigCount: GetVideoDecoderConfigCount::<Identity, OFFSET>,
            GetVideoDecoderConfig: GetVideoDecoderConfig::<Identity, OFFSET>,
            GetContentProtectionCaps: GetContentProtectionCaps::<Identity, OFFSET>,
            CheckCryptoKeyExchange: CheckCryptoKeyExchange::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11VideoDevice {}
windows_core::imp::define_interface!(ID3D11VideoProcessor, ID3D11VideoProcessor_Vtbl, 0x1d7b0652_185f_41c6_85ce_0c5be3d4ae6c);
impl core::ops::Deref for ID3D11VideoProcessor {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessor, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VideoProcessor {
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn GetContentDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetContentDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
    pub unsafe fn GetRateConversionCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
        unsafe {
            (windows_core::Interface::vtable(self).GetRateConversionCaps)(windows_core::Interface::as_raw(self), pcaps as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoProcessor_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    #[cfg(feature = "Win32_dxgicommon")]
    pub GetContentDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    GetContentDesc: usize,
    pub GetRateConversionCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS),
}
#[cfg(feature = "Win32_dxgicommon")]
pub trait ID3D11VideoProcessor_Impl: ID3D11DeviceChild_Impl {
    fn GetContentDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC);
    fn GetRateConversionCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS);
}
#[cfg(feature = "Win32_dxgicommon")]
impl ID3D11VideoProcessor_Vtbl {
    pub const fn new<Identity: ID3D11VideoProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContentDesc<Identity: ID3D11VideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessor_Impl::GetContentDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        unsafe extern "system" fn GetRateConversionCaps<Identity: ID3D11VideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessor_Impl::GetRateConversionCaps(this, core::mem::transmute_copy(&pcaps));
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetContentDesc: GetContentDesc::<Identity, OFFSET>,
            GetRateConversionCaps: GetRateConversionCaps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessor as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgicommon")]
impl windows_core::RuntimeName for ID3D11VideoProcessor {}
windows_core::imp::define_interface!(ID3D11VideoProcessorEnumerator, ID3D11VideoProcessorEnumerator_Vtbl, 0x31627037_53ab_4200_9061_05faa9ab45f9);
impl core::ops::Deref for ID3D11VideoProcessorEnumerator {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorEnumerator, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VideoProcessorEnumerator {
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn GetVideoProcessorContentDesc(&self, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorContentDesc)(windows_core::Interface::as_raw(self), pcontentdesc as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CheckVideoProcessorFormat(&self, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckVideoProcessorFormat)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVideoProcessorCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorCaps)(windows_core::Interface::as_raw(self), pcaps as _) }
    }
    pub unsafe fn GetVideoProcessorRateConversionCaps(&self, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorRateConversionCaps)(windows_core::Interface::as_raw(self), typeindex, pcaps as _) }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn GetVideoProcessorCustomRate(&self, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorCustomRate)(windows_core::Interface::as_raw(self), typeindex, customrateindex, prate as _) }
    }
    pub unsafe fn GetVideoProcessorFilterRange(&self, filter: D3D11_VIDEO_PROCESSOR_FILTER) -> windows_core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoProcessorFilterRange)(windows_core::Interface::as_raw(self), filter, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoProcessorEnumerator_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    #[cfg(feature = "Win32_dxgicommon")]
    pub GetVideoProcessorContentDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgicommon"))]
    GetVideoProcessorContentDesc: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CheckVideoProcessorFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CheckVideoProcessorFormat: usize,
    pub GetVideoProcessorCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CAPS) -> windows_core::HRESULT,
    pub GetVideoProcessorRateConversionCaps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgicommon")]
    pub GetVideoProcessorCustomRate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgicommon"))]
    GetVideoProcessorCustomRate: usize,
    pub GetVideoProcessorFilterRange: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_VIDEO_PROCESSOR_FILTER, *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11VideoProcessorEnumerator_Impl: ID3D11DeviceChild_Impl {
    fn GetVideoProcessorContentDesc(&self, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::Result<()>;
    fn CheckVideoProcessorFormat(&self, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<u32>;
    fn GetVideoProcessorCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> windows_core::Result<()>;
    fn GetVideoProcessorRateConversionCaps(&self, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> windows_core::Result<()>;
    fn GetVideoProcessorCustomRate(&self, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> windows_core::Result<()>;
    fn GetVideoProcessorFilterRange(&self, filter: D3D11_VIDEO_PROCESSOR_FILTER) -> windows_core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D11VideoProcessorEnumerator_Vtbl {
    pub const fn new<Identity: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVideoProcessorContentDesc<Identity: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorContentDesc(this, core::mem::transmute_copy(&pcontentdesc)).into()
            }
        }
        unsafe extern "system" fn CheckVideoProcessorFormat<Identity: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoProcessorEnumerator_Impl::CheckVideoProcessorFormat(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Identity: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorRateConversionCaps<Identity: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorRateConversionCaps(this, core::mem::transmute_copy(&typeindex), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorCustomRate<Identity: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorCustomRate(this, core::mem::transmute_copy(&typeindex), core::mem::transmute_copy(&customrateindex), core::mem::transmute_copy(&prate)).into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<Identity: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: D3D11_VIDEO_PROCESSOR_FILTER, prange: *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorFilterRange(this, core::mem::transmute_copy(&filter)) {
                    Ok(ok__) => {
                        prange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetVideoProcessorContentDesc: GetVideoProcessorContentDesc::<Identity, OFFSET>,
            CheckVideoProcessorFormat: CheckVideoProcessorFormat::<Identity, OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Identity, OFFSET>,
            GetVideoProcessorRateConversionCaps: GetVideoProcessorRateConversionCaps::<Identity, OFFSET>,
            GetVideoProcessorCustomRate: GetVideoProcessorCustomRate::<Identity, OFFSET>,
            GetVideoProcessorFilterRange: GetVideoProcessorFilterRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11VideoProcessorEnumerator {}
windows_core::imp::define_interface!(ID3D11VideoProcessorInputView, ID3D11VideoProcessorInputView_Vtbl, 0x11ec5a5f_51dc_4945_ab34_6e8c21300ea5);
impl core::ops::Deref for ID3D11VideoProcessorInputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorInputView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11VideoProcessorInputView {
    pub unsafe fn GetDesc(&self) -> D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoProcessorInputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC),
}
pub trait ID3D11VideoProcessorInputView_Impl: ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC);
}
impl ID3D11VideoProcessorInputView_Vtbl {
    pub const fn new<Identity: ID3D11VideoProcessorInputView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11VideoProcessorInputView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorInputView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorInputView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VideoProcessorInputView {}
windows_core::imp::define_interface!(ID3D11VideoProcessorOutputView, ID3D11VideoProcessorOutputView_Vtbl, 0xa048285e_25a9_4527_bd93_d68b68c44254);
impl core::ops::Deref for ID3D11VideoProcessorOutputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorOutputView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11VideoProcessorOutputView {
    pub unsafe fn GetDesc(&self) -> D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoProcessorOutputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC),
}
pub trait ID3D11VideoProcessorOutputView_Impl: ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC);
}
impl ID3D11VideoProcessorOutputView_Vtbl {
    pub const fn new<Identity: ID3D11VideoProcessorOutputView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11VideoProcessorOutputView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorOutputView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorOutputView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VideoProcessorOutputView {}
windows_core::imp::define_interface!(ID3D11View, ID3D11View_Vtbl, 0x839d1216_bb2e_412b_b7f4_a9dbebe08ed1);
impl core::ops::Deref for ID3D11View {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11View, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11View {
    pub unsafe fn GetResource(&self) -> windows_core::Result<ID3D11Resource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11View_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait ID3D11View_Impl: ID3D11DeviceChild_Impl {
    fn GetResource(&self, ppresource: windows_core::OutRef<ID3D11Resource>);
}
impl ID3D11View_Vtbl {
    pub const fn new<Identity: ID3D11View_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource<Identity: ID3D11View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11View_Impl::GetResource(this, core::mem::transmute_copy(&ppresource));
            }
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetResource: GetResource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11View as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11View {}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
pub type PFN_D3D11_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: windows_core::Ref<super::dxgi::IDXGIAdapter>, param1: super::d3dcommon::D3D_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: windows_core::OutRef<ID3D11Device>, param8: *mut super::d3dcommon::D3D_FEATURE_LEVEL, param9: windows_core::OutRef<ID3D11DeviceContext>) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type PFN_D3D11_CREATE_DEVICE_AND_SWAP_CHAIN = Option<unsafe extern "system" fn(param0: windows_core::Ref<super::dxgi::IDXGIAdapter>, param1: super::d3dcommon::D3D_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *const super::dxgi::DXGI_SWAP_CHAIN_DESC, param8: windows_core::OutRef<super::dxgi::IDXGISwapChain>, param9: windows_core::OutRef<ID3D11Device>, param10: *mut super::d3dcommon::D3D_FEATURE_LEVEL, param11: windows_core::OutRef<ID3D11DeviceContext>) -> windows_core::HRESULT>;
