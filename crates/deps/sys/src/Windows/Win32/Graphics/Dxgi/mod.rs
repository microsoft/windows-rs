#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory(riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory1(riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory2(flags: u32, riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn DXGIDeclareAdapterRemovalSupport() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn DXGIGetDebugInterface1(flags: u32, riid: *const ::windows_sys::core::GUID, pdebug: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub struct DXGI_ADAPTER_DESC(i32);
pub struct DXGI_ADAPTER_DESC1(i32);
pub struct DXGI_ADAPTER_DESC2(i32);
pub struct DXGI_ADAPTER_DESC3(i32);
pub struct DXGI_ADAPTER_FLAG(i32);
pub struct DXGI_ADAPTER_FLAG3(i32);
pub struct DXGI_COMPUTE_PREEMPTION_GRANULARITY(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1u32;
pub const DXGI_DEBUG_ALL: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3834307203,
    data2: 55936,
    data3: 18699,
    data4: [135, 230, 67, 233, 169, 207, 218, 8],
};
pub const DXGI_DEBUG_APP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 114126337, data2: 16921, data3: 20157, data4: [135, 9, 39, 237, 35, 54, 12, 98] };
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
pub const DXGI_DEBUG_DX: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 902682620, data2: 5042, data3: 16925, data4: [165, 215, 126, 68, 81, 40, 125, 100] };
pub const DXGI_DEBUG_DXGI: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 634247844, data2: 45510, data3: 18401, data4: [172, 62, 152, 135, 91, 90, 46, 42] };
pub struct DXGI_DEBUG_RLO_FLAGS(i32);
pub struct DXGI_DECODE_SWAP_CHAIN_DESC(i32);
pub struct DXGI_DISPLAY_COLOR_SPACE(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ENUM_MODES_SCALING: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ENUM_MODES_STEREO: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_ACCESS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270485i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_ACCESS_LOST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270490i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270474i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_CACHE_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270477i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_CACHE_FULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270476i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_CACHE_HASH_COLLISION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270475i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270486i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_DEVICE_HUNG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270522i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_DEVICE_REMOVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270523i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_DEVICE_RESET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270521i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270496i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270479i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270517i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270516i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270480i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_INVALID_CALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270527i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270491i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_MORE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270525i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270484i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_NONEXCLUSIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270495i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_NON_COMPOSITED_UI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270478i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_NOT_CURRENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270482i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270494i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270526i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270493i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270492i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270487i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270483i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_SESSION_DISCONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270488i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270524i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_WAIT_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270489i32 as _);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_ERROR_WAS_STILL_DRAWING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270518i32 as _);
pub struct DXGI_FEATURE(i32);
pub struct DXGI_FRAME_PRESENTATION_MODE(i32);
pub struct DXGI_FRAME_STATISTICS(i32);
pub struct DXGI_FRAME_STATISTICS_MEDIA(i32);
pub struct DXGI_GPU_PREFERENCE(i32);
pub struct DXGI_GRAPHICS_PREEMPTION_GRANULARITY(i32);
pub struct DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(i32);
pub struct DXGI_HDR_METADATA_HDR10(i32);
pub struct DXGI_HDR_METADATA_HDR10PLUS(i32);
pub struct DXGI_HDR_METADATA_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
pub struct DXGI_INFO_QUEUE_FILTER(i32);
pub struct DXGI_INFO_QUEUE_FILTER_DESC(i32);
pub struct DXGI_INFO_QUEUE_MESSAGE(i32);
pub struct DXGI_INFO_QUEUE_MESSAGE_CATEGORY(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
pub struct DXGI_INFO_QUEUE_MESSAGE_SEVERITY(i32);
pub struct DXGI_MAPPED_RECT(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_MAP_DISCARD: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_MAP_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_MAP_WRITE: u32 = 2u32;
pub struct DXGI_MATRIX_3X2_F(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
pub struct DXGI_MEMORY_SEGMENT_GROUP(i32);
pub struct DXGI_MODE_DESC1(i32);
pub struct DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_MWA_VALID: u32 = 7u32;
pub struct DXGI_Message_Id(i32);
pub struct DXGI_OFFER_RESOURCE_FLAGS(i32);
pub struct DXGI_OFFER_RESOURCE_PRIORITY(i32);
pub struct DXGI_OUTDUPL_DESC(i32);
pub struct DXGI_OUTDUPL_FLAG(i32);
pub struct DXGI_OUTDUPL_FRAME_INFO(i32);
pub struct DXGI_OUTDUPL_MOVE_RECT(i32);
pub struct DXGI_OUTDUPL_POINTER_POSITION(i32);
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO(i32);
pub struct DXGI_OUTDUPL_POINTER_SHAPE_TYPE(i32);
pub struct DXGI_OUTPUT_DESC(i32);
pub struct DXGI_OUTPUT_DESC1(i32);
pub struct DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(i32);
pub struct DXGI_OVERLAY_SUPPORT_FLAG(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8u32;
pub struct DXGI_PRESENT_PARAMETERS(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_RESTART: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_TEST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_PRESENT_USE_DURATION: u32 = 256u32;
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO(i32);
pub struct DXGI_RECLAIM_RESOURCE_RESULTS(i32);
pub struct DXGI_RESIDENCY(i32);
pub struct DXGI_RESOURCE_PRIORITY(i32);
pub struct DXGI_RGBA(i32);
pub struct DXGI_SCALING(i32);
pub struct DXGI_SHARED_RESOURCE(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1u32;
pub struct DXGI_SURFACE_DESC(i32);
pub struct DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(i32);
pub struct DXGI_SWAP_CHAIN_DESC(i32);
pub struct DXGI_SWAP_CHAIN_DESC1(i32);
pub struct DXGI_SWAP_CHAIN_FLAG(i32);
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC(i32);
pub struct DXGI_SWAP_EFFECT(i32);
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_USAGE_READ_ONLY: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_USAGE_SHARED: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024u32;
pub struct IDXGIAdapter(i32);
pub struct IDXGIAdapter1(i32);
pub struct IDXGIAdapter2(i32);
pub struct IDXGIAdapter3(i32);
pub struct IDXGIAdapter4(i32);
pub struct IDXGIDebug(i32);
pub struct IDXGIDebug1(i32);
pub struct IDXGIDecodeSwapChain(i32);
pub struct IDXGIDevice(i32);
pub struct IDXGIDevice1(i32);
pub struct IDXGIDevice2(i32);
pub struct IDXGIDevice3(i32);
pub struct IDXGIDevice4(i32);
pub struct IDXGIDeviceSubObject(i32);
pub struct IDXGIDisplayControl(i32);
pub struct IDXGIFactory(i32);
pub struct IDXGIFactory1(i32);
pub struct IDXGIFactory2(i32);
pub struct IDXGIFactory3(i32);
pub struct IDXGIFactory4(i32);
pub struct IDXGIFactory5(i32);
pub struct IDXGIFactory6(i32);
pub struct IDXGIFactory7(i32);
pub struct IDXGIFactoryMedia(i32);
pub struct IDXGIInfoQueue(i32);
pub struct IDXGIKeyedMutex(i32);
pub struct IDXGIObject(i32);
pub struct IDXGIOutput(i32);
pub struct IDXGIOutput1(i32);
pub struct IDXGIOutput2(i32);
pub struct IDXGIOutput3(i32);
pub struct IDXGIOutput4(i32);
pub struct IDXGIOutput5(i32);
pub struct IDXGIOutput6(i32);
pub struct IDXGIOutputDuplication(i32);
pub struct IDXGIResource(i32);
pub struct IDXGIResource1(i32);
pub struct IDXGISurface(i32);
pub struct IDXGISurface1(i32);
pub struct IDXGISurface2(i32);
pub struct IDXGISwapChain(i32);
pub struct IDXGISwapChain1(i32);
pub struct IDXGISwapChain2(i32);
pub struct IDXGISwapChain3(i32);
pub struct IDXGISwapChain4(i32);
pub struct IDXGISwapChainMedia(i32);
pub struct IDXGraphicsAnalysis(i32);
