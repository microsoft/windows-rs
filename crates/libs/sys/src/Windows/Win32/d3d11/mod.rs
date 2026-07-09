#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
windows_link::link!("d3d11.dll" "system" fn D3D11CreateDevice(padapter : *mut core::ffi::c_void, drivertype : super::d3dcommon::D3D_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, pfeaturelevels : *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut super::d3dcommon::D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("d3d11.dll" "system" fn D3D11CreateDeviceAndSwapChain(padapter : *mut core::ffi::c_void, drivertype : super::d3dcommon::D3D_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, pfeaturelevels : *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, pswapchaindesc : *const super::dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut super::d3dcommon::D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type APP_DEPRECATED_HRESULT = windows_sys::core::HRESULT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_BLEND_DESC {
    pub Base: D3D11_BLEND_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_BOX {
    pub Base: D3D11_BOX,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_BUFFER_DESC {
    pub Base: D3D11_BUFFER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_COUNTER_DESC {
    pub Base: D3D11_COUNTER_DESC,
}
#[repr(C, align(1))]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_DEFAULT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct CD3D11_QUERY_DESC {
    pub Base: D3D11_QUERY_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_RASTERIZER_DESC {
    pub Base: D3D11_RASTERIZER_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct CD3D11_TEXTURE1D_DESC {
    pub Base: D3D11_TEXTURE1D_DESC,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_TEXTURE2D_DESC {
    pub Base: D3D11_TEXTURE2D_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct CD3D11_VIDEO_DEFAULT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub EncryptionGuid: windows_sys::core::GUID,
}
pub const D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6346cc54_2cfc_4ad4_8224_d15837de7700);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub DecoderHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41fff286_6ae0_4d43_9d55_a46e9efd158a);
pub const D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06114bdb_3523_470a_8dca_fbc2845154f0);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub StartSequenceQuery: u32,
    pub StartSequenceConfigure: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_PROTECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50455658_3f47_4362_bf99_bfdfcde9ed29);
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
pub const D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0772d047_1b40_48e8_9ca6_b5f510de9f01);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub ProcessType: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::winnt::HANDLE,
    pub AllowAccess: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_PROTECTION_FLAGS_0 {
    pub _bitfield: u32,
}
pub const D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6214d9d2_432c_4abb_9fce_216eea269e3b);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub EncryptionGuidIndex: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub BusType: D3D11_BUS_TYPE,
    pub AccessibleInContiguousBlocks: windows_sys::core::BOOL,
    pub AccessibleInNonContiguousBlocks: windows_sys::core::BOOL,
}
pub const D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc1b18a5_b1fb_42ab_bd94_b5828b4bf7be);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ChannelType: D3D11_AUTHENTICATED_CHANNEL_TYPE,
}
pub const D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2634499e_d018_4d74_ac17_7f724059528d);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DecoderHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DecoderHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuid: windows_sys::core::GUID,
}
pub const D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec1791c7_dad3_4f15_9ec3_faa93d60d4f0);
pub const D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec1c539d_8cff_4e2a_bcc4_f5692f99f480);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf83a5958_e986_4bda_beb0_411f6a7a01b7);
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb30f7066_203c_4b07_93fc_ceaafd61241e);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_INPUT {
    pub QueryType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT {
    pub omac: D3D11_OMAC,
    pub QueryType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x839ddca3_9b4e_41e4_b053_892bd2a11ee7);
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2c042b5e_8c07_46d5_aabe_8f75cbad4c31);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDCount: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_PROTECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa84eb584_c495_48aa_b94d_8bd2d6fbce05);
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
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x649bbadb_f0f4_4639_a15b_24393fc3abac);
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0db207b3_9450_46a6_82de_1b96d44f9cf2);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub RestrictedSharedResourceProcessCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub ProcessIndex: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProcessIndex: u32,
    pub ProcessIdentifier: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x012f0bd6_e662_4474_befd_aa53e5143c6d);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct D3D11_BLEND_DESC {
    pub AlphaToCoverageEnable: windows_sys::core::BOOL,
    pub IndependentBlendEnable: windows_sys::core::BOOL,
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_BUFFEREX_SRV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
pub type D3D11_BUFFEREX_SRV_FLAG = i32;
pub const D3D11_BUFFEREX_SRV_FLAG_RAW: D3D11_BUFFEREX_SRV_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_CLASS_INSTANCE_DESC {
    pub InstanceId: u32,
    pub InstanceIndex: u32,
    pub TypeId: u32,
    pub ConstantBuffer: u32,
    pub BaseConstantBufferOffset: u32,
    pub BaseTexture: u32,
    pub BaseSampler: u32,
    pub Created: windows_sys::core::BOOL,
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_COUNTER_DESC {
    pub Counter: D3D11_COUNTER,
    pub MiscFlags: u32,
}
pub const D3D11_COUNTER_DEVICE_DEPENDENT_0: D3D11_COUNTER = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const D3D11_CRYPTO_TYPE_AES128_CTR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9b6bd711_4f74_41c9_9e7b_0be2d7d93b4f);
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
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CBCS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x422d9319_9d21_4bb7_9371_faf5a82c3e04);
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CENC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb0405235_c13d_44f2_9ae5_dd48e08e5b67);
pub const D3D11_DECODER_ENCRYPTION_HW_CENC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x89d6ac4f_09f2_4229_b2cd_37740a6dfd81);
pub const D3D11_DECODER_PROFILE_APV_VLD_400_10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x37148862_6bd6_4618_8293_777b686b0824);
pub const D3D11_DECODER_PROFILE_APV_VLD_422_10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x226a709d_ae12_44c5_ba21_164feeb7f9b6);
pub const D3D11_DECODER_PROFILE_APV_VLD_422_12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf6f152ad_94e5_4bfa_9227_676cddeff42b);
pub const D3D11_DECODER_PROFILE_APV_VLD_4444_10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc83799b9_9655_4b95_8008_56a322ce5d81);
pub const D3D11_DECODER_PROFILE_APV_VLD_4444_12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a763ee3_4d05_47fe_a429_723474b69d7c);
pub const D3D11_DECODER_PROFILE_APV_VLD_444_10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a4a8d7d_7610_469f_855f_39f13051c013);
pub const D3D11_DECODER_PROFILE_APV_VLD_444_12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf1039a1c_e208_45c1_952c_040841b67667);
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x17127009_a00f_4ce1_994e_bf4081f6f3f0);
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d80bed6_9cac_4835_9e91_327bbc4f9ee8);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb8be4ccb_cf53_46ba_8d59_d6b8a6da5d2a);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6936ff0f_45b1_4163_9cc1_646ef6946108);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0c5f2aa1_e541_4089_bb7b_98110a19d7c8);
pub const D3D11_DECODER_PROFILE_H264_IDCT_FGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be67_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_IDCT_NOFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be66_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_FGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be65_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_NOFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be64_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_FGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be69_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_MULTIVIEW_NOFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const D3D11_DECODER_PROFILE_H264_VLD_NOFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_NOFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_PROGRESSIVE_NOFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const D3D11_DECODER_PROFILE_H264_VLD_WITHFMOASO_NOFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd5f04ff9_3418_45d8_9561_32a76aae2ddd);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0bac4fe5_1532_4429_a854_f84de04953db);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0dabeffa_4458_4602_bc03_0795659d617c);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10_EXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9cc55490_e37c_4932_8684_4920f9f6409c);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1a72925f_0c2c_4f15_96fb_b17d1473603f);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN12_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55bcac81_f311_4093_a7d0_1cbc0b849bee);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN12_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9798634d_fe9d_48e5_b4da_dbec45b3df01);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN16: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4fbdbb0_a113_482b_a232_635cc0697f6d);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4008018f_f537_4b36_98cf_61af8a2c1a33);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MONOCHROME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0685b993_3d8c_43a0_8b28_d74c2d6899a4);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MONOCHROME10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x142a1d0f_69dd_4ec9_8591_b12ffcb91a29);
pub const D3D11_DECODER_PROFILE_JPEG_VLD_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf782c83_bef5_4a2c_87cb_6019e7b175ac);
pub const D3D11_DECODER_PROFILE_JPEG_VLD_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf04df417_eee2_4067_a778_f35c15ab9721);
pub const D3D11_DECODER_PROFILE_JPEG_VLD_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4cd00e17_89ba_48ef_b9f9_edcb82713f65);
pub const D3D11_DECODER_PROFILE_MJPEG_VLD_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x725cb506_0c29_43c4_9440_8e9397903a04);
pub const D3D11_DECODER_PROFILE_MJPEG_VLD_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b77b9cd_1a35_4c30_9fd8_ef4b60c035dd);
pub const D3D11_DECODER_PROFILE_MJPEG_VLD_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd95161f9_0d44_47e6_bcf5_1bfbfb268f97);
pub const D3D11_DECODER_PROFILE_MJPEG_VLD_4444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc91748d5_fd18_4aca_9db3_3a6634ab547d);
pub const D3D11_DECODER_PROFILE_MPEG1_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f3ec719_3735_42cc_8063_65cc3cb36616);
pub const D3D11_DECODER_PROFILE_MPEG2_IDCT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbf22ad00_03ea_4690_8077_473346209b7e);
pub const D3D11_DECODER_PROFILE_MPEG2_MOCOMP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe6a9f44b_61b0_4563_9ea4_63d2a3c6fe66);
pub const D3D11_DECODER_PROFILE_MPEG2_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const D3D11_DECODER_PROFILE_MPEG2and1_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_GMC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab998b5b_4258_44a9_9feb_94e597a6baae);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_NOGMC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_SIMPLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const D3D11_DECODER_PROFILE_VC1_D2010: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_IDCT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea2_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_MOCOMP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea1_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_POSTPROC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea0_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VP8_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const D3D11_DECODER_PROFILE_VP9_VLD_10BIT_PROFILE2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
pub const D3D11_DECODER_PROFILE_VP9_VLD_PROFILE0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const D3D11_DECODER_PROFILE_WMV8_MOCOMP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be81_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV8_POSTPROC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be80_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_IDCT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be94_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_MOCOMP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be91_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_POSTPROC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be90_a0c7_11d3_b984_00c04f2e73c5);
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D11_STENCIL_OP,
    pub StencilDepthFailOp: D3D11_STENCIL_OP,
    pub StencilPassOp: D3D11_STENCIL_OP,
    pub StencilFunc: D3D11_COMPARISON_FUNC,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: windows_sys::core::BOOL,
    pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D11_COMPARISON_FUNC,
    pub StencilEnable: windows_sys::core::BOOL,
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    pub IndexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartIndexLocation: u32,
    pub BaseVertexLocation: i32,
    pub StartInstanceLocation: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    pub TileBasedDeferredRenderer: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    pub ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS {
    pub OutputMergerLogicOp: windows_sys::core::BOOL,
    pub UAVOnlyRenderingForcedSampleCount: windows_sys::core::BOOL,
    pub DiscardAPIsSeenByDriver: windows_sys::core::BOOL,
    pub FlagsForUpdateAndCopySeenByDriver: windows_sys::core::BOOL,
    pub ClearView: windows_sys::core::BOOL,
    pub CopyWithOverlap: windows_sys::core::BOOL,
    pub ConstantBufferPartialUpdate: windows_sys::core::BOOL,
    pub ConstantBufferOffsetting: windows_sys::core::BOOL,
    pub MapNoOverwriteOnDynamicConstantBuffer: windows_sys::core::BOOL,
    pub MapNoOverwriteOnDynamicBufferSRV: windows_sys::core::BOOL,
    pub MultisampleRTVWithForcedSampleCountOne: windows_sys::core::BOOL,
    pub SAD4ShaderInstructions: windows_sys::core::BOOL,
    pub ExtendedDoublesShaderInstructions: windows_sys::core::BOOL,
    pub ExtendedResourceSharing: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
    pub MinMaxFiltering: windows_sys::core::BOOL,
    pub ClearViewAlsoSupportsDepthOnlyFormats: windows_sys::core::BOOL,
    pub MapOnDefaultBuffers: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    pub PSSpecifiedStencilRefSupported: windows_sys::core::BOOL,
    pub TypedUAVLoadAdditionalFormats: windows_sys::core::BOOL,
    pub ROVsSupported: windows_sys::core::BOOL,
    pub ConservativeRasterizationTier: D3D11_CONSERVATIVE_RASTERIZATION_TIER,
    pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
    pub MapOnDefaultTextures: windows_sys::core::BOOL,
    pub StandardSwizzle: windows_sys::core::BOOL,
    pub UnifiedMemoryArchitecture: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    pub VPAndRTArrayIndexFromAnyShaderFeedingRasterizer: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    pub SharedResourceTier: D3D11_SHARED_RESOURCE_TIER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS6 {
    pub ShaderAccessRestrictedResourceTier: D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS {
    pub FullNonPow2TextureSupport: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    pub FullNonPow2TextureSupported: windows_sys::core::BOOL,
    pub DepthAsTextureWithLessEqualComparisonFilterSupported: windows_sys::core::BOOL,
    pub SimpleInstancingSupported: windows_sys::core::BOOL,
    pub TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    pub SupportsDepthAsTextureWithLessEqualComparisonFilter: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    pub SimpleInstancingSupported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_DISPLAYABLE {
    pub DisplayableTexture: windows_sys::core::BOOL,
    pub SharedResourceTier: D3D11_SHARED_RESOURCE_TIER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_DOUBLES {
    pub DoublePrecisionFloatShaderOps: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    pub InFormat: super::dxgiformat::DXGI_FORMAT,
    pub OutFormatSupport: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    pub InFormat: super::dxgiformat::DXGI_FORMAT,
    pub OutFormatSupport2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub MaxGPUVirtualAddressBitsPerProcess: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_MARKER_SUPPORT {
    pub Profile: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_SHADER_CACHE {
    pub SupportFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    pub PixelShaderMinPrecision: u32,
    pub AllOtherShaderStagesMinPrecision: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_THREADING {
    pub DriverConcurrentCreates: windows_sys::core::BOOL,
    pub DriverCommandLists: windows_sys::core::BOOL,
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
#[derive(Clone, Copy)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D11_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_INPUT_PER_INSTANCE_DATA: D3D11_INPUT_CLASSIFICATION = 1;
pub const D3D11_INPUT_PER_VERTEX_DATA: D3D11_INPUT_CLASSIFICATION = 0;
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295;
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295;
pub const D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295;
pub const D3D11_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295;
pub const D3D11_KEY_EXCHANGE_HW_PROTECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb1170d8a_628d_4da3_ad3b_82ddb08b4970);
pub const D3D11_KEY_EXCHANGE_RSAES_OAEP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc1949895_d72a_4a1d_8e5d_ed857d171520);
pub const D3D11_MAG_FILTER_SHIFT: u32 = 2;
pub const D3D11_MAJOR_VERSION: u32 = 11;
pub type D3D11_MAP = i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
pub type D3D11_PRIMITIVE = super::d3dcommon::D3D_PRIMITIVE;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D11_PRIMITIVE_TOPOLOGY = super::d3dcommon::D3D_PRIMITIVE_TOPOLOGY;
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: windows_sys::core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_sys::core::BOOL,
    pub ScissorEnable: windows_sys::core::BOOL,
    pub MultisampleEnable: windows_sys::core::BOOL,
    pub AntialiasedLineEnable: windows_sys::core::BOOL,
}
pub const D3D11_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16;
#[cfg(feature = "Win32_windef")]
pub type D3D11_RECT = super::windef::RECT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: windows_sys::core::BOOL,
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct D3D11_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
impl Default for D3D11_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_SO_NO_RASTERIZED_STREAM: u32 = 4294967295;
pub const D3D11_SO_OUTPUT_COMPONENT_COUNT: u32 = 128;
pub const D3D11_SO_STREAM_COUNT: u32 = 4;
pub const D3D11_SPEC_DATE_DAY: u32 = 16;
pub const D3D11_SPEC_DATE_MONTH: u32 = 5;
pub const D3D11_SPEC_DATE_YEAR: u32 = 2011;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D11_SRV_DIMENSION = super::d3dcommon::D3D_SRV_DIMENSION;
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX1D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX1D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX1D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_ARRAY_VPOV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_VDOV {
    pub ArraySlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_VPIV {
    pub MipSlice: u32,
    pub ArraySlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_VPOV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
pub const D3D11_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_COLOR_RGBA {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_COLOR_YCbCrA {
    pub Y: f32,
    pub Cb: f32,
    pub Cr: f32,
    pub A: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    pub Caps: u32,
    pub KeyExchangeTypeCount: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
pub const D3D11_VIDEO_DECODER_BUFFER_BITSTREAM: D3D11_VIDEO_DECODER_BUFFER_TYPE = 6;
pub const D3D11_VIDEO_DECODER_BUFFER_DEBLOCKING_CONTROL: D3D11_VIDEO_DECODER_BUFFER_TYPE = 3;
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub PartialEncryption: windows_sys::core::BOOL,
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_DECODER_CONFIG {
    pub guidConfigBitstreamEncryption: windows_sys::core::GUID,
    pub guidConfigMBcontrolEncryption: windows_sys::core::GUID,
    pub guidConfigResidDiffEncryption: windows_sys::core::GUID,
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_DECODER_DESC {
    pub Guid: windows_sys::core::GUID,
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub OutputFormat: super::dxgiformat::DXGI_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_EXTENSION {
    pub Function: u32,
    pub pPrivateInputData: *mut core::ffi::c_void,
    pub PrivateInputDataSize: u32,
    pub pPrivateOutputData: *mut core::ffi::c_void,
    pub PrivateOutputDataSize: u32,
    pub ResourceCount: u32,
    pub ppResourceList: *mut *mut core::ffi::c_void,
}
impl Default for D3D11_VIDEO_DECODER_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    pub DecodeProfile: windows_sys::core::GUID,
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgicommon")]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    pub CustomRate: super::dxgicommon::DXGI_RATIONAL,
    pub OutputFrames: u32,
    pub InputInterlaced: windows_sys::core::BOOL,
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_PROCESSOR_STREAM {
    pub Enable: windows_sys::core::BOOL,
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ppPastSurfaces: *mut *mut core::ffi::c_void,
    pub pInputSurface: *mut core::ffi::c_void,
    pub ppFutureSurfaces: *mut *mut core::ffi::c_void,
    pub ppPastSurfacesRight: *mut *mut core::ffi::c_void,
    pub pInputSurfaceRight: *mut core::ffi::c_void,
    pub ppFutureSurfacesRight: *mut *mut core::ffi::c_void,
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
#[derive(Clone, Copy, Default)]
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
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
pub type PFN_D3D11_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::d3dcommon::D3D_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *mut *mut core::ffi::c_void, param8: *mut super::d3dcommon::D3D_FEATURE_LEVEL, param9: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type PFN_D3D11_CREATE_DEVICE_AND_SWAP_CHAIN = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::d3dcommon::D3D_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *const super::dxgi::DXGI_SWAP_CHAIN_DESC, param8: *mut *mut core::ffi::c_void, param9: *mut *mut core::ffi::c_void, param10: *mut super::d3dcommon::D3D_FEATURE_LEVEL, param11: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
