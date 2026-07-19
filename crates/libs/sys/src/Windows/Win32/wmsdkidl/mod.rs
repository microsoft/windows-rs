windows_link::link!("wmvcore.dll" "system" fn WMCreateBackupRestorer(pcallback : *mut core::ffi::c_void, ppbackup : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateEditor(ppeditor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateIndexer(ppindexer : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateProfileManager(ppprofilemanager : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateReader(punkcert : *mut core::ffi::c_void, dwrights : u32, ppreader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateSyncReader(punkcert : *mut core::ffi::c_void, dwrights : u32, ppsyncreader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateWriter(punkcert : *mut core::ffi::c_void, ppwriter : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateWriterFileSink(ppsink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateWriterNetworkSink(ppsink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateWriterPushSink(ppsink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMIsContentProtected(pwszfilename : *const u16, pfisprotected : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: u16,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: u16,
    pub wUncompressedDigitalVideo: u16,
    pub wAnalogVideo: u16,
    pub wCompressedDigitalAudio: u16,
    pub wUncompressedDigitalAudio: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: u16,
    pub rgIds: *mut windows_sys::core::GUID,
}
impl Default for DRM_OPL_OUTPUT_IDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRM_OPL_TYPES: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: windows_sys::core::GUID,
    pub bConfigData: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRM_VAL16 {
    pub val: [u8; 16],
}
impl Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRM_VIDEO_OUTPUT_PROTECTION = DRM_OUTPUT_PROTECTION;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: u16,
    pub rgVop: *mut DRM_VIDEO_OUTPUT_PROTECTION,
}
impl Default for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPCWSTR_WMSDK_TYPE_SAFE = windows_sys::core::PCWSTR;
pub const WEBSTREAM_SAMPLE_TYPE_FILE: i32 = 1;
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: u32,
    pub cbEncryptedSessionKeyMessage: u32,
    pub pbEncryptedSessionKeyMessage: *mut u8,
    pub cbEncryptedKeyMessage: u32,
    pub pbEncryptedKeyMessage: *mut u8,
}
impl Default for WMDRM_IMPORT_INIT_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl Default for WMMPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WMSCRIPTFORMAT {
    pub scriptType: windows_sys::core::GUID,
}
pub const WMT_ACQUIRE_LICENSE: WMT_STATUS = 23;
pub type WMT_ATTR_DATATYPE = i32;
pub type WMT_ATTR_IMAGETYPE = i32;
pub const WMT_BACKUPRESTORE_BEGIN: WMT_STATUS = 21;
pub const WMT_BACKUPRESTORE_CONNECTING: WMT_STATUS = 28;
pub const WMT_BACKUPRESTORE_DISCONNECTING: WMT_STATUS = 29;
pub const WMT_BACKUPRESTORE_END: WMT_STATUS = 27;
pub const WMT_BUFFERING_START: WMT_STATUS = 2;
pub const WMT_BUFFERING_STOP: WMT_STATUS = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: *mut core::ffi::c_void,
    pub cbOffset: u32,
    pub cbLength: u32,
}
impl Default for WMT_BUFFER_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMT_CLEANPOINT_ONLY: WMT_STREAM_SELECTION = 1;
pub const WMT_CLIENT_CONNECT: WMT_STATUS = 32;
pub const WMT_CLIENT_CONNECT_EX: WMT_STATUS = 37;
pub const WMT_CLIENT_DISCONNECT: WMT_STATUS = 33;
pub const WMT_CLIENT_DISCONNECT_EX: WMT_STATUS = 38;
pub const WMT_CLIENT_PROPERTIES: WMT_STATUS = 42;
pub const WMT_CLOSED: WMT_STATUS = 13;
pub const WMT_CODECINFO_AUDIO: WMT_CODEC_INFO_TYPE = 0;
pub const WMT_CODECINFO_UNKNOWN: WMT_CODEC_INFO_TYPE = -1;
pub const WMT_CODECINFO_VIDEO: WMT_CODEC_INFO_TYPE = 1;
pub type WMT_CODEC_INFO_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: u8,
    pub ucColorTransferChar: u8,
    pub ucColorMatrixCoef: u8,
}
pub const WMT_CONNECTING: WMT_STATUS = 8;
pub const WMT_CONTENT_ENABLER: WMT_STATUS = 51;
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = 4;
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = 2;
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = 16;
pub type WMT_CREDENTIAL_FLAGS = i32;
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = 8;
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = 1;
pub const WMT_DRMLA_TAMPERED: WMT_DRMLA_TRUST = 2;
pub type WMT_DRMLA_TRUST = i32;
pub const WMT_DRMLA_TRUSTED: WMT_DRMLA_TRUST = 1;
pub const WMT_DRMLA_UNTRUSTED: WMT_DRMLA_TRUST = 0;
pub const WMT_END_OF_FILE: WMT_STATUS = 4;
pub const WMT_END_OF_SEGMENT: WMT_STATUS = 5;
pub const WMT_END_OF_STREAMING: WMT_STATUS = 6;
pub const WMT_EOF: WMT_STATUS = 4;
pub const WMT_ERROR: WMT_STATUS = 0;
pub const WMT_ERROR_WITHURL: WMT_STATUS = 30;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: u32,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: u32,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
}
impl Default for WMT_FILESINK_DATA_UNIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WMT_FILESINK_MODE = i32;
pub const WMT_FM_FILESINK_DATA_UNITS: WMT_FILESINK_MODE = 2;
pub const WMT_FM_FILESINK_UNBUFFERED: WMT_FILESINK_MODE = 4;
pub const WMT_FM_SINGLE_BUFFERS: WMT_FILESINK_MODE = 1;
pub const WMT_IMAGETYPE_BITMAP: WMT_ATTR_IMAGETYPE = 1;
pub const WMT_IMAGETYPE_GIF: WMT_ATTR_IMAGETYPE = 3;
pub const WMT_IMAGETYPE_JPEG: WMT_ATTR_IMAGETYPE = 2;
pub type WMT_IMAGE_TYPE = i32;
pub type WMT_INDEXER_TYPE = i32;
pub const WMT_INDEX_PROGRESS: WMT_STATUS = 16;
pub type WMT_INDEX_TYPE = i32;
pub const WMT_INDIVIDUALIZE: WMT_STATUS = 24;
pub const WMT_INIT_PLAYLIST_BURN: WMT_STATUS = 44;
pub const WMT_IT_BITMAP: WMT_IMAGE_TYPE = 1;
pub const WMT_IT_FRAME_NUMBERS: WMT_INDEXER_TYPE = 1;
pub const WMT_IT_GIF: WMT_IMAGE_TYPE = 3;
pub const WMT_IT_JPEG: WMT_IMAGE_TYPE = 2;
pub const WMT_IT_NEAREST_CLEAN_POINT: WMT_INDEX_TYPE = 3;
pub const WMT_IT_NEAREST_DATA_UNIT: WMT_INDEX_TYPE = 1;
pub const WMT_IT_NEAREST_OBJECT: WMT_INDEX_TYPE = 2;
pub const WMT_IT_NONE: WMT_IMAGE_TYPE = 0;
pub const WMT_IT_PRESENTATION_TIME: WMT_INDEXER_TYPE = 0;
pub const WMT_IT_TIMECODE: WMT_INDEXER_TYPE = 2;
pub const WMT_LICENSEURL_SIGNATURE_STATE: WMT_STATUS = 43;
pub const WMT_LOCATING: WMT_STATUS = 7;
pub const WMT_MISSING_CODEC: WMT_STATUS = 10;
pub const WMT_MS_CLASS_MIXED: WMT_MUSICSPEECH_CLASS_MODE = 2;
pub const WMT_MS_CLASS_MUSIC: WMT_MUSICSPEECH_CLASS_MODE = 0;
pub const WMT_MS_CLASS_SPEECH: WMT_MUSICSPEECH_CLASS_MODE = 1;
pub type WMT_MUSICSPEECH_CLASS_MODE = i32;
pub const WMT_NATIVE_OUTPUT_PROPS_CHANGED: WMT_STATUS = 34;
pub const WMT_NEEDS_INDIVIDUALIZATION: WMT_STATUS = 25;
pub type WMT_NET_PROTOCOL = i32;
pub const WMT_NEW_METADATA: WMT_STATUS = 20;
pub const WMT_NEW_SOURCEFLAGS: WMT_STATUS = 19;
pub const WMT_NO_RIGHTS: WMT_STATUS = 9;
pub const WMT_NO_RIGHTS_EX: WMT_STATUS = 26;
pub const WMT_OFF: WMT_STREAM_SELECTION = 0;
pub type WMT_OFFSET_FORMAT = i32;
pub const WMT_OFFSET_FORMAT_100NS: WMT_OFFSET_FORMAT = 0;
pub const WMT_OFFSET_FORMAT_100NS_APPROXIMATE: WMT_OFFSET_FORMAT = 4;
pub const WMT_OFFSET_FORMAT_FRAME_NUMBERS: WMT_OFFSET_FORMAT = 1;
pub const WMT_OFFSET_FORMAT_PLAYLIST_OFFSET: WMT_OFFSET_FORMAT = 2;
pub const WMT_OFFSET_FORMAT_TIMECODE: WMT_OFFSET_FORMAT = 3;
pub const WMT_ON: WMT_STREAM_SELECTION = 2;
pub const WMT_OPENED: WMT_STATUS = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: u32,
    pub segmentData: WMT_BUFFER_SEGMENT,
}
pub type WMT_PLAY_MODE = i32;
pub const WMT_PLAY_MODE_AUTOSELECT: WMT_PLAY_MODE = 0;
pub const WMT_PLAY_MODE_DOWNLOAD: WMT_PLAY_MODE = 2;
pub const WMT_PLAY_MODE_LOCAL: WMT_PLAY_MODE = 1;
pub const WMT_PLAY_MODE_STREAMING: WMT_PLAY_MODE = 3;
pub const WMT_PREROLL_COMPLETE: WMT_STATUS = 41;
pub const WMT_PREROLL_READY: WMT_STATUS = 40;
pub const WMT_PROTOCOL_HTTP: WMT_NET_PROTOCOL = 0;
pub const WMT_PROXIMITY_COMPLETED: WMT_STATUS = 50;
pub const WMT_PROXIMITY_RESULT: WMT_STATUS = 49;
pub type WMT_PROXY_SETTINGS = i32;
pub const WMT_PROXY_SETTING_AUTO: WMT_PROXY_SETTINGS = 2;
pub const WMT_PROXY_SETTING_BROWSER: WMT_PROXY_SETTINGS = 3;
pub const WMT_PROXY_SETTING_MANUAL: WMT_PROXY_SETTINGS = 1;
pub const WMT_PROXY_SETTING_MAX: WMT_PROXY_SETTINGS = 4;
pub const WMT_PROXY_SETTING_NONE: WMT_PROXY_SETTINGS = 0;
pub const WMT_RECONNECT_END: WMT_STATUS = 36;
pub const WMT_RECONNECT_START: WMT_STATUS = 35;
pub const WMT_RESTRICTED_LICENSE: WMT_STATUS = 31;
pub type WMT_RIGHTS = i32;
pub const WMT_RIGHT_COLLABORATIVE_PLAY: WMT_RIGHTS = 256;
pub const WMT_RIGHT_COPY: WMT_RIGHTS = 128;
pub const WMT_RIGHT_COPY_TO_CD: WMT_RIGHTS = 8;
pub const WMT_RIGHT_COPY_TO_NON_SDMI_DEVICE: WMT_RIGHTS = 2;
pub const WMT_RIGHT_COPY_TO_SDMI_DEVICE: WMT_RIGHTS = 16;
pub const WMT_RIGHT_ONE_TIME: WMT_RIGHTS = 32;
pub const WMT_RIGHT_PLAYBACK: WMT_RIGHTS = 1;
pub const WMT_RIGHT_SAVE_STREAM_PROTECTED: WMT_RIGHTS = 64;
pub const WMT_RIGHT_SDMI_NOMORECOPIES: WMT_RIGHTS = 131072;
pub const WMT_RIGHT_SDMI_TRIGGER: WMT_RIGHTS = 65536;
pub const WMT_SAVEAS_START: WMT_STATUS = 17;
pub const WMT_SAVEAS_STOP: WMT_STATUS = 18;
pub const WMT_SET_FEC_SPAN: WMT_STATUS = 39;
pub const WMT_SOURCE_SWITCH: WMT_STATUS = 22;
pub const WMT_STARTED: WMT_STATUS = 11;
pub type WMT_STATUS = i32;
pub const WMT_STOPPED: WMT_STATUS = 12;
pub type WMT_STORAGE_FORMAT = i32;
pub type WMT_STREAM_SELECTION = i32;
pub const WMT_STRIDING: WMT_STATUS = 14;
pub const WMT_Storage_Format_MP3: WMT_STORAGE_FORMAT = 0;
pub const WMT_Storage_Format_V1: WMT_STORAGE_FORMAT = 1;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: u16,
    pub dwTimecode: u32,
    pub dwUserbits: u32,
    pub dwAmFlags: u32,
}
pub const WMT_TIMECODE_FRAMERATE_24: i32 = 3;
pub const WMT_TIMECODE_FRAMERATE_25: i32 = 2;
pub const WMT_TIMECODE_FRAMERATE_30: i32 = 0;
pub const WMT_TIMECODE_FRAMERATE_30DROP: i32 = 1;
pub const WMT_TIMER: WMT_STATUS = 15;
pub const WMT_TRANSCRYPTOR_CLOSED: WMT_STATUS = 48;
pub const WMT_TRANSCRYPTOR_INIT: WMT_STATUS = 45;
pub const WMT_TRANSCRYPTOR_READ: WMT_STATUS = 47;
pub const WMT_TRANSCRYPTOR_SEEKED: WMT_STATUS = 46;
pub type WMT_TRANSPORT_TYPE = i32;
pub const WMT_TYPE_BINARY: WMT_ATTR_DATATYPE = 2;
pub const WMT_TYPE_BOOL: WMT_ATTR_DATATYPE = 3;
pub const WMT_TYPE_DWORD: WMT_ATTR_DATATYPE = 0;
pub const WMT_TYPE_GUID: WMT_ATTR_DATATYPE = 6;
pub const WMT_TYPE_QWORD: WMT_ATTR_DATATYPE = 4;
pub const WMT_TYPE_STRING: WMT_ATTR_DATATYPE = 1;
pub const WMT_TYPE_WORD: WMT_ATTR_DATATYPE = 5;
pub const WMT_Transport_Type_Reliable: WMT_TRANSPORT_TYPE = 1;
pub const WMT_Transport_Type_Unreliable: WMT_TRANSPORT_TYPE = 0;
pub type WMT_VERSION = i32;
pub const WMT_VER_4_0: WMT_VERSION = 262144;
pub const WMT_VER_7_0: WMT_VERSION = 458752;
pub const WMT_VER_8_0: WMT_VERSION = 524288;
pub const WMT_VER_9_0: WMT_VERSION = 589824;
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: u32 = 65536;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: u32,
    pub cbStruct: u32,
    pub dwControlFlags: u32,
    pub dwInputFlagsCur: u32,
    pub lCurMotionXtoX: i32,
    pub lCurMotionYtoX: i32,
    pub lCurMotionXoffset: i32,
    pub lCurMotionXtoY: i32,
    pub lCurMotionYtoY: i32,
    pub lCurMotionYoffset: i32,
    pub lCurBlendCoef1: i32,
    pub lCurBlendCoef2: i32,
    pub dwInputFlagsPrev: u32,
    pub lPrevMotionXtoX: i32,
    pub lPrevMotionYtoX: i32,
    pub lPrevMotionXoffset: i32,
    pub lPrevMotionXtoY: i32,
    pub lPrevMotionYtoY: i32,
    pub lPrevMotionYoffset: i32,
    pub lPrevBlendCoef1: i32,
    pub lPrevBlendCoef2: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: u32,
    pub dwStructSize: u32,
    pub dwControlFlags: u32,
    pub dwViewportWidth: u32,
    pub dwViewportHeight: u32,
    pub dwCurrImageWidth: u32,
    pub dwCurrImageHeight: u32,
    pub fCurrRegionX0: f32,
    pub fCurrRegionY0: f32,
    pub fCurrRegionWidth: f32,
    pub fCurrRegionHeight: f32,
    pub fCurrBlendCoef: f32,
    pub dwPrevImageWidth: u32,
    pub dwPrevImageHeight: u32,
    pub fPrevRegionX0: f32,
    pub fPrevRegionY0: f32,
    pub fPrevRegionWidth: f32,
    pub fPrevRegionHeight: f32,
    pub fPrevBlendCoef: f32,
    pub dwEffectType: u32,
    pub dwNumEffectParas: u32,
    pub fEffectPara0: f32,
    pub fEffectPara1: f32,
    pub fEffectPara2: f32,
    pub fEffectPara3: f32,
    pub fEffectPara4: f32,
    pub bKeepPrevImage: windows_sys::core::BOOL,
}
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8;
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4;
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1;
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1;
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2;
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8;
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11;
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12;
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15;
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16;
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17;
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18;
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19;
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20;
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21;
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23;
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24;
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27;
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29;
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30;
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: windows_sys::core::GUID,
    pub cbDisplayName: u32,
    pub pwszDisplayName: windows_sys::core::PWSTR,
}
impl Default for WMT_WATERMARK_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WMT_WATERMARK_ENTRY_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: u16,
    pub cbSampleHeaderFixedData: u16,
    pub wVersion: u16,
    pub wReserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: u16,
    pub wPart: u16,
    pub cTotalParts: u16,
    pub wSampleType: u16,
    pub wszURL: [u16; 1],
}
impl Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = 1;
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = 2;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::BITMAPINFOHEADER,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub bmiHeader: super::BITMAPINFOHEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: u32,
    pub dwMask: u32,
}
pub type WM_AETYPE = i32;
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = 101;
pub const WM_AETYPE_INCLUDE: WM_AETYPE = 105;
pub const WM_BACKUP_OVERWRITE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: u32,
    pub dwPort: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: u32,
    pub pwszIPAddress: windows_sys::core::PCWSTR,
    pub pwszPort: windows_sys::core::PCWSTR,
    pub pwszDNSName: windows_sys::core::PCWSTR,
}
impl Default for WM_CLIENT_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WM_CL_INTERLACED420: u32 = 0;
pub const WM_CL_PROGRESSIVE420: u32 = 1;
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32;
pub const WM_CT_INTERLACED: u32 = 128;
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16;
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64;
pub const WM_DM_DEINTERLACE_HALFSIZE: i32 = 2;
pub const WM_DM_DEINTERLACE_HALFSIZEDOUBLERATE: i32 = 3;
pub const WM_DM_DEINTERLACE_INVERSETELECINE: i32 = 4;
pub const WM_DM_DEINTERLACE_NORMAL: i32 = 1;
pub const WM_DM_DEINTERLACE_VERTICALHALFSIZEDOUBLERATE: i32 = 5;
pub const WM_DM_IT_DISABLE_COHERENT_MODE: i32 = 0;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_BOTTOM: i32 = 6;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_TOP: i32 = 1;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_BOTTOM: i32 = 7;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_TOP: i32 = 2;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_BOTTOM: i32 = 8;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_TOP: i32 = 3;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_BOTTOM: i32 = 9;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_TOP: i32 = 4;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_BOTTOM: i32 = 10;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_TOP: i32 = 5;
pub const WM_DM_NOTINTERLACED: i32 = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
pub const WM_MAX_STREAMS: u32 = 63;
pub const WM_MAX_VIDEO_STREAMS: u32 = 63;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WM_MEDIA_TYPE {
    pub majortype: windows_sys::core::GUID,
    pub subtype: windows_sys::core::GUID,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub lSampleSize: u32,
    pub formattype: windows_sys::core::GUID,
    pub pUnk: *mut core::ffi::c_void,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
impl Default for WM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_PICTURE {
    pub pwszMIMEType: windows_sys::core::PWSTR,
    pub bPictureType: u8,
    pub pwszDescription: windows_sys::core::PWSTR,
    pub dwDataLen: u32,
    pub pbData: *mut u8,
}
impl Default for WM_PICTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WM_PLAYBACK_DRC_HIGH: i32 = 0;
pub const WM_PLAYBACK_DRC_LOW: i32 = 2;
pub const WM_PLAYBACK_DRC_MEDIUM: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: u16,
    pub wPortEnd: u16,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: u32,
    pub wszLang: *mut u16,
    pub wszBrowserUserAgent: *mut u16,
    pub wszBrowserWebPage: *mut u16,
    pub qwReserved: u64,
    pub pReserved: *mut super::LPARAM,
    pub wszHostExe: *mut u16,
    pub qwHostVersion: u64,
    pub wszPlayerUserAgent: *mut u16,
}
#[cfg(feature = "minwindef")]
impl Default for WM_READER_CLIENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WM_READER_STATISTICS {
    pub cbSize: u32,
    pub dwBandwidth: u32,
    pub cPacketsReceived: u32,
    pub cPacketsRecovered: u32,
    pub cPacketsLost: u32,
    pub wQuality: u16,
}
pub const WM_RESTORE_INDIVIDUALIZE: u32 = 2;
pub const WM_SFEX_DATALOSS: i32 = 4;
pub const WM_SFEX_NOTASYNCPOINT: i32 = 2;
pub const WM_SF_CLEANPOINT: i32 = 1;
pub const WM_SF_DATALOSS: i32 = 4;
pub const WM_SF_DISCONTINUITY: i32 = 2;
pub const WM_START_CURRENTPOSITION: u64 = 18446744073709551615;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: u16,
    pub fMandatory: windows_sys::core::BOOL,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: windows_sys::core::GUID,
    pub cbFormat: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub pwszContentDescriptor: windows_sys::core::PWSTR,
    pub dwLyricsLen: u32,
    pub pbLyrics: *mut u8,
}
impl Default for WM_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1;
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3;
pub const WM_SampleExtension_ContentType_Size: u32 = 1;
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2;
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2;
pub const WM_SampleExtension_Timecode_Size: u32 = 14;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_USER_TEXT {
    pub pwszDescription: windows_sys::core::PWSTR,
    pub pwszText: windows_sys::core::PWSTR,
}
impl Default for WM_USER_TEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: windows_sys::core::PWSTR,
    pub pwszURL: windows_sys::core::PWSTR,
}
impl Default for WM_USER_WEB_URL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: u64,
    pub qwByteCount: u64,
    pub qwDroppedSampleCount: u64,
    pub qwDroppedByteCount: u64,
    pub dwCurrentBitrate: u32,
    pub dwAverageBitrate: u32,
    pub dwExpectedBitrate: u32,
    pub dwCurrentSampleRate: u32,
    pub dwAverageSampleRate: u32,
    pub dwExpectedSampleRate: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: u32,
    pub dwCurrentSampleDropRateInQueue: u32,
    pub dwCurrentSampleDropRateInCodec: u32,
    pub dwCurrentSampleDropRateInMultiplexer: u32,
    pub dwTotalSampleDropsInQueue: u32,
    pub dwTotalSampleDropsInCodec: u32,
    pub dwTotalSampleDropsInMultiplexer: u32,
}
