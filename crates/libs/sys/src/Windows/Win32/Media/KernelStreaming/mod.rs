#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateAllocator(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateAllocator2(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateClock(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateClock2(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreatePin(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreatePin2(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateTopologyNode(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateTopologyNode2(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AEC_MODE_FULL_DUPLEX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AEC_MODE_HALF_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AEC_MODE_PASS_THROUGH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AEC_STATUS_FD_CURRENTLY_CONVERGED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AEC_STATUS_FD_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AEC_STATUS_FD_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AEC_STATUS_FD_HISTORY_UNINITIALIZED: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct ALLOCATOR_PROPERTIES_EX {
    pub cBuffers: i32,
    pub cbBuffer: i32,
    pub cbAlign: i32,
    pub cbPrefix: i32,
    pub MemoryType: ::windows_sys::core::GUID,
    pub BusType: ::windows_sys::core::GUID,
    pub State: PIPE_STATE,
    pub Input: PIPE_TERMINATION,
    pub Output: PIPE_TERMINATION,
    pub Strategy: u32,
    pub Flags: u32,
    pub Weight: u32,
    pub LogicalMemoryType: KS_LogicalMemoryType,
    pub AllocatorPlace: PIPE_ALLOCATOR_PLACE,
    pub Dimensions: PIPE_DIMENSIONS,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub PrevSegment: *mut IKsAllocatorEx,
    pub CountNextSegments: u32,
    pub NextSegments: *mut *mut IKsAllocatorEx,
    pub InsideFactors: u32,
    pub NumberPins: u32,
}
impl ::core::marker::Copy for ALLOCATOR_PROPERTIES_EX {}
impl ::core::clone::Clone for ALLOCATOR_PROPERTIES_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const APO_CLASS_UUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1502215400, data2: 40144, data3: 18045, data4: [138, 106, 84, 25, 227, 21, 41, 212] };
pub const AUDIOENDPOINT_CLASS_UUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3244708412, data2: 65036, data3: 19092, data4: [165, 134, 241, 168, 12, 251, 191, 62] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOMODULE_MAX_DATA_SIZE: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOMODULE_MAX_NAME_CCH_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type AUDIOPOSTURE_ORIENTATION = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOPOSTURE_ORIENTATION_NOTROTATED: AUDIOPOSTURE_ORIENTATION = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOPOSTURE_ORIENTATION_ROTATED90DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOPOSTURE_ORIENTATION_ROTATED180DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOPOSTURE_ORIENTATION_ROTATED270DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    pub ResourceGroupAcquired: super::super::Foundation::BOOL,
    pub ResourceGroupName: [u16; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type AUDIO_CURVE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIO_CURVE_TYPE_NONE: AUDIO_CURVE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIO_CURVE_TYPE_WINDOWS_FADE: AUDIO_CURVE_TYPE = 1i32;
pub const AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869054, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_AUTOMATIC_GAIN_CONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869056, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_BASS_BOOST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869061, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_BASS_MANAGEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869066, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_BEAMFORMING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869057, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_CONSTANT_TONE_REMOVAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869058, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_DEEP_NOISE_SUPPRESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869072, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_DYNAMIC_RANGE_COMPRESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869070, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_ENVIRONMENTAL_EFFECTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869067, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_EQUALIZER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869059, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_FAR_FIELD_BEAMFORMING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869071, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_LOUDNESS_EQUALIZER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869060, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_NOISE_SUPPRESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869055, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_ROOM_CORRECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869065, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_SPEAKER_COMPENSATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869069, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_SPEAKER_FILL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869064, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_SPEAKER_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869068, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_VIRTUAL_HEADPHONES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869063, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_EFFECT_TYPE_VIRTUAL_SURROUND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1868869062, data2: 33297, data3: 4578, data4: [140, 112, 44, 39, 215, 240, 1, 250] };
pub const AUDIO_SIGNALPROCESSINGMODE_COMMUNICATIONS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2559906611, data2: 47565, data3: 18609, data4: [160, 163, 255, 64, 104, 45, 115, 247] };
pub const AUDIO_SIGNALPROCESSINGMODE_DEFAULT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3247320958, data2: 37693, data3: 18789, data4: [183, 209, 30, 239, 34, 141, 42, 243] };
pub const AUDIO_SIGNALPROCESSINGMODE_FAR_FIELD_SPEECH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 680795322, data2: 15334, data3: 19064, data4: [154, 118, 48, 253, 145, 85, 155, 100] };
pub const AUDIO_SIGNALPROCESSINGMODE_MEDIA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1199571022, data2: 28979, data3: 16856, data4: [140, 116, 102, 13, 173, 210, 192, 238] };
pub const AUDIO_SIGNALPROCESSINGMODE_MOVIE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2993679117, data2: 60564, data3: 18300, data4: [148, 148, 209, 171, 142, 117, 63, 110] };
pub const AUDIO_SIGNALPROCESSINGMODE_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2633148171, data2: 62327, data3: 16443, data4: [189, 107, 54, 8, 99, 224, 53, 92] };
pub const AUDIO_SIGNALPROCESSINGMODE_RAW: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2660297248, data2: 46227, data3: 20433, data4: [161, 168, 126, 19, 97, 169, 86, 207] };
pub const AUDIO_SIGNALPROCESSINGMODE_SPEECH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4229758107, data2: 47574, data3: 19706, data4: [181, 224, 75, 178, 22, 104, 120, 178] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AllocatorStrategy_DontCare: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AllocatorStrategy_MaximizeSpeed: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AllocatorStrategy_MinimizeFrameSize: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AllocatorStrategy_MinimizeNumberOfAllocators: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AllocatorStrategy_MinimizeNumberOfFrames: u32 = 1u32;
pub const BLUETOOTHLE_MIDI_SERVICE_UUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 62393946, data2: 60904, data3: 19251, data4: [167, 81, 108, 227, 78, 196, 199, 0] };
pub const BLUETOOTH_MIDI_DATAIO_CHARACTERISTIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2004018651, data2: 14440, data3: 16658, data4: [161, 169, 242, 102, 157, 16, 107, 243] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const BUS_INTERFACE_REFERENCE_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type CAPTURE_MEMORY_ALLOCATION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_INVALID: CAPTURE_MEMORY_ALLOCATION_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_SYSTEM: CAPTURE_MEMORY_ALLOCATION_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_VRAM: CAPTURE_MEMORY_ALLOCATION_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_SYSTEM_AGP: CAPTURE_MEMORY_ALLOCATION_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_VRAM_MAPPED: CAPTURE_MEMORY_ALLOCATION_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_SECURE_BUFFER: CAPTURE_MEMORY_ALLOCATION_FLAGS = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct CC_BYTE_PAIR {
    pub Decoded: [u8; 2],
    pub Reserved: u16,
}
impl ::core::marker::Copy for CC_BYTE_PAIR {}
impl ::core::clone::Clone for CC_BYTE_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct CC_HW_FIELD {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub fieldFlags: u32,
    pub PictureNumber: i64,
    pub Lines: [CC_BYTE_PAIR; 12],
}
impl ::core::marker::Copy for CC_HW_FIELD {}
impl ::core::clone::Clone for CC_HW_FIELD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const CC_MAX_HW_DECODE_LINES: u32 = 12u32;
pub const CLSID_KsIBasicAudioInterfaceHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3120081982, data2: 3953, data3: 4562, data4: [183, 44, 0, 192, 79, 182, 189, 61] };
pub const CLSID_Proxy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 399288091, data2: 60631, data3: 4560, data4: [185, 8, 0, 160, 201, 34, 49, 150] };
pub const CODECAPI_ALLSETTINGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1784118930, data2: 33761, data3: 16659, data4: [173, 194, 79, 206, 195, 47, 131, 161] };
pub const CODECAPI_AUDIO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3117521470, data2: 63639, data3: 17052, data4: [188, 70, 129, 56, 183, 39, 43, 45] };
pub const CODECAPI_CHANGELISTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1655777999, data2: 63152, data3: 18393, data4: [148, 86, 150, 242, 44, 78, 11, 157] };
pub const CODECAPI_CURRENTCHANGELIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 481382019, data2: 32114, data3: 18007, data4: [131, 253, 71, 162, 197, 185, 209, 61] };
pub const CODECAPI_SETALLDEFAULTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1818126972, data2: 44280, data3: 20309, data4: [169, 153, 26, 98, 129, 9, 5, 27] };
pub const CODECAPI_SUPPORTSEVENTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 92385175, data2: 30355, data3: 19901, data4: [157, 202, 63, 158, 189, 101, 133, 161] };
pub const CODECAPI_VIDEO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1897064673, data2: 15619, data3: 18415, data4: [142, 96, 3, 241, 207, 83, 115, 1] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type CONSTRICTOR_OPTION = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const CONSTRICTOR_OPTION_DISABLE: CONSTRICTOR_OPTION = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const CONSTRICTOR_OPTION_MUTE: CONSTRICTOR_OPTION = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct DEVCAPS {
    pub CanRecord: i32,
    pub CanRecordStrobe: i32,
    pub HasAudio: i32,
    pub HasVideo: i32,
    pub UsesFiles: i32,
    pub CanSave: i32,
    pub DeviceType: i32,
    pub TCRead: i32,
    pub TCWrite: i32,
    pub CTLRead: i32,
    pub IndexRead: i32,
    pub Preroll: i32,
    pub Postroll: i32,
    pub SyncAcc: i32,
    pub NormRate: i32,
    pub CanPreview: i32,
    pub CanMonitorSrc: i32,
    pub CanTest: i32,
    pub VideoIn: i32,
    pub AudioIn: i32,
    pub Calibrate: i32,
    pub SeekType: i32,
    pub SimulatedHardware: i32,
}
impl ::core::marker::Copy for DEVCAPS {}
impl ::core::clone::Clone for DEVCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_KsAudio_Controller_DeviceInterface_Path: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 333448406, data2: 45158, data3: 17341, data4: [145, 59, 164, 21, 205, 19, 218, 135] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_KsAudio_PacketSize_Constraints: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 333448406, data2: 45158, data3: 17341, data4: [145, 59, 164, 21, 205, 19, 218, 135] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_KsAudio_PacketSize_Constraints2: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 2483353473, data2: 29073, data3: 16539, data4: [139, 11, 128, 191, 110, 194, 41, 174] }, pid: 2u32 };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct DS3DVECTOR {
    pub Anonymous1: DS3DVECTOR_0,
    pub Anonymous2: DS3DVECTOR_1,
    pub Anonymous3: DS3DVECTOR_2,
}
impl ::core::marker::Copy for DS3DVECTOR {}
impl ::core::clone::Clone for DS3DVECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union DS3DVECTOR_0 {
    pub x: f32,
    pub dvX: f32,
}
impl ::core::marker::Copy for DS3DVECTOR_0 {}
impl ::core::clone::Clone for DS3DVECTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union DS3DVECTOR_1 {
    pub y: f32,
    pub dvY: f32,
}
impl ::core::marker::Copy for DS3DVECTOR_1 {}
impl ::core::clone::Clone for DS3DVECTOR_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union DS3DVECTOR_2 {
    pub z: f32,
    pub dvZ: f32,
}
impl ::core::marker::Copy for DS3DVECTOR_2 {}
impl ::core::clone::Clone for DS3DVECTOR_2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ENCAPIPARAM_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238125635, data2: 51843, data3: 19156, data4: [169, 175, 243, 105, 106, 246, 102, 223] };
pub const ENCAPIPARAM_BITRATE_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3999248988, data2: 50963, data3: 16593, data4: [157, 88, 192, 215, 36, 30, 37, 15] };
pub const ENCAPIPARAM_PEAK_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1883182761, data2: 15688, data3: 17569, data4: [176, 119, 1, 141, 255, 145, 93, 25] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type EPcxConnectionType = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeUnknown: EPcxConnectionType = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnType3Point5mm: EPcxConnectionType = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeQuarter: EPcxConnectionType = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeAtapiInternal: EPcxConnectionType = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeRCA: EPcxConnectionType = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeOptical: EPcxConnectionType = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeOtherDigital: EPcxConnectionType = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeOtherAnalog: EPcxConnectionType = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeMultichannelAnalogDIN: EPcxConnectionType = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeXlrProfessional: EPcxConnectionType = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeRJ11Modem: EPcxConnectionType = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeCombination: EPcxConnectionType = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type EPcxGenLocation = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGenLocPrimaryBox: EPcxGenLocation = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGenLocInternal: EPcxGenLocation = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGenLocSeparate: EPcxGenLocation = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGenLocOther: EPcxGenLocation = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const EPcxGenLocation_enum_count: EPcxGenLocation = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type EPcxGeoLocation = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocRear: EPcxGeoLocation = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocFront: EPcxGeoLocation = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocLeft: EPcxGeoLocation = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocRight: EPcxGeoLocation = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocTop: EPcxGeoLocation = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocBottom: EPcxGeoLocation = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocRearPanel: EPcxGeoLocation = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocRiser: EPcxGeoLocation = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocInsideMobileLid: EPcxGeoLocation = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocDrivebay: EPcxGeoLocation = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocHDMI: EPcxGeoLocation = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocOutsideMobileLid: EPcxGeoLocation = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocATAPI: EPcxGeoLocation = 13i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocNotApplicable: EPcxGeoLocation = 14i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocReserved6: EPcxGeoLocation = 15i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const EPcxGeoLocation_enum_count: EPcxGeoLocation = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type EPxcPortConnection = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const ePortConnJack: EPxcPortConnection = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const ePortConnIntegratedDevice: EPxcPortConnection = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const ePortConnBothIntegratedAndJack: EPxcPortConnection = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const ePortConnUnknown: EPxcPortConnection = 3i32;
pub const EVENTSETID_CROSSBAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401153, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const EVENTSETID_TUNER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401094, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const EVENTSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 803208285, data2: 50994, data3: 19366, data4: [181, 223, 107, 77, 127, 200, 139, 139] };
pub const EVENTSETID_VIDEODECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401121, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type FRAMING_CACHE_OPS = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Framing_Cache_Update: FRAMING_CACHE_OPS = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Framing_Cache_ReadLast: FRAMING_CACHE_OPS = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Framing_Cache_ReadOrig: FRAMING_CACHE_OPS = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Framing_Cache_Write: FRAMING_CACHE_OPS = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type FRAMING_PROP = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FramingProp_Uninitialized: FRAMING_PROP = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FramingProp_None: FRAMING_PROP = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FramingProp_Old: FRAMING_PROP = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FramingProp_Ex: FRAMING_PROP = 3i32;
pub const GUID_NULL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub type IKsAggregateControl = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IKsAllocator(pub u8);
#[repr(C)]
pub struct IKsAllocatorEx(pub u8);
pub type IKsControl = *mut ::core::ffi::c_void;
pub type IKsFormatSupport = *mut ::core::ffi::c_void;
pub type IKsJackContainerId = *mut ::core::ffi::c_void;
pub type IKsJackDescription = *mut ::core::ffi::c_void;
pub type IKsJackDescription2 = *mut ::core::ffi::c_void;
pub type IKsJackSinkInformation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IKsPin(pub u8);
pub type IKsPropertySet = *mut ::core::ffi::c_void;
pub type IKsTopology = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    pub Size: u32,
    pub PrimaryChannelCount: u32,
    pub PrimaryChannelStartPosition: u32,
    pub PrimaryChannelMask: u32,
    pub InterleavedChannelCount: u32,
    pub InterleavedChannelStartPosition: u32,
    pub InterleavedChannelMask: u32,
}
impl ::core::marker::Copy for INTERLEAVED_AUDIO_FORMAT_INFORMATION {}
impl ::core::clone::Clone for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const IOCTL_KS_DISABLE_EVENT: u32 = 3080203u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const IOCTL_KS_ENABLE_EVENT: u32 = 3080199u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const IOCTL_KS_HANDSHAKE: u32 = 3080223u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const IOCTL_KS_METHOD: u32 = 3080207u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const IOCTL_KS_PROPERTY: u32 = 3080195u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const IOCTL_KS_READ_STREAM: u32 = 3096599u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const IOCTL_KS_RESET_STATE: u32 = 3080219u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const IOCTL_KS_WRITE_STREAM: u32 = 3112979u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const JACKDESC2_DYNAMIC_FORMAT_CHANGE_CAPABILITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const JACKDESC2_PRESENCE_DETECT_CAPABILITY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSAC3_ALTERNATE_AUDIO {
    pub fStereo: super::super::Foundation::BOOL,
    pub DualMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAC3_ALTERNATE_AUDIO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAC3_ALTERNATE_AUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_ALTERNATE_AUDIO_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_ALTERNATE_AUDIO_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_ALTERNATE_AUDIO_BOTH: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAC3_BIT_STREAM_MODE {
    pub BitStreamMode: i32,
}
impl ::core::marker::Copy for KSAC3_BIT_STREAM_MODE {}
impl ::core::clone::Clone for KSAC3_BIT_STREAM_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAC3_DIALOGUE_LEVEL {
    pub DialogueLevel: u32,
}
impl ::core::marker::Copy for KSAC3_DIALOGUE_LEVEL {}
impl ::core::clone::Clone for KSAC3_DIALOGUE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSAC3_DOWNMIX {
    pub fDownMix: super::super::Foundation::BOOL,
    pub fDolbySurround: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAC3_DOWNMIX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAC3_DOWNMIX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSAC3_ERROR_CONCEALMENT {
    pub fRepeatPreviousBlock: super::super::Foundation::BOOL,
    pub fErrorInCurrentBlock: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAC3_ERROR_CONCEALMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAC3_ERROR_CONCEALMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSAC3_ROOM_TYPE {
    pub fLargeRoom: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAC3_ROOM_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAC3_ROOM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_SERVICE_COMMENTARY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_SERVICE_DIALOG_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_SERVICE_EMERGENCY_FLASH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_SERVICE_HEARING_IMPAIRED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_SERVICE_MAIN_AUDIO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_SERVICE_NO_DIALOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_SERVICE_VISUALLY_IMPAIRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAC3_SERVICE_VOICE_OVER: u32 = 7u32;
pub const KSALGORITHMINSTANCE_SYSTEM_ACOUSTIC_ECHO_CANCEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 472040813, data2: 39033, data3: 20315, data4: [163, 137, 39, 153, 109, 220, 40, 16] };
pub const KSALGORITHMINSTANCE_SYSTEM_AGC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2500744633, data2: 34684, data3: 19559, data4: [190, 8, 228, 123, 86, 17, 19, 10] };
pub const KSALGORITHMINSTANCE_SYSTEM_MICROPHONE_ARRAY_PROCESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3069550752, data2: 40545, data3: 20364, data4: [145, 227, 118, 207, 15, 60, 71, 31] };
pub const KSALGORITHMINSTANCE_SYSTEM_NOISE_SUPPRESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1521518638, data2: 29300, data3: 17686, data4: [135, 125, 78, 238, 153, 186, 79, 208] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSALLOCATORMODE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KsAllocatorMode_User: KSALLOCATORMODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KsAllocatorMode_Kernel: KSALLOCATORMODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_2D_BUFFER_REQUIRED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_ALLOCATOR_EXISTS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_ATTENTION_STEPPING: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_CAN_ALLOCATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_CYCLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_DEVICE_SPECIFIC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_ENABLE_CACHED_MDL: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_INDEPENDENT_RANGES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_INSIST_ON_FRAMESIZE_RATIO: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_MULTIPLE_OUTPUT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_NO_FRAME_INTEGRITY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_FLAG_PARTIAL_READ_SUPPORT: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSALLOCATOR_FRAMING {
    pub Anonymous1: KSALLOCATOR_FRAMING_0,
    pub PoolType: u32,
    pub Frames: u32,
    pub FrameSize: u32,
    pub Anonymous2: KSALLOCATOR_FRAMING_1,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSALLOCATOR_FRAMING {}
impl ::core::clone::Clone for KSALLOCATOR_FRAMING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSALLOCATOR_FRAMING_0 {
    pub OptionsFlags: u32,
    pub RequirementsFlags: u32,
}
impl ::core::marker::Copy for KSALLOCATOR_FRAMING_0 {}
impl ::core::clone::Clone for KSALLOCATOR_FRAMING_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSALLOCATOR_FRAMING_1 {
    pub FileAlignment: u32,
    pub FramePitch: i32,
}
impl ::core::marker::Copy for KSALLOCATOR_FRAMING_1 {}
impl ::core::clone::Clone for KSALLOCATOR_FRAMING_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSALLOCATOR_FRAMING_EX {
    pub CountItems: u32,
    pub PinFlags: u32,
    pub OutputCompression: KS_COMPRESSION,
    pub PinWeight: u32,
    pub FramingItem: [KS_FRAMING_ITEM; 1],
}
impl ::core::marker::Copy for KSALLOCATOR_FRAMING_EX {}
impl ::core::clone::Clone for KSALLOCATOR_FRAMING_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_OPTIONF_COMPATIBLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_OPTIONF_SYSTEM_MEMORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_OPTIONF_VALID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_REQUIREMENTF_FRAME_INTEGRITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_REQUIREMENTF_INPLACE_MODIFIER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_REQUIREMENTF_MUST_ALLOCATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_REQUIREMENTF_PREFERENCES_ONLY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_REQUIREMENTF_SYSTEM_MEMORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSALLOCATOR_REQUIREMENTF_SYSTEM_MEMORY_CUSTOM_ALLOCATION: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSATTRIBUTE {
    pub Size: u32,
    pub Flags: u32,
    pub Attribute: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for KSATTRIBUTE {}
impl ::core::clone::Clone for KSATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSATTRIBUTEID_AUDIOSIGNALPROCESSING_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3791167157, data2: 24390, data3: 16795, data4: [150, 123, 255, 103, 112, 185, 132, 1] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    pub AttributeHeader: KSATTRIBUTE,
    pub SignalProcessingMode: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {}
impl ::core::clone::Clone for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSATTRIBUTE_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDDECOUTMODE_PCM_51: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDDECOUTMODE_SPDIFF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDDECOUTMODE_STEREO_ANALOG: u32 = 1u32;
pub const KSAUDFNAME_3D_CENTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2667999412, data2: 39199, data3: 4562, data4: [172, 77, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_3D_DEPTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1677678407, data2: 39199, data3: 4562, data4: [172, 77, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_3D_STEREO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940002, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_ALTERNATE_MICROPHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 734207339, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_AUX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940030, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_AUX_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940029, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_AUX_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940028, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_BASS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940000, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_CD_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940027, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_CD_IN_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940019, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_CD_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940010, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_CD_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940009, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_LINE_IN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940025, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_LINE_IN_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940020, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_LINE_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940012, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_LINE_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940011, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MASTER_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940004, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MASTER_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940003, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MICROPHONE_BOOST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 734207338, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_MIC_IN_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940021, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MIC_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940014, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MIC_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940013, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MIDI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940024, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MIDI_IN_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940018, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MIDI_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940008, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MIDI_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940007, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_MIDRANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2731271288, data2: 44676, data3: 18849, data4: [139, 114, 74, 208, 155, 120, 237, 52] };
pub const KSAUDFNAME_MONO_MIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 14676088, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_MONO_MIX_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 734207337, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_MONO_MIX_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 582019838, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_MONO_OUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4189330883, data2: 38626, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_MONO_OUT_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 449988588, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_MONO_OUT_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 449988587, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_PC_SPEAKER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940031, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_PC_SPEAKER_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940017, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_PC_SPEAKER_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940016, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_PEAKMETER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1474446144, data2: 64603, data3: 17938, data4: [165, 98, 114, 177, 26, 41, 223, 174] };
pub const KSAUDFNAME_RECORDING_CONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940026, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_RECORDING_SOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940015, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_STEREO_MIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 14676087, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_STEREO_MIX_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 582019837, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_STEREO_MIX_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 449988589, data2: 38627, data3: 4562, data4: [172, 76, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_TREBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940001, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2438835908, data2: 42036, data3: 4562, data4: [172, 82, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_VIDEO_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2605115145, data2: 39210, data3: 4562, data4: [172, 77, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_VIDEO_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2605115144, data2: 39210, data3: 4562, data4: [172, 77, 0, 192, 79, 142, 251, 104] };
pub const KSAUDFNAME_VOLUME_CONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940023, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_WAVE_IN_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940022, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_WAVE_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940006, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_WAVE_OUT_MIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940032, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
pub const KSAUDFNAME_WAVE_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408940005, data2: 39173, data3: 4561, data4: [149, 169, 0, 192, 79, 185, 37, 211] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    pub MinBufferBytes: u32,
    pub MaxBufferBytes: u32,
}
impl ::core::marker::Copy for KSAUDIOENGINE_BUFFER_SIZE_RANGE {}
impl ::core::clone::Clone for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOENGINE_DESCRIPTOR {
    pub nHostPinId: u32,
    pub nOffloadPinId: u32,
    pub nLoopbackPinId: u32,
}
impl ::core::marker::Copy for KSAUDIOENGINE_DESCRIPTOR {}
impl ::core::clone::Clone for KSAUDIOENGINE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOENGINE_VOLUMELEVEL {
    pub TargetVolume: i32,
    pub CurveType: AUDIO_CURVE_TYPE,
    pub CurveDuration: u64,
}
impl ::core::marker::Copy for KSAUDIOENGINE_VOLUMELEVEL {}
impl ::core::clone::Clone for KSAUDIOENGINE_VOLUMELEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOMODULE_DESCRIPTOR {
    pub ClassId: ::windows_sys::core::GUID,
    pub InstanceId: u32,
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub Name: [u16; 128],
}
impl ::core::marker::Copy for KSAUDIOMODULE_DESCRIPTOR {}
impl ::core::clone::Clone for KSAUDIOMODULE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOMODULE_NOTIFICATION {
    pub Anonymous: KSAUDIOMODULE_NOTIFICATION_0,
}
impl ::core::marker::Copy for KSAUDIOMODULE_NOTIFICATION {}
impl ::core::clone::Clone for KSAUDIOMODULE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSAUDIOMODULE_NOTIFICATION_0 {
    pub ProviderId: KSAUDIOMODULE_NOTIFICATION_0_0,
    pub Alignment: i64,
}
impl ::core::marker::Copy for KSAUDIOMODULE_NOTIFICATION_0 {}
impl ::core::clone::Clone for KSAUDIOMODULE_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOMODULE_NOTIFICATION_0_0 {
    pub DeviceId: ::windows_sys::core::GUID,
    pub ClassId: ::windows_sys::core::GUID,
    pub InstanceId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSAUDIOMODULE_NOTIFICATION_0_0 {}
impl ::core::clone::Clone for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOMODULE_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub ClassId: ::windows_sys::core::GUID,
    pub InstanceId: u32,
}
impl ::core::marker::Copy for KSAUDIOMODULE_PROPERTY {}
impl ::core::clone::Clone for KSAUDIOMODULE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_CHANNEL_CONFIG {
    pub ActiveSpeakerPositions: i32,
}
impl ::core::marker::Copy for KSAUDIO_CHANNEL_CONFIG {}
impl ::core::clone::Clone for KSAUDIO_CHANNEL_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSAUDIO_COPY_PROTECTION {
    pub fCopyrighted: super::super::Foundation::BOOL,
    pub fOriginal: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAUDIO_COPY_PROTECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAUDIO_COPY_PROTECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_CPU_RESOURCES_HOST_CPU: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_CPU_RESOURCES_NOT_HOST_CPU: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_DYNAMIC_RANGE {
    pub QuietCompression: u32,
    pub LoudCompression: u32,
}
impl ::core::marker::Copy for KSAUDIO_DYNAMIC_RANGE {}
impl ::core::clone::Clone for KSAUDIO_DYNAMIC_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_MICROPHONE_COORDINATES {
    pub usType: u16,
    pub wXCoord: i16,
    pub wYCoord: i16,
    pub wZCoord: i16,
    pub wVerticalAngle: i16,
    pub wHorizontalAngle: i16,
}
impl ::core::marker::Copy for KSAUDIO_MICROPHONE_COORDINATES {}
impl ::core::clone::Clone for KSAUDIO_MICROPHONE_COORDINATES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_MIC_ARRAY_GEOMETRY {
    pub usVersion: u16,
    pub usMicArrayType: u16,
    pub wVerticalAngleBegin: i16,
    pub wVerticalAngleEnd: i16,
    pub wHorizontalAngleBegin: i16,
    pub wHorizontalAngleEnd: i16,
    pub usFrequencyBandLo: u16,
    pub usFrequencyBandHi: u16,
    pub usNumberOfMicrophones: u16,
    pub KsMicCoord: [KSAUDIO_MICROPHONE_COORDINATES; 1],
}
impl ::core::marker::Copy for KSAUDIO_MIC_ARRAY_GEOMETRY {}
impl ::core::clone::Clone for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSAUDIO_MIXCAP_TABLE {
    pub InputChannels: u32,
    pub OutputChannels: u32,
    pub Capabilities: [KSAUDIO_MIX_CAPS; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAUDIO_MIXCAP_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAUDIO_MIXCAP_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSAUDIO_MIXLEVEL {
    pub Mute: super::super::Foundation::BOOL,
    pub Level: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAUDIO_MIXLEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAUDIO_MIXLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSAUDIO_MIX_CAPS {
    pub Mute: super::super::Foundation::BOOL,
    pub Minimum: i32,
    pub Maximum: i32,
    pub Anonymous: KSAUDIO_MIX_CAPS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAUDIO_MIX_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAUDIO_MIX_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KSAUDIO_MIX_CAPS_0 {
    pub Reset: i32,
    pub Resolution: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSAUDIO_MIX_CAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSAUDIO_MIX_CAPS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS {
    pub MinPacketPeriodInHns: u32,
    pub PacketSizeFileAlignment: u32,
    pub Reserved: u32,
    pub NumProcessingModeConstraints: u32,
    pub ProcessingModeConstraints: [_KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT; 1],
}
impl ::core::marker::Copy for KSAUDIO_PACKETSIZE_CONSTRAINTS {}
impl ::core::clone::Clone for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    pub MinPacketPeriodInHns: u32,
    pub PacketSizeFileAlignment: u32,
    pub MaxPacketSizeInBytes: u32,
    pub NumProcessingModeConstraints: u32,
    pub ProcessingModeConstraints: [_KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT; 1],
}
impl ::core::marker::Copy for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {}
impl ::core::clone::Clone for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_POSITION {
    pub PlayOffset: u64,
    pub WriteOffset: u64,
}
impl ::core::marker::Copy for KSAUDIO_POSITION {}
impl ::core::clone::Clone for KSAUDIO_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_POSITIONEX {
    pub TimerFrequency: i64,
    pub TimeStamp1: i64,
    pub Position: KSAUDIO_POSITION,
    pub TimeStamp2: i64,
}
impl ::core::marker::Copy for KSAUDIO_POSITIONEX {}
impl ::core::clone::Clone for KSAUDIO_POSITIONEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIO_PRESENTATION_POSITION {
    pub u64PositionInBlocks: u64,
    pub u64QPCPosition: u64,
}
impl ::core::marker::Copy for KSAUDIO_PRESENTATION_POSITION {}
impl ::core::clone::Clone for KSAUDIO_PRESENTATION_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_QUALITY_ADVANCED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_QUALITY_BASIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_QUALITY_PC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_QUALITY_WORST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_DIRECTOUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_GROUND_FRONT_CENTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_GROUND_FRONT_LEFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_GROUND_FRONT_RIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_GROUND_REAR_LEFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_GROUND_REAR_RIGHT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_MONO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_SUPER_WOOFER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_SPEAKER_TOP_MIDDLE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_HEADPHONE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MAX: u32 = 180u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_NARROW: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_WIDE: u32 = 20u32;
pub const KSCAMERAPROFILE_BalancedVideoAndPhoto: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1800581143, data2: 17095, data3: 18977, data4: [191, 227, 35, 240, 9, 20, 152, 135] };
pub const KSCAMERAPROFILE_CompressedCamera: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 238341569, data2: 10157, data3: 17279, data4: [171, 222, 2, 182, 41, 243, 123, 68] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERAPROFILE_FLAGS_FACEDETECTION: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERAPROFILE_FLAGS_PHOTOHDR: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERAPROFILE_FLAGS_PREVIEW_RES_MUSTMATCH: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERAPROFILE_FLAGS_VARIABLEPHOTOSEQUENCE: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERAPROFILE_FLAGS_VIDEOHDR: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERAPROFILE_FLAGS_VIDEOSTABLIZATION: u64 = 1u64;
pub const KSCAMERAPROFILE_FaceAuth_Mode: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2167806754, data2: 28683, data3: 17734, data4: [162, 212, 197, 46, 144, 123, 252, 39] };
pub const KSCAMERAPROFILE_HDRWithWCGPhoto: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2616652287, data2: 46421, data3: 17957, data4: [179, 38, 164, 109, 239, 49, 143, 183] };
pub const KSCAMERAPROFILE_HDRWithWCGVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1260897078, data2: 18724, data3: 18825, data4: [185, 148, 253, 175, 29, 199, 205, 133] };
pub const KSCAMERAPROFILE_HighFrameRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1450074387, data2: 35893, data3: 18663, data4: [184, 159, 210, 63, 220, 18, 25, 220] };
pub const KSCAMERAPROFILE_HighQualityPhoto: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 843319077, data2: 38427, data3: 19619, data4: [181, 178, 133, 78, 113, 157, 158, 27] };
pub const KSCAMERAPROFILE_Legacy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3028897153, data2: 25271, data3: 20204, data4: [135, 64, 128, 101, 140, 74, 157, 62] };
pub const KSCAMERAPROFILE_PhotoSequence: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 37330333, data2: 20200, data3: 18874, data4: [188, 7, 95, 241, 86, 83, 20, 19] };
pub const KSCAMERAPROFILE_VariablePhotoSequence: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2683489110, data2: 59226, data3: 18865, data4: [169, 40, 153, 133, 213, 148, 111, 135] };
pub const KSCAMERAPROFILE_VideoConferencing: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3309587080, data2: 57791, data3: 17815, data4: [178, 221, 158, 30, 173, 134, 75, 184] };
pub const KSCAMERAPROFILE_VideoHDR8: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3572757740, data2: 48639, data3: 17172, data4: [177, 212, 0, 142, 40, 31, 116, 231] };
pub const KSCAMERAPROFILE_VideoRecording: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2699368424, data2: 36748, data3: 20335, data4: [154, 87, 70, 252, 47, 100, 126, 192] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_AUTO: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_FNF: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_HDR: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_ULTRALOWLIGHT: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_BLUR: u64 = 1u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    pub Resolution: super::super::Foundation::SIZE,
    pub MaxFrameRate: KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0,
    pub MaskResolution: super::super::Foundation::SIZE,
    pub SubType: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    pub Numerator: i32,
    pub Denominator: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_MASK: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_OFF: u64 = 0u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    pub PitchAngle: i32,
    pub YawAngle: i32,
    pub Flag: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_CAPS_ASYNCCONTROL: u64 = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_CAPS_CANCELLABLE: u64 = 4611686018427387904u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_CAPS_MASK: u64 = 18374686479671623680u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_CAPS_RESERVED: u64 = 18374686479671623680u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_AUTOFACEFRAMING: u64 = 1u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    pub ResolutionX: i32,
    pub ResolutionY: i32,
    pub PorchTop: i32,
    pub PorchLeft: i32,
    pub PorchBottom: i32,
    pub PorchRight: i32,
    pub NonUpscalingWindowSize: i32,
    pub MinWindowSize: i32,
    pub MaxWindowSize: i32,
    pub Reserved: i32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    pub Size: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_MANUAL: u64 = 0u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    pub OriginX: i32,
    pub OriginY: i32,
    pub WindowSize: i32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Value: i32,
    pub Reserved: u64,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_FULLSTEP: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_HALFSTEP: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_QUARTERSTEP: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_SIXTHSTEP: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_THIRDSTEP: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_ALTERNATIVE_FRAME_ILLUMINATION: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_BACKGROUND_SUBTRACTION: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_DISABLED: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_BLINK: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PHOTO: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PREVIEW: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_SMILE: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_VIDEO: u64 = 2u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    pub NormalizedFocalLengthX: u32,
    pub NormalizedFocalLengthY: u32,
    pub Flag: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FILTERSCOPE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLAG_CANCELOPERATION: u64 = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLAG_MASK: u64 = 18374686479671623680u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_AUTO: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_ON: u64 = 128u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO_ADJUSTABLEPOWER: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_MULTIFLASHSUPPORTED: u64 = 64u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON_ADJUSTABLEPOWER: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_REDEYEREDUCTION: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_SINGLEFLASH: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSCAMERA_EXTENDEDPROP_FOCUSSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_UNINITIALIZED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_LOST: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_SEARCHING: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FOCUSED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FAILED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUS: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUSLOCK: u64 = 512u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_HYPERFOCAL: u64 = 33554432u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_INFINITY: u64 = 16777216u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_NEAREST: u64 = 67108864u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DRIVERFALLBACK_OFF: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_FULLRANGE: u64 = 262144u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_HYPERFOCAL: u64 = 1048576u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_INFINITY: u64 = 524288u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_MACRO: u64 = 65536u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_NORMAL: u64 = 131072u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_REGIONBASED: u64 = 4096u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_UNLOCK: u64 = 1024u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_HEADER {
    pub Version: u32,
    pub PinId: u32,
    pub Size: u32,
    pub Result: u32,
    pub Flags: u64,
    pub Capability: u64,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_HEADER {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALTERNATING_FRAME_ILLUMINATION: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALWAYS_ON: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_OFF: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_100: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_12800: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_1600: u64 = 128u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_200: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_25600: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_3200: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_400: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_50: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_6400: u64 = 512u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_80: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_800: u64 = 64u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_AUTO: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_MANUAL: u64 = 36028797018963968u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_METADATAINFO {
    pub BufferAlignment: i32,
    pub MaxMetadataBufferSize: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_METADATAINFO {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_ALIGNMENTREQUIRED: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_MEMORYTYPE_MASK: u64 = 255u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_SYSTEMMEMORY: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSCAMERA_EXTENDEDPROP_MetadataAlignment = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_16: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_32: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_64: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_128: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_256: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_512: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_1024: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_2048: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_4096: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_8192: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 13i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OIS_AUTO: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OIS_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OIS_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_DEFAULT: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_LATENCY: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PHOTO: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_POWER: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_QUALITY: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_VIDEO: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_ON: u64 = 1u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    pub RequestedHistoryFrames: u32,
    pub MaxHistoryFrames: u32,
    pub SubMode: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_PHOTOMODE {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_NORMAL: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_VARIABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_16X: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_2X: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_4X: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_8X: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_DISABLE: u64 = 0u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_PROFILE {
    pub ProfileId: ::windows_sys::core::GUID,
    pub Index: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_PROFILE {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_DYNAMIC: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSCAMERA_EXTENDEDPROP_ROITYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_UNKNOWN: KSCAMERA_EXTENDEDPROP_ROITYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_FACE: KSCAMERA_EXTENDEDPROP_ROITYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    pub ControlId: u32,
    pub MaxNumberOfROIs: u32,
    pub Capability: u64,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    pub Size: u32,
    pub ConfigCapCount: u32,
    pub Reserved: u64,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_ROI_INFO {
    pub Region: super::super::Foundation::RECT,
    pub Flags: u64,
    pub Weight: i32,
    pub RegionOfInterestType: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    pub ControlId: u32,
    pub ROICount: u32,
    pub Result: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    pub Size: u32,
    pub ControlCount: u32,
    pub Reserved: u64,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_AUTO: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BACKLIT: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BEACH: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_CANDLELIGHT: u64 = 128u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_LANDSCAPE: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MACRO: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MANUAL: u64 = 36028797018963968u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHT: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHTPORTRAIT: u64 = 512u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_PORTRAIT: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SNOW: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SPORT: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SUNSET: u64 = 64u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_DISABLED: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_ENABLED: u64 = 2u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_VALUE {
    pub Value: KSCAMERA_EXTENDEDPROP_VALUE_0,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_VALUE {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSCAMERA_EXTENDEDPROP_VALUE_0 {
    pub dbl: f64,
    pub ull: u64,
    pub ul: u32,
    pub ratio: u64,
    pub l: i32,
    pub ll: i64,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_VALUE_0 {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VFR_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VFR_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_AUTO: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_AUTO: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_LOCK: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_MANUAL: u64 = 2u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Step: i32,
    pub VideoProc: KSCAMERA_EXTENDEDPROP_VALUE,
    pub Reserved: u64,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_AUTO: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_AUTO: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_OFF: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_ON: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON_ADJUSTABLEPOWER: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_DISABLED: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_ENABLED: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSCAMERA_EXTENDEDPROP_WBPRESET = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CLOUDY: KSCAMERA_EXTENDEDPROP_WBPRESET = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_DAYLIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLASH: KSCAMERA_EXTENDEDPROP_WBPRESET = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLUORESCENT: KSCAMERA_EXTENDEDPROP_WBPRESET = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_TUNGSTEN: KSCAMERA_EXTENDEDPROP_WBPRESET = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CANDLELIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_TEMPERATURE: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_PRESET: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DEFAULT: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DIRECT: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_SMOOTH: u64 = 2u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    pub PhotoResWidth: u32,
    pub PhotoResHeight: u32,
    pub PreviewFPSNum: u32,
    pub PreviewFPSDenom: u32,
    pub CaptureFPSNum: u32,
    pub CaptureFPSDenom: u32,
}
impl ::core::marker::Copy for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {}
impl ::core::clone::Clone for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub MaskCoverageBoundingBox: super::super::Foundation::RECT,
    pub MaskResolution: super::super::Foundation::SIZE,
    pub ForegroundBoundingBox: super::super::Foundation::RECT,
    pub MaskData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_METADATA_CAPTURESTATS {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Flags: u32,
    pub Reserved: u32,
    pub ExposureTime: u64,
    pub ExposureCompensationFlags: u64,
    pub ExposureCompensationValue: i32,
    pub IsoSpeed: u32,
    pub FocusState: u32,
    pub LensPosition: u32,
    pub WhiteBalance: u32,
    pub Flash: u32,
    pub FlashPower: u32,
    pub ZoomFactor: u32,
    pub SceneMode: u64,
    pub SensorFramerate: u64,
}
impl ::core::marker::Copy for KSCAMERA_METADATA_CAPTURESTATS {}
impl ::core::clone::Clone for KSCAMERA_METADATA_CAPTURESTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURECOMPENSATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURETIME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASHPOWER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FOCUSSTATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ISOSPEED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_LENSPOSITION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SCENEMODE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SENSORFRAMERATE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_WHITEBALANCE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ZOOMFACTOR: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_METADATA_DIGITALWINDOW {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Window: KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING,
}
impl ::core::marker::Copy for KSCAMERA_METADATA_DIGITALWINDOW {}
impl ::core::clone::Clone for KSCAMERA_METADATA_DIGITALWINDOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_METADATA_FRAMEILLUMINATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_METADATA_FRAMEILLUMINATION {}
impl ::core::clone::Clone for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_METADATA_FRAMEILLUMINATION_FLAG_ON: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_METADATA_ITEMHEADER {
    pub MetadataId: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for KSCAMERA_METADATA_ITEMHEADER {}
impl ::core::clone::Clone for KSCAMERA_METADATA_ITEMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_METADATA_PHOTOCONFIRMATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub PhotoConfirmationIndex: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_METADATA_PHOTOCONFIRMATION {}
impl ::core::clone::Clone for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSCAMERA_MetadataId = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_Standard_Start: KSCAMERA_MetadataId = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_PhotoConfirmation: KSCAMERA_MetadataId = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_UsbVideoHeader: KSCAMERA_MetadataId = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_CaptureStats: KSCAMERA_MetadataId = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_CameraExtrinsics: KSCAMERA_MetadataId = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_CameraIntrinsics: KSCAMERA_MetadataId = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_FrameIllumination: KSCAMERA_MetadataId = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_DigitalWindow: KSCAMERA_MetadataId = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_BackgroundSegmentationMask: KSCAMERA_MetadataId = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_Standard_End: KSCAMERA_MetadataId = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_Custom_Start: KSCAMERA_MetadataId = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_AUTO: u64 = 4294967296u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    pub Size: u32,
    pub ItemCount: u32,
    pub Flags: u64,
}
impl ::core::marker::Copy for KSCAMERA_PERFRAMESETTING_CAP_HEADER {}
impl ::core::clone::Clone for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
impl ::core::marker::Copy for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {}
impl ::core::clone::Clone for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    pub Size: u32,
    pub Reserved: u32,
    pub Id: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {}
impl ::core::clone::Clone for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    pub Size: u32,
    pub Id: u32,
    pub ItemCount: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {}
impl ::core::clone::Clone for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PERFRAMESETTING_HEADER {
    pub Size: u32,
    pub FrameCount: u32,
    pub Id: ::windows_sys::core::GUID,
    pub Flags: u64,
    pub LoopCount: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_PERFRAMESETTING_HEADER {}
impl ::core::clone::Clone for KSCAMERA_PERFRAMESETTING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
impl ::core::marker::Copy for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {}
impl ::core::clone::Clone for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSCAMERA_PERFRAMESETTING_ITEM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_TIME: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_FLASH: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_COMPENSATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_ISO: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_FOCUS: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_PHOTOCONFIRMATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_CUSTOM: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_MANUAL: u64 = 8589934592u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_CONCURRENCYINFO {
    pub ReferenceGuid: ::windows_sys::core::GUID,
    pub Reserved: u32,
    pub ProfileCount: u32,
    pub Profiles: *mut KSCAMERA_PROFILE_INFO,
}
impl ::core::marker::Copy for KSCAMERA_PROFILE_CONCURRENCYINFO {}
impl ::core::clone::Clone for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_INFO {
    pub ProfileId: ::windows_sys::core::GUID,
    pub Index: u32,
    pub PinCount: u32,
    pub Pins: *mut KSCAMERA_PROFILE_PININFO,
}
impl ::core::marker::Copy for KSCAMERA_PROFILE_INFO {}
impl ::core::clone::Clone for KSCAMERA_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_MEDIAINFO {
    pub Resolution: KSCAMERA_PROFILE_MEDIAINFO_1,
    pub MaxFrameRate: KSCAMERA_PROFILE_MEDIAINFO_0,
    pub Flags: u64,
    pub Data0: u32,
    pub Data1: u32,
    pub Data2: u32,
    pub Data3: u32,
}
impl ::core::marker::Copy for KSCAMERA_PROFILE_MEDIAINFO {}
impl ::core::clone::Clone for KSCAMERA_PROFILE_MEDIAINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_MEDIAINFO_0 {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl ::core::marker::Copy for KSCAMERA_PROFILE_MEDIAINFO_0 {}
impl ::core::clone::Clone for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_MEDIAINFO_1 {
    pub X: u32,
    pub Y: u32,
}
impl ::core::marker::Copy for KSCAMERA_PROFILE_MEDIAINFO_1 {}
impl ::core::clone::Clone for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_PININFO {
    pub PinCategory: ::windows_sys::core::GUID,
    pub Anonymous: KSCAMERA_PROFILE_PININFO_0,
    pub MediaInfoCount: u32,
    pub MediaInfos: *mut KSCAMERA_PROFILE_MEDIAINFO,
}
impl ::core::marker::Copy for KSCAMERA_PROFILE_PININFO {}
impl ::core::clone::Clone for KSCAMERA_PROFILE_PININFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSCAMERA_PROFILE_PININFO_0 {
    pub Anonymous: KSCAMERA_PROFILE_PININFO_0_0,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_PROFILE_PININFO_0 {}
impl ::core::clone::Clone for KSCAMERA_PROFILE_PININFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_PININFO_0_0 {
    pub PinIndex: u16,
    pub ProfileSensorType: u16,
}
impl ::core::marker::Copy for KSCAMERA_PROFILE_PININFO_0_0 {}
impl ::core::clone::Clone for KSCAMERA_PROFILE_PININFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3214294400, data2: 50521, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSCATEGORY_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1771351300, data2: 37871, data3: 4560, data4: [163, 204, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_BRIDGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 140181248, data2: 25294, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSCATEGORY_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1709733693, data2: 36694, data3: 4560, data4: [163, 185, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_CLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1394025600, data2: 18321, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSCATEGORY_COMMUNICATIONSTRANSFORM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3474840108, data2: 38723, data3: 4560, data4: [163, 238, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_CROSSBAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2811865089, data2: 42093, data3: 4560, data4: [161, 140, 0, 160, 36, 1, 220, 212] };
pub const KSCATEGORY_DATACOMPRESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 512018688, data2: 32368, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSCATEGORY_DATADECOMPRESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 656518688, data2: 32368, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSCATEGORY_DATATRANSFORM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 783318688, data2: 32368, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSCATEGORY_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 426286070, data2: 50052, data3: 18685, data4: [173, 81, 144, 229, 140, 121, 247, 11] };
pub const KSCATEGORY_ESCALANTE_PLATFORM_DRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1962127016, data2: 38760, data3: 4561, data4: [142, 7, 0, 160, 201, 94, 194, 46] };
pub const KSCATEGORY_FILESYSTEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1980755294, data2: 37719, data3: 4560, data4: [163, 204, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_INTERFACETRANSFORM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3474840109, data2: 38723, data3: 4560, data4: [163, 238, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_MEDIUMTRANSFORM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3474840110, data2: 38723, data3: 4560, data4: [163, 238, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_MICROPHONE_ARRAY_PROCESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2198488306, data2: 41773, data3: 18283, data4: [190, 151, 66, 132, 86, 115, 179, 90] };
pub const KSCATEGORY_MIXER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2910886912, data2: 31624, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSCATEGORY_MULTIPLEXER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2052973011, data2: 417, data3: 17708, data4: [180, 129, 79, 162, 185, 98, 113, 232] };
pub const KSCATEGORY_NETWORK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1741278268, data2: 27076, data3: 4562, data4: [135, 89, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_NETWORK_CAMERA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3089335890, data2: 46336, data3: 16875, data4: [180, 243, 66, 52, 247, 245, 174, 153] };
pub const KSCATEGORY_PROXY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2548804298, data2: 38333, data3: 4560, data4: [163, 234, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_QUALITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2548804299, data2: 38333, data3: 4560, data4: [163, 234, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_REALTIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3943784444, data2: 4296, data3: 18788, data4: [131, 29, 109, 203, 2, 230, 242, 63] };
pub const KSCATEGORY_RENDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1709733694, data2: 36694, data3: 4560, data4: [163, 185, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_SENSOR_CAMERA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 619008727, data2: 25891, data3: 18423, data4: [166, 71, 211, 70, 91, 241, 245, 202] };
pub const KSCATEGORY_SENSOR_GROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1721528852, data2: 2696, data3: 17169, data4: [167, 243, 78, 121, 130, 14, 51, 189] };
pub const KSCATEGORY_SPLITTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 172118688, data2: 32368, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSCATEGORY_TEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1771351302, data2: 37871, data3: 4560, data4: [163, 204, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_TOPOLOGY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3718597184, data2: 7756, data3: 4561, data4: [160, 80, 64, 87, 5, 193, 0, 0] };
pub const KSCATEGORY_TVAUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2811865090, data2: 42093, data3: 4560, data4: [161, 140, 0, 160, 36, 1, 220, 212] };
pub const KSCATEGORY_TVTUNER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2811865088, data2: 42093, data3: 4560, data4: [161, 140, 0, 160, 36, 1, 220, 212] };
pub const KSCATEGORY_VBICODEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 131782240, data2: 8945, data3: 4561, data4: [169, 244, 0, 192, 79, 187, 222, 143] };
pub const KSCATEGORY_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1771351301, data2: 37871, data3: 4560, data4: [163, 204, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_VIDEO_CAMERA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3845273463, data2: 63862, data3: 20315, data4: [155, 85, 185, 70, 153, 196, 110, 68] };
pub const KSCATEGORY_VIRTUAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 889449156, data2: 7974, data3: 4561, data4: [138, 176, 0, 160, 201, 34, 49, 150] };
pub const KSCATEGORY_VPMUX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2811865091, data2: 42093, data3: 4560, data4: [161, 140, 0, 160, 36, 1, 220, 212] };
pub const KSCATEGORY_WDMAUD_USE_PIN_NAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1201994272, data2: 41553, data3: 4561, data4: [160, 80, 0, 0, 248, 0, 71, 136] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCLOCK_CREATE {
    pub CreateFlags: u32,
}
impl ::core::marker::Copy for KSCLOCK_CREATE {}
impl ::core::clone::Clone for KSCLOCK_CREATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCOMPONENTID {
    pub Manufacturer: ::windows_sys::core::GUID,
    pub Product: ::windows_sys::core::GUID,
    pub Component: ::windows_sys::core::GUID,
    pub Name: ::windows_sys::core::GUID,
    pub Version: u32,
    pub Revision: u32,
}
impl ::core::marker::Copy for KSCOMPONENTID {}
impl ::core::clone::Clone for KSCOMPONENTID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSCOMPONENTID_USBAUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2400351728, data2: 9961, data3: 16996, data4: [186, 77, 57, 255, 240, 29, 148, 170] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCORRELATED_TIME {
    pub Time: i64,
    pub SystemTime: i64,
}
impl ::core::marker::Copy for KSCORRELATED_TIME {}
impl ::core::clone::Clone for KSCORRELATED_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCREATE_ITEM_FREEONSTOP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCREATE_ITEM_NOPARAMETERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCREATE_ITEM_SECURITYCHANGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCREATE_ITEM_WILDCARD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCameraProfileSensorType_Custom: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCameraProfileSensorType_Depth: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCameraProfileSensorType_ImageSegmentation: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCameraProfileSensorType_Infrared: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCameraProfileSensorType_PoseTracking: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCameraProfileSensorType_RGB: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSDATAFORMAT {
    pub Anonymous: KSDATAFORMAT_0,
    pub Alignment: i64,
}
impl ::core::marker::Copy for KSDATAFORMAT {}
impl ::core::clone::Clone for KSDATAFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDATAFORMAT_0 {
    pub FormatSize: u32,
    pub Flags: u32,
    pub SampleSize: u32,
    pub Reserved: u32,
    pub MajorFormat: ::windows_sys::core::GUID,
    pub SubFormat: ::windows_sys::core::GUID,
    pub Specifier: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for KSDATAFORMAT_0 {}
impl ::core::clone::Clone for KSDATAFORMAT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDATAFORMAT_BIT_ATTRIBUTES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDATAFORMAT_BIT_TEMPORAL_COMPRESSION: u32 = 0u32;
pub const KSDATAFORMAT_SPECIFIER_AC3_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272804, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SPECIFIER_ANALOGVIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 75685344, data2: 30743, data3: 4559, data4: [138, 3, 0, 170, 0, 110, 203, 101] };
pub const KSDATAFORMAT_SPECIFIER_DIALECT_AC3_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358773, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358770, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358769, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358772, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358771, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SPECIFIER_DSOUND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1367707810, data2: 41348, data3: 4560, data4: [133, 34, 0, 192, 79, 217, 186, 243] };
pub const KSDATAFORMAT_SPECIFIER_FILEHANDLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1709733692, data2: 36694, data3: 4560, data4: [163, 185, 0, 160, 201, 34, 49, 150] };
pub const KSDATAFORMAT_SPECIFIER_FILENAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860088128, data2: 59764, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSDATAFORMAT_SPECIFIER_H264_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 538426885, data2: 26153, data3: 16968, data4: [170, 237, 126, 26, 71, 188, 155, 156] };
pub const KSDATAFORMAT_SPECIFIER_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1764729721, data2: 54248, data3: 18001, data4: [181, 180, 11, 148, 176, 19, 238, 175] };
pub const KSDATAFORMAT_SPECIFIER_JPEG_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1764729721, data2: 54248, data3: 18001, data4: [181, 180, 11, 148, 176, 19, 238, 175] };
pub const KSDATAFORMAT_SPECIFIER_LPCM_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272806, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SPECIFIER_MPEG1_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 89694082, data2: 50006, data3: 4558, data4: [191, 1, 0, 170, 0, 85, 89, 90] };
pub const KSDATAFORMAT_SPECIFIER_MPEG2_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272805, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SPECIFIER_MPEG2_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272803, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SPECIFIER_NONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 258217942, data2: 49944, data3: 4560, data4: [164, 63, 0, 160, 201, 34, 49, 150] };
pub const KSDATAFORMAT_SPECIFIER_VBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146755296, data2: 60170, data3: 4560, data4: [172, 228, 0, 0, 192, 204, 22, 186] };
pub const KSDATAFORMAT_SPECIFIER_VC_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2912473476, data2: 43715, data3: 4560, data4: [164, 28, 0, 160, 201, 34, 49, 150] };
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 89694080, data2: 50006, data3: 4558, data4: [191, 1, 0, 170, 0, 85, 89, 90] };
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146755232, data2: 60170, data3: 4560, data4: [172, 228, 0, 0, 192, 204, 22, 186] };
pub const KSDATAFORMAT_SPECIFIER_WAVEFORMATEX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 89694081, data2: 50006, data3: 4558, data4: [191, 1, 0, 170, 0, 85, 89, 90] };
pub const KSDATAFORMAT_SUBTYPE_AC3_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272620, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SUBTYPE_ANALOG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1840918928, data2: 26557, data3: 4559, data4: [160, 247, 0, 32, 175, 209, 86, 228] };
pub const KSDATAFORMAT_SUBTYPE_CC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 857820353, data2: 287, data3: 4562, data4: [180, 177, 0, 160, 209, 2, 207, 190] };
pub const KSDATAFORMAT_SUBTYPE_D16: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 80, data2: 4, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_DSS_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2695843714, data2: 57699, data3: 4560, data4: [186, 217, 0, 96, 151, 68, 17, 26] };
pub const KSDATAFORMAT_SUBTYPE_DSS_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2695843713, data2: 57699, data3: 4560, data4: [186, 217, 0, 96, 151, 68, 17, 26] };
pub const KSDATAFORMAT_SUBTYPE_DTS_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272627, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 6, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ATRAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 8, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 146, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 10, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS_ATMOS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 266, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT20: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 268, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 780, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MLP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 12, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 13, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 8, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 267, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 779, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS_HD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 11, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 5, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ONE_BIT_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 9, data2: 3306, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IEC61937_WMA_PRO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 356, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_IMAGE_RGB32: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 22, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_JPEG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 434415018, data2: 22114, data3: 20421, data4: [160, 192, 23, 88, 2, 142, 16, 87] };
pub const KSDATAFORMAT_SUBTYPE_L16: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 81, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_L16_CUSTOM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 81, data2: 32768, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_L16_IR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 81, data2: 2, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_L8: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 50, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_L8_CUSTOM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 50, data2: 32768, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_L8_IR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 50, data2: 2, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_LPCM_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272626, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SUBTYPE_Line21_BytePair: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1854753314, data2: 12556, data3: 4560, data4: [183, 154, 0, 170, 0, 55, 103, 167] };
pub const KSDATAFORMAT_SUBTYPE_Line21_GOPPacket: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1854753315, data2: 12556, data3: 4560, data4: [183, 154, 0, 170, 0, 55, 103, 167] };
pub const KSDATAFORMAT_SUBTYPE_MIDI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 489039712, data2: 59735, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSDATAFORMAT_SUBTYPE_MIDI_BUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 748773280, data2: 27902, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSDATAFORMAT_SUBTYPE_MJPG_CUSTOM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1196444237, data2: 32768, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_MJPG_DEPTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1196444237, data2: 4, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_MJPG_IR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1196444237, data2: 2, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_MPEG1Packet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804480, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const KSDATAFORMAT_SUBTYPE_MPEG1Payload: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804481, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const KSDATAFORMAT_SUBTYPE_MPEG1Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804486, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const KSDATAFORMAT_SUBTYPE_MPEG2_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272619, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SUBTYPE_MPEG2_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272614, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SUBTYPE_MPEGLAYER3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 85, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_MPEG_HEAAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 5648, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_NABTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146755298, data2: 60170, data3: 4560, data4: [172, 228, 0, 0, 192, 204, 22, 186] };
pub const KSDATAFORMAT_SUBTYPE_NABTS_FEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3881286817, data2: 14764, data3: 4561, data4: [169, 245, 0, 192, 79, 187, 222, 143] };
pub const KSDATAFORMAT_SUBTYPE_NONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804494, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const KSDATAFORMAT_SUBTYPE_OVERLAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804479, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const KSDATAFORMAT_SUBTYPE_PCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_RAW8: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3391150496, data2: 15934, data3: 4561, data4: [155, 249, 0, 192, 79, 187, 222, 191] };
pub const KSDATAFORMAT_SUBTYPE_RIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1234557678, data2: 40678, data3: 4560, data4: [164, 14, 0, 160, 201, 34, 49, 150] };
pub const KSDATAFORMAT_SUBTYPE_RIFFMIDI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1234557680, data2: 40678, data3: 4560, data4: [164, 14, 0, 160, 201, 34, 49, 150] };
pub const KSDATAFORMAT_SUBTYPE_RIFFWAVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804491, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const KSDATAFORMAT_SUBTYPE_SDDS_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272628, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SUBTYPE_STANDARD_AC3_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358757, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358754, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358753, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358756, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358755, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_SUBTYPE_SUBPICTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272621, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_SUBTYPE_TELETEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146755299, data2: 60170, data3: 4560, data4: [172, 228, 0, 0, 192, 204, 22, 186] };
pub const KSDATAFORMAT_SUBTYPE_VPVBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1520134721, data2: 6690, data3: 4561, data4: [186, 217, 0, 96, 151, 68, 17, 26] };
pub const KSDATAFORMAT_SUBTYPE_VPVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1520134720, data2: 6690, data3: 4561, data4: [186, 217, 0, 96, 151, 68, 17, 26] };
pub const KSDATAFORMAT_SUBTYPE_WAVEFORMATEX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 354, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO_LOSSLESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_TYPE_ANALOGAUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 75685601, data2: 30743, data3: 4559, data4: [138, 3, 0, 170, 0, 110, 203, 101] };
pub const KSDATAFORMAT_TYPE_ANALOGVIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 75685345, data2: 30743, data3: 4559, data4: [138, 3, 0, 170, 0, 110, 203, 101] };
pub const KSDATAFORMAT_TYPE_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935963489, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_TYPE_AUXLine21Data: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1728768640, data2: 14978, data3: 4560, data4: [183, 155, 0, 170, 0, 55, 103, 167] };
pub const KSDATAFORMAT_TYPE_DVD_ENCRYPTED_PACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3976958314, data2: 1101, data3: 4561, data4: [170, 120, 0, 192, 79, 195, 29, 96] };
pub const KSDATAFORMAT_TYPE_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1914145827, data2: 58459, data3: 4565, data4: [188, 42, 0, 176, 208, 243, 244, 171] };
pub const KSDATAFORMAT_TYPE_MIDI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935960429, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_TYPE_MPEG2_PES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272608, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_TYPE_MPEG2_PROGRAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272610, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_TYPE_MPEG2_TRANSPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272611, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const KSDATAFORMAT_TYPE_MUSIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3878015840, data2: 25292, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSDATAFORMAT_TYPE_NABTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3881286816, data2: 14764, data3: 4561, data4: [169, 245, 0, 192, 79, 187, 222, 143] };
pub const KSDATAFORMAT_TYPE_STANDARD_ELEMENTARY_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358737, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_TYPE_STANDARD_PACK_HEADER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358739, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_TYPE_STANDARD_PES_PACKET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911358738, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
pub const KSDATAFORMAT_TYPE_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804483, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const KSDATAFORMAT_TYPE_TEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1937012852, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const KSDATAFORMAT_TYPE_VBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146755297, data2: 60170, data3: 4560, data4: [172, 228, 0, 0, 192, 204, 22, 186] };
pub const KSDATAFORMAT_TYPE_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935960438, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDATARANGE_AUDIO {
    pub DataRange: KSDATAFORMAT,
    pub MaximumChannels: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
}
impl ::core::marker::Copy for KSDATARANGE_AUDIO {}
impl ::core::clone::Clone for KSDATARANGE_AUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDATARANGE_BIT_ATTRIBUTES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDATARANGE_BIT_REQUIRED_ATTRIBUTES: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDATARANGE_MUSIC {
    pub DataRange: KSDATAFORMAT,
    pub Technology: ::windows_sys::core::GUID,
    pub Channels: u32,
    pub Notes: u32,
    pub ChannelMask: u32,
}
impl ::core::marker::Copy for KSDATARANGE_MUSIC {}
impl ::core::clone::Clone for KSDATARANGE_MUSIC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSDEGRADESETID_Standard: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2673230208, data2: 28748, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSDEGRADE_STANDARD = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEGRADE_STANDARD_SAMPLE: KSDEGRADE_STANDARD = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEGRADE_STANDARD_QUALITY: KSDEGRADE_STANDARD = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEGRADE_STANDARD_COMPUTATION: KSDEGRADE_STANDARD = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEGRADE_STANDARD_SKIP: KSDEGRADE_STANDARD = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_DESCRIPTOR_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_DESCRIPTOR_VERSION_2: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_FLAG_ENABLE_QUERYINTERFACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_FLAG_ENABLE_REMOTE_WAKEUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_FLAG_LOWPOWER_PASSTHROUGH: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDEVICE_PROFILE_INFO {
    pub Type: u32,
    pub Size: u32,
    pub Anonymous: KSDEVICE_PROFILE_INFO_0,
}
impl ::core::marker::Copy for KSDEVICE_PROFILE_INFO {}
impl ::core::clone::Clone for KSDEVICE_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSDEVICE_PROFILE_INFO_0 {
    pub Camera: KSDEVICE_PROFILE_INFO_0_0,
}
impl ::core::marker::Copy for KSDEVICE_PROFILE_INFO_0 {}
impl ::core::clone::Clone for KSDEVICE_PROFILE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDEVICE_PROFILE_INFO_0_0 {
    pub Info: KSCAMERA_PROFILE_INFO,
    pub Reserved: u32,
    pub ConcurrencyCount: u32,
    pub Concurrency: *mut KSCAMERA_PROFILE_CONCURRENCYINFO,
}
impl ::core::marker::Copy for KSDEVICE_PROFILE_INFO_0_0 {}
impl ::core::clone::Clone for KSDEVICE_PROFILE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_PROFILE_TYPE_CAMERA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_PROFILE_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSDEVICE_THERMAL_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_THERMAL_STATE_LOW: KSDEVICE_THERMAL_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_THERMAL_STATE_HIGH: KSDEVICE_THERMAL_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDISPATCH_FASTIO: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDISPLAYCHANGE {
    pub PelsWidth: u32,
    pub PelsHeight: u32,
    pub BitsPerPel: u32,
    pub DeviceID: [u16; 1],
}
impl ::core::marker::Copy for KSDISPLAYCHANGE {}
impl ::core::clone::Clone for KSDISPLAYCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDS3D_BUFFER_ALL {
    pub Position: DS3DVECTOR,
    pub Velocity: DS3DVECTOR,
    pub InsideConeAngle: u32,
    pub OutsideConeAngle: u32,
    pub ConeOrientation: DS3DVECTOR,
    pub ConeOutsideVolume: i32,
    pub MinDistance: f32,
    pub MaxDistance: f32,
    pub Mode: u32,
}
impl ::core::marker::Copy for KSDS3D_BUFFER_ALL {}
impl ::core::clone::Clone for KSDS3D_BUFFER_ALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDS3D_BUFFER_CONE_ANGLES {
    pub InsideConeAngle: u32,
    pub OutsideConeAngle: u32,
}
impl ::core::marker::Copy for KSDS3D_BUFFER_CONE_ANGLES {}
impl ::core::clone::Clone for KSDS3D_BUFFER_CONE_ANGLES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSDS3D_HRTF_COEFF_FORMAT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FLOAT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SHORT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDS3D_COEFF_COUNT: KSDS3D_HRTF_COEFF_FORMAT = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDS3D_HRTF_FILTER_FORMAT_MSG {
    pub FilterMethod: KSDS3D_HRTF_FILTER_METHOD,
    pub CoeffFormat: KSDS3D_HRTF_COEFF_FORMAT,
    pub Version: KSDS3D_HRTF_FILTER_VERSION,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSDS3D_HRTF_FILTER_FORMAT_MSG {}
impl ::core::clone::Clone for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSDS3D_HRTF_FILTER_METHOD = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const DIRECT_FORM: KSDS3D_HRTF_FILTER_METHOD = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const CASCADE_FORM: KSDS3D_HRTF_FILTER_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDS3D_FILTER_METHOD_COUNT: KSDS3D_HRTF_FILTER_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSDS3D_HRTF_FILTER_QUALITY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FULL_FILTER: KSDS3D_HRTF_FILTER_QUALITY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const LIGHT_FILTER: KSDS3D_HRTF_FILTER_QUALITY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDS3D_FILTER_QUALITY_COUNT: KSDS3D_HRTF_FILTER_QUALITY = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSDS3D_HRTF_FILTER_VERSION = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const DS3D_HRTF_VERSION_1: KSDS3D_HRTF_FILTER_VERSION = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDS3D_HRTF_INIT_MSG {
    pub Size: u32,
    pub Quality: KSDS3D_HRTF_FILTER_QUALITY,
    pub SampleRate: f32,
    pub MaxFilterSize: u32,
    pub FilterTransientMuteLength: u32,
    pub FilterOverlapBufferLength: u32,
    pub OutputOverlapBufferLength: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSDS3D_HRTF_INIT_MSG {}
impl ::core::clone::Clone for KSDS3D_HRTF_INIT_MSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSDS3D_HRTF_PARAMS_MSG {
    pub Size: u32,
    pub Enabled: u32,
    pub SwapChannels: super::super::Foundation::BOOL,
    pub ZeroAzimuth: super::super::Foundation::BOOL,
    pub CrossFadeOutput: super::super::Foundation::BOOL,
    pub FilterSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSDS3D_HRTF_PARAMS_MSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSDS3D_HRTF_PARAMS_MSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDS3D_ITD_PARAMS {
    pub Channel: i32,
    pub VolSmoothScale: f32,
    pub TotalDryAttenuation: f32,
    pub TotalWetAttenuation: f32,
    pub SmoothFrequency: i32,
    pub Delay: i32,
}
impl ::core::marker::Copy for KSDS3D_ITD_PARAMS {}
impl ::core::clone::Clone for KSDS3D_ITD_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDS3D_ITD_PARAMS_MSG {
    pub Enabled: u32,
    pub LeftParams: KSDS3D_ITD_PARAMS,
    pub RightParams: KSDS3D_ITD_PARAMS,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSDS3D_ITD_PARAMS_MSG {}
impl ::core::clone::Clone for KSDS3D_ITD_PARAMS_MSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDS3D_LISTENER_ALL {
    pub Position: DS3DVECTOR,
    pub Velocity: DS3DVECTOR,
    pub OrientFront: DS3DVECTOR,
    pub OrientTop: DS3DVECTOR,
    pub DistanceFactor: f32,
    pub RolloffFactor: f32,
    pub DopplerFactor: f32,
}
impl ::core::marker::Copy for KSDS3D_LISTENER_ALL {}
impl ::core::clone::Clone for KSDS3D_LISTENER_ALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDS3D_LISTENER_ORIENTATION {
    pub Front: DS3DVECTOR,
    pub Top: DS3DVECTOR,
}
impl ::core::marker::Copy for KSDS3D_LISTENER_ORIENTATION {}
impl ::core::clone::Clone for KSDS3D_LISTENER_ORIENTATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_3D_MODE_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_3D_MODE_HEADRELATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_3D_MODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_CTRL_3D: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_CTRL_FREQUENCY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_CTRL_HRTF_3D: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_CTRL_PAN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_CTRL_POSITIONNOTIFY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_CTRL_VOLUME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_LOCHARDWARE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_LOCSOFTWARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDSOUND_BUFFER_STATIC: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSERROR {
    pub Context: *mut ::core::ffi::c_void,
    pub Status: u32,
}
impl ::core::marker::Copy for KSERROR {}
impl ::core::clone::Clone for KSERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENTDATA {
    pub NotificationType: u32,
    pub Anonymous: KSEVENTDATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSEVENTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSEVENTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KSEVENTDATA_0 {
    pub EventHandle: KSEVENTDATA_0_1,
    pub SemaphoreHandle: KSEVENTDATA_0_2,
    pub Alignment: KSEVENTDATA_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSEVENTDATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSEVENTDATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENTDATA_0_0 {
    pub Unused: *mut ::core::ffi::c_void,
    pub Alignment: [isize; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSEVENTDATA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSEVENTDATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENTDATA_0_1 {
    pub Event: super::super::Foundation::HANDLE,
    pub Reserved: [usize; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSEVENTDATA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSEVENTDATA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENTDATA_0_2 {
    pub Semaphore: super::super::Foundation::HANDLE,
    pub Reserved: u32,
    pub Adjustment: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSEVENTDATA_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSEVENTDATA_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENTF_DPC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENTF_EVENT_HANDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENTF_EVENT_OBJECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENTF_KSWORKITEM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENTF_SEMAPHORE_HANDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENTF_SEMAPHORE_OBJECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENTF_WORKITEM: u32 = 32u32;
pub const KSEVENTSETID_AudioControlChange: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3898513048, data2: 64047, data3: 4561, data4: [149, 189, 0, 192, 79, 185, 37, 211] };
pub const KSEVENTSETID_CameraAsyncControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 580982612, data2: 38657, data3: 16520, data4: [179, 63, 107, 156, 188, 82, 223, 94] };
pub const KSEVENTSETID_CameraEvent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2023338720, data2: 27459, data3: 18788, data4: [157, 42, 162, 31, 64, 97, 245, 118] };
pub const KSEVENTSETID_Clock: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911052320, data2: 25287, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSEVENTSETID_Connection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2135673824, data2: 40613, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSEVENTSETID_Device: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 679646956, data2: 40852, data3: 16820, data4: [161, 83, 170, 49, 174, 236, 179, 63] };
pub const KSEVENTSETID_DynamicFormatChange: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 371901526, data2: 33751, data3: 16953, data4: [150, 223, 199, 95, 250, 19, 139, 198] };
pub const KSEVENTSETID_EXTDEV_Command: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 278690184, data2: 46027, data3: 4562, data4: [180, 142, 0, 96, 151, 179, 57, 27] };
pub const KSEVENTSETID_ExtendedCameraControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1461490377, data2: 5026, data3: 18403, data4: [166, 73, 210, 167, 120, 22, 99, 132] };
pub const KSEVENTSETID_LoopedStreaming: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1182972224, data2: 50927, data3: 4560, data4: [150, 216, 0, 170, 0, 81, 229, 29] };
pub const KSEVENTSETID_PinCapsChange: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3712948526, data2: 15224, data3: 18861, data4: [165, 52, 44, 49, 91, 130, 32, 0] };
pub const KSEVENTSETID_SoundDetector: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1769495707, data2: 64557, data3: 18902, data4: [172, 50, 71, 153, 248, 125, 233, 246] };
pub const KSEVENTSETID_StreamAllocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1977177457, data2: 1852, data3: 4560, data4: [161, 97, 0, 32, 175, 209, 86, 228] };
pub const KSEVENTSETID_Telephony: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3078558388, data2: 52916, data3: 17540, data4: [141, 94, 82, 193, 231, 216, 118, 45] };
pub const KSEVENTSETID_VIDCAPTOSTI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3678920224, data2: 63016, data3: 4561, data4: [186, 65, 0, 160, 201, 13, 43, 5] };
pub const KSEVENTSETID_VIDCAP_TVAUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401169, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const KSEVENTSETID_VPNotify: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 549804430, data2: 54216, data3: 4560, data4: [141, 252, 0, 192, 79, 215, 192, 139] };
pub const KSEVENTSETID_VPVBINotify: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3964836609, data2: 6687, data3: 4561, data4: [186, 217, 0, 96, 151, 68, 17, 26] };
pub const KSEVENTSETID_VolumeLimit: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3658908773, data2: 14972, data3: 18520, data4: [157, 74, 62, 142, 36, 112, 26, 239] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_AUDIO_CONTROL_CHANGE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONTROL_CHANGE: KSEVENT_AUDIO_CONTROL_CHANGE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_CAMERACONTROL = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CAMERACONTROL_FOCUS: KSEVENT_CAMERACONTROL = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CAMERACONTROL_ZOOM: KSEVENT_CAMERACONTROL = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_CAMERAEVENT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_PHOTO_SAMPLE_SCANNED: KSEVENT_CAMERAEVENT = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_CLOCK_POSITION = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CLOCK_INTERVAL_MARK: KSEVENT_CLOCK_POSITION = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CLOCK_POSITION_MARK: KSEVENT_CLOCK_POSITION = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_CONNECTION = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_POSITIONUPDATE: KSEVENT_CONNECTION = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_DATADISCONTINUITY: KSEVENT_CONNECTION = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_TIMEDISCONTINUITY: KSEVENT_CONNECTION = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_PRIORITY: KSEVENT_CONNECTION = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_ENDOFSTREAM: KSEVENT_CONNECTION = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_CROSSBAR = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CROSSBAR_CHANGED: KSEVENT_CROSSBAR = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_DEVCMD = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_COMMAND_NOTIFY_INTERIM_READY: KSEVENT_DEVCMD = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_COMMAND_CONTROL_INTERIM_READY: KSEVENT_DEVCMD = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_COMMAND_BUSRESET: KSEVENT_DEVCMD = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_TIMECODE_UPDATE: KSEVENT_DEVCMD = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_OPERATION_MODE_UPDATE: KSEVENT_DEVCMD = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_TRANSPORT_STATE_UPDATE: KSEVENT_DEVCMD = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_NOTIFY_REMOVAL: KSEVENT_DEVCMD = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_NOTIFY_MEDIUM_CHANGE: KSEVENT_DEVCMD = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_DEVICE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DEVICE_LOST: KSEVENT_DEVICE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DEVICE_PREEMPTED: KSEVENT_DEVICE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DEVICE_THERMAL_HIGH: KSEVENT_DEVICE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DEVICE_THERMAL_LOW: KSEVENT_DEVICE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_DYNAMICFORMATCHANGE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DYNAMIC_FORMAT_CHANGE: KSEVENT_DYNAMICFORMATCHANGE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_ENTRY_BUFFERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_ENTRY_DELETED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_ENTRY_ONESHOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_LOOPEDSTREAMING = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_LOOPEDSTREAMING_POSITION: KSEVENT_LOOPEDSTREAMING = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_PINCAPS_CHANGENOTIFICATIONS = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_PINCAPS_FORMATCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_PINCAPS_JACKINFOCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_SOUNDDETECTOR = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_SOUNDDETECTOR_MATCHDETECTED: KSEVENT_SOUNDDETECTOR = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_STREAMALLOCATOR = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_STREAMALLOCATOR_INTERNAL_FREEFRAME: KSEVENT_STREAMALLOCATOR = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_STREAMALLOCATOR_FREEFRAME: KSEVENT_STREAMALLOCATOR = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_TELEPHONY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TELEPHONY_ENDPOINTPAIRS_CHANGED: KSEVENT_TELEPHONY = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENT_TIME_INTERVAL {
    pub EventData: KSEVENTDATA,
    pub TimeBase: i64,
    pub Interval: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSEVENT_TIME_INTERVAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSEVENT_TIME_INTERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENT_TIME_MARK {
    pub EventData: KSEVENTDATA,
    pub MarkTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSEVENT_TIME_MARK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSEVENT_TIME_MARK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_TUNER = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TUNER_CHANGED: KSEVENT_TUNER = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TUNER_INITIATE_SCAN: KSEVENT_TUNER = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENT_TUNER_INITIATE_SCAN_S {
    pub EventData: KSEVENTDATA,
    pub StartFrequency: u32,
    pub EndFrequency: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSEVENT_TUNER_INITIATE_SCAN_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSEVENT_TUNER_INITIATE_SCAN_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_TVAUDIO = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TVAUDIO_CHANGED: KSEVENT_TVAUDIO = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TYPE_BASICSUPPORT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TYPE_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TYPE_ENABLEBUFFERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TYPE_ONESHOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TYPE_QUERYBUFFER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TYPE_SETSUPPORT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TYPE_TOPOLOGY: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_VIDCAPTOSTI = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VIDCAPTOSTI_EXT_TRIGGER: KSEVENT_VIDCAPTOSTI = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VIDCAP_AUTO_UPDATE: KSEVENT_VIDCAPTOSTI = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VIDCAP_SEARCH: KSEVENT_VIDCAPTOSTI = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_VIDEODECODER = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VIDEODECODER_CHANGED: KSEVENT_VIDEODECODER = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_VOLUMELIMIT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VOLUMELIMIT_CHANGED: KSEVENT_VOLUMELIMIT = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_VPNOTIFY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VPNOTIFY_FORMATCHANGE: KSEVENT_VPNOTIFY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSEVENT_VPVBINOTIFY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VPVBINOTIFY_FORMATCHANGE: KSEVENT_VPVBINOTIFY = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSE_NODE {
    pub Event: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSE_NODE {}
impl ::core::clone::Clone for KSE_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSE_PIN {
    pub Event: KSIDENTIFIER,
    pub PinId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSE_PIN {}
impl ::core::clone::Clone for KSE_PIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSFILTER_FLAG_CRITICAL_PROCESSING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSFILTER_FLAG_DENY_USERMODE_ACCESS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSFILTER_FLAG_DISPATCH_LEVEL_PROCESSING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSFILTER_FLAG_HYPERCRITICAL_PROCESSING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSFILTER_FLAG_PRIORITIZE_REFERENCEGUID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSFILTER_FLAG_RECEIVE_ZERO_LENGTH_SAMPLES: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSFRAMETIME {
    pub Duration: i64,
    pub FrameFlags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSFRAMETIME {}
impl ::core::clone::Clone for KSFRAMETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSFRAMETIME_VARIABLESIZE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSGOP_USERDATA {
    pub sc: u32,
    pub reserved1: u32,
    pub cFields: u8,
    pub l21Data: [super::super::Foundation::CHAR; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSGOP_USERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSGOP_USERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSIDENTIFIER {
    pub Anonymous: KSIDENTIFIER_0,
}
impl ::core::marker::Copy for KSIDENTIFIER {}
impl ::core::clone::Clone for KSIDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSIDENTIFIER_0 {
    pub Anonymous: KSIDENTIFIER_0_0,
    pub Alignment: i64,
}
impl ::core::marker::Copy for KSIDENTIFIER_0 {}
impl ::core::clone::Clone for KSIDENTIFIER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSIDENTIFIER_0_0 {
    pub Set: ::windows_sys::core::GUID,
    pub Id: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for KSIDENTIFIER_0_0 {}
impl ::core::clone::Clone for KSIDENTIFIER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSINTERFACESETID_FileIo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2356122412, data2: 59249, data3: 4560, data4: [184, 255, 0, 160, 201, 34, 49, 150] };
pub const KSINTERFACESETID_Media: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 974383936, data2: 12455, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSINTERFACESETID_Standard: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 445081248, data2: 25294, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSINTERFACE_FILEIO = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_FILEIO_STREAMING: KSINTERFACE_FILEIO = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSINTERFACE_MEDIA = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_MEDIA_MUSIC: KSINTERFACE_MEDIA = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_MEDIA_WAVE_BUFFERED: KSINTERFACE_MEDIA = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_MEDIA_WAVE_QUEUED: KSINTERFACE_MEDIA = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSINTERFACE_STANDARD = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_STANDARD_STREAMING: KSINTERFACE_STANDARD = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_STANDARD_LOOPED_STREAMING: KSINTERFACE_STANDARD = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_STANDARD_CONTROL: KSINTERFACE_STANDARD = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSINTERVAL {
    pub TimeBase: i64,
    pub Interval: i64,
}
impl ::core::marker::Copy for KSINTERVAL {}
impl ::core::clone::Clone for KSINTERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSJACK_DESCRIPTION {
    pub ChannelMapping: u32,
    pub Color: u32,
    pub ConnectionType: EPcxConnectionType,
    pub GeoLocation: EPcxGeoLocation,
    pub GenLocation: EPcxGenLocation,
    pub PortConnection: EPxcPortConnection,
    pub IsConnected: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSJACK_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSJACK_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSJACK_DESCRIPTION2 {
    pub DeviceStateInfo: u32,
    pub JackCapabilities: u32,
}
impl ::core::marker::Copy for KSJACK_DESCRIPTION2 {}
impl ::core::clone::Clone for KSJACK_DESCRIPTION2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSJACK_SINK_CONNECTIONTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSJACK_SINK_CONNECTIONTYPE_HDMI: KSJACK_SINK_CONNECTIONTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSJACK_SINK_CONNECTIONTYPE_DISPLAYPORT: KSJACK_SINK_CONNECTIONTYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSJACK_SINK_INFORMATION {
    pub ConnType: KSJACK_SINK_CONNECTIONTYPE,
    pub ManufacturerId: u16,
    pub ProductId: u16,
    pub AudioLatency: u16,
    pub HDCPCapable: super::super::Foundation::BOOL,
    pub AICapable: super::super::Foundation::BOOL,
    pub SinkDescriptionLength: u8,
    pub SinkDescription: [u16; 32],
    pub PortId: super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSJACK_SINK_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSJACK_SINK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSMEDIUMSETID_MidiBus: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 93356096, data2: 12870, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSMEDIUMSETID_Standard: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1195881248, data2: 25294, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSMEDIUMSETID_VPBus: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2710312428, data2: 52803, data3: 4560, data4: [171, 231, 0, 160, 201, 34, 49, 150] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMEDIUM_STANDARD_DEVIO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMEDIUM_TYPE_ANYINSTANCE: u32 = 0u32;
pub const KSMEMORY_TYPE_DEVICE_UNKNOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 152811065, data2: 24639, data3: 4561, data4: [176, 103, 0, 160, 201, 6, 40, 2] };
pub const KSMEMORY_TYPE_KERNEL_NONPAGED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1248681924, data2: 30869, data3: 4561, data4: [176, 105, 0, 160, 201, 6, 40, 2] };
pub const KSMEMORY_TYPE_KERNEL_PAGED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3627284728, data2: 30868, data3: 4561, data4: [176, 105, 0, 160, 201, 6, 40, 2] };
pub const KSMEMORY_TYPE_SYSTEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 152811064, data2: 24639, data3: 4561, data4: [176, 103, 0, 160, 201, 6, 40, 2] };
pub const KSMEMORY_TYPE_USER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2360409128, data2: 30867, data3: 4561, data4: [176, 105, 0, 160, 201, 6, 40, 2] };
pub const KSMETHODSETID_StreamAllocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3480109889, data2: 60551, data3: 4559, data4: [161, 48, 0, 32, 175, 209, 86, 228] };
pub const KSMETHODSETID_StreamIo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1708131274, data2: 5411, data3: 4562, data4: [178, 122, 0, 160, 201, 34, 49, 150] };
pub const KSMETHODSETID_Wavetable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3706663403, data2: 55559, data3: 4560, data4: [149, 131, 0, 192, 79, 185, 37, 211] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSMETHOD_STREAMALLOCATOR = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_STREAMALLOCATOR_ALLOC: KSMETHOD_STREAMALLOCATOR = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_STREAMALLOCATOR_FREE: KSMETHOD_STREAMALLOCATOR = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSMETHOD_STREAMIO = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_STREAMIO_READ: KSMETHOD_STREAMIO = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_STREAMIO_WRITE: KSMETHOD_STREAMIO = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_BASICSUPPORT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_MODIFY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_SEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_SETSUPPORT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_SOURCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_TOPOLOGY: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_TYPE_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSMETHOD_WAVETABLE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVETABLE_WAVE_ALLOC: KSMETHOD_WAVETABLE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVETABLE_WAVE_FREE: KSMETHOD_WAVETABLE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVETABLE_WAVE_FIND: KSMETHOD_WAVETABLE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVETABLE_WAVE_WRITE: KSMETHOD_WAVETABLE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVE_QUEUED_BREAKLOOP: u32 = 1u32;
pub const KSMFT_CATEGORY_AUDIO_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2661760948, data2: 61306, data3: 17753, data4: [141, 93, 113, 157, 143, 4, 38, 199] };
pub const KSMFT_CATEGORY_AUDIO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 285625416, data2: 13896, data3: 20176, data4: [147, 46, 5, 206, 138, 200, 17, 183] };
pub const KSMFT_CATEGORY_AUDIO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2445691856, data2: 63774, data3: 19852, data4: [146, 118, 219, 36, 130, 121, 217, 117] };
pub const KSMFT_CATEGORY_DEMULTIPLEXER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2825915002, data2: 37787, data3: 17605, data4: [153, 215, 118, 34, 107, 35, 179, 241] };
pub const KSMFT_CATEGORY_MULTIPLEXER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 94131742, data2: 1454, data3: 19297, data4: [182, 157, 85, 182, 30, 229, 74, 123] };
pub const KSMFT_CATEGORY_OTHER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2417450327, data2: 47082, data3: 18689, data4: [174, 179, 147, 58, 135, 71, 117, 111] };
pub const KSMFT_CATEGORY_VIDEO_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3602918731, data2: 26675, data3: 17844, data4: [151, 26, 5, 164, 176, 75, 171, 145] };
pub const KSMFT_CATEGORY_VIDEO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 316767265, data2: 21292, data3: 19054, data4: [138, 28, 64, 130, 90, 115, 99, 151] };
pub const KSMFT_CATEGORY_VIDEO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4154371197, data2: 58693, data3: 17287, data4: [189, 238, 214, 71, 215, 189, 228, 42] };
pub const KSMFT_CATEGORY_VIDEO_PROCESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 808363004, data2: 43615, data3: 18425, data4: [159, 122, 194, 24, 139, 177, 99, 2] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSMICARRAY_MICARRAYTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICARRAYTYPE_LINEAR: KSMICARRAY_MICARRAYTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICARRAYTYPE_PLANAR: KSMICARRAY_MICARRAYTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICARRAYTYPE_3D: KSMICARRAY_MICARRAYTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSMICARRAY_MICTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_OMNIDIRECTIONAL: KSMICARRAY_MICTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_SUBCARDIOID: KSMICARRAY_MICTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_CARDIOID: KSMICARRAY_MICTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_SUPERCARDIOID: KSMICARRAY_MICTYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_HYPERCARDIOID: KSMICARRAY_MICTYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_8SHAPED: KSMICARRAY_MICTYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_VENDORDEFINED: KSMICARRAY_MICTYPE = 15i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMPEGVIDMODE_LTRBOX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMPEGVIDMODE_PANSCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMPEGVIDMODE_SCALE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSMPEGVID_RECT {
    pub StartX: u32,
    pub StartY: u32,
    pub EndX: u32,
    pub EndY: u32,
}
impl ::core::marker::Copy for KSMPEGVID_RECT {}
impl ::core::clone::Clone for KSMPEGVID_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSMULTIPLE_DATA_PROP {
    pub Property: KSIDENTIFIER,
    pub MultipleItem: KSMULTIPLE_ITEM,
}
impl ::core::marker::Copy for KSMULTIPLE_DATA_PROP {}
impl ::core::clone::Clone for KSMULTIPLE_DATA_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSMULTIPLE_ITEM {
    pub Size: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for KSMULTIPLE_ITEM {}
impl ::core::clone::Clone for KSMULTIPLE_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSMUSICFORMAT {
    pub TimeDeltaMs: u32,
    pub ByteCount: u32,
}
impl ::core::marker::Copy for KSMUSICFORMAT {}
impl ::core::clone::Clone for KSMUSICFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSMUSIC_TECHNOLOGY_FMSYNTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 623664256, data2: 25321, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSMUSIC_TECHNOLOGY_PORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2261331552, data2: 25320, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSMUSIC_TECHNOLOGY_SQSYNTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 248464256, data2: 25321, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSMUSIC_TECHNOLOGY_SWSYNTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 926971702, data2: 13856, data3: 4561, data4: [133, 211, 0, 0, 248, 117, 67, 128] };
pub const KSMUSIC_TECHNOLOGY_WAVETABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 961464256, data2: 25321, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSM_NODE {
    pub Method: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSM_NODE {}
impl ::core::clone::Clone for KSM_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSNAME_Allocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1680825600, data2: 18321, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSNAME_Clock: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1394025600, data2: 18321, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSNAME_Filter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2604030096, data2: 5727, data3: 4560, data4: [161, 149, 0, 32, 175, 209, 86, 228] };
pub const KSNAME_Pin: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 342825600, data2: 18321, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSNAME_TopologyNode: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 102827546, data2: 61045, data3: 4560, data4: [185, 21, 0, 160, 201, 34, 49, 150] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_AEC_CAPTURE_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_AEC_CAPTURE_OUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_AEC_RENDER_IN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_AEC_RENDER_OUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_DEMUX_IN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_DEMUX_OUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_STANDARD_IN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_STANDARD_OUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_SUM_MUX_IN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSNODEPIN_SUM_MUX_OUT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSNODEPROPERTY {
    pub Property: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSNODEPROPERTY {}
impl ::core::clone::Clone for KSNODEPROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER {
    pub NodeProperty: KSNODEPROPERTY,
    pub ListenerId: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for KSNODEPROPERTY_AUDIO_3D_LISTENER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[cfg(target_arch = "x86")]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER {
    pub NodeProperty: KSNODEPROPERTY,
    pub ListenerId: *mut ::core::ffi::c_void,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for KSNODEPROPERTY_AUDIO_3D_LISTENER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSNODEPROPERTY_AUDIO_CHANNEL {
    pub NodeProperty: KSNODEPROPERTY,
    pub Channel: i32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSNODEPROPERTY_AUDIO_CHANNEL {}
impl ::core::clone::Clone for KSNODEPROPERTY_AUDIO_CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    pub NodeProperty: KSNODEPROPERTY,
    pub DevSpecificId: u32,
    pub DeviceInfo: u32,
    pub Length: u32,
}
impl ::core::marker::Copy for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {}
impl ::core::clone::Clone for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY {
    pub NodeProperty: KSNODEPROPERTY,
    pub AppContext: *mut ::core::ffi::c_void,
    pub Length: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for KSNODEPROPERTY_AUDIO_PROPERTY {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[cfg(target_arch = "x86")]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY {
    pub NodeProperty: KSNODEPROPERTY,
    pub AppContext: *mut ::core::ffi::c_void,
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for KSNODEPROPERTY_AUDIO_PROPERTY {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSNODETYPE_1394_DA_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187046, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_1394_DV_STREAM_SOUNDTRACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187047, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_3D_EFFECTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1431394400, data2: 50521, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_ADC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1300463584, data2: 50517, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_AGC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3901528992, data2: 50519, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_ANALOG_CONNECTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187041, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_ANALOG_TAPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187303, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_AUDIO_ENGINE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 902493924, data2: 62387, data3: 16744, data4: [187, 75, 85, 231, 122, 70, 28, 126] };
pub const KSNODETYPE_AUDIO_KEYWORDDETECTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 941088952, data2: 57176, data3: 17269, data4: [182, 105, 196, 150, 52, 51, 31, 157] };
pub const KSNODETYPE_AUDIO_LOOPBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2403516594, data2: 37326, data3: 19407, data4: [156, 205, 14, 89, 144, 55, 171, 53] };
pub const KSNODETYPE_AUDIO_MODULE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1168815150, data2: 51947, data3: 16466, data4: [138, 169, 179, 140, 181, 16, 150, 25] };
pub const KSNODETYPE_BIDIRECTIONAL_UNDEFINED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186528, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_CABLE_TUNER_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187310, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_CD_PLAYER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187299, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_CHORUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 538394400, data2: 50521, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_COMMUNICATION_SPEAKER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186278, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1350230880, data2: 50516, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_DAT_IO_DIGITAL_AUDIO_TAPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187300, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DCC_IO_DIGITAL_COMPACT_CASSETTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187301, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DELAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 340361696, data2: 50520, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_DEMUX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3236653012, data2: 59399, data3: 4560, data4: [149, 138, 0, 192, 79, 185, 37, 211] };
pub const KSNODETYPE_DESKTOP_MICROPHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186018, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DESKTOP_SPEAKER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186276, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DEV_SPECIFIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2484894400, data2: 50521, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_DIGITAL_AUDIO_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187042, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DISPLAYPORT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3833479217, data2: 16038, data3: 16781, data4: [143, 155, 183, 56, 67, 204, 186, 151] };
pub const KSNODETYPE_DOWN_LINE_PHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186787, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DRM_DESCRAMBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4290473535, data2: 52478, data3: 19844, data4: [144, 217, 66, 20, 24, 176, 58, 142] };
pub const KSNODETYPE_DSS_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187311, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DVD_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187307, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_DYN_RANGE_COMPRESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 147367592, data2: 24607, data3: 19192, data4: [135, 147, 217, 5, 255, 76, 169, 125] };
pub const KSNODETYPE_ECHO_CANCELING_SPEAKERPHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186533, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_ECHO_SUPPRESSING_SPEAKERPHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186532, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_EMBEDDED_UNDEFINED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187296, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_EQUALIZATION_NOISE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187298, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_EQUALIZER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2638328992, data2: 50519, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_EXTERNAL_UNDEFINED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187040, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_FM_RX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2202694460, data2: 62597, data3: 16832, data4: [166, 43, 81, 48, 37, 1, 78, 64] };
pub const KSNODETYPE_HANDSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186529, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_HDMI_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3518614570, data2: 62745, data3: 16767, data4: [145, 201, 85, 250, 101, 72, 16, 1] };
pub const KSNODETYPE_HEADPHONES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186274, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_HEADSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186530, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_HEAD_MOUNTED_DISPLAY_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186275, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_INPUT_UNDEFINED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186016, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_LEGACY_AUDIO_CONNECTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187044, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_LEVEL_CALIBRATION_NOISE_SOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187297, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_LINE_CONNECTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187043, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_LOUDNESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1099461696, data2: 50520, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_LOW_FREQUENCY_EFFECTS_SPEAKER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186279, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_MICROPHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186017, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_MICROPHONE_ARRAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186021, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_MIDI_ELEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 29818470, data2: 28232, data3: 19557, data4: [172, 155, 82, 219, 93, 101, 108, 126] };
pub const KSNODETYPE_MIDI_JACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 643697727, data2: 64057, data3: 19955, data4: [171, 4, 190, 1, 185, 30, 41, 154] };
pub const KSNODETYPE_MINIDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187302, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_MULTITRACK_RECORDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187314, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_MUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 45228992, data2: 50519, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_MUX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 753596288, data2: 50518, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_NOISE_SUPPRESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3766456383, data2: 25341, data3: 20064, data4: [140, 221, 222, 167, 35, 102, 101, 181] };
pub const KSNODETYPE_OMNI_DIRECTIONAL_MICROPHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186020, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_OUTPUT_UNDEFINED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186272, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_PARAMETRIC_EQUALIZER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 431700586, data2: 52779, data3: 17474, data4: [135, 236, 103, 39, 195, 202, 180, 119] };
pub const KSNODETYPE_PEAKMETER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2693096734, data2: 24333, data3: 19254, data4: [168, 105, 209, 149, 214, 171, 75, 158] };
pub const KSNODETYPE_PERSONAL_MICROPHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186019, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_PHONE_LINE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186785, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_PHONOGRAPH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187304, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_PROCESSING_MICROPHONE_ARRAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186022, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_PROLOGIC_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2199661696, data2: 50520, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_PROLOGIC_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2155136434, data2: 15462, data3: 4562, data4: [180, 90, 48, 120, 48, 44, 32, 48] };
pub const KSNODETYPE_RADIO_RECEIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187312, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_RADIO_TRANSMITTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187313, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_REVERB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4009961696, data2: 50520, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_ROOM_SPEAKER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186277, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_SATELLITE_RECEIVER_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187309, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_SPDIF_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187045, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_SPEAKER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186273, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_SPEAKERPHONE_NO_ECHO_REDUCTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186531, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_SPEAKERS_STATIC_JACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 685789063, data2: 19902, data3: 20365, data4: [133, 137, 2, 93, 32, 157, 251, 74] };
pub const KSNODETYPE_SRC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2646063584, data2: 50517, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_STEREO_WIDE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2850461696, data2: 50520, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_SUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3661896288, data2: 50518, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_SUPERMIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3849563584, data2: 50517, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_SYNTHESIZER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187315, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_TELEPHONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186786, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_TELEPHONY_BIDI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1752005824, data2: 55555, data3: 16984, data4: [180, 67, 58, 61, 53, 128, 116, 28] };
pub const KSNODETYPE_TELEPHONY_UNDEFINED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757186784, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_TONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1980228992, data2: 50519, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const KSNODETYPE_TV_TUNER_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187308, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_UPDOWN_MIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3085813199, data2: 31587, data3: 20194, data4: [161, 0, 41, 238, 44, 182, 178, 222] };
pub const KSNODETYPE_VCR_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187305, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_CAMERA_TERMINAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757189606, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_DISC_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757187306, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_INPUT_MTT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757189607, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_INPUT_TERMINAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757189602, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_OUTPUT_MTT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757189608, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_OUTPUT_TERMINAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757189603, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_PROCESSING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757189605, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_SELECTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757189604, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VIDEO_STREAMING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757189601, data2: 63247, data3: 4560, data4: [185, 23, 0, 160, 201, 34, 49, 150] };
pub const KSNODETYPE_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 979028992, data2: 50519, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSNODE_CREATE {
    pub CreateFlags: u32,
    pub Node: u32,
}
impl ::core::marker::Copy for KSNODE_CREATE {}
impl ::core::clone::Clone for KSNODE_CREATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSNOTIFICATIONID_AudioModule: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2619482352, data2: 55718, data3: 19804, data4: [160, 54, 87, 56, 87, 253, 80, 210] };
pub const KSNOTIFICATIONID_SoundDetector: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1669978180, data2: 47922, data3: 19532, data4: [168, 2, 244, 180, 183, 122, 254, 173] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPIN_CINSTANCES {
    pub PossibleCount: u32,
    pub CurrentCount: u32,
}
impl ::core::marker::Copy for KSPIN_CINSTANCES {}
impl ::core::clone::Clone for KSPIN_CINSTANCES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPIN_COMMUNICATION = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_NONE: KSPIN_COMMUNICATION = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_SINK: KSPIN_COMMUNICATION = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_SOURCE: KSPIN_COMMUNICATION = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_BOTH: KSPIN_COMMUNICATION = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_BRIDGE: KSPIN_COMMUNICATION = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPIN_CONNECT {
    pub Interface: KSIDENTIFIER,
    pub Medium: KSIDENTIFIER,
    pub PinId: u32,
    pub PinToHandle: super::super::Foundation::HANDLE,
    pub Priority: KSPRIORITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPIN_CONNECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPIN_CONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPIN_DATAFLOW = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_DATAFLOW_IN: KSPIN_DATAFLOW = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_DATAFLOW_OUT: KSPIN_DATAFLOW = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_ASYNCHRONOUS_PROCESSING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_CRITICAL_PROCESSING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_DENY_USERMODE_ACCESS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_DISPATCH_LEVEL_PROCESSING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_DISTINCT_TRAILING_EDGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_DO_NOT_INITIATE_PROCESSING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_DO_NOT_USE_STANDARD_TRANSPORT: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_ENFORCE_FIFO: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_FIXED_FORMAT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_FRAMES_NOT_REQUIRED_FOR_PROCESSING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_GENERATE_EOS_EVENTS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_GENERATE_MAPPINGS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_HYPERCRITICAL_PROCESSING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_IMPLEMENT_CLOCK: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_INITIATE_PROCESSING_ON_EVERY_ARRIVAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_PROCESS_IF_ANY_IN_RUN_STATE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_PROCESS_IN_RUN_STATE_ONLY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_SOME_FRAMES_REQUIRED_FOR_PROCESSING: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_SPLITTER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_FLAG_USE_STANDARD_TRANSPORT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPIN_MDL_CACHING_EVENT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANUP: KSPIN_MDL_CACHING_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_WAIT: KSPIN_MDL_CACHING_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_NOWAIT: KSPIN_MDL_CACHING_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_MDL_CACHING_NOTIFY_ADDSAMPLE: KSPIN_MDL_CACHING_EVENT = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPIN_MDL_CACHING_NOTIFICATION {
    pub Event: KSPIN_MDL_CACHING_EVENT,
    pub Buffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for KSPIN_MDL_CACHING_NOTIFICATION {}
impl ::core::clone::Clone for KSPIN_MDL_CACHING_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPIN_MDL_CACHING_NOTIFICATION32 {
    pub Event: KSPIN_MDL_CACHING_EVENT,
    pub Buffer: u32,
}
impl ::core::marker::Copy for KSPIN_MDL_CACHING_NOTIFICATION32 {}
impl ::core::clone::Clone for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPIN_PHYSICALCONNECTION {
    pub Size: u32,
    pub Pin: u32,
    pub SymbolicLinkName: [u16; 1],
}
impl ::core::marker::Copy for KSPIN_PHYSICALCONNECTION {}
impl ::core::clone::Clone for KSPIN_PHYSICALCONNECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPPROPERTY_ALLOCATOR_MDLCACHING = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CLEANUP_CACHEDMDLPAGES: KSPPROPERTY_ALLOCATOR_MDLCACHING = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPRIORITY {
    pub PriorityClass: u32,
    pub PrioritySubClass: u32,
}
impl ::core::marker::Copy for KSPRIORITY {}
impl ::core::clone::Clone for KSPRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPRIORITY_EXCLUSIVE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPRIORITY_HIGH: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPRIORITY_LOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPRIORITY_NORMAL: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROBE_ALLOCATEMDL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROBE_ALLOWFORMATCHANGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROBE_MODIFY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROBE_PROBEANDLOCK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROBE_STREAMREAD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROBE_STREAMWRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROBE_SYSTEMADDRESS: u32 = 64u32;
pub const KSPROPERTYSETID_ExtendedCameraControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 481792274, data2: 49362, data3: 16915, data4: [156, 166, 205, 79, 219, 146, 121, 114] };
pub const KSPROPERTYSETID_NetworkCameraControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 242749193, data2: 22341, data3: 20026, data4: [188, 159, 242, 38, 234, 67, 166, 236] };
pub const KSPROPERTYSETID_PerFrameSettingControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4059292257, data2: 57062, data3: 17719, data4: [191, 245, 238, 32, 109, 181, 74, 172] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_AC3 = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_ERROR_CONCEALMENT: KSPROPERTY_AC3 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_ALTERNATE_AUDIO: KSPROPERTY_AC3 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_DOWNMIX: KSPROPERTY_AC3 = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_BIT_STREAM_MODE: KSPROPERTY_AC3 = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_DIALOGUE_LEVEL: KSPROPERTY_AC3 = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_LANGUAGE_CODE: KSPROPERTY_AC3 = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_ROOM_TYPE: KSPROPERTY_AC3 = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_ALLOCATOR_CONTROL = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CONTROL_HONOR_COUNT: KSPROPERTY_ALLOCATOR_CONTROL = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE: KSPROPERTY_ALLOCATOR_CONTROL = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS: KSPROPERTY_ALLOCATOR_CONTROL = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE: KSPROPERTY_ALLOCATOR_CONTROL = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    pub InterleavedCapSupported: u32,
}
impl ::core::marker::Copy for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    pub InterleavedCapPossible: u32,
}
impl ::core::marker::Copy for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {}
impl ::core::clone::Clone for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    pub CX: u32,
    pub CY: u32,
}
impl ::core::marker::Copy for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {}
impl ::core::clone::Clone for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_AUDDECOUT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDDECOUT_MODES: KSPROPERTY_AUDDECOUT = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDDECOUT_CUR_MODE: KSPROPERTY_AUDDECOUT = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_AUDIO = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_LATENCY: KSPROPERTY_AUDIO = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_COPY_PROTECTION: KSPROPERTY_AUDIO = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CHANNEL_CONFIG: KSPROPERTY_AUDIO = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_VOLUMELEVEL: KSPROPERTY_AUDIO = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_POSITION: KSPROPERTY_AUDIO = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DYNAMIC_RANGE: KSPROPERTY_AUDIO = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_QUALITY: KSPROPERTY_AUDIO = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_SAMPLING_RATE: KSPROPERTY_AUDIO = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DYNAMIC_SAMPLING_RATE: KSPROPERTY_AUDIO = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIX_LEVEL_TABLE: KSPROPERTY_AUDIO = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIX_LEVEL_CAPS: KSPROPERTY_AUDIO = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MUX_SOURCE: KSPROPERTY_AUDIO = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MUTE: KSPROPERTY_AUDIO = 13i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_BASS: KSPROPERTY_AUDIO = 14i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MID: KSPROPERTY_AUDIO = 15i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_TREBLE: KSPROPERTY_AUDIO = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_BASS_BOOST: KSPROPERTY_AUDIO = 17i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_EQ_LEVEL: KSPROPERTY_AUDIO = 18i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_NUM_EQ_BANDS: KSPROPERTY_AUDIO = 19i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_EQ_BANDS: KSPROPERTY_AUDIO = 20i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_AGC: KSPROPERTY_AUDIO = 21i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DELAY: KSPROPERTY_AUDIO = 22i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_LOUDNESS: KSPROPERTY_AUDIO = 23i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_WIDE_MODE: KSPROPERTY_AUDIO = 24i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_WIDENESS: KSPROPERTY_AUDIO = 25i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_REVERB_LEVEL: KSPROPERTY_AUDIO = 26i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CHORUS_LEVEL: KSPROPERTY_AUDIO = 27i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DEV_SPECIFIC: KSPROPERTY_AUDIO = 28i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DEMUX_DEST: KSPROPERTY_AUDIO = 29i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_STEREO_ENHANCE: KSPROPERTY_AUDIO = 30i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MANUFACTURE_GUID: KSPROPERTY_AUDIO = 31i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PRODUCT_GUID: KSPROPERTY_AUDIO = 32i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CPU_RESOURCES: KSPROPERTY_AUDIO = 33i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_STEREO_SPEAKER_GEOMETRY: KSPROPERTY_AUDIO = 34i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_SURROUND_ENCODE: KSPROPERTY_AUDIO = 35i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_3D_INTERFACE: KSPROPERTY_AUDIO = 36i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEAKMETER: KSPROPERTY_AUDIO = 37i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_ALGORITHM_INSTANCE: KSPROPERTY_AUDIO = 38i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_FILTER_STATE: KSPROPERTY_AUDIO = 39i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PREFERRED_STATUS: KSPROPERTY_AUDIO = 40i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_MAX_BANDS: KSPROPERTY_AUDIO = 41i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_NUM_BANDS: KSPROPERTY_AUDIO = 42i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_BAND_CENTER_FREQ: KSPROPERTY_AUDIO = 43i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_BAND_Q_FACTOR: KSPROPERTY_AUDIO = 44i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_BAND_LEVEL: KSPROPERTY_AUDIO = 45i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_RATE: KSPROPERTY_AUDIO = 46i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_DEPTH: KSPROPERTY_AUDIO = 47i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_REVERB_TIME: KSPROPERTY_AUDIO = 48i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_REVERB_DELAY_FEEDBACK: KSPROPERTY_AUDIO = 49i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_POSITIONEX: KSPROPERTY_AUDIO = 50i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIC_ARRAY_GEOMETRY: KSPROPERTY_AUDIO = 51i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PRESENTATION_POSITION: KSPROPERTY_AUDIO = 52i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_POSITION: KSPROPERTY_AUDIO = 53i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_LINEAR_BUFFER_POSITION: KSPROPERTY_AUDIO = 54i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEAKMETER2: KSPROPERTY_AUDIO = 55i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_LASTBUFFER_POSITION: KSPROPERTY_AUDIO = 56i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_VOLUMELIMIT_ENGAGED: KSPROPERTY_AUDIO = 57i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY: KSPROPERTY_AUDIO = 58i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIC_SNR: KSPROPERTY_AUDIO = 59i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY2: KSPROPERTY_AUDIO = 60i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_AUDIOENGINE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_LFXENABLE: KSPROPERTY_AUDIOENGINE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_GFXENABLE: KSPROPERTY_AUDIOENGINE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_MIXFORMAT: KSPROPERTY_AUDIOENGINE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_DEVICEFORMAT: KSPROPERTY_AUDIOENGINE = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_SUPPORTEDDEVICEFORMATS: KSPROPERTY_AUDIOENGINE = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_DESCRIPTOR: KSPROPERTY_AUDIOENGINE = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_BUFFER_SIZE_RANGE: KSPROPERTY_AUDIOENGINE = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_LOOPBACK_PROTECTION: KSPROPERTY_AUDIOENGINE = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_VOLUMELEVEL: KSPROPERTY_AUDIOENGINE = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_AUDIOMODULE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOMODULE_DESCRIPTORS: KSPROPERTY_AUDIOMODULE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOMODULE_COMMAND: KSPROPERTY_AUDIOMODULE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOMODULE_NOTIFICATION_DEVICE_ID: KSPROPERTY_AUDIOMODULE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_AUDIOPOSTURE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOPOSTURE_ORIENTATION: KSPROPERTY_AUDIOPOSTURE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_AUDIORESOURCEMANAGEMENT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIORESOURCEMANAGEMENT_RESOURCEGROUP: KSPROPERTY_AUDIORESOURCEMANAGEMENT = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_AUDIOSIGNALPROCESSING = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOSIGNALPROCESSING_MODES: KSPROPERTY_AUDIOSIGNALPROCESSING = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_BUFFER_DURATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_BIBLIOGRAPHIC = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_LEADER: KSPROPERTY_BIBLIOGRAPHIC = 1380207648i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_LCCN: KSPROPERTY_BIBLIOGRAPHIC = 808529952i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ISBN: KSPROPERTY_BIBLIOGRAPHIC = 808595488i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ISSN: KSPROPERTY_BIBLIOGRAPHIC = 842149920i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_CATALOGINGSOURCE: KSPROPERTY_BIBLIOGRAPHIC = 808726560i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = 808464672i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINCORPORATEBODY: KSPROPERTY_BIBLIOGRAPHIC = 808530208i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINMEETINGNAME: KSPROPERTY_BIBLIOGRAPHIC = 825307424i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808661280i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_UNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808727072i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_TITLESTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = 892613152i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_VARYINGFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 909390368i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_PUBLICATION: KSPROPERTY_BIBLIOGRAPHIC = 808858144i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_PHYSICALDESCRIPTION: KSPROPERTY_BIBLIOGRAPHIC = 808465184i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808727584i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = 809055264i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_GENERALNOTE: KSPROPERTY_BIBLIOGRAPHIC = 808465696i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_BIBLIOGRAPHYNOTE: KSPROPERTY_BIBLIOGRAPHIC = 875574560i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_CONTENTSNOTE: KSPROPERTY_BIBLIOGRAPHIC = 892351776i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_CREATIONCREDIT: KSPROPERTY_BIBLIOGRAPHIC = 942683424i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_CITATION: KSPROPERTY_BIBLIOGRAPHIC = 808531232i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_PARTICIPANT: KSPROPERTY_BIBLIOGRAPHIC = 825308448i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SUMMARY: KSPROPERTY_BIBLIOGRAPHIC = 808596768i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_TARGETAUDIENCE: KSPROPERTY_BIBLIOGRAPHIC = 825373984i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDFORMAVAILABLE: KSPROPERTY_BIBLIOGRAPHIC = 808662304i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SYSTEMDETAILS: KSPROPERTY_BIBLIOGRAPHIC = 942880032i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_AWARDS: KSPROPERTY_BIBLIOGRAPHIC = 909653280i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = 808465952i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTOPICALTERM: KSPROPERTY_BIBLIOGRAPHIC = 808793632i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYGEOGRAPHIC: KSPROPERTY_BIBLIOGRAPHIC = 825570848i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMGENRE: KSPROPERTY_BIBLIOGRAPHIC = 892679712i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMCURRICULUM: KSPROPERTY_BIBLIOGRAPHIC = 943011360i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808662816i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYRELATED: KSPROPERTY_BIBLIOGRAPHIC = 808728352i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = 808466464i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808663072i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSPROPERTY_BOUNDS_LONG {
    pub Anonymous1: KSPROPERTY_BOUNDS_LONG_0,
    pub Anonymous2: KSPROPERTY_BOUNDS_LONG_1,
}
impl ::core::marker::Copy for KSPROPERTY_BOUNDS_LONG {}
impl ::core::clone::Clone for KSPROPERTY_BOUNDS_LONG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_BOUNDS_LONG_0 {
    pub SignedMinimum: i32,
    pub SignedMaximum: i32,
}
impl ::core::marker::Copy for KSPROPERTY_BOUNDS_LONG_0 {}
impl ::core::clone::Clone for KSPROPERTY_BOUNDS_LONG_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_BOUNDS_LONG_1 {
    pub UnsignedMinimum: u32,
    pub UnsignedMaximum: u32,
}
impl ::core::marker::Copy for KSPROPERTY_BOUNDS_LONG_1 {}
impl ::core::clone::Clone for KSPROPERTY_BOUNDS_LONG_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSPROPERTY_BOUNDS_LONGLONG {
    pub Anonymous1: KSPROPERTY_BOUNDS_LONGLONG_0,
    pub Anonymous2: KSPROPERTY_BOUNDS_LONGLONG_1,
}
impl ::core::marker::Copy for KSPROPERTY_BOUNDS_LONGLONG {}
impl ::core::clone::Clone for KSPROPERTY_BOUNDS_LONGLONG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_BOUNDS_LONGLONG_0 {
    pub SignedMinimum: i64,
    pub SignedMaximum: i64,
}
impl ::core::marker::Copy for KSPROPERTY_BOUNDS_LONGLONG_0 {}
impl ::core::clone::Clone for KSPROPERTY_BOUNDS_LONGLONG_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_BOUNDS_LONGLONG_1 {
    pub UnsignedMinimum: u64,
    pub UnsignedMaximum: u64,
}
impl ::core::marker::Copy for KSPROPERTY_BOUNDS_LONGLONG_1 {}
impl ::core::clone::Clone for KSPROPERTY_BOUNDS_LONGLONG_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_BTAUDIO = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ONESHOT_RECONNECT: KSPROPERTY_BTAUDIO = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ONESHOT_DISCONNECT: KSPROPERTY_BTAUDIO = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMAXFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTRIGGERTIME: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WARMSTART: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MAXVIDFPS_PHOTORES: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTHUMBNAIL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SCENEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_TORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FLASHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OPTIMIZATIONHINT: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WHITEBALANCEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EXPOSUREMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 13i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 14i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FIELDOFVIEW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 15i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EVCOMPENSATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_CAMERAANGLEOFFSET: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 17i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_METADATA: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 18i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSPRIORITY: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 19i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSSTATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 20i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 21i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_ISPCONTROL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 22i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOCONFIRMATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 23i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ZOOM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 24i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MCC: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 25i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO_ADVANCED: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 26i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOSTABILIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 27i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VFR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 28i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEDETECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 29i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOHDR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 30i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_HISTOGRAM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 31i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OIS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 32i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ADVANCEDPHOTO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 33i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PROFILE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 34i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEAUTH_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 35i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SECURE_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 36i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOTEMPORALDENOISING: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 37i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_IRTORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 38i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_RELATIVEPANELOPTIMIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 39i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EYEGAZECORRECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 40i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_BACKGROUNDSEGMENTATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 41i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 42i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 43i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 44i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END2: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 44i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ABSOLUTE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ASYNCHRONOUS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_MANUAL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_RELATIVE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CAMERACONTROL_FLASH = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_FLASH = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_AUTO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_MANUAL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_ON: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_FLASH_S {
    pub Flash: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_FLASH_S {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    pub Property: KSIDENTIFIER,
    pub lOcularFocalLength: i32,
    pub lObjectiveFocalLengthMin: i32,
    pub lObjectiveFocalLengthMax: i32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_EXCLUSIVE_WITH_RECORD: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    pub Capabilities: u32,
    pub Reserved0: u32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_SEQUENCE_EXCLUSIVE_WITH_RECORD: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    pub NodeProperty: KSNODEPROPERTY,
    pub lOcularFocalLength: i32,
    pub lObjectiveFocalLengthMin: i32,
    pub lObjectiveFocalLengthMax: i32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_NODE_S {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_NODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_NODE_S2 {
    pub NodeProperty: KSP_NODE,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_NODE_S2 {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CAPABILITY: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_SET: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CLEAR: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_EXPOSURE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_FOCUS: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_WB: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONVERGEMODE: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_ASYNC: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_MANUAL: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    pub FocusRect: super::super::Foundation::RECT,
    pub AutoFocusLock: super::super::Foundation::BOOL,
    pub AutoExposureLock: super::super::Foundation::BOOL,
    pub AutoWhitebalanceLock: super::super::Foundation::BOOL,
    pub Anonymous: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    pub Capabilities: u32,
    pub Configuration: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_S {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_S2 {
    pub Property: KSIDENTIFIER,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_S2 {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_S2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_CAMERACONTROL_S_EX {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub FocusRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_S_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_S_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_AUTO: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_MANUAL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_HIGH: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_LOW: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_MEDIUM: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_OFF: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    pub VideoStabilizationMode: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_CLEAR: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_SET: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CLOCK = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_TIME: KSPROPERTY_CLOCK = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_PHYSICALTIME: KSPROPERTY_CLOCK = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_CORRELATEDTIME: KSPROPERTY_CLOCK = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_CORRELATEDPHYSICALTIME: KSPROPERTY_CLOCK = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_RESOLUTION: KSPROPERTY_CLOCK = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_STATE: KSPROPERTY_CLOCK = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CONNECTION = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_STATE: KSPROPERTY_CONNECTION = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_PRIORITY: KSPROPERTY_CONNECTION = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_DATAFORMAT: KSPROPERTY_CONNECTION = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING: KSPROPERTY_CONNECTION = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_PROPOSEDATAFORMAT: KSPROPERTY_CONNECTION = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_ACQUIREORDERING: KSPROPERTY_CONNECTION = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING_EX: KSPROPERTY_CONNECTION = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_STARTAT: KSPROPERTY_CONNECTION = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_COPYPROT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_CHLG_KEY: KSPROPERTY_COPYPROT = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_DVD_KEY1: KSPROPERTY_COPYPROT = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_DEC_KEY2: KSPROPERTY_COPYPROT = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_TITLE_KEY: KSPROPERTY_COPYPROT = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_COPY_MACROVISION: KSPROPERTY_COPYPROT = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_REGION: KSPROPERTY_COPYPROT = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_SET_COPY_STATE: KSPROPERTY_COPYPROT = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_DISC_KEY: KSPROPERTY_COPYPROT = 128i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CROSSBAR_ACTIVE_S {
    pub Property: KSIDENTIFIER,
    pub IndexInputPin: u32,
    pub Active: u32,
}
impl ::core::marker::Copy for KSPROPERTY_CROSSBAR_ACTIVE_S {}
impl ::core::clone::Clone for KSPROPERTY_CROSSBAR_ACTIVE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CROSSBAR_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfInputs: u32,
    pub NumberOfOutputs: u32,
}
impl ::core::marker::Copy for KSPROPERTY_CROSSBAR_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_CROSSBAR_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CROSSBAR_PININFO_S {
    pub Property: KSIDENTIFIER,
    pub Direction: KSPIN_DATAFLOW,
    pub Index: u32,
    pub PinType: u32,
    pub RelatedPinIndex: u32,
    pub Medium: KSIDENTIFIER,
}
impl ::core::marker::Copy for KSPROPERTY_CROSSBAR_PININFO_S {}
impl ::core::clone::Clone for KSPROPERTY_CROSSBAR_PININFO_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_CROSSBAR_ROUTE_S {
    pub Property: KSIDENTIFIER,
    pub IndexInputPin: u32,
    pub IndexOutputPin: u32,
    pub CanRoute: u32,
}
impl ::core::marker::Copy for KSPROPERTY_CROSSBAR_ROUTE_S {}
impl ::core::clone::Clone for KSPROPERTY_CROSSBAR_ROUTE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_CYCLIC = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CYCLIC_POSITION: KSPROPERTY_CYCLIC = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_DESCRIPTION {
    pub AccessFlags: u32,
    pub DescriptionSize: u32,
    pub PropTypeSet: KSIDENTIFIER,
    pub MembersListCount: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSPROPERTY_DESCRIPTION {}
impl ::core::clone::Clone for KSPROPERTY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_DIRECTSOUND3DBUFFER = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_ALL: KSPROPERTY_DIRECTSOUND3DBUFFER = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_POSITION: KSPROPERTY_DIRECTSOUND3DBUFFER = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_VELOCITY: KSPROPERTY_DIRECTSOUND3DBUFFER = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEANGLES: KSPROPERTY_DIRECTSOUND3DBUFFER = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEORIENTATION: KSPROPERTY_DIRECTSOUND3DBUFFER = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEOUTSIDEVOLUME: KSPROPERTY_DIRECTSOUND3DBUFFER = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MINDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MAXDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MODE: KSPROPERTY_DIRECTSOUND3DBUFFER = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_DIRECTSOUND3DLISTENER = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALL: KSPROPERTY_DIRECTSOUND3DLISTENER = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_POSITION: KSPROPERTY_DIRECTSOUND3DLISTENER = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_VELOCITY: KSPROPERTY_DIRECTSOUND3DLISTENER = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ORIENTATION: KSPROPERTY_DIRECTSOUND3DLISTENER = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DISTANCEFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ROLLOFFFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DOPPLERFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_BATCH: KSPROPERTY_DIRECTSOUND3DLISTENER = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALLOCATION: KSPROPERTY_DIRECTSOUND3DLISTENER = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_DRMAUDIOSTREAM = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DRMAUDIOSTREAM_CONTENTID: KSPROPERTY_DRMAUDIOSTREAM = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    pub Property: KSIDENTIFIER,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub AverageFrameSize: u32,
}
impl ::core::marker::Copy for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {}
impl ::core::clone::Clone for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_DVDSUBPIC = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDSUBPIC_PALETTE: KSPROPERTY_DVDSUBPIC = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDSUBPIC_HLI: KSPROPERTY_DVDSUBPIC = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDSUBPIC_COMPOSIT_ON: KSPROPERTY_DVDSUBPIC = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_EXTDEVICE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_ID: KSPROPERTY_EXTDEVICE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_VERSION: KSPROPERTY_EXTDEVICE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_POWER_STATE: KSPROPERTY_EXTDEVICE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_PORT: KSPROPERTY_EXTDEVICE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_CAPABILITIES: KSPROPERTY_EXTDEVICE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_EXTDEVICE_S {
    pub Property: KSIDENTIFIER,
    pub u: KSPROPERTY_EXTDEVICE_S_0,
}
impl ::core::marker::Copy for KSPROPERTY_EXTDEVICE_S {}
impl ::core::clone::Clone for KSPROPERTY_EXTDEVICE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSPROPERTY_EXTDEVICE_S_0 {
    pub Capabilities: DEVCAPS,
    pub DevPort: u32,
    pub PowerState: u32,
    pub pawchString: [u16; 260],
    pub NodeUniqueID: [u32; 2],
}
impl ::core::marker::Copy for KSPROPERTY_EXTDEVICE_S_0 {}
impl ::core::clone::Clone for KSPROPERTY_EXTDEVICE_S_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_EXTENSION_UNIT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTENSION_UNIT_INFO: KSPROPERTY_EXTENSION_UNIT = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTENSION_UNIT_CONTROL: KSPROPERTY_EXTENSION_UNIT = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTENSION_UNIT_PASS_THROUGH: KSPROPERTY_EXTENSION_UNIT = 65535i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_EXTXPORT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_CAPABILITIES: KSPROPERTY_EXTXPORT = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_INPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_OUTPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_LOAD_MEDIUM: KSPROPERTY_EXTXPORT = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_MEDIUM_INFO: KSPROPERTY_EXTXPORT = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_STATE: KSPROPERTY_EXTXPORT = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_STATE_NOTIFY: KSPROPERTY_EXTXPORT = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_TIMECODE_SEARCH: KSPROPERTY_EXTXPORT = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_ATN_SEARCH: KSPROPERTY_EXTXPORT = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_RTC_SEARCH: KSPROPERTY_EXTXPORT = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RAW_AVC_CMD: KSPROPERTY_EXTXPORT = 10i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub u: KSPROPERTY_EXTXPORT_NODE_S_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT_NODE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT_NODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KSPROPERTY_EXTXPORT_NODE_S_0 {
    pub Capabilities: u32,
    pub SignalMode: u32,
    pub LoadMedium: u32,
    pub MediumInfo: MEDIUM_INFO,
    pub XPrtState: TRANSPORT_STATE,
    pub Timecode: KSPROPERTY_EXTXPORT_NODE_S_0_1,
    pub dwTimecode: u32,
    pub dwAbsTrackNumber: u32,
    pub RawAVC: KSPROPERTY_EXTXPORT_NODE_S_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT_NODE_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT_NODE_S_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    pub PayloadSize: u32,
    pub Payload: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT_NODE_S_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT_NODE_S_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_S {
    pub Property: KSIDENTIFIER,
    pub u: KSPROPERTY_EXTXPORT_S_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KSPROPERTY_EXTXPORT_S_0 {
    pub Capabilities: u32,
    pub SignalMode: u32,
    pub LoadMedium: u32,
    pub MediumInfo: MEDIUM_INFO,
    pub XPrtState: TRANSPORT_STATE,
    pub Timecode: KSPROPERTY_EXTXPORT_S_0_1,
    pub dwTimecode: u32,
    pub dwAbsTrackNumber: u32,
    pub RawAVC: KSPROPERTY_EXTXPORT_S_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT_S_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_S_0_0 {
    pub PayloadSize: u32,
    pub Payload: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT_S_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT_S_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_S_0_1 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT_S_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT_S_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_FMRX_CONTROL = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_FMRX_STATE: KSPROPERTY_FMRX_CONTROL = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_FMRX_TOPOLOGY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_FMRX_ENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_FMRX_VOLUME: KSPROPERTY_FMRX_TOPOLOGY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_FMRX_ANTENNAENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_GENERAL = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_GENERAL_COMPONENTID: KSPROPERTY_GENERAL = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_HRTF3D = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_HRTF3D_PARAMS: KSPROPERTY_HRTF3D = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_HRTF3D_INITIALIZE: KSPROPERTY_HRTF3D = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_HRTF3D_FILTER_FORMAT: KSPROPERTY_HRTF3D = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_INTERLEAVEDAUDIO = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_INTERLEAVEDAUDIO_FORMATINFORMATION: KSPROPERTY_INTERLEAVEDAUDIO = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_ITD3D = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ITD3D_PARAMS: KSPROPERTY_ITD3D = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_JACK = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_JACK_DESCRIPTION: KSPROPERTY_JACK = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_JACK_DESCRIPTION2: KSPROPERTY_JACK = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_JACK_SINK_INFO: KSPROPERTY_JACK = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_JACK_CONTAINERID: KSPROPERTY_JACK = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_MEDIAAVAILABLE {
    pub Earliest: i64,
    pub Latest: i64,
}
impl ::core::marker::Copy for KSPROPERTY_MEDIAAVAILABLE {}
impl ::core::clone::Clone for KSPROPERTY_MEDIAAVAILABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_MEDIASEEKING = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_CAPABILITIES: KSPROPERTY_MEDIASEEKING = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_FORMATS: KSPROPERTY_MEDIASEEKING = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_TIMEFORMAT: KSPROPERTY_MEDIASEEKING = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_POSITION: KSPROPERTY_MEDIASEEKING = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_STOPPOSITION: KSPROPERTY_MEDIASEEKING = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_POSITIONS: KSPROPERTY_MEDIASEEKING = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_DURATION: KSPROPERTY_MEDIASEEKING = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_AVAILABLE: KSPROPERTY_MEDIASEEKING = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_PREROLL: KSPROPERTY_MEDIASEEKING = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_CONVERTTIMEFORMAT: KSPROPERTY_MEDIASEEKING = 9i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_MEMBERSHEADER {
    pub MembersFlags: u32,
    pub MembersSize: u32,
    pub MembersCount: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for KSPROPERTY_MEMBERSHEADER {}
impl ::core::clone::Clone for KSPROPERTY_MEMBERSHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEMBER_FLAG_BASICSUPPORT_MULTICHANNEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEMBER_FLAG_BASICSUPPORT_UNIFORM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEMBER_FLAG_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEMBER_RANGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEMBER_STEPPEDRANGES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEMBER_VALUES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEMORY_TRANSPORT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_MPEG2VID = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_MODES: KSPROPERTY_MPEG2VID = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_CUR_MODE: KSPROPERTY_MPEG2VID = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_4_3_RECT: KSPROPERTY_MPEG2VID = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_16_9_RECT: KSPROPERTY_MPEG2VID = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_16_9_PANSCAN: KSPROPERTY_MPEG2VID = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG4_MEDIATYPE_SD_BOX: KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub EventFilter: [u16; 1],
}
impl ::core::marker::Copy for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {}
impl ::core::clone::Clone for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    pub MetadataItems: u32,
    pub Size: u32,
    pub PTZStatus: super::super::Foundation::BOOL,
    pub Events: super::super::Foundation::BOOL,
    pub Analytics: super::super::Foundation::BOOL,
    pub Reserved: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE_EVENTSINFO: KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    pub Size: u32,
    pub Type: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE,
}
impl ::core::marker::Copy for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {}
impl ::core::clone::Clone for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_DISABLE: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_HOSTNTP: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERYT_NETWORKCAMERACONTROL_NTPINFO_TYPE_CUSTOM: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTP: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_URI: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_EVENTTOPICS_XML: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_OVERLAYUPDATE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_INTERESTS: KSPROPERTY_OVERLAYUPDATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_CLIPLIST: KSPROPERTY_OVERLAYUPDATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_PALETTE: KSPROPERTY_OVERLAYUPDATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_COLORKEY: KSPROPERTY_OVERLAYUPDATE = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_VIDEOPOSITION: KSPROPERTY_OVERLAYUPDATE = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_DISPLAYCHANGE: KSPROPERTY_OVERLAYUPDATE = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_COLORREF: KSPROPERTY_OVERLAYUPDATE = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_PIN = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_CINSTANCES: KSPROPERTY_PIN = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_CTYPES: KSPROPERTY_PIN = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_DATAFLOW: KSPROPERTY_PIN = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_DATARANGES: KSPROPERTY_PIN = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_DATAINTERSECTION: KSPROPERTY_PIN = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_INTERFACES: KSPROPERTY_PIN = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_MEDIUMS: KSPROPERTY_PIN = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_COMMUNICATION: KSPROPERTY_PIN = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_GLOBALCINSTANCES: KSPROPERTY_PIN = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_NECESSARYINSTANCES: KSPROPERTY_PIN = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_PHYSICALCONNECTION: KSPROPERTY_PIN = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_CATEGORY: KSPROPERTY_PIN = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_NAME: KSPROPERTY_PIN = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_CONSTRAINEDDATARANGES: KSPROPERTY_PIN = 13i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT: KSPROPERTY_PIN = 14i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT2: KSPROPERTY_PIN = 15i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_MODEDATAFORMATS: KSPROPERTY_PIN = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_FLAGS_ATTRIBUTE_RANGE_AWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_FLAGS_MASK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_POSITIONS {
    pub Current: i64,
    pub Stop: i64,
    pub CurrentFlags: KS_SEEKING_FLAGS,
    pub StopFlags: KS_SEEKING_FLAGS,
}
impl ::core::marker::Copy for KSPROPERTY_POSITIONS {}
impl ::core::clone::Clone for KSPROPERTY_POSITIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_QUALITY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_QUALITY_REPORT: KSPROPERTY_QUALITY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_QUALITY_ERROR: KSPROPERTY_QUALITY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_RTAUDIO = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_GETPOSITIONFUNCTION: KSPROPERTY_RTAUDIO = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_BUFFER: KSPROPERTY_RTAUDIO = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_HWLATENCY: KSPROPERTY_RTAUDIO = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_POSITIONREGISTER: KSPROPERTY_RTAUDIO = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_CLOCKREGISTER: KSPROPERTY_RTAUDIO = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_BUFFER_WITH_NOTIFICATION: KSPROPERTY_RTAUDIO = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_REGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_UNREGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_QUERY_NOTIFICATION_SUPPORT: KSPROPERTY_RTAUDIO = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_PACKETCOUNT: KSPROPERTY_RTAUDIO = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_PRESENTATION_POSITION: KSPROPERTY_RTAUDIO = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_GETREADPACKET: KSPROPERTY_RTAUDIO = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_SETWRITEPACKET: KSPROPERTY_RTAUDIO = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_PACKETVREGISTER: KSPROPERTY_RTAUDIO = 13i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_SELECTOR_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_SELECTOR_NODE_S {}
impl ::core::clone::Clone for KSPROPERTY_SELECTOR_NODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_SELECTOR_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_SELECTOR_S {}
impl ::core::clone::Clone for KSPROPERTY_SELECTOR_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_SERIAL {
    pub PropTypeSet: KSIDENTIFIER,
    pub Id: u32,
    pub PropertyLength: u32,
}
impl ::core::marker::Copy for KSPROPERTY_SERIAL {}
impl ::core::clone::Clone for KSPROPERTY_SERIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_SERIALHDR {
    pub PropertySet: ::windows_sys::core::GUID,
    pub Count: u32,
}
impl ::core::marker::Copy for KSPROPERTY_SERIALHDR {}
impl ::core::clone::Clone for KSPROPERTY_SERIALHDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_SOUNDDETECTOR = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_SUPPORTEDPATTERNS: KSPROPERTY_SOUNDDETECTOR = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_PATTERNS: KSPROPERTY_SOUNDDETECTOR = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_ARMED: KSPROPERTY_SOUNDDETECTOR = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_MATCHRESULT: KSPROPERTY_SOUNDDETECTOR = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_RESET: KSPROPERTY_SOUNDDETECTOR = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_STREAMINGSUPPORT: KSPROPERTY_SOUNDDETECTOR = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_SPHLI {
    pub HLISS: u16,
    pub Reserved: u16,
    pub StartPTM: u32,
    pub EndPTM: u32,
    pub StartX: u16,
    pub StartY: u16,
    pub StopX: u16,
    pub StopY: u16,
    pub ColCon: KS_COLCON,
}
impl ::core::marker::Copy for KSPROPERTY_SPHLI {}
impl ::core::clone::Clone for KSPROPERTY_SPHLI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_SPPAL {
    pub sppal: [KS_DVD_YUV; 16],
}
impl ::core::marker::Copy for KSPROPERTY_SPPAL {}
impl ::core::clone::Clone for KSPROPERTY_SPPAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_STEPPING_LONG {
    pub SteppingDelta: u32,
    pub Reserved: u32,
    pub Bounds: KSPROPERTY_BOUNDS_LONG,
}
impl ::core::marker::Copy for KSPROPERTY_STEPPING_LONG {}
impl ::core::clone::Clone for KSPROPERTY_STEPPING_LONG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_STEPPING_LONGLONG {
    pub SteppingDelta: u64,
    pub Bounds: KSPROPERTY_BOUNDS_LONGLONG,
}
impl ::core::marker::Copy for KSPROPERTY_STEPPING_LONGLONG {}
impl ::core::clone::Clone for KSPROPERTY_STEPPING_LONGLONG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_STREAM = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_ALLOCATOR: KSPROPERTY_STREAM = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_QUALITY: KSPROPERTY_STREAM = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_DEGRADATION: KSPROPERTY_STREAM = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_MASTERCLOCK: KSPROPERTY_STREAM = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_TIMEFORMAT: KSPROPERTY_STREAM = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_PRESENTATIONTIME: KSPROPERTY_STREAM = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_PRESENTATIONEXTENT: KSPROPERTY_STREAM = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_FRAMETIME: KSPROPERTY_STREAM = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_RATECAPABILITY: KSPROPERTY_STREAM = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_RATE: KSPROPERTY_STREAM = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_PIPE_ID: KSPROPERTY_STREAM = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_STREAMINTERFACE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAMINTERFACE_HEADERSIZE: KSPROPERTY_STREAMINTERFACE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_TELEPHONY_CONTROL = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_PROVIDERID: KSPROPERTY_TELEPHONY_CONTROL = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_CALLINFO: KSPROPERTY_TELEPHONY_CONTROL = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_CALLCONTROL: KSPROPERTY_TELEPHONY_CONTROL = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_PROVIDERCHANGE: KSPROPERTY_TELEPHONY_CONTROL = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_CALLHOLD: KSPROPERTY_TELEPHONY_CONTROL = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_MUTE_TX: KSPROPERTY_TELEPHONY_CONTROL = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_TELEPHONY_TOPOLOGY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_ENDPOINTIDPAIR: KSPROPERTY_TELEPHONY_TOPOLOGY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_VOLUME: KSPROPERTY_TELEPHONY_TOPOLOGY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_TIMECODE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TIMECODE_READER: KSPROPERTY_TIMECODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ATN_READER: KSPROPERTY_TIMECODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTC_READER: KSPROPERTY_TIMECODE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TIMECODE_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub TimecodeSamp: super::TIMECODE_SAMPLE,
}
impl ::core::marker::Copy for KSPROPERTY_TIMECODE_NODE_S {}
impl ::core::clone::Clone for KSPROPERTY_TIMECODE_NODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TIMECODE_S {
    pub Property: KSIDENTIFIER,
    pub TimecodeSamp: super::TIMECODE_SAMPLE,
}
impl ::core::marker::Copy for KSPROPERTY_TIMECODE_S {}
impl ::core::clone::Clone for KSPROPERTY_TIMECODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_TOPOLOGY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGY_CATEGORIES: KSPROPERTY_TOPOLOGY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGY_NODES: KSPROPERTY_TOPOLOGY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGY_CONNECTIONS: KSPROPERTY_TOPOLOGY = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGY_NAME: KSPROPERTY_TOPOLOGY = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_TOPOLOGYNODE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGYNODE_ENABLE: KSPROPERTY_TOPOLOGYNODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGYNODE_RESET: KSPROPERTY_TOPOLOGYNODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_TUNER = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_CAPS: KSPROPERTY_TUNER = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_CAPS: KSPROPERTY_TUNER = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE: KSPROPERTY_TUNER = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_STANDARD: KSPROPERTY_TUNER = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_FREQUENCY: KSPROPERTY_TUNER = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_INPUT: KSPROPERTY_TUNER = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_STATUS: KSPROPERTY_TUNER = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_IF_MEDIUM: KSPROPERTY_TUNER = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_SCAN_CAPS: KSPROPERTY_TUNER = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_SCAN_STATUS: KSPROPERTY_TUNER = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_STANDARD_MODE: KSPROPERTY_TUNER = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS: KSPROPERTY_TUNER = 11i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub ModesSupported: u32,
    pub VideoMedium: KSIDENTIFIER,
    pub TVAudioMedium: KSIDENTIFIER,
    pub RadioAudioMedium: KSIDENTIFIER,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_FREQUENCY_S {
    pub Property: KSIDENTIFIER,
    pub Frequency: u32,
    pub LastFrequency: u32,
    pub TuningFlags: u32,
    pub VideoSubChannel: u32,
    pub AudioSubChannel: u32,
    pub Channel: u32,
    pub Country: u32,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_FREQUENCY_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_FREQUENCY_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_IF_MEDIUM_S {
    pub Property: KSIDENTIFIER,
    pub IFMedium: KSIDENTIFIER,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_IF_MEDIUM_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_IF_MEDIUM_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_INPUT_S {
    pub Property: KSIDENTIFIER,
    pub InputIndex: u32,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_INPUT_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_INPUT_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_TUNER_MODES = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_TV: KSPROPERTY_TUNER_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_FM_RADIO: KSPROPERTY_TUNER_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_AM_RADIO: KSPROPERTY_TUNER_MODES = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_DSS: KSPROPERTY_TUNER_MODES = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_ATSC: KSPROPERTY_TUNER_MODES = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_MODE_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub Mode: u32,
    pub StandardsSupported: u32,
    pub MinFrequency: u32,
    pub MaxFrequency: u32,
    pub TuningGranularity: u32,
    pub NumberOfInputs: u32,
    pub SettlingTime: u32,
    pub Strategy: u32,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_MODE_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_MODE_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_MODE_S {
    pub Property: KSIDENTIFIER,
    pub Mode: u32,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_MODE_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_MODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub NetworkType: ::windows_sys::core::GUID,
    pub BufferSize: u32,
    pub NetworkTunerCapabilities: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_TUNER_SCAN_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub fSupportsHardwareAssistedScanning: super::super::Foundation::BOOL,
    pub SupportedBroadcastStandards: u32,
    pub GUIDBucket: *mut ::core::ffi::c_void,
    pub lengthofBucket: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_TUNER_SCAN_CAPS_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_TUNER_SCAN_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_SCAN_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub LockStatus: _TunerDecoderLockType,
    pub CurrentFrequency: u32,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_SCAN_STATUS_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_SCAN_STATUS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_TUNER_STANDARD_MODE_S {
    pub Property: KSIDENTIFIER,
    pub AutoDetect: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_TUNER_STANDARD_MODE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_TUNER_STANDARD_MODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_STANDARD_S {
    pub Property: KSIDENTIFIER,
    pub Standard: u32,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_STANDARD_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_STANDARD_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub CurrentFrequency: u32,
    pub PLLOffset: u32,
    pub SignalStrength: u32,
    pub Busy: u32,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_STATUS_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_STATUS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TVAUDIO_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub Capabilities: u32,
    pub InputMedium: KSIDENTIFIER,
    pub OutputMedium: KSIDENTIFIER,
}
impl ::core::marker::Copy for KSPROPERTY_TVAUDIO_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_TVAUDIO_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TVAUDIO_S {
    pub Property: KSIDENTIFIER,
    pub Mode: u32,
}
impl ::core::marker::Copy for KSPROPERTY_TVAUDIO_S {}
impl ::core::clone::Clone for KSPROPERTY_TVAUDIO_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_BASICSUPPORT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_COPYPAYLOAD: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_DEFAULTVALUES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_FSFILTERSCOPE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_GET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_GETPAYLOADSIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_HIGHPRIORITY: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_RELATIONS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_SERIALIZERAW: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_SERIALIZESET: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_SERIALIZESIZE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_SETSUPPORT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_TOPOLOGY: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_UNSERIALIZERAW: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TYPE_UNSERIALIZESET: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VBICAP = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICAP_PROPERTIES_PROTECTION: KSPROPERTY_VBICAP = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VBICODECFILTERING = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_STATISTICS: KSPROPERTY_VBICODECFILTERING = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    pub Property: KSIDENTIFIER,
    pub Substreams: VBICODECFILTERING_CC_SUBSTREAMS,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    pub Property: KSIDENTIFIER,
    pub Substreams: VBICODECFILTERING_NABTS_SUBSTREAMS,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    pub Property: KSIDENTIFIER,
    pub Scanlines: VBICODECFILTERING_SCANLINES,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_CC_PIN,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_CC,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_COMMON,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_NABTS_PIN,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_NABTS,
}
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_CAMERACONTROL = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PAN: KSPROPERTY_VIDCAP_CAMERACONTROL = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_TILT: KSPROPERTY_VIDCAP_CAMERACONTROL = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_ROLL: KSPROPERTY_VIDCAP_CAMERACONTROL = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_ZOOM: KSPROPERTY_VIDCAP_CAMERACONTROL = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE: KSPROPERTY_VIDCAP_CAMERACONTROL = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_IRIS: KSPROPERTY_VIDCAP_CAMERACONTROL = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FOCUS: KSPROPERTY_VIDCAP_CAMERACONTROL = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_SCANMODE: KSPROPERTY_VIDCAP_CAMERACONTROL = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PRIVACY: KSPROPERTY_VIDCAP_CAMERACONTROL = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PANTILT: KSPROPERTY_VIDCAP_CAMERACONTROL = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PAN_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_TILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_ROLL_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_ZOOM_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 13i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 14i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_IRIS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 15i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FOCUS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PANTILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 17i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH: KSPROPERTY_VIDCAP_CAMERACONTROL = 18i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_AUTO_EXPOSURE_PRIORITY: KSPROPERTY_VIDCAP_CAMERACONTROL = 19i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_CROSSBAR = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_CAPS: KSPROPERTY_VIDCAP_CROSSBAR = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_PININFO: KSPROPERTY_VIDCAP_CROSSBAR = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_CAN_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_INPUT_ACTIVE: KSPROPERTY_VIDCAP_CROSSBAR = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_DROPPEDFRAMES = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DROPPEDFRAMES_CURRENT: KSPROPERTY_VIDCAP_DROPPEDFRAMES = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_SELECTOR = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SELECTOR_SOURCE_NODE_ID: KSPROPERTY_VIDCAP_SELECTOR = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SELECTOR_NUM_SOURCES: KSPROPERTY_VIDCAP_SELECTOR = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_TVAUDIO = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TVAUDIO_CAPS: KSPROPERTY_VIDCAP_TVAUDIO = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TVAUDIO_MODE: KSPROPERTY_VIDCAP_TVAUDIO = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TVAUDIO_CURRENTLY_AVAILABLE_MODES: KSPROPERTY_VIDCAP_TVAUDIO = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_GETINFO: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_KEYFRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_PFRAMES_PER_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_QUALITY: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_FRAME_SIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_WINDOWSIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_VIDEOCONTROL = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCONTROL_CAPS: KSPROPERTY_VIDCAP_VIDEOCONTROL = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCONTROL = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCONTROL_FRAME_RATES: KSPROPERTY_VIDCAP_VIDEOCONTROL = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCONTROL_MODE: KSPROPERTY_VIDCAP_VIDEOCONTROL = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_VIDEODECODER = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_CAPS: KSPROPERTY_VIDCAP_VIDEODECODER = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_STANDARD: KSPROPERTY_VIDCAP_VIDEODECODER = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_STATUS: KSPROPERTY_VIDCAP_VIDEODECODER = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_OUTPUT_ENABLE: KSPROPERTY_VIDCAP_VIDEODECODER = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_VCR_TIMING: KSPROPERTY_VIDCAP_VIDEODECODER = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_STATUS2: KSPROPERTY_VIDCAP_VIDEODECODER = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_VIDEOENCODER = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOENCODER_CAPS: KSPROPERTY_VIDCAP_VIDEOENCODER = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOENCODER_STANDARD: KSPROPERTY_VIDCAP_VIDEOENCODER = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOENCODER_COPYPROTECTION: KSPROPERTY_VIDCAP_VIDEOENCODER = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOENCODER_CC_ENABLE: KSPROPERTY_VIDCAP_VIDEOENCODER = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDCAP_VIDEOPROCAMP = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_BRIGHTNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_CONTRAST: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_HUE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_SATURATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_SHARPNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_GAMMA: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_COLORENABLE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_BACKLIGHT_COMPENSATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_GAIN: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER_LIMIT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE_COMPONENT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_POWERLINE_FREQUENCY: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 13i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub DefaultKeyFrameRate: i32,
    pub DefaultPFrameRate: i32,
    pub DefaultQuality: i32,
    pub NumberOfQualitySettings: i32,
    pub Capabilities: i32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOCOMPRESSION_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Value: i32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOCOMPRESSION_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOCOMPRESSION_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOCOMPRESSION_S1 {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Value: i32,
    pub Flags: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOCOMPRESSION_S1 {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::super::Foundation::SIZE,
    pub CurrentActualFrameRate: i64,
    pub CurrentMaxAvailableFrameRate: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOCONTROL_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub VideoControlCaps: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOCONTROL_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOCONTROL_MODE_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Mode: i32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOCONTROL_MODE_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOCONTROL_MODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEODECODER_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub StandardsSupported: u32,
    pub Capabilities: u32,
    pub SettlingTime: u32,
    pub HSyncPerVSync: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEODECODER_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEODECODER_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEODECODER_S {
    pub Property: KSIDENTIFIER,
    pub Value: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEODECODER_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEODECODER_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEODECODER_STATUS2_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfLines: u32,
    pub SignalLocked: u32,
    pub ChromaLock: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEODECODER_STATUS2_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEODECODER_STATUS2_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEODECODER_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfLines: u32,
    pub SignalLocked: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEODECODER_STATUS_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEODECODER_STATUS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOENCODER_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOENCODER_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOENCODER_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_MANUAL: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOPROCAMP_NODE_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    pub NodeProperty: KSP_NODE,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOPROCAMP_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOPROCAMP_S {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOPROCAMP_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_VIDEOPROCAMP_S2 {
    pub Property: KSIDENTIFIER,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl ::core::marker::Copy for KSPROPERTY_VIDEOPROCAMP_S2 {}
impl ::core::clone::Clone for KSPROPERTY_VIDEOPROCAMP_S2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VIDMEM_TRANSPORT = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DISPLAY_ADAPTER_GUID: KSPROPERTY_VIDMEM_TRANSPORT = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PREFERRED_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CURRENT_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MAP_CAPTURE_HANDLE_TO_VRAM_ADDRESS: KSPROPERTY_VIDMEM_TRANSPORT = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_VPCONFIG = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_NUMCONNECTINFO: KSPROPERTY_VPCONFIG = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_GETCONNECTINFO: KSPROPERTY_VPCONFIG = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_SETCONNECTINFO: KSPROPERTY_VPCONFIG = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_VPDATAINFO: KSPROPERTY_VPCONFIG = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_MAXPIXELRATE: KSPROPERTY_VPCONFIG = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_INFORMVPINPUT: KSPROPERTY_VPCONFIG = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_NUMVIDEOFORMAT: KSPROPERTY_VPCONFIG = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_GETVIDEOFORMAT: KSPROPERTY_VPCONFIG = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_SETVIDEOFORMAT: KSPROPERTY_VPCONFIG = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_INVERTPOLARITY: KSPROPERTY_VPCONFIG = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_DECIMATIONCAPABILITY: KSPROPERTY_VPCONFIG = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_SCALEFACTOR: KSPROPERTY_VPCONFIG = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_DDRAWHANDLE: KSPROPERTY_VPCONFIG = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_VIDEOPORTID: KSPROPERTY_VPCONFIG = 13i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_DDRAWSURFACEHANDLE: KSPROPERTY_VPCONFIG = 14i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_SURFACEPARAMS: KSPROPERTY_VPCONFIG = 15i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSPROPERTY_WAVE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_COMPATIBLE_CAPABILITIES: KSPROPERTY_WAVE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_INPUT_CAPABILITIES: KSPROPERTY_WAVE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_OUTPUT_CAPABILITIES: KSPROPERTY_WAVE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_BUFFER: KSPROPERTY_WAVE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_FREQUENCY: KSPROPERTY_WAVE = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_VOLUME: KSPROPERTY_WAVE = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_PAN: KSPROPERTY_WAVE = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_QUEUED_POSITION: u32 = 1u32;
pub const KSPROPSETID_AC3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3215714080, data2: 28191, data3: 4560, data4: [188, 242, 68, 69, 83, 84, 0, 0] };
pub const KSPROPSETID_Audio: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1174383264, data2: 28187, data3: 4560, data4: [188, 242, 68, 69, 83, 84, 0, 0] };
pub const KSPROPSETID_AudioBufferDuration: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1316208767, data2: 9164, data3: 18773, data4: [167, 234, 61, 165, 2, 73, 98, 144] };
pub const KSPROPSETID_AudioDecoderOut: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1822875680, data2: 17341, data3: 4560, data4: [189, 106, 0, 53, 5, 193, 3, 169] };
pub const KSPROPSETID_AudioEngine: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 976192220, data2: 34927, data3: 19370, data4: [158, 180, 8, 43, 144, 37, 197, 54] };
pub const KSPROPSETID_AudioModule: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3224698288, data2: 65397, data3: 18376, data4: [170, 60, 238, 70, 113, 107, 80, 198] };
pub const KSPROPSETID_AudioPosture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2751167245, data2: 18254, data3: 20305, data4: [163, 121, 81, 40, 45, 212, 250, 143] };
pub const KSPROPSETID_AudioResourceManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3501393377, data2: 45772, data3: 18508, data4: [143, 35, 229, 210, 138, 217, 207, 136] };
pub const KSPROPSETID_AudioSignalProcessing: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1332196648, data2: 12489, data3: 16606, data4: [178, 251, 133, 157, 221, 31, 52, 112] };
pub const KSPROPSETID_Bibliographic: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 129635598, data2: 58033, data3: 4560, data4: [172, 23, 0, 160, 201, 34, 49, 150] };
pub const KSPROPSETID_BtAudio: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2141219904, data2: 47350, data3: 19582, data4: [133, 86, 232, 195, 58, 18, 229, 77] };
pub const KSPROPSETID_Clock: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3742540992, data2: 44055, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSPROPSETID_Connection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 492357920, data2: 44187, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSPROPSETID_CopyProt: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 243927616, data2: 27375, data3: 4560, data4: [158, 208, 0, 160, 36, 202, 25, 179] };
pub const KSPROPSETID_Cyclic: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1073655456, data2: 11246, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSPROPSETID_DirectSound3DBuffer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1132147729, data2: 53344, data3: 4560, data4: [133, 131, 0, 192, 79, 217, 186, 243] };
pub const KSPROPSETID_DirectSound3DListener: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1132147732, data2: 53344, data3: 4560, data4: [133, 131, 0, 192, 79, 217, 186, 243] };
pub const KSPROPSETID_DrmAudioStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 791449053, data2: 16792, data3: 20396, data4: [186, 41, 97, 187, 5, 183, 222, 6] };
pub const KSPROPSETID_DvdSubPic: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2889417824, data2: 17327, data3: 4560, data4: [189, 106, 0, 53, 5, 193, 3, 169] };
pub const KSPROPSETID_FMRXControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2491136570, data2: 59630, data3: 18310, data4: [144, 196, 132, 40, 24, 95, 5, 190] };
pub const KSPROPSETID_FMRXTopology: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 205966991, data2: 56365, data3: 16900, data4: [157, 201, 245, 137, 99, 54, 101, 99] };
pub const KSPROPSETID_General: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 342158757, data2: 27279, data3: 4561, data4: [154, 167, 0, 160, 201, 34, 49, 150] };
pub const KSPROPSETID_Hrtf3d: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3060657328, data2: 41091, data3: 4560, data4: [133, 30, 0, 192, 79, 217, 186, 243] };
pub const KSPROPSETID_InterleavedAudio: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3924550992, data2: 54809, data3: 19466, data4: [151, 107, 112, 98, 50, 43, 48, 6] };
pub const KSPROPSETID_Itd3d: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1680470160, data2: 40921, data3: 4560, data4: [167, 91, 0, 160, 201, 3, 101, 227] };
pub const KSPROPSETID_Jack: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1158281047, data2: 11590, data3: 17975, data4: [142, 98, 206, 125, 185, 68, 245, 123] };
pub const KSPROPSETID_MPEG4_MediaType_Attributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4285287418, data2: 1961, data3: 19579, data4: [162, 55, 103, 47, 157, 104, 6, 95] };
pub const KSPROPSETID_MediaSeeking: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4002434828, data2: 53403, data3: 4560, data4: [171, 233, 0, 160, 201, 34, 49, 150] };
pub const KSPROPSETID_MemoryTransport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 171777117, data2: 21059, data3: 18457, data4: [158, 208, 174, 232, 4, 76, 238, 43] };
pub const KSPROPSETID_Mpeg2Vid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3370195808, data2: 3273, data3: 4560, data4: [189, 105, 0, 53, 5, 193, 3, 169] };
pub const KSPROPSETID_OverlayUpdate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1225696719, data2: 30337, data3: 4561, data4: [162, 28, 0, 160, 201, 34, 49, 150] };
pub const KSPROPSETID_Pin: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2350074208, data2: 20909, data3: 4559, data4: [135, 138, 148, 248, 1, 193, 0, 0] };
pub const KSPROPSETID_PinMDLCacheClearProp: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3178334843, data2: 38908, data3: 16583, data4: [136, 206, 211, 255, 6, 245, 91, 22] };
pub const KSPROPSETID_Quality: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3513439104, data2: 44058, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSPROPSETID_RtAudio: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2824184972, data2: 12152, data3: 18217, data4: [144, 81, 25, 104, 116, 107, 158, 239] };
pub const KSPROPSETID_SoundDetector: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 289161822, data2: 64791, data3: 16471, data4: [180, 34, 237, 64, 116, 241, 175, 223] };
pub const KSPROPSETID_SoundDetector2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4261929762, data2: 17676, data3: 19413, data4: [132, 202, 169, 72, 80, 14, 166, 170] };
pub const KSPROPSETID_Stream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1705687648, data2: 39086, data3: 4559, data4: [161, 13, 0, 32, 175, 209, 86, 228] };
pub const KSPROPSETID_StreamAllocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3480109890, data2: 60551, data3: 4559, data4: [161, 48, 0, 32, 175, 209, 86, 228] };
pub const KSPROPSETID_StreamInterface: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 534613729, data2: 40147, data3: 4560, data4: [130, 170, 0, 0, 248, 34, 254, 138] };
pub const KSPROPSETID_TSRateChange: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2768487872, data2: 7453, data3: 4561, data4: [173, 128, 68, 69, 83, 84, 0, 0] };
pub const KSPROPSETID_TelephonyControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3068100273, data2: 53401, data3: 18591, data4: [166, 160, 192, 16, 111, 8, 135, 167] };
pub const KSPROPSETID_TelephonyTopology: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2884787326, data2: 3684, data3: 20018, data4: [177, 144, 208, 246, 215, 197, 62, 151] };
pub const KSPROPSETID_Topology: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1913473728, data2: 30003, data3: 4560, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
pub const KSPROPSETID_TopologyNode: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1174383265, data2: 28187, data3: 4560, data4: [188, 242, 68, 69, 83, 84, 0, 0] };
pub const KSPROPSETID_VBICAP_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4049782279, data2: 31541, data3: 18799, data4: [173, 127, 45, 202, 59, 70, 183, 24] };
pub const KSPROPSETID_VBICodecFiltering: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3405689034, data2: 34581, data3: 4560, data4: [189, 106, 0, 53, 192, 237, 186, 190] };
pub const KSPROPSETID_VPConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3156846176, data2: 12515, data3: 4560, data4: [158, 105, 0, 192, 79, 215, 193, 91] };
pub const KSPROPSETID_VPVBIConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3964836608, data2: 6687, data3: 4561, data4: [186, 217, 0, 96, 151, 68, 17, 26] };
pub const KSPROPSETID_VramCapture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3879709923, data2: 10368, data3: 18690, data4: [183, 153, 136, 208, 205, 99, 78, 15] };
pub const KSPROPSETID_Wave: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2454607024, data2: 25359, data3: 4559, data4: [173, 167, 8, 0, 62, 48, 73, 74] };
pub const KSPROPTYPESETID_General: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2548669344, data2: 48618, data3: 4559, data4: [165, 214, 40, 219, 4, 193, 0, 0] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSP_NODE {
    pub Property: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSP_NODE {}
impl ::core::clone::Clone for KSP_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSP_PIN {
    pub Property: KSIDENTIFIER,
    pub PinId: u32,
    pub Anonymous: KSP_PIN_0,
}
impl ::core::marker::Copy for KSP_PIN {}
impl ::core::clone::Clone for KSP_PIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSP_PIN_0 {
    pub Reserved: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for KSP_PIN_0 {}
impl ::core::clone::Clone for KSP_PIN_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSP_TIMEFORMAT {
    pub Property: KSIDENTIFIER,
    pub SourceFormat: ::windows_sys::core::GUID,
    pub TargetFormat: ::windows_sys::core::GUID,
    pub Time: i64,
}
impl ::core::marker::Copy for KSP_TIMEFORMAT {}
impl ::core::clone::Clone for KSP_TIMEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSQUALITY {
    pub Context: *mut ::core::ffi::c_void,
    pub Proportion: u32,
    pub DeltaTime: i64,
}
impl ::core::marker::Copy for KSQUALITY {}
impl ::core::clone::Clone for KSQUALITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSQUALITY_MANAGER {
    pub QualityManager: super::super::Foundation::HANDLE,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSQUALITY_MANAGER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSQUALITY_MANAGER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSQUERYBUFFER {
    pub Event: KSIDENTIFIER,
    pub EventData: *mut KSEVENTDATA,
    pub Reserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSQUERYBUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSQUERYBUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRATE {
    pub PresentationStart: i64,
    pub Duration: i64,
    pub Interface: KSIDENTIFIER,
    pub Rate: i32,
    pub Flags: u32,
}
impl ::core::marker::Copy for KSRATE {}
impl ::core::clone::Clone for KSRATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRATE_CAPABILITY {
    pub Property: KSIDENTIFIER,
    pub Rate: KSRATE,
}
impl ::core::marker::Copy for KSRATE_CAPABILITY {}
impl ::core::clone::Clone for KSRATE_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRATE_NOPRESENTATIONDURATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRATE_NOPRESENTATIONSTART: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSRELATIVEEVENT {
    pub Size: u32,
    pub Flags: u32,
    pub Anonymous: KSRELATIVEEVENT_0,
    pub Reserved: *mut ::core::ffi::c_void,
    pub Event: KSIDENTIFIER,
    pub EventData: KSEVENTDATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSRELATIVEEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSRELATIVEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KSRELATIVEEVENT_0 {
    pub ObjectHandle: super::super::Foundation::HANDLE,
    pub ObjectPointer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSRELATIVEEVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSRELATIVEEVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRELATIVEEVENT_FLAG_HANDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRELATIVEEVENT_FLAG_POINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSRESET = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRESET_BEGIN: KSRESET = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRESET_END: KSRESET = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRESOLUTION {
    pub Granularity: i64,
    pub Error: i64,
}
impl ::core::marker::Copy for KSRESOLUTION {}
impl ::core::clone::Clone for KSRESOLUTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSRTAUDIO_BUFFER {
    pub BufferAddress: *mut ::core::ffi::c_void,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSRTAUDIO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSRTAUDIO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSRTAUDIO_BUFFER32 {
    pub BufferAddress: u32,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSRTAUDIO_BUFFER32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSRTAUDIO_BUFFER32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_BUFFER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub RequestedBufferSize: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_BUFFER_PROPERTY {}
impl ::core::clone::Clone for KSRTAUDIO_BUFFER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_BUFFER_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
    pub RequestedBufferSize: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_BUFFER_PROPERTY32 {}
impl ::core::clone::Clone for KSRTAUDIO_BUFFER_PROPERTY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub RequestedBufferSize: u32,
    pub NotificationCount: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {}
impl ::core::clone::Clone for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
    pub RequestedBufferSize: u32,
    pub NotificationCount: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {}
impl ::core::clone::Clone for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSRTAUDIO_GETREADPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub PerformanceCounterValue: u64,
    pub MoreData: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSRTAUDIO_GETREADPACKET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSRTAUDIO_GETREADPACKET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_HWLATENCY {
    pub FifoSize: u32,
    pub ChipsetDelay: u32,
    pub CodecDelay: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_HWLATENCY {}
impl ::core::clone::Clone for KSRTAUDIO_HWLATENCY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_HWREGISTER {
    pub Register: *mut ::core::ffi::c_void,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_HWREGISTER {}
impl ::core::clone::Clone for KSRTAUDIO_HWREGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_HWREGISTER32 {
    pub Register: u32,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_HWREGISTER32 {}
impl ::core::clone::Clone for KSRTAUDIO_HWREGISTER32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_HWREGISTER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for KSRTAUDIO_HWREGISTER_PROPERTY {}
impl ::core::clone::Clone for KSRTAUDIO_HWREGISTER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_HWREGISTER_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_HWREGISTER_PROPERTY32 {}
impl ::core::clone::Clone for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub NotificationEvent: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub NotificationEvent: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {}
impl ::core::clone::Clone for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_PACKETVREGISTER {
    pub CompletedPacketCount: *mut u64,
    pub CompletedPacketQPC: *mut u64,
    pub CompletedPacketHash: *mut u64,
}
impl ::core::marker::Copy for KSRTAUDIO_PACKETVREGISTER {}
impl ::core::clone::Clone for KSRTAUDIO_PACKETVREGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for KSRTAUDIO_PACKETVREGISTER_PROPERTY {}
impl ::core::clone::Clone for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSRTAUDIO_SETWRITEPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub EosPacketLength: u32,
}
impl ::core::marker::Copy for KSRTAUDIO_SETWRITEPACKET_INFO {}
impl ::core::clone::Clone for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSSOUNDDETECTORPROPERTY {
    pub Property: KSIDENTIFIER,
    pub EventId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for KSSOUNDDETECTORPROPERTY {}
impl ::core::clone::Clone for KSSOUNDDETECTORPROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KSSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTATE_STOP: KSSTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTATE_ACQUIRE: KSSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTATE_PAUSE: KSSTATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTATE_RUN: KSSTATE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSSTREAMALLOCATOR_STATUS {
    pub Framing: KSALLOCATOR_FRAMING,
    pub AllocatedFrames: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSSTREAMALLOCATOR_STATUS {}
impl ::core::clone::Clone for KSSTREAMALLOCATOR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSSTREAMALLOCATOR_STATUS_EX {
    pub Framing: KSALLOCATOR_FRAMING_EX,
    pub AllocatedFrames: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSSTREAMALLOCATOR_STATUS_EX {}
impl ::core::clone::Clone for KSSTREAMALLOCATOR_STATUS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_FAILUREEXCEPTION: u32 = 8192u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct KSSTREAM_HEADER {
    pub Size: u32,
    pub TypeSpecificFlags: u32,
    pub PresentationTime: KSTIME,
    pub Duration: i64,
    pub FrameExtent: u32,
    pub DataUsed: u32,
    pub Data: *mut ::core::ffi::c_void,
    pub OptionsFlags: u32,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for KSSTREAM_HEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for KSSTREAM_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[cfg(target_arch = "x86")]
pub struct KSSTREAM_HEADER {
    pub Size: u32,
    pub TypeSpecificFlags: u32,
    pub PresentationTime: KSTIME,
    pub Duration: i64,
    pub FrameExtent: u32,
    pub DataUsed: u32,
    pub Data: *mut ::core::ffi::c_void,
    pub OptionsFlags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for KSSTREAM_HEADER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for KSSTREAM_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_BUFFEREDTRANSFER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_DATADISCONTINUITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_DURATIONVALID: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_ENDOFPHOTOSEQUENCE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_ENDOFSTREAM: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_FLUSHONPAUSE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_FRAMEINFO: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_LOOPEDDATA: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_METADATA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_PERSIST_SAMPLE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_PREROLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_SAMPLE_PERSISTED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_SECUREBUFFERTRANSFER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_SPLICEPOINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_TIMEDISCONTINUITY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_TIMEVALID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_TYPECHANGED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_OPTIONSF_VRAM_DATA_TRANSFER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_HEADER_TRACK_COMPLETION_NUMBERS: u32 = 131072u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSSTREAM_METADATA_INFO {
    pub BufferSize: u32,
    pub UsedSize: u32,
    pub Data: *mut ::core::ffi::c_void,
    pub SystemVa: *mut ::core::ffi::c_void,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSSTREAM_METADATA_INFO {}
impl ::core::clone::Clone for KSSTREAM_METADATA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_NONPAGED_DATA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_PAGED_DATA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_READ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_SYNCHRONOUS: u32 = 4096u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSSTREAM_UVC_METADATA {
    pub StartOfFrameTimestamp: KSSTREAM_UVC_METADATATYPE_TIMESTAMP,
    pub EndOfFrameTimestamp: KSSTREAM_UVC_METADATATYPE_TIMESTAMP,
}
impl ::core::marker::Copy for KSSTREAM_UVC_METADATA {}
impl ::core::clone::Clone for KSSTREAM_UVC_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    pub PresentationTimeStamp: u32,
    pub SourceClockReference: u32,
    pub Anonymous: KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0,
    pub Reserved0: u16,
    pub Reserved1: u32,
}
impl ::core::marker::Copy for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {}
impl ::core::clone::Clone for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    pub Anonymous: KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0,
    pub SCRToken: u16,
}
impl ::core::marker::Copy for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {}
impl ::core::clone::Clone for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {}
impl ::core::clone::Clone for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_UVC_SECURE_ATTRIBUTE_SIZE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTREAM_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTRING_Allocator: &str = "{642F5D00-4791-11D0-A5D6-28DB04C10000}";
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTRING_AllocatorEx: &str = "{091BB63B-603F-11D1-B067-00A0C9062802}";
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTRING_Clock: &str = "{53172480-4791-11D0-A5D6-28DB04C10000}";
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTRING_Filter: &str = "{9B365890-165F-11D0-A195-0020AFD156E4}";
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTRING_Pin: &str = "{146F1A80-4791-11D0-A5D6-28DB04C10000}";
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTRING_TopologyNode: &str = "{0621061A-EE75-11D0-B915-00A0C9223196}";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTELEPHONY_CALLCONTROL {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallControlOp: TELEPHONY_CALLCONTROLOP,
}
impl ::core::marker::Copy for KSTELEPHONY_CALLCONTROL {}
impl ::core::clone::Clone for KSTELEPHONY_CALLCONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTELEPHONY_CALLINFO {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallState: TELEPHONY_CALLSTATE,
}
impl ::core::marker::Copy for KSTELEPHONY_CALLINFO {}
impl ::core::clone::Clone for KSTELEPHONY_CALLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTELEPHONY_PROVIDERCHANGE {
    pub CallType: TELEPHONY_CALLTYPE,
    pub ProviderChangeOp: TELEPHONY_PROVIDERCHANGEOP,
}
impl ::core::marker::Copy for KSTELEPHONY_PROVIDERCHANGE {}
impl ::core::clone::Clone for KSTELEPHONY_PROVIDERCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTIME {
    pub Time: i64,
    pub Numerator: u32,
    pub Denominator: u32,
}
impl ::core::marker::Copy for KSTIME {}
impl ::core::clone::Clone for KSTIME {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KSTIME_FORMAT_BYTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2071483761, data2: 35970, data3: 4559, data4: [188, 12, 0, 170, 0, 172, 116, 246] };
pub const KSTIME_FORMAT_FIELD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2071483763, data2: 35970, data3: 4559, data4: [188, 12, 0, 170, 0, 172, 116, 246] };
pub const KSTIME_FORMAT_FRAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2071483760, data2: 35970, data3: 4559, data4: [188, 12, 0, 170, 0, 172, 116, 246] };
pub const KSTIME_FORMAT_MEDIA_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2071483764, data2: 35970, data3: 4559, data4: [188, 12, 0, 170, 0, 172, 116, 246] };
pub const KSTIME_FORMAT_SAMPLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2071483762, data2: 35970, data3: 4559, data4: [188, 12, 0, 170, 0, 172, 116, 246] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTOPOLOGY {
    pub CategoriesCount: u32,
    pub Categories: *const ::windows_sys::core::GUID,
    pub TopologyNodesCount: u32,
    pub TopologyNodes: *const ::windows_sys::core::GUID,
    pub TopologyConnectionsCount: u32,
    pub TopologyConnections: *const KSTOPOLOGY_CONNECTION,
    pub TopologyNodesNames: *const ::windows_sys::core::GUID,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSTOPOLOGY {}
impl ::core::clone::Clone for KSTOPOLOGY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTOPOLOGY_CONNECTION {
    pub FromNode: u32,
    pub FromNodePin: u32,
    pub ToNode: u32,
    pub ToNodePin: u32,
}
impl ::core::marker::Copy for KSTOPOLOGY_CONNECTION {}
impl ::core::clone::Clone for KSTOPOLOGY_CONNECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTOPOLOGY_ENDPOINTID {
    pub TopologyName: [u16; 260],
    pub PinId: u32,
}
impl ::core::marker::Copy for KSTOPOLOGY_ENDPOINTID {}
impl ::core::clone::Clone for KSTOPOLOGY_ENDPOINTID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTOPOLOGY_ENDPOINTIDPAIR {
    pub RenderEndpoint: KSTOPOLOGY_ENDPOINTID,
    pub CaptureEndpoint: KSTOPOLOGY_ENDPOINTID,
}
impl ::core::marker::Copy for KSTOPOLOGY_ENDPOINTIDPAIR {}
impl ::core::clone::Clone for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSVPMAXPIXELRATE {
    pub Size: KS_AMVPSIZE,
    pub MaxPixelsPerSecond: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSVPMAXPIXELRATE {}
impl ::core::clone::Clone for KSVPMAXPIXELRATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSVPSIZE_PROP {
    pub Property: KSIDENTIFIER,
    pub Size: KS_AMVPSIZE,
}
impl ::core::marker::Copy for KSVPSIZE_PROP {}
impl ::core::clone::Clone for KSVPSIZE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSVPSURFACEPARAMS {
    pub dwPitch: u32,
    pub dwXOrigin: u32,
    pub dwYOrigin: u32,
}
impl ::core::marker::Copy for KSVPSURFACEPARAMS {}
impl ::core::clone::Clone for KSVPSURFACEPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KSWAVETABLE_WAVE_DESC {
    pub Identifier: KSIDENTIFIER,
    pub Size: u32,
    pub Looped: super::super::Foundation::BOOL,
    pub LoopPoint: u32,
    pub InROM: super::super::Foundation::BOOL,
    pub Format: KSDATAFORMAT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSWAVETABLE_WAVE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSWAVETABLE_WAVE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSWAVE_BUFFER {
    pub Attributes: u32,
    pub BufferSize: u32,
    pub BufferAddress: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for KSWAVE_BUFFER {}
impl ::core::clone::Clone for KSWAVE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSWAVE_BUFFER_ATTRIBUTEF_LOOPING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSWAVE_BUFFER_ATTRIBUTEF_STATIC: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSWAVE_COMPATCAPS {
    pub ulDeviceType: u32,
}
impl ::core::marker::Copy for KSWAVE_COMPATCAPS {}
impl ::core::clone::Clone for KSWAVE_COMPATCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSWAVE_COMPATCAPS_INPUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSWAVE_COMPATCAPS_OUTPUT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSWAVE_INPUT_CAPABILITIES {
    pub MaximumChannelsPerConnection: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
    pub TotalConnections: u32,
    pub ActiveConnections: u32,
}
impl ::core::marker::Copy for KSWAVE_INPUT_CAPABILITIES {}
impl ::core::clone::Clone for KSWAVE_INPUT_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSWAVE_OUTPUT_CAPABILITIES {
    pub MaximumChannelsPerConnection: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
    pub TotalConnections: u32,
    pub StaticConnections: u32,
    pub StreamingConnections: u32,
    pub ActiveConnections: u32,
    pub ActiveStaticConnections: u32,
    pub ActiveStreamingConnections: u32,
    pub Total3DConnections: u32,
    pub Static3DConnections: u32,
    pub Streaming3DConnections: u32,
    pub Active3DConnections: u32,
    pub ActiveStatic3DConnections: u32,
    pub ActiveStreaming3DConnections: u32,
    pub TotalSampleMemory: u32,
    pub FreeSampleMemory: u32,
    pub LargestFreeContiguousSampleMemory: u32,
}
impl ::core::marker::Copy for KSWAVE_OUTPUT_CAPABILITIES {}
impl ::core::clone::Clone for KSWAVE_OUTPUT_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSWAVE_VOLUME {
    pub LeftAttenuation: i32,
    pub RightAttenuation: i32,
}
impl ::core::marker::Copy for KSWAVE_VOLUME {}
impl ::core::clone::Clone for KSWAVE_VOLUME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMCONTROL_COLORINFO_PRESENT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMCONTROL_PAD_TO_16x9: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMCONTROL_PAD_TO_4x3: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMCONTROL_USED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_AMPixAspectRatio = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PixAspectRatio_NTSC4x3: KS_AMPixAspectRatio = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PixAspectRatio_NTSC16x9: KS_AMPixAspectRatio = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PixAspectRatio_PAL4x3: KS_AMPixAspectRatio = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PixAspectRatio_PAL16x9: KS_AMPixAspectRatio = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_AMVPDATAINFO {
    pub dwSize: u32,
    pub dwMicrosecondsPerField: u32,
    pub amvpDimInfo: KS_AMVPDIMINFO,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub bEnableDoubleClock: super::super::Foundation::BOOL,
    pub bEnableVACT: super::super::Foundation::BOOL,
    pub bDataIsInterlaced: super::super::Foundation::BOOL,
    pub lHalfLinesOdd: i32,
    pub bFieldPolarityInverted: super::super::Foundation::BOOL,
    pub dwNumLinesInVREF: u32,
    pub lHalfLinesEven: i32,
    pub dwReserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_AMVPDATAINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_AMVPDATAINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_AMVPDIMINFO {
    pub dwFieldWidth: u32,
    pub dwFieldHeight: u32,
    pub dwVBIWidth: u32,
    pub dwVBIHeight: u32,
    pub rcValidRegion: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_AMVPDIMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_AMVPDIMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_AMVPSIZE {
    pub dwWidth: u32,
    pub dwHeight: u32,
}
impl ::core::marker::Copy for KS_AMVPSIZE {}
impl ::core::clone::Clone for KS_AMVPSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_AMVP_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_WEAVE: KS_AMVP_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_BOBINTERLEAVED: KS_AMVP_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_BOBNONINTERLEAVED: KS_AMVP_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_SKIPEVEN: KS_AMVP_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_SKIPODD: KS_AMVP_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_AMVP_SELECTFORMATBY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_DO_NOT_CARE: KS_AMVP_SELECTFORMATBY = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_BEST_BANDWIDTH: KS_AMVP_SELECTFORMATBY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_INPUT_SAME_AS_OUTPUT: KS_AMVP_SELECTFORMATBY = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_AM_ExactRateChange {
    pub OutputZeroTime: i64,
    pub Rate: i32,
}
impl ::core::marker::Copy for KS_AM_ExactRateChange {}
impl ::core::clone::Clone for KS_AM_ExactRateChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_AM_PROPERTY_TS_RATE_CHANGE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_RATE_SimpleRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_RATE_ExactRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_RATE_MaxFullDataRate: KS_AM_PROPERTY_TS_RATE_CHANGE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_RATE_Step: KS_AM_PROPERTY_TS_RATE_CHANGE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_AM_SimpleRateChange {
    pub StartTime: i64,
    pub Rate: i32,
}
impl ::core::marker::Copy for KS_AM_SimpleRateChange {}
impl ::core::clone::Clone for KS_AM_SimpleRateChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_UseNewCSSKey: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_AnalogVideoInfo {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_AnalogVideoInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_AnalogVideoInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_AnalogVideoStandard = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_None: KS_AnalogVideoStandard = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_NTSC_M: KS_AnalogVideoStandard = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_NTSC_M_J: KS_AnalogVideoStandard = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_NTSC_433: KS_AnalogVideoStandard = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_B: KS_AnalogVideoStandard = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_D: KS_AnalogVideoStandard = 32i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_G: KS_AnalogVideoStandard = 64i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_H: KS_AnalogVideoStandard = 128i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_I: KS_AnalogVideoStandard = 256i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_M: KS_AnalogVideoStandard = 512i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_N: KS_AnalogVideoStandard = 1024i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_60: KS_AnalogVideoStandard = 2048i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_B: KS_AnalogVideoStandard = 4096i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_D: KS_AnalogVideoStandard = 8192i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_G: KS_AnalogVideoStandard = 16384i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_H: KS_AnalogVideoStandard = 32768i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_K: KS_AnalogVideoStandard = 65536i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_K1: KS_AnalogVideoStandard = 131072i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_L: KS_AnalogVideoStandard = 262144i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_L1: KS_AnalogVideoStandard = 524288i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_N_COMBO: KS_AnalogVideoStandard = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_NTSC_Mask: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_Mask: u32 = 1052656u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_Mask: u32 = 1044480u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
impl ::core::marker::Copy for KS_BITMAPINFOHEADER {}
impl ::core::clone::Clone for KS_BITMAPINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_BI_BITFIELDS: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_BI_JPEG: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_BI_RGB: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_BI_RLE4: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_BI_RLE8: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_EVEN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_FIELD1_MASK: i32 = 240i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_FIELD2_MASK: i32 = 7936i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_ODD: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_CC1: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_CC2: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_CC3: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_CC4: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_T1: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_T2: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_T3: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_T4: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CC_SUBSTREAM_SERVICE_XDS: i32 = 4096i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_COLCON {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
}
impl ::core::marker::Copy for KS_COLCON {}
impl ::core::clone::Clone for KS_COLCON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_COMPRESSION {
    pub RatioNumerator: u32,
    pub RatioDenominator: u32,
    pub RatioConstantMargin: u32,
}
impl ::core::marker::Copy for KS_COMPRESSION {}
impl ::core::clone::Clone for KS_COMPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_COPYPROTECT_RestrictDuplication: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_COPY_MACROVISION {
    pub MACROVISIONLevel: u32,
}
impl ::core::marker::Copy for KS_COPY_MACROVISION {}
impl ::core::clone::Clone for KS_COPY_MACROVISION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_COPY_MACROVISION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MACROVISION_DISABLED: KS_COPY_MACROVISION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MACROVISION_LEVEL1: KS_COPY_MACROVISION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MACROVISION_LEVEL2: KS_COPY_MACROVISION_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MACROVISION_LEVEL3: KS_COPY_MACROVISION_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_CameraControlAsyncOperation = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAMERACONTROL_ASYNC_START: KS_CameraControlAsyncOperation = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAMERACONTROL_ASYNC_STOP: KS_CameraControlAsyncOperation = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAMERACONTROL_ASYNC_RESET: KS_CameraControlAsyncOperation = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_CompressionCaps = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanQuality: KS_CompressionCaps = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanCrunch: KS_CompressionCaps = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanKeyFrame: KS_CompressionCaps = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanBFrame: KS_CompressionCaps = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanWindow: KS_CompressionCaps = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DATAFORMAT_H264VIDEOINFO {
    pub DataFormat: KSDATAFORMAT,
    pub H264VideoInfoHeader: KS_H264VIDEOINFO,
}
impl ::core::marker::Copy for KS_DATAFORMAT_H264VIDEOINFO {}
impl ::core::clone::Clone for KS_DATAFORMAT_H264VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DATAFORMAT_IMAGEINFO {
    pub DataFormat: KSDATAFORMAT,
    pub ImageInfoHeader: KS_BITMAPINFOHEADER,
}
impl ::core::marker::Copy for KS_DATAFORMAT_IMAGEINFO {}
impl ::core::clone::Clone for KS_DATAFORMAT_IMAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATAFORMAT_MPEGVIDEOINFO2 {
    pub DataFormat: KSDATAFORMAT,
    pub MpegVideoInfoHeader2: KS_MPEGVIDEOINFO2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATAFORMAT_MPEGVIDEOINFO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DATAFORMAT_VBIINFOHEADER {
    pub DataFormat: KSDATAFORMAT,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
impl ::core::marker::Copy for KS_DATAFORMAT_VBIINFOHEADER {}
impl ::core::clone::Clone for KS_DATAFORMAT_VBIINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATAFORMAT_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATAFORMAT_VIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER2 {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfoHeader2: KS_VIDEOINFOHEADER2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATAFORMAT_VIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATAFORMAT_VIDEOINFO_PALETTE {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfo: KS_VIDEOINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATAFORMAT_VIDEOINFO_PALETTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_ANALOGVIDEO {
    pub DataRange: KSDATAFORMAT,
    pub AnalogVideoInfo: KS_AnalogVideoInfo,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_ANALOGVIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_ANALOGVIDEO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_H264_VIDEO {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_H264VIDEOINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_H264_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_H264_VIDEO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_IMAGE {
    pub DataRange: KSDATAFORMAT,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub ImageInfoHeader: KS_BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_IMAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_IMAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_MPEG1_VIDEO {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_MPEG1VIDEOINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_MPEG1_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_MPEG1_VIDEO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_MPEG2_VIDEO {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_MPEGVIDEOINFO2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_MPEG2_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_MPEG2_VIDEO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_VIDEO {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_VIDEO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_VIDEO2 {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_VIDEO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_VIDEO2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_VIDEO_PALETTE {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfo: KS_VIDEOINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_VIDEO_PALETTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_VIDEO_PALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_VIDEO_VBI {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_DATARANGE_VIDEO_VBI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_DATARANGE_VIDEO_VBI {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_DVDCOPYSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_INITIALIZE: KS_DVDCOPYSTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_INITIALIZE_TITLE: KS_DVDCOPYSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_AUTHENTICATION_NOT_REQUIRED: KS_DVDCOPYSTATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_AUTHENTICATION_REQUIRED: KS_DVDCOPYSTATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_DONE: KS_DVDCOPYSTATE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DVDCOPY_BUSKEY {
    pub BusKey: [u8; 5],
    pub Reserved: [u8; 1],
}
impl ::core::marker::Copy for KS_DVDCOPY_BUSKEY {}
impl ::core::clone::Clone for KS_DVDCOPY_BUSKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DVDCOPY_CHLGKEY {
    pub ChlgKey: [u8; 10],
    pub Reserved: [u8; 2],
}
impl ::core::marker::Copy for KS_DVDCOPY_CHLGKEY {}
impl ::core::clone::Clone for KS_DVDCOPY_CHLGKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DVDCOPY_DISCKEY {
    pub DiscKey: [u8; 2048],
}
impl ::core::marker::Copy for KS_DVDCOPY_DISCKEY {}
impl ::core::clone::Clone for KS_DVDCOPY_DISCKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DVDCOPY_REGION {
    pub Reserved: u8,
    pub RegionData: u8,
    pub Reserved2: [u8; 2],
}
impl ::core::marker::Copy for KS_DVDCOPY_REGION {}
impl ::core::clone::Clone for KS_DVDCOPY_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DVDCOPY_SET_COPY_STATE {
    pub DVDCopyState: u32,
}
impl ::core::marker::Copy for KS_DVDCOPY_SET_COPY_STATE {}
impl ::core::clone::Clone for KS_DVDCOPY_SET_COPY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DVDCOPY_TITLEKEY {
    pub KeyFlags: u32,
    pub ReservedNT: [u32; 2],
    pub TitleKey: [u8; 6],
    pub Reserved: [u8; 2],
}
impl ::core::marker::Copy for KS_DVDCOPY_TITLEKEY {}
impl ::core::clone::Clone for KS_DVDCOPY_TITLEKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_CGMS_COPY_ONCE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_CGMS_COPY_PERMITTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_CGMS_COPY_PROTECT_MASK: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_CGMS_NO_COPY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_CGMS_RESERVED_MASK: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_COPYRIGHTED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_COPYRIGHT_MASK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_NOT_COPYRIGHTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_SECTOR_NOT_PROTECTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_SECTOR_PROTECTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVD_SECTOR_PROTECT_MASK: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DVD_YCrCb {
    pub Reserved: u8,
    pub Y: u8,
    pub Cr: u8,
    pub Cb: u8,
}
impl ::core::marker::Copy for KS_DVD_YCrCb {}
impl ::core::clone::Clone for KS_DVD_YCrCb {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_DVD_YUV {
    pub Reserved: u8,
    pub Y: u8,
    pub V: u8,
    pub U: u8,
}
impl ::core::marker::Copy for KS_DVD_YUV {}
impl ::core::clone::Clone for KS_DVD_YUV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub hDirectDraw: super::super::Foundation::HANDLE,
    pub hSurfaceHandle: super::super::Foundation::HANDLE,
    pub DirectDrawRect: super::super::Foundation::RECT,
    pub Anonymous1: KS_FRAME_INFO_0,
    pub Reserved2: u32,
    pub Anonymous2: KS_FRAME_INFO_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_FRAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KS_FRAME_INFO_0 {
    pub lSurfacePitch: i32,
    pub Reserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_FRAME_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_FRAME_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KS_FRAME_INFO_1 {
    pub Anonymous: KS_FRAME_INFO_1_0,
    pub FrameCompletionNumber: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_FRAME_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_FRAME_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_FRAME_INFO_1_0 {
    pub Reserved3: u32,
    pub Reserved4: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_FRAME_INFO_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_FRAME_INFO_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_FRAMING_ITEM {
    pub MemoryType: ::windows_sys::core::GUID,
    pub BusType: ::windows_sys::core::GUID,
    pub MemoryFlags: u32,
    pub BusFlags: u32,
    pub Flags: u32,
    pub Frames: u32,
    pub Anonymous: KS_FRAMING_ITEM_0,
    pub MemoryTypeWeight: u32,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub FramingRange: KS_FRAMING_RANGE_WEIGHTED,
}
impl ::core::marker::Copy for KS_FRAMING_ITEM {}
impl ::core::clone::Clone for KS_FRAMING_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union KS_FRAMING_ITEM_0 {
    pub FileAlignment: u32,
    pub FramePitch: i32,
}
impl ::core::marker::Copy for KS_FRAMING_ITEM_0 {}
impl ::core::clone::Clone for KS_FRAMING_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_FRAMING_RANGE {
    pub MinFrameSize: u32,
    pub MaxFrameSize: u32,
    pub Stepping: u32,
}
impl ::core::marker::Copy for KS_FRAMING_RANGE {}
impl ::core::clone::Clone for KS_FRAMING_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_FRAMING_RANGE_WEIGHTED {
    pub Range: KS_FRAMING_RANGE,
    pub InPlaceWeight: u32,
    pub NotInPlaceWeight: u32,
}
impl ::core::marker::Copy for KS_FRAMING_RANGE_WEIGHTED {}
impl ::core::clone::Clone for KS_FRAMING_RANGE_WEIGHTED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_H264VIDEOINFO {
    pub wWidth: u16,
    pub wHeight: u16,
    pub wSARwidth: u16,
    pub wSARheight: u16,
    pub wProfile: u16,
    pub bLevelIDC: u8,
    pub wConstrainedToolset: u16,
    pub bmSupportedUsages: u32,
    pub bmCapabilities: u16,
    pub bmSVCCapabilities: u32,
    pub bmMVCCapabilities: u32,
    pub dwFrameInterval: u32,
    pub bMaxCodecConfigDelay: u8,
    pub bmSupportedSliceModes: u8,
    pub bmSupportedSyncFrameTypes: u8,
    pub bResolutionScaling: u8,
    pub bSimulcastSupport: u8,
    pub bmSupportedRateControlModes: u8,
    pub wMaxMBperSecOneResolutionNoScalability: u16,
    pub wMaxMBperSecTwoResolutionsNoScalability: u16,
    pub wMaxMBperSecThreeResolutionsNoScalability: u16,
    pub wMaxMBperSecFourResolutionsNoScalability: u16,
    pub wMaxMBperSecOneResolutionTemporalScalability: u16,
    pub wMaxMBperSecTwoResolutionsTemporalScalablility: u16,
    pub wMaxMBperSecThreeResolutionsTemporalScalability: u16,
    pub wMaxMBperSecFourResolutionsTemporalScalability: u16,
    pub wMaxMBperSecOneResolutionTemporalQualityScalability: u16,
    pub wMaxMBperSecTwoResolutionsTemporalQualityScalability: u16,
    pub wMaxMBperSecThreeResolutionsTemporalQualityScalablity: u16,
    pub wMaxMBperSecFourResolutionsTemporalQualityScalability: u16,
    pub wMaxMBperSecOneResolutionTemporalSpatialScalability: u16,
    pub wMaxMBperSecTwoResolutionsTemporalSpatialScalability: u16,
    pub wMaxMBperSecThreeResolutionsTemporalSpatialScalablity: u16,
    pub wMaxMBperSecFourResolutionsTemporalSpatialScalability: u16,
    pub wMaxMBperSecOneResolutionFullScalability: u16,
    pub wMaxMBperSecTwoResolutionsFullScalability: u16,
    pub wMaxMBperSecThreeResolutionsFullScalability: u16,
    pub wMaxMBperSecFourResolutionsFullScalability: u16,
}
impl ::core::marker::Copy for KS_H264VIDEOINFO {}
impl ::core::clone::Clone for KS_H264VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_1FieldPerSample: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_DisplayModeBobOnly: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_DisplayModeBobOrWeave: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_DisplayModeMask: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_DisplayModeWeaveOnly: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_Field1First: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_FieldPatBothIrregular: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_FieldPatBothRegular: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_FieldPatField1Only: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_FieldPatField2Only: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_FieldPatternMask: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_IsInterlaced: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_INTERLACE_UNUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_LogicalMemoryType = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeDontCare: KS_LogicalMemoryType = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeKernelPaged: KS_LogicalMemoryType = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeKernelNonPaged: KS_LogicalMemoryType = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeDeviceHostMapped: KS_LogicalMemoryType = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeDeviceSpecific: KS_LogicalMemoryType = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeUser: KS_LogicalMemoryType = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeAnyHost: KS_LogicalMemoryType = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_MPEAUDIOINFO {
    pub dwFlags: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
}
impl ::core::marker::Copy for KS_MPEAUDIOINFO {}
impl ::core::clone::Clone for KS_MPEAUDIOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_MPEG1VIDEOINFO {
    pub hdr: KS_VIDEOINFOHEADER,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub bSequenceHeader: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_MPEG1VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_MPEG1VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_MPEG2Level = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Level_Low: KS_MPEG2Level = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Level_Main: KS_MPEG2Level = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Level_High1440: KS_MPEG2Level = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Level_High: KS_MPEG2Level = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_MPEG2Profile = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_Simple: KS_MPEG2Profile = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_Main: KS_MPEG2Profile = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_SNRScalable: KS_MPEG2Profile = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_SpatiallyScalable: KS_MPEG2Profile = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_High: KS_MPEG2Profile = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_27MhzTimebase: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_DSS_UserData: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_DVB_UserData: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_DVDLine21Field1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_DVDLine21Field2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_DoPanScan: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_FilmCameraMode: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_LetterboxAnalogOut: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_SourceIsLetterboxed: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2_WidescreenAnalogOut: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEGAUDIOINFO_27MhzTimebase: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_MPEGVIDEOINFO2 {
    pub hdr: KS_VIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub bSequenceHeader: [u32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_MPEGVIDEOINFO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_MPEGVIDEOINFO2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_ADVERTISER_BASE: u32 = 2224u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_CONTENT_BASE: u32 = 2208u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_MICROSOFT_RESERVED_TEST_DATA_BASE: u32 = 2288u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_NETWORK_WIDE_ADVERTISER_BASE: u32 = 2160u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_NETWORK_WIDE_CONTENT_BASE: u32 = 2144u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_ADVERTISER_BASE: u32 = 2064u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_BASE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_ADVERTISER_BASE: u32 = 2096u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_CONTENT_BASE: u32 = 2080u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_ADVERTISER_BASE: u32 = 2128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_CONTENT_BASE: u32 = 2112u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_TELEVISION_STATION_ADVERTISER_BASE: u32 = 2192u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_NABTS_GROUPID_TELEVISION_STATION_CONTENT_BASE: u32 = 2176u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_PhysicalConnectorType = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_Tuner: KS_PhysicalConnectorType = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_Composite: KS_PhysicalConnectorType = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_SVideo: KS_PhysicalConnectorType = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_RGB: KS_PhysicalConnectorType = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_YRYBY: KS_PhysicalConnectorType = 5i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_SerialDigital: KS_PhysicalConnectorType = 6i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_ParallelDigital: KS_PhysicalConnectorType = 7i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_SCSI: KS_PhysicalConnectorType = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_AUX: KS_PhysicalConnectorType = 9i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_1394: KS_PhysicalConnectorType = 10i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_USB: KS_PhysicalConnectorType = 11i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_VideoDecoder: KS_PhysicalConnectorType = 12i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_VideoEncoder: KS_PhysicalConnectorType = 13i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_SCART: KS_PhysicalConnectorType = 14i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_Tuner: KS_PhysicalConnectorType = 4096i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_Line: KS_PhysicalConnectorType = 4097i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_Mic: KS_PhysicalConnectorType = 4098i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_AESDigital: KS_PhysicalConnectorType = 4099i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_SPDIFDigital: KS_PhysicalConnectorType = 4100i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_SCSI: KS_PhysicalConnectorType = 4101i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_AUX: KS_PhysicalConnectorType = 4102i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_1394: KS_PhysicalConnectorType = 4103i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_USB: KS_PhysicalConnectorType = 4104i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_AudioDecoder: KS_PhysicalConnectorType = 4105i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
impl ::core::marker::Copy for KS_RGBQUAD {}
impl ::core::clone::Clone for KS_RGBQUAD {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KS_SECURE_CAMERA_SCENARIO_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2924739694, data2: 36233, data3: 17544, data4: [157, 46, 77, 0, 135, 49, 197, 253] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_SEEKING_CAPABILITIES = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanSeekAbsolute: KS_SEEKING_CAPABILITIES = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanSeekForwards: KS_SEEKING_CAPABILITIES = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanSeekBackwards: KS_SEEKING_CAPABILITIES = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanGetCurrentPos: KS_SEEKING_CAPABILITIES = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanGetStopPos: KS_SEEKING_CAPABILITIES = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanGetDuration: KS_SEEKING_CAPABILITIES = 32i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanPlayBackwards: KS_SEEKING_CAPABILITIES = 64i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_SEEKING_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_NoPositioning: KS_SEEKING_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_AbsolutePositioning: KS_SEEKING_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_RelativePositioning: KS_SEEKING_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_IncrementalPositioning: KS_SEEKING_FLAGS = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_PositioningBitsMask: KS_SEEKING_FLAGS = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_SeekToKeyFrame: KS_SEEKING_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_ReturnTime: KS_SEEKING_FLAGS = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_TRUECOLORINFO {
    pub dwBitMasks: [u32; 3],
    pub bmiColors: [KS_RGBQUAD; 256],
}
impl ::core::marker::Copy for KS_TRUECOLORINFO {}
impl ::core::clone::Clone for KS_TRUECOLORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_TUNER_STRATEGY = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_STRATEGY_PLL: KS_TUNER_STRATEGY = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_STRATEGY_SIGNAL_STRENGTH: KS_TUNER_STRATEGY = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_STRATEGY_DRIVER_TUNES: KS_TUNER_STRATEGY = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_TUNER_TUNING_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_TUNING_EXACT: KS_TUNER_TUNING_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_TUNING_FINE: KS_TUNER_TUNING_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_TUNING_COARSE: KS_TUNER_TUNING_FLAGS = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_MODE_LANG_A: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_MODE_LANG_B: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_MODE_LANG_C: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_MODE_MONO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_MODE_STEREO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_PRESET_LANG_A: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_PRESET_LANG_B: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_PRESET_LANG_C: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVAUDIO_PRESET_STEREO: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TVTUNER_CHANGE_END_TUNE: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_TVTUNER_CHANGE_INFO {
    pub dwFlags: u32,
    pub dwCountryCode: u32,
    pub dwAnalogVideoStandard: u32,
    pub dwChannel: u32,
}
impl ::core::marker::Copy for KS_TVTUNER_CHANGE_INFO {}
impl ::core::clone::Clone for KS_TVTUNER_CHANGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBICAP_PROTECTION_MV_DETECTED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBICAP_PROTECTION_MV_HARDWARE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBICAP_PROTECTION_MV_PRESENT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBIDATARATE_CC: i32 = 503493i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBIDATARATE_NABTS: i32 = 5727272i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_VBIINFOHEADER {
    pub StartLine: u32,
    pub EndLine: u32,
    pub SamplingFrequency: u32,
    pub MinLineStartTime: u32,
    pub MaxLineStartTime: u32,
    pub ActualLineStartTime: u32,
    pub ActualLineEndTime: u32,
    pub VideoStandard: u32,
    pub SamplesPerLine: u32,
    pub StrideInBytes: u32,
    pub BufferSize: u32,
}
impl ::core::marker::Copy for KS_VBIINFOHEADER {}
impl ::core::clone::Clone for KS_VBIINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBI_FLAG_FIELD1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBI_FLAG_FIELD2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBI_FLAG_FRAME: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBI_FLAG_MV_DETECTED: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBI_FLAG_MV_HARDWARE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBI_FLAG_MV_PRESENT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBI_FLAG_TVTUNER_CHANGE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VBI_FLAG_VBIINFOHEADER_CHANGE: i32 = 32i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_VBI_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub dwSamplingFrequency: u32,
    pub TvTunerChangeInfo: KS_TVTUNER_CHANGE_INFO,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
impl ::core::marker::Copy for KS_VBI_FRAME_INFO {}
impl ::core::clone::Clone for KS_VBI_FRAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_VIDEODECODER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEODECODER_FLAGS_CAN_DISABLE_OUTPUT: KS_VIDEODECODER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEODECODER_FLAGS_CAN_USE_VCR_LOCKING: KS_VIDEODECODER_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEODECODER_FLAGS_CAN_INDICATE_LOCKED: KS_VIDEODECODER_FLAGS = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_VIDEOINFO {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: KS_BITMAPINFOHEADER,
    pub Anonymous: KS_VIDEOINFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KS_VIDEOINFO_0 {
    pub bmiColors: [KS_RGBQUAD; 256],
    pub dwBitMasks: [u32; 3],
    pub TrueColorInfo: KS_TRUECOLORINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_VIDEOINFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_VIDEOINFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_VIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: KS_BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_VIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_VIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub Anonymous: KS_VIDEOINFOHEADER2_0,
    pub dwReserved2: u32,
    pub bmiHeader: KS_BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_VIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_VIDEOINFOHEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KS_VIDEOINFOHEADER2_0 {
    pub dwControlFlags: u32,
    pub dwReserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_VIDEOINFOHEADER2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_VIDEOINFOHEADER2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_CAPTURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_CC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_EDS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_IS_VPE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_NABTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_PREVIEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_STILL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_TELETEXT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEOSTREAM_VBI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_ALLOC_VPE_AGP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_ALLOC_VPE_DISPLAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_ALLOC_VPE_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_B_FRAME: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_FIELD1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_FIELD1FIRST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_FIELD2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_FIELD_MASK: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_FRAME: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_IPB_MASK: i32 = 48i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_I_FRAME: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_P_FRAME: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_REPEAT_FIELD: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEO_FLAG_WEAVE: i32 = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_VIDEO_STREAM_CONFIG_CAPS {
    pub guid: ::windows_sys::core::GUID,
    pub VideoStandard: u32,
    pub InputSize: super::super::Foundation::SIZE,
    pub MinCroppingSize: super::super::Foundation::SIZE,
    pub MaxCroppingSize: super::super::Foundation::SIZE,
    pub CropGranularityX: i32,
    pub CropGranularityY: i32,
    pub CropAlignX: i32,
    pub CropAlignY: i32,
    pub MinOutputSize: super::super::Foundation::SIZE,
    pub MaxOutputSize: super::super::Foundation::SIZE,
    pub OutputGranularityX: i32,
    pub OutputGranularityY: i32,
    pub StretchTapsX: i32,
    pub StretchTapsY: i32,
    pub ShrinkTapsX: i32,
    pub ShrinkTapsY: i32,
    pub MinFrameInterval: i64,
    pub MaxFrameInterval: i64,
    pub MinBitsPerSecond: i32,
    pub MaxBitsPerSecond: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KS_VIDEO_STREAM_CONFIG_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_VideoControlFlags = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_FlipHorizontal: KS_VideoControlFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_FlipVertical: KS_VideoControlFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_Obsolete_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = 16i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_Obsolete_VideoControlFlag_Trigger: KS_VideoControlFlags = 32i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = 4i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_Trigger: KS_VideoControlFlags = 8i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_IndependentImagePin: KS_VideoControlFlags = 64i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_StillCapturePreviewFrame: KS_VideoControlFlags = 128i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_StartPhotoSequenceCapture: KS_VideoControlFlags = 256i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_StopPhotoSequenceCapture: KS_VideoControlFlags = 512i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type KS_VideoStreamingHints = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_FrameInterval: KS_VideoStreamingHints = 256i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_KeyFrameRate: KS_VideoStreamingHints = 512i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_PFrameRate: KS_VideoStreamingHints = 1024i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_CompQuality: KS_VideoStreamingHints = 2048i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_CompWindowSize: KS_VideoStreamingHints = 4096i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iBLUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iEGA_COLORS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iGREEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iMASK_COLORS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iMAXBITS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iPALETTE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iPALETTE_COLORS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iRED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_iTRUECOLOR: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOOPEDSTREAMING_POSITION_EVENT_DATA {
    pub KsEventData: KSEVENTDATA,
    pub Position: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOOPEDSTREAMING_POSITION_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MAX_NABTS_VBI_LINES_PER_FIELD: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MAX_RESOURCEGROUPID_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MAX_SINK_DESCRIPTION_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MAX_WST_VBI_LINES_PER_FIELD: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MEDIUM_INFO {
    pub MediaPresent: super::super::Foundation::BOOL,
    pub MediaType: u32,
    pub RecordInhibit: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MEDIUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MEDIUM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union MF_MDL_SHARED_PAYLOAD_KEY {
    pub combined: MF_MDL_SHARED_PAYLOAD_KEY_0,
    pub GMDLHandle: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for MF_MDL_SHARED_PAYLOAD_KEY {}
impl ::core::clone::Clone for MF_MDL_SHARED_PAYLOAD_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct MF_MDL_SHARED_PAYLOAD_KEY_0 {
    pub pHandle: u32,
    pub fHandle: u32,
    pub uPayload: u64,
}
impl ::core::marker::Copy for MF_MDL_SHARED_PAYLOAD_KEY_0 {}
impl ::core::clone::Clone for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MIN_DEV_VER_FOR_FLAGS: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MIN_DEV_VER_FOR_QI: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct NABTSFEC_BUFFER {
    pub dataSize: u32,
    pub groupID: u16,
    pub Reserved: u16,
    pub data: [u8; 448],
}
impl ::core::marker::Copy for NABTSFEC_BUFFER {}
impl ::core::clone::Clone for NABTSFEC_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct NABTS_BUFFER {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub PictureNumber: i64,
    pub NabtsLines: [NABTS_BUFFER_LINE; 11],
}
impl ::core::marker::Copy for NABTS_BUFFER {}
impl ::core::clone::Clone for NABTS_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct NABTS_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 36],
}
impl ::core::marker::Copy for NABTS_BUFFER_LINE {}
impl ::core::clone::Clone for NABTS_BUFFER_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const NABTS_BUFFER_PICTURENUMBER_SUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const NABTS_BYTES_PER_LINE: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const NABTS_LINES_PER_BUNDLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const NABTS_PAYLOAD_PER_LINE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const NANOSECONDS: u32 = 10000000u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct OPTIMAL_WEIGHT_TOTALS {
    pub MinTotalNominator: i64,
    pub MaxTotalNominator: i64,
    pub TotalDenominator: i64,
}
impl ::core::marker::Copy for OPTIMAL_WEIGHT_TOTALS {}
impl ::core::clone::Clone for OPTIMAL_WEIGHT_TOTALS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PINNAME_DISPLAYPORT_OUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 570143529, data2: 6730, data3: 18650, data4: [160, 118, 35, 24, 163, 197, 155, 38] };
pub const PINNAME_HDMI_OUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 947649539, data2: 59375, data3: 18689, data4: [134, 224, 53, 183, 195, 43, 0, 239] };
pub const PINNAME_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 950062488, data2: 54427, data3: 19688, data4: [180, 138, 52, 70, 103, 161, 120, 48] };
pub const PINNAME_SPDIF_IN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 366776357, data2: 8877, data3: 16819, data4: [136, 117, 244, 206, 176, 41, 158, 32] };
pub const PINNAME_SPDIF_OUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 975586433, data2: 58668, data3: 19330, data4: [142, 122, 200, 226, 249, 29, 195, 128] };
pub const PINNAME_VIDEO_ANALOGVIDEOIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176131, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176129, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_CC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176137, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_CC_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 447578209, data2: 301, data3: 4562, data4: [180, 177, 0, 160, 209, 2, 207, 190] };
pub const PINNAME_VIDEO_EDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176135, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_NABTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176134, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_NABTS_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 695219808, data2: 18826, data3: 4562, data4: [180, 177, 0, 160, 209, 2, 207, 190] };
pub const PINNAME_VIDEO_PREVIEW: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176130, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_STILL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176138, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_TELETEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176136, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_TIMECODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176139, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_VBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176132, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_VIDEOPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176133, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
pub const PINNAME_VIDEO_VIDEOPORT_VBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218176140, data2: 851, data3: 4561, data4: [144, 95, 0, 0, 192, 204, 22, 186] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type PIPE_ALLOCATOR_PLACE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Pipe_Allocator_None: PIPE_ALLOCATOR_PLACE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Pipe_Allocator_FirstPin: PIPE_ALLOCATOR_PLACE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Pipe_Allocator_LastPin: PIPE_ALLOCATOR_PLACE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Pipe_Allocator_MiddlePin: PIPE_ALLOCATOR_PLACE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct PIPE_DIMENSIONS {
    pub AllocatorPin: KS_COMPRESSION,
    pub MaxExpansionPin: KS_COMPRESSION,
    pub EndPin: KS_COMPRESSION,
}
impl ::core::marker::Copy for PIPE_DIMENSIONS {}
impl ::core::clone::Clone for PIPE_DIMENSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type PIPE_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_DontCare: PIPE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_RangeNotFixed: PIPE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_RangeFixed: PIPE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_CompressionUnknown: PIPE_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_Finalized: PIPE_STATE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct PIPE_TERMINATION {
    pub Flags: u32,
    pub OutsideFactors: u32,
    pub Weigth: u32,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub OptimalRange: KS_FRAMING_RANGE_WEIGHTED,
    pub Compression: KS_COMPRESSION,
}
impl ::core::marker::Copy for PIPE_TERMINATION {}
impl ::core::clone::Clone for PIPE_TERMINATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROPSETID_ALLOCATOR_CONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1394022752, data2: 5262, data3: 4562, data4: [153, 121, 0, 0, 192, 204, 22, 186] };
pub const PROPSETID_EXT_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3044215440, data2: 6700, data3: 4559, data4: [140, 35, 0, 170, 0, 107, 104, 20] };
pub const PROPSETID_EXT_TRANSPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2688341488, data2: 12357, data3: 4559, data4: [140, 68, 0, 170, 0, 107, 104, 20] };
pub const PROPSETID_TIMECODE_READER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2605280481, data2: 33051, data3: 4559, data4: [140, 119, 0, 170, 0, 107, 104, 20] };
pub const PROPSETID_TUNER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401093, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_CAMERACONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3336647536, data2: 12460, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_CAMERACONTROL_FLASH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2019462985, data2: 25506, data3: 16708, data4: [171, 112, 255, 178, 120, 250, 38, 206] };
pub const PROPSETID_VIDCAP_CAMERACONTROL_IMAGE_PIN_CAPABILITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2638052287, data2: 23661, data3: 16696, data4: [187, 0, 88, 78, 221, 32, 247, 197] };
pub const PROPSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2635256216, data2: 63596, data3: 20461, data4: [176, 35, 93, 135, 101, 61, 167, 147] };
pub const PROPSETID_VIDCAP_CAMERACONTROL_VIDEO_STABILIZATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1133923283, data2: 30486, data3: 16462, data4: [139, 225, 210, 153, 178, 14, 80, 253] };
pub const PROPSETID_VIDCAP_CROSSBAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401152, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_DROPPEDFRAMES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3336647492, data2: 12460, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_SELECTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 448638666, data2: 26806, data3: 20355, data4: [147, 113, 180, 19, 144, 124, 123, 159] };
pub const PROPSETID_VIDCAP_TVAUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401168, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_VIDEOCOMPRESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3336647491, data2: 12460, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_VIDEOCONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401200, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_VIDEODECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3336647504, data2: 12460, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_VIDEOENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781401104, data2: 10468, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
pub const PROPSETID_VIDCAP_VIDEOPROCAMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3336647520, data2: 12460, data3: 4560, data4: [161, 140, 0, 160, 201, 17, 137, 86] };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_Align: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_Buffers: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_FixedCompression: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_Flags: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_LogicalEnd: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_MemoryTypes: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_None: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_OptimalRanges: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_PhysicalEnd: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_PhysicalRanges: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_UnknownCompression: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_UserModeDownstream: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeFactor_UserModeUpstream: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const RT_RCDATA: ::windows_sys::core::PCWSTR = 10i32 as _;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const RT_STRING: ::windows_sys::core::PCWSTR = 6i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct SECURE_BUFFER_INFO {
    pub guidBufferIdentifier: ::windows_sys::core::GUID,
    pub cbBufferSize: u32,
    pub cbCaptured: u32,
    pub ullReserved: [u64; 16],
}
impl ::core::marker::Copy for SECURE_BUFFER_INFO {}
impl ::core::clone::Clone for SECURE_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct SOUNDDETECTOR_PATTERNHEADER {
    pub Size: u32,
    pub PatternType: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SOUNDDETECTOR_PATTERNHEADER {}
impl ::core::clone::Clone for SOUNDDETECTOR_PATTERNHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_ALL: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_BACK_CENTER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_BACK_LEFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_BACK_RIGHT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_FRONT_CENTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_FRONT_LEFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_FRONT_LEFT_OF_CENTER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_FRONT_RIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_LOW_FREQUENCY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_RESERVED: u32 = 2147221504u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_SIDE_LEFT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_SIDE_RIGHT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_TOP_BACK_CENTER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_TOP_BACK_LEFT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_TOP_BACK_RIGHT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_TOP_CENTER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_TOP_FRONT_CENTER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_TOP_FRONT_LEFT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SPEAKER_TOP_FRONT_RIGHT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SYSAUDIO_FLAGS_CLEAR_PREFERRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SYSAUDIO_FLAGS_DONT_COMBINE_PINS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type TELEPHONY_CALLCONTROLOP = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLCONTROLOP_DISABLE: TELEPHONY_CALLCONTROLOP = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLCONTROLOP_ENABLE: TELEPHONY_CALLCONTROLOP = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type TELEPHONY_CALLSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLSTATE_DISABLED: TELEPHONY_CALLSTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLSTATE_ENABLED: TELEPHONY_CALLSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLSTATE_HOLD: TELEPHONY_CALLSTATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLSTATE_PROVIDERTRANSITION: TELEPHONY_CALLSTATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type TELEPHONY_CALLTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLTYPE_CIRCUITSWITCHED: TELEPHONY_CALLTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_LTE: TELEPHONY_CALLTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_WLAN: TELEPHONY_CALLTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type TELEPHONY_PROVIDERCHANGEOP = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_PROVIDERCHANGEOP_END: TELEPHONY_PROVIDERCHANGEOP = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_PROVIDERCHANGEOP_BEGIN: TELEPHONY_PROVIDERCHANGEOP = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_PROVIDERCHANGEOP_CANCEL: TELEPHONY_PROVIDERCHANGEOP = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct TRANSPORTAUDIOPARMS {
    pub EnableOutput: i32,
    pub EnableRecord: i32,
    pub EnableSelsync: i32,
    pub Input: i32,
    pub MonitorSource: i32,
}
impl ::core::marker::Copy for TRANSPORTAUDIOPARMS {}
impl ::core::clone::Clone for TRANSPORTAUDIOPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct TRANSPORTBASICPARMS {
    pub TimeFormat: i32,
    pub TimeReference: i32,
    pub Superimpose: i32,
    pub EndStopAction: i32,
    pub RecordFormat: i32,
    pub StepFrames: i32,
    pub SetpField: i32,
    pub Preroll: i32,
    pub RecPreroll: i32,
    pub Postroll: i32,
    pub EditDelay: i32,
    pub PlayTCDelay: i32,
    pub RecTCDelay: i32,
    pub EditField: i32,
    pub FrameServo: i32,
    pub ColorFrameServo: i32,
    pub ServoRef: i32,
    pub WarnGenlock: i32,
    pub SetTracking: i32,
    pub VolumeName: [i8; 40],
    pub Ballistic: [i32; 20],
    pub Speed: i32,
    pub CounterFormat: i32,
    pub TunerChannel: i32,
    pub TunerNumber: i32,
    pub TimerEvent: i32,
    pub TimerStartDay: i32,
    pub TimerStartTime: i32,
    pub TimerStopDay: i32,
    pub TimerStopTime: i32,
}
impl ::core::marker::Copy for TRANSPORTBASICPARMS {}
impl ::core::clone::Clone for TRANSPORTBASICPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct TRANSPORTSTATUS {
    pub Mode: i32,
    pub LastError: i32,
    pub RecordInhibit: i32,
    pub ServoLock: i32,
    pub MediaPresent: i32,
    pub MediaLength: i32,
    pub MediaSize: i32,
    pub MediaTrackCount: i32,
    pub MediaTrackLength: i32,
    pub MediaTrackSide: i32,
    pub MediaType: i32,
    pub LinkMode: i32,
    pub NotifyOn: i32,
}
impl ::core::marker::Copy for TRANSPORTSTATUS {}
impl ::core::clone::Clone for TRANSPORTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct TRANSPORTVIDEOPARMS {
    pub OutputMode: i32,
    pub Input: i32,
}
impl ::core::marker::Copy for TRANSPORTVIDEOPARMS {}
impl ::core::clone::Clone for TRANSPORTVIDEOPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct TRANSPORT_STATE {
    pub Mode: u32,
    pub State: u32,
}
impl ::core::marker::Copy for TRANSPORT_STATE {}
impl ::core::clone::Clone for TRANSPORT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct TUNER_ANALOG_CAPS_S {
    pub Mode: u32,
    pub StandardsSupported: u32,
    pub MinFrequency: u32,
    pub MaxFrequency: u32,
    pub TuningGranularity: u32,
    pub SettlingTime: u32,
    pub ScanSensingRange: u32,
    pub FineTuneSensingRange: u32,
}
impl ::core::marker::Copy for TUNER_ANALOG_CAPS_S {}
impl ::core::clone::Clone for TUNER_ANALOG_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICAP_PROPERTIES_PROTECTION_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Status: u32,
}
impl ::core::marker::Copy for VBICAP_PROPERTIES_PROTECTION_S {}
impl ::core::clone::Clone for VBICAP_PROPERTIES_PROTECTION_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_CC_SUBSTREAMS {
    pub SubstreamMask: u32,
}
impl ::core::marker::Copy for VBICODECFILTERING_CC_SUBSTREAMS {}
impl ::core::clone::Clone for VBICODECFILTERING_CC_SUBSTREAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_NABTS_SUBSTREAMS {
    pub SubstreamMask: [u32; 128],
}
impl ::core::marker::Copy for VBICODECFILTERING_NABTS_SUBSTREAMS {}
impl ::core::clone::Clone for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_SCANLINES {
    pub DwordBitArray: [u32; 32],
}
impl ::core::marker::Copy for VBICODECFILTERING_SCANLINES {}
impl ::core::clone::Clone for VBICODECFILTERING_SCANLINES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_STATISTICS_CC {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
impl ::core::marker::Copy for VBICODECFILTERING_STATISTICS_CC {}
impl ::core::clone::Clone for VBICODECFILTERING_STATISTICS_CC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_STATISTICS_CC_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl ::core::marker::Copy for VBICODECFILTERING_STATISTICS_CC_PIN {}
impl ::core::clone::Clone for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_STATISTICS_COMMON {
    pub InputSRBsProcessed: u32,
    pub OutputSRBsProcessed: u32,
    pub SRBsIgnored: u32,
    pub InputSRBsMissing: u32,
    pub OutputSRBsMissing: u32,
    pub OutputFailures: u32,
    pub InternalErrors: u32,
    pub ExternalErrors: u32,
    pub InputDiscontinuities: u32,
    pub DSPFailures: u32,
    pub TvTunerChanges: u32,
    pub VBIHeaderChanges: u32,
    pub LineConfidenceAvg: u32,
    pub BytesOutput: u32,
}
impl ::core::marker::Copy for VBICODECFILTERING_STATISTICS_COMMON {}
impl ::core::clone::Clone for VBICODECFILTERING_STATISTICS_COMMON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_STATISTICS_COMMON_PIN {
    pub SRBsProcessed: u32,
    pub SRBsIgnored: u32,
    pub SRBsMissing: u32,
    pub InternalErrors: u32,
    pub ExternalErrors: u32,
    pub Discontinuities: u32,
    pub LineConfidenceAvg: u32,
    pub BytesOutput: u32,
}
impl ::core::marker::Copy for VBICODECFILTERING_STATISTICS_COMMON_PIN {}
impl ::core::clone::Clone for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_STATISTICS_NABTS {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
    pub FECBundleBadLines: u32,
    pub FECQueueOverflows: u32,
    pub FECCorrectedLines: u32,
    pub FECUncorrectableLines: u32,
    pub BundlesProcessed: u32,
    pub BundlesSent2IP: u32,
    pub FilteredLines: u32,
}
impl ::core::marker::Copy for VBICODECFILTERING_STATISTICS_NABTS {}
impl ::core::clone::Clone for VBICODECFILTERING_STATISTICS_NABTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_STATISTICS_NABTS_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl ::core::marker::Copy for VBICODECFILTERING_STATISTICS_NABTS_PIN {}
impl ::core::clone::Clone for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
impl ::core::marker::Copy for VBICODECFILTERING_STATISTICS_TELETEXT {}
impl ::core::clone::Clone for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl ::core::marker::Copy for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {}
impl ::core::clone::Clone for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VRAM_SURFACE_INFO {
    pub hSurface: usize,
    pub VramPhysicalAddress: i64,
    pub cbCaptured: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwLinearSize: u32,
    pub lPitch: i32,
    pub ullReserved: [u64; 16],
}
impl ::core::marker::Copy for VRAM_SURFACE_INFO {}
impl ::core::clone::Clone for VRAM_SURFACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct VRAM_SURFACE_INFO_PROPERTY_S {
    pub Property: KSIDENTIFIER,
    pub pVramSurfaceInfo: *mut VRAM_SURFACE_INFO,
}
impl ::core::marker::Copy for VRAM_SURFACE_INFO_PROPERTY_S {}
impl ::core::clone::Clone for VRAM_SURFACE_INFO_PROPERTY_S {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const WAVE_FORMAT_EXTENSIBLE: u32 = 65534u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct WNF_KSCAMERA_STREAMSTATE_INFO {
    pub ProcessId: u32,
    pub SessionId: u32,
    pub StreamState: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for WNF_KSCAMERA_STREAMSTATE_INFO {}
impl ::core::clone::Clone for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct WST_BUFFER {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub WstLines: [WST_BUFFER_LINE; 17],
}
impl ::core::marker::Copy for WST_BUFFER {}
impl ::core::clone::Clone for WST_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct WST_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 42],
}
impl ::core::marker::Copy for WST_BUFFER_LINE {}
impl ::core::clone::Clone for WST_BUFFER_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const WST_BYTES_PER_LINE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const WST_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const WST_TVTUNER_CHANGE_END_TUNE: i32 = 8192i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    pub ProcessingMode: ::windows_sys::core::GUID,
    pub SamplesPerProcessingPacket: u32,
    pub ProcessingPacketDurationInHns: u32,
}
impl ::core::marker::Copy for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {}
impl ::core::clone::Clone for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub type _TunerDecoderLockType = i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Tuner_LockType_None: _TunerDecoderLockType = 0i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Tuner_LockType_Within_Scan_Sensing_Range: _TunerDecoderLockType = 1i32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Tuner_LockType_Locked: _TunerDecoderLockType = 2i32;
