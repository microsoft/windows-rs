#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DCompositionAttachMouseDragToHwnd();
    fn DCompositionAttachMouseWheelToHwnd();
    fn DCompositionBoostCompositorClock();
    fn DCompositionCreateDevice();
    fn DCompositionCreateDevice2();
    fn DCompositionCreateDevice3();
    fn DCompositionCreateSurfaceHandle();
    fn DCompositionGetFrameId();
    fn DCompositionGetStatistics();
    fn DCompositionGetTargetStatistics();
    fn DCompositionWaitForCompositorClock();
}
