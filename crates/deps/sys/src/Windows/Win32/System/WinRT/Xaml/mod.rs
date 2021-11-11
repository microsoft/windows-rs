#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn E_SURFACE_CONTENTS_LOST();
    fn IDesktopWindowXamlSourceNative();
    fn IDesktopWindowXamlSourceNative2();
    fn IFindReferenceTargetsCallback();
    fn IReferenceTracker();
    fn IReferenceTrackerExtension();
    fn IReferenceTrackerHost();
    fn IReferenceTrackerManager();
    fn IReferenceTrackerTarget();
    fn ISurfaceImageSourceManagerNative();
    fn ISurfaceImageSourceNative();
    fn ISurfaceImageSourceNativeWithD2D();
    fn ISwapChainBackgroundPanelNative();
    fn ISwapChainPanelNative();
    fn ISwapChainPanelNative2();
    fn ITrackerOwner();
    fn IVirtualSurfaceImageSourceNative();
    fn IVirtualSurfaceUpdatesCallbackNative();
    fn TrackerHandle__();
    fn XAML_REFERENCETRACKER_DISCONNECT();
}
