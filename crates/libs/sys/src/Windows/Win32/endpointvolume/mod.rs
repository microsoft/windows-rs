#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: windows_sys::core::GUID,
    pub bMuted: windows_sys::core::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: super::UINT,
    pub afChannelVolumes: [f32; 1],
}
#[cfg(feature = "shared")]
impl Default for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENDPOINT_HARDWARE_SUPPORT_METER: i32 = 4;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: i32 = 2;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: i32 = 1;
#[cfg(feature = "shared")]
pub type PAUDIO_VOLUME_NOTIFICATION_DATA = *mut AUDIO_VOLUME_NOTIFICATION_DATA;
