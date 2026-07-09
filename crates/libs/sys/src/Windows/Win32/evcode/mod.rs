pub const AM_CONTENTPROPERTY_AUTHOR: u32 = 2;
pub const AM_CONTENTPROPERTY_COPYRIGHT: u32 = 4;
pub const AM_CONTENTPROPERTY_DESCRIPTION: u32 = 8;
pub const AM_CONTENTPROPERTY_TITLE: u32 = 1;
pub const AM_LOADSTATUS_CLOSED: u32 = 0;
pub const AM_LOADSTATUS_CONNECTING: u32 = 4;
pub const AM_LOADSTATUS_LOADINGDESCR: u32 = 1;
pub const AM_LOADSTATUS_LOADINGMCAST: u32 = 2;
pub const AM_LOADSTATUS_LOCATING: u32 = 3;
pub const AM_LOADSTATUS_OPEN: u32 = 6;
pub const AM_LOADSTATUS_OPENING: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AM_WMT_EVENT_DATA {
    pub hrStatus: windows_sys::core::HRESULT,
    pub pData: *mut core::ffi::c_void,
}
impl Default for AM_WMT_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EC_ACTIVATE: u32 = 19;
pub const EC_BANDWIDTHCHANGE: u32 = 72;
pub const EC_BUFFERING_DATA: u32 = 17;
pub const EC_BUILT: u32 = 768;
pub const EC_CLOCK_CHANGED: u32 = 13;
pub const EC_CLOCK_UNSET: u32 = 81;
pub const EC_CODECAPI_EVENT: u32 = 87;
pub const EC_COMPLETE: u32 = 1;
pub const EC_CONTENTPROPERTY_CHANGED: u32 = 71;
pub const EC_DEVICE_LOST: u32 = 31;
pub const EC_DISPLAY_CHANGED: u32 = 22;
pub const EC_END_OF_SEGMENT: u32 = 28;
pub const EC_EOS_SOON: u32 = 70;
pub const EC_ERRORABORT: u32 = 3;
pub const EC_ERRORABORTEX: u32 = 69;
pub const EC_ERROR_STILLPLAYING: u32 = 8;
pub const EC_EXTDEVICE_MODE_CHANGE: u32 = 49;
pub const EC_FILE_CLOSED: u32 = 68;
pub const EC_FULLSCREEN_LOST: u32 = 18;
pub const EC_GRAPH_CHANGED: u32 = 80;
pub const EC_LENGTH_CHANGED: u32 = 30;
pub const EC_LOADSTATUS: u32 = 67;
pub const EC_MARKER_HIT: u32 = 66;
pub const EC_NEED_RESTART: u32 = 20;
pub const EC_NEW_PIN: u32 = 32;
pub const EC_NOTIFY_WINDOW: u32 = 25;
pub const EC_OLE_EVENT: u32 = 24;
pub const EC_OPENING_FILE: u32 = 16;
pub const EC_PALETTE_CHANGED: u32 = 9;
pub const EC_PAUSED: u32 = 14;
pub const EC_PLEASE_REOPEN: u32 = 64;
pub const EC_PREPROCESS_COMPLETE: u32 = 86;
pub const EC_PROCESSING_LATENCY: u32 = 33;
pub const EC_QUALITY_CHANGE: u32 = 11;
pub const EC_RENDER_FINISHED: u32 = 33;
pub const EC_REPAINT: u32 = 5;
pub const EC_SAMPLE_LATENCY: u32 = 34;
pub const EC_SAMPLE_NEEDED: u32 = 32;
pub const EC_SCRUB_TIME: u32 = 35;
pub const EC_SEGMENT_STARTED: u32 = 29;
pub const EC_SHUTTING_DOWN: u32 = 12;
pub const EC_SKIP_FRAMES: u32 = 37;
pub const EC_STARVATION: u32 = 23;
pub const EC_STATE_CHANGE: u32 = 50;
pub const EC_STATUS: u32 = 65;
pub const EC_STEP_COMPLETE: u32 = 36;
pub const EC_STREAM_CONTROL_STARTED: u32 = 27;
pub const EC_STREAM_CONTROL_STOPPED: u32 = 26;
pub const EC_STREAM_ERROR_STILLPLAYING: u32 = 7;
pub const EC_STREAM_ERROR_STOPPED: u32 = 6;
pub const EC_SYSTEMBASE: u32 = 0;
pub const EC_TIME: u32 = 4;
pub const EC_TIMECODE_AVAILABLE: u32 = 48;
pub const EC_UNBUILT: u32 = 769;
pub const EC_USER: u32 = 32768;
pub const EC_USERABORT: u32 = 2;
pub const EC_VIDEOFRAMEREADY: u32 = 73;
pub const EC_VIDEO_SIZE_CHANGED: u32 = 10;
pub const EC_VMR_RECONNECTION_FAILED: u32 = 85;
pub const EC_VMR_RENDERDEVICE_SET: u32 = 83;
pub const EC_VMR_SURFACE_FLIPPED: u32 = 84;
pub const EC_WINDOW_DESTROYED: u32 = 21;
pub const EC_WMT_EVENT: u32 = 594;
pub const EC_WMT_EVENT_BASE: u32 = 593;
pub const EC_WMT_INDEX_EVENT: u32 = 593;
pub const VMR_RENDER_DEVICE_OVERLAY: u32 = 1;
pub const VMR_RENDER_DEVICE_SYSMEM: u32 = 4;
pub const VMR_RENDER_DEVICE_VIDMEM: u32 = 2;
