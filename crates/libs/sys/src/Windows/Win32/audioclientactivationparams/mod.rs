#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0,
}
impl Default for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    pub ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
}
impl Default for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AUDIOCLIENT_ACTIVATION_TYPE = i32;
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = 0;
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
pub type PROCESS_LOOPBACK_MODE = i32;
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = 1;
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = 0;
pub const VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK: windows_sys::core::PCWSTR = windows_sys::core::w!("VAD\\Process_Loopback");
