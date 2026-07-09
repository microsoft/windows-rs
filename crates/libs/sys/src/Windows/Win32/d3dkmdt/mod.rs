pub const D3DKARG_FENCESTORAGETYPE_CURRENTVALUE: DXGKARG_FENCESTORAGEVALUETYPE = 0;
pub const D3DKARG_FENCESTORAGETYPE_MONITOREDVALUE: DXGKARG_FENCESTORAGEVALUETYPE = 1;
pub type D3DKMDT_2DOFFSET = D3DKMDT_2DREGION;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_2DREGION {
    pub cx: u32,
    pub cy: u32,
}
pub type D3DKMDT_ADAPTER = *mut core::ffi::c_void;
pub const D3DKMDT_BITS_PER_COMPONENT_06: u32 = 1;
pub const D3DKMDT_BITS_PER_COMPONENT_08: u32 = 2;
pub const D3DKMDT_BITS_PER_COMPONENT_10: u32 = 4;
pub const D3DKMDT_BITS_PER_COMPONENT_12: u32 = 8;
pub const D3DKMDT_BITS_PER_COMPONENT_14: u32 = 16;
pub const D3DKMDT_BITS_PER_COMPONENT_16: u32 = 32;
pub const D3DKMDT_CB_INTENSITY: D3DKMDT_COLOR_BASIS = 1;
pub const D3DKMDT_CB_SCRGB: D3DKMDT_COLOR_BASIS = 3;
pub const D3DKMDT_CB_SRGB: D3DKMDT_COLOR_BASIS = 2;
pub const D3DKMDT_CB_UNINITIALIZED: D3DKMDT_COLOR_BASIS = 0;
pub const D3DKMDT_CB_YCBCR: D3DKMDT_COLOR_BASIS = 4;
pub const D3DKMDT_CB_YPBPR: D3DKMDT_COLOR_BASIS = 5;
pub type D3DKMDT_COLOR_BASIS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_COLOR_COEFF_DYNAMIC_RANGES {
    pub FirstChannel: u32,
    pub SecondChannel: u32,
    pub ThirdChannel: u32,
    pub FourthChannel: u32,
}
pub const D3DKMDT_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 200;
pub const D3DKMDT_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 100;
pub type D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = i32;
pub const D3DKMDT_COMPUTE_PREEMPTION_NONE: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 0;
pub const D3DKMDT_COMPUTE_PREEMPTION_SHADER_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 500;
pub const D3DKMDT_COMPUTE_PREEMPTION_THREAD_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 400;
pub const D3DKMDT_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 300;
pub const D3DKMDT_DIMENSION_NOTSPECIFIED: i32 = -2;
pub const D3DKMDT_DIMENSION_UNINITIALIZED: i32 = -1;
pub type D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = i32;
pub const D3DKMDT_EPT_NOPIVOT: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 5;
pub const D3DKMDT_EPT_ROTATION: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 4;
pub const D3DKMDT_EPT_SCALING: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 3;
pub const D3DKMDT_EPT_UNINITIALIZED: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 0;
pub const D3DKMDT_EPT_VIDPNSOURCE: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 1;
pub const D3DKMDT_EPT_VIDPNTARGET: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 2;
pub const D3DKMDT_FREQUENCY_NOTSPECIFIED: i32 = -2;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_FREQUENCY_RANGE {
    pub MinVSyncFreq: super::d3dukmdt::D3DDDI_RATIONAL,
    pub MaxVSyncFreq: super::d3dukmdt::D3DDDI_RATIONAL,
    pub MinHSyncFreq: super::d3dukmdt::D3DDDI_RATIONAL,
    pub MaxHSyncFreq: super::d3dukmdt::D3DDDI_RATIONAL,
}
pub const D3DKMDT_FREQUENCY_UNINITIALIZED: i32 = -1;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMDT_GAMMA_RAMP {
    pub Type: super::d3dukmdt::D3DDDI_GAMMARAMP_TYPE,
    pub DataSize: usize,
    pub Data: D3DKMDT_GAMMA_RAMP_0,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_GAMMA_RAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMDT_GAMMA_RAMP_0 {
    pub pRgb256x3x16: *mut super::d3dukmdt::D3DDDI_GAMMA_RAMP_RGB256x3x16,
    pub pDxgi1: *mut super::d3dukmdt::D3DDDI_GAMMA_RAMP_DXGI_1,
    pub p3x4: *mut super::d3dukmdt::D3DKMDT_3x4_COLORSPACE_TRANSFORM,
    pub pMatrixV2: *mut super::d3dukmdt::D3DKMDT_COLORSPACE_TRANSFORM_MATRIX_V2,
    pub pRaw: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_GAMMA_RAMP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMDT_GDISURFACEDATA {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::d3dukmdt::D3DDDIFORMAT,
    pub Type: D3DKMDT_GDISURFACETYPE,
    pub Flags: D3DKMDT_GDISURFACEFLAGS,
    pub Pitch: u32,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_GDISURFACEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_GDISURFACEFLAGS {
    pub Anonymous: D3DKMDT_GDISURFACEFLAGS_0,
}
impl Default for D3DKMDT_GDISURFACEFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_GDISURFACEFLAGS_0 {
    pub Anonymous: D3DKMDT_GDISURFACEFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMDT_GDISURFACEFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_GDISURFACEFLAGS_0_0 {
    pub _bitfield: u32,
}
pub type D3DKMDT_GDISURFACETYPE = i32;
pub const D3DKMDT_GDISURFACE_EXISTINGSYSMEM: D3DKMDT_GDISURFACETYPE = 5;
pub const D3DKMDT_GDISURFACE_INVALID: D3DKMDT_GDISURFACETYPE = 0;
pub const D3DKMDT_GDISURFACE_LOOKUPTABLE: D3DKMDT_GDISURFACETYPE = 4;
pub const D3DKMDT_GDISURFACE_STAGING: D3DKMDT_GDISURFACETYPE = 3;
pub const D3DKMDT_GDISURFACE_STAGING_CPUVISIBLE: D3DKMDT_GDISURFACETYPE = 2;
pub const D3DKMDT_GDISURFACE_TEXTURE: D3DKMDT_GDISURFACETYPE = 1;
pub const D3DKMDT_GDISURFACE_TEXTURE_CPUVISIBLE: D3DKMDT_GDISURFACETYPE = 6;
pub const D3DKMDT_GDISURFACE_TEXTURE_CPUVISIBLE_CROSSADAPTER: D3DKMDT_GDISURFACETYPE = 8;
pub const D3DKMDT_GDISURFACE_TEXTURE_CROSSADAPTER: D3DKMDT_GDISURFACETYPE = 7;
pub const D3DKMDT_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 100;
pub type D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = i32;
pub const D3DKMDT_GRAPHICS_PREEMPTION_NONE: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 0;
pub const D3DKMDT_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 400;
pub const D3DKMDT_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 200;
pub const D3DKMDT_GRAPHICS_PREEMPTION_SHADER_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 500;
pub const D3DKMDT_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 300;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_GRAPHICS_RENDERING_FORMAT {
    pub PrimSurfSize: D3DKMDT_2DREGION,
    pub VisibleRegionSize: D3DKMDT_2DREGION,
    pub Stride: u32,
    pub PixelFormat: super::d3dukmdt::D3DDDIFORMAT,
    pub ColorBasis: D3DKMDT_COLOR_BASIS,
    pub PixelValueAccessMode: D3DKMDT_PIXEL_VALUE_ACCESS_MODE,
}
pub type D3DKMDT_GTFCOMPLIANCE = i32;
pub const D3DKMDT_GTF_COMPLIANT: D3DKMDT_GTFCOMPLIANCE = 1;
pub const D3DKMDT_GTF_NOTCOMPLIANT: D3DKMDT_GTFCOMPLIANCE = 2;
pub const D3DKMDT_GTF_UNINITIALIZED: D3DKMDT_GTFCOMPLIANCE = 0;
pub type D3DKMDT_HMONITORDESCRIPTORSET = *mut core::ffi::c_void;
pub type D3DKMDT_HMONITORFREQUENCYRANGESET = *mut core::ffi::c_void;
pub type D3DKMDT_HMONITORSOURCEMODESET = *mut core::ffi::c_void;
pub type D3DKMDT_HVIDEOPRESENTSOURCESET = *mut core::ffi::c_void;
pub type D3DKMDT_HVIDEOPRESENTTARGETSET = *mut core::ffi::c_void;
pub type D3DKMDT_HVIDPN = *mut core::ffi::c_void;
pub type D3DKMDT_HVIDPNSOURCEMODESET = *mut core::ffi::c_void;
pub type D3DKMDT_HVIDPNTARGETMODESET = *mut core::ffi::c_void;
pub type D3DKMDT_HVIDPNTOPOLOGY = *mut core::ffi::c_void;
pub const D3DKMDT_MACROVISION_OEMCOPYPROTECTION_SIZE: u32 = 256;
pub const D3DKMDT_MAX_OVERLAYS: u32 = 4;
pub const D3DKMDT_MAX_OVERLAYS_BITCOUNT: u32 = 2;
pub const D3DKMDT_MCC_ENFORCE: D3DKMDT_MONITOR_CONNECTIVITY_CHECKS = 2;
pub const D3DKMDT_MCC_IGNORE: D3DKMDT_MONITOR_CONNECTIVITY_CHECKS = 1;
pub const D3DKMDT_MCC_UNINITIALIZED: D3DKMDT_MONITOR_CONNECTIVITY_CHECKS = 0;
pub const D3DKMDT_MCO_DEFAULTMONITORPROFILE: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 1;
pub const D3DKMDT_MCO_DRIVER: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 5;
pub const D3DKMDT_MCO_MONITORDESCRIPTOR: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 2;
pub const D3DKMDT_MCO_MONITORDESCRIPTOR_REGISTRYOVERRIDE: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 3;
pub const D3DKMDT_MCO_SPECIFICCAP_REGISTRYOVERRIDE: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 4;
pub const D3DKMDT_MCO_UNINITIALIZED: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 0;
pub const D3DKMDT_MDT_OTHER: D3DKMDT_MONITOR_DESCRIPTOR_TYPE = 255;
pub const D3DKMDT_MDT_UNINITIALIZED: D3DKMDT_MONITOR_DESCRIPTOR_TYPE = 0;
pub const D3DKMDT_MDT_VESA_EDID_V1_BASEBLOCK: D3DKMDT_MONITOR_DESCRIPTOR_TYPE = 1;
pub const D3DKMDT_MDT_VESA_EDID_V1_BLOCKMAP: D3DKMDT_MONITOR_DESCRIPTOR_TYPE = 2;
pub const D3DKMDT_MFRC_ACTIVESIZE: D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT = 1;
pub const D3DKMDT_MFRC_MAXPIXELRATE: D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT = 2;
pub const D3DKMDT_MFRC_UNINITIALIZED: D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT = 0;
pub const D3DKMDT_MOA_INTERRUPTIBLE: D3DKMDT_MONITOR_ORIENTATION_AWARENESS = 3;
pub const D3DKMDT_MOA_NONE: D3DKMDT_MONITOR_ORIENTATION_AWARENESS = 1;
pub const D3DKMDT_MOA_POLLED: D3DKMDT_MONITOR_ORIENTATION_AWARENESS = 2;
pub const D3DKMDT_MOA_UNINITIALIZED: D3DKMDT_MONITOR_ORIENTATION_AWARENESS = 0;
pub type D3DKMDT_MODE_PREFERENCE = i32;
pub type D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = i32;
pub type D3DKMDT_MONITOR_CONNECTIVITY_CHECKS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_MONITOR_DESCRIPTOR {
    pub Id: D3DKMDT_MONITOR_DESCRIPTOR_ID,
    pub Type: D3DKMDT_MONITOR_DESCRIPTOR_TYPE,
    pub DataSize: usize,
    pub pData: *mut core::ffi::c_void,
    pub Origin: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN,
}
impl Default for D3DKMDT_MONITOR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMDT_MONITOR_DESCRIPTOR_ID = u32;
pub type D3DKMDT_MONITOR_DESCRIPTOR_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMDT_MONITOR_FREQUENCY_RANGE {
    pub Origin: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN,
    pub RangeLimits: D3DKMDT_FREQUENCY_RANGE,
    pub ConstraintType: D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT,
    pub Constraint: D3DKMDT_MONITOR_FREQUENCY_RANGE_0,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_MONITOR_FREQUENCY_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMDT_MONITOR_FREQUENCY_RANGE_0 {
    pub ActiveSize: D3DKMDT_2DREGION,
    pub MaxPixelRate: usize,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_MONITOR_FREQUENCY_RANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT = i32;
pub type D3DKMDT_MONITOR_ORIENTATION = i32;
pub type D3DKMDT_MONITOR_ORIENTATION_AWARENESS = i32;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMDT_MONITOR_SOURCE_MODE {
    pub Id: D3DKMDT_MONITOR_SOURCE_MODE_ID,
    pub VideoSignalInfo: D3DKMDT_VIDEO_SIGNAL_INFO,
    pub ColorBasis: D3DKMDT_COLOR_BASIS,
    pub ColorCoeffDynamicRanges: D3DKMDT_COLOR_COEFF_DYNAMIC_RANGES,
    pub Origin: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN,
    pub Preference: D3DKMDT_MODE_PREFERENCE,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_MONITOR_SOURCE_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMDT_MONITOR_SOURCE_MODE_ID = u32;
pub type D3DKMDT_MONITOR_TIMING_TYPE = i32;
pub const D3DKMDT_MO_0DEG: D3DKMDT_MONITOR_ORIENTATION = 1;
pub const D3DKMDT_MO_180DEG: D3DKMDT_MONITOR_ORIENTATION = 3;
pub const D3DKMDT_MO_270DEG: D3DKMDT_MONITOR_ORIENTATION = 4;
pub const D3DKMDT_MO_90DEG: D3DKMDT_MONITOR_ORIENTATION = 2;
pub const D3DKMDT_MO_UNINITIALIZED: D3DKMDT_MONITOR_ORIENTATION = 0;
pub const D3DKMDT_MP_NOTPREFERRED: D3DKMDT_MODE_PREFERENCE = 2;
pub const D3DKMDT_MP_PREFERRED: D3DKMDT_MODE_PREFERENCE = 1;
pub const D3DKMDT_MP_UNINITIALIZED: D3DKMDT_MODE_PREFERENCE = 0;
pub const D3DKMDT_MTT_DEFAULTMONITORPROFILE: D3DKMDT_MONITOR_TIMING_TYPE = 5;
pub const D3DKMDT_MTT_DETAILED: D3DKMDT_MONITOR_TIMING_TYPE = 4;
pub const D3DKMDT_MTT_DRIVER: D3DKMDT_MONITOR_TIMING_TYPE = 6;
pub const D3DKMDT_MTT_ESTABLISHED: D3DKMDT_MONITOR_TIMING_TYPE = 1;
pub const D3DKMDT_MTT_EXTRASTANDARD: D3DKMDT_MONITOR_TIMING_TYPE = 3;
pub const D3DKMDT_MTT_STANDARD: D3DKMDT_MONITOR_TIMING_TYPE = 2;
pub const D3DKMDT_MTT_UNINITIALIZED: D3DKMDT_MONITOR_TIMING_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_PALETTEDATA {
    pub Red: u8,
    pub Green: u8,
    pub Blue: u8,
    pub Unused: u8,
}
pub type D3DKMDT_PIXEL_VALUE_ACCESS_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_PREEMPTION_CAPS {
    pub GraphicsPreemptionGranularity: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY,
}
pub const D3DKMDT_PVAM_DIRECT: D3DKMDT_PIXEL_VALUE_ACCESS_MODE = 1;
pub const D3DKMDT_PVAM_PRESETPALETTE: D3DKMDT_PIXEL_VALUE_ACCESS_MODE = 2;
pub const D3DKMDT_PVAM_SETTABLEPALETTE: D3DKMDT_PIXEL_VALUE_ACCESS_MODE = 3;
pub const D3DKMDT_PVAM_UNINITIALIZED: D3DKMDT_PIXEL_VALUE_ACCESS_MODE = 0;
pub const D3DKMDT_RMT_GRAPHICS: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 1;
pub const D3DKMDT_RMT_GRAPHICS_STEREO: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 3;
pub const D3DKMDT_RMT_GRAPHICS_STEREO_ADVANCED_SCAN: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 4;
pub const D3DKMDT_RMT_TEXT: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 2;
pub const D3DKMDT_RMT_UNINITIALIZED: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 0;
pub const D3DKMDT_ROTATION_SUPPORT_MASK: u32 = 255;
pub const D3DKMDT_SCALING_SUPPORT_MASK: u32 = 31;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_SHADOWSURFACEDATA {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::d3dukmdt::D3DDDIFORMAT,
    pub Pitch: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_SHAREDPRIMARYSURFACEDATA {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::d3dukmdt::D3DDDIFORMAT,
    pub RefreshRate: super::d3dukmdt::D3DDDI_RATIONAL,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_STAGINGSURFACEDATA {
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
}
pub const D3DKMDT_STANDARDALLOCATION_FENCESTORAGE: D3DKMDT_STANDARDALLOCATION_TYPE = 6;
pub const D3DKMDT_STANDARDALLOCATION_GDISURFACE: D3DKMDT_STANDARDALLOCATION_TYPE = 4;
pub const D3DKMDT_STANDARDALLOCATION_SHADOWSURFACE: D3DKMDT_STANDARDALLOCATION_TYPE = 2;
pub const D3DKMDT_STANDARDALLOCATION_SHAREDPRIMARYSURFACE: D3DKMDT_STANDARDALLOCATION_TYPE = 1;
pub const D3DKMDT_STANDARDALLOCATION_STAGINGSURFACE: D3DKMDT_STANDARDALLOCATION_TYPE = 3;
pub type D3DKMDT_STANDARDALLOCATION_TYPE = i32;
pub const D3DKMDT_STANDARDALLOCATION_VGPU: D3DKMDT_STANDARDALLOCATION_TYPE = 5;
pub type D3DKMDT_TEXT_RENDERING_FORMAT = i32;
pub const D3DKMDT_TRF_UNINITIALIZED: D3DKMDT_TEXT_RENDERING_FORMAT = 0;
pub type D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = i32;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDEO_PRESENT_SOURCE {
    pub Id: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub dwReserved: u32,
}
pub type D3DKMDT_VIDEO_PRESENT_SOURCE_MODE_ID = u32;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDEO_PRESENT_TARGET {
    pub Id: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_TARGET_ID,
    pub VideoOutputTechnology: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY,
    pub VideoOutputHpdAwareness: DXGK_CHILD_DEVICE_HPD_AWARENESS,
    pub MonitorOrientationAwareness: D3DKMDT_MONITOR_ORIENTATION_AWARENESS,
    pub SupportsSdtvModes: bool,
}
pub type D3DKMDT_VIDEO_PRESENT_TARGET_MODE_ID = u32;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDEO_SIGNAL_INFO {
    pub VideoStandard: D3DKMDT_VIDEO_SIGNAL_STANDARD,
    pub TotalSize: D3DKMDT_2DREGION,
    pub ActiveSize: D3DKMDT_2DREGION,
    pub VSyncFreq: super::d3dukmdt::D3DDDI_RATIONAL,
    pub HSyncFreq: super::d3dukmdt::D3DDDI_RATIONAL,
    pub PixelRate: usize,
    pub Anonymous: D3DKMDT_VIDEO_SIGNAL_INFO_0,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_VIDEO_SIGNAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMDT_VIDEO_SIGNAL_INFO_0 {
    pub AdditionalSignalInfo: D3DKMDT_VIDEO_SIGNAL_INFO_0_0,
    pub ScanLineOrdering: super::d3dukmdt::D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_VIDEO_SIGNAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDEO_SIGNAL_INFO_0_0 {
    pub _bitfield: super::d3dukmdt::D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING,
}
pub type D3DKMDT_VIDEO_SIGNAL_STANDARD = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDPN_HW_CAPABILITY {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub VidPnTargetId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_TARGET_ID,
    pub ImportanceOrdinal: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE,
    pub ContentTransformation: D3DKMDT_VIDPN_PRESENT_PATH_TRANSFORMATION,
    pub VisibleFromActiveTLOffset: D3DKMDT_2DOFFSET,
    pub VisibleFromActiveBROffset: D3DKMDT_2DOFFSET,
    pub VidPnTargetColorBasis: D3DKMDT_COLOR_BASIS,
    pub VidPnTargetColorCoeffDynamicRanges: D3DKMDT_COLOR_COEFF_DYNAMIC_RANGES,
    pub Content: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT,
    pub CopyProtection: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION,
    pub GammaRamp: D3DKMDT_GAMMA_RAMP,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_VIDPN_PRESENT_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION {
    pub CopyProtectionType: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE,
    pub APSTriggerBits: u32,
    pub OEMCopyProtection: [u8; 256],
    pub CopyProtectionSupport: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_SUPPORT,
}
impl Default for D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_SUPPORT {
    pub _bitfield: u32,
}
pub type D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = i32;
pub type D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = i32;
pub type D3DKMDT_VIDPN_PRESENT_PATH_INDEX = usize;
pub type D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_ROTATION_SUPPORT {
    pub _bitfield: u32,
}
pub type D3DKMDT_VIDPN_PRESENT_PATH_SCALING = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_SCALING_SUPPORT {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_TRANSFORMATION {
    pub Scaling: D3DKMDT_VIDPN_PRESENT_PATH_SCALING,
    pub ScalingSupport: D3DKMDT_VIDPN_PRESENT_PATH_SCALING_SUPPORT,
    pub Rotation: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION,
    pub RotationSupport: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION_SUPPORT,
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_SOURCE_MODE {
    pub Id: D3DKMDT_VIDEO_PRESENT_SOURCE_MODE_ID,
    pub Type: D3DKMDT_VIDPN_SOURCE_MODE_TYPE,
    pub Format: D3DKMDT_VIDPN_SOURCE_MODE_0,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_VIDPN_SOURCE_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMDT_VIDPN_SOURCE_MODE_0 {
    pub Graphics: D3DKMDT_GRAPHICS_RENDERING_FORMAT,
    pub Text: D3DKMDT_TEXT_RENDERING_FORMAT,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_VIDPN_SOURCE_MODE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMDT_VIDPN_SOURCE_MODE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_TARGET_MODE {
    pub Id: D3DKMDT_VIDEO_PRESENT_TARGET_MODE_ID,
    pub VideoSignalInfo: D3DKMDT_VIDEO_SIGNAL_INFO,
    pub Anonymous: D3DKMDT_VIDPN_TARGET_MODE_0,
    pub MinimumVSyncFreq: super::d3dukmdt::D3DDDI_RATIONAL,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_VIDPN_TARGET_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMDT_VIDPN_TARGET_MODE_0 {
    pub WireFormatAndPreference: D3DKMDT_WIRE_FORMAT_AND_PREFERENCE,
    pub Anonymous: D3DKMDT_VIDPN_TARGET_MODE_0_0,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMDT_VIDPN_TARGET_MODE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIDPN_TARGET_MODE_0_0 {
    pub _bitfield: D3DKMDT_MODE_PREFERENCE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_VIRTUALGPUSURFACEDATA {
    pub Size: u64,
    pub Alignment: u32,
    pub DriverSegmentId: u32,
    pub PrivateDriverData: u32,
}
pub const D3DKMDT_VOT_BNC: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 3;
pub const D3DKMDT_VOT_COMPONENT_VIDEO: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 3;
pub const D3DKMDT_VOT_COMPOSITE_VIDEO: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 2;
pub const D3DKMDT_VOT_DISPLAYPORT_EMBEDDED: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 11;
pub const D3DKMDT_VOT_DISPLAYPORT_EXTERNAL: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 10;
pub const D3DKMDT_VOT_DVI: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 4;
pub const D3DKMDT_VOT_D_JPN: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 8;
pub const D3DKMDT_VOT_HD15: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 0;
pub const D3DKMDT_VOT_HDMI: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 5;
pub const D3DKMDT_VOT_INDIRECT_WIRED: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 16;
pub const D3DKMDT_VOT_INTERNAL: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = -2147483648;
pub const D3DKMDT_VOT_LVDS: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 6;
pub const D3DKMDT_VOT_MIRACAST: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 15;
pub const D3DKMDT_VOT_OTHER: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = -1;
pub const D3DKMDT_VOT_RCA_3COMPONENT: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 3;
pub const D3DKMDT_VOT_RF: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 2;
pub const D3DKMDT_VOT_SDI: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 9;
pub const D3DKMDT_VOT_SDTVDONGLE: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 14;
pub const D3DKMDT_VOT_SVIDEO: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 1;
pub const D3DKMDT_VOT_SVIDEO_4PIN: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 1;
pub const D3DKMDT_VOT_SVIDEO_7PIN: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 1;
pub const D3DKMDT_VOT_UDI_EMBEDDED: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 13;
pub const D3DKMDT_VOT_UDI_EXTERNAL: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 12;
pub const D3DKMDT_VOT_UNINITIALIZED: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = -2;
pub const D3DKMDT_VPPC_GRAPHICS: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = 1;
pub const D3DKMDT_VPPC_NOTSPECIFIED: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = 255;
pub const D3DKMDT_VPPC_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = 0;
pub const D3DKMDT_VPPC_VIDEO: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = 2;
pub const D3DKMDT_VPPI_DENARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 10;
pub const D3DKMDT_VPPI_NONARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 9;
pub const D3DKMDT_VPPI_OCTONARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 8;
pub const D3DKMDT_VPPI_PRIMARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 1;
pub const D3DKMDT_VPPI_QUATERNARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 4;
pub const D3DKMDT_VPPI_QUINARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 5;
pub const D3DKMDT_VPPI_SECONDARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 2;
pub const D3DKMDT_VPPI_SENARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 6;
pub const D3DKMDT_VPPI_SEPTENARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 7;
pub const D3DKMDT_VPPI_TERTIARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 3;
pub const D3DKMDT_VPPI_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 0;
pub const D3DKMDT_VPPMT_MACROVISION_APSTRIGGER: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = 2;
pub const D3DKMDT_VPPMT_MACROVISION_FULLSUPPORT: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = 3;
pub const D3DKMDT_VPPMT_NOPROTECTION: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = 1;
pub const D3DKMDT_VPPMT_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = 0;
pub const D3DKMDT_VPPR_IDENTITY: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 1;
pub const D3DKMDT_VPPR_IDENTITY_OFFSET180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 9;
pub const D3DKMDT_VPPR_IDENTITY_OFFSET270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 13;
pub const D3DKMDT_VPPR_IDENTITY_OFFSET90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 5;
pub const D3DKMDT_VPPR_NOTSPECIFIED: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 255;
pub const D3DKMDT_VPPR_ROTATE180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 3;
pub const D3DKMDT_VPPR_ROTATE180_OFFSET180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 11;
pub const D3DKMDT_VPPR_ROTATE180_OFFSET270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 15;
pub const D3DKMDT_VPPR_ROTATE180_OFFSET90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 7;
pub const D3DKMDT_VPPR_ROTATE270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 4;
pub const D3DKMDT_VPPR_ROTATE270_OFFSET180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 12;
pub const D3DKMDT_VPPR_ROTATE270_OFFSET270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 16;
pub const D3DKMDT_VPPR_ROTATE270_OFFSET90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 8;
pub const D3DKMDT_VPPR_ROTATE90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 2;
pub const D3DKMDT_VPPR_ROTATE90_OFFSET180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 10;
pub const D3DKMDT_VPPR_ROTATE90_OFFSET270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 14;
pub const D3DKMDT_VPPR_ROTATE90_OFFSET90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 6;
pub const D3DKMDT_VPPR_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 0;
pub const D3DKMDT_VPPR_UNPINNED: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 254;
pub const D3DKMDT_VPPS_ASPECTRATIOCENTEREDMAX: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 4;
pub const D3DKMDT_VPPS_CENTERED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 2;
pub const D3DKMDT_VPPS_CUSTOM: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 5;
pub const D3DKMDT_VPPS_IDENTITY: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 1;
pub const D3DKMDT_VPPS_NOTSPECIFIED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 255;
pub const D3DKMDT_VPPS_RESERVED1: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 253;
pub const D3DKMDT_VPPS_STRETCHED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 3;
pub const D3DKMDT_VPPS_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 0;
pub const D3DKMDT_VPPS_UNPINNED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 254;
pub const D3DKMDT_VSS_APPLE: D3DKMDT_VIDEO_SIGNAL_STANDARD = 5;
pub const D3DKMDT_VSS_EIA_861: D3DKMDT_VIDEO_SIGNAL_STANDARD = 25;
pub const D3DKMDT_VSS_EIA_861A: D3DKMDT_VIDEO_SIGNAL_STANDARD = 26;
pub const D3DKMDT_VSS_EIA_861B: D3DKMDT_VIDEO_SIGNAL_STANDARD = 27;
pub const D3DKMDT_VSS_IBM: D3DKMDT_VIDEO_SIGNAL_STANDARD = 4;
pub const D3DKMDT_VSS_NTSC_443: D3DKMDT_VIDEO_SIGNAL_STANDARD = 8;
pub const D3DKMDT_VSS_NTSC_J: D3DKMDT_VIDEO_SIGNAL_STANDARD = 7;
pub const D3DKMDT_VSS_NTSC_M: D3DKMDT_VIDEO_SIGNAL_STANDARD = 6;
pub const D3DKMDT_VSS_OTHER: D3DKMDT_VIDEO_SIGNAL_STANDARD = 255;
pub const D3DKMDT_VSS_PAL_B: D3DKMDT_VIDEO_SIGNAL_STANDARD = 9;
pub const D3DKMDT_VSS_PAL_B1: D3DKMDT_VIDEO_SIGNAL_STANDARD = 10;
pub const D3DKMDT_VSS_PAL_D: D3DKMDT_VIDEO_SIGNAL_STANDARD = 14;
pub const D3DKMDT_VSS_PAL_G: D3DKMDT_VIDEO_SIGNAL_STANDARD = 11;
pub const D3DKMDT_VSS_PAL_H: D3DKMDT_VIDEO_SIGNAL_STANDARD = 12;
pub const D3DKMDT_VSS_PAL_I: D3DKMDT_VIDEO_SIGNAL_STANDARD = 13;
pub const D3DKMDT_VSS_PAL_K: D3DKMDT_VIDEO_SIGNAL_STANDARD = 28;
pub const D3DKMDT_VSS_PAL_K1: D3DKMDT_VIDEO_SIGNAL_STANDARD = 29;
pub const D3DKMDT_VSS_PAL_L: D3DKMDT_VIDEO_SIGNAL_STANDARD = 30;
pub const D3DKMDT_VSS_PAL_M: D3DKMDT_VIDEO_SIGNAL_STANDARD = 31;
pub const D3DKMDT_VSS_PAL_N: D3DKMDT_VIDEO_SIGNAL_STANDARD = 15;
pub const D3DKMDT_VSS_PAL_NC: D3DKMDT_VIDEO_SIGNAL_STANDARD = 16;
pub const D3DKMDT_VSS_SECAM_B: D3DKMDT_VIDEO_SIGNAL_STANDARD = 17;
pub const D3DKMDT_VSS_SECAM_D: D3DKMDT_VIDEO_SIGNAL_STANDARD = 18;
pub const D3DKMDT_VSS_SECAM_G: D3DKMDT_VIDEO_SIGNAL_STANDARD = 19;
pub const D3DKMDT_VSS_SECAM_H: D3DKMDT_VIDEO_SIGNAL_STANDARD = 20;
pub const D3DKMDT_VSS_SECAM_K: D3DKMDT_VIDEO_SIGNAL_STANDARD = 21;
pub const D3DKMDT_VSS_SECAM_K1: D3DKMDT_VIDEO_SIGNAL_STANDARD = 22;
pub const D3DKMDT_VSS_SECAM_L: D3DKMDT_VIDEO_SIGNAL_STANDARD = 23;
pub const D3DKMDT_VSS_SECAM_L1: D3DKMDT_VIDEO_SIGNAL_STANDARD = 24;
pub const D3DKMDT_VSS_UNINITIALIZED: D3DKMDT_VIDEO_SIGNAL_STANDARD = 0;
pub const D3DKMDT_VSS_VESA_CVT: D3DKMDT_VIDEO_SIGNAL_STANDARD = 3;
pub const D3DKMDT_VSS_VESA_DMT: D3DKMDT_VIDEO_SIGNAL_STANDARD = 1;
pub const D3DKMDT_VSS_VESA_GTF: D3DKMDT_VIDEO_SIGNAL_STANDARD = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_WIRE_FORMAT_AND_PREFERENCE {
    pub Anonymous: D3DKMDT_WIRE_FORMAT_AND_PREFERENCE_0,
    pub Value: u32,
}
impl Default for D3DKMDT_WIRE_FORMAT_AND_PREFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_WIRE_FORMAT_AND_PREFERENCE_0 {
    pub _bitfield: D3DKMDT_MODE_PREFERENCE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DISPLAY_CAPS {
    pub Anonymous: D3DKMT_DISPLAY_CAPS_0,
}
impl Default for D3DKMT_DISPLAY_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_DISPLAY_CAPS_0 {
    pub Anonymous: D3DKMT_DISPLAY_CAPS_0_0,
    pub Value: u64,
}
impl Default for D3DKMT_DISPLAY_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DISPLAY_CAPS_0_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DRIVERCAPS_EXT {
    pub Anonymous: D3DKMT_DRIVERCAPS_EXT_0,
}
impl Default for D3DKMT_DRIVERCAPS_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_DRIVERCAPS_EXT_0 {
    pub Anonymous: D3DKMT_DRIVERCAPS_EXT_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_DRIVERCAPS_EXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DRIVERCAPS_EXT_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MOVE_RECT {
    pub SourcePoint: super::windef::POINT,
    pub DestRect: super::windef::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_NODEMETADATA {
    pub NodeOrdinalAndAdapterIndex: u32,
    pub NodeData: DXGK_NODEMETADATA,
}
impl Default for D3DKMT_NODEMETADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYCLOCKCALIBRATION {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub NodeOrdinal: u32,
    pub PhysicalAdapterIndex: u32,
    pub ClockData: DXGK_GPUCLOCKDATA,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for D3DKMT_QUERYCLOCKCALIBRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_TRACKEDWORKLOAD_SUPPORT {
    pub PhysicalAdapterIndex: u32,
    pub EngineType: DXGK_ENGINE_TYPE,
    pub Support: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_1_2_CAPS {
    pub PreemptionCaps: D3DKMDT_PREEMPTION_CAPS,
    pub Anonymous: D3DKMT_WDDM_1_2_CAPS_0,
}
impl Default for D3DKMT_WDDM_1_2_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_1_2_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_1_2_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_WDDM_1_2_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WDDM_1_2_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_1_3_CAPS {
    pub Anonymous: D3DKMT_WDDM_1_3_CAPS_0,
}
impl Default for D3DKMT_WDDM_1_3_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_1_3_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_1_3_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_WDDM_1_3_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WDDM_1_3_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_0_CAPS {
    pub Anonymous: D3DKMT_WDDM_2_0_CAPS_0,
}
impl Default for D3DKMT_WDDM_2_0_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_2_0_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_2_0_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_WDDM_2_0_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WDDM_2_0_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_7_CAPS {
    pub Anonymous: D3DKMT_WDDM_2_7_CAPS_0,
}
impl Default for D3DKMT_WDDM_2_7_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_2_7_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_2_7_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_WDDM_2_7_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WDDM_2_7_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_9_CAPS {
    pub Anonymous: D3DKMT_WDDM_2_9_CAPS_0,
}
impl Default for D3DKMT_WDDM_2_9_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_2_9_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_2_9_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_WDDM_2_9_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WDDM_2_9_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_3_0_CAPS {
    pub Anonymous: D3DKMT_WDDM_3_0_CAPS_0,
}
impl Default for D3DKMT_WDDM_3_0_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_3_0_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_3_0_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_WDDM_3_0_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WDDM_3_0_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_3_1_CAPS {
    pub Anonymous: D3DKMT_WDDM_3_1_CAPS_0,
}
impl Default for D3DKMT_WDDM_3_1_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_3_1_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_3_1_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_WDDM_3_1_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WDDM_3_1_CAPS_0_0 {
    pub _bitfield: u32,
}
pub const DIDDT1_AspectRatio_15x9: _DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 3;
pub const DIDDT1_AspectRatio_16x10: _DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 5;
pub const DIDDT1_AspectRatio_16x9: _DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 4;
pub const DIDDT1_AspectRatio_1x1: _DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 0;
pub const DIDDT1_AspectRatio_4x3: _DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 2;
pub const DIDDT1_AspectRatio_5x4: _DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 1;
pub const DIDDT1_Dependent: _DISPLAYID_DETAILED_TIMING_TYPE_I_STEREO_MODE = 2;
pub const DIDDT1_Interlaced: _DISPLAYID_DETAILED_TIMING_TYPE_I_SCANNING_MODE = 1;
pub const DIDDT1_Monoscopic: _DISPLAYID_DETAILED_TIMING_TYPE_I_STEREO_MODE = 0;
pub const DIDDT1_Progressive: _DISPLAYID_DETAILED_TIMING_TYPE_I_SCANNING_MODE = 0;
pub const DIDDT1_Stereo: _DISPLAYID_DETAILED_TIMING_TYPE_I_STEREO_MODE = 1;
pub const DIDDT1_Sync_Negative: _DISPLAYID_DETAILED_TIMING_TYPE_I_SYNC_POLARITY = 1;
pub const DIDDT1_Sync_Positive: _DISPLAYID_DETAILED_TIMING_TYPE_I_SYNC_POLARITY = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYID_DETAILED_TIMING_TYPE_I {
    pub Anonymous: DISPLAYID_DETAILED_TIMING_TYPE_I_0,
    pub HorizontalActivePixels: u16,
    pub HorizontalBlankPixels: u16,
    pub Anonymous2: DISPLAYID_DETAILED_TIMING_TYPE_I_1,
    pub HorizontalSyncWidth: u16,
    pub VerticalActiveLines: u16,
    pub VerticalBlankLines: u16,
    pub Anonymous3: DISPLAYID_DETAILED_TIMING_TYPE_I_2,
    pub VerticalSyncWidth: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYID_DETAILED_TIMING_TYPE_I_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYID_DETAILED_TIMING_TYPE_I_1 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYID_DETAILED_TIMING_TYPE_I_2 {
    pub _bitfield: u16,
}
pub const DISPLAYID_DETAILED_TIMING_TYPE_I_SIZE: u32 = 20;
pub type DXGKARG_CALIBRATEGPUCLOCK = DXGK_GPUCLOCKDATA;
pub type DXGKARG_FENCESTORAGEVALUETYPE = i32;
pub type DXGKARG_GETNODEMETADATA = DXGK_NODEMETADATA;
#[repr(C)]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy)]
pub struct DXGKARG_SETPALETTE {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub FirstEntry: u32,
    pub NumEntries: u32,
    pub pLookupTable: *mut D3DKMDT_PALETTEDATA,
}
#[cfg(feature = "Win32_d3dukmdt")]
impl Default for DXGKARG_SETPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGKDT_OPM_DVI_CHARACTERISTICS = i32;
pub type DXGKMDT_CERTIFICATE_TYPE = i32;
pub const DXGKMDT_COPP_CERTIFICATE: DXGKMDT_CERTIFICATE_TYPE = 1;
pub const DXGKMDT_FORCE_ULONG: DXGKMDT_CERTIFICATE_TYPE = -1;
pub const DXGKMDT_I2C_DEVICE_TRANSMITS_DATA_LENGTH: u32 = 1;
pub const DXGKMDT_I2C_NO_FLAGS: u32 = 0;
pub const DXGKMDT_INDIRECT_DISPLAY_CERTIFICATE: DXGKMDT_CERTIFICATE_TYPE = 3;
pub const DXGKMDT_OPM_128_BIT_RANDOM_NUMBER_SIZE: u32 = 16;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_ACP_AND_CGMSA_SIGNALING {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulAvailableTVProtectionStandards: u32,
    pub ulActiveTVProtectionStandard: u32,
    pub ulReserved: u32,
    pub ulAspectRatioValidMask1: u32,
    pub ulAspectRatioData1: u32,
    pub ulAspectRatioValidMask2: u32,
    pub ulAspectRatioData2: u32,
    pub ulAspectRatioValidMask3: u32,
    pub ulAspectRatioData3: u32,
    pub ulReserved2: [u32; 4],
    pub ulReserved3: [u32; 4],
}
impl Default for DXGKMDT_OPM_ACP_AND_CGMSA_SIGNALING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_ACP_FORCE_ULONG: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 2147483647;
pub const DXGKMDT_OPM_ACP_LEVEL_ONE: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 1;
pub const DXGKMDT_OPM_ACP_LEVEL_THREE: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 3;
pub const DXGKMDT_OPM_ACP_LEVEL_TWO: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 2;
pub const DXGKMDT_OPM_ACP_OFF: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 0;
pub type DXGKMDT_OPM_ACP_PROTECTION_LEVEL = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGKMDT_OPM_ACTUAL_OUTPUT_FORMAT {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulDisplayWidth: u32,
    pub ulDisplayHeight: u32,
    pub ifInterleaveFormat: DXGKMDT_OPM_INTERLEAVE_FORMAT,
    pub d3dFormat: u32,
    pub ulFrequencyNumerator: u32,
    pub ulFrequencyDenominator: u32,
}
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_14_BY_9_CENTER: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 1;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_14_BY_9_TOP: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 2;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_16_BY_9_CENTER: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 3;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_16_BY_9_TOP: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 4;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_GT_16_BY_9_CENTER: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 5;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_16_BY_9_ANAMORPHIC: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 7;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_4_BY_3: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 0;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_4_BY_3_PROTECTED_CENTER: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 6;
pub const DXGKMDT_OPM_ASPECT_RATIO_FORCE_ULONG: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 2147483647;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_DAUGHTER_BOARD_CONNECTOR: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 262144;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 327680;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_INSIDE_OF_CHIPSET: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 65536;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_NON_STANDARD: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = -2147483648;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 131072;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 196608;
pub const DXGKMDT_OPM_BUS_TYPE_AGP: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 4;
pub type DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = i32;
pub const DXGKMDT_OPM_BUS_TYPE_OTHER: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 0;
pub const DXGKMDT_OPM_BUS_TYPE_PCI: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 1;
pub const DXGKMDT_OPM_BUS_TYPE_PCIEXPRESS: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 3;
pub const DXGKMDT_OPM_BUS_TYPE_PCIX: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 2;
pub const DXGKMDT_OPM_CERTIFICATE: DXGKMDT_CERTIFICATE_TYPE = 0;
pub type DXGKMDT_OPM_CGMSA = i32;
pub const DXGKMDT_OPM_CGMSA_COPY_FREELY: DXGKMDT_OPM_CGMSA = 1;
pub const DXGKMDT_OPM_CGMSA_COPY_NEVER: DXGKMDT_OPM_CGMSA = 4;
pub const DXGKMDT_OPM_CGMSA_COPY_NO_MORE: DXGKMDT_OPM_CGMSA = 2;
pub const DXGKMDT_OPM_CGMSA_COPY_ONE_GENERATION: DXGKMDT_OPM_CGMSA = 3;
pub const DXGKMDT_OPM_CGMSA_OFF: DXGKMDT_OPM_CGMSA = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_CONFIGURE_PARAMETERS {
    pub omac: DXGKMDT_OPM_OMAC,
    pub guidSetting: windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
impl Default for DXGKMDT_OPM_CONFIGURE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_CONFIGURE_SETTING_DATA_SIZE: u32 = 4056;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_CONNECTED_HDCP_DEVICE_INFORMATION {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulHDCPFlags: u32,
    pub ksvB: DXGKMDT_OPM_HDCP_KEY_SELECTION_VECTOR,
    pub Reserved: [u8; 11],
    pub Reserved2: [u8; 16],
    pub Reserved3: [u8; 16],
}
impl Default for DXGKMDT_OPM_CONNECTED_HDCP_DEVICE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGKMDT_OPM_CONNECTOR_TYPE = i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_COMPONENT_VIDEO: DXGKMDT_OPM_CONNECTOR_TYPE = 3;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_COMPOSITE_VIDEO: DXGKMDT_OPM_CONNECTOR_TYPE = 2;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_DISPLAYPORT_EMBEDDED: DXGKMDT_OPM_CONNECTOR_TYPE = 11;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_DISPLAYPORT_EXTERNAL: DXGKMDT_OPM_CONNECTOR_TYPE = 10;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_DVI: DXGKMDT_OPM_CONNECTOR_TYPE = 4;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_D_JPN: DXGKMDT_OPM_CONNECTOR_TYPE = 8;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_HD15: DXGKMDT_OPM_CONNECTOR_TYPE = 0;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_HDMI: DXGKMDT_OPM_CONNECTOR_TYPE = 5;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_LVDS: DXGKMDT_OPM_CONNECTOR_TYPE = 6;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_MIRACAST: DXGKMDT_OPM_CONNECTOR_TYPE = 15;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_OTHER: DXGKMDT_OPM_CONNECTOR_TYPE = -1;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_RESERVED: DXGKMDT_OPM_CONNECTOR_TYPE = 14;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_SDI: DXGKMDT_OPM_CONNECTOR_TYPE = 9;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_SVIDEO: DXGKMDT_OPM_CONNECTOR_TYPE = 1;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_TRANSPORT_AGNOSTIC_DIGITAL_MODE_A: DXGKMDT_OPM_CONNECTOR_TYPE = 16;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_TRANSPORT_AGNOSTIC_DIGITAL_MODE_B: DXGKMDT_OPM_CONNECTOR_TYPE = 17;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_UDI_EMBEDDED: DXGKMDT_OPM_CONNECTOR_TYPE = 13;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_UDI_EXTERNAL: DXGKMDT_OPM_CONNECTOR_TYPE = 12;
pub const DXGKMDT_OPM_COPP_COMPATIBLE_BUS_TYPE_INTEGRATED: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = -2147483648;
pub const DXGKMDT_OPM_COPP_COMPATIBLE_CONNECTOR_TYPE_INTERNAL: DXGKMDT_OPM_CONNECTOR_TYPE = -2147483648;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub guidInformation: windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
impl Default for DXGKMDT_OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_d3dukmdt", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DXGKMDT_OPM_CREATE_VIDEO_OUTPUT_FOR_TARGET_PARAMETERS {
    pub AdapterLuid: super::winnt::LUID,
    pub TargetId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_TARGET_ID,
    pub Vos: DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS,
}
pub const DXGKMDT_OPM_DPCP_FORCE_ULONG: DXGKMDT_OPM_DPCP_PROTECTION_LEVEL = 2147483647;
pub const DXGKMDT_OPM_DPCP_OFF: DXGKMDT_OPM_DPCP_PROTECTION_LEVEL = 0;
pub const DXGKMDT_OPM_DPCP_ON: DXGKMDT_OPM_DPCP_PROTECTION_LEVEL = 1;
pub type DXGKMDT_OPM_DPCP_PROTECTION_LEVEL = i32;
pub const DXGKMDT_OPM_DVI_CHARACTERISTICS_FORCE_ULONG: DXGKDT_OPM_DVI_CHARACTERISTICS = -1;
pub const DXGKMDT_OPM_DVI_CHARACTERISTIC_1_0: DXGKDT_OPM_DVI_CHARACTERISTICS = 1;
pub const DXGKMDT_OPM_DVI_CHARACTERISTIC_1_1_OR_ABOVE: DXGKDT_OPM_DVI_CHARACTERISTICS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_ENCRYPTED_PARAMETERS {
    pub abEncryptedParameters: [u8; 256],
}
impl Default for DXGKMDT_OPM_ENCRYPTED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_ENCRYPTED_PARAMETERS_SIZE: u32 = 256;
pub const DXGKMDT_OPM_GET_ACP_AND_CGMSA_SIGNALING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6629a591_3b79_4cf3_924a_11e8e7811671);
pub const DXGKMDT_OPM_GET_ACTUAL_OUTPUT_FORMAT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7bf1ba3_ad13_4f8e_af98_0dcb3ca204cc);
pub const DXGKMDT_OPM_GET_ACTUAL_PROTECTION_LEVEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1957210a_7766_452a_b99a_d27aed54f03a);
pub const DXGKMDT_OPM_GET_ADAPTER_BUS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc6f4d673_6174_4184_8e35_f6db5200bcba);
pub const DXGKMDT_OPM_GET_CODEC_INFO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f374491_8f5f_4445_9dba_95588f6b58b4);
pub const DXGKMDT_OPM_GET_CONNECTED_HDCP_DEVICE_INFORMATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0db59d74_a992_492e_a0bd_c23fda564e00);
pub const DXGKMDT_OPM_GET_CONNECTOR_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81d0bfd5_6afe_48c2_99c0_95a08f97c5da);
pub const DXGKMDT_OPM_GET_CURRENT_HDCP_SRM_VERSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x99c5ceff_5f1d_4879_81c1_c52443c9482b);
pub const DXGKMDT_OPM_GET_DVI_CHARACTERISTICS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa470b3bb_5dd7_4172_839c_3d3776e0ebf5);
pub const DXGKMDT_OPM_GET_INFORMATION_PARAMETERS_SIZE: u32 = 4056;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_GET_INFO_PARAMETERS {
    pub omac: DXGKMDT_OPM_OMAC,
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub guidInformation: windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
impl Default for DXGKMDT_OPM_GET_INFO_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_GET_OUTPUT_HARDWARE_PROTECTION_SUPPORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3b129589_2af8_4ef0_96a2_704a845a218e);
pub const DXGKMDT_OPM_GET_OUTPUT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x72cb6df3_244f_40ce_b09e_20506af6302f);
pub const DXGKMDT_OPM_GET_SUPPORTED_PROTECTION_TYPES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x38f2a801_9a6c_48bb_9107_b6696e6f1797);
pub const DXGKMDT_OPM_GET_VIRTUAL_PROTECTION_LEVEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb2075857_3eda_4d5d_88db_748f8c1a0549);
pub type DXGKMDT_OPM_HDCP_FLAG = i32;
pub const DXGKMDT_OPM_HDCP_FLAG_NONE: DXGKMDT_OPM_HDCP_FLAG = 0;
pub const DXGKMDT_OPM_HDCP_FLAG_REPEATER: DXGKMDT_OPM_HDCP_FLAG = 1;
pub const DXGKMDT_OPM_HDCP_FORCE_ULONG: DXGKMDT_OPM_HDCP_PROTECTION_LEVEL = 2147483647;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_HDCP_KEY_SELECTION_VECTOR {
    pub abKeySelectionVector: [u8; 5],
}
impl Default for DXGKMDT_OPM_HDCP_KEY_SELECTION_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_HDCP_KEY_SELECTION_VECTOR_SIZE: u32 = 5;
pub const DXGKMDT_OPM_HDCP_OFF: DXGKMDT_OPM_HDCP_PROTECTION_LEVEL = 0;
pub const DXGKMDT_OPM_HDCP_ON: DXGKMDT_OPM_HDCP_PROTECTION_LEVEL = 1;
pub type DXGKMDT_OPM_HDCP_PROTECTION_LEVEL = i32;
pub type DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = i32;
pub type DXGKMDT_OPM_INTERLEAVE_FORMAT = i32;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_FORCE_ULONG: DXGKMDT_OPM_INTERLEAVE_FORMAT = -1;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_INTERLEAVED_EVEN_FIRST: DXGKMDT_OPM_INTERLEAVE_FORMAT = 3;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_INTERLEAVED_ODD_FIRST: DXGKMDT_OPM_INTERLEAVE_FORMAT = 4;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_OTHER: DXGKMDT_OPM_INTERLEAVE_FORMAT = 0;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_PROGRESSIVE: DXGKMDT_OPM_INTERLEAVE_FORMAT = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_OMAC {
    pub abOMAC: [u8; 16],
}
impl Default for DXGKMDT_OPM_OMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_OMAC_SIZE: u32 = 16;
pub type DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION = i32;
pub const DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION_NOT_SUPPORTED: DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION = 0;
pub const DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION_SUPPORTED: DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGKMDT_OPM_OUTPUT_ID {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub OutputId: u64,
}
pub type DXGKMDT_OPM_PROTECTION_STANDARD = i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_ARIBTRB15_1125I: DXGKMDT_OPM_PROTECTION_STANDARD = 16384;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_ARIBTRB15_525I: DXGKMDT_OPM_PROTECTION_STANDARD = 2048;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_ARIBTRB15_525P: DXGKMDT_OPM_PROTECTION_STANDARD = 4096;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_ARIBTRB15_750P: DXGKMDT_OPM_PROTECTION_STANDARD = 8192;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEA_1125I: DXGKMDT_OPM_PROTECTION_STANDARD = 128;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEA_525P: DXGKMDT_OPM_PROTECTION_STANDARD = 32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEA_750P: DXGKMDT_OPM_PROTECTION_STANDARD = 64;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEB_1125I: DXGKMDT_OPM_PROTECTION_STANDARD = 1024;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEB_525P: DXGKMDT_OPM_PROTECTION_STANDARD = 256;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEB_750P: DXGKMDT_OPM_PROTECTION_STANDARD = 512;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_EIA608B_525: DXGKMDT_OPM_PROTECTION_STANDARD = 8;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_EN300294_625I: DXGKMDT_OPM_PROTECTION_STANDARD = 16;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_IEC61880_2_525I: DXGKMDT_OPM_PROTECTION_STANDARD = 2;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_IEC61880_525I: DXGKMDT_OPM_PROTECTION_STANDARD = 1;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_IEC62375_625P: DXGKMDT_OPM_PROTECTION_STANDARD = 4;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_NONE: DXGKMDT_OPM_PROTECTION_STANDARD = 0;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_OTHER: DXGKMDT_OPM_PROTECTION_STANDARD = -2147483648;
pub type DXGKMDT_OPM_PROTECTION_TYPE = i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_ACP: DXGKMDT_OPM_PROTECTION_TYPE = 2;
pub const DXGKMDT_OPM_PROTECTION_TYPE_CGMSA: DXGKMDT_OPM_PROTECTION_TYPE = 4;
pub const DXGKMDT_OPM_PROTECTION_TYPE_COPP_COMPATIBLE_HDCP: DXGKMDT_OPM_PROTECTION_TYPE = 1;
pub const DXGKMDT_OPM_PROTECTION_TYPE_DPCP: DXGKMDT_OPM_PROTECTION_TYPE = 16;
pub const DXGKMDT_OPM_PROTECTION_TYPE_HDCP: DXGKMDT_OPM_PROTECTION_TYPE = 8;
pub const DXGKMDT_OPM_PROTECTION_TYPE_MASK: DXGKMDT_OPM_PROTECTION_TYPE = -2147483585;
pub const DXGKMDT_OPM_PROTECTION_TYPE_NONE: DXGKMDT_OPM_PROTECTION_TYPE = 0;
pub const DXGKMDT_OPM_PROTECTION_TYPE_OTHER: DXGKMDT_OPM_PROTECTION_TYPE = -2147483648;
pub const DXGKMDT_OPM_PROTECTION_TYPE_SIZE: u32 = 4;
pub const DXGKMDT_OPM_PROTECTION_TYPE_TYPE_ENFORCEMENT_HDCP: DXGKMDT_OPM_PROTECTION_TYPE = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_RANDOM_NUMBER {
    pub abRandomNumber: [u8; 16],
}
impl Default for DXGKMDT_OPM_RANDOM_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_REDISTRIBUTION_CONTROL_REQUIRED: DXGKMDT_OPM_CGMSA = 8;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_REQUESTED_INFORMATION {
    pub omac: DXGKMDT_OPM_OMAC,
    pub cbRequestedInformationSize: u32,
    pub abRequestedInformation: [u8; 4076],
}
impl Default for DXGKMDT_OPM_REQUESTED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_REQUESTED_INFORMATION_SIZE: u32 = 4076;
pub const DXGKMDT_OPM_SET_ACP_AND_CGMSA_SIGNALING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09a631a5_d684_4c60_8e4d_d3bb0f0be3ee);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS {
    pub ulNewTVProtectionStandard: u32,
    pub ulAspectRatioChangeMask1: u32,
    pub ulAspectRatioData1: u32,
    pub ulAspectRatioChangeMask2: u32,
    pub ulAspectRatioData2: u32,
    pub ulAspectRatioChangeMask3: u32,
    pub ulAspectRatioData3: u32,
    pub ulReserved: [u32; 4],
    pub ulReserved2: [u32; 4],
    pub ulReserved3: u32,
}
impl Default for DXGKMDT_OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGKMDT_OPM_SET_HDCP_SRM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8b5ef5d1_c30d_44ff_84a5_ea71dce78f13);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGKMDT_OPM_SET_HDCP_SRM_PARAMETERS {
    pub ulSRMVersion: u32,
}
pub const DXGKMDT_OPM_SET_PROTECTION_LEVEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9bb9327c_4eb5_4727_9f00_b42b0919c0da);
pub const DXGKMDT_OPM_SET_PROTECTION_LEVEL_ACCORDING_TO_CSS_DVD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39ce333e_4cc0_44ae_bfcc_da50b5f82e72);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGKMDT_OPM_SET_PROTECTION_LEVEL_PARAMETERS {
    pub ulProtectionType: u32,
    pub ulProtectionLevel: u32,
    pub Reserved: u32,
    pub Reserved2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGKMDT_OPM_STANDARD_INFORMATION {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulInformation: u32,
    pub ulReserved: u32,
    pub ulReserved2: u32,
}
pub type DXGKMDT_OPM_STATUS = i32;
pub const DXGKMDT_OPM_STATUS_LINK_LOST: DXGKMDT_OPM_STATUS = 1;
pub const DXGKMDT_OPM_STATUS_NORMAL: DXGKMDT_OPM_STATUS = 0;
pub const DXGKMDT_OPM_STATUS_RENEGOTIATION_REQUIRED: DXGKMDT_OPM_STATUS = 2;
pub const DXGKMDT_OPM_STATUS_REVOKED_HDCP_DEVICE_ATTACHED: DXGKMDT_OPM_STATUS = 8;
pub const DXGKMDT_OPM_STATUS_TAMPERING_DETECTED: DXGKMDT_OPM_STATUS = 4;
pub const DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_FORCE_ULONG: DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 2147483647;
pub const DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_OFF: DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 0;
pub const DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_ON_WITH_NO_TYPE_RESTRICTION: DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 1;
pub const DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_ON_WITH_TYPE1_RESTRICTION: DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 2;
pub type DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = i32;
pub type DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS = i32;
pub const DXGKMDT_OPM_VOS_COPP_SEMANTICS: DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS = 0;
pub const DXGKMDT_OPM_VOS_OPM_INDIRECT_DISPLAY: DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS = 2;
pub const DXGKMDT_OPM_VOS_OPM_SEMANTICS: DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS = 1;
pub const DXGKMDT_UAB_CERTIFICATE: DXGKMDT_CERTIFICATE_TYPE = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_ADAPTER_PERFDATA {
    pub MemoryFrequency: u64,
    pub MaxMemoryFrequency: u64,
    pub MaxMemoryFrequencyOC: u64,
    pub MemoryBandwidth: u64,
    pub PCIEBandwidth: u64,
    pub FanRPM: u32,
    pub Power: u32,
    pub Temperature: u32,
    pub PowerStateOverride: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_ADAPTER_PERFDATACAPS {
    pub MaxMemoryBandwidth: u64,
    pub MaxPCIEBandwidth: u64,
    pub MaxFanRPM: u32,
    pub TemperatureMax: u32,
    pub TemperatureWarning: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct DXGK_BACKLIGHT_INFO {
    pub BacklightUsersetting: u16,
    pub BacklightEffective: u16,
    pub GammaRamp: super::d3dukmdt::D3DDDI_GAMMA_RAMP_RGB256x3x16,
}
pub type DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = i32;
pub const DXGK_BIND_TABLE_ENTRY_UNKNOWN: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_CAPS {
    pub Anonymous: DXGK_BRIGHTNESS_CAPS_0,
}
impl Default for DXGK_BRIGHTNESS_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_BRIGHTNESS_CAPS_0 {
    pub Anonymous: DXGK_BRIGHTNESS_CAPS_0_0,
    pub Value: u32,
}
impl Default for DXGK_BRIGHTNESS_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_BRIGHTNESS_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_GET_NIT_RANGES_OUT {
    pub NormalRangeCount: u32,
    pub RangeCount: u32,
    pub PreferredMaximumBrightness: u32,
    pub SupportedRanges: [DXGK_BRIGHTNESS_NIT_RANGE; 16],
}
impl Default for DXGK_BRIGHTNESS_GET_NIT_RANGES_OUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_BRIGHTNESS_GET_OUT {
    pub CurrentBrightnessMillinits: u32,
    pub TargetBrightnessMillinits: u32,
}
pub const DXGK_BRIGHTNESS_MAXIMUM_NIT_RANGE_COUNT: u32 = 16;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_BRIGHTNESS_NIT_RANGE {
    pub MinimumLevelMillinit: u32,
    pub MaximumLevelMillinit: u32,
    pub StepSizeMillinit: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_SENSOR_DATA {
    pub Size: u32,
    pub Anonymous: DXGK_BRIGHTNESS_SENSOR_DATA_0,
    pub AlsReading: f32,
    pub Chromaticity: DXGK_BRIGHTNESS_SENSOR_DATA_CHROMATICITY,
    pub ColorTemperature: f32,
}
impl Default for DXGK_BRIGHTNESS_SENSOR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_BRIGHTNESS_SENSOR_DATA_0 {
    pub Flags: DXGK_BRIGHTNESS_SENSOR_DATA_0_0,
    pub ValidSensorValues: u32,
}
impl Default for DXGK_BRIGHTNESS_SENSOR_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_BRIGHTNESS_SENSOR_DATA_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_BRIGHTNESS_SENSOR_DATA_CHROMATICITY {
    pub ChromaticityX: f32,
    pub ChromaticityY: f32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_SET_IN {
    pub BrightnessMillinits: u32,
    pub TransitionTimeMs: u32,
    pub SensorReadings: DXGK_BRIGHTNESS_SENSOR_DATA,
}
impl Default for DXGK_BRIGHTNESS_SET_IN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_STATE {
    pub Anonymous: DXGK_BRIGHTNESS_STATE_0,
}
impl Default for DXGK_BRIGHTNESS_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_BRIGHTNESS_STATE_0 {
    pub Anonymous: DXGK_BRIGHTNESS_STATE_0_0,
    pub Value: u32,
}
impl Default for DXGK_BRIGHTNESS_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_BRIGHTNESS_STATE_0_0 {
    pub _bitfield: u32,
}
pub type DXGK_CHILD_DEVICE_HPD_AWARENESS = i32;
pub const DXGK_DDT_DISPLAYID: DXGK_DISPLAY_DESCRIPTOR_TYPE = 2;
pub const DXGK_DDT_EDID: DXGK_DISPLAY_DESCRIPTOR_TYPE = 1;
pub const DXGK_DDT_INVALID: DXGK_DISPLAY_DESCRIPTOR_TYPE = 0;
pub const DXGK_DISPLAYMUX_DRIVER_SUPPORT_LEVEL_DEVELOPMENT: DXGK_DISPLAYMUX_SUPPORT_LEVEL = 2;
pub const DXGK_DISPLAYMUX_DRIVER_SUPPORT_LEVEL_EXPERIMENTAL: DXGK_DISPLAYMUX_SUPPORT_LEVEL = 3;
pub const DXGK_DISPLAYMUX_DRIVER_SUPPORT_LEVEL_FULL: DXGK_DISPLAYMUX_SUPPORT_LEVEL = 4;
pub const DXGK_DISPLAYMUX_DRIVER_SUPPORT_LEVEL_NONE: DXGK_DISPLAYMUX_SUPPORT_LEVEL = 1;
pub const DXGK_DISPLAYMUX_DRIVER_SUPPORT_LEVEL_UNINITIALIZED: DXGK_DISPLAYMUX_SUPPORT_LEVEL = 0;
pub type DXGK_DISPLAYMUX_RUNTIME_STATUS = i32;
pub const DXGK_DISPLAYMUX_RUNTIME_STATUS_CRITICAL_SYSTEM_INFO_MISSING: DXGK_DISPLAYMUX_RUNTIME_STATUS = 4;
pub const DXGK_DISPLAYMUX_RUNTIME_STATUS_NON_CRITICAL_SYSTEM_INFO_MISSING: DXGK_DISPLAYMUX_RUNTIME_STATUS = 3;
pub const DXGK_DISPLAYMUX_RUNTIME_STATUS_NO_GPU_SUPPORT: DXGK_DISPLAYMUX_RUNTIME_STATUS = 2;
pub const DXGK_DISPLAYMUX_RUNTIME_STATUS_OK: DXGK_DISPLAYMUX_RUNTIME_STATUS = 1;
pub const DXGK_DISPLAYMUX_RUNTIME_STATUS_UNINITIALIZED: DXGK_DISPLAYMUX_RUNTIME_STATUS = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_DISPLAYMUX_SET_INTERNAL_PANEL_INFO {
    pub Brightness3Supported: bool,
    pub Brightness3Caps: DXGK_BRIGHTNESS_CAPS,
    pub Bridgtness3Ranges: DXGK_BRIGHTNESS_GET_NIT_RANGES_OUT,
}
impl Default for DXGK_DISPLAYMUX_SET_INTERNAL_PANEL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGK_DISPLAYMUX_SUPPORT_LEVEL = i32;
pub type DXGK_DISPLAY_DESCRIPTOR_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_d3dukmdt", feature = "Win32_usb"))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_DISPLAY_INFORMATION {
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub ColorFormat: super::d3dukmdt::D3DDDIFORMAT,
    pub PhysicAddress: super::usb::PHYSICAL_ADDRESS,
    pub TargetId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_TARGET_ID,
    pub AcpiId: u32,
}
pub type DXGK_DISPLAY_TECHNOLOGY = i32;
pub type DXGK_DISPLAY_USAGE = i32;
pub const DXGK_DT_INVALID: DXGK_DISPLAY_TECHNOLOGY = 0;
pub const DXGK_DT_LCD: DXGK_DISPLAY_TECHNOLOGY = 2;
pub const DXGK_DT_MAX: DXGK_DISPLAY_TECHNOLOGY = 5;
pub const DXGK_DT_OLED: DXGK_DISPLAY_TECHNOLOGY = 3;
pub const DXGK_DT_OTHER: DXGK_DISPLAY_TECHNOLOGY = 1;
pub const DXGK_DT_PROJECTOR: DXGK_DISPLAY_TECHNOLOGY = 4;
pub const DXGK_DU_ACCESSORY: DXGK_DISPLAY_USAGE = 5;
pub const DXGK_DU_AR: DXGK_DISPLAY_USAGE = 2;
pub const DXGK_DU_GENERIC: DXGK_DISPLAY_USAGE = 1;
pub const DXGK_DU_INVALID: DXGK_DISPLAY_USAGE = 0;
pub const DXGK_DU_MAX: DXGK_DISPLAY_USAGE = 6;
pub const DXGK_DU_MEDICAL_IMAGING: DXGK_DISPLAY_USAGE = 4;
pub const DXGK_DU_VR: DXGK_DISPLAY_USAGE = 3;
pub type DXGK_ENGINE_TYPE = i32;
pub const DXGK_ENGINE_TYPE_3D: DXGK_ENGINE_TYPE = 1;
pub const DXGK_ENGINE_TYPE_COPY: DXGK_ENGINE_TYPE = 6;
pub const DXGK_ENGINE_TYPE_CRYPTO: DXGK_ENGINE_TYPE = 8;
pub const DXGK_ENGINE_TYPE_MAX: DXGK_ENGINE_TYPE = 10;
pub const DXGK_ENGINE_TYPE_OTHER: DXGK_ENGINE_TYPE = 0;
pub const DXGK_ENGINE_TYPE_OVERLAY: DXGK_ENGINE_TYPE = 7;
pub const DXGK_ENGINE_TYPE_SCENE_ASSEMBLY: DXGK_ENGINE_TYPE = 5;
pub const DXGK_ENGINE_TYPE_VIDEO_CODEC: DXGK_ENGINE_TYPE = 9;
pub const DXGK_ENGINE_TYPE_VIDEO_DECODE: DXGK_ENGINE_TYPE = 2;
pub const DXGK_ENGINE_TYPE_VIDEO_ENCODE: DXGK_ENGINE_TYPE = 3;
pub const DXGK_ENGINE_TYPE_VIDEO_PROCESSING: DXGK_ENGINE_TYPE = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_FAULT_ERROR_CODE {
    pub Anonymous: DXGK_FAULT_ERROR_CODE_0,
}
impl Default for DXGK_FAULT_ERROR_CODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_FAULT_ERROR_CODE_0 {
    pub Anonymous: DXGK_FAULT_ERROR_CODE_0_0,
    pub Anonymous2: DXGK_FAULT_ERROR_CODE_0_1,
}
impl Default for DXGK_FAULT_ERROR_CODE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_FAULT_ERROR_CODE_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_FAULT_ERROR_CODE_0_1 {
    pub _bitfield: u32,
}
pub const DXGK_FEATURE_SUPPORT_ALWAYS_OFF: u32 = 0;
pub const DXGK_FEATURE_SUPPORT_ALWAYS_ON: u32 = 3;
pub const DXGK_FEATURE_SUPPORT_EXPERIMENTAL: u32 = 1;
pub const DXGK_FEATURE_SUPPORT_STABLE: u32 = 2;
pub type DXGK_GENERAL_ERROR_CODE = i32;
pub const DXGK_GENERAL_ERROR_INVALID_INSTRUCTION: DXGK_GENERAL_ERROR_CODE = 1;
pub const DXGK_GENERAL_ERROR_PAGE_FAULT: DXGK_GENERAL_ERROR_CODE = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_GPUCLOCKDATA {
    pub GpuFrequency: u64,
    pub GpuClockCounter: u64,
    pub CpuClockCounter: u64,
    pub Flags: DXGK_GPUCLOCKDATA_FLAGS,
}
impl Default for DXGK_GPUCLOCKDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_GPUCLOCKDATA_FLAGS {
    pub Anonymous: DXGK_GPUCLOCKDATA_FLAGS_0,
}
impl Default for DXGK_GPUCLOCKDATA_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_GPUCLOCKDATA_FLAGS_0 {
    pub Anonymous: DXGK_GPUCLOCKDATA_FLAGS_0_0,
    pub Value: u32,
}
impl Default for DXGK_GPUCLOCKDATA_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_GPUCLOCKDATA_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_GPUVERSION {
    pub BiosVersion: [u16; 32],
    pub GpuArchitecture: [u16; 32],
}
impl Default for DXGK_GPUVERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGK_MAX_GPUVERSION_NAME_LENGTH: u32 = 32;
pub const DXGK_MAX_METADATA_NAME_LENGTH: u32 = 32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_MONITORLINKINFO_CAPABILITIES {
    pub Anonymous: DXGK_MONITORLINKINFO_CAPABILITIES_0,
    pub Value: u32,
}
impl Default for DXGK_MONITORLINKINFO_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_MONITORLINKINFO_CAPABILITIES_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_MONITORLINKINFO_USAGEHINTS {
    pub Anonymous: DXGK_MONITORLINKINFO_USAGEHINTS_0,
    pub Value: u32,
}
impl Default for DXGK_MONITORLINKINFO_USAGEHINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_MONITORLINKINFO_USAGEHINTS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_NODEMETADATA {
    pub EngineType: DXGK_ENGINE_TYPE,
    pub FriendlyName: [u16; 32],
    pub Flags: DXGK_NODEMETADATA_FLAGS,
    pub GpuMmuSupported: bool,
    pub IoMmuSupported: bool,
}
impl Default for DXGK_NODEMETADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_NODEMETADATA_FLAGS {
    pub Anonymous: DXGK_NODEMETADATA_FLAGS_0,
}
impl Default for DXGK_NODEMETADATA_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_NODEMETADATA_FLAGS_0 {
    pub Anonymous: DXGK_NODEMETADATA_FLAGS_0_0,
    pub Value: u32,
}
impl Default for DXGK_NODEMETADATA_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_NODEMETADATA_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_NODE_PERFDATA {
    pub Frequency: u64,
    pub MaxFrequency: u64,
    pub MaxFrequencyOC: u64,
    pub Voltage: u32,
    pub VoltageMax: u32,
    pub VoltageMaxOC: u32,
    pub MaxTransitionLatency: u64,
}
pub const DXGK_PAGE_FAULT_ADAPTER_RESET_REQUIRED: DXGK_PAGE_FAULT_FLAGS = 4;
pub const DXGK_PAGE_FAULT_AND_STALL: DXGK_PAGE_FAULT_FLAGS = 256;
pub const DXGK_PAGE_FAULT_ENGINE_RESET_REQUIRED: DXGK_PAGE_FAULT_FLAGS = 8;
pub const DXGK_PAGE_FAULT_FATAL_HARDWARE_ERROR: DXGK_PAGE_FAULT_FLAGS = 16;
pub const DXGK_PAGE_FAULT_FENCE_INVALID: DXGK_PAGE_FAULT_FLAGS = 2;
pub const DXGK_PAGE_FAULT_FIRST_INVALID: DXGK_PAGE_FAULT_FLAGS = 512;
pub type DXGK_PAGE_FAULT_FLAGS = i32;
pub const DXGK_PAGE_FAULT_HW_CONTEXT_VALID: DXGK_PAGE_FAULT_FLAGS = 64;
pub const DXGK_PAGE_FAULT_IOMMU: DXGK_PAGE_FAULT_FLAGS = 32;
pub const DXGK_PAGE_FAULT_PROCESS_HANDLE_VALID: DXGK_PAGE_FAULT_FLAGS = 128;
pub const DXGK_PAGE_FAULT_WRITE: DXGK_PAGE_FAULT_FLAGS = 1;
pub const DXGK_PRIMITIVE_API_SEQUENCE_NUMBER_UNKNOWN: i32 = -1;
pub type DXGK_RENDER_PIPELINE_STAGE = i32;
pub const DXGK_RENDER_PIPELINE_STAGE_GEOMETRY_SHADER: DXGK_RENDER_PIPELINE_STAGE = 3;
pub const DXGK_RENDER_PIPELINE_STAGE_INPUT_ASSEMBLER: DXGK_RENDER_PIPELINE_STAGE = 1;
pub const DXGK_RENDER_PIPELINE_STAGE_OUTPUT_MERGER: DXGK_RENDER_PIPELINE_STAGE = 7;
pub const DXGK_RENDER_PIPELINE_STAGE_PIXEL_SHADER: DXGK_RENDER_PIPELINE_STAGE = 6;
pub const DXGK_RENDER_PIPELINE_STAGE_RASTERIZER: DXGK_RENDER_PIPELINE_STAGE = 5;
pub const DXGK_RENDER_PIPELINE_STAGE_STREAM_OUTPUT: DXGK_RENDER_PIPELINE_STAGE = 4;
pub const DXGK_RENDER_PIPELINE_STAGE_UNKNOWN: DXGK_RENDER_PIPELINE_STAGE = 0;
pub const DXGK_RENDER_PIPELINE_STAGE_VERTEX_SHADER: DXGK_RENDER_PIPELINE_STAGE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGK_TARGETMODE_DETAIL_TIMING {
    pub VideoStandard: D3DKMDT_VIDEO_SIGNAL_STANDARD,
    pub TimingId: u32,
    pub DetailTiming: DISPLAYID_DETAILED_TIMING_TYPE_I,
}
pub const DXGK_VALID_FAULT_AND_STALL_FLAGS: u32 = 268;
pub const DxgkBacklightOptimizationDesktop: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 1;
pub const DxgkBacklightOptimizationDimmed: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 3;
pub const DxgkBacklightOptimizationDisable: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 0;
pub const DxgkBacklightOptimizationDynamic: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 2;
pub const DxgkBacklightOptimizationEDR: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 4;
pub const HpdAwarenessAlwaysConnected: DXGK_CHILD_DEVICE_HPD_AWARENESS = 1;
pub const HpdAwarenessInterruptible: DXGK_CHILD_DEVICE_HPD_AWARENESS = 4;
pub const HpdAwarenessNone: DXGK_CHILD_DEVICE_HPD_AWARENESS = 2;
pub const HpdAwarenessPolled: DXGK_CHILD_DEVICE_HPD_AWARENESS = 3;
pub const HpdAwarenessUninitialized: DXGK_CHILD_DEVICE_HPD_AWARENESS = 0;
pub type PD3DKMDT_WIRE_FORMAT_AND_PREFERENCE = *mut D3DKMDT_WIRE_FORMAT_AND_PREFERENCE;
pub type PDXGKMDT_OPM_CONFIGURE_PARAMETERS = *mut DXGKMDT_OPM_CONFIGURE_PARAMETERS;
pub type PDXGKMDT_OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS = *mut DXGKMDT_OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS;
pub type PDXGKMDT_OPM_ENCRYPTED_PARAMETERS = *mut DXGKMDT_OPM_ENCRYPTED_PARAMETERS;
pub type PDXGKMDT_OPM_GET_INFO_PARAMETERS = *mut DXGKMDT_OPM_GET_INFO_PARAMETERS;
pub type PDXGKMDT_OPM_OMAC = *mut DXGKMDT_OPM_OMAC;
pub type PDXGKMDT_OPM_RANDOM_NUMBER = *mut DXGKMDT_OPM_RANDOM_NUMBER;
pub type PDXGKMDT_OPM_REQUESTED_INFORMATION = *mut DXGKMDT_OPM_REQUESTED_INFORMATION;
pub type PDXGK_BRIGHTNESS_GET_NIT_RANGES_OUT = *mut DXGK_BRIGHTNESS_GET_NIT_RANGES_OUT;
pub type PDXGK_BRIGHTNESS_GET_OUT = *mut DXGK_BRIGHTNESS_GET_OUT;
pub type PDXGK_BRIGHTNESS_SET_IN = *mut DXGK_BRIGHTNESS_SET_IN;
pub type PDXGK_CHILD_DEVICE_HPD_AWARENESS = *mut DXGK_CHILD_DEVICE_HPD_AWARENESS;
pub type PDXGK_DISPLAYMUX_RUNTIME_STATUS = *mut DXGK_DISPLAYMUX_RUNTIME_STATUS;
pub type PDXGK_DISPLAYMUX_SET_INTERNAL_PANEL_INFO = *mut DXGK_DISPLAYMUX_SET_INTERNAL_PANEL_INFO;
pub type PDXGK_DISPLAYMUX_SUPPORT_LEVEL = *mut DXGK_DISPLAYMUX_SUPPORT_LEVEL;
pub type PDXGK_DISPLAY_DESCRIPTOR_TYPE = *mut DXGK_DISPLAY_DESCRIPTOR_TYPE;
#[cfg(all(feature = "Win32_d3dukmdt", feature = "Win32_usb"))]
pub type PDXGK_DISPLAY_INFORMATION = *mut DXGK_DISPLAY_INFORMATION;
pub type PDXGK_DISPLAY_TECHNOLOGY = *mut DXGK_DISPLAY_TECHNOLOGY;
pub type PDXGK_DISPLAY_USAGE = *mut DXGK_DISPLAY_USAGE;
pub type PDXGK_MONITORLINKINFO_CAPABILITIES = *mut DXGK_MONITORLINKINFO_CAPABILITIES;
pub type PDXGK_MONITORLINKINFO_USAGEHINTS = *mut DXGK_MONITORLINKINFO_USAGEHINTS;
pub type _DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = i32;
pub type _DISPLAYID_DETAILED_TIMING_TYPE_I_SCANNING_MODE = i32;
pub type _DISPLAYID_DETAILED_TIMING_TYPE_I_STEREO_MODE = i32;
pub type _DISPLAYID_DETAILED_TIMING_TYPE_I_SYNC_POLARITY = i32;
