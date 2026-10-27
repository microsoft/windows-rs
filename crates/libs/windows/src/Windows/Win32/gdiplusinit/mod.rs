#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GdiplusShutdown(token: super::ULONG_PTR) {
    windows_core::link!("gdiplus.dll" "system" fn GdiplusShutdown(token : super::ULONG_PTR));
    unsafe { GdiplusShutdown(token) }
}
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdiplusStartup(token: *mut super::ULONG_PTR, input: *const GdiplusStartupInput, output: *mut GdiplusStartupOutput) -> super::Status {
    windows_core::link!("gdiplus.dll" "system" fn GdiplusStartup(token : *mut super::ULONG_PTR, input : *const GdiplusStartupInput, output : *mut GdiplusStartupOutput) -> super::Status);
    unsafe { GdiplusStartup(token as _, input, output as _) }
}
pub type DebugEventLevel = i32;
pub const DebugEventLevelFatal: DebugEventLevel = 0;
pub const DebugEventLevelWarning: DebugEventLevel = 1;
#[cfg(feature = "shared")]
pub type DebugEventProc = Option<unsafe extern "system" fn(level: DebugEventLevel, message: *mut super::CHAR)>;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GdiplusStartupInput {
    pub GdiplusVersion: super::UINT32,
    pub DebugEventCallback: DebugEventProc,
    pub SuppressBackgroundThread: windows_core::BOOL,
    pub SuppressExternalCodecs: windows_core::BOOL,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct GdiplusStartupOutput {
    pub NotificationHook: NotificationHookProc,
    pub NotificationUnhook: NotificationUnhookProc,
}
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
pub type NotificationHookProc = Option<unsafe extern "system" fn(token: *mut super::ULONG_PTR) -> super::Status>;
#[cfg(feature = "shared")]
pub type NotificationUnhookProc = Option<unsafe extern "system" fn(token: super::ULONG_PTR)>;
