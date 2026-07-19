#[cfg(feature = "windef")]
windows_link::link!("dcomp.dll" "system" fn DCompositionAttachMouseDragToHwnd(visual : *mut core::ffi::c_void, hwnd : super::HWND, enable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("dcomp.dll" "system" fn DCompositionAttachMouseWheelToHwnd(visual : *mut core::ffi::c_void, hwnd : super::HWND, enable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("dcomp.dll" "system" fn DCompositionBoostCompositorClock(enable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "dxgi")]
windows_link::link!("dcomp.dll" "system" fn DCompositionCreateDevice(dxgidevice : *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dcomp.dll" "system" fn DCompositionCreateDevice2(renderingdevice : *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dcomp.dll" "system" fn DCompositionCreateDevice3(renderingdevice : *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("dcomp.dll" "system" fn DCompositionCreateSurfaceHandle(desiredaccess : u32, securityattributes : *const super::SECURITY_ATTRIBUTES, surfacehandle : *mut super::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("dcomp.dll" "system" fn DCompositionGetFrameId(frameidtype : COMPOSITION_FRAME_ID_TYPE, frameid : *mut COMPOSITION_FRAME_ID) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("dcomp.dll" "system" fn DCompositionGetStatistics(frameid : COMPOSITION_FRAME_ID, framestats : *mut COMPOSITION_FRAME_STATS, targetidcount : u32, targetids : *mut COMPOSITION_TARGET_ID, actualtargetidcount : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("dcomp.dll" "system" fn DCompositionGetTargetStatistics(frameid : COMPOSITION_FRAME_ID, targetid : *const COMPOSITION_TARGET_ID, targetstats : *mut COMPOSITION_TARGET_STATS) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("dcomp.dll" "system" fn DCompositionWaitForCompositorClock(count : u32, handles : *const super::HANDLE, timeoutinms : u32) -> u32);
pub const COMPOSITIONOBJECT_ALL_ACCESS: u32 = 3;
pub const COMPOSITIONOBJECT_READ: u32 = 1;
pub const COMPOSITIONOBJECT_WRITE: u32 = 2;
pub type COMPOSITION_FRAME_ID = u64;
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = 2;
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = 1;
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = 0;
pub type COMPOSITION_FRAME_ID_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::LUID,
    pub renderAdapterLuid: super::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
pub type DCOMPOSITION_BACKFACE_VISIBILITY = i32;
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = 1;
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = -1;
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = 0;
pub type DCOMPOSITION_BITMAP_INTERPOLATION_MODE = i32;
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = -1;
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = 1;
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = 0;
pub type DCOMPOSITION_BORDER_MODE = i32;
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = 1;
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = -1;
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = 0;
pub type DCOMPOSITION_COMPOSITE_MODE = i32;
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = 1;
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = -1;
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = 2;
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = 0;
pub type DCOMPOSITION_DEPTH_MODE = i32;
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = -1;
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = 3;
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = 1;
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = 0;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32;
pub type DCOMPOSITION_OPACITY_MODE = i32;
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = -1;
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = 0;
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
