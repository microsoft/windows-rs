#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: windows_sys::core::GUID,
    pub bMuted: windows_sys::core::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
impl Default for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1;
pub type PAUDIO_VOLUME_NOTIFICATION_DATA = *mut AUDIO_VOLUME_NOTIFICATION_DATA;
