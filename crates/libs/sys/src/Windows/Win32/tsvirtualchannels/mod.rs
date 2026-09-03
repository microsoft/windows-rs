#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
pub const Desktop: RdpSessionType = 0;
pub const E_DUPLICATE_WINDOW_HINT: i32 = -2147024713;
pub const E_MAPPEDRENDERER_SHUTDOWN: i32 = -2147019873;
pub type PBITMAP_RENDERER_STATISTICS = *mut BITMAP_RENDERER_STATISTICS;
pub type RdpSessionType = i32;
pub const RemoteApp: RdpSessionType = 1;
pub const TS_VC_LISTENER_STATIC_CHANNEL: i32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct WTSWindowInfo {
    pub Hwnd: super::HWND,
    pub Height: i32,
    pub Width: i32,
    pub ViewWidth: i32,
    pub ViewHeight: i32,
    pub ViewOffsetX: i32,
    pub ViewOffsetY: i32,
    pub Scale: f32,
}
pub const WTS_PROPERTY_DEFAULT_CONFIG: windows_sys::core::PCWSTR = windows_sys::core::w!("DefaultConfig");
