windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory(riid : *const windows_sys::core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory1(riid : *const windows_sys::core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ADAPTER_FLAG = i32;
pub const DXGI_ADAPTER_FLAG_FORCE_DWORD: DXGI_ADAPTER_FLAG = -1;
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = 0;
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = 1;
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [[f32; 2]; 8],
    pub WhitePoints: [[f32; 2]; 16],
}
impl Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1;
pub const DXGI_ENUM_MODES_SCALING: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_MAP_DISCARD: u32 = 4;
pub const DXGI_MAP_READ: u32 = 1;
pub const DXGI_MAP_WRITE: u32 = 2;
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16;
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2;
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4;
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1;
pub const DXGI_MWA_VALID: u32 = 7;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgitype", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::windef::RECT,
    pub AttachedToDesktop: windows_sys::core::BOOL,
    pub Rotation: super::dxgitype::DXGI_MODE_ROTATION,
    pub Monitor: super::windef::HMONITOR,
}
#[cfg(all(feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2;
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8;
pub const DXGI_PRESENT_RESTART: u32 = 4;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32;
pub const DXGI_PRESENT_TEST: u32 = 1;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256;
pub type DXGI_RESIDENCY = i32;
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = 3;
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = 1;
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = 2;
pub const DXGI_RESOURCE_PRIORITY_HIGH: u32 = 2684354560;
pub const DXGI_RESOURCE_PRIORITY_LOW: u32 = 1342177280;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200;
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: super::dxgitype::DXGI_MODE_DESC,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: super::windef::HWND,
    pub Windowed: windows_sys::core::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_SWAP_CHAIN_FLAG = i32;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = 2;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = 2048;
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = 32;
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = 128;
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = 64;
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = 256;
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = 4;
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = 1024;
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = 1;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = 8;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = 4096;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = 16;
pub const DXGI_SWAP_CHAIN_FLAG_USE_DEFAULT_COLOR_SPACE: DXGI_SWAP_CHAIN_FLAG = 32768;
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = 512;
pub type DXGI_SWAP_EFFECT = i32;
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1;
pub type DXGI_USAGE = u32;
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512;
pub const DXGI_USAGE_READ_ONLY: u32 = 256;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32;
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16;
pub const DXGI_USAGE_SHARED: u32 = 128;
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024;
