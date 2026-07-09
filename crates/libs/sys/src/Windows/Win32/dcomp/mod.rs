#[cfg(feature = "Win32_windef")]
windows_link::link!("dcomp.dll" "system" fn DCompositionAttachMouseDragToHwnd(visual : *mut core::ffi::c_void, hwnd : super::windef::HWND, enable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("dcomp.dll" "system" fn DCompositionAttachMouseWheelToHwnd(visual : *mut core::ffi::c_void, hwnd : super::windef::HWND, enable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("dcomp.dll" "system" fn DCompositionBoostCompositorClock(enable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_dxgi")]
windows_link::link!("dcomp.dll" "system" fn DCompositionCreateDevice(dxgidevice : *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dcomp.dll" "system" fn DCompositionCreateDevice2(renderingdevice : *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dcomp.dll" "system" fn DCompositionCreateDevice3(renderingdevice : *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("dcomp.dll" "system" fn DCompositionCreateSurfaceHandle(desiredaccess : u32, securityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, surfacehandle : *mut super::winnt::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_dcomptypes")]
windows_link::link!("dcomp.dll" "system" fn DCompositionGetFrameId(frameidtype : super::dcomptypes::COMPOSITION_FRAME_ID_TYPE, frameid : *mut super::dcomptypes::COMPOSITION_FRAME_ID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_dcomptypes", feature = "Win32_winnt"))]
windows_link::link!("dcomp.dll" "system" fn DCompositionGetStatistics(frameid : super::dcomptypes::COMPOSITION_FRAME_ID, framestats : *mut super::dcomptypes::COMPOSITION_FRAME_STATS, targetidcount : u32, targetids : *mut super::dcomptypes::COMPOSITION_TARGET_ID, actualtargetidcount : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_dcomptypes", feature = "Win32_winnt"))]
windows_link::link!("dcomp.dll" "system" fn DCompositionGetTargetStatistics(frameid : super::dcomptypes::COMPOSITION_FRAME_ID, targetid : *const super::dcomptypes::COMPOSITION_TARGET_ID, targetstats : *mut super::dcomptypes::COMPOSITION_TARGET_STATS) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("dcomp.dll" "system" fn DCompositionWaitForCompositorClock(count : u32, handles : *const super::winnt::HANDLE, timeoutinms : u32) -> u32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
