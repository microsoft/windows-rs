pub type TrackerHandle = *mut TrackerHandle__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TrackerHandle__ {
    pub unused: i32,
}
pub type XAML_REFERENCETRACKER_DISCONNECT = i32;
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = 0;
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = 1;
