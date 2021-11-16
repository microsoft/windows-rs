#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    pub fn CreateDXGIFactory(riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CreateDXGIFactory1(riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CreateDXGIFactory2(flags: u32, riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DXGIDeclareAdapterRemovalSupport() -> ::windows_sys::core::HRESULT;
    pub fn DXGIGetDebugInterface1(flags: u32, riid: *const ::windows_sys::core::GUID, pdebug: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_ADAPTER_FLAG_NONE: u32 = 0u32;
pub const DXGI_ADAPTER_FLAG_REMOTE: u32 = 1u32;
pub const DXGI_ADAPTER_FLAG_SOFTWARE: u32 = 2u32;
pub const DXGI_ADAPTER_FLAG3_NONE: u32 = 0u32;
pub const DXGI_ADAPTER_FLAG3_REMOTE: u32 = 1u32;
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: u32 = 2u32;
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: u32 = 4u32;
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: u32 = 8u32;
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: u32 = 16u32;
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: u32 = 32u32;
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: u32 = 4294967295u32;
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: i32 = 0i32;
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: i32 = 1i32;
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: i32 = 2i32;
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: i32 = 3i32;
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: i32 = 4i32;
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1u32;
pub const DXGI_DEBUG_ALL: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3834307203,
    data2: 55936,
    data3: 18699,
    data4: [135, 230, 67, 233, 169, 207, 218, 8],
};
pub const DXGI_DEBUG_APP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 114126337, data2: 16921, data3: 20157, data4: [135, 9, 39, 237, 35, 54, 12, 98] };
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
pub const DXGI_DEBUG_DX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 902682620, data2: 5042, data3: 16925, data4: [165, 215, 126, 68, 81, 40, 125, 100] };
pub const DXGI_DEBUG_DXGI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 634247844, data2: 45510, data3: 18401, data4: [172, 62, 152, 135, 91, 90, 46, 42] };
pub const DXGI_DEBUG_RLO_SUMMARY: u32 = 1u32;
pub const DXGI_DEBUG_RLO_DETAIL: u32 = 2u32;
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: u32 = 4u32;
pub const DXGI_DEBUG_RLO_ALL: u32 = 7u32;
#[repr(C)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
impl ::core::marker::Copy for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::clone::Clone for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [f32; 16],
    pub WhitePoints: [f32; 32],
}
impl ::core::marker::Copy for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::clone::Clone for DXGI_DISPLAY_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8u32;
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1u32;
pub const DXGI_ENUM_MODES_SCALING: u32 = 2u32;
pub const DXGI_ENUM_MODES_STEREO: u32 = 4u32;
pub const DXGI_ERROR_ACCESS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270485i32 as _);
pub const DXGI_ERROR_ACCESS_LOST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270490i32 as _);
pub const DXGI_ERROR_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270474i32 as _);
pub const DXGI_ERROR_CACHE_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270477i32 as _);
pub const DXGI_ERROR_CACHE_FULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270476i32 as _);
pub const DXGI_ERROR_CACHE_HASH_COLLISION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270475i32 as _);
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270486i32 as _);
pub const DXGI_ERROR_DEVICE_HUNG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270522i32 as _);
pub const DXGI_ERROR_DEVICE_REMOVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270523i32 as _);
pub const DXGI_ERROR_DEVICE_RESET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270521i32 as _);
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270496i32 as _);
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270479i32 as _);
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270517i32 as _);
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270516i32 as _);
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270480i32 as _);
pub const DXGI_ERROR_INVALID_CALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270527i32 as _);
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270491i32 as _);
pub const DXGI_ERROR_MORE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270525i32 as _);
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270484i32 as _);
pub const DXGI_ERROR_NONEXCLUSIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270495i32 as _);
pub const DXGI_ERROR_NON_COMPOSITED_UI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270478i32 as _);
pub const DXGI_ERROR_NOT_CURRENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270482i32 as _);
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270494i32 as _);
pub const DXGI_ERROR_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270526i32 as _);
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270493i32 as _);
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270492i32 as _);
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270487i32 as _);
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270483i32 as _);
pub const DXGI_ERROR_SESSION_DISCONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270488i32 as _);
pub const DXGI_ERROR_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270524i32 as _);
pub const DXGI_ERROR_WAIT_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270489i32 as _);
pub const DXGI_ERROR_WAS_STILL_DRAWING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005270518i32 as _);
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: i32 = 0i32;
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: i32 = 0i32;
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: i32 = 1i32;
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: i32 = 2i32;
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: i32 = 3i32;
#[repr(C)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS_MEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: i32 = 0i32;
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: i32 = 1i32;
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: i32 = 2i32;
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: i32 = 0i32;
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: i32 = 1i32;
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: i32 = 2i32;
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: i32 = 3i32;
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: i32 = 4i32;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: u32 = 1u32;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: u32 = 2u32;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: u32 = 4u32;
#[repr(C)]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10 {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10PLUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_HDR_METADATA_TYPE_NONE: i32 = 0i32;
pub const DXGI_HDR_METADATA_TYPE_HDR10: i32 = 1i32;
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: i32 = 2i32;
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut i32,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: ::windows_sys::core::GUID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: i32,
    pub pDescription: *mut u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: i32 = 0i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: i32 = 1i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: i32 = 2i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: i32 = 3i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: i32 = 4i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: i32 = 5i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: i32 = 6i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: i32 = 7i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: i32 = 8i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: i32 = 9i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: i32 = 10i32;
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: i32 = 0i32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: i32 = 1i32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: i32 = 2i32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: i32 = 3i32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: i32 = 4i32;
#[repr(C)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl ::core::marker::Copy for DXGI_MAPPED_RECT {}
impl ::core::clone::Clone for DXGI_MAPPED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_MAP_DISCARD: u32 = 4u32;
pub const DXGI_MAP_READ: u32 = 1u32;
pub const DXGI_MAP_WRITE: u32 = 2u32;
#[repr(C)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl ::core::marker::Copy for DXGI_MATRIX_3X2_F {}
impl ::core::clone::Clone for DXGI_MATRIX_3X2_F {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: i32 = 0i32;
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: i32 = 1i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub Format: Common::DXGI_FORMAT,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Stereo: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_MODE_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_MODE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: i32 = 1i32;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: i32 = 2i32;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: i32 = 4i32;
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2u32;
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4u32;
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1u32;
pub const DXGI_MWA_VALID: u32 = 7u32;
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_InvalidOutputWindow: i32 = 0i32;
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferWidthInferred: i32 = 1i32;
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferHeightInferred: i32 = 2i32;
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_NoScanoutFlagChanged: i32 = 3i32;
pub const DXGI_MSG_IDXGISwapChain_Creation_MaxBufferCountExceeded: i32 = 4i32;
pub const DXGI_MSG_IDXGISwapChain_Creation_TooFewBuffers: i32 = 5i32;
pub const DXGI_MSG_IDXGISwapChain_Creation_NoOutputWindow: i32 = 6i32;
pub const DXGI_MSG_IDXGISwapChain_Destruction_OtherMethodsCalled: i32 = 7i32;
pub const DXGI_MSG_IDXGISwapChain_GetDesc_pDescIsNULL: i32 = 8i32;
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_ppSurfaceIsNULL: i32 = 9i32;
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_NoAllocatedBuffers: i32 = 10i32;
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferMustBeZero: i32 = 11i32;
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferOOB: i32 = 12i32;
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_ppOutputIsNULL: i32 = 13i32;
pub const DXGI_MSG_IDXGISwapChain_Present_SyncIntervalOOB: i32 = 14i32;
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidNonPreRotatedFlag: i32 = 15i32;
pub const DXGI_MSG_IDXGISwapChain_Present_NoAllocatedBuffers: i32 = 16i32;
pub const DXGI_MSG_IDXGISwapChain_Present_GetDXGIAdapterFailed: i32 = 17i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOB: i32 = 18i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_UnreleasedReferences: i32 = 19i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidSwapChainFlag: i32 = 20i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidNonPreRotatedFlag: i32 = 21i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_RefreshRateDivideByZero: i32 = 22i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_InvalidTarget: i32 = 23i32;
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_pStatsIsNULL: i32 = 24i32;
pub const DXGI_MSG_IDXGISwapChain_GetLastPresentCount_pLastPresentCountIsNULL: i32 = 25i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_RemoteNotSupported: i32 = 26i32;
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_FailedToAcquireFullscreenMutex: i32 = 27i32;
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ppAdapterInterfaceIsNULL: i32 = 28i32;
pub const DXGI_MSG_IDXGIFactory_EnumAdapters_ppAdapterInterfaceIsNULL: i32 = 29i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ppSwapChainIsNULL: i32 = 30i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDescIsNULL: i32 = 31i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnknownSwapEffect: i32 = 32i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFlags: i32 = 33i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedFlagAndWindowed: i32 = 34i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NullDeviceInterface: i32 = 35i32;
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_phWndIsNULL: i32 = 36i32;
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_InvalidFlags: i32 = 37i32;
pub const DXGI_MSG_IDXGISurface_Map_InvalidSurface: i32 = 38i32;
pub const DXGI_MSG_IDXGISurface_Map_FlagsSetToZero: i32 = 39i32;
pub const DXGI_MSG_IDXGISurface_Map_DiscardAndReadFlagSet: i32 = 40i32;
pub const DXGI_MSG_IDXGISurface_Map_DiscardButNotWriteFlagSet: i32 = 41i32;
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess: i32 = 42i32;
pub const DXGI_MSG_IDXGISurface_Map_ReadFlagSetButCPUAccessIsDynamic: i32 = 43i32;
pub const DXGI_MSG_IDXGISurface_Map_DiscardFlagSetButCPUAccessIsNotDynamic: i32 = 44i32;
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_pNumModesIsNULL: i32 = 45i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasInvalidWidthOrHeight: i32 = 46i32;
pub const DXGI_MSG_IDXGIOutput_GetCammaControlCapabilities_NoOwnerDevice: i32 = 47i32;
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_pDeviceIsNULL: i32 = 48i32;
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_NoOwnerDevice: i32 = 49i32;
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_pDestinationIsNULL: i32 = 50i32;
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_MapOfDestinationFailed: i32 = 51i32;
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_NoOwnerDevice: i32 = 52i32;
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_pStatsIsNULL: i32 = 53i32;
pub const DXGI_MSG_IDXGIOutput_SetGammaControl_NoOwnerDevice: i32 = 54i32;
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoOwnerDevice: i32 = 55i32;
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoGammaControls: i32 = 56i32;
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_IDXGIResourceNotSupportedBypPrimary: i32 = 57i32;
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_pPrimaryIsInvalid: i32 = 58i32;
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_NoOwnerDevice: i32 = 59i32;
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteDeviceNotSupported: i32 = 60i32;
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteDeviceNotSupported: i32 = 61i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteDeviceNotSupported: i32 = 62i32;
pub const DXGI_MSG_IDXGIDevice_CreateSurface_InvalidParametersWithpSharedResource: i32 = 63i32;
pub const DXGI_MSG_IDXGIObject_GetPrivateData_puiDataSizeIsNULL: i32 = 64i32;
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidOutputWindow: i32 = 65i32;
pub const DXGI_MSG_IDXGISwapChain_Release_SwapChainIsFullscreen: i32 = 66i32;
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_InvalidTargetSurfaceFormat: i32 = 67i32;
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ModuleIsNULL: i32 = 68i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_IDXGIDeviceNotSupportedBypConcernedDevice: i32 = 69i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_pModeToMatchOrpClosestMatchIsNULL: i32 = 70i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasRefreshRateDenominatorZero: i32 = 71i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_UnknownFormatIsInvalidForConfiguration: i32 = 72i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScanlineOrdering: i32 = 73i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScaling: i32 = 74i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeFormatAndDeviceCombination: i32 = 75i32;
pub const DXGI_MSG_IDXGIFactory_Creation_CalledFromDllMain: i32 = 76i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_OutputNotOwnedBySwapChainDevice: i32 = 77i32;
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidWindowStyle: i32 = 78i32;
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_UnsupportedStatistics: i32 = 79i32;
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_SwapchainAdapterDoesNotControlOutput: i32 = 80i32;
pub const DXGI_MSG_IDXGIOutput_SetOrGetGammaControl_pArrayIsNULL: i32 = 81i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FullscreenInvalidForChildWindows: i32 = 82i32;
pub const DXGI_MSG_IDXGIFactory_Release_CalledFromDllMain: i32 = 83i32;
pub const DXGI_MSG_IDXGISwapChain_Present_UnreleasedHDC: i32 = 84i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_NonPreRotatedAndGDICompatibleFlags: i32 = 85i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedAndGDICompatibleFlags: i32 = 86i32;
pub const DXGI_MSG_IDXGISurface1_GetDC_pHdcIsNULL: i32 = 87i32;
pub const DXGI_MSG_IDXGISurface1_GetDC_SurfaceNotTexture2D: i32 = 88i32;
pub const DXGI_MSG_IDXGISurface1_GetDC_GDICompatibleFlagNotSet: i32 = 89i32;
pub const DXGI_MSG_IDXGISurface1_GetDC_UnreleasedHDC: i32 = 90i32;
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess2: i32 = 91i32;
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_GetDCNotCalled: i32 = 92i32;
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_InvalidRectangleDimensions: i32 = 93i32;
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteOutputNotSupported: i32 = 94i32;
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteOutputNotSupported: i32 = 95i32;
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteOutputNotSupported: i32 = 96i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDeviceHasMismatchedDXGIFactory: i32 = 97i32;
pub const DXGI_MSG_IDXGISwapChain_Present_NonOptimalFSConfiguration: i32 = 98i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSequentialNotSupportedOnD3D10: i32 = 99i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_BufferCountOOBForFlipSequential: i32 = 100i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFormatForFlipSequential: i32 = 101i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultiSamplingNotSupportedForFlipSequential: i32 = 102i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOBForFlipSequential: i32 = 103i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidFormatForFlipSequential: i32 = 104i32;
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationBeforeStandardPresentation: i32 = 105i32;
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenPartialPresentIsInvalid: i32 = 106i32;
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidPresentTestOrDoNotSequenceFlag: i32 = 107i32;
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollInfoWithNoDirtyRectsSpecified: i32 = 108i32;
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyScrollRect: i32 = 109i32;
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBounds: i32 = 110i32;
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBoundsWithOffset: i32 = 111i32;
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyDirtyRect: i32 = 112i32;
pub const DXGI_MSG_IDXGISwapChain_Present_DirtyRectOutOfBackbufferBounds: i32 = 113i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnsupportedBufferUsageFlags: i32 = 114i32;
pub const DXGI_MSG_IDXGISwapChain_Present_DoNotSequenceFlagSetButPreviousBufferIsUndefined: i32 = 115i32;
pub const DXGI_MSG_IDXGISwapChain_Present_UnsupportedFlags: i32 = 116i32;
pub const DXGI_MSG_IDXGISwapChain_Present_FlipModelChainMustResizeOrCreateOnFSTransition: i32 = 117i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pRestrictToOutputFromOtherIDXGIFactory: i32 = 118i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictOutputNotSupportedOnAdapter: i32 = 119i32;
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagSetButInvalidpRestrictToOutput: i32 = 120i32;
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagdWithFullscreen: i32 = 121i32;
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictOutputFlagWithStaleSwapChain: i32 = 122i32;
pub const DXGI_MSG_IDXGISwapChain_Present_OtherFlagsCausingInvalidPresentTestFlag: i32 = 123i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnavailableInSession0: i32 = 124i32;
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_UnavailableInSession0: i32 = 125i32;
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_UnavailableInSession0: i32 = 126i32;
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs_UnavailableInSession0: i32 = 127i32;
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_StereoDisabled: i32 = 128i32;
pub const DXGI_MSG_IDXGIFactory2_UnregisterStatus_CookieNotFound: i32 = 129i32;
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFSOrOverlay: i32 = 130i32;
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFlipSequential: i32 = 131i32;
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentWithRDPDriver: i32 = 132i32;
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity: i32 = 133i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_WidthOrHeightIsZero: i32 = 134i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_OnlyFlipSequentialSupported: i32 = 135i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnAdapter: i32 = 136i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnWindows7: i32 = 137i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSTransitionWithCompositionSwapChain: i32 = 138i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_InvalidWithCompositionSwapChain: i32 = 139i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_WidthOrHeightIsZero: i32 = 140i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneIsFlipModelOnly: i32 = 141i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingUnrecognized: i32 = 142i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyFullscreenUnsupported: i32 = 143i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyUnsupported: i32 = 144i32;
pub const DXGI_MSG_IDXGISwapChain_Present_RestartIsFullscreenOnly: i32 = 145i32;
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedWindowlessPresentationRequiresDisplayOnly: i32 = 146i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_DisplayOnlyUnsupported: i32 = 147i32;
pub const DXGI_MSG_IDXGISwapChain1_SetBackgroundColor_OutOfRange: i32 = 148i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyFullscreenUnsupported: i32 = 149i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyUnsupported: i32 = 150i32;
pub const DXGI_MSG_IDXGISwapchain_Present_ScrollUnsupported: i32 = 151i32;
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_UnsupportedOS: i32 = 152i32;
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_UnsupportedOS: i32 = 153i32;
pub const DXGI_MSG_IDXGISwapchain_Present_FullscreenRotation: i32 = 154i32;
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithMSAABuffers: i32 = 155i32;
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_FlipSequentialRequired: i32 = 156i32;
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_InvalidRotation: i32 = 157i32;
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_FlipSequentialRequired: i32 = 158i32;
pub const DXGI_MSG_IDXGISwapChain_GetHwnd_WrongType: i32 = 159i32;
pub const DXGI_MSG_IDXGISwapChain_GetCompositionSurface_WrongType: i32 = 160i32;
pub const DXGI_MSG_IDXGISwapChain_GetCoreWindow_WrongType: i32 = 161i32;
pub const DXGI_MSG_IDXGISwapChain_GetFullscreenDesc_NonHwnd: i32 = 162i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_CoreWindow: i32 = 163i32;
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_UnsupportedOnWindows7: i32 = 164i32;
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsNULL: i32 = 165i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FSUnsupportedForModernApps: i32 = 166i32;
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_ModernApp: i32 = 167i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_ModernApp: i32 = 168i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_pNewTargetParametersIsNULL: i32 = 169i32;
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_ModernApp: i32 = 170i32;
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_ModernApp: i32 = 171i32;
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsInvalid: i32 = 172i32;
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCompositionSurface_InvalidHandle: i32 = 173i32;
pub const DXGI_MSG_IDXGISurface1_GetDC_ModernApp: i32 = 174i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneRequiresWindows8OrNewer: i32 = 175i32;
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoAndPreferRight: i32 = 176i32;
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithDoNotSequence: i32 = 177i32;
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithoutStereo: i32 = 178i32;
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoUnsupported: i32 = 179i32;
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_ArraySizeMismatch: i32 = 180i32;
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithSwapEffectDiscard: i32 = 181i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaUnrecognized: i32 = 182i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsWindowlessOnly: i32 = 183i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsFlipModelOnly: i32 = 184i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictToOutputAdapterMismatch: i32 = 185i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyOnLegacy: i32 = 186i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyOnLegacy: i32 = 187i32;
pub const DXGI_MSG_IDXGIResource1_CreateSubresourceSurface_InvalidIndex: i32 = 188i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidScaling: i32 = 189i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForCoreWindow_InvalidSwapEffect: i32 = 190i32;
pub const DXGI_MSG_IDXGIResource1_CreateSharedHandle_UnsupportedOS: i32 = 191i32;
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusWindow_UnsupportedOS: i32 = 192i32;
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusEvent_UnsupportedOS: i32 = 193i32;
pub const DXGI_MSG_IDXGIOutput1_DuplicateOutput_UnsupportedOS: i32 = 194i32;
pub const DXGI_MSG_IDXGIDisplayControl_IsStereoEnabled_UnsupportedOS: i32 = 195i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidAlphaMode: i32 = 196i32;
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidResource: i32 = 197i32;
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidLUID: i32 = 198i32;
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_UnsupportedOS: i32 = 199i32;
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_2DOnly: i32 = 200i32;
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_StagingOnly: i32 = 201i32;
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NeedCPUAccessWrite: i32 = 202i32;
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NoShared: i32 = 203i32;
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_OnlyMipLevels1: i32 = 204i32;
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_MappedOrOfferedResource: i32 = 205i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSUnsupportedForModernApps: i32 = 206i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FailedToGoFSButNonPreRotated: i32 = 207i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainOrRegisterOcclusionStatus_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: i32 = 208i32;
pub const DXGI_MSG_IDXGISwapChain_Present_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: i32 = 209i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreFlipModelOnly: i32 = 210i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreNotFullscreen: i32 = 211i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_Waitable: i32 = 212i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveWaitableFlag: i32 = 213i32;
pub const DXGI_MSG_IDXGISwapChain_GetFrameLatencyWaitableObject_OnlyWaitable: i32 = 214i32;
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_OnlyWaitable: i32 = 215i32;
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_pMaxLatencyIsNULL: i32 = 216i32;
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_OnlyWaitable: i32 = 217i32;
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_MaxLatencyIsOutOfBounds: i32 = 218i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ForegroundIsCoreWindowOnly: i32 = 219i32;
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_ForegroundUnsupportedOnAdapter: i32 = 220i32;
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidScaling: i32 = 221i32;
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidAlphaMode: i32 = 222i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveForegroundFlag: i32 = 223i32;
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixPointerCannotBeNull: i32 = 224i32;
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_RequiresCompositionSwapChain: i32 = 225i32;
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeFinite: i32 = 226i32;
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeTranslateAndOrScale: i32 = 227i32;
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_MatrixPointerCannotBeNull: i32 = 228i32;
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_RequiresCompositionSwapChain: i32 = 229i32;
pub const DXGI_MSG_DXGIGetDebugInterface1_NULL_ppDebug: i32 = 230i32;
pub const DXGI_MSG_DXGIGetDebugInterface1_InvalidFlags: i32 = 231i32;
pub const DXGI_MSG_IDXGISwapChain_Present_Decode: i32 = 232i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Decode: i32 = 233i32;
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_FlipModel: i32 = 234i32;
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_Decode: i32 = 235i32;
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_WidthHeight: i32 = 236i32;
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_NullPointers: i32 = 237i32;
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_Decode: i32 = 238i32;
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetColorSpace_InvalidFlags: i32 = 239i32;
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetSourceRect_InvalidRect: i32 = 240i32;
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetTargetRect_InvalidRect: i32 = 241i32;
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetDestSize_InvalidSize: i32 = 242i32;
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetSourceRect_InvalidPointer: i32 = 243i32;
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetTargetRect_InvalidPointer: i32 = 244i32;
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetDestSize_InvalidPointer: i32 = 245i32;
pub const DXGI_MSG_IDXGISwapChain_PresentBuffer_YUV: i32 = 246i32;
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_YUV: i32 = 247i32;
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_YUV: i32 = 248i32;
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_YUV: i32 = 249i32;
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_YUV: i32 = 250i32;
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentation_YUV: i32 = 251i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveFlag_YUV: i32 = 252i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Alignment_YUV: i32 = 253i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ShaderInputUnsupported_YUV: i32 = 254i32;
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_NullPointers: i32 = 255i32;
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_IDXGIDeviceNotSupportedBypConcernedDevice: i32 = 256i32;
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs2_InvalidEnumOutputs2Flag: i32 = 257i32;
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_FSUnsupportedForFlipDiscard: i32 = 258i32;
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_NullPointers: i32 = 259i32;
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_IDXGIDeviceNotSupportedBypConcernedDevice: i32 = 260i32;
pub const DXGI_MSG_IDXGISwapChain3_CheckColorSpaceSupport_NullPointers: i32 = 261i32;
pub const DXGI_MSG_IDXGISwapChain3_SetColorSpace1_InvalidColorSpace: i32 = 262i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidHwProtect: i32 = 263i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_HwProtectUnsupported: i32 = 264i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtect: i32 = 265i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_HwProtectUnsupported: i32 = 266i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_D3D12Only: i32 = 267i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_FlipModel: i32 = 268i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_NodeMaskAndQueueRequired: i32 = 269i32;
pub const DXGI_MSG_IDXGISwapChain_CreateSwapChain_InvalidHwProtectGdiFlag: i32 = 270i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtectGdiFlag: i32 = 271i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_10BitFormatNotSupported: i32 = 272i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSwapEffectRequired: i32 = 273i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidDevice: i32 = 274i32;
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_Unsupported: i32 = 275i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidQueue: i32 = 276i32;
pub const DXGI_MSG_IDXGISwapChain3_ResizeBuffers1_InvalidQueue: i32 = 277i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForHwnd_InvalidScaling: i32 = 278i32;
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidSize: i32 = 279i32;
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidPointer: i32 = 280i32;
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidType: i32 = 281i32;
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenAllowTearingIsInvalid: i32 = 282i32;
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresPresentIntervalZero: i32 = 283i32;
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresCreationFlag: i32 = 284i32;
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveAllowTearingFlag: i32 = 285i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AllowTearingFlagIsFlipModelOnly: i32 = 286i32;
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidFeature: i32 = 287i32;
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidSize: i32 = 288i32;
pub const DXGI_MSG_IDXGIOutput6_CheckHardwareCompositionSupport_NullPointer: i32 = 289i32;
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_PerMonitorDpiShimApplied: i32 = 290i32;
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput_PerMonitorDpiShimApplied: i32 = 291i32;
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput1_PerMonitorDpiRequired: i32 = 292i32;
pub const DXGI_MSG_IDXGIFactory7_UnregisterAdaptersChangedEvent_CookieNotFound: i32 = 293i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_LegacyBltModelSwapEffect: i32 = 294i32;
pub const DXGI_MSG_IDXGISwapChain4_SetHDRMetaData_MetadataUnchanged: i32 = 295i32;
pub const DXGI_MSG_IDXGISwapChain_Present_11On12_Released_Resource: i32 = 296i32;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultipleSwapchainRefToSurface_DeferredDtr: i32 = 297i32;
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_NoOpBehavior: i32 = 298i32;
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow: i32 = 1000i32;
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_DISCARD_BufferCount: i32 = 1001i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_SetFullscreenState_NotAvailable: i32 = 1002i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeBuffers_NotAvailable: i32 = 1003i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeTarget_NotAvailable: i32 = 1004i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerIndex: i32 = 1005i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleLayerIndex: i32 = 1006i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerFlag: i32 = 1007i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidRotation: i32 = 1008i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidBlend: i32 = 1009i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidResource: i32 = 1010i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidMultiPlaneOverlayResource: i32 = 1011i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForPrimary: i32 = 1012i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForOverlay: i32 = 1013i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSubResourceIndex: i32 = 1014i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSourceRect: i32 = 1015i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidDestinationRect: i32 = 1016i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleResource: i32 = 1017i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_NotSharedResource: i32 = 1018i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidFlag: i32 = 1019i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidInterval: i32 = 1020i32;
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_MSAA_NotSupported: i32 = 1021i32;
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_ScalingAspectRatioStretch_Supported_ModernApp: i32 = 1022i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_GetFrameStatistics_NotAvailable_ModernApp: i32 = 1023i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_ReplaceInterval0With1: i32 = 1024i32;
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FailedRegisterWithCompositor: i32 = 1025i32;
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow_AtRendering: i32 = 1026i32;
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_SEQUENTIAL_BufferCount: i32 = 1027i32;
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_Modern_CoreWindow_Only: i32 = 1028i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_Present1_RequiresOverlays: i32 = 1029i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_SetBackgroundColor_FlipSequentialRequired: i32 = 1030i32;
pub const DXGI_MSG_Phone_IDXGISwapChain_GetBackgroundColor_FlipSequentialRequired: i32 = 1031i32;
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: i32 = 1i32;
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: i32 = 1i32;
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: i32 = 2i32;
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: i32 = 3i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: Common::DXGI_MODE_DESC,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_OUTDUPL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_OUTDUPL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: super::super::Foundation::BOOL,
    pub ProtectedContentMaskedOut: super::super::Foundation::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_FRAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::super::Foundation::POINT,
    pub DestinationRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_MOVE_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::super::Foundation::POINT,
    pub Visible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: i32 = 1i32;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: i32 = 2i32;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: i32 = 4i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: Common::DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: i32 = 1i32;
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: i32 = 1i32;
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: i32 = 2i32;
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512u32;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2u32;
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::super::Foundation::RECT,
    pub pScrollRect: *mut super::super::Foundation::RECT,
    pub pScrollOffset: *mut super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_PRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_PRESENT_RESTART: u32 = 4u32;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64u32;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16u32;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32u32;
pub const DXGI_PRESENT_TEST: u32 = 1u32;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256u32;
#[repr(C)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
impl ::core::marker::Copy for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::clone::Clone for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: i32 = 0i32;
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: i32 = 1i32;
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: i32 = 2i32;
pub const DXGI_RESIDENCY_FULLY_RESIDENT: i32 = 1i32;
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: i32 = 2i32;
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: i32 = 3i32;
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640u32;
pub const DXGI_RESOURCE_PRIORITY_LOW: u32 = 1342177280u32;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920u32;
pub const DXGI_RESOURCE_PRIORITY_HIGH: u32 = 2684354560u32;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200u32;
#[repr(C)]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for DXGI_RGBA {}
impl ::core::clone::Clone for DXGI_RGBA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_SCALING_STRETCH: i32 = 0i32;
pub const DXGI_SCALING_NONE: i32 = 1i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_SHARED_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648u32;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_SURFACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: i32 = 1i32;
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: i32 = 2i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: Common::DXGI_MODE_DESC,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub OutputWindow: super::super::Foundation::HWND,
    pub Windowed: super::super::Foundation::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub Stereo: super::super::Foundation::BOOL,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: Common::DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: i32 = 1i32;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: i32 = 2i32;
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: i32 = 4i32;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: i32 = 8i32;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: i32 = 16i32;
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: i32 = 32i32;
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: i32 = 64i32;
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: i32 = 128i32;
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: i32 = 256i32;
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: i32 = 512i32;
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: i32 = 1024i32;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: i32 = 2048i32;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: i32 = 4096i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Windowed: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_SWAP_EFFECT_DISCARD: i32 = 0i32;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: i32 = 1i32;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: i32 = 3i32;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: i32 = 4i32;
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64u32;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512u32;
pub const DXGI_USAGE_READ_ONLY: u32 = 256u32;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32u32;
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16u32;
pub const DXGI_USAGE_SHARED: u32 = 128u32;
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024u32;
#[repr(transparent)]
pub struct IDXGIAdapter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIAdapter {}
impl ::core::clone::Clone for IDXGIAdapter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIAdapter1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIAdapter1 {}
impl ::core::clone::Clone for IDXGIAdapter1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIAdapter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIAdapter2 {}
impl ::core::clone::Clone for IDXGIAdapter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIAdapter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIAdapter3 {}
impl ::core::clone::Clone for IDXGIAdapter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIAdapter4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIAdapter4 {}
impl ::core::clone::Clone for IDXGIAdapter4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDebug(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDebug {}
impl ::core::clone::Clone for IDXGIDebug {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDebug1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDebug1 {}
impl ::core::clone::Clone for IDXGIDebug1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDecodeSwapChain(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDecodeSwapChain {}
impl ::core::clone::Clone for IDXGIDecodeSwapChain {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDevice {}
impl ::core::clone::Clone for IDXGIDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDevice1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDevice1 {}
impl ::core::clone::Clone for IDXGIDevice1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDevice2 {}
impl ::core::clone::Clone for IDXGIDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDevice3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDevice3 {}
impl ::core::clone::Clone for IDXGIDevice3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDevice4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDevice4 {}
impl ::core::clone::Clone for IDXGIDevice4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDeviceSubObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDeviceSubObject {}
impl ::core::clone::Clone for IDXGIDeviceSubObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIDisplayControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIDisplayControl {}
impl ::core::clone::Clone for IDXGIDisplayControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactory {}
impl ::core::clone::Clone for IDXGIFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactory1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactory1 {}
impl ::core::clone::Clone for IDXGIFactory1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactory2 {}
impl ::core::clone::Clone for IDXGIFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactory3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactory3 {}
impl ::core::clone::Clone for IDXGIFactory3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactory4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactory4 {}
impl ::core::clone::Clone for IDXGIFactory4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactory5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactory5 {}
impl ::core::clone::Clone for IDXGIFactory5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactory6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactory6 {}
impl ::core::clone::Clone for IDXGIFactory6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactory7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactory7 {}
impl ::core::clone::Clone for IDXGIFactory7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIFactoryMedia(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIFactoryMedia {}
impl ::core::clone::Clone for IDXGIFactoryMedia {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIInfoQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIInfoQueue {}
impl ::core::clone::Clone for IDXGIInfoQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIKeyedMutex(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIKeyedMutex {}
impl ::core::clone::Clone for IDXGIKeyedMutex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIObject {}
impl ::core::clone::Clone for IDXGIObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIOutput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIOutput {}
impl ::core::clone::Clone for IDXGIOutput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIOutput1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIOutput1 {}
impl ::core::clone::Clone for IDXGIOutput1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIOutput2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIOutput2 {}
impl ::core::clone::Clone for IDXGIOutput2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIOutput3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIOutput3 {}
impl ::core::clone::Clone for IDXGIOutput3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIOutput4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIOutput4 {}
impl ::core::clone::Clone for IDXGIOutput4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIOutput5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIOutput5 {}
impl ::core::clone::Clone for IDXGIOutput5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIOutput6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIOutput6 {}
impl ::core::clone::Clone for IDXGIOutput6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIOutputDuplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIOutputDuplication {}
impl ::core::clone::Clone for IDXGIOutputDuplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIResource {}
impl ::core::clone::Clone for IDXGIResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGIResource1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGIResource1 {}
impl ::core::clone::Clone for IDXGIResource1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISurface {}
impl ::core::clone::Clone for IDXGISurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISurface1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISurface1 {}
impl ::core::clone::Clone for IDXGISurface1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISurface2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISurface2 {}
impl ::core::clone::Clone for IDXGISurface2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISwapChain(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISwapChain {}
impl ::core::clone::Clone for IDXGISwapChain {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISwapChain1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISwapChain1 {}
impl ::core::clone::Clone for IDXGISwapChain1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISwapChain2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISwapChain2 {}
impl ::core::clone::Clone for IDXGISwapChain2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISwapChain3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISwapChain3 {}
impl ::core::clone::Clone for IDXGISwapChain3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISwapChain4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISwapChain4 {}
impl ::core::clone::Clone for IDXGISwapChain4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGISwapChainMedia(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGISwapChainMedia {}
impl ::core::clone::Clone for IDXGISwapChainMedia {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXGraphicsAnalysis(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXGraphicsAnalysis {}
impl ::core::clone::Clone for IDXGraphicsAnalysis {
    fn clone(&self) -> Self {
        *self
    }
}
