#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AdjustWindowRectExForDpi();
    fn AreDpiAwarenessContextsEqual();
    fn EnableNonClientDpiScaling();
    fn GetAwarenessFromDpiAwarenessContext();
    fn GetDialogControlDpiChangeBehavior();
    fn GetDialogDpiChangeBehavior();
    fn GetDpiAwarenessContextForProcess();
    fn GetDpiForMonitor();
    fn GetDpiForSystem();
    fn GetDpiForWindow();
    fn GetDpiFromDpiAwarenessContext();
    fn GetProcessDpiAwareness();
    fn GetSystemDpiForProcess();
    fn GetSystemMetricsForDpi();
    fn GetThreadDpiAwarenessContext();
    fn GetThreadDpiHostingBehavior();
    fn GetWindowDpiAwarenessContext();
    fn GetWindowDpiHostingBehavior();
    fn IsValidDpiAwarenessContext();
    fn LogicalToPhysicalPointForPerMonitorDPI();
    fn OpenThemeDataForDpi();
    fn PhysicalToLogicalPointForPerMonitorDPI();
    fn SetDialogControlDpiChangeBehavior();
    fn SetDialogDpiChangeBehavior();
    fn SetProcessDpiAwareness();
    fn SetProcessDpiAwarenessContext();
    fn SetThreadDpiAwarenessContext();
    fn SetThreadDpiHostingBehavior();
    fn SystemParametersInfoForDpi();
}
