#[cfg(all(feature = "d3dcommon", feature = "dxgi", feature = "minwindef"))]
windows_link::link!("d3d11.dll" "system" fn D3D11CreateDevice(padapter : *mut core::ffi::c_void, drivertype : super::d3dcommon::D3D_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, pfeaturelevels : *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut super::d3dcommon::D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "dxgi", feature = "minwindef", feature = "windef"))]
windows_link::link!("d3d11.dll" "system" fn D3D11CreateDeviceAndSwapChain(padapter : *mut core::ffi::c_void, drivertype : super::d3dcommon::D3D_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, pfeaturelevels : *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, pswapchaindesc : *const super::dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut super::d3dcommon::D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d11.dll" "system" fn D3D11On12CreateDevice(pdevice : *mut core::ffi::c_void, flags : u32, pfeaturelevels : *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels : u32, ppcommandqueues : *const *mut core::ffi::c_void, numqueues : u32, nodemask : u32, ppdevice : *mut *mut core::ffi::c_void, ppimmediatecontext : *mut *mut core::ffi::c_void, pchosenfeaturelevel : *mut super::d3dcommon::D3D_FEATURE_LEVEL) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble11Trace(psrcdata : *const core::ffi::c_void, srcdatasize : usize, ptrace : *mut core::ffi::c_void, startstep : u32, numsteps : u32, flags : u32, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type APP_DEPRECATED_HRESULT = windows_sys::core::HRESULT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_BLEND_DESC {
    pub Base: D3D11_BLEND_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_BLEND_DESC1 {
    pub Base: D3D11_BLEND_DESC1,
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
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct CD3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Base: D3D11_DEPTH_STENCIL_VIEW_DESC,
}
#[cfg(feature = "dxgi")]
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
pub struct CD3D11_QUERY_DESC1 {
    pub Base: D3D11_QUERY_DESC1,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_RASTERIZER_DESC {
    pub Base: D3D11_RASTERIZER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_RASTERIZER_DESC1 {
    pub Base: D3D11_RASTERIZER_DESC1,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_RASTERIZER_DESC2 {
    pub Base: D3D11_RASTERIZER_DESC2,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_RECT {
    pub Base: D3D11_RECT,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct CD3D11_RENDER_TARGET_VIEW_DESC {
    pub Base: D3D11_RENDER_TARGET_VIEW_DESC,
}
#[cfg(feature = "dxgi")]
impl Default for CD3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct CD3D11_RENDER_TARGET_VIEW_DESC1 {
    pub Base: D3D11_RENDER_TARGET_VIEW_DESC1,
}
#[cfg(feature = "dxgi")]
impl Default for CD3D11_RENDER_TARGET_VIEW_DESC1 {
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
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
pub struct CD3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Base: D3D11_SHADER_RESOURCE_VIEW_DESC,
}
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for CD3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
pub struct CD3D11_SHADER_RESOURCE_VIEW_DESC1 {
    pub Base: D3D11_SHADER_RESOURCE_VIEW_DESC1,
}
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for CD3D11_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_TEXTURE1D_DESC {
    pub Base: D3D11_TEXTURE1D_DESC,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_TEXTURE2D_DESC {
    pub Base: D3D11_TEXTURE2D_DESC,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_TEXTURE2D_DESC1 {
    pub Base: D3D11_TEXTURE2D_DESC1,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_TEXTURE3D_DESC {
    pub Base: D3D11_TEXTURE3D_DESC,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_TEXTURE3D_DESC1 {
    pub Base: D3D11_TEXTURE3D_DESC1,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct CD3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Base: D3D11_UNORDERED_ACCESS_VIEW_DESC,
}
#[cfg(feature = "dxgi")]
impl Default for CD3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct CD3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    pub Base: D3D11_UNORDERED_ACCESS_VIEW_DESC1,
}
#[cfg(feature = "dxgi")]
impl Default for CD3D11_UNORDERED_ACCESS_VIEW_DESC1 {
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
pub type D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG = i32;
pub const D3D11_1_CREATE_DEVICE_CONTEXT_STATE_SINGLETHREADED: D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG = 1;
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub EncryptionGuid: windows_sys::core::GUID,
}
pub const D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6346cc54_2cfc_4ad4_8224_d15837de7700);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub DecoderHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41fff286_6ae0_4d43_9d55_a46e9efd158a);
pub const D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06114bdb_3523_470a_8dca_fbc2845154f0);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub StartSequenceQuery: u32,
    pub StartSequenceConfigure: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_sys::core::HRESULT,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_PROTECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50455658_3f47_4362_bf99_bfdfcde9ed29);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub Protections: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0772d047_1b40_48e8_9ca6_b5f510de9f01);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub ProcessType: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::winnt::HANDLE,
    pub AllowAccess: windows_sys::core::BOOL,
}
#[cfg(feature = "winnt")]
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidCount: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub EncryptionGuidIndex: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub BusType: D3D11_BUS_TYPE,
    pub AccessibleInContiguousBlocks: windows_sys::core::BOOL,
    pub AccessibleInNonContiguousBlocks: windows_sys::core::BOOL,
}
pub const D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc1b18a5_b1fb_42ab_bd94_b5828b4bf7be);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ChannelType: D3D11_AUTHENTICATED_CHANNEL_TYPE,
}
pub const D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2634499e_d018_4d74_ac17_7f724059528d);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DecoderHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DecoderHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuid: windows_sys::core::GUID,
}
pub const D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec1791c7_dad3_4f15_9ec3_faa93d60d4f0);
pub const D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec1c539d_8cff_4e2a_bcc4_f5692f99f480);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf83a5958_e986_4bda_beb0_411f6a7a01b7);
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb30f7066_203c_4b07_93fc_ceaafd61241e);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_INPUT {
    pub QueryType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT {
    pub omac: D3D11_OMAC,
    pub QueryType: windows_sys::core::GUID,
    pub hChannel: super::winnt::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_sys::core::HRESULT,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x839ddca3_9b4e_41e4_b053_892bd2a11ee7);
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2c042b5e_8c07_46d5_aabe_8f75cbad4c31);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDCount: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::winnt::HANDLE,
    pub CryptoSessionHandle: super::winnt::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_PROTECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa84eb584_c495_48aa_b94d_8bd2d6fbce05);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProtectionFlags: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x649bbadb_f0f4_4639_a15b_24393fc3abac);
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0db207b3_9450_46a6_82de_1b96d44f9cf2);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub RestrictedSharedResourceProcessCount: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub ProcessIndex: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProcessIndex: u32,
    pub ProcessIdentifier: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x012f0bd6_e662_4474_befd_aa53e5143c6d);
#[repr(C)]
#[cfg(feature = "winnt")]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BLEND_DESC1 {
    pub AlphaToCoverageEnable: windows_sys::core::BOOL,
    pub IndependentBlendEnable: windows_sys::core::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1; 8],
}
impl Default for D3D11_BLEND_DESC1 {
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
#[cfg(feature = "d3dcommon")]
pub type D3D11_CBUFFER_TYPE = super::d3dcommon::D3D_CBUFFER_TYPE;
pub const D3D11_CENTER_MULTISAMPLE_PATTERN: D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -2;
pub type D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG = i32;
pub const D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_TILED_RESOURCE: D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG = 1;
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
pub const D3D11_COMPUTE_SHADER: D3D11_SHADER_TYPE = 6;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_COMPUTE_SHADER_TRACE_DESC {
    pub Invocation: u64,
    pub ThreadIDInGroup: [u32; 3],
    pub ThreadGroupID: [u32; 3],
}
impl Default for D3D11_COMPUTE_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_CONSERVATIVE_RASTERIZATION_MODE = i32;
pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_OFF: D3D11_CONSERVATIVE_RASTERIZATION_MODE = 0;
pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_ON: D3D11_CONSERVATIVE_RASTERIZATION_MODE = 1;
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
pub type D3D11_CONTEXT_TYPE = i32;
pub const D3D11_CONTEXT_TYPE_3D: D3D11_CONTEXT_TYPE = 1;
pub const D3D11_CONTEXT_TYPE_ALL: D3D11_CONTEXT_TYPE = 0;
pub const D3D11_CONTEXT_TYPE_COMPUTE: D3D11_CONTEXT_TYPE = 2;
pub const D3D11_CONTEXT_TYPE_COPY: D3D11_CONTEXT_TYPE = 3;
pub const D3D11_CONTEXT_TYPE_VIDEO: D3D11_CONTEXT_TYPE = 4;
pub const D3D11_COPY_DISCARD: D3D11_COPY_FLAGS = 2;
pub type D3D11_COPY_FLAGS = i32;
pub const D3D11_COPY_NO_OVERWRITE: D3D11_COPY_FLAGS = 1;
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
pub type D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS = u32;
pub const D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAG_NONE: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS = 0;
pub type D3D11_CRYPTO_SESSION_STATUS = i32;
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_AND_CONTENT_LOST: D3D11_CRYPTO_SESSION_STATUS = 2;
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_LOST: D3D11_CRYPTO_SESSION_STATUS = 1;
pub const D3D11_CRYPTO_SESSION_STATUS_OK: D3D11_CRYPTO_SESSION_STATUS = 0;
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
pub const D3D11_DEBUG_FEATURE_ALWAYS_DISCARD_OFFERED_RESOURCE: u32 = 8;
pub const D3D11_DEBUG_FEATURE_AVOID_BEHAVIOR_CHANGING_DEBUG_AIDS: u32 = 64;
pub const D3D11_DEBUG_FEATURE_DISABLE_TILED_RESOURCE_MAPPING_TRACKING_AND_VALIDATION: u32 = 128;
pub const D3D11_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2;
pub const D3D11_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1;
pub const D3D11_DEBUG_FEATURE_NEVER_DISCARD_OFFERED_RESOURCE: u32 = 16;
pub const D3D11_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4;
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
pub const D3D11_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1.0;
pub const D3D11_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1.0;
pub const D3D11_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1.0;
pub const D3D11_DEFAULT_BLEND_FACTOR_RED: f32 = 1.0;
pub const D3D11_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0.0;
pub const D3D11_DEFAULT_DEPTH_BIAS: u32 = 0;
pub const D3D11_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0.0;
pub const D3D11_DEFAULT_MAX_ANISOTROPY: u32 = 16;
pub const D3D11_DEFAULT_MIP_LOD_BIAS: f32 = 0.0;
pub const D3D11_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0;
pub const D3D11_DEFAULT_SAMPLE_MASK: u32 = 4294967295;
pub const D3D11_DEFAULT_SCISSOR_ENDX: u32 = 0;
pub const D3D11_DEFAULT_SCISSOR_ENDY: u32 = 0;
pub const D3D11_DEFAULT_SCISSOR_STARTX: u32 = 0;
pub const D3D11_DEFAULT_SCISSOR_STARTY: u32 = 0;
pub const D3D11_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0.0;
pub const D3D11_DEFAULT_STENCIL_READ_MASK: u32 = 255;
pub const D3D11_DEFAULT_STENCIL_REFERENCE: u32 = 0;
pub const D3D11_DEFAULT_STENCIL_WRITE_MASK: u32 = 255;
pub const D3D11_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0;
pub const D3D11_DEFAULT_VIEWPORT_HEIGHT: u32 = 0;
pub const D3D11_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0.0;
pub const D3D11_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0.0;
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
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::dxgi::DXGI_FORMAT,
    pub ViewDimension: D3D11_DSV_DIMENSION,
    pub Flags: u32,
    pub Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "dxgi")]
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D11_TEX1D_DSV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D11_TEX2D_DSV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D11_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "dxgi")]
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
pub const D3D11_DOMAIN_SHADER: D3D11_SHADER_TYPE = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_DOMAIN_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
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
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    pub ExtendedNV12SharedTextureSupported: windows_sys::core::BOOL,
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
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    pub InFormat: super::dxgi::DXGI_FORMAT,
    pub OutFormatSupport: u32,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    pub InFormat: super::dxgi::DXGI_FORMAT,
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
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    pub DecoderDesc: D3D11_VIDEO_DECODER_DESC,
    pub Components: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS,
    pub BinCount: u32,
    pub CounterBitDepth: u32,
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
pub type D3D11_FEATURE_VIDEO = i32;
pub const D3D11_FEATURE_VIDEO_DECODER_HISTOGRAM: D3D11_FEATURE_VIDEO = 0;
pub type D3D11_FENCE_FLAG = u32;
pub const D3D11_FENCE_FLAG_NONE: D3D11_FENCE_FLAG = 0;
pub const D3D11_FENCE_FLAG_NON_MONITORED: D3D11_FENCE_FLAG = 8;
pub const D3D11_FENCE_FLAG_SHARED: D3D11_FENCE_FLAG = 2;
pub const D3D11_FENCE_FLAG_SHARED_CROSS_ADAPTER: D3D11_FENCE_FLAG = 4;
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
pub const D3D11_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6;
pub const D3D11_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000.0;
pub const D3D11_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6;
pub const D3D11_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4;
pub const D3D11_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1.0;
pub const D3D11_FLOAT_TO_SRGB_OFFSET: f32 = 0.055;
pub const D3D11_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92;
pub const D3D11_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055;
pub const D3D11_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308;
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
pub const D3D11_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600.0;
pub const D3D11_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300.0;
pub const D3D11_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0.0;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_FUNCTION_DESC {
    pub Version: u32,
    pub Creator: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub MovInstructionCount: u32,
    pub MovcInstructionCount: u32,
    pub ConversionInstructionCount: u32,
    pub BitwiseInstructionCount: u32,
    pub MinFeatureLevel: super::d3dcommon::D3D_FEATURE_LEVEL,
    pub RequiredFeatureFlags: u64,
    pub Name: windows_sys::core::PCSTR,
    pub FunctionParameterCount: i32,
    pub HasReturn: windows_sys::core::BOOL,
    pub Has10Level9VertexShader: windows_sys::core::BOOL,
    pub Has10Level9PixelShader: windows_sys::core::BOOL,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D11_FUNCTION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_GEOMETRY_SHADER: D3D11_SHADER_TYPE = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_GEOMETRY_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
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
pub const D3D11_HS_MAXTESSFACTOR_LOWER_BOUND: f32 = 1.0;
pub const D3D11_HS_MAXTESSFACTOR_UPPER_BOUND: f32 = 64.0;
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
pub const D3D11_HULL_SHADER: D3D11_SHADER_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_HULL_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
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
pub const D3D11_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_INFO_QUEUE_FILTER {
    pub AllowList: D3D11_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D11_INFO_QUEUE_FILTER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D11_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D11_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D11_MESSAGE_ID,
}
impl Default for D3D11_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_INPUT_CLASSIFICATION = i32;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::dxgi::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "dxgi")]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    pub HWProtectionFunctionID: u32,
    pub pInputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA,
    pub pOutputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA,
    pub Status: windows_sys::core::HRESULT,
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    pub PrivateDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub pbInput: [u8; 4],
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    pub PrivateDataSize: u32,
    pub MaxHWProtectionDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub TransportTime: u64,
    pub ExecutionTime: u64,
    pub pbOutput: [u8; 4],
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_KEY_EXCHANGE_RSAES_OAEP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc1949895_d72a_4a1d_8e5d_ed857d171520);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_LIBRARY_DESC {
    pub Creator: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
impl Default for D3D11_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_LINEAR_GAMMA: f32 = 1.0;
pub type D3D11_LOGIC_OP = i32;
pub const D3D11_LOGIC_OP_AND: D3D11_LOGIC_OP = 6;
pub const D3D11_LOGIC_OP_AND_INVERTED: D3D11_LOGIC_OP = 13;
pub const D3D11_LOGIC_OP_AND_REVERSE: D3D11_LOGIC_OP = 12;
pub const D3D11_LOGIC_OP_CLEAR: D3D11_LOGIC_OP = 0;
pub const D3D11_LOGIC_OP_COPY: D3D11_LOGIC_OP = 2;
pub const D3D11_LOGIC_OP_COPY_INVERTED: D3D11_LOGIC_OP = 3;
pub const D3D11_LOGIC_OP_EQUIV: D3D11_LOGIC_OP = 11;
pub const D3D11_LOGIC_OP_INVERT: D3D11_LOGIC_OP = 5;
pub const D3D11_LOGIC_OP_NAND: D3D11_LOGIC_OP = 7;
pub const D3D11_LOGIC_OP_NOOP: D3D11_LOGIC_OP = 4;
pub const D3D11_LOGIC_OP_NOR: D3D11_LOGIC_OP = 9;
pub const D3D11_LOGIC_OP_OR: D3D11_LOGIC_OP = 8;
pub const D3D11_LOGIC_OP_OR_INVERTED: D3D11_LOGIC_OP = 15;
pub const D3D11_LOGIC_OP_OR_REVERSE: D3D11_LOGIC_OP = 14;
pub const D3D11_LOGIC_OP_SET: D3D11_LOGIC_OP = 1;
pub const D3D11_LOGIC_OP_XOR: D3D11_LOGIC_OP = 10;
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
pub const D3D11_MAX_BORDER_COLOR_COMPONENT: f32 = 1.0;
pub const D3D11_MAX_DEPTH: f32 = 1.0;
pub const D3D11_MAX_MAXANISOTROPY: u32 = 16;
pub const D3D11_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32;
pub const D3D11_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000.0;
pub const D3D11_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_MESSAGE {
    pub Category: D3D11_MESSAGE_CATEGORY,
    pub Severity: D3D11_MESSAGE_SEVERITY,
    pub ID: D3D11_MESSAGE_ID,
    pub pDescription: *const i8,
    pub DescriptionByteLength: usize,
}
impl Default for D3D11_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_MESSAGE_CATEGORY = i32;
pub const D3D11_MESSAGE_CATEGORY_APPLICATION_DEFINED: D3D11_MESSAGE_CATEGORY = 0;
pub const D3D11_MESSAGE_CATEGORY_CLEANUP: D3D11_MESSAGE_CATEGORY = 3;
pub const D3D11_MESSAGE_CATEGORY_COMPILATION: D3D11_MESSAGE_CATEGORY = 4;
pub const D3D11_MESSAGE_CATEGORY_EXECUTION: D3D11_MESSAGE_CATEGORY = 9;
pub const D3D11_MESSAGE_CATEGORY_INITIALIZATION: D3D11_MESSAGE_CATEGORY = 2;
pub const D3D11_MESSAGE_CATEGORY_MISCELLANEOUS: D3D11_MESSAGE_CATEGORY = 1;
pub const D3D11_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: D3D11_MESSAGE_CATEGORY = 8;
pub const D3D11_MESSAGE_CATEGORY_SHADER: D3D11_MESSAGE_CATEGORY = 10;
pub const D3D11_MESSAGE_CATEGORY_STATE_CREATION: D3D11_MESSAGE_CATEGORY = 5;
pub const D3D11_MESSAGE_CATEGORY_STATE_GETTING: D3D11_MESSAGE_CATEGORY = 7;
pub const D3D11_MESSAGE_CATEGORY_STATE_SETTING: D3D11_MESSAGE_CATEGORY = 6;
pub type D3D11_MESSAGE_ID = i32;
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDARRAY: D3D11_MESSAGE_ID = 3146029;
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDBIND: D3D11_MESSAGE_ID = 3146028;
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDTYPE: D3D11_MESSAGE_ID = 3146027;
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_NULLPARAM: D3D11_MESSAGE_ID = 3146026;
pub const D3D11_MESSAGE_ID_BLENDSTATE_GETDESC_LEGACY: D3D11_MESSAGE_ID = 392;
pub const D3D11_MESSAGE_ID_BUFFER_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = 297;
pub const D3D11_MESSAGE_ID_BUFFER_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = 298;
pub const D3D11_MESSAGE_ID_BUFFER_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = 296;
pub const D3D11_MESSAGE_ID_BUFFER_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = 295;
pub const D3D11_MESSAGE_ID_BUFFER_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = 299;
pub const D3D11_MESSAGE_ID_CANNOT_ADD_TRACKED_WORKLOAD: D3D11_MESSAGE_ID = 3146278;
pub const D3D11_MESSAGE_ID_CHECKCOUNTER_OUTOFRANGE_COUNTER: D3D11_MESSAGE_ID = 402;
pub const D3D11_MESSAGE_ID_CHECKCOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D11_MESSAGE_ID = 403;
pub const D3D11_MESSAGE_ID_CHECKCRYPTOKEYEXCHANGE_INVALIDINDEX: D3D11_MESSAGE_ID = 3145994;
pub const D3D11_MESSAGE_ID_CHECKCRYPTOKEYEXCHANGE_NULLPARAM: D3D11_MESSAGE_ID = 3145993;
pub const D3D11_MESSAGE_ID_CHECKCRYPTOSESSIONSTATUS_NULLPARAM: D3D11_MESSAGE_ID = 3146085;
pub const D3D11_MESSAGE_ID_CHECKFEATURESUPPORT_FORMAT_DEPRECATED: D3D11_MESSAGE_ID = 2097371;
pub const D3D11_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_DEPRECATED: D3D11_MESSAGE_ID = 321;
pub const D3D11_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_NOT_SUPPORTED: D3D11_MESSAGE_ID = 2097401;
pub const D3D11_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_FORMAT_DEPRECATED: D3D11_MESSAGE_ID = 322;
pub const D3D11_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_INVALIDFLAGS: D3D11_MESSAGE_ID = 3146138;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_INVALIDCOLORSPACE: D3D11_MESSAGE_ID = 3146090;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_NULLPARAM: D3D11_MESSAGE_ID = 3146089;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_ZEROWIDTHHEIGHT: D3D11_MESSAGE_ID = 3146091;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERFORMAT_NULLPARAM: D3D11_MESSAGE_ID = 3145768;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERFORMAT_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145769;
pub const D3D11_MESSAGE_ID_CHECKVIDEOPROCESSORFORMATCONVERSION_NULLPARAM: D3D11_MESSAGE_ID = 3146096;
pub const D3D11_MESSAGE_ID_CHECKVIDEOPROCESSORFORMAT_NULLPARAM: D3D11_MESSAGE_ID = 3145799;
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DENORMFLUSH: D3D11_MESSAGE_ID = 262;
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DEPTH_READONLY: D3D11_MESSAGE_ID = 2097369;
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: D3D11_MESSAGE_ID = 263;
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_STENCIL_READONLY: D3D11_MESSAGE_ID = 2097370;
pub const D3D11_MESSAGE_ID_CLEARRENDERTARGETVIEW_DENORMFLUSH: D3D11_MESSAGE_ID = 261;
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWFLOAT_HAZARD: D3D11_MESSAGE_ID = 3146145;
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWFLOAT_INVALIDFORMAT: D3D11_MESSAGE_ID = 2097405;
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWUINT_HAZARD: D3D11_MESSAGE_ID = 3146144;
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_DENORMFLUSH: D3D11_MESSAGE_ID = 2097355;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_INVALIDPROCESSIDTYPE: D3D11_MESSAGE_ID = 3146015;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_NULLPARAM: D3D11_MESSAGE_ID = 3146011;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_UNSUPPORTEDCONFIGURE: D3D11_MESSAGE_ID = 3146013;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_WRONGCHANNEL: D3D11_MESSAGE_ID = 3146012;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_WRONGSIZE: D3D11_MESSAGE_ID = 3146014;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDDESTINATIONSTATE: D3D11_MESSAGE_ID = 285;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCE: D3D11_MESSAGE_ID = 284;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCESTATE: D3D11_MESSAGE_ID = 286;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_3D_MISMATCHED_UPDATES: D3D11_MESSAGE_ID = 1048637;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_3D_READBACK: D3D11_MESSAGE_ID = 1048597;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_ONLY_READBACK: D3D11_MESSAGE_ID = 1048598;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_ONLY_TEXTURE_2D_WITHIN_GPU_MEMORY: D3D11_MESSAGE_ID = 1048596;
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDDESTINATIONSTATE: D3D11_MESSAGE_ID = 2097399;
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDOFFSET: D3D11_MESSAGE_ID = 2097397;
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDSOURCESTATE: D3D11_MESSAGE_ID = 2097400;
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_LARGEOFFSET: D3D11_MESSAGE_ID = 2097398;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION1_INVALIDCOPYFLAGS: D3D11_MESSAGE_ID = 3145755;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_EMPTYSOURCEBOX: D3D11_MESSAGE_ID = 3146078;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSTATE: D3D11_MESSAGE_ID = 282;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSUBRESOURCE: D3D11_MESSAGE_ID = 278;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCE: D3D11_MESSAGE_ID = 281;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCEBOX: D3D11_MESSAGE_ID = 280;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESTATE: D3D11_MESSAGE_ID = 283;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESUBRESOURCE: D3D11_MESSAGE_ID = 279;
pub const D3D11_MESSAGE_ID_COPYTILEMAPPINGS_INVALID_PARAMETER: D3D11_MESSAGE_ID = 3146126;
pub const D3D11_MESSAGE_ID_COPYTILES_INVALID_PARAMETER: D3D11_MESSAGE_ID = 3146127;
pub const D3D11_MESSAGE_ID_CORRUPTED_MULTITHREADING: D3D11_MESSAGE_ID = 28;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER1: D3D11_MESSAGE_ID = 13;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER10: D3D11_MESSAGE_ID = 22;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER11: D3D11_MESSAGE_ID = 23;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER12: D3D11_MESSAGE_ID = 24;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER13: D3D11_MESSAGE_ID = 25;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER14: D3D11_MESSAGE_ID = 26;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER15: D3D11_MESSAGE_ID = 27;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER2: D3D11_MESSAGE_ID = 14;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER3: D3D11_MESSAGE_ID = 15;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER4: D3D11_MESSAGE_ID = 16;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER5: D3D11_MESSAGE_ID = 17;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER6: D3D11_MESSAGE_ID = 18;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER7: D3D11_MESSAGE_ID = 19;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER8: D3D11_MESSAGE_ID = 20;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER9: D3D11_MESSAGE_ID = 21;
pub const D3D11_MESSAGE_ID_CORRUPTED_THIS: D3D11_MESSAGE_ID = 12;
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_INVALIDTYPE: D3D11_MESSAGE_ID = 3145997;
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_NULLPARAM: D3D11_MESSAGE_ID = 3145995;
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145998;
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_UNSUPPORTED: D3D11_MESSAGE_ID = 3145996;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: D3D11_MESSAGE_ID = 214;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: D3D11_MESSAGE_ID = 217;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: D3D11_MESSAGE_ID = 213;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: D3D11_MESSAGE_ID = 216;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDLOGICOPS: D3D11_MESSAGE_ID = 3145941;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: D3D11_MESSAGE_ID = 218;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: D3D11_MESSAGE_ID = 212;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: D3D11_MESSAGE_ID = 215;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_ALPHA_TO_COVERAGE: D3D11_MESSAGE_ID = 1048600;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_BLEND_ENABLE: D3D11_MESSAGE_ID = 1048612;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_WRITE_MASKS: D3D11_MESSAGE_ID = 1048613;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_MRT_BLEND: D3D11_MESSAGE_ID = 1048623;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_SEPARATE_ALPHA_BLEND: D3D11_MESSAGE_ID = 1048622;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NULLDESC: D3D11_MESSAGE_ID = 220;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_OPERATION_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048624;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 219;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 69;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDBINDFLAGS: D3D11_MESSAGE_ID = 64;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDCONSTANTBUFFERBINDINGS: D3D11_MESSAGE_ID = 72;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDCPUACCESSFLAGS: D3D11_MESSAGE_ID = 63;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 66;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDINITIALDATA: D3D11_MESSAGE_ID = 65;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDMIPLEVELS: D3D11_MESSAGE_ID = 67;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = 68;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDSAMPLES: D3D11_MESSAGE_ID = 58;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDSTRUCTURESTRIDE: D3D11_MESSAGE_ID = 2097339;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDUSAGE: D3D11_MESSAGE_ID = 3146120;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_LARGEALLOCATION: D3D11_MESSAGE_ID = 73;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_NULLDESC: D3D11_MESSAGE_ID = 71;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 70;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDBINDFLAGS: D3D11_MESSAGE_ID = 60;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDCPUACCESSFLAGS: D3D11_MESSAGE_ID = 61;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = 57;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDMISCFLAGS: D3D11_MESSAGE_ID = 62;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDUSAGE: D3D11_MESSAGE_ID = 59;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCALL: D3D11_MESSAGE_ID = 2097320;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = 2097324;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = 2097322;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = 2097323;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = 2097321;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_NONEXCLUSIVE_RETURN: D3D11_MESSAGE_ID = 400;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_NULLDESC: D3D11_MESSAGE_ID = 401;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 399;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_OUTOFRANGE_COUNTER: D3D11_MESSAGE_ID = 396;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_SIMULTANEOUS_ACTIVE_COUNTERS_EXHAUSTED: D3D11_MESSAGE_ID = 397;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D11_MESSAGE_ID = 398;
pub const D3D11_MESSAGE_ID_CREATECRYPTOSESSION_NULLPARAM: D3D11_MESSAGE_ID = 3145951;
pub const D3D11_MESSAGE_ID_CREATECRYPTOSESSION_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145952;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 2097163;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALID_CALL_RETURN: D3D11_MESSAGE_ID = 2097164;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALID_COMMANDLISTFLAGS: D3D11_MESSAGE_ID = 2097161;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 2097165;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_SINGLETHREADED: D3D11_MESSAGE_ID = 2097162;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: D3D11_MESSAGE_ID = 206;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: D3D11_MESSAGE_ID = 209;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: D3D11_MESSAGE_ID = 208;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: D3D11_MESSAGE_ID = 207;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: D3D11_MESSAGE_ID = 201;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: D3D11_MESSAGE_ID = 200;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: D3D11_MESSAGE_ID = 202;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: D3D11_MESSAGE_ID = 205;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: D3D11_MESSAGE_ID = 204;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: D3D11_MESSAGE_ID = 203;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_NULLDESC: D3D11_MESSAGE_ID = 211;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_STENCIL_NO_TWO_SIDED: D3D11_MESSAGE_ID = 1048577;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 210;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 148;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: D3D11_MESSAGE_ID = 143;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 145;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFLAGS: D3D11_MESSAGE_ID = 2097153;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = 144;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: D3D11_MESSAGE_ID = 146;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 149;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 147;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = 142;
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_FEATURELEVELS_NOT_SUPPORTED: D3D11_MESSAGE_ID = 3145752;
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDFEATURELEVEL: D3D11_MESSAGE_ID = 3145751;
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDFLAGS: D3D11_MESSAGE_ID = 3145750;
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDREFIID: D3D11_MESSAGE_ID = 3145753;
pub const D3D11_MESSAGE_ID_CREATEDEVICE_INVALIDARGS: D3D11_MESSAGE_ID = 3146142;
pub const D3D11_MESSAGE_ID_CREATEDEVICE_WARNING: D3D11_MESSAGE_ID = 3146143;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCALL: D3D11_MESSAGE_ID = 2097193;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = 2097197;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = 2097195;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = 2097196;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = 2097194;
pub const D3D11_MESSAGE_ID_CREATEFENCE_INVALIDFLAGS: D3D11_MESSAGE_ID = 3146256;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: D3D11_MESSAGE_ID = 188;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: D3D11_MESSAGE_ID = 189;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_EXPECTEDDECL: D3D11_MESSAGE_ID = 177;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = 2097159;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: D3D11_MESSAGE_ID = 181;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: D3D11_MESSAGE_ID = 183;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: D3D11_MESSAGE_ID = 174;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTREAMS: D3D11_MESSAGE_ID = 2097156;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTRIDES: D3D11_MESSAGE_ID = 2097172;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: D3D11_MESSAGE_ID = 179;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: D3D11_MESSAGE_ID = 185;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = 172;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = 173;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: D3D11_MESSAGE_ID = 182;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAM: D3D11_MESSAGE_ID = 2097169;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAMTORASTERIZER: D3D11_MESSAGE_ID = 2097157;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: D3D11_MESSAGE_ID = 187;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: D3D11_MESSAGE_ID = 190;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: D3D11_MESSAGE_ID = 186;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: D3D11_MESSAGE_ID = 180;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: D3D11_MESSAGE_ID = 171;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: D3D11_MESSAGE_ID = 178;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: D3D11_MESSAGE_ID = 175;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: D3D11_MESSAGE_ID = 184;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: D3D11_MESSAGE_ID = 386;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDDECL: D3D11_MESSAGE_ID = 176;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDENTRIES: D3D11_MESSAGE_ID = 2097170;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTREAMS: D3D11_MESSAGE_ID = 2097158;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTRIDES: D3D11_MESSAGE_ID = 2097171;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = 2097155;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = 169;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = 170;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = 168;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDCALL: D3D11_MESSAGE_ID = 2097177;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = 2097181;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = 2097179;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = 2097180;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = 2097178;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: D3D11_MESSAGE_ID = 160;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: D3D11_MESSAGE_ID = 420;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: D3D11_MESSAGE_ID = 153;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: D3D11_MESSAGE_ID = 159;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: D3D11_MESSAGE_ID = 152;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: D3D11_MESSAGE_ID = 155;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: D3D11_MESSAGE_ID = 154;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: D3D11_MESSAGE_ID = 157;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: D3D11_MESSAGE_ID = 158;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_LEVEL9_INSTANCING_NOT_SUPPORTED: D3D11_MESSAGE_ID = 3146124;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_LEVEL9_STEPRATE_NOT_1: D3D11_MESSAGE_ID = 3146123;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: D3D11_MESSAGE_ID = 163;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_NULLDESC: D3D11_MESSAGE_ID = 164;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: D3D11_MESSAGE_ID = 162;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: D3D11_MESSAGE_ID = 150;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: D3D11_MESSAGE_ID = 156;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: D3D11_MESSAGE_ID = 151;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: D3D11_MESSAGE_ID = 385;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: D3D11_MESSAGE_ID = 391;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: D3D11_MESSAGE_ID = 161;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_UNSUPPORTED_FORMAT: D3D11_MESSAGE_ID = 1048599;
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = 2097160;
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = 192;
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = 193;
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = 191;
pub const D3D11_MESSAGE_ID_CREATEPREDICATE_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 395;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_DECODENOTSUPPORTED: D3D11_MESSAGE_ID = 3146158;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_ENCODENOTSUPPORTED: D3D11_MESSAGE_ID = 3146159;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDCONTEXTTYPE: D3D11_MESSAGE_ID = 3146157;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = 233;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDQUERY: D3D11_MESSAGE_ID = 232;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_NULLDESC: D3D11_MESSAGE_ID = 235;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_UNEXPECTEDMISCFLAG: D3D11_MESSAGE_ID = 234;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_UNSUPPORTEDCONTEXTTTYPEFORQUERY: D3D11_MESSAGE_ID = 3146222;
pub const D3D11_MESSAGE_ID_CREATEQUERY_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 394;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_DepthBiasClamp_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048578;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_DepthClipEnable_MUST_BE_TRUE: D3D11_MESSAGE_ID = 1048601;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: D3D11_MESSAGE_ID = 195;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: D3D11_MESSAGE_ID = 196;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: D3D11_MESSAGE_ID = 194;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFORCEDSAMPLECOUNT: D3D11_MESSAGE_ID = 3145757;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: D3D11_MESSAGE_ID = 197;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_CONSERVATIVERASTERMODE: D3D11_MESSAGE_ID = 3146155;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_NULLDESC: D3D11_MESSAGE_ID = 199;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 198;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_AMBIGUOUSVIDEOPLANEINDEX: D3D11_MESSAGE_ID = 3146165;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 140;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDARRAYWITHDECODER: D3D11_MESSAGE_ID = 3145944;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: D3D11_MESSAGE_ID = 135;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 137;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = 136;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDPLANEINDEX: D3D11_MESSAGE_ID = 3146163;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: D3D11_MESSAGE_ID = 138;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDVIDEOPLANEINDEX: D3D11_MESSAGE_ID = 3146164;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 141;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 139;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = 133;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = 134;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DIMENSION_EXCEEDS_FEATURE_LEVEL_DEFINITION: D3D11_MESSAGE_ID = 1048630;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DIMENSION_OUT_OF_RANGE: D3D11_MESSAGE_ID = 1048588;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DXGI_FORMAT_R8G8B8A8_CANNOT_BE_SHARED: D3D11_MESSAGE_ID = 1048617;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_MSAA_PRECLUDES_SHADER_RESOURCE: D3D11_MESSAGE_ID = 1048610;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NON_POW_2_MIPMAP: D3D11_MESSAGE_ID = 1048634;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_RENDER_TARGET: D3D11_MESSAGE_ID = 1048608;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_SHADER_RESOURCE: D3D11_MESSAGE_ID = 1048589;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_ARRAYS: D3D11_MESSAGE_ID = 1048585;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_AUTOGEN_FOR_VOLUMES: D3D11_MESSAGE_ID = 1048616;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_DWORD_INDEX_BUFFER: D3D11_MESSAGE_ID = 1048609;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_STREAM_OUT: D3D11_MESSAGE_ID = 1048614;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_TEXTURE_1D: D3D11_MESSAGE_ID = 1048587;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_VB_AND_IB_BIND: D3D11_MESSAGE_ID = 1048586;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_ONLY_SINGLE_MIP_LEVEL_DEPTH_STENCIL_SUPPORTED: D3D11_MESSAGE_ID = 1048631;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_ONLY_VB_IB_FOR_BUFFERS: D3D11_MESSAGE_ID = 1048615;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_PRESENTATION_PRECLUDES_SHADER_RESOURCE: D3D11_MESSAGE_ID = 1048611;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048635;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_OUT_OF_RANGE: D3D11_MESSAGE_ID = 1048581;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_EXCESSIVE_ANISOTROPY: D3D11_MESSAGE_ID = 1048580;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSU: D3D11_MESSAGE_ID = 222;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSV: D3D11_MESSAGE_ID = 223;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSW: D3D11_MESSAGE_ID = 224;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDCOMPARISONFUNC: D3D11_MESSAGE_ID = 227;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDFILTER: D3D11_MESSAGE_ID = 221;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXANISOTROPY: D3D11_MESSAGE_ID = 226;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXLOD: D3D11_MESSAGE_ID = 229;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMINLOD: D3D11_MESSAGE_ID = 228;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMIPLODBIAS: D3D11_MESSAGE_ID = 225;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_MAXLOD_MUST_BE_FLT_MAX: D3D11_MESSAGE_ID = 1048605;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_MINLOD_MUST_NOT_BE_FRACTIONAL: D3D11_MESSAGE_ID = 1048604;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NO_COMPARISON_SUPPORT: D3D11_MESSAGE_ID = 1048579;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NO_MIRRORONCE: D3D11_MESSAGE_ID = 1048625;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NULLDESC: D3D11_MESSAGE_ID = 231;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 230;
pub const D3D11_MESSAGE_ID_CREATESHADERRESESOURCEVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 2097359;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_AMBIGUOUSVIDEOPLANEINDEX: D3D11_MESSAGE_ID = 3146162;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_CUBES_MUST_HAVE_6_SIDES: D3D11_MESSAGE_ID = 1048607;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_FIRSTARRAYSLICE_MUST_BE_ZERO: D3D11_MESSAGE_ID = 1048606;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 131;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDARRAYWITHDECODER: D3D11_MESSAGE_ID = 3145942;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: D3D11_MESSAGE_ID = 126;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 128;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFLAGS: D3D11_MESSAGE_ID = 2097340;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = 127;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANEINDEX: D3D11_MESSAGE_ID = 3146160;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: D3D11_MESSAGE_ID = 129;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANEINDEX: D3D11_MESSAGE_ID = 3146161;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_MUST_USE_LOWEST_LOD: D3D11_MESSAGE_ID = 1048603;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 132;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 130;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = 125;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 87;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDBINDFLAGS: D3D11_MESSAGE_ID = 82;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDCPUACCESSFLAGS: D3D11_MESSAGE_ID = 81;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 84;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDINITIALDATA: D3D11_MESSAGE_ID = 83;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDMIPLEVELS: D3D11_MESSAGE_ID = 85;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = 86;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDSAMPLES: D3D11_MESSAGE_ID = 76;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDUSAGE: D3D11_MESSAGE_ID = 3146121;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_LARGEALLOCATION: D3D11_MESSAGE_ID = 90;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_NULLDESC: D3D11_MESSAGE_ID = 89;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 88;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDBINDFLAGS: D3D11_MESSAGE_ID = 78;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDCPUACCESSFLAGS: D3D11_MESSAGE_ID = 79;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = 74;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDMISCFLAGS: D3D11_MESSAGE_ID = 80;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDUSAGE: D3D11_MESSAGE_ID = 77;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = 75;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 104;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDBINDFLAGS: D3D11_MESSAGE_ID = 99;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDCPUACCESSFLAGS: D3D11_MESSAGE_ID = 98;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 101;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDINITIALDATA: D3D11_MESSAGE_ID = 100;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDMIPLEVELS: D3D11_MESSAGE_ID = 102;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = 103;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDSAMPLES: D3D11_MESSAGE_ID = 93;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDUSAGE: D3D11_MESSAGE_ID = 3146122;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_LARGEALLOCATION: D3D11_MESSAGE_ID = 107;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_NULLDESC: D3D11_MESSAGE_ID = 106;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 105;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDBINDFLAGS: D3D11_MESSAGE_ID = 95;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDCPUACCESSFLAGS: D3D11_MESSAGE_ID = 96;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = 91;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDMISCFLAGS: D3D11_MESSAGE_ID = 97;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDUSAGE: D3D11_MESSAGE_ID = 94;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = 92;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 121;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDBINDFLAGS: D3D11_MESSAGE_ID = 116;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDCPUACCESSFLAGS: D3D11_MESSAGE_ID = 115;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 118;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDINITIALDATA: D3D11_MESSAGE_ID = 117;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDMIPLEVELS: D3D11_MESSAGE_ID = 119;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = 120;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDSAMPLES: D3D11_MESSAGE_ID = 110;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_LARGEALLOCATION: D3D11_MESSAGE_ID = 124;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_NULLDESC: D3D11_MESSAGE_ID = 123;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 122;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDBINDFLAGS: D3D11_MESSAGE_ID = 112;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDCPUACCESSFLAGS: D3D11_MESSAGE_ID = 113;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = 108;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDMISCFLAGS: D3D11_MESSAGE_ID = 114;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDUSAGE: D3D11_MESSAGE_ID = 111;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = 109;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_AMBIGUOUSVIDEOPLANEINDEX: D3D11_MESSAGE_ID = 3146168;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 2097351;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDARRAYWITHDECODER: D3D11_MESSAGE_ID = 3145943;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC: D3D11_MESSAGE_ID = 2097342;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 2097344;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS: D3D11_MESSAGE_ID = 2097358;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = 2097343;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANEINDEX: D3D11_MESSAGE_ID = 3146166;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE: D3D11_MESSAGE_ID = 2097341;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANEINDEX: D3D11_MESSAGE_ID = 3146167;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 2097352;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = 2097353;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = 2097345;
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = 2097154;
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = 166;
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = 167;
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = 165;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDARRAY: D3D11_MESSAGE_ID = 3145915;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDARRAYSIZE: D3D11_MESSAGE_ID = 3145914;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDBIND: D3D11_MESSAGE_ID = 3145910;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDDIMENSION: D3D11_MESSAGE_ID = 3145916;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDMIP: D3D11_MESSAGE_ID = 3145912;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDTYPE: D3D11_MESSAGE_ID = 3145909;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_NULLPARAM: D3D11_MESSAGE_ID = 3145908;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145907;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = 3145911;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_UNSUPPORTEMIP: D3D11_MESSAGE_ID = 3145913;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_DRIVER_INVALIDBUFFERSIZE: D3D11_MESSAGE_ID = 3145762;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_DRIVER_INVALIDBUFFERUSAGE: D3D11_MESSAGE_ID = 3145763;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_INVALIDFORMAT: D3D11_MESSAGE_ID = 3145760;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_NULLPARAM: D3D11_MESSAGE_ID = 3145759;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145758;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_ZEROWIDTHHEIGHT: D3D11_MESSAGE_ID = 3145761;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDFRAMEFORMAT: D3D11_MESSAGE_ID = 3145793;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDINPUTFRAMERATE: D3D11_MESSAGE_ID = 3145795;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDOUTPUTFRAMERATE: D3D11_MESSAGE_ID = 3145796;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDUSAGE: D3D11_MESSAGE_ID = 3145794;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDWIDTHHEIGHT: D3D11_MESSAGE_ID = 3145797;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_NULLPARAM: D3D11_MESSAGE_ID = 3145792;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145791;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDARRAY: D3D11_MESSAGE_ID = 3145928;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDARRAYSIZE: D3D11_MESSAGE_ID = 3145927;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDBIND: D3D11_MESSAGE_ID = 3145920;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDDIMENSION: D3D11_MESSAGE_ID = 3145929;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = 3145923;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDFOURCC: D3D11_MESSAGE_ID = 3145924;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMIP: D3D11_MESSAGE_ID = 3145925;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMISC: D3D11_MESSAGE_ID = 3145921;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMSAA: D3D11_MESSAGE_ID = 3146073;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDTYPE: D3D11_MESSAGE_ID = 3145919;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDUSAGE: D3D11_MESSAGE_ID = 3145922;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_NULLPARAM: D3D11_MESSAGE_ID = 3145918;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145917;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_UNSUPPORTEDMIP: D3D11_MESSAGE_ID = 3145926;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDARRAY: D3D11_MESSAGE_ID = 3145938;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDBIND: D3D11_MESSAGE_ID = 3145933;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDDIMENSION: D3D11_MESSAGE_ID = 3145939;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = 3145934;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDMIP: D3D11_MESSAGE_ID = 3145935;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDMSAA: D3D11_MESSAGE_ID = 3146074;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDTYPE: D3D11_MESSAGE_ID = 3145932;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_NULLPARAM: D3D11_MESSAGE_ID = 3145931;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145930;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_UNSUPPORTEDARRAY: D3D11_MESSAGE_ID = 3145937;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_UNSUPPORTEDMIP: D3D11_MESSAGE_ID = 3145936;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOR_NULLPARAM: D3D11_MESSAGE_ID = 3145808;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOR_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145807;
pub const D3D11_MESSAGE_ID_CREATE_AUTHENTICATEDCHANNEL: D3D11_MESSAGE_ID = 3146148;
pub const D3D11_MESSAGE_ID_CREATE_BLENDSTATE: D3D11_MESSAGE_ID = 2097270;
pub const D3D11_MESSAGE_ID_CREATE_BUFFER: D3D11_MESSAGE_ID = 2097228;
pub const D3D11_MESSAGE_ID_CREATE_CLASSINSTANCE: D3D11_MESSAGE_ID = 2097290;
pub const D3D11_MESSAGE_ID_CREATE_CLASSLINKAGE: D3D11_MESSAGE_ID = 2097293;
pub const D3D11_MESSAGE_ID_CREATE_COMMANDLIST: D3D11_MESSAGE_ID = 2097287;
pub const D3D11_MESSAGE_ID_CREATE_COMPUTESHADER: D3D11_MESSAGE_ID = 2097298;
pub const D3D11_MESSAGE_ID_CREATE_CONTEXT: D3D11_MESSAGE_ID = 2097225;
pub const D3D11_MESSAGE_ID_CREATE_COUNTER: D3D11_MESSAGE_ID = 2097285;
pub const D3D11_MESSAGE_ID_CREATE_CRYPTOSESSION: D3D11_MESSAGE_ID = 3146147;
pub const D3D11_MESSAGE_ID_CREATE_DECODEROUTPUTVIEW: D3D11_MESSAGE_ID = 3145732;
pub const D3D11_MESSAGE_ID_CREATE_DEPTHSTENCILSTATE: D3D11_MESSAGE_ID = 2097273;
pub const D3D11_MESSAGE_ID_CREATE_DEPTHSTENCILVIEW: D3D11_MESSAGE_ID = 2097246;
pub const D3D11_MESSAGE_ID_CREATE_DEVICECONTEXTSTATE: D3D11_MESSAGE_ID = 3145735;
pub const D3D11_MESSAGE_ID_CREATE_DOMAINSHADER: D3D11_MESSAGE_ID = 2097255;
pub const D3D11_MESSAGE_ID_CREATE_FENCE: D3D11_MESSAGE_ID = 3146250;
pub const D3D11_MESSAGE_ID_CREATE_GEOMETRYSHADER: D3D11_MESSAGE_ID = 2097258;
pub const D3D11_MESSAGE_ID_CREATE_HULLSHADER: D3D11_MESSAGE_ID = 2097252;
pub const D3D11_MESSAGE_ID_CREATE_INPUTLAYOUT: D3D11_MESSAGE_ID = 2097264;
pub const D3D11_MESSAGE_ID_CREATE_PIXELSHADER: D3D11_MESSAGE_ID = 2097261;
pub const D3D11_MESSAGE_ID_CREATE_PREDICATE: D3D11_MESSAGE_ID = 2097282;
pub const D3D11_MESSAGE_ID_CREATE_PROCESSORINPUTVIEW: D3D11_MESSAGE_ID = 3145733;
pub const D3D11_MESSAGE_ID_CREATE_PROCESSOROUTPUTVIEW: D3D11_MESSAGE_ID = 3145734;
pub const D3D11_MESSAGE_ID_CREATE_QUERY: D3D11_MESSAGE_ID = 2097279;
pub const D3D11_MESSAGE_ID_CREATE_RASTERIZERSTATE: D3D11_MESSAGE_ID = 2097276;
pub const D3D11_MESSAGE_ID_CREATE_RENDERTARGETVIEW: D3D11_MESSAGE_ID = 2097243;
pub const D3D11_MESSAGE_ID_CREATE_SAMPLER: D3D11_MESSAGE_ID = 2097267;
pub const D3D11_MESSAGE_ID_CREATE_SHADERRESOURCEVIEW: D3D11_MESSAGE_ID = 2097240;
pub const D3D11_MESSAGE_ID_CREATE_SYNCHRONIZEDCHANNEL: D3D11_MESSAGE_ID = 3146253;
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE1D: D3D11_MESSAGE_ID = 2097231;
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE2D: D3D11_MESSAGE_ID = 2097234;
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE3D: D3D11_MESSAGE_ID = 2097237;
pub const D3D11_MESSAGE_ID_CREATE_TRACKEDWORKLOAD: D3D11_MESSAGE_ID = 3146267;
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_DEADLINE_TYPE: D3D11_MESSAGE_ID = 3146272;
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_ENGINE_TYPE: D3D11_MESSAGE_ID = 3146273;
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_MAX_INSTANCES: D3D11_MESSAGE_ID = 3146271;
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_NULLPARAM: D3D11_MESSAGE_ID = 3146270;
pub const D3D11_MESSAGE_ID_CREATE_UNORDEREDACCESSVIEW: D3D11_MESSAGE_ID = 2097301;
pub const D3D11_MESSAGE_ID_CREATE_VERTEXSHADER: D3D11_MESSAGE_ID = 2097249;
pub const D3D11_MESSAGE_ID_CREATE_VIDEODECODER: D3D11_MESSAGE_ID = 3145729;
pub const D3D11_MESSAGE_ID_CREATE_VIDEOPROCESSOR: D3D11_MESSAGE_ID = 3145731;
pub const D3D11_MESSAGE_ID_CREATE_VIDEOPROCESSORENUM: D3D11_MESSAGE_ID = 3145730;
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = 2097326;
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = 3146021;
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097319;
pub const D3D11_MESSAGE_ID_CSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097415;
pub const D3D11_MESSAGE_ID_CSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097318;
pub const D3D11_MESSAGE_ID_CSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097418;
pub const D3D11_MESSAGE_ID_CSSETUNORDEREDACCESSVIEWS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097349;
pub const D3D11_MESSAGE_ID_D3D10L9_MESSAGES_END: D3D11_MESSAGE_ID = 1048638;
pub const D3D11_MESSAGE_ID_D3D10L9_MESSAGES_START: D3D11_MESSAGE_ID = 1048576;
pub const D3D11_MESSAGE_ID_D3D10_MESSAGES_END: D3D11_MESSAGE_ID = 443;
pub const D3D11_MESSAGE_ID_D3D11_1_MESSAGES_END: D3D11_MESSAGE_ID = 3146118;
pub const D3D11_MESSAGE_ID_D3D11_1_MESSAGES_START: D3D11_MESSAGE_ID = 3145728;
pub const D3D11_MESSAGE_ID_D3D11_2_MESSAGES_END: D3D11_MESSAGE_ID = 3146153;
pub const D3D11_MESSAGE_ID_D3D11_2_MESSAGES_START: D3D11_MESSAGE_ID = 3146119;
pub const D3D11_MESSAGE_ID_D3D11_3_MESSAGES_END: D3D11_MESSAGE_ID = 3146257;
pub const D3D11_MESSAGE_ID_D3D11_3_MESSAGES_START: D3D11_MESSAGE_ID = 3146154;
pub const D3D11_MESSAGE_ID_D3D11_5_MESSAGES_END: D3D11_MESSAGE_ID = 3146285;
pub const D3D11_MESSAGE_ID_D3D11_5_MESSAGES_START: D3D11_MESSAGE_ID = 3146258;
pub const D3D11_MESSAGE_ID_D3D11_MESSAGES_END: D3D11_MESSAGE_ID = 2097424;
pub const D3D11_MESSAGE_ID_D3D11_MESSAGES_START: D3D11_MESSAGE_ID = 2097152;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_HAZARD: D3D11_MESSAGE_ID = 3145785;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_MISC_FLAGS: D3D11_MESSAGE_ID = 3146265;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_OFFSET: D3D11_MESSAGE_ID = 3146266;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_SIZE: D3D11_MESSAGE_ID = 3146263;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_USAGE: D3D11_MESSAGE_ID = 3146264;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_COMPONENT: D3D11_MESSAGE_ID = 3146262;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_COMPONENT_COUNT: D3D11_MESSAGE_ID = 3146261;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_NULLPARAM: D3D11_MESSAGE_ID = 3145784;
pub const D3D11_MESSAGE_ID_DECODERENDFRAME_NULLPARAM: D3D11_MESSAGE_ID = 3145786;
pub const D3D11_MESSAGE_ID_DECODEREXTENSION_INVALIDRESOURCE: D3D11_MESSAGE_ID = 3145790;
pub const D3D11_MESSAGE_ID_DECODEREXTENSION_NULLPARAM: D3D11_MESSAGE_ID = 3145789;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_MAPPED: D3D11_MESSAGE_ID = 3145983;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_MULTISAMPLED: D3D11_MESSAGE_ID = 3145979;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_NOT_RENDER_TARGET: D3D11_MESSAGE_ID = 3145981;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_OFFERED: D3D11_MESSAGE_ID = 3145985;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_WRONGDEVICE: D3D11_MESSAGE_ID = 3145976;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_FORMAT_MISMATCH: D3D11_MESSAGE_ID = 3145977;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_NULLPARAM: D3D11_MESSAGE_ID = 3145974;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SIZE_MISMATCH: D3D11_MESSAGE_ID = 3145978;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_CONTENT_UNDEFINED: D3D11_MESSAGE_ID = 3145986;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_MAPPED: D3D11_MESSAGE_ID = 3145982;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_NOT_STAGING: D3D11_MESSAGE_ID = 3145980;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_OFFERED: D3D11_MESSAGE_ID = 3145984;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_WRONGDEVICE: D3D11_MESSAGE_ID = 3145975;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_UNSUPPORTED: D3D11_MESSAGE_ID = 3145973;
pub const D3D11_MESSAGE_ID_DEFERRED_CONTEXT_REMOVAL_PROCESS_AT_FAULT: D3D11_MESSAGE_ID = 2097206;
pub const D3D11_MESSAGE_ID_DESTROY_AUTHENTICATEDCHANNEL: D3D11_MESSAGE_ID = 3146152;
pub const D3D11_MESSAGE_ID_DESTROY_BLENDSTATE: D3D11_MESSAGE_ID = 2097272;
pub const D3D11_MESSAGE_ID_DESTROY_BUFFER: D3D11_MESSAGE_ID = 2097230;
pub const D3D11_MESSAGE_ID_DESTROY_CLASSINSTANCE: D3D11_MESSAGE_ID = 2097292;
pub const D3D11_MESSAGE_ID_DESTROY_CLASSLINKAGE: D3D11_MESSAGE_ID = 2097295;
pub const D3D11_MESSAGE_ID_DESTROY_COMMANDLIST: D3D11_MESSAGE_ID = 2097289;
pub const D3D11_MESSAGE_ID_DESTROY_COMPUTESHADER: D3D11_MESSAGE_ID = 2097300;
pub const D3D11_MESSAGE_ID_DESTROY_CONTEXT: D3D11_MESSAGE_ID = 2097227;
pub const D3D11_MESSAGE_ID_DESTROY_COUNTER: D3D11_MESSAGE_ID = 2097286;
pub const D3D11_MESSAGE_ID_DESTROY_CRYPTOSESSION: D3D11_MESSAGE_ID = 3146151;
pub const D3D11_MESSAGE_ID_DESTROY_DECODEROUTPUTVIEW: D3D11_MESSAGE_ID = 3145746;
pub const D3D11_MESSAGE_ID_DESTROY_DEPTHSTENCILSTATE: D3D11_MESSAGE_ID = 2097275;
pub const D3D11_MESSAGE_ID_DESTROY_DEPTHSTENCILVIEW: D3D11_MESSAGE_ID = 2097248;
pub const D3D11_MESSAGE_ID_DESTROY_DEVICECONTEXTSTATE: D3D11_MESSAGE_ID = 3145749;
pub const D3D11_MESSAGE_ID_DESTROY_DOMAINSHADER: D3D11_MESSAGE_ID = 2097257;
pub const D3D11_MESSAGE_ID_DESTROY_FENCE: D3D11_MESSAGE_ID = 3146252;
pub const D3D11_MESSAGE_ID_DESTROY_GEOMETRYSHADER: D3D11_MESSAGE_ID = 2097260;
pub const D3D11_MESSAGE_ID_DESTROY_HULLSHADER: D3D11_MESSAGE_ID = 2097254;
pub const D3D11_MESSAGE_ID_DESTROY_INPUTLAYOUT: D3D11_MESSAGE_ID = 2097266;
pub const D3D11_MESSAGE_ID_DESTROY_PIXELSHADER: D3D11_MESSAGE_ID = 2097263;
pub const D3D11_MESSAGE_ID_DESTROY_PREDICATE: D3D11_MESSAGE_ID = 2097284;
pub const D3D11_MESSAGE_ID_DESTROY_PROCESSORINPUTVIEW: D3D11_MESSAGE_ID = 3145747;
pub const D3D11_MESSAGE_ID_DESTROY_PROCESSOROUTPUTVIEW: D3D11_MESSAGE_ID = 3145748;
pub const D3D11_MESSAGE_ID_DESTROY_QUERY: D3D11_MESSAGE_ID = 2097281;
pub const D3D11_MESSAGE_ID_DESTROY_RASTERIZERSTATE: D3D11_MESSAGE_ID = 2097278;
pub const D3D11_MESSAGE_ID_DESTROY_RENDERTARGETVIEW: D3D11_MESSAGE_ID = 2097245;
pub const D3D11_MESSAGE_ID_DESTROY_SAMPLER: D3D11_MESSAGE_ID = 2097269;
pub const D3D11_MESSAGE_ID_DESTROY_SHADERRESOURCEVIEW: D3D11_MESSAGE_ID = 2097242;
pub const D3D11_MESSAGE_ID_DESTROY_SYNCHRONIZEDCHANNEL: D3D11_MESSAGE_ID = 3146255;
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE1D: D3D11_MESSAGE_ID = 2097233;
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE2D: D3D11_MESSAGE_ID = 2097236;
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE3D: D3D11_MESSAGE_ID = 2097239;
pub const D3D11_MESSAGE_ID_DESTROY_TRACKEDWORKLOAD: D3D11_MESSAGE_ID = 3146269;
pub const D3D11_MESSAGE_ID_DESTROY_UNORDEREDACCESSVIEW: D3D11_MESSAGE_ID = 2097303;
pub const D3D11_MESSAGE_ID_DESTROY_VERTEXSHADER: D3D11_MESSAGE_ID = 2097251;
pub const D3D11_MESSAGE_ID_DESTROY_VIDEODECODER: D3D11_MESSAGE_ID = 3145743;
pub const D3D11_MESSAGE_ID_DESTROY_VIDEOPROCESSOR: D3D11_MESSAGE_ID = 3145745;
pub const D3D11_MESSAGE_ID_DESTROY_VIDEOPROCESSORENUM: D3D11_MESSAGE_ID = 3145744;
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 2097315;
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_MISMATCHED_DATA_SIZE: D3D11_MESSAGE_ID = 2097314;
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_UNRECOGNIZED_FEATURE: D3D11_MESSAGE_ID = 2097313;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_EMPTYRECT: D3D11_MESSAGE_ID = 3146076;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDRECT: D3D11_MESSAGE_ID = 3146066;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDSOURCERECT: D3D11_MESSAGE_ID = 3146075;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDVIEW: D3D11_MESSAGE_ID = 3146035;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_NOTSUPPORTED: D3D11_MESSAGE_ID = 3146062;
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146048;
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = 2097338;
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146049;
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146059;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146040;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = 2097334;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146041;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146055;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146044;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = 2097336;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146045;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146057;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146042;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = 2097335;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146043;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146056;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146038;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = 2097333;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146039;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146054;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146046;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = 2097337;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146047;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146058;
pub const D3D11_MESSAGE_ID_DEVICE_CREATESHADER_CLASSLINKAGE_FULL: D3D11_MESSAGE_ID = 2097312;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146036;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = 2097332;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146037;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = 3146053;
pub const D3D11_MESSAGE_ID_DEVICE_CSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 2097330;
pub const D3D11_MESSAGE_ID_DEVICE_CSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 2097331;
pub const D3D11_MESSAGE_ID_DEVICE_CSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 2097329;
pub const D3D11_MESSAGE_ID_DEVICE_CSGETUNORDEREDACCESSS_VIEWS_EMPTY: D3D11_MESSAGE_ID = 2097357;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 2097327;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = 2097317;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 2097328;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = 2097316;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 2097325;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSS_VIEWS_EMPTY: D3D11_MESSAGE_ID = 2097356;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_HAZARD: D3D11_MESSAGE_ID = 2097354;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_INVALIDOFFSET: D3D11_MESSAGE_ID = 2097403;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_INVALIDVIEW: D3D11_MESSAGE_ID = 2097402;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_TOOMANYVIEWS: D3D11_MESSAGE_ID = 2097404;
pub const D3D11_MESSAGE_ID_DEVICE_DISCARDVIEW_INVALIDVIEW: D3D11_MESSAGE_ID = 3145754;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_INVALID_ARG_BUFFER: D3D11_MESSAGE_ID = 2097360;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_OFFSET_OVERFLOW: D3D11_MESSAGE_ID = 2097362;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = 2097361;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_UNSUPPORTED: D3D11_MESSAGE_ID = 2097396;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_BOUND_RESOURCE_MAPPED: D3D11_MESSAGE_ID = 2097389;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_THREADGROUPCOUNT_OVERFLOW: D3D11_MESSAGE_ID = 2097390;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_THREADGROUPCOUNT_ZERO: D3D11_MESSAGE_ID = 2097391;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_UNSUPPORTED: D3D11_MESSAGE_ID = 2097395;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INDEXPOS_OVERFLOW: D3D11_MESSAGE_ID = 340;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INSTANCEPOS_OVERFLOW: D3D11_MESSAGE_ID = 339;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXED_INDEXPOS_OVERFLOW: D3D11_MESSAGE_ID = 336;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_INVALID_ARG_BUFFER: D3D11_MESSAGE_ID = 2097207;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_OFFSET_OVERFLOW: D3D11_MESSAGE_ID = 2097209;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = 2097208;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINSTANCED_INSTANCEPOS_OVERFLOW: D3D11_MESSAGE_ID = 338;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINSTANCED_VERTEXPOS_OVERFLOW: D3D11_MESSAGE_ID = 337;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_BOUND_RESOURCE_MAPPED: D3D11_MESSAGE_ID = 364;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_NOT_SET: D3D11_MESSAGE_ID = 350;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_TOO_SMALL: D3D11_MESSAGE_ID = 351;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_DEPTHSTENCILVIEW_NOT_SET: D3D11_MESSAGE_ID = 3146080;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_GS_INPUT_PRIMITIVE_MISMATCH: D3D11_MESSAGE_ID = 360;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_CONTROL_POINT_COUNT_MISMATCH: D3D11_MESSAGE_ID = 2097223;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_SIGNATURE_MISMATCH: D3D11_MESSAGE_ID = 2097221;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_TESSELLATOR_DOMAIN_MISMATCH: D3D11_MESSAGE_ID = 2097224;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_XOR_DS_MISMATCH: D3D11_MESSAGE_ID = 2097205;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HULL_SHADER_INPUT_TOPOLOGY_MISMATCH: D3D11_MESSAGE_ID = 2097222;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_FORMAT_INVALID: D3D11_MESSAGE_ID = 358;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_NOT_SET: D3D11_MESSAGE_ID = 357;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_TOO_SMALL: D3D11_MESSAGE_ID = 359;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = 368;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INPUTLAYOUT_NOT_SET: D3D11_MESSAGE_ID = 349;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_PRIMITIVETOPOLOGY: D3D11_MESSAGE_ID = 365;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_SYSTEMVALUE: D3D11_MESSAGE_ID = 3146156;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: D3D11_MESSAGE_ID = 417;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_FORCED_SAMPLE_COUNT: D3D11_MESSAGE_ID = 3145940;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: D3D11_MESSAGE_ID = 377;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: D3D11_MESSAGE_ID = 376;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_LOGIC_OPS: D3D11_MESSAGE_ID = 3146079;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_NOT_SET: D3D11_MESSAGE_ID = 363;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = 369;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_PIXEL_SHADER_WITHOUT_RTV_OR_DSV: D3D11_MESSAGE_ID = 2097408;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_POSITION_NOT_PRESENT: D3D11_MESSAGE_ID = 362;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_PS_OUTPUT_TYPE_MISMATCH: D3D11_MESSAGE_ID = 415;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RASTERIZING_CONTROL_POINTS: D3D11_MESSAGE_ID = 2097219;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RENDERTARGETVIEW_NOT_SET: D3D11_MESSAGE_ID = 3146081;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RENDERTARGETVIEW_NOT_SET_DUE_TO_FLIP_PRESENT: D3D11_MESSAGE_ID = 3146082;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_AND_WRITE_MASK_MISMATCH: D3D11_MESSAGE_ID = 3146284;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_GATHER_UNSUPPORTED: D3D11_MESSAGE_ID = 416;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_LD_UNSUPPORTED: D3D11_MESSAGE_ID = 370;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_C_UNSUPPORTED: D3D11_MESSAGE_ID = 372;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_UNSUPPORTED: D3D11_MESSAGE_ID = 371;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_MULTISAMPLE_UNSUPPORTED: D3D11_MESSAGE_ID = 373;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_RETURN_TYPE_MISMATCH: D3D11_MESSAGE_ID = 361;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_SAMPLE_COUNT_MISMATCH: D3D11_MESSAGE_ID = 421;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLER_MISMATCH: D3D11_MESSAGE_ID = 390;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLER_NOT_SET: D3D11_MESSAGE_ID = 352;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLE_MASK_IGNORED_ON_FL9: D3D11_MESSAGE_ID = 3146067;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SHADERRESOURCEVIEW_NOT_SET: D3D11_MESSAGE_ID = 353;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SO_STRIDE_LARGER_THAN_BUFFER: D3D11_MESSAGE_ID = 375;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SO_TARGETS_BOUND_WITHOUT_SOURCE: D3D11_MESSAGE_ID = 374;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_UNORDEREDACCESSVIEW_RENDERTARGETVIEW_OVERLAP: D3D11_MESSAGE_ID = 2097374;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEXPOS_OVERFLOW: D3D11_MESSAGE_ID = 335;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_NOT_SET: D3D11_MESSAGE_ID = 348;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: D3D11_MESSAGE_ID = 355;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_TOO_SMALL: D3D11_MESSAGE_ID = 356;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = 366;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_SHADER_NOT_SET: D3D11_MESSAGE_ID = 341;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_STRIDE_UNALIGNED: D3D11_MESSAGE_ID = 367;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VIEWPORT_NOT_SET: D3D11_MESSAGE_ID = 384;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VIEW_DIMENSION_MISMATCH: D3D11_MESSAGE_ID = 354;
pub const D3D11_MESSAGE_ID_DEVICE_DSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 2097203;
pub const D3D11_MESSAGE_ID_DEVICE_DSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 2097204;
pub const D3D11_MESSAGE_ID_DEVICE_DSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 2097202;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 2097200;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = 2097190;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 2097201;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = 2097189;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 2097198;
pub const D3D11_MESSAGE_ID_DEVICE_GENERATEMIPS_RESOURCE_INVALID: D3D11_MESSAGE_ID = 277;
pub const D3D11_MESSAGE_ID_DEVICE_GETRESOURCEMINLOD_INVALIDCONTEXT: D3D11_MESSAGE_ID = 2097366;
pub const D3D11_MESSAGE_ID_DEVICE_GETRESOURCEMINLOD_INVALIDRESOURCE: D3D11_MESSAGE_ID = 2097367;
pub const D3D11_MESSAGE_ID_DEVICE_GSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 269;
pub const D3D11_MESSAGE_ID_DEVICE_GSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 270;
pub const D3D11_MESSAGE_ID_DEVICE_GSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 268;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 251;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = 6;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 252;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = 5;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 249;
pub const D3D11_MESSAGE_ID_DEVICE_HSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 2097187;
pub const D3D11_MESSAGE_ID_DEVICE_HSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 2097188;
pub const D3D11_MESSAGE_ID_DEVICE_HSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 2097186;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 2097184;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = 2097174;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 2097185;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = 2097173;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 2097182;
pub const D3D11_MESSAGE_ID_DEVICE_IAGETVERTEXBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 264;
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_FORMAT_INVALID: D3D11_MESSAGE_ID = 242;
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_HAZARD: D3D11_MESSAGE_ID = 2;
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_TOO_LARGE: D3D11_MESSAGE_ID = 243;
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = 244;
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_ADJACENCY_UNSUPPORTED: D3D11_MESSAGE_ID = 1048594;
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNDEFINED: D3D11_MESSAGE_ID = 237;
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNRECOGNIZED: D3D11_MESSAGE_ID = 236;
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNSUPPORTED: D3D11_MESSAGE_ID = 2097220;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 240;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_HAZARD: D3D11_MESSAGE_ID = 1;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_INVALIDRANGE: D3D11_MESSAGE_ID = 419;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_OFFSET_TOO_LARGE: D3D11_MESSAGE_ID = 239;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_STRIDE_TOO_LARGE: D3D11_MESSAGE_ID = 418;
pub const D3D11_MESSAGE_ID_DEVICE_LOCKEDOUT_INTERFACE: D3D11_MESSAGE_ID = 3145945;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_HAZARD: D3D11_MESSAGE_ID = 2097346;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_INVALIDOFFSET: D3D11_MESSAGE_ID = 3146060;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_NO_OP: D3D11_MESSAGE_ID = 2097348;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_NUMUAVS_INVALIDRANGE: D3D11_MESSAGE_ID = 2097422;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_OVERLAPPING_OLD_SLOTS: D3D11_MESSAGE_ID = 2097347;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_TOOMANYVIEWS: D3D11_MESSAGE_ID = 3146061;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETS_HAZARD: D3D11_MESSAGE_ID = 9;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE1_ACCESS_DENIED: D3D11_MESSAGE_ID = 3146117;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE1_NOT_SUPPORTED: D3D11_MESSAGE_ID = 3146068;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BADINTERFACE_RETURN: D3D11_MESSAGE_ID = 383;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BY_NAME_NOT_SUPPORTED: D3D11_MESSAGE_ID = 3146069;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 381;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 382;
pub const D3D11_MESSAGE_ID_DEVICE_PSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 273;
pub const D3D11_MESSAGE_ID_DEVICE_PSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 274;
pub const D3D11_MESSAGE_ID_DEVICE_PSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 272;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 257;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = 8;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 258;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = 7;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 255;
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: D3D11_MESSAGE_ID = 378;
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: D3D11_MESSAGE_ID = 380;
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: D3D11_MESSAGE_ID = 379;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_INVALID: D3D11_MESSAGE_ID = 290;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_SUBRESOURCE_INVALID: D3D11_MESSAGE_ID = 291;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_FORMAT_INVALID: D3D11_MESSAGE_ID = 294;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_INVALID: D3D11_MESSAGE_ID = 292;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_SUBRESOURCE_INVALID: D3D11_MESSAGE_ID = 293;
pub const D3D11_MESSAGE_ID_DEVICE_RSGETSCISSORRECTS_RECTS_EMPTY: D3D11_MESSAGE_ID = 276;
pub const D3D11_MESSAGE_ID_DEVICE_RSGETVIEWPORTS_VIEWPORTS_EMPTY: D3D11_MESSAGE_ID = 275;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_INVALIDSCISSOR: D3D11_MESSAGE_ID = 260;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_NEGATIVESCISSOR: D3D11_MESSAGE_ID = 1048632;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_TOO_MANY_SCISSORS: D3D11_MESSAGE_ID = 1048595;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_DENORMFLUSH: D3D11_MESSAGE_ID = 387;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_INVALIDVIEWPORT: D3D11_MESSAGE_ID = 259;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_TOO_MANY_VIEWPORTS: D3D11_MESSAGE_ID = 1048593;
pub const D3D11_MESSAGE_ID_DEVICE_SETHARDWAREPROTECTION_INVALIDCONTEXT: D3D11_MESSAGE_ID = 3146224;
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDCONTEXT: D3D11_MESSAGE_ID = 2097363;
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDMINLOD: D3D11_MESSAGE_ID = 2097365;
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDRESOURCE: D3D11_MESSAGE_ID = 2097364;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INSTANCE_DATA_BINDINGS: D3D11_MESSAGE_ID = 2097311;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INTERFACES_FEATURELEVEL: D3D11_MESSAGE_ID = 2097304;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INTERFACE_COUNT_MISMATCH: D3D11_MESSAGE_ID = 2097305;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE: D3D11_MESSAGE_ID = 2097306;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_DATA: D3D11_MESSAGE_ID = 2097309;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_INDEX: D3D11_MESSAGE_ID = 2097307;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_TYPE: D3D11_MESSAGE_ID = 2097308;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_UNBOUND_INSTANCE_DATA: D3D11_MESSAGE_ID = 2097310;
pub const D3D11_MESSAGE_ID_DEVICE_SETTEXTFILTERSIZE_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = 389;
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_BUFFER_TYPE_MISMATCH: D3D11_MESSAGE_ID = 2097393;
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_RAW_UNSUPPORTED: D3D11_MESSAGE_ID = 2097394;
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_STRUCTURE_STRIDE_MISMATCH: D3D11_MESSAGE_ID = 2097392;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_COMPONENTTYPE: D3D11_MESSAGE_ID = 344;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_MINPRECISION: D3D11_MESSAGE_ID = 3146050;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: D3D11_MESSAGE_ID = 347;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERINDEX: D3D11_MESSAGE_ID = 343;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERMASK: D3D11_MESSAGE_ID = 345;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: D3D11_MESSAGE_ID = 342;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SYSTEMVALUE: D3D11_MESSAGE_ID = 346;
pub const D3D11_MESSAGE_ID_DEVICE_SOGETTARGETS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 271;
pub const D3D11_MESSAGE_ID_DEVICE_SOSETTARGETS_HAZARD: D3D11_MESSAGE_ID = 10;
pub const D3D11_MESSAGE_ID_DEVICE_SOSETTARGETS_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = 254;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_APPEND_UNSUPPORTED: D3D11_MESSAGE_ID = 2097376;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMICS_UNSUPPORTED: D3D11_MESSAGE_ID = 2097377;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_ADD_UNSUPPORTED: D3D11_MESSAGE_ID = 2097383;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_BITWISE_OPS_UNSUPPORTED: D3D11_MESSAGE_ID = 2097384;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_CMPSTORE_CMPEXCHANGE_UNSUPPORTED: D3D11_MESSAGE_ID = 2097385;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_EXCHANGE_UNSUPPORTED: D3D11_MESSAGE_ID = 2097386;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_SIGNED_MINMAX_UNSUPPORTED: D3D11_MESSAGE_ID = 2097387;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_UNSIGNED_MINMAX_UNSUPPORTED: D3D11_MESSAGE_ID = 2097388;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_BUFFER_TYPE_MISMATCH: D3D11_MESSAGE_ID = 2097379;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_COUNTER_UNSUPPORTED: D3D11_MESSAGE_ID = 2097406;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_DIMENSION_MISMATCH: D3D11_MESSAGE_ID = 2097375;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_FORMAT_LD_UNSUPPORTED: D3D11_MESSAGE_ID = 2097381;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_FORMAT_STORE_UNSUPPORTED: D3D11_MESSAGE_ID = 2097382;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_NOT_SET: D3D11_MESSAGE_ID = 2097373;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_NOT_SET_DUE_TO_FLIP_PRESENT: D3D11_MESSAGE_ID = 3146083;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_RAW_UNSUPPORTED: D3D11_MESSAGE_ID = 2097380;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_RETURN_TYPE_MISMATCH: D3D11_MESSAGE_ID = 2097372;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_STRUCTURE_STRIDE_MISMATCH: D3D11_MESSAGE_ID = 2097378;
pub const D3D11_MESSAGE_ID_DEVICE_VSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 266;
pub const D3D11_MESSAGE_ID_DEVICE_VSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 267;
pub const D3D11_MESSAGE_ID_DEVICE_VSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 265;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = 247;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = 4;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = 248;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = 3;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = 245;
pub const D3D11_MESSAGE_ID_DIRTY_TILE_MAPPING_ACCESS: D3D11_MESSAGE_ID = 3146133;
pub const D3D11_MESSAGE_ID_DRAWINDEXEDINSTANCED_NOT_SUPPORTED_BELOW_9_3: D3D11_MESSAGE_ID = 1048627;
pub const D3D11_MESSAGE_ID_DRAWINDEXED_POINTLIST_UNSUPPORTED: D3D11_MESSAGE_ID = 1048628;
pub const D3D11_MESSAGE_ID_DRAWINDEXED_STARTINDEXLOCATION_MUST_BE_POSITIVE: D3D11_MESSAGE_ID = 1048602;
pub const D3D11_MESSAGE_ID_DRAWINSTANCED_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048626;
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = 2097199;
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = 3146017;
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097192;
pub const D3D11_MESSAGE_ID_DSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097414;
pub const D3D11_MESSAGE_ID_DSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097191;
pub const D3D11_MESSAGE_ID_DSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097417;
pub const D3D11_MESSAGE_ID_DUPLICATE_TILE_MAPPINGS_IN_COVERED_AREA: D3D11_MESSAGE_ID = 3146134;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_MAPPED: D3D11_MESSAGE_ID = 3145969;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_NOT_STAGING: D3D11_MESSAGE_ID = 3145967;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_OFFERED: D3D11_MESSAGE_ID = 3145971;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_WRONGDEVICE: D3D11_MESSAGE_ID = 3145963;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_FORMAT_MISMATCH: D3D11_MESSAGE_ID = 3145964;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_NULLPARAM: D3D11_MESSAGE_ID = 3145961;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SIZE_MISMATCH: D3D11_MESSAGE_ID = 3145965;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_CONTENT_UNDEFINED: D3D11_MESSAGE_ID = 3145972;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_MAPPED: D3D11_MESSAGE_ID = 3145968;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_MULTISAMPLED: D3D11_MESSAGE_ID = 3145966;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_OFFERED: D3D11_MESSAGE_ID = 3145970;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_WRONGDEVICE: D3D11_MESSAGE_ID = 3145962;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_UNSUPPORTED: D3D11_MESSAGE_ID = 3145960;
pub const D3D11_MESSAGE_ID_END_TRACKED_WORKLOAD_INVALID_ARG: D3D11_MESSAGE_ID = 3146282;
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_ACCESSDENIED_RETURN: D3D11_MESSAGE_ID = 2097421;
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 2097419;
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_NOT_SUPPORTED: D3D11_MESSAGE_ID = 3146070;
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 2097420;
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_INVALID_CALL_RETURN: D3D11_MESSAGE_ID = 2097168;
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_ONIMMEDIATECONTEXT: D3D11_MESSAGE_ID = 2097166;
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 2097167;
pub const D3D11_MESSAGE_ID_FINISHSESSIONKEYREFRESH_NULLPARAM: D3D11_MESSAGE_ID = 3145989;
pub const D3D11_MESSAGE_ID_FLUSH1_INVALIDCONTEXTTYPE: D3D11_MESSAGE_ID = 3146223;
pub const D3D11_MESSAGE_ID_GEOMETRY_SHADER_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048619;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATESIZE_INVALIDCHANNEL: D3D11_MESSAGE_ID = 3145999;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATESIZE_NULLPARAM: D3D11_MESSAGE_ID = 3146000;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_INVALIDCHANNEL: D3D11_MESSAGE_ID = 3146001;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_NULLPARAM: D3D11_MESSAGE_ID = 3146002;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_WRONGSIZE: D3D11_MESSAGE_ID = 3146003;
pub const D3D11_MESSAGE_ID_GETCONTENTPROTECTIONCAPS_NULLPARAM: D3D11_MESSAGE_ID = 3145992;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATESIZE_NULLPARAM: D3D11_MESSAGE_ID = 3145955;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATE_NULLPARAM: D3D11_MESSAGE_ID = 3145956;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATE_WRONGSIZE: D3D11_MESSAGE_ID = 3145957;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONHANDLE_OUTOFMEMORY: D3D11_MESSAGE_ID = 3146025;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONHANDLE_WRONGSIZE: D3D11_MESSAGE_ID = 3145958;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONPRIVATEDATASIZE_INVALID_KEY_EXCHANGE_TYPE: D3D11_MESSAGE_ID = 3146116;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONPRIVATEDATASIZE_NULLPARAM: D3D11_MESSAGE_ID = 3146086;
pub const D3D11_MESSAGE_ID_GETCRYPTOTYPE_NULLPARAM: D3D11_MESSAGE_ID = 3145953;
pub const D3D11_MESSAGE_ID_GETDATAFORNEWHARDWAREKEY_NULLPARAM: D3D11_MESSAGE_ID = 3146084;
pub const D3D11_MESSAGE_ID_GETDC_INACCESSIBLE: D3D11_MESSAGE_ID = 3146065;
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_INVALIDBUFFER: D3D11_MESSAGE_ID = 3145778;
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_INVALIDTYPE: D3D11_MESSAGE_ID = 3145779;
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_LOCKED: D3D11_MESSAGE_ID = 3145780;
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_NULLPARAM: D3D11_MESSAGE_ID = 3145777;
pub const D3D11_MESSAGE_ID_GETDECODERCREATIONPARAMS_NULLPARAM: D3D11_MESSAGE_ID = 3145775;
pub const D3D11_MESSAGE_ID_GETDECODERDRIVERHANDLE_NULLPARAM: D3D11_MESSAGE_ID = 3145776;
pub const D3D11_MESSAGE_ID_GETDECODERPROFILE_NULLPARAM: D3D11_MESSAGE_ID = 3145954;
pub const D3D11_MESSAGE_ID_GETENCRYPTIONBLTKEY_INVALIDSIZE: D3D11_MESSAGE_ID = 3145991;
pub const D3D11_MESSAGE_ID_GETENCRYPTIONBLTKEY_NULLPARAM: D3D11_MESSAGE_ID = 3145990;
pub const D3D11_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: D3D11_MESSAGE_ID = 51;
pub const D3D11_MESSAGE_ID_GETRESOURCETILING_NONTILED_RESOURCE: D3D11_MESSAGE_ID = 3146139;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCAPS_NULLPARAM: D3D11_MESSAGE_ID = 3146087;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCAPS_ZEROWIDTHHEIGHT: D3D11_MESSAGE_ID = 3146088;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIGCOUNT_NULLPARAM: D3D11_MESSAGE_ID = 3145770;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIGCOUNT_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145771;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_INVALIDINDEX: D3D11_MESSAGE_ID = 3145773;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_NULLPARAM: D3D11_MESSAGE_ID = 3145772;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145774;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILECOUNT_OUTOFMEMORY: D3D11_MESSAGE_ID = 3145764;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_INVALIDINDEX: D3D11_MESSAGE_ID = 3145766;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_NULLPARAM: D3D11_MESSAGE_ID = 3145765;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 3145767;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCAPS_NULLPARAM: D3D11_MESSAGE_ID = 3145800;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCONTENTDESC_NULLPARAM: D3D11_MESSAGE_ID = 3145798;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCUSTOMRATE_INVALIDINDEX: D3D11_MESSAGE_ID = 3145804;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCUSTOMRATE_NULLPARAM: D3D11_MESSAGE_ID = 3145803;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORFILTERRANGE_NULLPARAM: D3D11_MESSAGE_ID = 3145805;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORFILTERRANGE_UNSUPPORTED: D3D11_MESSAGE_ID = 3145806;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORRATECONVERSIONCAPS_INVALIDINDEX: D3D11_MESSAGE_ID = 3145802;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORRATECONVERSIONCAPS_NULLPARAM: D3D11_MESSAGE_ID = 3145801;
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = 250;
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = 3146019;
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 39;
pub const D3D11_MESSAGE_ID_GSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 40;
pub const D3D11_MESSAGE_ID_GSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 38;
pub const D3D11_MESSAGE_ID_GSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 37;
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = 2097183;
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = 3146018;
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097176;
pub const D3D11_MESSAGE_ID_HSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097413;
pub const D3D11_MESSAGE_ID_HSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097175;
pub const D3D11_MESSAGE_ID_HSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097416;
pub const D3D11_MESSAGE_ID_IASETINDEXBUFFER_INVALIDBUFFER: D3D11_MESSAGE_ID = 241;
pub const D3D11_MESSAGE_ID_IASETINDEXBUFFER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 32;
pub const D3D11_MESSAGE_ID_IASETINPUTLAYOUT_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 30;
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_BAD_BUFFER_INDEX: D3D11_MESSAGE_ID = 1048592;
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = 238;
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 31;
pub const D3D11_MESSAGE_ID_INCOMPLETE_TRACKED_WORKLOAD_PAIR: D3D11_MESSAGE_ID = 3146276;
pub const D3D11_MESSAGE_ID_JPEGDECODE_1DESTUNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = 3146194;
pub const D3D11_MESSAGE_ID_JPEGDECODE_3DESTUNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = 3146195;
pub const D3D11_MESSAGE_ID_JPEGDECODE_BACKBUFFERNOTSUPPORTED: D3D11_MESSAGE_ID = 3146203;
pub const D3D11_MESSAGE_ID_JPEGDECODE_CHROMASIZEMISMATCH: D3D11_MESSAGE_ID = 3146190;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXESINTERSECT: D3D11_MESSAGE_ID = 3146182;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXNOT2D: D3D11_MESSAGE_ID = 3146180;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXNOTSUB: D3D11_MESSAGE_ID = 3146181;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTINATIONNOT2D: D3D11_MESSAGE_ID = 3146173;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DIMENSIONSTOOLARGE: D3D11_MESSAGE_ID = 3146171;
pub const D3D11_MESSAGE_ID_JPEGDECODE_EMPTYDESTBOX: D3D11_MESSAGE_ID = 3146179;
pub const D3D11_MESSAGE_ID_JPEGDECODE_FORMATUNSUPPORTED: D3D11_MESSAGE_ID = 3146176;
pub const D3D11_MESSAGE_ID_JPEGDECODE_FRACTIONALDOWNSCALETOLARGE: D3D11_MESSAGE_ID = 3146189;
pub const D3D11_MESSAGE_ID_JPEGDECODE_GUARDRECTSUNSUPPORTED: D3D11_MESSAGE_ID = 3146175;
pub const D3D11_MESSAGE_ID_JPEGDECODE_HAZARD: D3D11_MESSAGE_ID = 3146199;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDCOMPONENTS: D3D11_MESSAGE_ID = 3146172;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDCOPYFLAGS: D3D11_MESSAGE_ID = 3146198;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDMIPLEVEL: D3D11_MESSAGE_ID = 3146178;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDNUMDESTINATIONS: D3D11_MESSAGE_ID = 3146192;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSCANDATAOFFSET: D3D11_MESSAGE_ID = 3146169;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSOURCESIZE: D3D11_MESSAGE_ID = 3146197;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 3146177;
pub const D3D11_MESSAGE_ID_JPEGDECODE_LUMACHROMASIZEMISMATCH: D3D11_MESSAGE_ID = 3146191;
pub const D3D11_MESSAGE_ID_JPEGDECODE_NONPOW2SCALEUNSUPPORTED: D3D11_MESSAGE_ID = 3146188;
pub const D3D11_MESSAGE_ID_JPEGDECODE_NOTSUPPORTED: D3D11_MESSAGE_ID = 3146170;
pub const D3D11_MESSAGE_ID_JPEGDECODE_OUTPUTDIMENSIONSTOOLARGE: D3D11_MESSAGE_ID = 3146187;
pub const D3D11_MESSAGE_ID_JPEGDECODE_SCALEUNSUPPORTED: D3D11_MESSAGE_ID = 3146196;
pub const D3D11_MESSAGE_ID_JPEGDECODE_SUBBOXUNSUPPORTED: D3D11_MESSAGE_ID = 3146193;
pub const D3D11_MESSAGE_ID_JPEGDECODE_TILEDRESOURCESUNSUPPORTED: D3D11_MESSAGE_ID = 3146174;
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDDSTTEXTUREUSAGE: D3D11_MESSAGE_ID = 3146202;
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDSRCBUFFERMISCFLAGS: D3D11_MESSAGE_ID = 3146201;
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDSRCBUFFERUSAGE: D3D11_MESSAGE_ID = 3146200;
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPRTEDCOPYFLAGS: D3D11_MESSAGE_ID = 3146204;
pub const D3D11_MESSAGE_ID_JPEGDECODE_XSUBSAMPLEMISMATCH: D3D11_MESSAGE_ID = 3146183;
pub const D3D11_MESSAGE_ID_JPEGDECODE_XSUBSAMPLEODD: D3D11_MESSAGE_ID = 3146185;
pub const D3D11_MESSAGE_ID_JPEGDECODE_YSUBSAMPLEMISMATCH: D3D11_MESSAGE_ID = 3146184;
pub const D3D11_MESSAGE_ID_JPEGDECODE_YSUBSAMPLEODD: D3D11_MESSAGE_ID = 3146186;
pub const D3D11_MESSAGE_ID_JPEGENCODE_BACKBUFFERNOTSUPPORTED: D3D11_MESSAGE_ID = 3146221;
pub const D3D11_MESSAGE_ID_JPEGENCODE_DIMENSIONSTOOLARGE: D3D11_MESSAGE_ID = 3146216;
pub const D3D11_MESSAGE_ID_JPEGENCODE_FORMATUNSUPPORTED: D3D11_MESSAGE_ID = 3146213;
pub const D3D11_MESSAGE_ID_JPEGENCODE_GUARDRECTSUNSUPPORTED: D3D11_MESSAGE_ID = 3146210;
pub const D3D11_MESSAGE_ID_JPEGENCODE_HAZARD: D3D11_MESSAGE_ID = 3146217;
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDCOMPONENTS: D3D11_MESSAGE_ID = 3146207;
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDMIPLEVEL: D3D11_MESSAGE_ID = 3146215;
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDSCANDATAOFFSET: D3D11_MESSAGE_ID = 3146206;
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 3146214;
pub const D3D11_MESSAGE_ID_JPEGENCODE_NOTSUPPORTED: D3D11_MESSAGE_ID = 3146205;
pub const D3D11_MESSAGE_ID_JPEGENCODE_SOURCENOT2D: D3D11_MESSAGE_ID = 3146208;
pub const D3D11_MESSAGE_ID_JPEGENCODE_TILEDRESOURCESUNSUPPORTED: D3D11_MESSAGE_ID = 3146209;
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDDSTBUFFERMISCFLAGS: D3D11_MESSAGE_ID = 3146219;
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDDSTBUFFERUSAGE: D3D11_MESSAGE_ID = 3146218;
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDSRCTEXTUREUSAGE: D3D11_MESSAGE_ID = 3146220;
pub const D3D11_MESSAGE_ID_JPEGENCODE_XSUBSAMPLEMISMATCH: D3D11_MESSAGE_ID = 3146211;
pub const D3D11_MESSAGE_ID_JPEGENCODE_YSUBSAMPLEMISMATCH: D3D11_MESSAGE_ID = 3146212;
pub const D3D11_MESSAGE_ID_LIVE_AUTHENTICATEDCHANNEL: D3D11_MESSAGE_ID = 3146150;
pub const D3D11_MESSAGE_ID_LIVE_BLENDSTATE: D3D11_MESSAGE_ID = 435;
pub const D3D11_MESSAGE_ID_LIVE_BLENDSTATE_WIN7: D3D11_MESSAGE_ID = 2097271;
pub const D3D11_MESSAGE_ID_LIVE_BUFFER: D3D11_MESSAGE_ID = 423;
pub const D3D11_MESSAGE_ID_LIVE_BUFFER_WIN7: D3D11_MESSAGE_ID = 2097229;
pub const D3D11_MESSAGE_ID_LIVE_CLASSINSTANCE: D3D11_MESSAGE_ID = 2097291;
pub const D3D11_MESSAGE_ID_LIVE_CLASSLINKAGE: D3D11_MESSAGE_ID = 2097294;
pub const D3D11_MESSAGE_ID_LIVE_COMMANDLIST: D3D11_MESSAGE_ID = 2097288;
pub const D3D11_MESSAGE_ID_LIVE_COMPUTESHADER: D3D11_MESSAGE_ID = 2097299;
pub const D3D11_MESSAGE_ID_LIVE_CONTEXT: D3D11_MESSAGE_ID = 2097226;
pub const D3D11_MESSAGE_ID_LIVE_COUNTER: D3D11_MESSAGE_ID = 440;
pub const D3D11_MESSAGE_ID_LIVE_CRYPTOSESSION: D3D11_MESSAGE_ID = 3146149;
pub const D3D11_MESSAGE_ID_LIVE_DECODEROUTPUTVIEW: D3D11_MESSAGE_ID = 3145739;
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE: D3D11_MESSAGE_ID = 436;
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE_WIN7: D3D11_MESSAGE_ID = 2097274;
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW: D3D11_MESSAGE_ID = 429;
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW_WIN7: D3D11_MESSAGE_ID = 2097247;
pub const D3D11_MESSAGE_ID_LIVE_DEVICE: D3D11_MESSAGE_ID = 441;
pub const D3D11_MESSAGE_ID_LIVE_DEVICECONTEXTSTATE: D3D11_MESSAGE_ID = 3145742;
pub const D3D11_MESSAGE_ID_LIVE_DEVICE_WIN7: D3D11_MESSAGE_ID = 2097296;
pub const D3D11_MESSAGE_ID_LIVE_DOMAINSHADER: D3D11_MESSAGE_ID = 2097256;
pub const D3D11_MESSAGE_ID_LIVE_FENCE: D3D11_MESSAGE_ID = 3146251;
pub const D3D11_MESSAGE_ID_LIVE_GEOMETRYSHADER: D3D11_MESSAGE_ID = 431;
pub const D3D11_MESSAGE_ID_LIVE_GEOMETRYSHADER_WIN7: D3D11_MESSAGE_ID = 2097259;
pub const D3D11_MESSAGE_ID_LIVE_HULLSHADER: D3D11_MESSAGE_ID = 2097253;
pub const D3D11_MESSAGE_ID_LIVE_INPUTLAYOUT: D3D11_MESSAGE_ID = 433;
pub const D3D11_MESSAGE_ID_LIVE_INPUTLAYOUT_WIN7: D3D11_MESSAGE_ID = 2097265;
pub const D3D11_MESSAGE_ID_LIVE_OBJECT_SUMMARY: D3D11_MESSAGE_ID = 422;
pub const D3D11_MESSAGE_ID_LIVE_OBJECT_SUMMARY_WIN7: D3D11_MESSAGE_ID = 2097297;
pub const D3D11_MESSAGE_ID_LIVE_PIXELSHADER: D3D11_MESSAGE_ID = 432;
pub const D3D11_MESSAGE_ID_LIVE_PIXELSHADER_WIN7: D3D11_MESSAGE_ID = 2097262;
pub const D3D11_MESSAGE_ID_LIVE_PREDICATE: D3D11_MESSAGE_ID = 439;
pub const D3D11_MESSAGE_ID_LIVE_PREDICATE_WIN7: D3D11_MESSAGE_ID = 2097283;
pub const D3D11_MESSAGE_ID_LIVE_PROCESSORINPUTVIEW: D3D11_MESSAGE_ID = 3145740;
pub const D3D11_MESSAGE_ID_LIVE_PROCESSOROUTPUTVIEW: D3D11_MESSAGE_ID = 3145741;
pub const D3D11_MESSAGE_ID_LIVE_QUERY: D3D11_MESSAGE_ID = 438;
pub const D3D11_MESSAGE_ID_LIVE_QUERY_WIN7: D3D11_MESSAGE_ID = 2097280;
pub const D3D11_MESSAGE_ID_LIVE_RASTERIZERSTATE: D3D11_MESSAGE_ID = 437;
pub const D3D11_MESSAGE_ID_LIVE_RASTERIZERSTATE_WIN7: D3D11_MESSAGE_ID = 2097277;
pub const D3D11_MESSAGE_ID_LIVE_RENDERTARGETVIEW: D3D11_MESSAGE_ID = 428;
pub const D3D11_MESSAGE_ID_LIVE_RENDERTARGETVIEW_WIN7: D3D11_MESSAGE_ID = 2097244;
pub const D3D11_MESSAGE_ID_LIVE_SAMPLER: D3D11_MESSAGE_ID = 434;
pub const D3D11_MESSAGE_ID_LIVE_SAMPLER_WIN7: D3D11_MESSAGE_ID = 2097268;
pub const D3D11_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW: D3D11_MESSAGE_ID = 427;
pub const D3D11_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW_WIN7: D3D11_MESSAGE_ID = 2097241;
pub const D3D11_MESSAGE_ID_LIVE_SWAPCHAIN: D3D11_MESSAGE_ID = 442;
pub const D3D11_MESSAGE_ID_LIVE_SYNCHRONIZEDCHANNEL: D3D11_MESSAGE_ID = 3146254;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE1D: D3D11_MESSAGE_ID = 424;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE1D_WIN7: D3D11_MESSAGE_ID = 2097232;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE2D: D3D11_MESSAGE_ID = 425;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE2D_WIN7: D3D11_MESSAGE_ID = 2097235;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE3D: D3D11_MESSAGE_ID = 426;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE3D_WIN7: D3D11_MESSAGE_ID = 2097238;
pub const D3D11_MESSAGE_ID_LIVE_TRACKEDWORKLOAD: D3D11_MESSAGE_ID = 3146268;
pub const D3D11_MESSAGE_ID_LIVE_UNORDEREDACCESSVIEW: D3D11_MESSAGE_ID = 2097302;
pub const D3D11_MESSAGE_ID_LIVE_VERTEXSHADER: D3D11_MESSAGE_ID = 430;
pub const D3D11_MESSAGE_ID_LIVE_VERTEXSHADER_WIN7: D3D11_MESSAGE_ID = 2097250;
pub const D3D11_MESSAGE_ID_LIVE_VIDEODECODER: D3D11_MESSAGE_ID = 3145736;
pub const D3D11_MESSAGE_ID_LIVE_VIDEOPROCESSOR: D3D11_MESSAGE_ID = 3145738;
pub const D3D11_MESSAGE_ID_LIVE_VIDEOPROCESSORENUM: D3D11_MESSAGE_ID = 3145737;
pub const D3D11_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: D3D11_MESSAGE_ID = 29;
pub const D3D11_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOADS: D3D11_MESSAGE_ID = 3146274;
pub const D3D11_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOAD_PAIRS: D3D11_MESSAGE_ID = 3146275;
pub const D3D11_MESSAGE_ID_NEED_TO_CALL_TILEDRESOURCEBARRIER: D3D11_MESSAGE_ID = 3146141;
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_INVALIDCHANNEL: D3D11_MESSAGE_ID = 3146004;
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_INVALIDSIZE: D3D11_MESSAGE_ID = 3146023;
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_NULLPARAM: D3D11_MESSAGE_ID = 3146005;
pub const D3D11_MESSAGE_ID_NEGOTIATECRPYTOSESSIONKEYEXCHANGE_INVALIDSIZE: D3D11_MESSAGE_ID = 3146022;
pub const D3D11_MESSAGE_ID_NEGOTIATECRPYTOSESSIONKEYEXCHANGE_NULLPARAM: D3D11_MESSAGE_ID = 3145959;
pub const D3D11_MESSAGE_ID_NEGOTIATECRYPTOSESSIONKEYEXCHANGEMT_INVALIDKEYEXCHANGETYPE: D3D11_MESSAGE_ID = 3146259;
pub const D3D11_MESSAGE_ID_NEGOTIATECRYPTOSESSIONKEYEXCHANGEMT_NOT_SUPPORTED: D3D11_MESSAGE_ID = 3146260;
pub const D3D11_MESSAGE_ID_NO_TRACKED_WORKLOAD_SLOT_AVAILABLE: D3D11_MESSAGE_ID = 3146281;
pub const D3D11_MESSAGE_ID_NULL_TILE_MAPPING_ACCESS_ERROR: D3D11_MESSAGE_ID = 3146132;
pub const D3D11_MESSAGE_ID_NULL_TILE_MAPPING_ACCESS_WARNING: D3D11_MESSAGE_ID = 3146131;
pub const D3D11_MESSAGE_ID_OFFERRELEASE_NOT_SUPPORTED: D3D11_MESSAGE_ID = 3146071;
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INACCESSIBLE: D3D11_MESSAGE_ID = 3146072;
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INVALIDPRIORITY: D3D11_MESSAGE_ID = 3146024;
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INVALIDRESOURCE: D3D11_MESSAGE_ID = 2097412;
pub const D3D11_MESSAGE_ID_OMSETBLENDSTATE_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 47;
pub const D3D11_MESSAGE_ID_OMSETDEPTHSTENCILSTATE_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 48;
pub const D3D11_MESSAGE_ID_OMSETDEPTHSTENCIL_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097368;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_INVALIDVIEW: D3D11_MESSAGE_ID = 388;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_NO_DIFFERING_BIT_DEPTHS: D3D11_MESSAGE_ID = 1048591;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_NO_SRGB_MRT: D3D11_MESSAGE_ID = 1048636;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_TOO_MANY_RENDER_TARGETS: D3D11_MESSAGE_ID = 1048590;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 49;
pub const D3D11_MESSAGE_ID_OUT_OF_ORDER_TRACKED_WORKLOAD_PAIR: D3D11_MESSAGE_ID = 3146277;
pub const D3D11_MESSAGE_ID_PREDICATE_BEGIN_DURING_PREDICATION: D3D11_MESSAGE_ID = 406;
pub const D3D11_MESSAGE_ID_PREDICATE_END_DURING_PREDICATION: D3D11_MESSAGE_ID = 409;
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = 256;
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = 3146020;
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 44;
pub const D3D11_MESSAGE_ID_PSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D11_MESSAGE_ID = 1048584;
pub const D3D11_MESSAGE_ID_PSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 45;
pub const D3D11_MESSAGE_ID_PSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 43;
pub const D3D11_MESSAGE_ID_PSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 42;
pub const D3D11_MESSAGE_ID_PSSETUNORDEREDACCESSVIEWS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 2097350;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_INVALIDPROCESSINDEX: D3D11_MESSAGE_ID = 3146010;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_NULLPARAM: D3D11_MESSAGE_ID = 3146006;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_UNSUPPORTEDQUERY: D3D11_MESSAGE_ID = 3146008;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_WRONGCHANNEL: D3D11_MESSAGE_ID = 3146007;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_WRONGSIZE: D3D11_MESSAGE_ID = 3146009;
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_ABANDONING_PREVIOUS_RESULTS: D3D11_MESSAGE_ID = 408;
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_DUPLICATE: D3D11_MESSAGE_ID = 407;
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_UNSUPPORTED: D3D11_MESSAGE_ID = 405;
pub const D3D11_MESSAGE_ID_QUERY_END_ABANDONING_PREVIOUS_RESULTS: D3D11_MESSAGE_ID = 410;
pub const D3D11_MESSAGE_ID_QUERY_END_WITHOUT_BEGIN: D3D11_MESSAGE_ID = 411;
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_CALL: D3D11_MESSAGE_ID = 414;
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_DATASIZE: D3D11_MESSAGE_ID = 412;
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_FLAGS: D3D11_MESSAGE_ID = 413;
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_INVALIDCOLORSPACE: D3D11_MESSAGE_ID = 3146107;
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_NULLPARAM: D3D11_MESSAGE_ID = 3146106;
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_ZEROWIDTHHEIGHT: D3D11_MESSAGE_ID = 3146108;
pub const D3D11_MESSAGE_ID_REF_ACCESSING_INDEXABLE_TEMP_OUT_OF_RANGE: D3D11_MESSAGE_ID = 331;
pub const D3D11_MESSAGE_ID_REF_HARDWARE_EXCEPTION: D3D11_MESSAGE_ID = 330;
pub const D3D11_MESSAGE_ID_REF_INFO: D3D11_MESSAGE_ID = 334;
pub const D3D11_MESSAGE_ID_REF_KMDRIVER_EXCEPTION: D3D11_MESSAGE_ID = 329;
pub const D3D11_MESSAGE_ID_REF_OUT_OF_MEMORY: D3D11_MESSAGE_ID = 333;
pub const D3D11_MESSAGE_ID_REF_PROBLEM_PARSING_SHADER: D3D11_MESSAGE_ID = 332;
pub const D3D11_MESSAGE_ID_REF_SIMULATING_INFINITELY_FAST_HARDWARE: D3D11_MESSAGE_ID = 326;
pub const D3D11_MESSAGE_ID_REF_THREADING_MODE: D3D11_MESSAGE_ID = 327;
pub const D3D11_MESSAGE_ID_REF_UMDRIVER_EXCEPTION: D3D11_MESSAGE_ID = 328;
pub const D3D11_MESSAGE_ID_REF_WARNING: D3D11_MESSAGE_ID = 2097407;
pub const D3D11_MESSAGE_ID_REF_WARNING_ATOMIC_INCONSISTENT: D3D11_MESSAGE_ID = 3145946;
pub const D3D11_MESSAGE_ID_REF_WARNING_RAW_HAZARD: D3D11_MESSAGE_ID = 3145948;
pub const D3D11_MESSAGE_ID_REF_WARNING_READING_UNINITIALIZED_RESOURCE: D3D11_MESSAGE_ID = 3145947;
pub const D3D11_MESSAGE_ID_REF_WARNING_WAR_HAZARD: D3D11_MESSAGE_ID = 3145949;
pub const D3D11_MESSAGE_ID_REF_WARNING_WAW_HAZARD: D3D11_MESSAGE_ID = 3145950;
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_INVALIDTYPE: D3D11_MESSAGE_ID = 3145782;
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_NOTLOCKED: D3D11_MESSAGE_ID = 3145783;
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_NULLPARAM: D3D11_MESSAGE_ID = 3145781;
pub const D3D11_MESSAGE_ID_RESIZETILEPOOL_INVALID_PARAMETER: D3D11_MESSAGE_ID = 3146129;
pub const D3D11_MESSAGE_ID_RESIZETILEPOOL_SHRINK_WITH_MAPPINGS_STILL_DEFINED_PAST_END: D3D11_MESSAGE_ID = 3146140;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = 2097213;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = 2097214;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = 2097212;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = 2097210;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 2097211;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = 2097215;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_WITHOUT_INITIAL_DISCARD: D3D11_MESSAGE_ID = 2097216;
pub const D3D11_MESSAGE_ID_RESOURCE_UNMAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 2097217;
pub const D3D11_MESSAGE_ID_RESOURCE_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = 2097218;
pub const D3D11_MESSAGE_ID_RSSETSTATE_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 46;
pub const D3D11_MESSAGE_ID_SETBLENDSTATE_SAMPLE_MASK_CANNOT_BE_ZERO: D3D11_MESSAGE_ID = 1048629;
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = 325;
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_INVALIDARG_RETURN: D3D11_MESSAGE_ID = 324;
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_UNRECOGNIZEDFLAGS: D3D11_MESSAGE_ID = 323;
pub const D3D11_MESSAGE_ID_SETPREDICATION_INVALID_PREDICATE_STATE: D3D11_MESSAGE_ID = 404;
pub const D3D11_MESSAGE_ID_SETPREDICATION_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 50;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: D3D11_MESSAGE_ID = 55;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDFLAGS: D3D11_MESSAGE_ID = 54;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: D3D11_MESSAGE_ID = 52;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDIUNKNOWN: D3D11_MESSAGE_ID = 53;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: D3D11_MESSAGE_ID = 56;
pub const D3D11_MESSAGE_ID_SHADERRESOURCEVIEW_GETDESC_LEGACY: D3D11_MESSAGE_ID = 393;
pub const D3D11_MESSAGE_ID_SHADER_ABORT: D3D11_MESSAGE_ID = 2097409;
pub const D3D11_MESSAGE_ID_SHADER_ERROR: D3D11_MESSAGE_ID = 2097411;
pub const D3D11_MESSAGE_ID_SHADER_MESSAGE: D3D11_MESSAGE_ID = 2097410;
pub const D3D11_MESSAGE_ID_SLOT_ZERO_MUST_BE_D3D10_INPUT_PER_VERTEX_DATA: D3D11_MESSAGE_ID = 1048633;
pub const D3D11_MESSAGE_ID_SOSETTARGETS_INVALIDBUFFER: D3D11_MESSAGE_ID = 253;
pub const D3D11_MESSAGE_ID_SOSETTARGETS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 41;
pub const D3D11_MESSAGE_ID_STARTSESSIONKEYREFRESH_INVALIDSIZE: D3D11_MESSAGE_ID = 3145988;
pub const D3D11_MESSAGE_ID_STARTSESSIONKEYREFRESH_NULLPARAM: D3D11_MESSAGE_ID = 3145987;
pub const D3D11_MESSAGE_ID_STREAM_OUT_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048620;
pub const D3D11_MESSAGE_ID_STRING_FROM_APPLICATION: D3D11_MESSAGE_ID = 11;
pub const D3D11_MESSAGE_ID_SUBMITDECODERBUFFERS_INVALIDTYPE: D3D11_MESSAGE_ID = 3145788;
pub const D3D11_MESSAGE_ID_SUBMITDECODERBUFFERS_NULLPARAM: D3D11_MESSAGE_ID = 3145787;
pub const D3D11_MESSAGE_ID_SWAPDEVICECONTEXTSTATE_NOTSUPPORTED: D3D11_MESSAGE_ID = 3146063;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = 303;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = 304;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = 302;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = 300;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 301;
pub const D3D11_MESSAGE_ID_TEXTURE1D_UNMAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 305;
pub const D3D11_MESSAGE_ID_TEXTURE1D_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = 306;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = 310;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = 311;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = 309;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = 307;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 308;
pub const D3D11_MESSAGE_ID_TEXTURE2D_UNMAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 312;
pub const D3D11_MESSAGE_ID_TEXTURE2D_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = 313;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = 317;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = 318;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = 316;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = 314;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 315;
pub const D3D11_MESSAGE_ID_TEXTURE3D_UNMAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = 319;
pub const D3D11_MESSAGE_ID_TEXTURE3D_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = 320;
pub const D3D11_MESSAGE_ID_TEXT_FILTER_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048621;
pub const D3D11_MESSAGE_ID_TILEDRESOURCEBARRIER_INVALID_PARAMETER: D3D11_MESSAGE_ID = 3146130;
pub const D3D11_MESSAGE_ID_TILED_RESOURCE_TIER_1_BUFFER_TEXTURE_MISMATCH: D3D11_MESSAGE_ID = 3146146;
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_IN_COVERED_AREA_DUPLICATED_OUTSIDE: D3D11_MESSAGE_ID = 3146135;
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_SHARED_BETWEEN_INCOMPATIBLE_RESOURCES: D3D11_MESSAGE_ID = 3146136;
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_SHARED_BETWEEN_INPUT_AND_OUTPUT: D3D11_MESSAGE_ID = 3146137;
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_DISJOINT_FAILURE: D3D11_MESSAGE_ID = 3146283;
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_ENGINE_TYPE_NOT_FOUND: D3D11_MESSAGE_ID = 3146280;
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_NOT_SUPPORTED: D3D11_MESSAGE_ID = 3146279;
pub const D3D11_MESSAGE_ID_UNKNOWN: D3D11_MESSAGE_ID = 0;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE1_INVALIDCOPYFLAGS: D3D11_MESSAGE_ID = 3145756;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_EMPTYDESTBOX: D3D11_MESSAGE_ID = 3146077;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONBOX: D3D11_MESSAGE_ID = 288;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSTATE: D3D11_MESSAGE_ID = 289;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSUBRESOURCE: D3D11_MESSAGE_ID = 287;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_PREFERUPDATESUBRESOURCE1: D3D11_MESSAGE_ID = 3146064;
pub const D3D11_MESSAGE_ID_UPDATETILEMAPPINGS_INVALID_PARAMETER: D3D11_MESSAGE_ID = 3146125;
pub const D3D11_MESSAGE_ID_UPDATETILES_INVALID_PARAMETER: D3D11_MESSAGE_ID = 3146128;
pub const D3D11_MESSAGE_ID_USE_OF_ZERO_REFCOUNT_OBJECT: D3D11_MESSAGE_ID = 2097423;
pub const D3D11_MESSAGE_ID_VIDEODECODERENABLEDOWNSAMPLING_NULLPARAM: D3D11_MESSAGE_ID = 3146092;
pub const D3D11_MESSAGE_ID_VIDEODECODERENABLEDOWNSAMPLING_UNSUPPORTED: D3D11_MESSAGE_ID = 3146093;
pub const D3D11_MESSAGE_ID_VIDEODECODERUPDATEDOWNSAMPLING_NULLPARAM: D3D11_MESSAGE_ID = 3146094;
pub const D3D11_MESSAGE_ID_VIDEODECODERUPDATEDOWNSAMPLING_UNSUPPORTED: D3D11_MESSAGE_ID = 3146095;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INPUTHAZARD: D3D11_MESSAGE_ID = 3145905;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDARRAY: D3D11_MESSAGE_ID = 3145899;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDARRAYSIZE: D3D11_MESSAGE_ID = 3145898;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDDESTRECT: D3D11_MESSAGE_ID = 3145896;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDFUTUREFRAMES: D3D11_MESSAGE_ID = 3145894;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDINPUTRESOURCE: D3D11_MESSAGE_ID = 3145897;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDOUTPUT: D3D11_MESSAGE_ID = 3145892;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDPASTFRAMES: D3D11_MESSAGE_ID = 3145893;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDRIGHTRESOURCE: D3D11_MESSAGE_ID = 3145903;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDSOURCERECT: D3D11_MESSAGE_ID = 3145895;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDSTREAMCOUNT: D3D11_MESSAGE_ID = 3145890;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_NOSTEREOSTREAMS: D3D11_MESSAGE_ID = 3145904;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_NULLPARAM: D3D11_MESSAGE_ID = 3145889;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_OUTPUTHAZARD: D3D11_MESSAGE_ID = 3145906;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_RIGHTEXPECTED: D3D11_MESSAGE_ID = 3145900;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_RIGHTNOTEXPECTED: D3D11_MESSAGE_ID = 3145901;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_STEREONOTENABLED: D3D11_MESSAGE_ID = 3145902;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_TARGETRECT: D3D11_MESSAGE_ID = 3145891;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDDESTRECT: D3D11_MESSAGE_ID = 3146115;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDSOURCERECT: D3D11_MESSAGE_ID = 3146114;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDSTREAMCOUNT: D3D11_MESSAGE_ID = 3146112;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_NULLPARAM: D3D11_MESSAGE_ID = 3146111;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_TARGETRECT: D3D11_MESSAGE_ID = 3146113;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTALPHAFILLMODE_NULLPARAM: D3D11_MESSAGE_ID = 3145824;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTBACKGROUNDCOLOR_NULLPARAM: D3D11_MESSAGE_ID = 3145822;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCOLORSPACE1_NULLPARAM: D3D11_MESSAGE_ID = 3146098;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCOLORSPACE_NULLPARAM: D3D11_MESSAGE_ID = 3145823;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCONSTRICTION_NULLPARAM: D3D11_MESSAGE_ID = 3145825;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTEXTENSION_NULLPARAM: D3D11_MESSAGE_ID = 3145829;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTHDRMETADATA_INVALIDSIZE: D3D11_MESSAGE_ID = 3146228;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTHDRMETADATA_NULLPARAM: D3D11_MESSAGE_ID = 3146227;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTSHADERUSAGE_NULLPARAM: D3D11_MESSAGE_ID = 3146110;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTSTEREOMODE_NULLPARAM: D3D11_MESSAGE_ID = 3145828;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTTARGETRECT_NULLPARAM: D3D11_MESSAGE_ID = 3145821;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMALPHA_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146240;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMALPHA_NULLPARAM: D3D11_MESSAGE_ID = 3145880;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMAUTOPROCESSINGMODE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146245;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMAUTOPROCESSINGMODE_NULLPARAM: D3D11_MESSAGE_ID = 3145885;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE1_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146248;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE1_NULLPARAM: D3D11_MESSAGE_ID = 3146104;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146236;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE_NULLPARAM: D3D11_MESSAGE_ID = 3145876;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMDESTRECT_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146239;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMDESTRECT_NULLPARAM: D3D11_MESSAGE_ID = 3145879;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMEXTENSION_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145888;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMEXTENSION_NULLPARAM: D3D11_MESSAGE_ID = 3145887;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFILTER_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146246;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFILTER_NULLPARAM: D3D11_MESSAGE_ID = 3145886;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFRAMEFORMAT_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146235;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFRAMEFORMAT_NULLPARAM: D3D11_MESSAGE_ID = 3145875;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_INVALIDSIZE: D3D11_MESSAGE_ID = 3146234;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146233;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_NULLPARAM: D3D11_MESSAGE_ID = 3146232;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMLUMAKEY_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146243;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMLUMAKEY_NULLPARAM: D3D11_MESSAGE_ID = 3145883;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMMIRROR_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146249;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMMIRROR_NULLPARAM: D3D11_MESSAGE_ID = 3146105;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMOUTPUTRATE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146237;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMOUTPUTRATE_NULLPARAM: D3D11_MESSAGE_ID = 3145877;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPALETTE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146241;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPALETTE_NULLPARAM: D3D11_MESSAGE_ID = 3145881;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPIXELASPECTRATIO_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146242;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPIXELASPECTRATIO_NULLPARAM: D3D11_MESSAGE_ID = 3145882;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMROTATION_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146247;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMROTATION_NULLPARAM: D3D11_MESSAGE_ID = 3146034;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSOURCERECT_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146238;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSOURCERECT_NULLPARAM: D3D11_MESSAGE_ID = 3145878;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSTEREOFORMAT_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146244;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSTEREOFORMAT_NULLPARAM: D3D11_MESSAGE_ID = 3145884;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_INVALIDFILLMODE: D3D11_MESSAGE_ID = 3145816;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145815;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_NULLPARAM: D3D11_MESSAGE_ID = 3145813;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_UNSUPPORTED: D3D11_MESSAGE_ID = 3145814;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTBACKGROUNDCOLOR_INVALIDALPHA: D3D11_MESSAGE_ID = 3145811;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTBACKGROUNDCOLOR_NULLPARAM: D3D11_MESSAGE_ID = 3145810;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCOLORSPACE1_NULLPARAM: D3D11_MESSAGE_ID = 3146097;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCOLORSPACE_NULLPARAM: D3D11_MESSAGE_ID = 3145812;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_INVALIDSIZE: D3D11_MESSAGE_ID = 3145827;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_NULLPARAM: D3D11_MESSAGE_ID = 3145817;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_UNSUPPORTED: D3D11_MESSAGE_ID = 3145826;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTEXTENSION_NULLPARAM: D3D11_MESSAGE_ID = 3145820;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTHDRMETADATA_INVALIDSIZE: D3D11_MESSAGE_ID = 3146226;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTHDRMETADATA_NULLPARAM: D3D11_MESSAGE_ID = 3146225;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSHADERUSAGE_NULLPARAM: D3D11_MESSAGE_ID = 3146109;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSTEREOMODE_NULLPARAM: D3D11_MESSAGE_ID = 3145818;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSTEREOMODE_UNSUPPORTED: D3D11_MESSAGE_ID = 3145819;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTTARGETRECT_NULLPARAM: D3D11_MESSAGE_ID = 3145809;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_INVALIDALPHA: D3D11_MESSAGE_ID = 3145847;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145846;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_NULLPARAM: D3D11_MESSAGE_ID = 3145845;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_UNSUPPORTED: D3D11_MESSAGE_ID = 3146051;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMAUTOPROCESSINGMODE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145867;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMAUTOPROCESSINGMODE_NULLPARAM: D3D11_MESSAGE_ID = 3145866;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE1_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146100;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE1_NULLPARAM: D3D11_MESSAGE_ID = 3146099;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145834;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE_NULLPARAM: D3D11_MESSAGE_ID = 3145833;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_INVALIDRECT: D3D11_MESSAGE_ID = 3145844;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145843;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_NULLPARAM: D3D11_MESSAGE_ID = 3145842;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMEXTENSION_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145874;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMEXTENSION_NULLPARAM: D3D11_MESSAGE_ID = 3145873;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDFILTER: D3D11_MESSAGE_ID = 3145870;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDLEVEL: D3D11_MESSAGE_ID = 3145872;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145869;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_NULLPARAM: D3D11_MESSAGE_ID = 3145868;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_UNSUPPORTED: D3D11_MESSAGE_ID = 3145871;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_INVALIDFORMAT: D3D11_MESSAGE_ID = 3145831;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145832;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_NULLPARAM: D3D11_MESSAGE_ID = 3145830;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_INVALIDSIZE: D3D11_MESSAGE_ID = 3146231;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146230;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_NULLPARAM: D3D11_MESSAGE_ID = 3146229;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_INVALIDRANGE: D3D11_MESSAGE_ID = 3145857;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145856;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_NULLPARAM: D3D11_MESSAGE_ID = 3145855;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_UNSUPPORTED: D3D11_MESSAGE_ID = 3145858;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146102;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_NULLPARAM: D3D11_MESSAGE_ID = 3146101;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_UNSUPPORTED: D3D11_MESSAGE_ID = 3146103;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDFLAG: D3D11_MESSAGE_ID = 3145837;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDRATE: D3D11_MESSAGE_ID = 3145836;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145838;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_NULLPARAM: D3D11_MESSAGE_ID = 3145835;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDALPHA: D3D11_MESSAGE_ID = 3145851;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDCOUNT: D3D11_MESSAGE_ID = 3145850;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145849;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_NULLPARAM: D3D11_MESSAGE_ID = 3145848;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_INVALIDRATIO: D3D11_MESSAGE_ID = 3145854;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145853;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_NULLPARAM: D3D11_MESSAGE_ID = 3145852;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_UNSUPPORTED: D3D11_MESSAGE_ID = 3146052;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_INVALID: D3D11_MESSAGE_ID = 3146032;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_INVALIDSTREAM: D3D11_MESSAGE_ID = 3146031;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_NULLPARAM: D3D11_MESSAGE_ID = 3146030;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_UNSUPPORTED: D3D11_MESSAGE_ID = 3146033;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_INVALIDRECT: D3D11_MESSAGE_ID = 3145841;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145840;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_NULLPARAM: D3D11_MESSAGE_ID = 3145839;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_FLIPUNSUPPORTED: D3D11_MESSAGE_ID = 3145862;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_FORMATUNSUPPORTED: D3D11_MESSAGE_ID = 3145864;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_INVALIDFORMAT: D3D11_MESSAGE_ID = 3145865;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_INVALIDSTREAM: D3D11_MESSAGE_ID = 3145860;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_MONOOFFSETUNSUPPORTED: D3D11_MESSAGE_ID = 3145863;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_NULLPARAM: D3D11_MESSAGE_ID = 3145859;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_UNSUPPORTED: D3D11_MESSAGE_ID = 3145861;
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = 246;
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = 3146016;
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 35;
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048582;
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D11_MESSAGE_ID = 1048583;
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 36;
pub const D3D11_MESSAGE_ID_VSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 34;
pub const D3D11_MESSAGE_ID_VSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = 33;
pub const D3D11_MESSAGE_ID_VSSHADERRESOURCES_NOT_SUPPORTED: D3D11_MESSAGE_ID = 1048618;
pub type D3D11_MESSAGE_SEVERITY = i32;
pub const D3D11_MESSAGE_SEVERITY_CORRUPTION: D3D11_MESSAGE_SEVERITY = 0;
pub const D3D11_MESSAGE_SEVERITY_ERROR: D3D11_MESSAGE_SEVERITY = 1;
pub const D3D11_MESSAGE_SEVERITY_INFO: D3D11_MESSAGE_SEVERITY = 3;
pub const D3D11_MESSAGE_SEVERITY_MESSAGE: D3D11_MESSAGE_SEVERITY = 4;
pub const D3D11_MESSAGE_SEVERITY_WARNING: D3D11_MESSAGE_SEVERITY = 2;
pub const D3D11_MINOR_VERSION: u32 = 0;
pub const D3D11_MIN_BORDER_COLOR_COMPONENT: f32 = 0.0;
pub const D3D11_MIN_DEPTH: f32 = 0.0;
pub const D3D11_MIN_FILTER_SHIFT: u32 = 4;
pub const D3D11_MIN_MAXANISOTROPY: u32 = 0;
pub const D3D11_MIP_FILTER_SHIFT: u32 = 0;
pub const D3D11_MIP_LOD_BIAS_MAX: f32 = 15.99;
pub const D3D11_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8;
pub const D3D11_MIP_LOD_RANGE_BIT_COUNT: u32 = 8;
pub const D3D11_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4;
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_PACKED_MIP_DESC {
    pub NumStandardMips: u8,
    pub NumPackedMips: u8,
    pub NumTilesForPackedMips: u32,
    pub StartTileIndexInOverallResource: u32,
}
pub const D3D11_PACKED_TILE: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_PARAMETER_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub SemanticName: windows_sys::core::PCSTR,
    pub Type: super::d3dcommon::D3D_SHADER_VARIABLE_TYPE,
    pub Class: super::d3dcommon::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub InterpolationMode: super::d3dcommon::D3D_INTERPOLATION_MODE,
    pub Flags: super::d3dcommon::D3D_PARAMETER_FLAGS,
    pub FirstInRegister: u32,
    pub FirstInComponent: u32,
    pub FirstOutRegister: u32,
    pub FirstOutComponent: u32,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D11_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15;
pub const D3D11_PIXEL_SHADER: D3D11_SHADER_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_PIXEL_SHADER_TRACE_DESC {
    pub Invocation: u64,
    pub X: i32,
    pub Y: i32,
    pub SampleMask: u64,
}
pub const D3D11_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16;
#[cfg(feature = "d3dcommon")]
pub type D3D11_PRIMITIVE = super::d3dcommon::D3D_PRIMITIVE;
#[cfg(feature = "d3dcommon")]
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
pub const D3D11_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.0;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1;
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D11_PS_OUTPUT_REGISTER_COUNT: u32 = 8;
pub const D3D11_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5;
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_QUERY_DESC1 {
    pub Query: D3D11_QUERY,
    pub MiscFlags: u32,
    pub ContextType: D3D11_CONTEXT_TYPE,
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_RASTERIZER_DESC1 {
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
    pub ForcedSampleCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_RASTERIZER_DESC2 {
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
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D11_CONSERVATIVE_RASTERIZATION_MODE,
}
pub const D3D11_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16;
#[cfg(feature = "windef")]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: windows_sys::core::BOOL,
    pub LogicOpEnable: windows_sys::core::BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub LogicOp: D3D11_LOGIC_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: super::dxgi::DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "dxgi")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
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
#[cfg(feature = "dxgi")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC1 {
    pub Format: super::dxgi::DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC1_0,
}
#[cfg(feature = "dxgi")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub union D3D11_RENDER_TARGET_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture1D: D3D11_TEX1D_RTV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV1,
    pub Texture2DMS: D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D11_TEX3D_RTV,
}
#[cfg(feature = "dxgi")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC1_0 {
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
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: f32 = 0.25;
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
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
#[cfg(feature = "d3dcommon")]
pub type D3D11_RESOURCE_RETURN_TYPE = super::d3dcommon::D3D_RESOURCE_RETURN_TYPE;
pub const D3D11_RLDO_DETAIL: D3D11_RLDO_FLAGS = 2;
pub type D3D11_RLDO_FLAGS = u32;
pub const D3D11_RLDO_IGNORE_INTERNAL: D3D11_RLDO_FLAGS = 4;
pub const D3D11_RLDO_SUMMARY: D3D11_RLDO_FLAGS = 1;
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
pub const D3D11_SDK_LAYERS_VERSION: u32 = 1;
pub const D3D11_SDK_VERSION: u32 = 7;
pub type D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER = i32;
pub const D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER_0: D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER = 0;
pub const D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER_1: D3D11_SHADER_ACCESS_RESTRICTED_RESOURCE_TIER = 1;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_BUFFER_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: super::d3dcommon::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D11_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = 2;
pub const D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = 1;
pub type D3D11_SHADER_CACHE_SUPPORT_FLAGS = i32;
pub const D3D11_SHADER_CACHE_SUPPORT_NONE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = 0;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_DESC {
    pub Version: u32,
    pub Creator: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InputParameters: u32,
    pub OutputParameters: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub CutInstructionCount: u32,
    pub EmitInstructionCount: u32,
    pub GSOutputTopology: super::d3dcommon::D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
    pub InputPrimitive: super::d3dcommon::D3D_PRIMITIVE,
    pub PatchConstantParameters: u32,
    pub cGSInstanceCount: u32,
    pub cControlPoints: u32,
    pub HSOutputPrimitive: super::d3dcommon::D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    pub HSPartitioning: super::d3dcommon::D3D_TESSELLATOR_PARTITIONING,
    pub TessellatorDomain: super::d3dcommon::D3D_TESSELLATOR_DOMAIN,
    pub cBarrierInstructions: u32,
    pub cInterlockedInstructions: u32,
    pub cTextureStoreInstructions: u32,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D11_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_INPUT_BIND_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: super::d3dcommon::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::d3dcommon::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::d3dcommon::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D11_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::dxgi::DXGI_FORMAT,
    pub ViewDimension: D3D11_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
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
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::dxgi::DXGI_FORMAT,
    pub ViewDimension: D3D11_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_SRV,
    pub Texture1D: D3D11_TEX1D_SRV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV1,
    pub Texture2DMS: D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D11_TEX3D_SRV,
    pub TextureCube: D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: D3D11_BUFFEREX_SRV,
}
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_TRACE_DESC {
    pub Type: D3D11_SHADER_TYPE,
    pub Flags: u32,
    pub Anonymous: D3D11_SHADER_TRACE_DESC_0,
}
impl Default for D3D11_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_TRACE_DESC_0 {
    pub VertexShaderTraceDesc: D3D11_VERTEX_SHADER_TRACE_DESC,
    pub HullShaderTraceDesc: D3D11_HULL_SHADER_TRACE_DESC,
    pub DomainShaderTraceDesc: D3D11_DOMAIN_SHADER_TRACE_DESC,
    pub GeometryShaderTraceDesc: D3D11_GEOMETRY_SHADER_TRACE_DESC,
    pub PixelShaderTraceDesc: D3D11_PIXEL_SHADER_TRACE_DESC,
    pub ComputeShaderTraceDesc: D3D11_COMPUTE_SHADER_TRACE_DESC,
}
impl Default for D3D11_SHADER_TRACE_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_SHADER_TRACE_FLAG_RECORD_REGISTER_READS: u32 = 2;
pub const D3D11_SHADER_TRACE_FLAG_RECORD_REGISTER_WRITES: u32 = 1;
pub type D3D11_SHADER_TRACKING_OPTIONS = i32;
pub const D3D11_SHADER_TRACKING_OPTION_ALLOW_SAME: D3D11_SHADER_TRACKING_OPTIONS = 16;
pub const D3D11_SHADER_TRACKING_OPTION_ALL_HAZARDS: D3D11_SHADER_TRACKING_OPTIONS = 1006;
pub const D3D11_SHADER_TRACKING_OPTION_ALL_HAZARDS_ALLOWING_SAME: D3D11_SHADER_TRACKING_OPTIONS = 1022;
pub const D3D11_SHADER_TRACKING_OPTION_ALL_OPTIONS: D3D11_SHADER_TRACKING_OPTIONS = 1023;
pub const D3D11_SHADER_TRACKING_OPTION_IGNORE: D3D11_SHADER_TRACKING_OPTIONS = 0;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_ATOMIC_CONSISTENCY: D3D11_SHADER_TRACKING_OPTIONS = 32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_ATOMIC_CONSISTENCY_ACROSS_THREADGROUPS: D3D11_SHADER_TRACKING_OPTIONS = 512;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_RAW: D3D11_SHADER_TRACKING_OPTIONS = 2;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_RAW_ACROSS_THREADGROUPS: D3D11_SHADER_TRACKING_OPTIONS = 64;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_UNINITIALIZED: D3D11_SHADER_TRACKING_OPTIONS = 1;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAR: D3D11_SHADER_TRACKING_OPTIONS = 4;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAR_ACROSS_THREADGROUPS: D3D11_SHADER_TRACKING_OPTIONS = 128;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAW: D3D11_SHADER_TRACKING_OPTIONS = 8;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAW_ACROSS_THREADGROUPS: D3D11_SHADER_TRACKING_OPTIONS = 256;
pub const D3D11_SHADER_TRACKING_OPTION_UAV_SPECIFIC_FLAGS: D3D11_SHADER_TRACKING_OPTIONS = 960;
pub type D3D11_SHADER_TRACKING_RESOURCE_TYPE = i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL: D3D11_SHADER_TRACKING_RESOURCE_TYPE = 7;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL_DEVICEMEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = 3;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL_SHARED_MEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = 5;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_GROUPSHARED_MEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = 4;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_GROUPSHARED_NON_UAV: D3D11_SHADER_TRACKING_RESOURCE_TYPE = 6;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_NONE: D3D11_SHADER_TRACKING_RESOURCE_TYPE = 0;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_NON_UAV_DEVICEMEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = 2;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_UAV_DEVICEMEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = 1;
pub type D3D11_SHADER_TYPE = i32;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_TYPE_DESC {
    pub Class: super::d3dcommon::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::d3dcommon::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
    pub Name: windows_sys::core::PCSTR,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D11_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_VARIABLE_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut core::ffi::c_void,
    pub StartTexture: u32,
    pub TextureSize: u32,
    pub StartSampler: u32,
    pub SamplerSize: u32,
}
impl Default for D3D11_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_SHADER_VERSION_TYPE = i32;
pub type D3D11_SHARED_RESOURCE_TIER = i32;
pub const D3D11_SHARED_RESOURCE_TIER_0: D3D11_SHARED_RESOURCE_TIER = 0;
pub const D3D11_SHARED_RESOURCE_TIER_1: D3D11_SHARED_RESOURCE_TIER = 1;
pub const D3D11_SHARED_RESOURCE_TIER_2: D3D11_SHARED_RESOURCE_TIER = 2;
pub const D3D11_SHARED_RESOURCE_TIER_3: D3D11_SHARED_RESOURCE_TIER = 3;
pub const D3D11_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0;
pub const D3D11_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5;
pub const D3D11_SHVER_COMPUTE_SHADER: D3D11_SHADER_VERSION_TYPE = 5;
pub const D3D11_SHVER_DOMAIN_SHADER: D3D11_SHADER_VERSION_TYPE = 4;
pub const D3D11_SHVER_GEOMETRY_SHADER: D3D11_SHADER_VERSION_TYPE = 2;
pub const D3D11_SHVER_HULL_SHADER: D3D11_SHADER_VERSION_TYPE = 3;
pub const D3D11_SHVER_PIXEL_SHADER: D3D11_SHADER_VERSION_TYPE = 0;
pub const D3D11_SHVER_RESERVED0: D3D11_SHADER_VERSION_TYPE = 65520;
pub const D3D11_SHVER_VERTEX_SHADER: D3D11_SHADER_VERSION_TYPE = 1;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::d3dcommon::D3D_NAME,
    pub ComponentType: super::d3dcommon::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
    pub Stream: u32,
    pub MinPrecision: super::d3dcommon::D3D_MIN_PRECISION,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D11_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
pub const D3D11_SPEC_VERSION: f64 = 1.07;
pub const D3D11_SRGB_GAMMA: f32 = 2.2;
pub const D3D11_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92;
pub const D3D11_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055;
pub const D3D11_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4;
pub const D3D11_SRGB_TO_FLOAT_OFFSET: f32 = 0.055;
pub const D3D11_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045;
pub const D3D11_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5;
#[cfg(feature = "d3dcommon")]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_SUBRESOURCE_TILING {
    pub WidthInTiles: u32,
    pub HeightInTiles: u16,
    pub DepthInTiles: u16,
    pub StartTileIndexInOverallResource: u32,
}
pub const D3D11_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8;
#[cfg(feature = "d3dcommon")]
pub type D3D11_TESSELLATOR_DOMAIN = super::d3dcommon::D3D_TESSELLATOR_DOMAIN;
pub const D3D11_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64;
pub const D3D11_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64;
pub const D3D11_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63;
pub const D3D11_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64;
pub const D3D11_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2;
pub const D3D11_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1;
pub const D3D11_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1;
#[cfg(feature = "d3dcommon")]
pub type D3D11_TESSELLATOR_OUTPUT_PRIMITIVE = super::d3dcommon::D3D_TESSELLATOR_OUTPUT_PRIMITIVE;
#[cfg(feature = "d3dcommon")]
pub type D3D11_TESSELLATOR_PARTITIONING = super::d3dcommon::D3D_TESSELLATOR_PARTITIONING;
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
pub struct D3D11_TEX2D_ARRAY_RTV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
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
pub struct D3D11_TEX2D_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
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
pub struct D3D11_TEX2D_ARRAY_UAV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
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
pub struct D3D11_TEX2D_RTV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_UAV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
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
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::dxgi::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::dxgi::DXGI_FORMAT,
    pub SampleDesc: super::dxgi::DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEXTURE2D_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::dxgi::DXGI_FORMAT,
    pub SampleDesc: super::dxgi::DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub TextureLayout: D3D11_TEXTURE_LAYOUT,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::dxgi::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEXTURE3D_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::dxgi::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub TextureLayout: D3D11_TEXTURE_LAYOUT,
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
pub type D3D11_TEXTURE_LAYOUT = i32;
pub const D3D11_TEXTURE_LAYOUT_64K_STANDARD_SWIZZLE: D3D11_TEXTURE_LAYOUT = 2;
pub const D3D11_TEXTURE_LAYOUT_ROW_MAJOR: D3D11_TEXTURE_LAYOUT = 1;
pub const D3D11_TEXTURE_LAYOUT_UNDEFINED: D3D11_TEXTURE_LAYOUT = 0;
pub const D3D11_TILED_RESOURCES_NOT_SUPPORTED: D3D11_TILED_RESOURCES_TIER = 0;
pub type D3D11_TILED_RESOURCES_TIER = i32;
pub const D3D11_TILED_RESOURCES_TIER_1: D3D11_TILED_RESOURCES_TIER = 1;
pub const D3D11_TILED_RESOURCES_TIER_2: D3D11_TILED_RESOURCES_TIER = 2;
pub const D3D11_TILED_RESOURCES_TIER_3: D3D11_TILED_RESOURCES_TIER = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TILED_RESOURCE_COORDINATE {
    pub X: u32,
    pub Y: u32,
    pub Z: u32,
    pub Subresource: u32,
}
pub type D3D11_TILE_COPY_FLAG = i32;
pub const D3D11_TILE_COPY_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE: D3D11_TILE_COPY_FLAG = 2;
pub const D3D11_TILE_COPY_NO_OVERWRITE: D3D11_TILE_COPY_FLAG = 1;
pub const D3D11_TILE_COPY_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER: D3D11_TILE_COPY_FLAG = 4;
pub type D3D11_TILE_MAPPING_FLAG = i32;
pub const D3D11_TILE_MAPPING_NO_OVERWRITE: D3D11_TILE_MAPPING_FLAG = 1;
pub type D3D11_TILE_RANGE_FLAG = i32;
pub const D3D11_TILE_RANGE_NULL: D3D11_TILE_RANGE_FLAG = 1;
pub const D3D11_TILE_RANGE_REUSE_SINGLE_TILE: D3D11_TILE_RANGE_FLAG = 4;
pub const D3D11_TILE_RANGE_SKIP: D3D11_TILE_RANGE_FLAG = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TILE_REGION_SIZE {
    pub NumTiles: u32,
    pub bUseBox: windows_sys::core::BOOL,
    pub Width: u32,
    pub Height: u16,
    pub Depth: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TILE_SHAPE {
    pub WidthInTexels: u32,
    pub HeightInTexels: u32,
    pub DepthInTexels: u32,
}
pub type D3D11_TRACE_COMPONENT_MASK = u8;
pub const D3D11_TRACE_COMPONENT_W: u32 = 8;
pub const D3D11_TRACE_COMPONENT_X: u32 = 1;
pub const D3D11_TRACE_COMPONENT_Y: u32 = 2;
pub const D3D11_TRACE_COMPONENT_Z: u32 = 4;
pub const D3D11_TRACE_CONSTANT_BUFFER: D3D11_TRACE_REGISTER_TYPE = 8;
pub type D3D11_TRACE_GS_INPUT_PRIMITIVE = i32;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_LINE: D3D11_TRACE_GS_INPUT_PRIMITIVE = 2;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_LINE_ADJ: D3D11_TRACE_GS_INPUT_PRIMITIVE = 6;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_POINT: D3D11_TRACE_GS_INPUT_PRIMITIVE = 1;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_TRIANGLE: D3D11_TRACE_GS_INPUT_PRIMITIVE = 3;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_TRIANGLE_ADJ: D3D11_TRACE_GS_INPUT_PRIMITIVE = 7;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_UNDEFINED: D3D11_TRACE_GS_INPUT_PRIMITIVE = 0;
pub const D3D11_TRACE_IMMEDIATE32: D3D11_TRACE_REGISTER_TYPE = 9;
pub const D3D11_TRACE_IMMEDIATE64: D3D11_TRACE_REGISTER_TYPE = 33;
pub const D3D11_TRACE_IMMEDIATE_CONSTANT_BUFFER: D3D11_TRACE_REGISTER_TYPE = 3;
pub const D3D11_TRACE_INDEXABLE_TEMP_REGISTER: D3D11_TRACE_REGISTER_TYPE = 5;
pub const D3D11_TRACE_INPUT_CONTROL_POINT_REGISTER: D3D11_TRACE_REGISTER_TYPE = 19;
pub const D3D11_TRACE_INPUT_COVERAGE_MASK_REGISTER: D3D11_TRACE_REGISTER_TYPE = 28;
pub const D3D11_TRACE_INPUT_CYCLE_COUNTER_REGISTER: D3D11_TRACE_REGISTER_TYPE = 34;
pub const D3D11_TRACE_INPUT_DOMAIN_POINT_REGISTER: D3D11_TRACE_REGISTER_TYPE = 22;
pub const D3D11_TRACE_INPUT_FORK_INSTANCE_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = 17;
pub const D3D11_TRACE_INPUT_GS_INSTANCE_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = 30;
pub const D3D11_TRACE_INPUT_JOIN_INSTANCE_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = 18;
pub const D3D11_TRACE_INPUT_PATCH_CONSTANT_REGISTER: D3D11_TRACE_REGISTER_TYPE = 21;
pub const D3D11_TRACE_INPUT_PRIMITIVE_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = 2;
pub const D3D11_TRACE_INPUT_REGISTER: D3D11_TRACE_REGISTER_TYPE = 1;
pub const D3D11_TRACE_INPUT_THREAD_GROUP_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = 26;
pub const D3D11_TRACE_INPUT_THREAD_ID_IN_GROUP_FLATTENED_REGISTER: D3D11_TRACE_REGISTER_TYPE = 29;
pub const D3D11_TRACE_INPUT_THREAD_ID_IN_GROUP_REGISTER: D3D11_TRACE_REGISTER_TYPE = 27;
pub const D3D11_TRACE_INPUT_THREAD_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = 25;
pub const D3D11_TRACE_INTERFACE_POINTER: D3D11_TRACE_REGISTER_TYPE = 35;
pub const D3D11_TRACE_MISC_GS_CUT: u32 = 2;
pub const D3D11_TRACE_MISC_GS_CUT_STREAM: u32 = 16;
pub const D3D11_TRACE_MISC_GS_EMIT: u32 = 1;
pub const D3D11_TRACE_MISC_GS_EMIT_STREAM: u32 = 8;
pub const D3D11_TRACE_MISC_HALT: u32 = 32;
pub const D3D11_TRACE_MISC_MESSAGE: u32 = 64;
pub type D3D11_TRACE_MISC_OPERATIONS_MASK = u16;
pub const D3D11_TRACE_MISC_PS_DISCARD: u32 = 4;
pub const D3D11_TRACE_OUTPUT_CONTROL_POINT_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = 16;
pub const D3D11_TRACE_OUTPUT_CONTROL_POINT_REGISTER: D3D11_TRACE_REGISTER_TYPE = 20;
pub const D3D11_TRACE_OUTPUT_COVERAGE_MASK: D3D11_TRACE_REGISTER_TYPE = 13;
pub const D3D11_TRACE_OUTPUT_DEPTH_GREATER_EQUAL_REGISTER: D3D11_TRACE_REGISTER_TYPE = 31;
pub const D3D11_TRACE_OUTPUT_DEPTH_LESS_EQUAL_REGISTER: D3D11_TRACE_REGISTER_TYPE = 32;
pub const D3D11_TRACE_OUTPUT_DEPTH_REGISTER: D3D11_TRACE_REGISTER_TYPE = 7;
pub const D3D11_TRACE_OUTPUT_NULL_REGISTER: D3D11_TRACE_REGISTER_TYPE = 0;
pub const D3D11_TRACE_OUTPUT_REGISTER: D3D11_TRACE_REGISTER_TYPE = 6;
pub const D3D11_TRACE_RASTERIZER: D3D11_TRACE_REGISTER_TYPE = 12;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_TRACE_REGISTER {
    pub RegType: D3D11_TRACE_REGISTER_TYPE,
    pub Anonymous: D3D11_TRACE_REGISTER_0,
    pub OperandIndex: u8,
    pub Flags: u8,
}
impl Default for D3D11_TRACE_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_TRACE_REGISTER_0 {
    pub Index1D: u16,
    pub Index2D: [u16; 2],
}
impl Default for D3D11_TRACE_REGISTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_TRACE_REGISTER_FLAGS_RELATIVE_INDEXING: u32 = 1;
pub type D3D11_TRACE_REGISTER_TYPE = i32;
pub const D3D11_TRACE_RESOURCE: D3D11_TRACE_REGISTER_TYPE = 11;
pub const D3D11_TRACE_SAMPLER: D3D11_TRACE_REGISTER_TYPE = 10;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_TRACE_STATS {
    pub TraceDesc: D3D11_SHADER_TRACE_DESC,
    pub NumInvocationsInStamp: u8,
    pub TargetStampIndex: u8,
    pub NumTraceSteps: u32,
    pub InputMask: [D3D11_TRACE_COMPONENT_MASK; 32],
    pub OutputMask: [D3D11_TRACE_COMPONENT_MASK; 32],
    pub NumTemps: u16,
    pub MaxIndexableTempIndex: u16,
    pub IndexableTempSize: [u16; 4096],
    pub ImmediateConstantBufferSize: u16,
    pub PixelPosition: [[u32; 2]; 4],
    pub PixelCoverageMask: [u64; 4],
    pub PixelDiscardedMask: [u64; 4],
    pub PixelCoverageMaskAfterShader: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMask: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMaskDepth: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMaskDepthStencil: [u64; 4],
    pub PSOutputsDepth: windows_sys::core::BOOL,
    pub PSOutputsMask: windows_sys::core::BOOL,
    pub GSInputPrimitive: D3D11_TRACE_GS_INPUT_PRIMITIVE,
    pub GSInputsPrimitiveID: windows_sys::core::BOOL,
    pub HSOutputPatchConstantMask: [D3D11_TRACE_COMPONENT_MASK; 32],
    pub DSInputPatchConstantMask: [D3D11_TRACE_COMPONENT_MASK; 32],
}
impl Default for D3D11_TRACE_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TRACE_STEP {
    pub ID: u32,
    pub InstructionActive: windows_sys::core::BOOL,
    pub NumRegistersWritten: u8,
    pub NumRegistersRead: u8,
    pub MiscOperations: D3D11_TRACE_MISC_OPERATIONS_MASK,
    pub OpcodeType: u32,
    pub CurrentGlobalCycle: u64,
}
pub const D3D11_TRACE_STREAM: D3D11_TRACE_REGISTER_TYPE = 14;
pub const D3D11_TRACE_TEMP_REGISTER: D3D11_TRACE_REGISTER_TYPE = 4;
pub const D3D11_TRACE_THIS_POINTER: D3D11_TRACE_REGISTER_TYPE = 15;
pub const D3D11_TRACE_THREAD_GROUP_SHARED_MEMORY: D3D11_TRACE_REGISTER_TYPE = 24;
pub const D3D11_TRACE_UNORDERED_ACCESS_VIEW: D3D11_TRACE_REGISTER_TYPE = 23;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_TRACE_VALUE {
    pub Bits: [u32; 4],
    pub ValidMask: D3D11_TRACE_COMPONENT_MASK,
}
impl Default for D3D11_TRACE_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: super::dxgi::DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC_0,
}
#[cfg(feature = "dxgi")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV,
    pub Texture3D: D3D11_TEX3D_UAV,
}
#[cfg(feature = "dxgi")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    pub Format: super::dxgi::DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC1_0,
}
#[cfg(feature = "dxgi")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV1,
    pub Texture3D: D3D11_TEX3D_UAV,
}
#[cfg(feature = "dxgi")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
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
pub const D3D11_VERTEX_SHADER: D3D11_SHADER_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VERTEX_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    pub pCryptoSession: *mut core::ffi::c_void,
    pub BlobSize: u32,
    pub pBlob: *mut core::ffi::c_void,
    pub pKeyInfoId: *mut windows_sys::core::GUID,
    pub PrivateDataSize: u32,
    pub pPrivateData: *mut core::ffi::c_void,
}
impl Default for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
}
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
    pub cBlocksStripeEncrypted: u32,
    pub cBlocksStripeClear: u32,
}
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC2 {
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
pub type D3D11_VIDEO_DECODER_CAPS = i32;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE: D3D11_VIDEO_DECODER_CAPS = 1;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_DYNAMIC: D3D11_VIDEO_DECODER_CAPS = 4;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_REQUIRED: D3D11_VIDEO_DECODER_CAPS = 8;
pub const D3D11_VIDEO_DECODER_CAPS_NON_REAL_TIME: D3D11_VIDEO_DECODER_CAPS = 2;
pub const D3D11_VIDEO_DECODER_CAPS_UNSUPPORTED: D3D11_VIDEO_DECODER_CAPS = 16;
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
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_DECODER_DESC {
    pub Guid: windows_sys::core::GUID,
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub OutputFormat: super::dxgi::DXGI_FORMAT,
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
pub type D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_A: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 3;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_B: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 2;
pub type D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_A: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 8;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_B: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 4;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_G: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 2;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_NONE: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 0;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_R: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 1;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_U: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 2;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_V: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 4;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_Y: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 1;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_G: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 1;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_R: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 0;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_U: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 1;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_V: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 2;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_Y: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 0;
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    pub ClearSize: u32,
    pub EncryptedSize: u32,
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
pub type D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = i32;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_COLOR_SPACE_CONVERSION: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 4;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_RESIZE: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 2;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_ROTATION: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 1;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_TRIPLE_BUFFER_OUTPUT: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 8;
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
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    pub InputFrameFormat: D3D11_VIDEO_FRAME_FORMAT,
    pub InputFrameRate: super::dxgi::DXGI_RATIONAL,
    pub InputWidth: u32,
    pub InputHeight: u32,
    pub OutputFrameRate: super::dxgi::DXGI_RATIONAL,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
    pub Usage: D3D11_VIDEO_USAGE,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    pub CustomRate: super::dxgi::DXGI_RATIONAL,
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
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    pub Enable: windows_sys::core::BOOL,
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgi::DXGI_FORMAT,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_SAMPLE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgi::DXGI_FORMAT,
    pub ColorSpace: super::dxgi::DXGI_COLOR_SPACE_TYPE,
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
pub const D3D_RETURN_PARAMETER_INDEX: i32 = -1;
pub const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: u32 = 32;
pub const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: u32 = 64;
pub const D3D_SHADER_REQUIRES_64_UAVS: u32 = 8;
pub const D3D_SHADER_REQUIRES_DOUBLES: u32 = 1;
pub const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: u32 = 2;
pub const D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING: u32 = 128;
pub const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: u32 = 16;
pub const D3D_SHADER_REQUIRES_TILED_RESOURCES: u32 = 256;
pub const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: u32 = 4;
pub const DXGI_DEBUG_D3D11: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b99317b_ac39_4aa6_bb0b_baa04784798f);
#[cfg(feature = "d3dcommon")]
pub type PFN_D3D11ON12_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: u32, param2: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const *mut core::ffi::c_void, numqueues: u32, param6: u32, param7: *mut *mut core::ffi::c_void, param8: *mut *mut core::ffi::c_void, param9: *mut super::d3dcommon::D3D_FEATURE_LEVEL) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "d3dcommon", feature = "dxgi", feature = "minwindef"))]
pub type PFN_D3D11_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::d3dcommon::D3D_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *mut *mut core::ffi::c_void, param8: *mut super::d3dcommon::D3D_FEATURE_LEVEL, param9: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "d3dcommon", feature = "dxgi", feature = "minwindef", feature = "windef"))]
pub type PFN_D3D11_CREATE_DEVICE_AND_SWAP_CHAIN = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::d3dcommon::D3D_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *const super::dxgi::DXGI_SWAP_CHAIN_DESC, param8: *mut *mut core::ffi::c_void, param9: *mut *mut core::ffi::c_void, param10: *mut super::d3dcommon::D3D_FEATURE_LEVEL, param11: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
