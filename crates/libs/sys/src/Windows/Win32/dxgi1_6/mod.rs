windows_link::link!("dxgi.dll" "system" fn DXGIDeclareAdapterRemovalSupport() -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn DXGIDisableVBlankVirtualization() -> windows_sys::core::HRESULT);
#[repr(C)]
#[cfg(all(feature = "Win32_dxgi1_2", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: super::dxgi1_2::DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: super::dxgi1_2::DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(all(feature = "Win32_dxgi1_2", feature = "Win32_winnt"))]
impl Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ADAPTER_FLAG3 = u32;
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = 4;
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: DXGI_ADAPTER_FLAG3 = 4294967295;
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 = 32;
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = 0;
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = 1;
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = 2;
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = 8;
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = 16;
pub type DXGI_GPU_PREFERENCE = i32;
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = 2;
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = 1;
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = 0;
pub type DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = u32;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 4;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 1;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::windef::RECT,
    pub AttachedToDesktop: windows_sys::core::BOOL,
    pub Rotation: super::dxgitype::DXGI_MODE_ROTATION,
    pub Monitor: super::windef::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
