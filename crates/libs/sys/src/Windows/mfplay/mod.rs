#[cfg(feature = "windef")]
windows_link::link!("mfplay.dll" "system" fn MFPCreateMediaPlayer(pwszurl : windows_sys::core::PCWSTR, fstartplayback : windows_sys::core::BOOL, creationoptions : MFP_CREATION_OPTIONS, pcallback : *mut core::ffi::c_void, hwnd : super::windef::HWND, ppmediaplayer : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C)]
#[cfg(all(feature = "mfidl", feature = "propsys"))]
#[derive(Clone, Copy)]
pub struct MFP_ACQUIRE_USER_CREDENTIAL_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub dwUserData: usize,
    pub fProceedWithAuthentication: windows_sys::core::BOOL,
    pub hrAuthenticationStatus: windows_sys::core::HRESULT,
    pub pwszURL: windows_sys::core::PCWSTR,
    pub pwszSite: windows_sys::core::PCWSTR,
    pub pwszRealm: windows_sys::core::PCWSTR,
    pub pwszPackage: windows_sys::core::PCWSTR,
    pub nRetries: i32,
    pub flags: MFP_CREDENTIAL_FLAGS,
    pub pCredential: *mut core::ffi::c_void,
}
#[cfg(all(feature = "mfidl", feature = "propsys"))]
impl Default for MFP_ACQUIRE_USER_CREDENTIAL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MFP_CREATION_OPTIONS = u32;
pub const MFP_CREDENTIAL_CLEAR_TEXT: _MFP_CREDENTIAL_FLAGS = 8;
pub const MFP_CREDENTIAL_DO_NOT_CACHE: _MFP_CREDENTIAL_FLAGS = 4;
pub type MFP_CREDENTIAL_FLAGS = u32;
pub const MFP_CREDENTIAL_LOGGED_ON_USER: _MFP_CREDENTIAL_FLAGS = 32;
pub const MFP_CREDENTIAL_PROMPT: _MFP_CREDENTIAL_FLAGS = 1;
pub const MFP_CREDENTIAL_PROXY: _MFP_CREDENTIAL_FLAGS = 16;
pub const MFP_CREDENTIAL_SAVE: _MFP_CREDENTIAL_FLAGS = 2;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy, Default)]
pub struct MFP_ERROR_EVENT {
    pub header: MFP_EVENT_HEADER,
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_EVENT_HEADER {
    pub eEventType: MFP_EVENT_TYPE,
    pub hrEvent: windows_sys::core::HRESULT,
    pub pMediaPlayer: *mut core::ffi::c_void,
    pub eState: MFP_MEDIAPLAYER_STATE,
    pub pPropertyStore: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MFP_EVENT_TYPE = i32;
pub const MFP_EVENT_TYPE_ACQUIRE_USER_CREDENTIAL: MFP_EVENT_TYPE = 12;
pub const MFP_EVENT_TYPE_ERROR: MFP_EVENT_TYPE = 10;
pub const MFP_EVENT_TYPE_FRAME_STEP: MFP_EVENT_TYPE = 7;
pub const MFP_EVENT_TYPE_MEDIAITEM_CLEARED: MFP_EVENT_TYPE = 8;
pub const MFP_EVENT_TYPE_MEDIAITEM_CREATED: MFP_EVENT_TYPE = 5;
pub const MFP_EVENT_TYPE_MEDIAITEM_SET: MFP_EVENT_TYPE = 6;
pub const MFP_EVENT_TYPE_MF: MFP_EVENT_TYPE = 9;
pub const MFP_EVENT_TYPE_PAUSE: MFP_EVENT_TYPE = 1;
pub const MFP_EVENT_TYPE_PLAY: MFP_EVENT_TYPE = 0;
pub const MFP_EVENT_TYPE_PLAYBACK_ENDED: MFP_EVENT_TYPE = 11;
pub const MFP_EVENT_TYPE_POSITION_SET: MFP_EVENT_TYPE = 3;
pub const MFP_EVENT_TYPE_RATE_SET: MFP_EVENT_TYPE = 4;
pub const MFP_EVENT_TYPE_STOP: MFP_EVENT_TYPE = 2;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_FRAME_STEP_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_FRAME_STEP_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFP_MEDIAITEM_CAN_PAUSE: _MFP_MEDIAITEM_CHARACTERISTICS = 4;
pub const MFP_MEDIAITEM_CAN_SEEK: _MFP_MEDIAITEM_CHARACTERISTICS = 2;
pub type MFP_MEDIAITEM_CHARACTERISTICS = u32;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_MEDIAITEM_CLEARED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_MEDIAITEM_CLEARED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_MEDIAITEM_CREATED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
    pub dwUserData: usize,
}
#[cfg(feature = "propsys")]
impl Default for MFP_MEDIAITEM_CREATED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFP_MEDIAITEM_HAS_SLOW_SEEK: _MFP_MEDIAITEM_CHARACTERISTICS = 8;
pub const MFP_MEDIAITEM_IS_LIVE: _MFP_MEDIAITEM_CHARACTERISTICS = 1;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_MEDIAITEM_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_MEDIAITEM_SET_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MFP_MEDIAPLAYER_STATE = i32;
pub const MFP_MEDIAPLAYER_STATE_EMPTY: MFP_MEDIAPLAYER_STATE = 0;
pub const MFP_MEDIAPLAYER_STATE_PAUSED: MFP_MEDIAPLAYER_STATE = 3;
pub const MFP_MEDIAPLAYER_STATE_PLAYING: MFP_MEDIAPLAYER_STATE = 2;
pub const MFP_MEDIAPLAYER_STATE_SHUTDOWN: MFP_MEDIAPLAYER_STATE = 4;
pub const MFP_MEDIAPLAYER_STATE_STOPPED: MFP_MEDIAPLAYER_STATE = 1;
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
#[derive(Clone, Copy)]
pub struct MFP_MF_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub MFEventType: super::mfobjects::MediaEventType,
    pub pMFMediaEvent: *mut core::ffi::c_void,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
impl Default for MFP_MF_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFP_OPTION_FREE_THREADED_CALLBACK: _MFP_CREATION_OPTIONS = 1;
pub const MFP_OPTION_NONE: _MFP_CREATION_OPTIONS = 0;
pub const MFP_OPTION_NO_MMCSS: _MFP_CREATION_OPTIONS = 2;
pub const MFP_OPTION_NO_REMOTE_DESKTOP_OPTIMIZATION: _MFP_CREATION_OPTIONS = 4;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_PAUSE_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_PAUSE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_PLAYBACK_ENDED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_PLAYBACK_ENDED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_PLAY_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_PLAY_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_POSITION_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_POSITION_SET_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_RATE_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
    pub flRate: f32,
}
#[cfg(feature = "propsys")]
impl Default for MFP_RATE_SET_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Copy)]
pub struct MFP_STOP_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: *mut core::ffi::c_void,
}
#[cfg(feature = "propsys")]
impl Default for MFP_STOP_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type _MFP_CREATION_OPTIONS = i32;
pub type _MFP_CREDENTIAL_FLAGS = i32;
pub type _MFP_MEDIAITEM_CHARACTERISTICS = i32;
