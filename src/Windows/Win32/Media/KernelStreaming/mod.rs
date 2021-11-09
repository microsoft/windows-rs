#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AEC_MODE_FULL_DUPLEX: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AEC_MODE_HALF_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AEC_MODE_PASS_THROUGH: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AEC_STATUS_FD_CURRENTLY_CONVERGED: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AEC_STATUS_FD_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AEC_STATUS_FD_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AEC_STATUS_FD_HISTORY_UNINITIALIZED: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct ALLOCATOR_PROPERTIES_EX {
    pub cBuffers: i32,
    pub cbBuffer: i32,
    pub cbAlign: i32,
    pub cbPrefix: i32,
    pub MemoryType: ::windows::runtime::GUID,
    pub BusType: ::windows::runtime::GUID,
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
impl ALLOCATOR_PROPERTIES_EX {}
impl ::core::default::Default for ALLOCATOR_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ALLOCATOR_PROPERTIES_EX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ALLOCATOR_PROPERTIES_EX")
            .field("cBuffers", &self.cBuffers)
            .field("cbBuffer", &self.cbBuffer)
            .field("cbAlign", &self.cbAlign)
            .field("cbPrefix", &self.cbPrefix)
            .field("MemoryType", &self.MemoryType)
            .field("BusType", &self.BusType)
            .field("State", &self.State)
            .field("Input", &self.Input)
            .field("Output", &self.Output)
            .field("Strategy", &self.Strategy)
            .field("Flags", &self.Flags)
            .field("Weight", &self.Weight)
            .field("LogicalMemoryType", &self.LogicalMemoryType)
            .field("AllocatorPlace", &self.AllocatorPlace)
            .field("Dimensions", &self.Dimensions)
            .field("PhysicalRange", &self.PhysicalRange)
            .field("PrevSegment", &self.PrevSegment)
            .field("CountNextSegments", &self.CountNextSegments)
            .field("NextSegments", &self.NextSegments)
            .field("InsideFactors", &self.InsideFactors)
            .field("NumberPins", &self.NumberPins)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ALLOCATOR_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cBuffers == other.cBuffers
            && self.cbBuffer == other.cbBuffer
            && self.cbAlign == other.cbAlign
            && self.cbPrefix == other.cbPrefix
            && self.MemoryType == other.MemoryType
            && self.BusType == other.BusType
            && self.State == other.State
            && self.Input == other.Input
            && self.Output == other.Output
            && self.Strategy == other.Strategy
            && self.Flags == other.Flags
            && self.Weight == other.Weight
            && self.LogicalMemoryType == other.LogicalMemoryType
            && self.AllocatorPlace == other.AllocatorPlace
            && self.Dimensions == other.Dimensions
            && self.PhysicalRange == other.PhysicalRange
            && self.PrevSegment == other.PrevSegment
            && self.CountNextSegments == other.CountNextSegments
            && self.NextSegments == other.NextSegments
            && self.InsideFactors == other.InsideFactors
            && self.NumberPins == other.NumberPins
    }
}
impl ::core::cmp::Eq for ALLOCATOR_PROPERTIES_EX {}
unsafe impl ::windows::runtime::Abi for ALLOCATOR_PROPERTIES_EX {
    type Abi = Self;
}
pub const APO_CLASS_UUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1502215400, 40144, 18045, [138, 106, 84, 25, 227, 21, 41, 212]);
pub const AUDIOENDPOINT_CLASS_UUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3244708412, 65036, 19092, [165, 134, 241, 168, 12, 251, 191, 62]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AUDIOMODULE_MAX_DATA_SIZE: u32 = 64000u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AUDIOMODULE_MAX_NAME_CCH_SIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIOPOSTURE_ORIENTATION(pub i32);
pub const AUDIOPOSTURE_ORIENTATION_NOTROTATED: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(0i32);
pub const AUDIOPOSTURE_ORIENTATION_ROTATED90DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(1i32);
pub const AUDIOPOSTURE_ORIENTATION_ROTATED180DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(2i32);
pub const AUDIOPOSTURE_ORIENTATION_ROTATED270DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(3i32);
impl ::core::convert::From<i32> for AUDIOPOSTURE_ORIENTATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUDIOPOSTURE_ORIENTATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    pub ResourceGroupAcquired: super::super::Foundation::BOOL,
    pub ResourceGroupName: [u16; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIORESOURCEMANAGEMENT_RESOURCEGROUP").field("ResourceGroupAcquired", &self.ResourceGroupAcquired).field("ResourceGroupName", &self.ResourceGroupName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn eq(&self, other: &Self) -> bool {
        self.ResourceGroupAcquired == other.ResourceGroupAcquired && self.ResourceGroupName == other.ResourceGroupName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIO_CURVE_TYPE(pub i32);
pub const AUDIO_CURVE_TYPE_NONE: AUDIO_CURVE_TYPE = AUDIO_CURVE_TYPE(0i32);
pub const AUDIO_CURVE_TYPE_WINDOWS_FADE: AUDIO_CURVE_TYPE = AUDIO_CURVE_TYPE(1i32);
impl ::core::convert::From<i32> for AUDIO_CURVE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUDIO_CURVE_TYPE {
    type Abi = Self;
}
pub const AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869054, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_AUTOMATIC_GAIN_CONTROL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869056, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_BASS_BOOST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869061, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_BASS_MANAGEMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869066, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_BEAMFORMING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869057, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_CONSTANT_TONE_REMOVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869058, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_DEEP_NOISE_SUPPRESSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869072, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_DYNAMIC_RANGE_COMPRESSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869070, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_ENVIRONMENTAL_EFFECTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869067, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_EQUALIZER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869059, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_FAR_FIELD_BEAMFORMING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869071, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_LOUDNESS_EQUALIZER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869060, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_NOISE_SUPPRESSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869055, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_ROOM_CORRECTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869065, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_SPEAKER_COMPENSATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869069, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_SPEAKER_FILL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869064, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_SPEAKER_PROTECTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869068, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_VIRTUAL_HEADPHONES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869063, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_EFFECT_TYPE_VIRTUAL_SURROUND: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868869062, 33297, 4578, [140, 112, 44, 39, 215, 240, 1, 250]);
pub const AUDIO_SIGNALPROCESSINGMODE_COMMUNICATIONS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2559906611, 47565, 18609, [160, 163, 255, 64, 104, 45, 115, 247]);
pub const AUDIO_SIGNALPROCESSINGMODE_DEFAULT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3247320958, 37693, 18789, [183, 209, 30, 239, 34, 141, 42, 243]);
pub const AUDIO_SIGNALPROCESSINGMODE_FAR_FIELD_SPEECH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(680795322, 15334, 19064, [154, 118, 48, 253, 145, 85, 155, 100]);
pub const AUDIO_SIGNALPROCESSINGMODE_MEDIA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1199571022, 28979, 16856, [140, 116, 102, 13, 173, 210, 192, 238]);
pub const AUDIO_SIGNALPROCESSINGMODE_MOVIE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2993679117, 60564, 18300, [148, 148, 209, 171, 142, 117, 63, 110]);
pub const AUDIO_SIGNALPROCESSINGMODE_NOTIFICATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2633148171, 62327, 16443, [189, 107, 54, 8, 99, 224, 53, 92]);
pub const AUDIO_SIGNALPROCESSINGMODE_RAW: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2660297248, 46227, 20433, [161, 168, 126, 19, 97, 169, 86, 207]);
pub const AUDIO_SIGNALPROCESSINGMODE_SPEECH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4229758107, 47574, 19706, [181, 224, 75, 178, 22, 104, 120, 178]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AllocatorStrategy_DontCare: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AllocatorStrategy_MaximizeSpeed: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AllocatorStrategy_MinimizeFrameSize: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AllocatorStrategy_MinimizeNumberOfAllocators: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AllocatorStrategy_MinimizeNumberOfFrames: u32 = 1u32;
pub const BLUETOOTHLE_MIDI_SERVICE_UUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(62393946, 60904, 19251, [167, 81, 108, 227, 78, 196, 199, 0]);
pub const BLUETOOTH_MIDI_DATAIO_CHARACTERISTIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2004018651, 14440, 16658, [161, 169, 242, 102, 157, 16, 107, 243]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const BUS_INTERFACE_REFERENCE_VERSION: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CAPTURE_MEMORY_ALLOCATION_FLAGS(pub i32);
pub const KS_CAPTURE_ALLOC_INVALID: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(0i32);
pub const KS_CAPTURE_ALLOC_SYSTEM: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(1i32);
pub const KS_CAPTURE_ALLOC_VRAM: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(2i32);
pub const KS_CAPTURE_ALLOC_SYSTEM_AGP: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(4i32);
pub const KS_CAPTURE_ALLOC_VRAM_MAPPED: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(8i32);
pub const KS_CAPTURE_ALLOC_SECURE_BUFFER: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(16i32);
impl ::core::convert::From<i32> for CAPTURE_MEMORY_ALLOCATION_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CAPTURE_MEMORY_ALLOCATION_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct CC_BYTE_PAIR {
    pub Decoded: [u8; 2],
    pub Reserved: u16,
}
impl CC_BYTE_PAIR {}
impl ::core::default::Default for CC_BYTE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CC_BYTE_PAIR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CC_BYTE_PAIR").field("Decoded", &self.Decoded).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for CC_BYTE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.Decoded == other.Decoded && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for CC_BYTE_PAIR {}
unsafe impl ::windows::runtime::Abi for CC_BYTE_PAIR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct CC_HW_FIELD {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub fieldFlags: u32,
    pub PictureNumber: i64,
    pub Lines: [CC_BYTE_PAIR; 12],
}
impl CC_HW_FIELD {}
impl ::core::default::Default for CC_HW_FIELD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CC_HW_FIELD {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CC_HW_FIELD").field("ScanlinesRequested", &self.ScanlinesRequested).field("fieldFlags", &self.fieldFlags).field("PictureNumber", &self.PictureNumber).field("Lines", &self.Lines).finish()
    }
}
impl ::core::cmp::PartialEq for CC_HW_FIELD {
    fn eq(&self, other: &Self) -> bool {
        self.ScanlinesRequested == other.ScanlinesRequested && self.fieldFlags == other.fieldFlags && self.PictureNumber == other.PictureNumber && self.Lines == other.Lines
    }
}
impl ::core::cmp::Eq for CC_HW_FIELD {}
unsafe impl ::windows::runtime::Abi for CC_HW_FIELD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const CC_MAX_HW_DECODE_LINES: u32 = 12u32;
pub const CLSID_KsIBasicAudioInterfaceHandler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3120081982, 3953, 4562, [183, 44, 0, 192, 79, 182, 189, 61]);
pub const CLSID_Proxy: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(399288091, 60631, 4560, [185, 8, 0, 160, 201, 34, 49, 150]);
pub const CODECAPI_ALLSETTINGS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1784118930, 33761, 16659, [173, 194, 79, 206, 195, 47, 131, 161]);
pub const CODECAPI_AUDIO_ENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3117521470, 63639, 17052, [188, 70, 129, 56, 183, 39, 43, 45]);
pub const CODECAPI_CHANGELISTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1655777999, 63152, 18393, [148, 86, 150, 242, 44, 78, 11, 157]);
pub const CODECAPI_CURRENTCHANGELIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(481382019, 32114, 18007, [131, 253, 71, 162, 197, 185, 209, 61]);
pub const CODECAPI_SETALLDEFAULTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1818126972, 44280, 20309, [169, 153, 26, 98, 129, 9, 5, 27]);
pub const CODECAPI_SUPPORTSEVENTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(92385175, 30355, 19901, [157, 202, 63, 158, 189, 101, 133, 161]);
pub const CODECAPI_VIDEO_ENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1897064673, 15619, 18415, [142, 96, 3, 241, 207, 83, 115, 1]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONSTRICTOR_OPTION(pub i32);
pub const CONSTRICTOR_OPTION_DISABLE: CONSTRICTOR_OPTION = CONSTRICTOR_OPTION(0i32);
pub const CONSTRICTOR_OPTION_MUTE: CONSTRICTOR_OPTION = CONSTRICTOR_OPTION(1i32);
impl ::core::convert::From<i32> for CONSTRICTOR_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONSTRICTOR_OPTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl DEVCAPS {}
impl ::core::default::Default for DEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DEVCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DEVCAPS")
            .field("CanRecord", &self.CanRecord)
            .field("CanRecordStrobe", &self.CanRecordStrobe)
            .field("HasAudio", &self.HasAudio)
            .field("HasVideo", &self.HasVideo)
            .field("UsesFiles", &self.UsesFiles)
            .field("CanSave", &self.CanSave)
            .field("DeviceType", &self.DeviceType)
            .field("TCRead", &self.TCRead)
            .field("TCWrite", &self.TCWrite)
            .field("CTLRead", &self.CTLRead)
            .field("IndexRead", &self.IndexRead)
            .field("Preroll", &self.Preroll)
            .field("Postroll", &self.Postroll)
            .field("SyncAcc", &self.SyncAcc)
            .field("NormRate", &self.NormRate)
            .field("CanPreview", &self.CanPreview)
            .field("CanMonitorSrc", &self.CanMonitorSrc)
            .field("CanTest", &self.CanTest)
            .field("VideoIn", &self.VideoIn)
            .field("AudioIn", &self.AudioIn)
            .field("Calibrate", &self.Calibrate)
            .field("SeekType", &self.SeekType)
            .field("SimulatedHardware", &self.SimulatedHardware)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.CanRecord == other.CanRecord
            && self.CanRecordStrobe == other.CanRecordStrobe
            && self.HasAudio == other.HasAudio
            && self.HasVideo == other.HasVideo
            && self.UsesFiles == other.UsesFiles
            && self.CanSave == other.CanSave
            && self.DeviceType == other.DeviceType
            && self.TCRead == other.TCRead
            && self.TCWrite == other.TCWrite
            && self.CTLRead == other.CTLRead
            && self.IndexRead == other.IndexRead
            && self.Preroll == other.Preroll
            && self.Postroll == other.Postroll
            && self.SyncAcc == other.SyncAcc
            && self.NormRate == other.NormRate
            && self.CanPreview == other.CanPreview
            && self.CanMonitorSrc == other.CanMonitorSrc
            && self.CanTest == other.CanTest
            && self.VideoIn == other.VideoIn
            && self.AudioIn == other.AudioIn
            && self.Calibrate == other.Calibrate
            && self.SeekType == other.SeekType
            && self.SimulatedHardware == other.SimulatedHardware
    }
}
impl ::core::cmp::Eq for DEVCAPS {}
unsafe impl ::windows::runtime::Abi for DEVCAPS {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_KsAudio_Controller_DeviceInterface_Path: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(333448406, 45158, 17341, [145, 59, 164, 21, 205, 19, 218, 135]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_KsAudio_PacketSize_Constraints: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(333448406, 45158, 17341, [145, 59, 164, 21, 205, 19, 218, 135]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_KsAudio_PacketSize_Constraints2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2483353473, 29073, 16539, [139, 11, 128, 191, 110, 194, 41, 174]),
    pid: 2u32,
};
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct DS3DVECTOR {
    pub Anonymous1: DS3DVECTOR_0,
    pub Anonymous2: DS3DVECTOR_1,
    pub Anonymous3: DS3DVECTOR_2,
}
impl DS3DVECTOR {}
impl ::core::default::Default for DS3DVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS3DVECTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DS3DVECTOR {}
unsafe impl ::windows::runtime::Abi for DS3DVECTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union DS3DVECTOR_0 {
    pub x: f32,
    pub dvX: f32,
}
impl DS3DVECTOR_0 {}
impl ::core::default::Default for DS3DVECTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS3DVECTOR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DS3DVECTOR_0 {}
unsafe impl ::windows::runtime::Abi for DS3DVECTOR_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union DS3DVECTOR_1 {
    pub y: f32,
    pub dvY: f32,
}
impl DS3DVECTOR_1 {}
impl ::core::default::Default for DS3DVECTOR_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS3DVECTOR_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DS3DVECTOR_1 {}
unsafe impl ::windows::runtime::Abi for DS3DVECTOR_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union DS3DVECTOR_2 {
    pub z: f32,
    pub dvZ: f32,
}
impl DS3DVECTOR_2 {}
impl ::core::default::Default for DS3DVECTOR_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS3DVECTOR_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DS3DVECTOR_2 {}
unsafe impl ::windows::runtime::Abi for DS3DVECTOR_2 {
    type Abi = Self;
}
pub const ENCAPIPARAM_BITRATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1238125635, 51843, 19156, [169, 175, 243, 105, 106, 246, 102, 223]);
pub const ENCAPIPARAM_BITRATE_MODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3999248988, 50963, 16593, [157, 88, 192, 215, 36, 30, 37, 15]);
pub const ENCAPIPARAM_PEAK_BITRATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1883182761, 15688, 17569, [176, 119, 1, 141, 255, 145, 93, 25]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EPcxConnectionType(pub i32);
pub const eConnTypeUnknown: EPcxConnectionType = EPcxConnectionType(0i32);
pub const eConnType3Point5mm: EPcxConnectionType = EPcxConnectionType(1i32);
pub const eConnTypeQuarter: EPcxConnectionType = EPcxConnectionType(2i32);
pub const eConnTypeAtapiInternal: EPcxConnectionType = EPcxConnectionType(3i32);
pub const eConnTypeRCA: EPcxConnectionType = EPcxConnectionType(4i32);
pub const eConnTypeOptical: EPcxConnectionType = EPcxConnectionType(5i32);
pub const eConnTypeOtherDigital: EPcxConnectionType = EPcxConnectionType(6i32);
pub const eConnTypeOtherAnalog: EPcxConnectionType = EPcxConnectionType(7i32);
pub const eConnTypeMultichannelAnalogDIN: EPcxConnectionType = EPcxConnectionType(8i32);
pub const eConnTypeXlrProfessional: EPcxConnectionType = EPcxConnectionType(9i32);
pub const eConnTypeRJ11Modem: EPcxConnectionType = EPcxConnectionType(10i32);
pub const eConnTypeCombination: EPcxConnectionType = EPcxConnectionType(11i32);
impl ::core::convert::From<i32> for EPcxConnectionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EPcxConnectionType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EPcxGenLocation(pub i32);
pub const eGenLocPrimaryBox: EPcxGenLocation = EPcxGenLocation(0i32);
pub const eGenLocInternal: EPcxGenLocation = EPcxGenLocation(1i32);
pub const eGenLocSeparate: EPcxGenLocation = EPcxGenLocation(2i32);
pub const eGenLocOther: EPcxGenLocation = EPcxGenLocation(3i32);
pub const EPcxGenLocation_enum_count: EPcxGenLocation = EPcxGenLocation(4i32);
impl ::core::convert::From<i32> for EPcxGenLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EPcxGenLocation {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EPcxGeoLocation(pub i32);
pub const eGeoLocRear: EPcxGeoLocation = EPcxGeoLocation(1i32);
pub const eGeoLocFront: EPcxGeoLocation = EPcxGeoLocation(2i32);
pub const eGeoLocLeft: EPcxGeoLocation = EPcxGeoLocation(3i32);
pub const eGeoLocRight: EPcxGeoLocation = EPcxGeoLocation(4i32);
pub const eGeoLocTop: EPcxGeoLocation = EPcxGeoLocation(5i32);
pub const eGeoLocBottom: EPcxGeoLocation = EPcxGeoLocation(6i32);
pub const eGeoLocRearPanel: EPcxGeoLocation = EPcxGeoLocation(7i32);
pub const eGeoLocRiser: EPcxGeoLocation = EPcxGeoLocation(8i32);
pub const eGeoLocInsideMobileLid: EPcxGeoLocation = EPcxGeoLocation(9i32);
pub const eGeoLocDrivebay: EPcxGeoLocation = EPcxGeoLocation(10i32);
pub const eGeoLocHDMI: EPcxGeoLocation = EPcxGeoLocation(11i32);
pub const eGeoLocOutsideMobileLid: EPcxGeoLocation = EPcxGeoLocation(12i32);
pub const eGeoLocATAPI: EPcxGeoLocation = EPcxGeoLocation(13i32);
pub const eGeoLocNotApplicable: EPcxGeoLocation = EPcxGeoLocation(14i32);
pub const eGeoLocReserved6: EPcxGeoLocation = EPcxGeoLocation(15i32);
pub const EPcxGeoLocation_enum_count: EPcxGeoLocation = EPcxGeoLocation(16i32);
impl ::core::convert::From<i32> for EPcxGeoLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EPcxGeoLocation {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EPxcPortConnection(pub i32);
pub const ePortConnJack: EPxcPortConnection = EPxcPortConnection(0i32);
pub const ePortConnIntegratedDevice: EPxcPortConnection = EPxcPortConnection(1i32);
pub const ePortConnBothIntegratedAndJack: EPxcPortConnection = EPxcPortConnection(2i32);
pub const ePortConnUnknown: EPxcPortConnection = EPxcPortConnection(3i32);
impl ::core::convert::From<i32> for EPxcPortConnection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EPxcPortConnection {
    type Abi = Self;
}
pub const EVENTSETID_CROSSBAR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401153, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const EVENTSETID_TUNER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401094, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const EVENTSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(803208285, 50994, 19366, [181, 223, 107, 77, 127, 200, 139, 139]);
pub const EVENTSETID_VIDEODECODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401121, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FRAMING_CACHE_OPS(pub i32);
pub const Framing_Cache_Update: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(0i32);
pub const Framing_Cache_ReadLast: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(1i32);
pub const Framing_Cache_ReadOrig: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(2i32);
pub const Framing_Cache_Write: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(3i32);
impl ::core::convert::From<i32> for FRAMING_CACHE_OPS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FRAMING_CACHE_OPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FRAMING_PROP(pub i32);
pub const FramingProp_Uninitialized: FRAMING_PROP = FRAMING_PROP(0i32);
pub const FramingProp_None: FRAMING_PROP = FRAMING_PROP(1i32);
pub const FramingProp_Old: FRAMING_PROP = FRAMING_PROP(2i32);
pub const FramingProp_Ex: FRAMING_PROP = FRAMING_PROP(3i32);
impl ::core::convert::From<i32> for FRAMING_PROP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FRAMING_PROP {
    type Abi = Self;
}
pub const GUID_NULL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsAggregateControl(pub ::windows::runtime::IUnknown);
impl IKsAggregateControl {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn KsAddAggregate(&self, aggregateclass: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(aggregateclass)).ok()
    }
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn KsRemoveAggregate(&self, aggregateclass: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(aggregateclass)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IKsAggregateControl {
    type Vtable = IKsAggregateControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2134960832, 14663, 4562, [135, 78, 0, 160, 201, 34, 49, 150]);
}
impl ::core::convert::From<IKsAggregateControl> for ::windows::runtime::IUnknown {
    fn from(value: IKsAggregateControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsAggregateControl> for ::windows::runtime::IUnknown {
    fn from(value: &IKsAggregateControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsAggregateControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsAggregateControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsAggregateControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aggregateclass: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aggregateclass: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IKsAllocator(pub u8);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IKsAllocatorEx(pub u8);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsControl(pub ::windows::runtime::IUnknown);
impl IKsControl {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn KsProperty(&self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(property), ::core::mem::transmute(propertylength), ::core::mem::transmute(propertydata), ::core::mem::transmute(datalength), ::core::mem::transmute(bytesreturned)).ok()
    }
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn KsMethod(&self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), ::core::mem::transmute(methodlength), ::core::mem::transmute(methoddata), ::core::mem::transmute(datalength), ::core::mem::transmute(bytesreturned)).ok()
    }
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn KsEvent(&self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(event), ::core::mem::transmute(eventlength), ::core::mem::transmute(eventdata), ::core::mem::transmute(datalength), ::core::mem::transmute(bytesreturned)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IKsControl {
    type Vtable = IKsControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(687163013, 1789, 4562, [178, 122, 0, 160, 201, 34, 49, 150]);
}
impl ::core::convert::From<IKsControl> for ::windows::runtime::IUnknown {
    fn from(value: IKsControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsControl> for ::windows::runtime::IUnknown {
    fn from(value: &IKsControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsFormatSupport(pub ::windows::runtime::IUnknown);
impl IKsFormatSupport {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    pub unsafe fn IsFormatSupported(&self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pksformat), ::core::mem::transmute(cbformat), ::core::mem::transmute(pbsupported)).ok()
    }
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn GetDevicePreferredFormat(&self) -> ::windows::runtime::Result<*mut KSDATAFORMAT> {
        let mut result__: <*mut KSDATAFORMAT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut KSDATAFORMAT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IKsFormatSupport {
    type Vtable = IKsFormatSupport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1018472093, 47983, 19755, [149, 183, 69, 45, 44, 21, 93, 181]);
}
impl ::core::convert::From<IKsFormatSupport> for ::windows::runtime::IUnknown {
    fn from(value: IKsFormatSupport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsFormatSupport> for ::windows::runtime::IUnknown {
    fn from(value: &IKsFormatSupport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsFormatSupport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsFormatSupport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsFormatSupport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppksformat: *mut *mut KSDATAFORMAT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsJackContainerId(pub ::windows::runtime::IUnknown);
impl IKsJackContainerId {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn GetJackContainerId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IKsJackContainerId {
    type Vtable = IKsJackContainerId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3382375523, 54825, 20164, [140, 0, 229, 77, 104, 21, 66, 72]);
}
impl ::core::convert::From<IKsJackContainerId> for ::windows::runtime::IUnknown {
    fn from(value: IKsJackContainerId) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsJackContainerId> for ::windows::runtime::IUnknown {
    fn from(value: &IKsJackContainerId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsJackContainerId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsJackContainerId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackContainerId_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pjackcontainerid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsJackDescription(pub ::windows::runtime::IUnknown);
impl IKsJackDescription {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn GetJackCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    pub unsafe fn GetJackDescription(&self, njack: u32) -> ::windows::runtime::Result<KSJACK_DESCRIPTION> {
        let mut result__: <KSJACK_DESCRIPTION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(njack), &mut result__).from_abi::<KSJACK_DESCRIPTION>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IKsJackDescription {
    type Vtable = IKsJackDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1158281047, 11590, 17975, [142, 98, 206, 125, 185, 68, 245, 123]);
}
impl ::core::convert::From<IKsJackDescription> for ::windows::runtime::IUnknown {
    fn from(value: IKsJackDescription) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsJackDescription> for ::windows::runtime::IUnknown {
    fn from(value: &IKsJackDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsJackDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsJackDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcjacks: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsJackDescription2(pub ::windows::runtime::IUnknown);
impl IKsJackDescription2 {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn GetJackCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn GetJackDescription2(&self, njack: u32) -> ::windows::runtime::Result<KSJACK_DESCRIPTION2> {
        let mut result__: <KSJACK_DESCRIPTION2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(njack), &mut result__).from_abi::<KSJACK_DESCRIPTION2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IKsJackDescription2 {
    type Vtable = IKsJackDescription2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1200568987, 57545, 18471, [146, 40, 111, 85, 5, 255, 231, 106]);
}
impl ::core::convert::From<IKsJackDescription2> for ::windows::runtime::IUnknown {
    fn from(value: IKsJackDescription2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsJackDescription2> for ::windows::runtime::IUnknown {
    fn from(value: &IKsJackDescription2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsJackDescription2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsJackDescription2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcjacks: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsJackSinkInformation(pub ::windows::runtime::IUnknown);
impl IKsJackSinkInformation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    pub unsafe fn GetJackSinkInformation(&self) -> ::windows::runtime::Result<KSJACK_SINK_INFORMATION> {
        let mut result__: <KSJACK_SINK_INFORMATION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<KSJACK_SINK_INFORMATION>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IKsJackSinkInformation {
    type Vtable = IKsJackSinkInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3653071597, 10511, 17793, [159, 243, 97, 2, 122, 143, 229, 50]);
}
impl ::core::convert::From<IKsJackSinkInformation> for ::windows::runtime::IUnknown {
    fn from(value: IKsJackSinkInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsJackSinkInformation> for ::windows::runtime::IUnknown {
    fn from(value: &IKsJackSinkInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsJackSinkInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsJackSinkInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackSinkInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IKsPin(pub u8);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsPropertySet(pub ::windows::runtime::IUnknown);
impl IKsPropertySet {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn Set(&self, propset: *const ::windows::runtime::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(propset), ::core::mem::transmute(id), ::core::mem::transmute(instancedata), ::core::mem::transmute(instancelength), ::core::mem::transmute(propertydata), ::core::mem::transmute(datalength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn Get(&self, propset: *const ::windows::runtime::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(propset), ::core::mem::transmute(id), ::core::mem::transmute(instancedata), ::core::mem::transmute(instancelength), ::core::mem::transmute(propertydata), ::core::mem::transmute(datalength), ::core::mem::transmute(bytesreturned)).ok()
    }
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn QuerySupported(&self, propset: *const ::windows::runtime::GUID, id: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(propset), ::core::mem::transmute(id), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IKsPropertySet {
    type Vtable = IKsPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(837790768, 20828, 4560, [169, 170, 0, 170, 0, 97, 190, 147]);
}
impl ::core::convert::From<IKsPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: IKsPropertySet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &IKsPropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsPropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propset: *const ::windows::runtime::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propset: *const ::windows::runtime::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propset: *const ::windows::runtime::GUID, id: u32, typesupport: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKsTopology(pub ::windows::runtime::IUnknown);
impl IKsTopology {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
    pub unsafe fn CreateNodeInstance<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: Param3, interfaceid: *const ::windows::runtime::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(nodeid), ::core::mem::transmute(flags), ::core::mem::transmute(desiredaccess), unkouter.into_param().abi(), ::core::mem::transmute(interfaceid), ::core::mem::transmute(interface)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IKsTopology {
    type Vtable = IKsTopology_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(687163011, 1789, 4562, [178, 122, 0, 160, 201, 34, 49, 150]);
}
impl ::core::convert::From<IKsTopology> for ::windows::runtime::IUnknown {
    fn from(value: IKsTopology) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKsTopology> for ::windows::runtime::IUnknown {
    fn from(value: &IKsTopology) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKsTopology {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKsTopology {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsTopology_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: ::windows::runtime::RawPtr, interfaceid: *const ::windows::runtime::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    pub Size: u32,
    pub PrimaryChannelCount: u32,
    pub PrimaryChannelStartPosition: u32,
    pub PrimaryChannelMask: u32,
    pub InterleavedChannelCount: u32,
    pub InterleavedChannelStartPosition: u32,
    pub InterleavedChannelMask: u32,
}
impl INTERLEAVED_AUDIO_FORMAT_INFORMATION {}
impl ::core::default::Default for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERLEAVED_AUDIO_FORMAT_INFORMATION")
            .field("Size", &self.Size)
            .field("PrimaryChannelCount", &self.PrimaryChannelCount)
            .field("PrimaryChannelStartPosition", &self.PrimaryChannelStartPosition)
            .field("PrimaryChannelMask", &self.PrimaryChannelMask)
            .field("InterleavedChannelCount", &self.InterleavedChannelCount)
            .field("InterleavedChannelStartPosition", &self.InterleavedChannelStartPosition)
            .field("InterleavedChannelMask", &self.InterleavedChannelMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.PrimaryChannelCount == other.PrimaryChannelCount && self.PrimaryChannelStartPosition == other.PrimaryChannelStartPosition && self.PrimaryChannelMask == other.PrimaryChannelMask && self.InterleavedChannelCount == other.InterleavedChannelCount && self.InterleavedChannelStartPosition == other.InterleavedChannelStartPosition && self.InterleavedChannelMask == other.InterleavedChannelMask
    }
}
impl ::core::cmp::Eq for INTERLEAVED_AUDIO_FORMAT_INFORMATION {}
unsafe impl ::windows::runtime::Abi for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const IOCTL_KS_DISABLE_EVENT: u32 = 3080203u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const IOCTL_KS_ENABLE_EVENT: u32 = 3080199u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const IOCTL_KS_HANDSHAKE: u32 = 3080223u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const IOCTL_KS_METHOD: u32 = 3080207u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const IOCTL_KS_PROPERTY: u32 = 3080195u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const IOCTL_KS_READ_STREAM: u32 = 3096599u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const IOCTL_KS_RESET_STATE: u32 = 3080219u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const IOCTL_KS_WRITE_STREAM: u32 = 3112979u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const JACKDESC2_DYNAMIC_FORMAT_CHANGE_CAPABILITY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const JACKDESC2_PRESENCE_DETECT_CAPABILITY: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSAC3_ALTERNATE_AUDIO {
    pub fStereo: super::super::Foundation::BOOL,
    pub DualMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSAC3_ALTERNATE_AUDIO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ALTERNATE_AUDIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ALTERNATE_AUDIO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAC3_ALTERNATE_AUDIO").field("fStereo", &self.fStereo).field("DualMode", &self.DualMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ALTERNATE_AUDIO {
    fn eq(&self, other: &Self) -> bool {
        self.fStereo == other.fStereo && self.DualMode == other.DualMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ALTERNATE_AUDIO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAC3_ALTERNATE_AUDIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_ALTERNATE_AUDIO_1: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_ALTERNATE_AUDIO_2: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_ALTERNATE_AUDIO_BOTH: u32 = 3u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAC3_BIT_STREAM_MODE {
    pub BitStreamMode: i32,
}
impl KSAC3_BIT_STREAM_MODE {}
impl ::core::default::Default for KSAC3_BIT_STREAM_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAC3_BIT_STREAM_MODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAC3_BIT_STREAM_MODE").field("BitStreamMode", &self.BitStreamMode).finish()
    }
}
impl ::core::cmp::PartialEq for KSAC3_BIT_STREAM_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.BitStreamMode == other.BitStreamMode
    }
}
impl ::core::cmp::Eq for KSAC3_BIT_STREAM_MODE {}
unsafe impl ::windows::runtime::Abi for KSAC3_BIT_STREAM_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAC3_DIALOGUE_LEVEL {
    pub DialogueLevel: u32,
}
impl KSAC3_DIALOGUE_LEVEL {}
impl ::core::default::Default for KSAC3_DIALOGUE_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAC3_DIALOGUE_LEVEL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAC3_DIALOGUE_LEVEL").field("DialogueLevel", &self.DialogueLevel).finish()
    }
}
impl ::core::cmp::PartialEq for KSAC3_DIALOGUE_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.DialogueLevel == other.DialogueLevel
    }
}
impl ::core::cmp::Eq for KSAC3_DIALOGUE_LEVEL {}
unsafe impl ::windows::runtime::Abi for KSAC3_DIALOGUE_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSAC3_DOWNMIX {
    pub fDownMix: super::super::Foundation::BOOL,
    pub fDolbySurround: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSAC3_DOWNMIX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_DOWNMIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_DOWNMIX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAC3_DOWNMIX").field("fDownMix", &self.fDownMix).field("fDolbySurround", &self.fDolbySurround).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_DOWNMIX {
    fn eq(&self, other: &Self) -> bool {
        self.fDownMix == other.fDownMix && self.fDolbySurround == other.fDolbySurround
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_DOWNMIX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAC3_DOWNMIX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSAC3_ERROR_CONCEALMENT {
    pub fRepeatPreviousBlock: super::super::Foundation::BOOL,
    pub fErrorInCurrentBlock: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSAC3_ERROR_CONCEALMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ERROR_CONCEALMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ERROR_CONCEALMENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAC3_ERROR_CONCEALMENT").field("fRepeatPreviousBlock", &self.fRepeatPreviousBlock).field("fErrorInCurrentBlock", &self.fErrorInCurrentBlock).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ERROR_CONCEALMENT {
    fn eq(&self, other: &Self) -> bool {
        self.fRepeatPreviousBlock == other.fRepeatPreviousBlock && self.fErrorInCurrentBlock == other.fErrorInCurrentBlock
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ERROR_CONCEALMENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAC3_ERROR_CONCEALMENT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSAC3_ROOM_TYPE {
    pub fLargeRoom: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSAC3_ROOM_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ROOM_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ROOM_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAC3_ROOM_TYPE").field("fLargeRoom", &self.fLargeRoom).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ROOM_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.fLargeRoom == other.fLargeRoom
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ROOM_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAC3_ROOM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_SERVICE_COMMENTARY: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_SERVICE_DIALOG_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_SERVICE_EMERGENCY_FLASH: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_SERVICE_HEARING_IMPAIRED: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_SERVICE_MAIN_AUDIO: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_SERVICE_NO_DIALOG: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_SERVICE_VISUALLY_IMPAIRED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_SERVICE_VOICE_OVER: u32 = 7u32;
pub const KSALGORITHMINSTANCE_SYSTEM_ACOUSTIC_ECHO_CANCEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(472040813, 39033, 20315, [163, 137, 39, 153, 109, 220, 40, 16]);
pub const KSALGORITHMINSTANCE_SYSTEM_AGC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2500744633, 34684, 19559, [190, 8, 228, 123, 86, 17, 19, 10]);
pub const KSALGORITHMINSTANCE_SYSTEM_MICROPHONE_ARRAY_PROCESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3069550752, 40545, 20364, [145, 227, 118, 207, 15, 60, 71, 31]);
pub const KSALGORITHMINSTANCE_SYSTEM_NOISE_SUPPRESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1521518638, 29300, 17686, [135, 125, 78, 238, 153, 186, 79, 208]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSALLOCATORMODE(pub i32);
pub const KsAllocatorMode_User: KSALLOCATORMODE = KSALLOCATORMODE(0i32);
pub const KsAllocatorMode_Kernel: KSALLOCATORMODE = KSALLOCATORMODE(1i32);
impl ::core::convert::From<i32> for KSALLOCATORMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSALLOCATORMODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_2D_BUFFER_REQUIRED: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_ALLOCATOR_EXISTS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_ATTENTION_STEPPING: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_CAN_ALLOCATE: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_CYCLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_DEVICE_SPECIFIC: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_ENABLE_CACHED_MDL: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_INDEPENDENT_RANGES: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_INSIST_ON_FRAMESIZE_RATIO: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_MULTIPLE_OUTPUT: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_NO_FRAME_INTEGRITY: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_FLAG_PARTIAL_READ_SUPPORT: u32 = 16u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSALLOCATOR_FRAMING {
    pub Anonymous1: KSALLOCATOR_FRAMING_0,
    pub PoolType: u32,
    pub Frames: u32,
    pub FrameSize: u32,
    pub Anonymous2: KSALLOCATOR_FRAMING_1,
    pub Reserved: u32,
}
impl KSALLOCATOR_FRAMING {}
impl ::core::default::Default for KSALLOCATOR_FRAMING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSALLOCATOR_FRAMING {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSALLOCATOR_FRAMING {}
unsafe impl ::windows::runtime::Abi for KSALLOCATOR_FRAMING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSALLOCATOR_FRAMING_0 {
    pub OptionsFlags: u32,
    pub RequirementsFlags: u32,
}
impl KSALLOCATOR_FRAMING_0 {}
impl ::core::default::Default for KSALLOCATOR_FRAMING_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSALLOCATOR_FRAMING_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSALLOCATOR_FRAMING_0 {}
unsafe impl ::windows::runtime::Abi for KSALLOCATOR_FRAMING_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSALLOCATOR_FRAMING_1 {
    pub FileAlignment: u32,
    pub FramePitch: i32,
}
impl KSALLOCATOR_FRAMING_1 {}
impl ::core::default::Default for KSALLOCATOR_FRAMING_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSALLOCATOR_FRAMING_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSALLOCATOR_FRAMING_1 {}
unsafe impl ::windows::runtime::Abi for KSALLOCATOR_FRAMING_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSALLOCATOR_FRAMING_EX {
    pub CountItems: u32,
    pub PinFlags: u32,
    pub OutputCompression: KS_COMPRESSION,
    pub PinWeight: u32,
    pub FramingItem: [KS_FRAMING_ITEM; 1],
}
impl KSALLOCATOR_FRAMING_EX {}
impl ::core::default::Default for KSALLOCATOR_FRAMING_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSALLOCATOR_FRAMING_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSALLOCATOR_FRAMING_EX {}
unsafe impl ::windows::runtime::Abi for KSALLOCATOR_FRAMING_EX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_OPTIONF_COMPATIBLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_OPTIONF_SYSTEM_MEMORY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_OPTIONF_VALID: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_REQUIREMENTF_FRAME_INTEGRITY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_REQUIREMENTF_INPLACE_MODIFIER: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_REQUIREMENTF_MUST_ALLOCATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_REQUIREMENTF_PREFERENCES_ONLY: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_REQUIREMENTF_SYSTEM_MEMORY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSALLOCATOR_REQUIREMENTF_SYSTEM_MEMORY_CUSTOM_ALLOCATION: u32 = 16u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSATTRIBUTE {
    pub Size: u32,
    pub Flags: u32,
    pub Attribute: ::windows::runtime::GUID,
}
impl KSATTRIBUTE {}
impl ::core::default::Default for KSATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSATTRIBUTE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSATTRIBUTE").field("Size", &self.Size).field("Flags", &self.Flags).field("Attribute", &self.Attribute).finish()
    }
}
impl ::core::cmp::PartialEq for KSATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.Attribute == other.Attribute
    }
}
impl ::core::cmp::Eq for KSATTRIBUTE {}
unsafe impl ::windows::runtime::Abi for KSATTRIBUTE {
    type Abi = Self;
}
pub const KSATTRIBUTEID_AUDIOSIGNALPROCESSING_MODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3791167157, 24390, 16795, [150, 123, 255, 103, 112, 185, 132, 1]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    pub AttributeHeader: KSATTRIBUTE,
    pub SignalProcessingMode: ::windows::runtime::GUID,
}
impl KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {}
impl ::core::default::Default for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE").field("AttributeHeader", &self.AttributeHeader).field("SignalProcessingMode", &self.SignalProcessingMode).finish()
    }
}
impl ::core::cmp::PartialEq for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.AttributeHeader == other.AttributeHeader && self.SignalProcessingMode == other.SignalProcessingMode
    }
}
impl ::core::cmp::Eq for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {}
unsafe impl ::windows::runtime::Abi for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSATTRIBUTE_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDDECOUTMODE_PCM_51: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDDECOUTMODE_SPDIFF: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDDECOUTMODE_STEREO_ANALOG: u32 = 1u32;
pub const KSAUDFNAME_3D_CENTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2667999412, 39199, 4562, [172, 77, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_3D_DEPTH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1677678407, 39199, 4562, [172, 77, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_3D_STEREO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940002, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_ALTERNATE_MICROPHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(734207339, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_AUX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940030, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_AUX_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940029, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_AUX_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940028, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_BASS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940000, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_CD_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940027, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_CD_IN_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940019, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_CD_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940010, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_CD_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940009, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_LINE_IN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940025, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_LINE_IN_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940020, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_LINE_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940012, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_LINE_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940011, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MASTER_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940004, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MASTER_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940003, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MICROPHONE_BOOST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(734207338, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_MIC_IN_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940021, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MIC_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940014, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MIC_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940013, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MIDI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940024, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MIDI_IN_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940018, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MIDI_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940008, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MIDI_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940007, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_MIDRANGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2731271288, 44676, 18849, [139, 114, 74, 208, 155, 120, 237, 52]);
pub const KSAUDFNAME_MONO_MIX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(14676088, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_MONO_MIX_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(734207337, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_MONO_MIX_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(582019838, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_MONO_OUT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4189330883, 38626, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_MONO_OUT_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(449988588, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_MONO_OUT_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(449988587, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_PC_SPEAKER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940031, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_PC_SPEAKER_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940017, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_PC_SPEAKER_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940016, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_PEAKMETER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1474446144, 64603, 17938, [165, 98, 114, 177, 26, 41, 223, 174]);
pub const KSAUDFNAME_RECORDING_CONTROL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940026, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_RECORDING_SOURCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940015, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_STEREO_MIX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(14676087, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_STEREO_MIX_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(582019837, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_STEREO_MIX_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(449988589, 38627, 4562, [172, 76, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_TREBLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940001, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2438835908, 42036, 4562, [172, 82, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_VIDEO_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2605115145, 39210, 4562, [172, 77, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_VIDEO_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2605115144, 39210, 4562, [172, 77, 0, 192, 79, 142, 251, 104]);
pub const KSAUDFNAME_VOLUME_CONTROL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940023, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_WAVE_IN_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940022, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_WAVE_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940006, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_WAVE_OUT_MIX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940032, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
pub const KSAUDFNAME_WAVE_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408940005, 39173, 4561, [149, 169, 0, 192, 79, 185, 37, 211]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    pub MinBufferBytes: u32,
    pub MaxBufferBytes: u32,
}
impl KSAUDIOENGINE_BUFFER_SIZE_RANGE {}
impl ::core::default::Default for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIOENGINE_BUFFER_SIZE_RANGE").field("MinBufferBytes", &self.MinBufferBytes).field("MaxBufferBytes", &self.MaxBufferBytes).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.MinBufferBytes == other.MinBufferBytes && self.MaxBufferBytes == other.MaxBufferBytes
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_BUFFER_SIZE_RANGE {}
unsafe impl ::windows::runtime::Abi for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIOENGINE_DESCRIPTOR {
    pub nHostPinId: u32,
    pub nOffloadPinId: u32,
    pub nLoopbackPinId: u32,
}
impl KSAUDIOENGINE_DESCRIPTOR {}
impl ::core::default::Default for KSAUDIOENGINE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIOENGINE_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIOENGINE_DESCRIPTOR").field("nHostPinId", &self.nHostPinId).field("nOffloadPinId", &self.nOffloadPinId).field("nLoopbackPinId", &self.nLoopbackPinId).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.nHostPinId == other.nHostPinId && self.nOffloadPinId == other.nOffloadPinId && self.nLoopbackPinId == other.nLoopbackPinId
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for KSAUDIOENGINE_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIOENGINE_VOLUMELEVEL {
    pub TargetVolume: i32,
    pub CurveType: AUDIO_CURVE_TYPE,
    pub CurveDuration: u64,
}
impl KSAUDIOENGINE_VOLUMELEVEL {}
impl ::core::default::Default for KSAUDIOENGINE_VOLUMELEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIOENGINE_VOLUMELEVEL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIOENGINE_VOLUMELEVEL").field("TargetVolume", &self.TargetVolume).field("CurveType", &self.CurveType).field("CurveDuration", &self.CurveDuration).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_VOLUMELEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.TargetVolume == other.TargetVolume && self.CurveType == other.CurveType && self.CurveDuration == other.CurveDuration
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_VOLUMELEVEL {}
unsafe impl ::windows::runtime::Abi for KSAUDIOENGINE_VOLUMELEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIOMODULE_DESCRIPTOR {
    pub ClassId: ::windows::runtime::GUID,
    pub InstanceId: u32,
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub Name: [u16; 128],
}
impl KSAUDIOMODULE_DESCRIPTOR {}
impl ::core::default::Default for KSAUDIOMODULE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIOMODULE_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIOMODULE_DESCRIPTOR").field("ClassId", &self.ClassId).field("InstanceId", &self.InstanceId).field("VersionMajor", &self.VersionMajor).field("VersionMinor", &self.VersionMinor).field("Name", &self.Name).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.ClassId == other.ClassId && self.InstanceId == other.InstanceId && self.VersionMajor == other.VersionMajor && self.VersionMinor == other.VersionMinor && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for KSAUDIOMODULE_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIOMODULE_NOTIFICATION {
    pub Anonymous: KSAUDIOMODULE_NOTIFICATION_0,
}
impl KSAUDIOMODULE_NOTIFICATION {}
impl ::core::default::Default for KSAUDIOMODULE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_NOTIFICATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for KSAUDIOMODULE_NOTIFICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSAUDIOMODULE_NOTIFICATION_0 {
    pub ProviderId: KSAUDIOMODULE_NOTIFICATION_0_0,
    pub Alignment: i64,
}
impl KSAUDIOMODULE_NOTIFICATION_0 {}
impl ::core::default::Default for KSAUDIOMODULE_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_NOTIFICATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_NOTIFICATION_0 {}
unsafe impl ::windows::runtime::Abi for KSAUDIOMODULE_NOTIFICATION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIOMODULE_NOTIFICATION_0_0 {
    pub DeviceId: ::windows::runtime::GUID,
    pub ClassId: ::windows::runtime::GUID,
    pub InstanceId: u32,
    pub Reserved: u32,
}
impl KSAUDIOMODULE_NOTIFICATION_0_0 {}
impl ::core::default::Default for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_ProviderId_e__Struct").field("DeviceId", &self.DeviceId).field("ClassId", &self.ClassId).field("InstanceId", &self.InstanceId).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceId == other.DeviceId && self.ClassId == other.ClassId && self.InstanceId == other.InstanceId && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_NOTIFICATION_0_0 {}
unsafe impl ::windows::runtime::Abi for KSAUDIOMODULE_NOTIFICATION_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIOMODULE_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub ClassId: ::windows::runtime::GUID,
    pub InstanceId: u32,
}
impl KSAUDIOMODULE_PROPERTY {}
impl ::core::default::Default for KSAUDIOMODULE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_PROPERTY {}
unsafe impl ::windows::runtime::Abi for KSAUDIOMODULE_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIO_CHANNEL_CONFIG {
    pub ActiveSpeakerPositions: i32,
}
impl KSAUDIO_CHANNEL_CONFIG {}
impl ::core::default::Default for KSAUDIO_CHANNEL_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_CHANNEL_CONFIG {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_CHANNEL_CONFIG").field("ActiveSpeakerPositions", &self.ActiveSpeakerPositions).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_CHANNEL_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.ActiveSpeakerPositions == other.ActiveSpeakerPositions
    }
}
impl ::core::cmp::Eq for KSAUDIO_CHANNEL_CONFIG {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_CHANNEL_CONFIG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSAUDIO_COPY_PROTECTION {
    pub fCopyrighted: super::super::Foundation::BOOL,
    pub fOriginal: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSAUDIO_COPY_PROTECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_COPY_PROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAUDIO_COPY_PROTECTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_COPY_PROTECTION").field("fCopyrighted", &self.fCopyrighted).field("fOriginal", &self.fOriginal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_COPY_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        self.fCopyrighted == other.fCopyrighted && self.fOriginal == other.fOriginal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_COPY_PROTECTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAUDIO_COPY_PROTECTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_CPU_RESOURCES_HOST_CPU: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_CPU_RESOURCES_NOT_HOST_CPU: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIO_DYNAMIC_RANGE {
    pub QuietCompression: u32,
    pub LoudCompression: u32,
}
impl KSAUDIO_DYNAMIC_RANGE {}
impl ::core::default::Default for KSAUDIO_DYNAMIC_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_DYNAMIC_RANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_DYNAMIC_RANGE").field("QuietCompression", &self.QuietCompression).field("LoudCompression", &self.LoudCompression).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_DYNAMIC_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.QuietCompression == other.QuietCompression && self.LoudCompression == other.LoudCompression
    }
}
impl ::core::cmp::Eq for KSAUDIO_DYNAMIC_RANGE {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_DYNAMIC_RANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIO_MICROPHONE_COORDINATES {
    pub usType: u16,
    pub wXCoord: i16,
    pub wYCoord: i16,
    pub wZCoord: i16,
    pub wVerticalAngle: i16,
    pub wHorizontalAngle: i16,
}
impl KSAUDIO_MICROPHONE_COORDINATES {}
impl ::core::default::Default for KSAUDIO_MICROPHONE_COORDINATES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_MICROPHONE_COORDINATES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_MICROPHONE_COORDINATES").field("usType", &self.usType).field("wXCoord", &self.wXCoord).field("wYCoord", &self.wYCoord).field("wZCoord", &self.wZCoord).field("wVerticalAngle", &self.wVerticalAngle).field("wHorizontalAngle", &self.wHorizontalAngle).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_MICROPHONE_COORDINATES {
    fn eq(&self, other: &Self) -> bool {
        self.usType == other.usType && self.wXCoord == other.wXCoord && self.wYCoord == other.wYCoord && self.wZCoord == other.wZCoord && self.wVerticalAngle == other.wVerticalAngle && self.wHorizontalAngle == other.wHorizontalAngle
    }
}
impl ::core::cmp::Eq for KSAUDIO_MICROPHONE_COORDINATES {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_MICROPHONE_COORDINATES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSAUDIO_MIC_ARRAY_GEOMETRY {}
impl ::core::default::Default for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_MIC_ARRAY_GEOMETRY")
            .field("usVersion", &self.usVersion)
            .field("usMicArrayType", &self.usMicArrayType)
            .field("wVerticalAngleBegin", &self.wVerticalAngleBegin)
            .field("wVerticalAngleEnd", &self.wVerticalAngleEnd)
            .field("wHorizontalAngleBegin", &self.wHorizontalAngleBegin)
            .field("wHorizontalAngleEnd", &self.wHorizontalAngleEnd)
            .field("usFrequencyBandLo", &self.usFrequencyBandLo)
            .field("usFrequencyBandHi", &self.usFrequencyBandHi)
            .field("usNumberOfMicrophones", &self.usNumberOfMicrophones)
            .field("KsMicCoord", &self.KsMicCoord)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn eq(&self, other: &Self) -> bool {
        self.usVersion == other.usVersion
            && self.usMicArrayType == other.usMicArrayType
            && self.wVerticalAngleBegin == other.wVerticalAngleBegin
            && self.wVerticalAngleEnd == other.wVerticalAngleEnd
            && self.wHorizontalAngleBegin == other.wHorizontalAngleBegin
            && self.wHorizontalAngleEnd == other.wHorizontalAngleEnd
            && self.usFrequencyBandLo == other.usFrequencyBandLo
            && self.usFrequencyBandHi == other.usFrequencyBandHi
            && self.usNumberOfMicrophones == other.usNumberOfMicrophones
            && self.KsMicCoord == other.KsMicCoord
    }
}
impl ::core::cmp::Eq for KSAUDIO_MIC_ARRAY_GEOMETRY {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_MIC_ARRAY_GEOMETRY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSAUDIO_MIXCAP_TABLE {
    pub InputChannels: u32,
    pub OutputChannels: u32,
    pub Capabilities: [KSAUDIO_MIX_CAPS; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl KSAUDIO_MIXCAP_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIXCAP_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIXCAP_TABLE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIXCAP_TABLE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAUDIO_MIXCAP_TABLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSAUDIO_MIXLEVEL {
    pub Mute: super::super::Foundation::BOOL,
    pub Level: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSAUDIO_MIXLEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIXLEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAUDIO_MIXLEVEL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_MIXLEVEL").field("Mute", &self.Mute).field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIXLEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.Mute == other.Mute && self.Level == other.Level
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIXLEVEL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAUDIO_MIXLEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSAUDIO_MIX_CAPS {
    pub Mute: super::super::Foundation::BOOL,
    pub Minimum: i32,
    pub Maximum: i32,
    pub Anonymous: KSAUDIO_MIX_CAPS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl KSAUDIO_MIX_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIX_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIX_CAPS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIX_CAPS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAUDIO_MIX_CAPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union KSAUDIO_MIX_CAPS_0 {
    pub Reset: i32,
    pub Resolution: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSAUDIO_MIX_CAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIX_CAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIX_CAPS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIX_CAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSAUDIO_MIX_CAPS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS {
    pub MinPacketPeriodInHns: u32,
    pub PacketSizeFileAlignment: u32,
    pub Reserved: u32,
    pub NumProcessingModeConstraints: u32,
    pub ProcessingModeConstraints: [_KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT; 1],
}
impl KSAUDIO_PACKETSIZE_CONSTRAINTS {}
impl ::core::default::Default for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_PACKETSIZE_CONSTRAINTS")
            .field("MinPacketPeriodInHns", &self.MinPacketPeriodInHns)
            .field("PacketSizeFileAlignment", &self.PacketSizeFileAlignment)
            .field("Reserved", &self.Reserved)
            .field("NumProcessingModeConstraints", &self.NumProcessingModeConstraints)
            .field("ProcessingModeConstraints", &self.ProcessingModeConstraints)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        self.MinPacketPeriodInHns == other.MinPacketPeriodInHns && self.PacketSizeFileAlignment == other.PacketSizeFileAlignment && self.Reserved == other.Reserved && self.NumProcessingModeConstraints == other.NumProcessingModeConstraints && self.ProcessingModeConstraints == other.ProcessingModeConstraints
    }
}
impl ::core::cmp::Eq for KSAUDIO_PACKETSIZE_CONSTRAINTS {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    pub MinPacketPeriodInHns: u32,
    pub PacketSizeFileAlignment: u32,
    pub MaxPacketSizeInBytes: u32,
    pub NumProcessingModeConstraints: u32,
    pub ProcessingModeConstraints: [_KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT; 1],
}
impl KSAUDIO_PACKETSIZE_CONSTRAINTS2 {}
impl ::core::default::Default for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_PACKETSIZE_CONSTRAINTS2")
            .field("MinPacketPeriodInHns", &self.MinPacketPeriodInHns)
            .field("PacketSizeFileAlignment", &self.PacketSizeFileAlignment)
            .field("MaxPacketSizeInBytes", &self.MaxPacketSizeInBytes)
            .field("NumProcessingModeConstraints", &self.NumProcessingModeConstraints)
            .field("ProcessingModeConstraints", &self.ProcessingModeConstraints)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn eq(&self, other: &Self) -> bool {
        self.MinPacketPeriodInHns == other.MinPacketPeriodInHns && self.PacketSizeFileAlignment == other.PacketSizeFileAlignment && self.MaxPacketSizeInBytes == other.MaxPacketSizeInBytes && self.NumProcessingModeConstraints == other.NumProcessingModeConstraints && self.ProcessingModeConstraints == other.ProcessingModeConstraints
    }
}
impl ::core::cmp::Eq for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIO_POSITION {
    pub PlayOffset: u64,
    pub WriteOffset: u64,
}
impl KSAUDIO_POSITION {}
impl ::core::default::Default for KSAUDIO_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_POSITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_POSITION").field("PlayOffset", &self.PlayOffset).field("WriteOffset", &self.WriteOffset).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.PlayOffset == other.PlayOffset && self.WriteOffset == other.WriteOffset
    }
}
impl ::core::cmp::Eq for KSAUDIO_POSITION {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_POSITION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIO_POSITIONEX {
    pub TimerFrequency: i64,
    pub TimeStamp1: i64,
    pub Position: KSAUDIO_POSITION,
    pub TimeStamp2: i64,
}
impl KSAUDIO_POSITIONEX {}
impl ::core::default::Default for KSAUDIO_POSITIONEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_POSITIONEX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_POSITIONEX").field("TimerFrequency", &self.TimerFrequency).field("TimeStamp1", &self.TimeStamp1).field("Position", &self.Position).field("TimeStamp2", &self.TimeStamp2).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_POSITIONEX {
    fn eq(&self, other: &Self) -> bool {
        self.TimerFrequency == other.TimerFrequency && self.TimeStamp1 == other.TimeStamp1 && self.Position == other.Position && self.TimeStamp2 == other.TimeStamp2
    }
}
impl ::core::cmp::Eq for KSAUDIO_POSITIONEX {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_POSITIONEX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSAUDIO_PRESENTATION_POSITION {
    pub u64PositionInBlocks: u64,
    pub u64QPCPosition: u64,
}
impl KSAUDIO_PRESENTATION_POSITION {}
impl ::core::default::Default for KSAUDIO_PRESENTATION_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSAUDIO_PRESENTATION_POSITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSAUDIO_PRESENTATION_POSITION").field("u64PositionInBlocks", &self.u64PositionInBlocks).field("u64QPCPosition", &self.u64QPCPosition).finish()
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_PRESENTATION_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.u64PositionInBlocks == other.u64PositionInBlocks && self.u64QPCPosition == other.u64QPCPosition
    }
}
impl ::core::cmp::Eq for KSAUDIO_PRESENTATION_POSITION {}
unsafe impl ::windows::runtime::Abi for KSAUDIO_PRESENTATION_POSITION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_QUALITY_ADVANCED: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_QUALITY_BASIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_QUALITY_PC: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_QUALITY_WORST: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_DIRECTOUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_GROUND_FRONT_CENTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_GROUND_FRONT_LEFT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_GROUND_FRONT_RIGHT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_GROUND_REAR_LEFT: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_GROUND_REAR_RIGHT: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_MONO: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_SUPER_WOOFER: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_SPEAKER_TOP_MIDDLE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_HEADPHONE: i32 = -1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MAX: u32 = 180u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MIN: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_NARROW: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_WIDE: u32 = 20u32;
pub const KSCAMERAPROFILE_BalancedVideoAndPhoto: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1800581143, 17095, 18977, [191, 227, 35, 240, 9, 20, 152, 135]);
pub const KSCAMERAPROFILE_CompressedCamera: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(238341569, 10157, 17279, [171, 222, 2, 182, 41, 243, 123, 68]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERAPROFILE_FLAGS_FACEDETECTION: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERAPROFILE_FLAGS_PHOTOHDR: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERAPROFILE_FLAGS_PREVIEW_RES_MUSTMATCH: u64 = 32u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERAPROFILE_FLAGS_VARIABLEPHOTOSEQUENCE: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERAPROFILE_FLAGS_VIDEOHDR: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERAPROFILE_FLAGS_VIDEOSTABLIZATION: u64 = 1u64;
pub const KSCAMERAPROFILE_FaceAuth_Mode: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2167806754, 28683, 17734, [162, 212, 197, 46, 144, 123, 252, 39]);
pub const KSCAMERAPROFILE_HDRWithWCGPhoto: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2616652287, 46421, 17957, [179, 38, 164, 109, 239, 49, 143, 183]);
pub const KSCAMERAPROFILE_HDRWithWCGVideo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1260897078, 18724, 18825, [185, 148, 253, 175, 29, 199, 205, 133]);
pub const KSCAMERAPROFILE_HighFrameRate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1450074387, 35893, 18663, [184, 159, 210, 63, 220, 18, 25, 220]);
pub const KSCAMERAPROFILE_HighQualityPhoto: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(843319077, 38427, 19619, [181, 178, 133, 78, 113, 157, 158, 27]);
pub const KSCAMERAPROFILE_Legacy: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3028897153, 25271, 20204, [135, 64, 128, 101, 140, 74, 157, 62]);
pub const KSCAMERAPROFILE_PhotoSequence: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(37330333, 20200, 18874, [188, 7, 95, 241, 86, 83, 20, 19]);
pub const KSCAMERAPROFILE_VariablePhotoSequence: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2683489110, 59226, 18865, [169, 40, 153, 133, 213, 148, 111, 135]);
pub const KSCAMERAPROFILE_VideoConferencing: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3309587080, 57791, 17815, [178, 221, 158, 30, 173, 134, 75, 184]);
pub const KSCAMERAPROFILE_VideoHDR8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3572757740, 48639, 17172, [177, 212, 0, 142, 40, 31, 116, 231]);
pub const KSCAMERAPROFILE_VideoRecording: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2699368424, 36748, 20335, [154, 87, 70, 252, 47, 100, 126, 192]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_AUTO: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_FNF: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_HDR: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_ULTRALOWLIGHT: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_BLUR: u64 = 1u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    pub Resolution: super::super::Foundation::SIZE,
    pub MaxFrameRate: KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0,
    pub MaskResolution: super::super::Foundation::SIZE,
    pub SubType: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS").field("Resolution", &self.Resolution).field("MaxFrameRate", &self.MaxFrameRate).field("MaskResolution", &self.MaskResolution).field("SubType", &self.SubType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Resolution == other.Resolution && self.MaxFrameRate == other.MaxFrameRate && self.MaskResolution == other.MaskResolution && self.SubType == other.SubType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    pub Numerator: i32,
    pub Denominator: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_MaxFrameRate_e__Struct").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_MASK: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_OFF: u64 = 0u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    pub PitchAngle: i32,
    pub YawAngle: i32,
    pub Flag: u32,
    pub Reserved: u32,
}
impl KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_CAMERAOFFSET").field("PitchAngle", &self.PitchAngle).field("YawAngle", &self.YawAngle).field("Flag", &self.Flag).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.PitchAngle == other.PitchAngle && self.YawAngle == other.YawAngle && self.Flag == other.Flag && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_CAPS_ASYNCCONTROL: u64 = 9223372036854775808u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_CAPS_CANCELLABLE: u64 = 4611686018427387904u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_CAPS_MASK: u64 = 18374686479671623680u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_CAPS_RESERVED: u64 = 18374686479671623680u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_AUTOFACEFRAMING: u64 = 1u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS")
            .field("ResolutionX", &self.ResolutionX)
            .field("ResolutionY", &self.ResolutionY)
            .field("PorchTop", &self.PorchTop)
            .field("PorchLeft", &self.PorchLeft)
            .field("PorchBottom", &self.PorchBottom)
            .field("PorchRight", &self.PorchRight)
            .field("NonUpscalingWindowSize", &self.NonUpscalingWindowSize)
            .field("MinWindowSize", &self.MinWindowSize)
            .field("MaxWindowSize", &self.MaxWindowSize)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.ResolutionX == other.ResolutionX && self.ResolutionY == other.ResolutionY && self.PorchTop == other.PorchTop && self.PorchLeft == other.PorchLeft && self.PorchBottom == other.PorchBottom && self.PorchRight == other.PorchRight && self.NonUpscalingWindowSize == other.NonUpscalingWindowSize && self.MinWindowSize == other.MinWindowSize && self.MaxWindowSize == other.MaxWindowSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    pub Size: u32,
    pub Count: u32,
}
impl KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_MANUAL: u64 = 0u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    pub OriginX: i32,
    pub OriginY: i32,
    pub WindowSize: i32,
    pub Reserved: u32,
}
impl KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING").field("OriginX", &self.OriginX).field("OriginY", &self.OriginY).field("WindowSize", &self.WindowSize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.OriginX == other.OriginX && self.OriginY == other.OriginY && self.WindowSize == other.WindowSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Value: i32,
    pub Reserved: u64,
}
impl KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_EVCOMPENSATION").field("Mode", &self.Mode).field("Min", &self.Min).field("Max", &self.Max).field("Value", &self.Value).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Min == other.Min && self.Max == other.Max && self.Value == other.Value && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_FULLSTEP: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_HALFSTEP: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_QUARTERSTEP: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_SIXTHSTEP: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_THIRDSTEP: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_ALTERNATIVE_FRAME_ILLUMINATION: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_BACKGROUND_SUBTRACTION: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_DISABLED: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_BLINK: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PHOTO: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PREVIEW: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_SMILE: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_VIDEO: u64 = 2u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    pub NormalizedFocalLengthX: u32,
    pub NormalizedFocalLengthY: u32,
    pub Flag: u32,
    pub Reserved: u32,
}
impl KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_FIELDOFVIEW").field("NormalizedFocalLengthX", &self.NormalizedFocalLengthX).field("NormalizedFocalLengthY", &self.NormalizedFocalLengthY).field("Flag", &self.Flag).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn eq(&self, other: &Self) -> bool {
        self.NormalizedFocalLengthX == other.NormalizedFocalLengthX && self.NormalizedFocalLengthY == other.NormalizedFocalLengthY && self.Flag == other.Flag && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FILTERSCOPE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLAG_CANCELOPERATION: u64 = 9223372036854775808u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLAG_MASK: u64 = 18374686479671623680u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_AUTO: u64 = 256u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_ON: u64 = 128u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO_ADJUSTABLEPOWER: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_MULTIFLASHSUPPORTED: u64 = 64u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON_ADJUSTABLEPOWER: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_REDEYEREDUCTION: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FLASH_SINGLEFLASH: u64 = 32u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSCAMERA_EXTENDEDPROP_FOCUSSTATE(pub i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_UNINITIALIZED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(0i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_LOST: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(1i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_SEARCHING: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(2i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FOCUSED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(3i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FAILED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(4i32);
impl ::core::convert::From<i32> for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUS: u64 = 256u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUSLOCK: u64 = 512u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_HYPERFOCAL: u64 = 33554432u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_INFINITY: u64 = 16777216u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_NEAREST: u64 = 67108864u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DRIVERFALLBACK_OFF: u64 = 2048u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_FULLRANGE: u64 = 262144u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_HYPERFOCAL: u64 = 1048576u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_INFINITY: u64 = 524288u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_MACRO: u64 = 65536u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_NORMAL: u64 = 131072u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_REGIONBASED: u64 = 4096u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUS_UNLOCK: u64 = 1024u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_HEADER {
    pub Version: u32,
    pub PinId: u32,
    pub Size: u32,
    pub Result: u32,
    pub Flags: u64,
    pub Capability: u64,
}
impl KSCAMERA_EXTENDEDPROP_HEADER {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_HEADER").field("Version", &self.Version).field("PinId", &self.PinId).field("Size", &self.Size).field("Result", &self.Result).field("Flags", &self.Flags).field("Capability", &self.Capability).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.PinId == other.PinId && self.Size == other.Size && self.Result == other.Result && self.Flags == other.Flags && self.Capability == other.Capability
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_HEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALTERNATING_FRAME_ILLUMINATION: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALWAYS_ON: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_OFF: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_100: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_12800: u64 = 1024u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_1600: u64 = 128u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_200: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_25600: u64 = 2048u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_3200: u64 = 256u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_400: u64 = 32u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_50: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_6400: u64 = 512u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_80: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_800: u64 = 64u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_AUTO: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ISO_MANUAL: u64 = 36028797018963968u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_METADATAINFO {
    pub BufferAlignment: i32,
    pub MaxMetadataBufferSize: u32,
}
impl KSCAMERA_EXTENDEDPROP_METADATAINFO {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_METADATAINFO").field("BufferAlignment", &self.BufferAlignment).field("MaxMetadataBufferSize", &self.MaxMetadataBufferSize).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.BufferAlignment == other.BufferAlignment && self.MaxMetadataBufferSize == other.MaxMetadataBufferSize
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_METADATAINFO {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_ALIGNMENTREQUIRED: u64 = 256u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_MEMORYTYPE_MASK: u64 = 255u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_SYSTEMMEMORY: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSCAMERA_EXTENDEDPROP_MetadataAlignment(pub i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_16: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(4i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_32: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(5i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_64: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(6i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_128: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(7i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_256: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(8i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_512: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(9i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_1024: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(10i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_2048: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(11i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_4096: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(12i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_8192: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(13i32);
impl ::core::convert::From<i32> for KSCAMERA_EXTENDEDPROP_MetadataAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_MetadataAlignment {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OIS_AUTO: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OIS_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OIS_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_DEFAULT: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_LATENCY: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PHOTO: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_POWER: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_QUALITY: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_VIDEO: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_ON: u64 = 1u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    pub RequestedHistoryFrames: u32,
    pub MaxHistoryFrames: u32,
    pub SubMode: u32,
    pub Reserved: u32,
}
impl KSCAMERA_EXTENDEDPROP_PHOTOMODE {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_PHOTOMODE").field("RequestedHistoryFrames", &self.RequestedHistoryFrames).field("MaxHistoryFrames", &self.MaxHistoryFrames).field("SubMode", &self.SubMode).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedHistoryFrames == other.RequestedHistoryFrames && self.MaxHistoryFrames == other.MaxHistoryFrames && self.SubMode == other.SubMode && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_PHOTOMODE {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_NORMAL: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_VARIABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_16X: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_2X: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_4X: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_8X: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_DISABLE: u64 = 0u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_PROFILE {
    pub ProfileId: ::windows::runtime::GUID,
    pub Index: u32,
    pub Reserved: u32,
}
impl KSCAMERA_EXTENDEDPROP_PROFILE {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_PROFILE").field("ProfileId", &self.ProfileId).field("Index", &self.Index).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ProfileId == other.ProfileId && self.Index == other.Index && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_PROFILE {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_PROFILE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_DYNAMIC: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSCAMERA_EXTENDEDPROP_ROITYPE(pub i32);
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_UNKNOWN: KSCAMERA_EXTENDEDPROP_ROITYPE = KSCAMERA_EXTENDEDPROP_ROITYPE(0i32);
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_FACE: KSCAMERA_EXTENDEDPROP_ROITYPE = KSCAMERA_EXTENDEDPROP_ROITYPE(1i32);
impl ::core::convert::From<i32> for KSCAMERA_EXTENDEDPROP_ROITYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROITYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    pub ControlId: u32,
    pub MaxNumberOfROIs: u32,
    pub Capability: u64,
}
impl KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS").field("ControlId", &self.ControlId).field("MaxNumberOfROIs", &self.MaxNumberOfROIs).field("Capability", &self.Capability).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.ControlId == other.ControlId && self.MaxNumberOfROIs == other.MaxNumberOfROIs && self.Capability == other.Capability
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    pub Size: u32,
    pub ConfigCapCount: u32,
    pub Reserved: u64,
}
impl KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER").field("Size", &self.Size).field("ConfigCapCount", &self.ConfigCapCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ConfigCapCount == other.ConfigCapCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn eq(&self, other: &Self) -> bool {
        self.ROIInfo == other.ROIInfo && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl KSCAMERA_EXTENDEDPROP_ROI_FOCUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_FOCUS").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn eq(&self, other: &Self) -> bool {
        self.ROIInfo == other.ROIInfo && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_INFO {
    pub Region: super::super::Foundation::RECT,
    pub Flags: u64,
    pub Weight: i32,
    pub RegionOfInterestType: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSCAMERA_EXTENDEDPROP_ROI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_INFO").field("Region", &self.Region).field("Flags", &self.Flags).field("Weight", &self.Weight).field("RegionOfInterestType", &self.RegionOfInterestType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Region == other.Region && self.Flags == other.Flags && self.Weight == other.Weight && self.RegionOfInterestType == other.RegionOfInterestType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    pub ControlId: u32,
    pub ROICount: u32,
    pub Result: u32,
    pub Reserved: u32,
}
impl KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL").field("ControlId", &self.ControlId).field("ROICount", &self.ROICount).field("Result", &self.Result).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.ControlId == other.ControlId && self.ROICount == other.ROICount && self.Result == other.Result && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    pub Size: u32,
    pub ControlCount: u32,
    pub Reserved: u64,
}
impl KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER").field("Size", &self.Size).field("ControlCount", &self.ControlCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ControlCount == other.ControlCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn eq(&self, other: &Self) -> bool {
        self.ROIInfo == other.ROIInfo && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_AUTO: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BACKLIT: u64 = 1024u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BEACH: u64 = 32u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_CANDLELIGHT: u64 = 128u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_LANDSCAPE: u64 = 256u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MACRO: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MANUAL: u64 = 36028797018963968u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHT: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHTPORTRAIT: u64 = 512u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_PORTRAIT: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SNOW: u64 = 8u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SPORT: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SUNSET: u64 = 64u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_DISABLED: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_ENABLED: u64 = 2u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_VALUE {
    pub Value: KSCAMERA_EXTENDEDPROP_VALUE_0,
}
impl KSCAMERA_EXTENDEDPROP_VALUE {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_VALUE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_VALUE {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_VALUE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSCAMERA_EXTENDEDPROP_VALUE_0 {
    pub dbl: f64,
    pub ull: u64,
    pub ul: u32,
    pub ratio: u64,
    pub l: i32,
    pub ll: i64,
}
impl KSCAMERA_EXTENDEDPROP_VALUE_0 {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_VALUE_0 {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VFR_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VFR_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_AUTO: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_AUTO: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_LOCK: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_MANUAL: u64 = 2u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Step: i32,
    pub VideoProc: KSCAMERA_EXTENDEDPROP_VALUE,
    pub Reserved: u64,
}
impl KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_AUTO: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_AUTO: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_OFF: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_ON: u64 = 4u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON_ADJUSTABLEPOWER: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_DISABLED: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_ENABLED: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSCAMERA_EXTENDEDPROP_WBPRESET(pub i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CLOUDY: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(1i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_DAYLIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(2i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLASH: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(3i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLUORESCENT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(4i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_TUNGSTEN: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(5i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CANDLELIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(6i32);
impl ::core::convert::From<i32> for KSCAMERA_EXTENDEDPROP_WBPRESET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_WBPRESET {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(pub i32);
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_TEMPERATURE: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(1i32);
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_PRESET: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(2i32);
impl ::core::convert::From<i32> for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DEFAULT: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DIRECT: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_SMOOTH: u64 = 2u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    pub PhotoResWidth: u32,
    pub PhotoResHeight: u32,
    pub PreviewFPSNum: u32,
    pub PreviewFPSDenom: u32,
    pub CaptureFPSNum: u32,
    pub CaptureFPSDenom: u32,
}
impl KSCAMERA_MAXVIDEOFPS_FORPHOTORES {}
impl ::core::default::Default for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_MAXVIDEOFPS_FORPHOTORES")
            .field("PhotoResWidth", &self.PhotoResWidth)
            .field("PhotoResHeight", &self.PhotoResHeight)
            .field("PreviewFPSNum", &self.PreviewFPSNum)
            .field("PreviewFPSDenom", &self.PreviewFPSDenom)
            .field("CaptureFPSNum", &self.CaptureFPSNum)
            .field("CaptureFPSDenom", &self.CaptureFPSDenom)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn eq(&self, other: &Self) -> bool {
        self.PhotoResWidth == other.PhotoResWidth && self.PhotoResHeight == other.PhotoResHeight && self.PreviewFPSNum == other.PreviewFPSNum && self.PreviewFPSDenom == other.PreviewFPSDenom && self.CaptureFPSNum == other.CaptureFPSNum && self.CaptureFPSDenom == other.CaptureFPSDenom
    }
}
impl ::core::cmp::Eq for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub MaskCoverageBoundingBox: super::super::Foundation::RECT,
    pub MaskResolution: super::super::Foundation::SIZE,
    pub ForegroundBoundingBox: super::super::Foundation::RECT,
    pub MaskData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK")
            .field("Header", &self.Header)
            .field("MaskCoverageBoundingBox", &self.MaskCoverageBoundingBox)
            .field("MaskResolution", &self.MaskResolution)
            .field("ForegroundBoundingBox", &self.ForegroundBoundingBox)
            .field("MaskData", &self.MaskData)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MaskCoverageBoundingBox == other.MaskCoverageBoundingBox && self.MaskResolution == other.MaskResolution && self.ForegroundBoundingBox == other.ForegroundBoundingBox && self.MaskData == other.MaskData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSCAMERA_METADATA_CAPTURESTATS {}
impl ::core::default::Default for KSCAMERA_METADATA_CAPTURESTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_METADATA_CAPTURESTATS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_METADATA_CAPTURESTATS")
            .field("Header", &self.Header)
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .field("ExposureTime", &self.ExposureTime)
            .field("ExposureCompensationFlags", &self.ExposureCompensationFlags)
            .field("ExposureCompensationValue", &self.ExposureCompensationValue)
            .field("IsoSpeed", &self.IsoSpeed)
            .field("FocusState", &self.FocusState)
            .field("LensPosition", &self.LensPosition)
            .field("WhiteBalance", &self.WhiteBalance)
            .field("Flash", &self.Flash)
            .field("FlashPower", &self.FlashPower)
            .field("ZoomFactor", &self.ZoomFactor)
            .field("SceneMode", &self.SceneMode)
            .field("SensorFramerate", &self.SensorFramerate)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_CAPTURESTATS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Flags == other.Flags
            && self.Reserved == other.Reserved
            && self.ExposureTime == other.ExposureTime
            && self.ExposureCompensationFlags == other.ExposureCompensationFlags
            && self.ExposureCompensationValue == other.ExposureCompensationValue
            && self.IsoSpeed == other.IsoSpeed
            && self.FocusState == other.FocusState
            && self.LensPosition == other.LensPosition
            && self.WhiteBalance == other.WhiteBalance
            && self.Flash == other.Flash
            && self.FlashPower == other.FlashPower
            && self.ZoomFactor == other.ZoomFactor
            && self.SceneMode == other.SceneMode
            && self.SensorFramerate == other.SensorFramerate
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_CAPTURESTATS {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_METADATA_CAPTURESTATS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURECOMPENSATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURETIME: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASH: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASHPOWER: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FOCUSSTATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ISOSPEED: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_LENSPOSITION: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SCENEMODE: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SENSORFRAMERATE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_WHITEBALANCE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ZOOMFACTOR: u32 = 256u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_METADATA_DIGITALWINDOW {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Window: KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING,
}
impl KSCAMERA_METADATA_DIGITALWINDOW {}
impl ::core::default::Default for KSCAMERA_METADATA_DIGITALWINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_METADATA_DIGITALWINDOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_METADATA_DIGITALWINDOW").field("Header", &self.Header).field("Window", &self.Window).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_DIGITALWINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Window == other.Window
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_DIGITALWINDOW {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_METADATA_DIGITALWINDOW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_METADATA_FRAMEILLUMINATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Flags: u32,
    pub Reserved: u32,
}
impl KSCAMERA_METADATA_FRAMEILLUMINATION {}
impl ::core::default::Default for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_METADATA_FRAMEILLUMINATION").field("Header", &self.Header).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_FRAMEILLUMINATION {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_METADATA_FRAMEILLUMINATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_FRAMEILLUMINATION_FLAG_ON: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_METADATA_ITEMHEADER {
    pub MetadataId: u32,
    pub Size: u32,
}
impl KSCAMERA_METADATA_ITEMHEADER {}
impl ::core::default::Default for KSCAMERA_METADATA_ITEMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_METADATA_ITEMHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_METADATA_ITEMHEADER").field("MetadataId", &self.MetadataId).field("Size", &self.Size).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_ITEMHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.MetadataId == other.MetadataId && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_ITEMHEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_METADATA_ITEMHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_METADATA_PHOTOCONFIRMATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub PhotoConfirmationIndex: u32,
    pub Reserved: u32,
}
impl KSCAMERA_METADATA_PHOTOCONFIRMATION {}
impl ::core::default::Default for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_METADATA_PHOTOCONFIRMATION").field("Header", &self.Header).field("PhotoConfirmationIndex", &self.PhotoConfirmationIndex).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PhotoConfirmationIndex == other.PhotoConfirmationIndex && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_PHOTOCONFIRMATION {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSCAMERA_MetadataId(pub i32);
pub const MetadataId_Standard_Start: KSCAMERA_MetadataId = KSCAMERA_MetadataId(1i32);
pub const MetadataId_PhotoConfirmation: KSCAMERA_MetadataId = KSCAMERA_MetadataId(1i32);
pub const MetadataId_UsbVideoHeader: KSCAMERA_MetadataId = KSCAMERA_MetadataId(2i32);
pub const MetadataId_CaptureStats: KSCAMERA_MetadataId = KSCAMERA_MetadataId(3i32);
pub const MetadataId_CameraExtrinsics: KSCAMERA_MetadataId = KSCAMERA_MetadataId(4i32);
pub const MetadataId_CameraIntrinsics: KSCAMERA_MetadataId = KSCAMERA_MetadataId(5i32);
pub const MetadataId_FrameIllumination: KSCAMERA_MetadataId = KSCAMERA_MetadataId(6i32);
pub const MetadataId_DigitalWindow: KSCAMERA_MetadataId = KSCAMERA_MetadataId(7i32);
pub const MetadataId_BackgroundSegmentationMask: KSCAMERA_MetadataId = KSCAMERA_MetadataId(8i32);
pub const MetadataId_Standard_End: KSCAMERA_MetadataId = KSCAMERA_MetadataId(8i32);
pub const MetadataId_Custom_Start: KSCAMERA_MetadataId = KSCAMERA_MetadataId(-2147483648i32);
impl ::core::convert::From<i32> for KSCAMERA_MetadataId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSCAMERA_MetadataId {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_PERFRAMESETTING_AUTO: u64 = 4294967296u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    pub Size: u32,
    pub ItemCount: u32,
    pub Flags: u64,
}
impl KSCAMERA_PERFRAMESETTING_CAP_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PERFRAMESETTING_CAP_HEADER").field("Size", &self.Size).field("ItemCount", &self.ItemCount).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ItemCount == other.ItemCount && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CAP_HEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
impl KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER").field("Size", &self.Size).field("Type", &self.Type).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    pub Size: u32,
    pub Reserved: u32,
    pub Id: ::windows::runtime::GUID,
}
impl KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM").field("Size", &self.Size).field("Reserved", &self.Reserved).field("Id", &self.Id).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Reserved == other.Reserved && self.Id == other.Id
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    pub Size: u32,
    pub Id: u32,
    pub ItemCount: u32,
    pub Reserved: u32,
}
impl KSCAMERA_PERFRAMESETTING_FRAME_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PERFRAMESETTING_FRAME_HEADER").field("Size", &self.Size).field("Id", &self.Id).field("ItemCount", &self.ItemCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Id == other.Id && self.ItemCount == other.ItemCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PERFRAMESETTING_HEADER {
    pub Size: u32,
    pub FrameCount: u32,
    pub Id: ::windows::runtime::GUID,
    pub Flags: u64,
    pub LoopCount: u32,
    pub Reserved: u32,
}
impl KSCAMERA_PERFRAMESETTING_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PERFRAMESETTING_HEADER").field("Size", &self.Size).field("FrameCount", &self.FrameCount).field("Id", &self.Id).field("Flags", &self.Flags).field("LoopCount", &self.LoopCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.FrameCount == other.FrameCount && self.Id == other.Id && self.Flags == other.Flags && self.LoopCount == other.LoopCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_HEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PERFRAMESETTING_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
impl KSCAMERA_PERFRAMESETTING_ITEM_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PERFRAMESETTING_ITEM_HEADER").field("Size", &self.Size).field("Type", &self.Type).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSCAMERA_PERFRAMESETTING_ITEM_TYPE(pub i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_TIME: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(1i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_FLASH: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(2i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_COMPENSATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(3i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_ISO: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(4i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_FOCUS: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(5i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_PHOTOCONFIRMATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(6i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_CUSTOM: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(7i32);
impl ::core::convert::From<i32> for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_PERFRAMESETTING_MANUAL: u64 = 8589934592u64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PROFILE_CONCURRENCYINFO {
    pub ReferenceGuid: ::windows::runtime::GUID,
    pub Reserved: u32,
    pub ProfileCount: u32,
    pub Profiles: *mut KSCAMERA_PROFILE_INFO,
}
impl KSCAMERA_PROFILE_CONCURRENCYINFO {}
impl ::core::default::Default for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PROFILE_CONCURRENCYINFO").field("ReferenceGuid", &self.ReferenceGuid).field("Reserved", &self.Reserved).field("ProfileCount", &self.ProfileCount).field("Profiles", &self.Profiles).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ReferenceGuid == other.ReferenceGuid && self.Reserved == other.Reserved && self.ProfileCount == other.ProfileCount && self.Profiles == other.Profiles
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_CONCURRENCYINFO {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PROFILE_CONCURRENCYINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PROFILE_INFO {
    pub ProfileId: ::windows::runtime::GUID,
    pub Index: u32,
    pub PinCount: u32,
    pub Pins: *mut KSCAMERA_PROFILE_PININFO,
}
impl KSCAMERA_PROFILE_INFO {}
impl ::core::default::Default for KSCAMERA_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PROFILE_INFO").field("ProfileId", &self.ProfileId).field("Index", &self.Index).field("PinCount", &self.PinCount).field("Pins", &self.Pins).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProfileId == other.ProfileId && self.Index == other.Index && self.PinCount == other.PinCount && self.Pins == other.Pins
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_INFO {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PROFILE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PROFILE_MEDIAINFO {
    pub Resolution: KSCAMERA_PROFILE_MEDIAINFO_1,
    pub MaxFrameRate: KSCAMERA_PROFILE_MEDIAINFO_0,
    pub Flags: u64,
    pub Data0: u32,
    pub Data1: u32,
    pub Data2: u32,
    pub Data3: u32,
}
impl KSCAMERA_PROFILE_MEDIAINFO {}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCAMERA_PROFILE_MEDIAINFO").field("Resolution", &self.Resolution).field("MaxFrameRate", &self.MaxFrameRate).field("Flags", &self.Flags).field("Data0", &self.Data0).field("Data1", &self.Data1).field("Data2", &self.Data2).field("Data3", &self.Data3).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Resolution == other.Resolution && self.MaxFrameRate == other.MaxFrameRate && self.Flags == other.Flags && self.Data0 == other.Data0 && self.Data1 == other.Data1 && self.Data2 == other.Data2 && self.Data3 == other.Data3
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PROFILE_MEDIAINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PROFILE_MEDIAINFO_0 {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl KSCAMERA_PROFILE_MEDIAINFO_0 {}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_MaxFrameRate_e__Struct").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO_0 {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PROFILE_MEDIAINFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PROFILE_MEDIAINFO_1 {
    pub X: u32,
    pub Y: u32,
}
impl KSCAMERA_PROFILE_MEDIAINFO_1 {}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Resolution_e__Struct").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO_1 {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PROFILE_MEDIAINFO_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PROFILE_PININFO {
    pub PinCategory: ::windows::runtime::GUID,
    pub Anonymous: KSCAMERA_PROFILE_PININFO_0,
    pub MediaInfoCount: u32,
    pub MediaInfos: *mut KSCAMERA_PROFILE_MEDIAINFO,
}
impl KSCAMERA_PROFILE_PININFO {}
impl ::core::default::Default for KSCAMERA_PROFILE_PININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_PININFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_PININFO {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PROFILE_PININFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSCAMERA_PROFILE_PININFO_0 {
    pub Anonymous: KSCAMERA_PROFILE_PININFO_0_0,
    pub Reserved: u32,
}
impl KSCAMERA_PROFILE_PININFO_0 {}
impl ::core::default::Default for KSCAMERA_PROFILE_PININFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_PININFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_PININFO_0 {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PROFILE_PININFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCAMERA_PROFILE_PININFO_0_0 {
    pub PinIndex: u16,
    pub ProfileSensorType: u16,
}
impl KSCAMERA_PROFILE_PININFO_0_0 {}
impl ::core::default::Default for KSCAMERA_PROFILE_PININFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_PININFO_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("PinIndex", &self.PinIndex).field("ProfileSensorType", &self.ProfileSensorType).finish()
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_PININFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PinIndex == other.PinIndex && self.ProfileSensorType == other.ProfileSensorType
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_PININFO_0_0 {}
unsafe impl ::windows::runtime::Abi for KSCAMERA_PROFILE_PININFO_0_0 {
    type Abi = Self;
}
pub const KSCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3214294400, 50521, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSCATEGORY_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1771351300, 37871, 4560, [163, 204, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_BRIDGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(140181248, 25294, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSCATEGORY_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1709733693, 36694, 4560, [163, 185, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_CLOCK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1394025600, 18321, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSCATEGORY_COMMUNICATIONSTRANSFORM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3474840108, 38723, 4560, [163, 238, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_CROSSBAR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2811865089, 42093, 4560, [161, 140, 0, 160, 36, 1, 220, 212]);
pub const KSCATEGORY_DATACOMPRESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(512018688, 32368, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSCATEGORY_DATADECOMPRESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(656518688, 32368, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSCATEGORY_DATATRANSFORM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(783318688, 32368, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSCATEGORY_ENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(426286070, 50052, 18685, [173, 81, 144, 229, 140, 121, 247, 11]);
pub const KSCATEGORY_ESCALANTE_PLATFORM_DRIVER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1962127016, 38760, 4561, [142, 7, 0, 160, 201, 94, 194, 46]);
pub const KSCATEGORY_FILESYSTEM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1980755294, 37719, 4560, [163, 204, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_INTERFACETRANSFORM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3474840109, 38723, 4560, [163, 238, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_MEDIUMTRANSFORM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3474840110, 38723, 4560, [163, 238, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_MICROPHONE_ARRAY_PROCESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2198488306, 41773, 18283, [190, 151, 66, 132, 86, 115, 179, 90]);
pub const KSCATEGORY_MIXER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2910886912, 31624, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSCATEGORY_MULTIPLEXER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2052973011, 417, 17708, [180, 129, 79, 162, 185, 98, 113, 232]);
pub const KSCATEGORY_NETWORK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1741278268, 27076, 4562, [135, 89, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_NETWORK_CAMERA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3089335890, 46336, 16875, [180, 243, 66, 52, 247, 245, 174, 153]);
pub const KSCATEGORY_PROXY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2548804298, 38333, 4560, [163, 234, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_QUALITY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2548804299, 38333, 4560, [163, 234, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_REALTIME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3943784444, 4296, 18788, [131, 29, 109, 203, 2, 230, 242, 63]);
pub const KSCATEGORY_RENDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1709733694, 36694, 4560, [163, 185, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_SENSOR_CAMERA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(619008727, 25891, 18423, [166, 71, 211, 70, 91, 241, 245, 202]);
pub const KSCATEGORY_SENSOR_GROUP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1721528852, 2696, 17169, [167, 243, 78, 121, 130, 14, 51, 189]);
pub const KSCATEGORY_SPLITTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(172118688, 32368, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSCATEGORY_TEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1771351302, 37871, 4560, [163, 204, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_TOPOLOGY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3718597184, 7756, 4561, [160, 80, 64, 87, 5, 193, 0, 0]);
pub const KSCATEGORY_TVAUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2811865090, 42093, 4560, [161, 140, 0, 160, 36, 1, 220, 212]);
pub const KSCATEGORY_TVTUNER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2811865088, 42093, 4560, [161, 140, 0, 160, 36, 1, 220, 212]);
pub const KSCATEGORY_VBICODEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(131782240, 8945, 4561, [169, 244, 0, 192, 79, 187, 222, 143]);
pub const KSCATEGORY_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1771351301, 37871, 4560, [163, 204, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_VIDEO_CAMERA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3845273463, 63862, 20315, [155, 85, 185, 70, 153, 196, 110, 68]);
pub const KSCATEGORY_VIRTUAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(889449156, 7974, 4561, [138, 176, 0, 160, 201, 34, 49, 150]);
pub const KSCATEGORY_VPMUX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2811865091, 42093, 4560, [161, 140, 0, 160, 36, 1, 220, 212]);
pub const KSCATEGORY_WDMAUD_USE_PIN_NAME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1201994272, 41553, 4561, [160, 80, 0, 0, 248, 0, 71, 136]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCLOCK_CREATE {
    pub CreateFlags: u32,
}
impl KSCLOCK_CREATE {}
impl ::core::default::Default for KSCLOCK_CREATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCLOCK_CREATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCLOCK_CREATE").field("CreateFlags", &self.CreateFlags).finish()
    }
}
impl ::core::cmp::PartialEq for KSCLOCK_CREATE {
    fn eq(&self, other: &Self) -> bool {
        self.CreateFlags == other.CreateFlags
    }
}
impl ::core::cmp::Eq for KSCLOCK_CREATE {}
unsafe impl ::windows::runtime::Abi for KSCLOCK_CREATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCOMPONENTID {
    pub Manufacturer: ::windows::runtime::GUID,
    pub Product: ::windows::runtime::GUID,
    pub Component: ::windows::runtime::GUID,
    pub Name: ::windows::runtime::GUID,
    pub Version: u32,
    pub Revision: u32,
}
impl KSCOMPONENTID {}
impl ::core::default::Default for KSCOMPONENTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCOMPONENTID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCOMPONENTID").field("Manufacturer", &self.Manufacturer).field("Product", &self.Product).field("Component", &self.Component).field("Name", &self.Name).field("Version", &self.Version).field("Revision", &self.Revision).finish()
    }
}
impl ::core::cmp::PartialEq for KSCOMPONENTID {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer && self.Product == other.Product && self.Component == other.Component && self.Name == other.Name && self.Version == other.Version && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for KSCOMPONENTID {}
unsafe impl ::windows::runtime::Abi for KSCOMPONENTID {
    type Abi = Self;
}
pub const KSCOMPONENTID_USBAUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2400351728, 9961, 16996, [186, 77, 57, 255, 240, 29, 148, 170]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSCORRELATED_TIME {
    pub Time: i64,
    pub SystemTime: i64,
}
impl KSCORRELATED_TIME {}
impl ::core::default::Default for KSCORRELATED_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSCORRELATED_TIME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSCORRELATED_TIME").field("Time", &self.Time).field("SystemTime", &self.SystemTime).finish()
    }
}
impl ::core::cmp::PartialEq for KSCORRELATED_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.Time == other.Time && self.SystemTime == other.SystemTime
    }
}
impl ::core::cmp::Eq for KSCORRELATED_TIME {}
unsafe impl ::windows::runtime::Abi for KSCORRELATED_TIME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCREATE_ITEM_FREEONSTOP: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCREATE_ITEM_NOPARAMETERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCREATE_ITEM_SECURITYCHANGED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCREATE_ITEM_WILDCARD: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCameraProfileSensorType_Custom: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCameraProfileSensorType_Depth: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCameraProfileSensorType_ImageSegmentation: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCameraProfileSensorType_Infrared: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCameraProfileSensorType_PoseTracking: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCameraProfileSensorType_RGB: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSDATAFORMAT {
    pub Anonymous: KSDATAFORMAT_0,
    pub Alignment: i64,
}
impl KSDATAFORMAT {}
impl ::core::default::Default for KSDATAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDATAFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSDATAFORMAT {}
unsafe impl ::windows::runtime::Abi for KSDATAFORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDATAFORMAT_0 {
    pub FormatSize: u32,
    pub Flags: u32,
    pub SampleSize: u32,
    pub Reserved: u32,
    pub MajorFormat: ::windows::runtime::GUID,
    pub SubFormat: ::windows::runtime::GUID,
    pub Specifier: ::windows::runtime::GUID,
}
impl KSDATAFORMAT_0 {}
impl ::core::default::Default for KSDATAFORMAT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSDATAFORMAT_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("FormatSize", &self.FormatSize).field("Flags", &self.Flags).field("SampleSize", &self.SampleSize).field("Reserved", &self.Reserved).field("MajorFormat", &self.MajorFormat).field("SubFormat", &self.SubFormat).field("Specifier", &self.Specifier).finish()
    }
}
impl ::core::cmp::PartialEq for KSDATAFORMAT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.FormatSize == other.FormatSize && self.Flags == other.Flags && self.SampleSize == other.SampleSize && self.Reserved == other.Reserved && self.MajorFormat == other.MajorFormat && self.SubFormat == other.SubFormat && self.Specifier == other.Specifier
    }
}
impl ::core::cmp::Eq for KSDATAFORMAT_0 {}
unsafe impl ::windows::runtime::Abi for KSDATAFORMAT_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDATAFORMAT_BIT_ATTRIBUTES: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDATAFORMAT_BIT_TEMPORAL_COMPRESSION: u32 = 0u32;
pub const KSDATAFORMAT_SPECIFIER_AC3_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272804, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SPECIFIER_ANALOGVIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(75685344, 30743, 4559, [138, 3, 0, 170, 0, 110, 203, 101]);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_AC3_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358773, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358770, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358769, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358772, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358771, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SPECIFIER_DSOUND: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1367707810, 41348, 4560, [133, 34, 0, 192, 79, 217, 186, 243]);
pub const KSDATAFORMAT_SPECIFIER_FILEHANDLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1709733692, 36694, 4560, [163, 185, 0, 160, 201, 34, 49, 150]);
pub const KSDATAFORMAT_SPECIFIER_FILENAME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860088128, 59764, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSDATAFORMAT_SPECIFIER_H264_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(538426885, 26153, 16968, [170, 237, 126, 26, 71, 188, 155, 156]);
pub const KSDATAFORMAT_SPECIFIER_IMAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1764729721, 54248, 18001, [181, 180, 11, 148, 176, 19, 238, 175]);
pub const KSDATAFORMAT_SPECIFIER_JPEG_IMAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1764729721, 54248, 18001, [181, 180, 11, 148, 176, 19, 238, 175]);
pub const KSDATAFORMAT_SPECIFIER_LPCM_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272806, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SPECIFIER_MPEG1_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89694082, 50006, 4558, [191, 1, 0, 170, 0, 85, 89, 90]);
pub const KSDATAFORMAT_SPECIFIER_MPEG2_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272805, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SPECIFIER_MPEG2_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272803, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SPECIFIER_NONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(258217942, 49944, 4560, [164, 63, 0, 160, 201, 34, 49, 150]);
pub const KSDATAFORMAT_SPECIFIER_VBI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4146755296, 60170, 4560, [172, 228, 0, 0, 192, 204, 22, 186]);
pub const KSDATAFORMAT_SPECIFIER_VC_ID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2912473476, 43715, 4560, [164, 28, 0, 160, 201, 34, 49, 150]);
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89694080, 50006, 4558, [191, 1, 0, 170, 0, 85, 89, 90]);
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4146755232, 60170, 4560, [172, 228, 0, 0, 192, 204, 22, 186]);
pub const KSDATAFORMAT_SPECIFIER_WAVEFORMATEX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89694081, 50006, 4558, [191, 1, 0, 170, 0, 85, 89, 90]);
pub const KSDATAFORMAT_SUBTYPE_AC3_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272620, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SUBTYPE_ANALOG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1840918928, 26557, 4559, [160, 247, 0, 32, 175, 209, 86, 228]);
pub const KSDATAFORMAT_SUBTYPE_CC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(857820353, 287, 4562, [180, 177, 0, 160, 209, 2, 207, 190]);
pub const KSDATAFORMAT_SUBTYPE_D16: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(80, 4, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_DSS_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2695843714, 57699, 4560, [186, 217, 0, 96, 151, 68, 17, 26]);
pub const KSDATAFORMAT_SUBTYPE_DSS_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2695843713, 57699, 4560, [186, 217, 0, 96, 151, 68, 17, 26]);
pub const KSDATAFORMAT_SUBTYPE_DTS_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272627, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_AAC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(6, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ATRAC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(8, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(146, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(10, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS_ATMOS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(266, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT20: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(268, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(780, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MLP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(12, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(13, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(8, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(267, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(779, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS_HD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(11, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(5, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ONE_BIT_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(9, 3306, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_WMA_PRO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(356, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_IMAGE_RGB32: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(22, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_JPEG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(434415018, 22114, 20421, [160, 192, 23, 88, 2, 142, 16, 87]);
pub const KSDATAFORMAT_SUBTYPE_L16: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(81, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_L16_CUSTOM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(81, 32768, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_L16_IR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(81, 2, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_L8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(50, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_L8_CUSTOM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(50, 32768, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_L8_IR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(50, 2, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_LPCM_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272626, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SUBTYPE_Line21_BytePair: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1854753314, 12556, 4560, [183, 154, 0, 170, 0, 55, 103, 167]);
pub const KSDATAFORMAT_SUBTYPE_Line21_GOPPacket: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1854753315, 12556, 4560, [183, 154, 0, 170, 0, 55, 103, 167]);
pub const KSDATAFORMAT_SUBTYPE_MIDI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(489039712, 59735, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSDATAFORMAT_SUBTYPE_MIDI_BUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(748773280, 27902, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSDATAFORMAT_SUBTYPE_MJPG_CUSTOM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1196444237, 32768, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_MJPG_DEPTH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1196444237, 4, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_MJPG_IR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1196444237, 2, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Packet: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804480, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Payload: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804481, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Video: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804486, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const KSDATAFORMAT_SUBTYPE_MPEG2_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272619, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SUBTYPE_MPEG2_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272614, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SUBTYPE_MPEGLAYER3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(85, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_MPEG_HEAAC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(5648, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_NABTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4146755298, 60170, 4560, [172, 228, 0, 0, 192, 204, 22, 186]);
pub const KSDATAFORMAT_SUBTYPE_NABTS_FEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881286817, 14764, 4561, [169, 245, 0, 192, 79, 187, 222, 143]);
pub const KSDATAFORMAT_SUBTYPE_NONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804494, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const KSDATAFORMAT_SUBTYPE_OVERLAY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804479, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const KSDATAFORMAT_SUBTYPE_PCM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_RAW8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3391150496, 15934, 4561, [155, 249, 0, 192, 79, 187, 222, 191]);
pub const KSDATAFORMAT_SUBTYPE_RIFF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1234557678, 40678, 4560, [164, 14, 0, 160, 201, 34, 49, 150]);
pub const KSDATAFORMAT_SUBTYPE_RIFFMIDI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1234557680, 40678, 4560, [164, 14, 0, 160, 201, 34, 49, 150]);
pub const KSDATAFORMAT_SUBTYPE_RIFFWAVE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804491, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const KSDATAFORMAT_SUBTYPE_SDDS_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272628, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_AC3_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358757, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358754, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358753, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358756, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358755, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_SUBTYPE_SUBPICTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272621, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_SUBTYPE_TELETEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4146755299, 60170, 4560, [172, 228, 0, 0, 192, 204, 22, 186]);
pub const KSDATAFORMAT_SUBTYPE_VPVBI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1520134721, 6690, 4561, [186, 217, 0, 96, 151, 68, 17, 26]);
pub const KSDATAFORMAT_SUBTYPE_VPVideo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1520134720, 6690, 4561, [186, 217, 0, 96, 151, 68, 17, 26]);
pub const KSDATAFORMAT_SUBTYPE_WAVEFORMATEX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(0, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(354, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO_LOSSLESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(355, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_TYPE_ANALOGAUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(75685601, 30743, 4559, [138, 3, 0, 170, 0, 110, 203, 101]);
pub const KSDATAFORMAT_TYPE_ANALOGVIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(75685345, 30743, 4559, [138, 3, 0, 170, 0, 110, 203, 101]);
pub const KSDATAFORMAT_TYPE_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935963489, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_TYPE_AUXLine21Data: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1728768640, 14978, 4560, [183, 155, 0, 170, 0, 55, 103, 167]);
pub const KSDATAFORMAT_TYPE_DVD_ENCRYPTED_PACK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3976958314, 1101, 4561, [170, 120, 0, 192, 79, 195, 29, 96]);
pub const KSDATAFORMAT_TYPE_IMAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1914145827, 58459, 4565, [188, 42, 0, 176, 208, 243, 244, 171]);
pub const KSDATAFORMAT_TYPE_MIDI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935960429, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_TYPE_MPEG2_PES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272608, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_TYPE_MPEG2_PROGRAM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272610, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_TYPE_MPEG2_TRANSPORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272611, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const KSDATAFORMAT_TYPE_MUSIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3878015840, 25292, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSDATAFORMAT_TYPE_NABTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881286816, 14764, 4561, [169, 245, 0, 192, 79, 187, 222, 143]);
pub const KSDATAFORMAT_TYPE_STANDARD_ELEMENTARY_STREAM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358737, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_TYPE_STANDARD_PACK_HEADER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358739, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_TYPE_STANDARD_PES_PACKET: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911358738, 36581, 4561, [140, 163, 0, 96, 176, 87, 102, 74]);
pub const KSDATAFORMAT_TYPE_STREAM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804483, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const KSDATAFORMAT_TYPE_TEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1937012852, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_TYPE_VBI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4146755297, 60170, 4560, [172, 228, 0, 0, 192, 204, 22, 186]);
pub const KSDATAFORMAT_TYPE_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935960438, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDATARANGE_AUDIO {
    pub DataRange: KSDATAFORMAT,
    pub MaximumChannels: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
}
impl KSDATARANGE_AUDIO {}
impl ::core::default::Default for KSDATARANGE_AUDIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDATARANGE_AUDIO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSDATARANGE_AUDIO {}
unsafe impl ::windows::runtime::Abi for KSDATARANGE_AUDIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDATARANGE_BIT_ATTRIBUTES: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDATARANGE_BIT_REQUIRED_ATTRIBUTES: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDATARANGE_MUSIC {
    pub DataRange: KSDATAFORMAT,
    pub Technology: ::windows::runtime::GUID,
    pub Channels: u32,
    pub Notes: u32,
    pub ChannelMask: u32,
}
impl KSDATARANGE_MUSIC {}
impl ::core::default::Default for KSDATARANGE_MUSIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDATARANGE_MUSIC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSDATARANGE_MUSIC {}
unsafe impl ::windows::runtime::Abi for KSDATARANGE_MUSIC {
    type Abi = Self;
}
pub const KSDEGRADESETID_Standard: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2673230208, 28748, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSDEGRADE_STANDARD(pub i32);
pub const KSDEGRADE_STANDARD_SAMPLE: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(0i32);
pub const KSDEGRADE_STANDARD_QUALITY: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(1i32);
pub const KSDEGRADE_STANDARD_COMPUTATION: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(2i32);
pub const KSDEGRADE_STANDARD_SKIP: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(3i32);
impl ::core::convert::From<i32> for KSDEGRADE_STANDARD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSDEGRADE_STANDARD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_DESCRIPTOR_VERSION: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_DESCRIPTOR_VERSION_2: u32 = 272u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_FLAG_ENABLE_QUERYINTERFACE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_FLAG_ENABLE_REMOTE_WAKEUP: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_FLAG_LOWPOWER_PASSTHROUGH: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDEVICE_PROFILE_INFO {
    pub Type: u32,
    pub Size: u32,
    pub Anonymous: KSDEVICE_PROFILE_INFO_0,
}
impl KSDEVICE_PROFILE_INFO {}
impl ::core::default::Default for KSDEVICE_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDEVICE_PROFILE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSDEVICE_PROFILE_INFO {}
unsafe impl ::windows::runtime::Abi for KSDEVICE_PROFILE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSDEVICE_PROFILE_INFO_0 {
    pub Camera: KSDEVICE_PROFILE_INFO_0_0,
}
impl KSDEVICE_PROFILE_INFO_0 {}
impl ::core::default::Default for KSDEVICE_PROFILE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDEVICE_PROFILE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSDEVICE_PROFILE_INFO_0 {}
unsafe impl ::windows::runtime::Abi for KSDEVICE_PROFILE_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDEVICE_PROFILE_INFO_0_0 {
    pub Info: KSCAMERA_PROFILE_INFO,
    pub Reserved: u32,
    pub ConcurrencyCount: u32,
    pub Concurrency: *mut KSCAMERA_PROFILE_CONCURRENCYINFO,
}
impl KSDEVICE_PROFILE_INFO_0_0 {}
impl ::core::default::Default for KSDEVICE_PROFILE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSDEVICE_PROFILE_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Camera_e__Struct").field("Info", &self.Info).field("Reserved", &self.Reserved).field("ConcurrencyCount", &self.ConcurrencyCount).field("Concurrency", &self.Concurrency).finish()
    }
}
impl ::core::cmp::PartialEq for KSDEVICE_PROFILE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Info == other.Info && self.Reserved == other.Reserved && self.ConcurrencyCount == other.ConcurrencyCount && self.Concurrency == other.Concurrency
    }
}
impl ::core::cmp::Eq for KSDEVICE_PROFILE_INFO_0_0 {}
unsafe impl ::windows::runtime::Abi for KSDEVICE_PROFILE_INFO_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_PROFILE_TYPE_CAMERA: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_PROFILE_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSDEVICE_THERMAL_STATE(pub i32);
pub const KSDEVICE_THERMAL_STATE_LOW: KSDEVICE_THERMAL_STATE = KSDEVICE_THERMAL_STATE(0i32);
pub const KSDEVICE_THERMAL_STATE_HIGH: KSDEVICE_THERMAL_STATE = KSDEVICE_THERMAL_STATE(1i32);
impl ::core::convert::From<i32> for KSDEVICE_THERMAL_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSDEVICE_THERMAL_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDISPATCH_FASTIO: u32 = 2147483648u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDISPLAYCHANGE {
    pub PelsWidth: u32,
    pub PelsHeight: u32,
    pub BitsPerPel: u32,
    pub DeviceID: [u16; 1],
}
impl KSDISPLAYCHANGE {}
impl ::core::default::Default for KSDISPLAYCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSDISPLAYCHANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSDISPLAYCHANGE").field("PelsWidth", &self.PelsWidth).field("PelsHeight", &self.PelsHeight).field("BitsPerPel", &self.BitsPerPel).field("DeviceID", &self.DeviceID).finish()
    }
}
impl ::core::cmp::PartialEq for KSDISPLAYCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.PelsWidth == other.PelsWidth && self.PelsHeight == other.PelsHeight && self.BitsPerPel == other.BitsPerPel && self.DeviceID == other.DeviceID
    }
}
impl ::core::cmp::Eq for KSDISPLAYCHANGE {}
unsafe impl ::windows::runtime::Abi for KSDISPLAYCHANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSDS3D_BUFFER_ALL {}
impl ::core::default::Default for KSDS3D_BUFFER_ALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDS3D_BUFFER_ALL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSDS3D_BUFFER_ALL {}
unsafe impl ::windows::runtime::Abi for KSDS3D_BUFFER_ALL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDS3D_BUFFER_CONE_ANGLES {
    pub InsideConeAngle: u32,
    pub OutsideConeAngle: u32,
}
impl KSDS3D_BUFFER_CONE_ANGLES {}
impl ::core::default::Default for KSDS3D_BUFFER_CONE_ANGLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSDS3D_BUFFER_CONE_ANGLES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSDS3D_BUFFER_CONE_ANGLES").field("InsideConeAngle", &self.InsideConeAngle).field("OutsideConeAngle", &self.OutsideConeAngle).finish()
    }
}
impl ::core::cmp::PartialEq for KSDS3D_BUFFER_CONE_ANGLES {
    fn eq(&self, other: &Self) -> bool {
        self.InsideConeAngle == other.InsideConeAngle && self.OutsideConeAngle == other.OutsideConeAngle
    }
}
impl ::core::cmp::Eq for KSDS3D_BUFFER_CONE_ANGLES {}
unsafe impl ::windows::runtime::Abi for KSDS3D_BUFFER_CONE_ANGLES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSDS3D_HRTF_COEFF_FORMAT(pub i32);
pub const FLOAT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(0i32);
pub const SHORT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(1i32);
pub const KSDS3D_COEFF_COUNT: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(2i32);
impl ::core::convert::From<i32> for KSDS3D_HRTF_COEFF_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSDS3D_HRTF_COEFF_FORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDS3D_HRTF_FILTER_FORMAT_MSG {
    pub FilterMethod: KSDS3D_HRTF_FILTER_METHOD,
    pub CoeffFormat: KSDS3D_HRTF_COEFF_FORMAT,
    pub Version: KSDS3D_HRTF_FILTER_VERSION,
    pub Reserved: u32,
}
impl KSDS3D_HRTF_FILTER_FORMAT_MSG {}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSDS3D_HRTF_FILTER_FORMAT_MSG").field("FilterMethod", &self.FilterMethod).field("CoeffFormat", &self.CoeffFormat).field("Version", &self.Version).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.FilterMethod == other.FilterMethod && self.CoeffFormat == other.CoeffFormat && self.Version == other.Version && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSDS3D_HRTF_FILTER_FORMAT_MSG {}
unsafe impl ::windows::runtime::Abi for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSDS3D_HRTF_FILTER_METHOD(pub i32);
pub const DIRECT_FORM: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(0i32);
pub const CASCADE_FORM: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(1i32);
pub const KSDS3D_FILTER_METHOD_COUNT: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(2i32);
impl ::core::convert::From<i32> for KSDS3D_HRTF_FILTER_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSDS3D_HRTF_FILTER_METHOD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSDS3D_HRTF_FILTER_QUALITY(pub i32);
pub const FULL_FILTER: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(0i32);
pub const LIGHT_FILTER: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(1i32);
pub const KSDS3D_FILTER_QUALITY_COUNT: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(2i32);
impl ::core::convert::From<i32> for KSDS3D_HRTF_FILTER_QUALITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSDS3D_HRTF_FILTER_QUALITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSDS3D_HRTF_FILTER_VERSION(pub i32);
pub const DS3D_HRTF_VERSION_1: KSDS3D_HRTF_FILTER_VERSION = KSDS3D_HRTF_FILTER_VERSION(0i32);
impl ::core::convert::From<i32> for KSDS3D_HRTF_FILTER_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSDS3D_HRTF_FILTER_VERSION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSDS3D_HRTF_INIT_MSG {}
impl ::core::default::Default for KSDS3D_HRTF_INIT_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSDS3D_HRTF_INIT_MSG {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSDS3D_HRTF_INIT_MSG")
            .field("Size", &self.Size)
            .field("Quality", &self.Quality)
            .field("SampleRate", &self.SampleRate)
            .field("MaxFilterSize", &self.MaxFilterSize)
            .field("FilterTransientMuteLength", &self.FilterTransientMuteLength)
            .field("FilterOverlapBufferLength", &self.FilterOverlapBufferLength)
            .field("OutputOverlapBufferLength", &self.OutputOverlapBufferLength)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSDS3D_HRTF_INIT_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Quality == other.Quality && self.SampleRate == other.SampleRate && self.MaxFilterSize == other.MaxFilterSize && self.FilterTransientMuteLength == other.FilterTransientMuteLength && self.FilterOverlapBufferLength == other.FilterOverlapBufferLength && self.OutputOverlapBufferLength == other.OutputOverlapBufferLength && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSDS3D_HRTF_INIT_MSG {}
unsafe impl ::windows::runtime::Abi for KSDS3D_HRTF_INIT_MSG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSDS3D_HRTF_PARAMS_MSG {
    pub Size: u32,
    pub Enabled: u32,
    pub SwapChannels: super::super::Foundation::BOOL,
    pub ZeroAzimuth: super::super::Foundation::BOOL,
    pub CrossFadeOutput: super::super::Foundation::BOOL,
    pub FilterSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSDS3D_HRTF_PARAMS_MSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSDS3D_HRTF_PARAMS_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSDS3D_HRTF_PARAMS_MSG {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSDS3D_HRTF_PARAMS_MSG").field("Size", &self.Size).field("Enabled", &self.Enabled).field("SwapChannels", &self.SwapChannels).field("ZeroAzimuth", &self.ZeroAzimuth).field("CrossFadeOutput", &self.CrossFadeOutput).field("FilterSize", &self.FilterSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSDS3D_HRTF_PARAMS_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Enabled == other.Enabled && self.SwapChannels == other.SwapChannels && self.ZeroAzimuth == other.ZeroAzimuth && self.CrossFadeOutput == other.CrossFadeOutput && self.FilterSize == other.FilterSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSDS3D_HRTF_PARAMS_MSG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSDS3D_HRTF_PARAMS_MSG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDS3D_ITD_PARAMS {
    pub Channel: i32,
    pub VolSmoothScale: f32,
    pub TotalDryAttenuation: f32,
    pub TotalWetAttenuation: f32,
    pub SmoothFrequency: i32,
    pub Delay: i32,
}
impl KSDS3D_ITD_PARAMS {}
impl ::core::default::Default for KSDS3D_ITD_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSDS3D_ITD_PARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSDS3D_ITD_PARAMS")
            .field("Channel", &self.Channel)
            .field("VolSmoothScale", &self.VolSmoothScale)
            .field("TotalDryAttenuation", &self.TotalDryAttenuation)
            .field("TotalWetAttenuation", &self.TotalWetAttenuation)
            .field("SmoothFrequency", &self.SmoothFrequency)
            .field("Delay", &self.Delay)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSDS3D_ITD_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Channel == other.Channel && self.VolSmoothScale == other.VolSmoothScale && self.TotalDryAttenuation == other.TotalDryAttenuation && self.TotalWetAttenuation == other.TotalWetAttenuation && self.SmoothFrequency == other.SmoothFrequency && self.Delay == other.Delay
    }
}
impl ::core::cmp::Eq for KSDS3D_ITD_PARAMS {}
unsafe impl ::windows::runtime::Abi for KSDS3D_ITD_PARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDS3D_ITD_PARAMS_MSG {
    pub Enabled: u32,
    pub LeftParams: KSDS3D_ITD_PARAMS,
    pub RightParams: KSDS3D_ITD_PARAMS,
    pub Reserved: u32,
}
impl KSDS3D_ITD_PARAMS_MSG {}
impl ::core::default::Default for KSDS3D_ITD_PARAMS_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSDS3D_ITD_PARAMS_MSG {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSDS3D_ITD_PARAMS_MSG").field("Enabled", &self.Enabled).field("LeftParams", &self.LeftParams).field("RightParams", &self.RightParams).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSDS3D_ITD_PARAMS_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.Enabled == other.Enabled && self.LeftParams == other.LeftParams && self.RightParams == other.RightParams && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSDS3D_ITD_PARAMS_MSG {}
unsafe impl ::windows::runtime::Abi for KSDS3D_ITD_PARAMS_MSG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDS3D_LISTENER_ALL {
    pub Position: DS3DVECTOR,
    pub Velocity: DS3DVECTOR,
    pub OrientFront: DS3DVECTOR,
    pub OrientTop: DS3DVECTOR,
    pub DistanceFactor: f32,
    pub RolloffFactor: f32,
    pub DopplerFactor: f32,
}
impl KSDS3D_LISTENER_ALL {}
impl ::core::default::Default for KSDS3D_LISTENER_ALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDS3D_LISTENER_ALL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSDS3D_LISTENER_ALL {}
unsafe impl ::windows::runtime::Abi for KSDS3D_LISTENER_ALL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSDS3D_LISTENER_ORIENTATION {
    pub Front: DS3DVECTOR,
    pub Top: DS3DVECTOR,
}
impl KSDS3D_LISTENER_ORIENTATION {}
impl ::core::default::Default for KSDS3D_LISTENER_ORIENTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDS3D_LISTENER_ORIENTATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSDS3D_LISTENER_ORIENTATION {}
unsafe impl ::windows::runtime::Abi for KSDS3D_LISTENER_ORIENTATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_3D_MODE_DISABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_3D_MODE_HEADRELATIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_3D_MODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_CTRL_3D: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_CTRL_FREQUENCY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_CTRL_HRTF_3D: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_CTRL_PAN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_CTRL_POSITIONNOTIFY: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_CTRL_VOLUME: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_LOCHARDWARE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_LOCSOFTWARE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDSOUND_BUFFER_STATIC: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSERROR {
    pub Context: *mut ::core::ffi::c_void,
    pub Status: u32,
}
impl KSERROR {}
impl ::core::default::Default for KSERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSERROR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSERROR").field("Context", &self.Context).field("Status", &self.Status).finish()
    }
}
impl ::core::cmp::PartialEq for KSERROR {
    fn eq(&self, other: &Self) -> bool {
        self.Context == other.Context && self.Status == other.Status
    }
}
impl ::core::cmp::Eq for KSERROR {}
unsafe impl ::windows::runtime::Abi for KSERROR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSEVENTDATA {
    pub NotificationType: u32,
    pub Anonymous: KSEVENTDATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl KSEVENTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSEVENTDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union KSEVENTDATA_0 {
    pub EventHandle: KSEVENTDATA_0_1,
    pub SemaphoreHandle: KSEVENTDATA_0_2,
    pub Alignment: KSEVENTDATA_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl KSEVENTDATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSEVENTDATA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENTDATA_0_0 {
    pub Unused: *mut ::core::ffi::c_void,
    pub Alignment: [isize; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl KSEVENTDATA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSEVENTDATA_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Alignment_e__Struct").field("Unused", &self.Unused).field("Alignment", &self.Alignment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Unused == other.Unused && self.Alignment == other.Alignment
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSEVENTDATA_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENTDATA_0_1 {
    pub Event: super::super::Foundation::HANDLE,
    pub Reserved: [usize; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl KSEVENTDATA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSEVENTDATA_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_EventHandle_e__Struct").field("Event", &self.Event).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Event == other.Event && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSEVENTDATA_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENTDATA_0_2 {
    pub Semaphore: super::super::Foundation::HANDLE,
    pub Reserved: u32,
    pub Adjustment: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSEVENTDATA_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSEVENTDATA_0_2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_SemaphoreHandle_e__Struct").field("Semaphore", &self.Semaphore).field("Reserved", &self.Reserved).field("Adjustment", &self.Adjustment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Semaphore == other.Semaphore && self.Reserved == other.Reserved && self.Adjustment == other.Adjustment
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSEVENTDATA_0_2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENTF_DPC: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENTF_EVENT_HANDLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENTF_EVENT_OBJECT: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENTF_KSWORKITEM: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENTF_SEMAPHORE_HANDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENTF_SEMAPHORE_OBJECT: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENTF_WORKITEM: u32 = 32u32;
pub const KSEVENTSETID_AudioControlChange: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3898513048, 64047, 4561, [149, 189, 0, 192, 79, 185, 37, 211]);
pub const KSEVENTSETID_CameraAsyncControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(580982612, 38657, 16520, [179, 63, 107, 156, 188, 82, 223, 94]);
pub const KSEVENTSETID_CameraEvent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2023338720, 27459, 18788, [157, 42, 162, 31, 64, 97, 245, 118]);
pub const KSEVENTSETID_Clock: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911052320, 25287, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSEVENTSETID_Connection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2135673824, 40613, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSEVENTSETID_Device: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(679646956, 40852, 16820, [161, 83, 170, 49, 174, 236, 179, 63]);
pub const KSEVENTSETID_DynamicFormatChange: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(371901526, 33751, 16953, [150, 223, 199, 95, 250, 19, 139, 198]);
pub const KSEVENTSETID_EXTDEV_Command: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(278690184, 46027, 4562, [180, 142, 0, 96, 151, 179, 57, 27]);
pub const KSEVENTSETID_ExtendedCameraControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1461490377, 5026, 18403, [166, 73, 210, 167, 120, 22, 99, 132]);
pub const KSEVENTSETID_LoopedStreaming: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1182972224, 50927, 4560, [150, 216, 0, 170, 0, 81, 229, 29]);
pub const KSEVENTSETID_PinCapsChange: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3712948526, 15224, 18861, [165, 52, 44, 49, 91, 130, 32, 0]);
pub const KSEVENTSETID_SoundDetector: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1769495707, 64557, 18902, [172, 50, 71, 153, 248, 125, 233, 246]);
pub const KSEVENTSETID_StreamAllocator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1977177457, 1852, 4560, [161, 97, 0, 32, 175, 209, 86, 228]);
pub const KSEVENTSETID_Telephony: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3078558388, 52916, 17540, [141, 94, 82, 193, 231, 216, 118, 45]);
pub const KSEVENTSETID_VIDCAPTOSTI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3678920224, 63016, 4561, [186, 65, 0, 160, 201, 13, 43, 5]);
pub const KSEVENTSETID_VIDCAP_TVAUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401169, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const KSEVENTSETID_VPNotify: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(549804430, 54216, 4560, [141, 252, 0, 192, 79, 215, 192, 139]);
pub const KSEVENTSETID_VPVBINotify: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3964836609, 6687, 4561, [186, 217, 0, 96, 151, 68, 17, 26]);
pub const KSEVENTSETID_VolumeLimit: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3658908773, 14972, 18520, [157, 74, 62, 142, 36, 112, 26, 239]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_AUDIO_CONTROL_CHANGE(pub i32);
pub const KSEVENT_CONTROL_CHANGE: KSEVENT_AUDIO_CONTROL_CHANGE = KSEVENT_AUDIO_CONTROL_CHANGE(0i32);
impl ::core::convert::From<i32> for KSEVENT_AUDIO_CONTROL_CHANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_AUDIO_CONTROL_CHANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_CAMERACONTROL(pub i32);
pub const KSEVENT_CAMERACONTROL_FOCUS: KSEVENT_CAMERACONTROL = KSEVENT_CAMERACONTROL(0i32);
pub const KSEVENT_CAMERACONTROL_ZOOM: KSEVENT_CAMERACONTROL = KSEVENT_CAMERACONTROL(1i32);
impl ::core::convert::From<i32> for KSEVENT_CAMERACONTROL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_CAMERACONTROL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_CAMERAEVENT(pub i32);
pub const KSEVENT_PHOTO_SAMPLE_SCANNED: KSEVENT_CAMERAEVENT = KSEVENT_CAMERAEVENT(0i32);
impl ::core::convert::From<i32> for KSEVENT_CAMERAEVENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_CAMERAEVENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_CLOCK_POSITION(pub i32);
pub const KSEVENT_CLOCK_INTERVAL_MARK: KSEVENT_CLOCK_POSITION = KSEVENT_CLOCK_POSITION(0i32);
pub const KSEVENT_CLOCK_POSITION_MARK: KSEVENT_CLOCK_POSITION = KSEVENT_CLOCK_POSITION(1i32);
impl ::core::convert::From<i32> for KSEVENT_CLOCK_POSITION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_CLOCK_POSITION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_CONNECTION(pub i32);
pub const KSEVENT_CONNECTION_POSITIONUPDATE: KSEVENT_CONNECTION = KSEVENT_CONNECTION(0i32);
pub const KSEVENT_CONNECTION_DATADISCONTINUITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(1i32);
pub const KSEVENT_CONNECTION_TIMEDISCONTINUITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(2i32);
pub const KSEVENT_CONNECTION_PRIORITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(3i32);
pub const KSEVENT_CONNECTION_ENDOFSTREAM: KSEVENT_CONNECTION = KSEVENT_CONNECTION(4i32);
impl ::core::convert::From<i32> for KSEVENT_CONNECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_CONNECTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_CROSSBAR(pub i32);
pub const KSEVENT_CROSSBAR_CHANGED: KSEVENT_CROSSBAR = KSEVENT_CROSSBAR(0i32);
impl ::core::convert::From<i32> for KSEVENT_CROSSBAR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_CROSSBAR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_DEVCMD(pub i32);
pub const KSEVENT_EXTDEV_COMMAND_NOTIFY_INTERIM_READY: KSEVENT_DEVCMD = KSEVENT_DEVCMD(0i32);
pub const KSEVENT_EXTDEV_COMMAND_CONTROL_INTERIM_READY: KSEVENT_DEVCMD = KSEVENT_DEVCMD(1i32);
pub const KSEVENT_EXTDEV_COMMAND_BUSRESET: KSEVENT_DEVCMD = KSEVENT_DEVCMD(2i32);
pub const KSEVENT_EXTDEV_TIMECODE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(3i32);
pub const KSEVENT_EXTDEV_OPERATION_MODE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(4i32);
pub const KSEVENT_EXTDEV_TRANSPORT_STATE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(5i32);
pub const KSEVENT_EXTDEV_NOTIFY_REMOVAL: KSEVENT_DEVCMD = KSEVENT_DEVCMD(6i32);
pub const KSEVENT_EXTDEV_NOTIFY_MEDIUM_CHANGE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(7i32);
impl ::core::convert::From<i32> for KSEVENT_DEVCMD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_DEVCMD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_DEVICE(pub i32);
pub const KSEVENT_DEVICE_LOST: KSEVENT_DEVICE = KSEVENT_DEVICE(0i32);
pub const KSEVENT_DEVICE_PREEMPTED: KSEVENT_DEVICE = KSEVENT_DEVICE(1i32);
pub const KSEVENT_DEVICE_THERMAL_HIGH: KSEVENT_DEVICE = KSEVENT_DEVICE(2i32);
pub const KSEVENT_DEVICE_THERMAL_LOW: KSEVENT_DEVICE = KSEVENT_DEVICE(3i32);
impl ::core::convert::From<i32> for KSEVENT_DEVICE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_DEVICE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_DYNAMICFORMATCHANGE(pub i32);
pub const KSEVENT_DYNAMIC_FORMAT_CHANGE: KSEVENT_DYNAMICFORMATCHANGE = KSEVENT_DYNAMICFORMATCHANGE(0i32);
impl ::core::convert::From<i32> for KSEVENT_DYNAMICFORMATCHANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_DYNAMICFORMATCHANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_ENTRY_BUFFERED: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_ENTRY_DELETED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_ENTRY_ONESHOT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_LOOPEDSTREAMING(pub i32);
pub const KSEVENT_LOOPEDSTREAMING_POSITION: KSEVENT_LOOPEDSTREAMING = KSEVENT_LOOPEDSTREAMING(0i32);
impl ::core::convert::From<i32> for KSEVENT_LOOPEDSTREAMING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_LOOPEDSTREAMING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_PINCAPS_CHANGENOTIFICATIONS(pub i32);
pub const KSEVENT_PINCAPS_FORMATCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = KSEVENT_PINCAPS_CHANGENOTIFICATIONS(0i32);
pub const KSEVENT_PINCAPS_JACKINFOCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = KSEVENT_PINCAPS_CHANGENOTIFICATIONS(1i32);
impl ::core::convert::From<i32> for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_SOUNDDETECTOR(pub i32);
pub const KSEVENT_SOUNDDETECTOR_MATCHDETECTED: KSEVENT_SOUNDDETECTOR = KSEVENT_SOUNDDETECTOR(1i32);
impl ::core::convert::From<i32> for KSEVENT_SOUNDDETECTOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_SOUNDDETECTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_STREAMALLOCATOR(pub i32);
pub const KSEVENT_STREAMALLOCATOR_INTERNAL_FREEFRAME: KSEVENT_STREAMALLOCATOR = KSEVENT_STREAMALLOCATOR(0i32);
pub const KSEVENT_STREAMALLOCATOR_FREEFRAME: KSEVENT_STREAMALLOCATOR = KSEVENT_STREAMALLOCATOR(1i32);
impl ::core::convert::From<i32> for KSEVENT_STREAMALLOCATOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_STREAMALLOCATOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_TELEPHONY(pub i32);
pub const KSEVENT_TELEPHONY_ENDPOINTPAIRS_CHANGED: KSEVENT_TELEPHONY = KSEVENT_TELEPHONY(0i32);
impl ::core::convert::From<i32> for KSEVENT_TELEPHONY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_TELEPHONY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSEVENT_TIME_INTERVAL {
    pub EventData: KSEVENTDATA,
    pub TimeBase: i64,
    pub Interval: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl KSEVENT_TIME_INTERVAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TIME_INTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENT_TIME_INTERVAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENT_TIME_INTERVAL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSEVENT_TIME_INTERVAL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSEVENT_TIME_MARK {
    pub EventData: KSEVENTDATA,
    pub MarkTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl KSEVENT_TIME_MARK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TIME_MARK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENT_TIME_MARK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENT_TIME_MARK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSEVENT_TIME_MARK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_TUNER(pub i32);
pub const KSEVENT_TUNER_CHANGED: KSEVENT_TUNER = KSEVENT_TUNER(0i32);
pub const KSEVENT_TUNER_INITIATE_SCAN: KSEVENT_TUNER = KSEVENT_TUNER(1i32);
impl ::core::convert::From<i32> for KSEVENT_TUNER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_TUNER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSEVENT_TUNER_INITIATE_SCAN_S {
    pub EventData: KSEVENTDATA,
    pub StartFrequency: u32,
    pub EndFrequency: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSEVENT_TUNER_INITIATE_SCAN_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TUNER_INITIATE_SCAN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENT_TUNER_INITIATE_SCAN_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENT_TUNER_INITIATE_SCAN_S {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSEVENT_TUNER_INITIATE_SCAN_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_TVAUDIO(pub i32);
pub const KSEVENT_TVAUDIO_CHANGED: KSEVENT_TVAUDIO = KSEVENT_TVAUDIO(0i32);
impl ::core::convert::From<i32> for KSEVENT_TVAUDIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_TVAUDIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_TYPE_BASICSUPPORT: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_TYPE_ENABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_TYPE_ENABLEBUFFERED: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_TYPE_ONESHOT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_TYPE_QUERYBUFFER: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_TYPE_SETSUPPORT: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_TYPE_TOPOLOGY: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_VIDCAPTOSTI(pub i32);
pub const KSEVENT_VIDCAPTOSTI_EXT_TRIGGER: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(0i32);
pub const KSEVENT_VIDCAP_AUTO_UPDATE: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(1i32);
pub const KSEVENT_VIDCAP_SEARCH: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(2i32);
impl ::core::convert::From<i32> for KSEVENT_VIDCAPTOSTI {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_VIDCAPTOSTI {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_VIDEODECODER(pub i32);
pub const KSEVENT_VIDEODECODER_CHANGED: KSEVENT_VIDEODECODER = KSEVENT_VIDEODECODER(0i32);
impl ::core::convert::From<i32> for KSEVENT_VIDEODECODER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_VIDEODECODER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_VOLUMELIMIT(pub i32);
pub const KSEVENT_VOLUMELIMIT_CHANGED: KSEVENT_VOLUMELIMIT = KSEVENT_VOLUMELIMIT(0i32);
impl ::core::convert::From<i32> for KSEVENT_VOLUMELIMIT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_VOLUMELIMIT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_VPNOTIFY(pub i32);
pub const KSEVENT_VPNOTIFY_FORMATCHANGE: KSEVENT_VPNOTIFY = KSEVENT_VPNOTIFY(0i32);
impl ::core::convert::From<i32> for KSEVENT_VPNOTIFY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_VPNOTIFY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSEVENT_VPVBINOTIFY(pub i32);
pub const KSEVENT_VPVBINOTIFY_FORMATCHANGE: KSEVENT_VPVBINOTIFY = KSEVENT_VPVBINOTIFY(0i32);
impl ::core::convert::From<i32> for KSEVENT_VPVBINOTIFY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSEVENT_VPVBINOTIFY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSE_NODE {
    pub Event: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl KSE_NODE {}
impl ::core::default::Default for KSE_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSE_NODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSE_NODE {}
unsafe impl ::windows::runtime::Abi for KSE_NODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSE_PIN {
    pub Event: KSIDENTIFIER,
    pub PinId: u32,
    pub Reserved: u32,
}
impl KSE_PIN {}
impl ::core::default::Default for KSE_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSE_PIN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSE_PIN {}
unsafe impl ::windows::runtime::Abi for KSE_PIN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSFILTER_FLAG_CRITICAL_PROCESSING: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSFILTER_FLAG_DENY_USERMODE_ACCESS: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSFILTER_FLAG_DISPATCH_LEVEL_PROCESSING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSFILTER_FLAG_HYPERCRITICAL_PROCESSING: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSFILTER_FLAG_PRIORITIZE_REFERENCEGUID: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSFILTER_FLAG_RECEIVE_ZERO_LENGTH_SAMPLES: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSFRAMETIME {
    pub Duration: i64,
    pub FrameFlags: u32,
    pub Reserved: u32,
}
impl KSFRAMETIME {}
impl ::core::default::Default for KSFRAMETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSFRAMETIME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSFRAMETIME").field("Duration", &self.Duration).field("FrameFlags", &self.FrameFlags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSFRAMETIME {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration && self.FrameFlags == other.FrameFlags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSFRAMETIME {}
unsafe impl ::windows::runtime::Abi for KSFRAMETIME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSFRAMETIME_VARIABLESIZE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSGOP_USERDATA {
    pub sc: u32,
    pub reserved1: u32,
    pub cFields: u8,
    pub l21Data: [super::super::Foundation::CHAR; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl KSGOP_USERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSGOP_USERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSGOP_USERDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSGOP_USERDATA").field("sc", &self.sc).field("reserved1", &self.reserved1).field("cFields", &self.cFields).field("l21Data", &self.l21Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSGOP_USERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.sc == other.sc && self.reserved1 == other.reserved1 && self.cFields == other.cFields && self.l21Data == other.l21Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSGOP_USERDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSGOP_USERDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSIDENTIFIER {
    pub Anonymous: KSIDENTIFIER_0,
}
impl KSIDENTIFIER {}
impl ::core::default::Default for KSIDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSIDENTIFIER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSIDENTIFIER {}
unsafe impl ::windows::runtime::Abi for KSIDENTIFIER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSIDENTIFIER_0 {
    pub Anonymous: KSIDENTIFIER_0_0,
    pub Alignment: i64,
}
impl KSIDENTIFIER_0 {}
impl ::core::default::Default for KSIDENTIFIER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSIDENTIFIER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSIDENTIFIER_0 {}
unsafe impl ::windows::runtime::Abi for KSIDENTIFIER_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSIDENTIFIER_0_0 {
    pub Set: ::windows::runtime::GUID,
    pub Id: u32,
    pub Flags: u32,
}
impl KSIDENTIFIER_0_0 {}
impl ::core::default::Default for KSIDENTIFIER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSIDENTIFIER_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Set", &self.Set).field("Id", &self.Id).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for KSIDENTIFIER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Set == other.Set && self.Id == other.Id && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSIDENTIFIER_0_0 {}
unsafe impl ::windows::runtime::Abi for KSIDENTIFIER_0_0 {
    type Abi = Self;
}
pub const KSINTERFACESETID_FileIo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2356122412, 59249, 4560, [184, 255, 0, 160, 201, 34, 49, 150]);
pub const KSINTERFACESETID_Media: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(974383936, 12455, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSINTERFACESETID_Standard: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(445081248, 25294, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSINTERFACE_FILEIO(pub i32);
pub const KSINTERFACE_FILEIO_STREAMING: KSINTERFACE_FILEIO = KSINTERFACE_FILEIO(0i32);
impl ::core::convert::From<i32> for KSINTERFACE_FILEIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSINTERFACE_FILEIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSINTERFACE_MEDIA(pub i32);
pub const KSINTERFACE_MEDIA_MUSIC: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(0i32);
pub const KSINTERFACE_MEDIA_WAVE_BUFFERED: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(1i32);
pub const KSINTERFACE_MEDIA_WAVE_QUEUED: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(2i32);
impl ::core::convert::From<i32> for KSINTERFACE_MEDIA {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSINTERFACE_MEDIA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSINTERFACE_STANDARD(pub i32);
pub const KSINTERFACE_STANDARD_STREAMING: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(0i32);
pub const KSINTERFACE_STANDARD_LOOPED_STREAMING: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(1i32);
pub const KSINTERFACE_STANDARD_CONTROL: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(2i32);
impl ::core::convert::From<i32> for KSINTERFACE_STANDARD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSINTERFACE_STANDARD {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSINTERVAL {
    pub TimeBase: i64,
    pub Interval: i64,
}
impl KSINTERVAL {}
impl ::core::default::Default for KSINTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSINTERVAL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSINTERVAL").field("TimeBase", &self.TimeBase).field("Interval", &self.Interval).finish()
    }
}
impl ::core::cmp::PartialEq for KSINTERVAL {
    fn eq(&self, other: &Self) -> bool {
        self.TimeBase == other.TimeBase && self.Interval == other.Interval
    }
}
impl ::core::cmp::Eq for KSINTERVAL {}
unsafe impl ::windows::runtime::Abi for KSINTERVAL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KSJACK_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSJACK_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSJACK_DESCRIPTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSJACK_DESCRIPTION")
            .field("ChannelMapping", &self.ChannelMapping)
            .field("Color", &self.Color)
            .field("ConnectionType", &self.ConnectionType)
            .field("GeoLocation", &self.GeoLocation)
            .field("GenLocation", &self.GenLocation)
            .field("PortConnection", &self.PortConnection)
            .field("IsConnected", &self.IsConnected)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSJACK_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelMapping == other.ChannelMapping && self.Color == other.Color && self.ConnectionType == other.ConnectionType && self.GeoLocation == other.GeoLocation && self.GenLocation == other.GenLocation && self.PortConnection == other.PortConnection && self.IsConnected == other.IsConnected
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSJACK_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSJACK_DESCRIPTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSJACK_DESCRIPTION2 {
    pub DeviceStateInfo: u32,
    pub JackCapabilities: u32,
}
impl KSJACK_DESCRIPTION2 {}
impl ::core::default::Default for KSJACK_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSJACK_DESCRIPTION2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSJACK_DESCRIPTION2").field("DeviceStateInfo", &self.DeviceStateInfo).field("JackCapabilities", &self.JackCapabilities).finish()
    }
}
impl ::core::cmp::PartialEq for KSJACK_DESCRIPTION2 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceStateInfo == other.DeviceStateInfo && self.JackCapabilities == other.JackCapabilities
    }
}
impl ::core::cmp::Eq for KSJACK_DESCRIPTION2 {}
unsafe impl ::windows::runtime::Abi for KSJACK_DESCRIPTION2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSJACK_SINK_CONNECTIONTYPE(pub i32);
pub const KSJACK_SINK_CONNECTIONTYPE_HDMI: KSJACK_SINK_CONNECTIONTYPE = KSJACK_SINK_CONNECTIONTYPE(0i32);
pub const KSJACK_SINK_CONNECTIONTYPE_DISPLAYPORT: KSJACK_SINK_CONNECTIONTYPE = KSJACK_SINK_CONNECTIONTYPE(1i32);
impl ::core::convert::From<i32> for KSJACK_SINK_CONNECTIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSJACK_SINK_CONNECTIONTYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KSJACK_SINK_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSJACK_SINK_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSJACK_SINK_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSJACK_SINK_INFORMATION")
            .field("ConnType", &self.ConnType)
            .field("ManufacturerId", &self.ManufacturerId)
            .field("ProductId", &self.ProductId)
            .field("AudioLatency", &self.AudioLatency)
            .field("HDCPCapable", &self.HDCPCapable)
            .field("AICapable", &self.AICapable)
            .field("SinkDescriptionLength", &self.SinkDescriptionLength)
            .field("SinkDescription", &self.SinkDescription)
            .field("PortId", &self.PortId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSJACK_SINK_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ConnType == other.ConnType && self.ManufacturerId == other.ManufacturerId && self.ProductId == other.ProductId && self.AudioLatency == other.AudioLatency && self.HDCPCapable == other.HDCPCapable && self.AICapable == other.AICapable && self.SinkDescriptionLength == other.SinkDescriptionLength && self.SinkDescription == other.SinkDescription && self.PortId == other.PortId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSJACK_SINK_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSJACK_SINK_INFORMATION {
    type Abi = Self;
}
pub const KSMEDIUMSETID_MidiBus: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(93356096, 12870, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSMEDIUMSETID_Standard: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1195881248, 25294, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSMEDIUMSETID_VPBus: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2710312428, 52803, 4560, [171, 231, 0, 160, 201, 34, 49, 150]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMEDIUM_STANDARD_DEVIO: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMEDIUM_TYPE_ANYINSTANCE: u32 = 0u32;
pub const KSMEMORY_TYPE_DEVICE_UNKNOWN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(152811065, 24639, 4561, [176, 103, 0, 160, 201, 6, 40, 2]);
pub const KSMEMORY_TYPE_KERNEL_NONPAGED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1248681924, 30869, 4561, [176, 105, 0, 160, 201, 6, 40, 2]);
pub const KSMEMORY_TYPE_KERNEL_PAGED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3627284728, 30868, 4561, [176, 105, 0, 160, 201, 6, 40, 2]);
pub const KSMEMORY_TYPE_SYSTEM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(152811064, 24639, 4561, [176, 103, 0, 160, 201, 6, 40, 2]);
pub const KSMEMORY_TYPE_USER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2360409128, 30867, 4561, [176, 105, 0, 160, 201, 6, 40, 2]);
pub const KSMETHODSETID_StreamAllocator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3480109889, 60551, 4559, [161, 48, 0, 32, 175, 209, 86, 228]);
pub const KSMETHODSETID_StreamIo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1708131274, 5411, 4562, [178, 122, 0, 160, 201, 34, 49, 150]);
pub const KSMETHODSETID_Wavetable: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3706663403, 55559, 4560, [149, 131, 0, 192, 79, 185, 37, 211]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSMETHOD_STREAMALLOCATOR(pub i32);
pub const KSMETHOD_STREAMALLOCATOR_ALLOC: KSMETHOD_STREAMALLOCATOR = KSMETHOD_STREAMALLOCATOR(0i32);
pub const KSMETHOD_STREAMALLOCATOR_FREE: KSMETHOD_STREAMALLOCATOR = KSMETHOD_STREAMALLOCATOR(1i32);
impl ::core::convert::From<i32> for KSMETHOD_STREAMALLOCATOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSMETHOD_STREAMALLOCATOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSMETHOD_STREAMIO(pub i32);
pub const KSMETHOD_STREAMIO_READ: KSMETHOD_STREAMIO = KSMETHOD_STREAMIO(0i32);
pub const KSMETHOD_STREAMIO_WRITE: KSMETHOD_STREAMIO = KSMETHOD_STREAMIO(1i32);
impl ::core::convert::From<i32> for KSMETHOD_STREAMIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSMETHOD_STREAMIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_BASICSUPPORT: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_MODIFY: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_SEND: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_SETSUPPORT: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_SOURCE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_TOPOLOGY: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_TYPE_WRITE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSMETHOD_WAVETABLE(pub i32);
pub const KSMETHOD_WAVETABLE_WAVE_ALLOC: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(0i32);
pub const KSMETHOD_WAVETABLE_WAVE_FREE: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(1i32);
pub const KSMETHOD_WAVETABLE_WAVE_FIND: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(2i32);
pub const KSMETHOD_WAVETABLE_WAVE_WRITE: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(3i32);
impl ::core::convert::From<i32> for KSMETHOD_WAVETABLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSMETHOD_WAVETABLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_WAVE_QUEUED_BREAKLOOP: u32 = 1u32;
pub const KSMFT_CATEGORY_AUDIO_DECODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2661760948, 61306, 17753, [141, 93, 113, 157, 143, 4, 38, 199]);
pub const KSMFT_CATEGORY_AUDIO_EFFECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(285625416, 13896, 20176, [147, 46, 5, 206, 138, 200, 17, 183]);
pub const KSMFT_CATEGORY_AUDIO_ENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2445691856, 63774, 19852, [146, 118, 219, 36, 130, 121, 217, 117]);
pub const KSMFT_CATEGORY_DEMULTIPLEXER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2825915002, 37787, 17605, [153, 215, 118, 34, 107, 35, 179, 241]);
pub const KSMFT_CATEGORY_MULTIPLEXER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(94131742, 1454, 19297, [182, 157, 85, 182, 30, 229, 74, 123]);
pub const KSMFT_CATEGORY_OTHER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2417450327, 47082, 18689, [174, 179, 147, 58, 135, 71, 117, 111]);
pub const KSMFT_CATEGORY_VIDEO_DECODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3602918731, 26675, 17844, [151, 26, 5, 164, 176, 75, 171, 145]);
pub const KSMFT_CATEGORY_VIDEO_EFFECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(316767265, 21292, 19054, [138, 28, 64, 130, 90, 115, 99, 151]);
pub const KSMFT_CATEGORY_VIDEO_ENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4154371197, 58693, 17287, [189, 238, 214, 71, 215, 189, 228, 42]);
pub const KSMFT_CATEGORY_VIDEO_PROCESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(808363004, 43615, 18425, [159, 122, 194, 24, 139, 177, 99, 2]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSMICARRAY_MICARRAYTYPE(pub i32);
pub const KSMICARRAY_MICARRAYTYPE_LINEAR: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(0i32);
pub const KSMICARRAY_MICARRAYTYPE_PLANAR: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(1i32);
pub const KSMICARRAY_MICARRAYTYPE_3D: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(2i32);
impl ::core::convert::From<i32> for KSMICARRAY_MICARRAYTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSMICARRAY_MICARRAYTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSMICARRAY_MICTYPE(pub i32);
pub const KSMICARRAY_MICTYPE_OMNIDIRECTIONAL: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(0i32);
pub const KSMICARRAY_MICTYPE_SUBCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(1i32);
pub const KSMICARRAY_MICTYPE_CARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(2i32);
pub const KSMICARRAY_MICTYPE_SUPERCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(3i32);
pub const KSMICARRAY_MICTYPE_HYPERCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(4i32);
pub const KSMICARRAY_MICTYPE_8SHAPED: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(5i32);
pub const KSMICARRAY_MICTYPE_VENDORDEFINED: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(15i32);
impl ::core::convert::From<i32> for KSMICARRAY_MICTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSMICARRAY_MICTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMPEGVIDMODE_LTRBOX: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMPEGVIDMODE_PANSCAN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMPEGVIDMODE_SCALE: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSMPEGVID_RECT {
    pub StartX: u32,
    pub StartY: u32,
    pub EndX: u32,
    pub EndY: u32,
}
impl KSMPEGVID_RECT {}
impl ::core::default::Default for KSMPEGVID_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSMPEGVID_RECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSMPEGVID_RECT").field("StartX", &self.StartX).field("StartY", &self.StartY).field("EndX", &self.EndX).field("EndY", &self.EndY).finish()
    }
}
impl ::core::cmp::PartialEq for KSMPEGVID_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.StartX == other.StartX && self.StartY == other.StartY && self.EndX == other.EndX && self.EndY == other.EndY
    }
}
impl ::core::cmp::Eq for KSMPEGVID_RECT {}
unsafe impl ::windows::runtime::Abi for KSMPEGVID_RECT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSMULTIPLE_DATA_PROP {
    pub Property: KSIDENTIFIER,
    pub MultipleItem: KSMULTIPLE_ITEM,
}
impl KSMULTIPLE_DATA_PROP {}
impl ::core::default::Default for KSMULTIPLE_DATA_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSMULTIPLE_DATA_PROP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSMULTIPLE_DATA_PROP {}
unsafe impl ::windows::runtime::Abi for KSMULTIPLE_DATA_PROP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSMULTIPLE_ITEM {
    pub Size: u32,
    pub Count: u32,
}
impl KSMULTIPLE_ITEM {}
impl ::core::default::Default for KSMULTIPLE_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSMULTIPLE_ITEM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSMULTIPLE_ITEM").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::core::cmp::PartialEq for KSMULTIPLE_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for KSMULTIPLE_ITEM {}
unsafe impl ::windows::runtime::Abi for KSMULTIPLE_ITEM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSMUSICFORMAT {
    pub TimeDeltaMs: u32,
    pub ByteCount: u32,
}
impl KSMUSICFORMAT {}
impl ::core::default::Default for KSMUSICFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSMUSICFORMAT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSMUSICFORMAT").field("TimeDeltaMs", &self.TimeDeltaMs).field("ByteCount", &self.ByteCount).finish()
    }
}
impl ::core::cmp::PartialEq for KSMUSICFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.TimeDeltaMs == other.TimeDeltaMs && self.ByteCount == other.ByteCount
    }
}
impl ::core::cmp::Eq for KSMUSICFORMAT {}
unsafe impl ::windows::runtime::Abi for KSMUSICFORMAT {
    type Abi = Self;
}
pub const KSMUSIC_TECHNOLOGY_FMSYNTH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(623664256, 25321, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSMUSIC_TECHNOLOGY_PORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2261331552, 25320, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSMUSIC_TECHNOLOGY_SQSYNTH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(248464256, 25321, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSMUSIC_TECHNOLOGY_SWSYNTH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(926971702, 13856, 4561, [133, 211, 0, 0, 248, 117, 67, 128]);
pub const KSMUSIC_TECHNOLOGY_WAVETABLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(961464256, 25321, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSM_NODE {
    pub Method: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl KSM_NODE {}
impl ::core::default::Default for KSM_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSM_NODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSM_NODE {}
unsafe impl ::windows::runtime::Abi for KSM_NODE {
    type Abi = Self;
}
pub const KSNAME_Allocator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1680825600, 18321, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSNAME_Clock: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1394025600, 18321, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSNAME_Filter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2604030096, 5727, 4560, [161, 149, 0, 32, 175, 209, 86, 228]);
pub const KSNAME_Pin: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(342825600, 18321, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSNAME_TopologyNode: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(102827546, 61045, 4560, [185, 21, 0, 160, 201, 34, 49, 150]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_AEC_CAPTURE_IN: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_AEC_CAPTURE_OUT: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_AEC_RENDER_IN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_AEC_RENDER_OUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_DEMUX_IN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_DEMUX_OUT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_STANDARD_IN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_STANDARD_OUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_SUM_MUX_IN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSNODEPIN_SUM_MUX_OUT: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSNODEPROPERTY {
    pub Property: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl KSNODEPROPERTY {}
impl ::core::default::Default for KSNODEPROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSNODEPROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSNODEPROPERTY {}
unsafe impl ::windows::runtime::Abi for KSNODEPROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER {
    pub NodeProperty: KSNODEPROPERTY,
    pub ListenerId: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl KSNODEPROPERTY_AUDIO_3D_LISTENER {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_3D_LISTENER {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::runtime::Abi for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER {
    pub NodeProperty: KSNODEPROPERTY,
    pub ListenerId: *mut ::core::ffi::c_void,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "x86",))]
impl KSNODEPROPERTY_AUDIO_3D_LISTENER {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_3D_LISTENER {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::runtime::Abi for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSNODEPROPERTY_AUDIO_CHANNEL {
    pub NodeProperty: KSNODEPROPERTY,
    pub Channel: i32,
    pub Reserved: u32,
}
impl KSNODEPROPERTY_AUDIO_CHANNEL {}
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_CHANNEL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_CHANNEL {}
unsafe impl ::windows::runtime::Abi for KSNODEPROPERTY_AUDIO_CHANNEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    pub NodeProperty: KSNODEPROPERTY,
    pub DevSpecificId: u32,
    pub DeviceInfo: u32,
    pub Length: u32,
}
impl KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {}
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {}
unsafe impl ::windows::runtime::Abi for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY {
    pub NodeProperty: KSNODEPROPERTY,
    pub AppContext: *mut ::core::ffi::c_void,
    pub Length: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl KSNODEPROPERTY_AUDIO_PROPERTY {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_PROPERTY {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::runtime::Abi for KSNODEPROPERTY_AUDIO_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY {
    pub NodeProperty: KSNODEPROPERTY,
    pub AppContext: *mut ::core::ffi::c_void,
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "x86",))]
impl KSNODEPROPERTY_AUDIO_PROPERTY {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_PROPERTY {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::runtime::Abi for KSNODEPROPERTY_AUDIO_PROPERTY {
    type Abi = Self;
}
pub const KSNODETYPE_1394_DA_STREAM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187046, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_1394_DV_STREAM_SOUNDTRACK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187047, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_3D_EFFECTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1431394400, 50521, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_ADC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1300463584, 50517, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_AGC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3901528992, 50519, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_ANALOG_CONNECTOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187041, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_ANALOG_TAPE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187303, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_AUDIO_ENGINE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(902493924, 62387, 16744, [187, 75, 85, 231, 122, 70, 28, 126]);
pub const KSNODETYPE_AUDIO_KEYWORDDETECTOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(941088952, 57176, 17269, [182, 105, 196, 150, 52, 51, 31, 157]);
pub const KSNODETYPE_AUDIO_LOOPBACK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2403516594, 37326, 19407, [156, 205, 14, 89, 144, 55, 171, 53]);
pub const KSNODETYPE_AUDIO_MODULE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1168815150, 51947, 16466, [138, 169, 179, 140, 181, 16, 150, 25]);
pub const KSNODETYPE_BIDIRECTIONAL_UNDEFINED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186528, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_CABLE_TUNER_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187310, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_CD_PLAYER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187299, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_CHORUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(538394400, 50521, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_COMMUNICATION_SPEAKER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186278, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DAC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1350230880, 50516, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_DAT_IO_DIGITAL_AUDIO_TAPE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187300, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DCC_IO_DIGITAL_COMPACT_CASSETTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187301, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DELAY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(340361696, 50520, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_DEMUX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3236653012, 59399, 4560, [149, 138, 0, 192, 79, 185, 37, 211]);
pub const KSNODETYPE_DESKTOP_MICROPHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186018, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DESKTOP_SPEAKER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186276, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DEV_SPECIFIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2484894400, 50521, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_DIGITAL_AUDIO_INTERFACE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187042, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DISPLAYPORT_INTERFACE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3833479217, 16038, 16781, [143, 155, 183, 56, 67, 204, 186, 151]);
pub const KSNODETYPE_DOWN_LINE_PHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186787, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DRM_DESCRAMBLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4290473535, 52478, 19844, [144, 217, 66, 20, 24, 176, 58, 142]);
pub const KSNODETYPE_DSS_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187311, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DVD_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187307, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_DYN_RANGE_COMPRESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(147367592, 24607, 19192, [135, 147, 217, 5, 255, 76, 169, 125]);
pub const KSNODETYPE_ECHO_CANCELING_SPEAKERPHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186533, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_ECHO_SUPPRESSING_SPEAKERPHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186532, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_EMBEDDED_UNDEFINED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187296, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_EQUALIZATION_NOISE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187298, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_EQUALIZER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2638328992, 50519, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_EXTERNAL_UNDEFINED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187040, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_FM_RX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2202694460, 62597, 16832, [166, 43, 81, 48, 37, 1, 78, 64]);
pub const KSNODETYPE_HANDSET: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186529, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_HDMI_INTERFACE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3518614570, 62745, 16767, [145, 201, 85, 250, 101, 72, 16, 1]);
pub const KSNODETYPE_HEADPHONES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186274, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_HEADSET: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186530, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_HEAD_MOUNTED_DISPLAY_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186275, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_INPUT_UNDEFINED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186016, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_LEGACY_AUDIO_CONNECTOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187044, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_LEVEL_CALIBRATION_NOISE_SOURCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187297, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_LINE_CONNECTOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187043, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_LOUDNESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1099461696, 50520, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_LOW_FREQUENCY_EFFECTS_SPEAKER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186279, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_MICROPHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186017, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_MICROPHONE_ARRAY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186021, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_MIDI_ELEMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(29818470, 28232, 19557, [172, 155, 82, 219, 93, 101, 108, 126]);
pub const KSNODETYPE_MIDI_JACK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(643697727, 64057, 19955, [171, 4, 190, 1, 185, 30, 41, 154]);
pub const KSNODETYPE_MINIDISK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187302, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_MULTITRACK_RECORDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187314, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_MUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(45228992, 50519, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_MUX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(753596288, 50518, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_NOISE_SUPPRESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3766456383, 25341, 20064, [140, 221, 222, 167, 35, 102, 101, 181]);
pub const KSNODETYPE_OMNI_DIRECTIONAL_MICROPHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186020, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_OUTPUT_UNDEFINED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186272, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_PARAMETRIC_EQUALIZER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(431700586, 52779, 17474, [135, 236, 103, 39, 195, 202, 180, 119]);
pub const KSNODETYPE_PEAKMETER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2693096734, 24333, 19254, [168, 105, 209, 149, 214, 171, 75, 158]);
pub const KSNODETYPE_PERSONAL_MICROPHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186019, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_PHONE_LINE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186785, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_PHONOGRAPH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187304, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_PROCESSING_MICROPHONE_ARRAY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186022, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_PROLOGIC_DECODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2199661696, 50520, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_PROLOGIC_ENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2155136434, 15462, 4562, [180, 90, 48, 120, 48, 44, 32, 48]);
pub const KSNODETYPE_RADIO_RECEIVER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187312, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_RADIO_TRANSMITTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187313, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_REVERB: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4009961696, 50520, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_ROOM_SPEAKER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186277, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_SATELLITE_RECEIVER_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187309, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_SPDIF_INTERFACE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187045, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_SPEAKER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186273, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_SPEAKERPHONE_NO_ECHO_REDUCTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186531, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_SPEAKERS_STATIC_JACK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(685789063, 19902, 20365, [133, 137, 2, 93, 32, 157, 251, 74]);
pub const KSNODETYPE_SRC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2646063584, 50517, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_STEREO_WIDE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2850461696, 50520, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_SUM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3661896288, 50518, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_SUPERMIX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3849563584, 50517, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_SYNTHESIZER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187315, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_TELEPHONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186786, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_TELEPHONY_BIDI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1752005824, 55555, 16984, [180, 67, 58, 61, 53, 128, 116, 28]);
pub const KSNODETYPE_TELEPHONY_UNDEFINED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757186784, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_TONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1980228992, 50519, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const KSNODETYPE_TV_TUNER_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187308, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_UPDOWN_MIX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085813199, 31587, 20194, [161, 0, 41, 238, 44, 182, 178, 222]);
pub const KSNODETYPE_VCR_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187305, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_CAMERA_TERMINAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757189606, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_DISC_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757187306, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_INPUT_MTT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757189607, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_INPUT_TERMINAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757189602, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_OUTPUT_MTT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757189608, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_OUTPUT_TERMINAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757189603, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_PROCESSING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757189605, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_SELECTOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757189604, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VIDEO_STREAMING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757189601, 63247, 4560, [185, 23, 0, 160, 201, 34, 49, 150]);
pub const KSNODETYPE_VOLUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(979028992, 50519, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSNODE_CREATE {
    pub CreateFlags: u32,
    pub Node: u32,
}
impl KSNODE_CREATE {}
impl ::core::default::Default for KSNODE_CREATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSNODE_CREATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSNODE_CREATE").field("CreateFlags", &self.CreateFlags).field("Node", &self.Node).finish()
    }
}
impl ::core::cmp::PartialEq for KSNODE_CREATE {
    fn eq(&self, other: &Self) -> bool {
        self.CreateFlags == other.CreateFlags && self.Node == other.Node
    }
}
impl ::core::cmp::Eq for KSNODE_CREATE {}
unsafe impl ::windows::runtime::Abi for KSNODE_CREATE {
    type Abi = Self;
}
pub const KSNOTIFICATIONID_AudioModule: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2619482352, 55718, 19804, [160, 54, 87, 56, 87, 253, 80, 210]);
pub const KSNOTIFICATIONID_SoundDetector: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1669978180, 47922, 19532, [168, 2, 244, 180, 183, 122, 254, 173]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPIN_CINSTANCES {
    pub PossibleCount: u32,
    pub CurrentCount: u32,
}
impl KSPIN_CINSTANCES {}
impl ::core::default::Default for KSPIN_CINSTANCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPIN_CINSTANCES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPIN_CINSTANCES").field("PossibleCount", &self.PossibleCount).field("CurrentCount", &self.CurrentCount).finish()
    }
}
impl ::core::cmp::PartialEq for KSPIN_CINSTANCES {
    fn eq(&self, other: &Self) -> bool {
        self.PossibleCount == other.PossibleCount && self.CurrentCount == other.CurrentCount
    }
}
impl ::core::cmp::Eq for KSPIN_CINSTANCES {}
unsafe impl ::windows::runtime::Abi for KSPIN_CINSTANCES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPIN_COMMUNICATION(pub i32);
pub const KSPIN_COMMUNICATION_NONE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(0i32);
pub const KSPIN_COMMUNICATION_SINK: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(1i32);
pub const KSPIN_COMMUNICATION_SOURCE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(2i32);
pub const KSPIN_COMMUNICATION_BOTH: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(3i32);
pub const KSPIN_COMMUNICATION_BRIDGE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(4i32);
impl ::core::convert::From<i32> for KSPIN_COMMUNICATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPIN_COMMUNICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPIN_CONNECT {
    pub Interface: KSIDENTIFIER,
    pub Medium: KSIDENTIFIER,
    pub PinId: u32,
    pub PinToHandle: super::super::Foundation::HANDLE,
    pub Priority: KSPRIORITY,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPIN_CONNECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPIN_CONNECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPIN_CONNECT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPIN_CONNECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPIN_CONNECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPIN_DATAFLOW(pub i32);
pub const KSPIN_DATAFLOW_IN: KSPIN_DATAFLOW = KSPIN_DATAFLOW(1i32);
pub const KSPIN_DATAFLOW_OUT: KSPIN_DATAFLOW = KSPIN_DATAFLOW(2i32);
impl ::core::convert::From<i32> for KSPIN_DATAFLOW {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPIN_DATAFLOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_ASYNCHRONOUS_PROCESSING: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_CRITICAL_PROCESSING: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_DENY_USERMODE_ACCESS: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_DISPATCH_LEVEL_PROCESSING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_DISTINCT_TRAILING_EDGE: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_DO_NOT_INITIATE_PROCESSING: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_DO_NOT_USE_STANDARD_TRANSPORT: u32 = 524288u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_ENFORCE_FIFO: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_FIXED_FORMAT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_FRAMES_NOT_REQUIRED_FOR_PROCESSING: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_GENERATE_EOS_EVENTS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_GENERATE_MAPPINGS: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_HYPERCRITICAL_PROCESSING: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_IMPLEMENT_CLOCK: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_INITIATE_PROCESSING_ON_EVERY_ARRIVAL: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_PROCESS_IF_ANY_IN_RUN_STATE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_PROCESS_IN_RUN_STATE_ONLY: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_SOME_FRAMES_REQUIRED_FOR_PROCESSING: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_SPLITTER: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPIN_FLAG_USE_STANDARD_TRANSPORT: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPIN_MDL_CACHING_EVENT(pub i32);
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANUP: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(0i32);
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_WAIT: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(1i32);
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_NOWAIT: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(2i32);
pub const KSPIN_MDL_CACHING_NOTIFY_ADDSAMPLE: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(3i32);
impl ::core::convert::From<i32> for KSPIN_MDL_CACHING_EVENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPIN_MDL_CACHING_EVENT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPIN_MDL_CACHING_NOTIFICATION {
    pub Event: KSPIN_MDL_CACHING_EVENT,
    pub Buffer: *mut ::core::ffi::c_void,
}
impl KSPIN_MDL_CACHING_NOTIFICATION {}
impl ::core::default::Default for KSPIN_MDL_CACHING_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPIN_MDL_CACHING_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPIN_MDL_CACHING_NOTIFICATION").field("Event", &self.Event).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::cmp::PartialEq for KSPIN_MDL_CACHING_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Event == other.Event && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for KSPIN_MDL_CACHING_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for KSPIN_MDL_CACHING_NOTIFICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPIN_MDL_CACHING_NOTIFICATION32 {
    pub Event: KSPIN_MDL_CACHING_EVENT,
    pub Buffer: u32,
}
impl KSPIN_MDL_CACHING_NOTIFICATION32 {}
impl ::core::default::Default for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPIN_MDL_CACHING_NOTIFICATION32").field("Event", &self.Event).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::cmp::PartialEq for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn eq(&self, other: &Self) -> bool {
        self.Event == other.Event && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for KSPIN_MDL_CACHING_NOTIFICATION32 {}
unsafe impl ::windows::runtime::Abi for KSPIN_MDL_CACHING_NOTIFICATION32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPIN_PHYSICALCONNECTION {
    pub Size: u32,
    pub Pin: u32,
    pub SymbolicLinkName: [u16; 1],
}
impl KSPIN_PHYSICALCONNECTION {}
impl ::core::default::Default for KSPIN_PHYSICALCONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPIN_PHYSICALCONNECTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPIN_PHYSICALCONNECTION").field("Size", &self.Size).field("Pin", &self.Pin).field("SymbolicLinkName", &self.SymbolicLinkName).finish()
    }
}
impl ::core::cmp::PartialEq for KSPIN_PHYSICALCONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Pin == other.Pin && self.SymbolicLinkName == other.SymbolicLinkName
    }
}
impl ::core::cmp::Eq for KSPIN_PHYSICALCONNECTION {}
unsafe impl ::windows::runtime::Abi for KSPIN_PHYSICALCONNECTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPPROPERTY_ALLOCATOR_MDLCACHING(pub i32);
pub const KSPROPERTY_ALLOCATOR_CLEANUP_CACHEDMDLPAGES: KSPPROPERTY_ALLOCATOR_MDLCACHING = KSPPROPERTY_ALLOCATOR_MDLCACHING(1i32);
impl ::core::convert::From<i32> for KSPPROPERTY_ALLOCATOR_MDLCACHING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPPROPERTY_ALLOCATOR_MDLCACHING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPRIORITY {
    pub PriorityClass: u32,
    pub PrioritySubClass: u32,
}
impl KSPRIORITY {}
impl ::core::default::Default for KSPRIORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPRIORITY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPRIORITY").field("PriorityClass", &self.PriorityClass).field("PrioritySubClass", &self.PrioritySubClass).finish()
    }
}
impl ::core::cmp::PartialEq for KSPRIORITY {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityClass == other.PriorityClass && self.PrioritySubClass == other.PrioritySubClass
    }
}
impl ::core::cmp::Eq for KSPRIORITY {}
unsafe impl ::windows::runtime::Abi for KSPRIORITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPRIORITY_EXCLUSIVE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPRIORITY_HIGH: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPRIORITY_LOW: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPRIORITY_NORMAL: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROBE_ALLOCATEMDL: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROBE_ALLOWFORMATCHANGE: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROBE_MODIFY: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROBE_PROBEANDLOCK: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROBE_STREAMREAD: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROBE_STREAMWRITE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROBE_SYSTEMADDRESS: u32 = 64u32;
pub const KSPROPERTYSETID_ExtendedCameraControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(481792274, 49362, 16915, [156, 166, 205, 79, 219, 146, 121, 114]);
pub const KSPROPERTYSETID_NetworkCameraControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(242749193, 22341, 20026, [188, 159, 242, 38, 234, 67, 166, 236]);
pub const KSPROPERTYSETID_PerFrameSettingControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4059292257, 57062, 17719, [191, 245, 238, 32, 109, 181, 74, 172]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AC3(pub i32);
pub const KSPROPERTY_AC3_ERROR_CONCEALMENT: KSPROPERTY_AC3 = KSPROPERTY_AC3(1i32);
pub const KSPROPERTY_AC3_ALTERNATE_AUDIO: KSPROPERTY_AC3 = KSPROPERTY_AC3(2i32);
pub const KSPROPERTY_AC3_DOWNMIX: KSPROPERTY_AC3 = KSPROPERTY_AC3(3i32);
pub const KSPROPERTY_AC3_BIT_STREAM_MODE: KSPROPERTY_AC3 = KSPROPERTY_AC3(4i32);
pub const KSPROPERTY_AC3_DIALOGUE_LEVEL: KSPROPERTY_AC3 = KSPROPERTY_AC3(5i32);
pub const KSPROPERTY_AC3_LANGUAGE_CODE: KSPROPERTY_AC3 = KSPROPERTY_AC3(6i32);
pub const KSPROPERTY_AC3_ROOM_TYPE: KSPROPERTY_AC3 = KSPROPERTY_AC3(7i32);
impl ::core::convert::From<i32> for KSPROPERTY_AC3 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AC3 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL(pub i32);
pub const KSPROPERTY_ALLOCATOR_CONTROL_HONOR_COUNT: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(0i32);
pub const KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(1i32);
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(2i32);
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(3i32);
impl ::core::convert::From<i32> for KSPROPERTY_ALLOCATOR_CONTROL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_ALLOCATOR_CONTROL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    pub InterleavedCapSupported: u32,
}
impl KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S").field("InterleavedCapSupported", &self.InterleavedCapSupported).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        self.InterleavedCapSupported == other.InterleavedCapSupported
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    pub InterleavedCapPossible: u32,
}
impl KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S").field("InterleavedCapPossible", &self.InterleavedCapPossible).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn eq(&self, other: &Self) -> bool {
        self.InterleavedCapPossible == other.InterleavedCapPossible
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    pub CX: u32,
    pub CY: u32,
}
impl KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S").field("CX", &self.CX).field("CY", &self.CY).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn eq(&self, other: &Self) -> bool {
        self.CX == other.CX && self.CY == other.CY
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AUDDECOUT(pub i32);
pub const KSPROPERTY_AUDDECOUT_MODES: KSPROPERTY_AUDDECOUT = KSPROPERTY_AUDDECOUT(0i32);
pub const KSPROPERTY_AUDDECOUT_CUR_MODE: KSPROPERTY_AUDDECOUT = KSPROPERTY_AUDDECOUT(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_AUDDECOUT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AUDDECOUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AUDIO(pub i32);
pub const KSPROPERTY_AUDIO_LATENCY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(1i32);
pub const KSPROPERTY_AUDIO_COPY_PROTECTION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(2i32);
pub const KSPROPERTY_AUDIO_CHANNEL_CONFIG: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(3i32);
pub const KSPROPERTY_AUDIO_VOLUMELEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(4i32);
pub const KSPROPERTY_AUDIO_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(5i32);
pub const KSPROPERTY_AUDIO_DYNAMIC_RANGE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(6i32);
pub const KSPROPERTY_AUDIO_QUALITY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(7i32);
pub const KSPROPERTY_AUDIO_SAMPLING_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(8i32);
pub const KSPROPERTY_AUDIO_DYNAMIC_SAMPLING_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(9i32);
pub const KSPROPERTY_AUDIO_MIX_LEVEL_TABLE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(10i32);
pub const KSPROPERTY_AUDIO_MIX_LEVEL_CAPS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(11i32);
pub const KSPROPERTY_AUDIO_MUX_SOURCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(12i32);
pub const KSPROPERTY_AUDIO_MUTE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(13i32);
pub const KSPROPERTY_AUDIO_BASS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(14i32);
pub const KSPROPERTY_AUDIO_MID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(15i32);
pub const KSPROPERTY_AUDIO_TREBLE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(16i32);
pub const KSPROPERTY_AUDIO_BASS_BOOST: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(17i32);
pub const KSPROPERTY_AUDIO_EQ_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(18i32);
pub const KSPROPERTY_AUDIO_NUM_EQ_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(19i32);
pub const KSPROPERTY_AUDIO_EQ_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(20i32);
pub const KSPROPERTY_AUDIO_AGC: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(21i32);
pub const KSPROPERTY_AUDIO_DELAY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(22i32);
pub const KSPROPERTY_AUDIO_LOUDNESS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(23i32);
pub const KSPROPERTY_AUDIO_WIDE_MODE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(24i32);
pub const KSPROPERTY_AUDIO_WIDENESS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(25i32);
pub const KSPROPERTY_AUDIO_REVERB_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(26i32);
pub const KSPROPERTY_AUDIO_CHORUS_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(27i32);
pub const KSPROPERTY_AUDIO_DEV_SPECIFIC: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(28i32);
pub const KSPROPERTY_AUDIO_DEMUX_DEST: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(29i32);
pub const KSPROPERTY_AUDIO_STEREO_ENHANCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(30i32);
pub const KSPROPERTY_AUDIO_MANUFACTURE_GUID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(31i32);
pub const KSPROPERTY_AUDIO_PRODUCT_GUID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(32i32);
pub const KSPROPERTY_AUDIO_CPU_RESOURCES: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(33i32);
pub const KSPROPERTY_AUDIO_STEREO_SPEAKER_GEOMETRY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(34i32);
pub const KSPROPERTY_AUDIO_SURROUND_ENCODE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(35i32);
pub const KSPROPERTY_AUDIO_3D_INTERFACE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(36i32);
pub const KSPROPERTY_AUDIO_PEAKMETER: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(37i32);
pub const KSPROPERTY_AUDIO_ALGORITHM_INSTANCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(38i32);
pub const KSPROPERTY_AUDIO_FILTER_STATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(39i32);
pub const KSPROPERTY_AUDIO_PREFERRED_STATUS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(40i32);
pub const KSPROPERTY_AUDIO_PEQ_MAX_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(41i32);
pub const KSPROPERTY_AUDIO_PEQ_NUM_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(42i32);
pub const KSPROPERTY_AUDIO_PEQ_BAND_CENTER_FREQ: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(43i32);
pub const KSPROPERTY_AUDIO_PEQ_BAND_Q_FACTOR: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(44i32);
pub const KSPROPERTY_AUDIO_PEQ_BAND_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(45i32);
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(46i32);
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_DEPTH: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(47i32);
pub const KSPROPERTY_AUDIO_REVERB_TIME: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(48i32);
pub const KSPROPERTY_AUDIO_REVERB_DELAY_FEEDBACK: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(49i32);
pub const KSPROPERTY_AUDIO_POSITIONEX: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(50i32);
pub const KSPROPERTY_AUDIO_MIC_ARRAY_GEOMETRY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(51i32);
pub const KSPROPERTY_AUDIO_PRESENTATION_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(52i32);
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(53i32);
pub const KSPROPERTY_AUDIO_LINEAR_BUFFER_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(54i32);
pub const KSPROPERTY_AUDIO_PEAKMETER2: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(55i32);
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_LASTBUFFER_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(56i32);
pub const KSPROPERTY_AUDIO_VOLUMELIMIT_ENGAGED: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(57i32);
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(58i32);
pub const KSPROPERTY_AUDIO_MIC_SNR: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(59i32);
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY2: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(60i32);
impl ::core::convert::From<i32> for KSPROPERTY_AUDIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AUDIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AUDIOENGINE(pub i32);
pub const KSPROPERTY_AUDIOENGINE_LFXENABLE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(0i32);
pub const KSPROPERTY_AUDIOENGINE_GFXENABLE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(1i32);
pub const KSPROPERTY_AUDIOENGINE_MIXFORMAT: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(2i32);
pub const KSPROPERTY_AUDIOENGINE_DEVICEFORMAT: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(4i32);
pub const KSPROPERTY_AUDIOENGINE_SUPPORTEDDEVICEFORMATS: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(5i32);
pub const KSPROPERTY_AUDIOENGINE_DESCRIPTOR: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(6i32);
pub const KSPROPERTY_AUDIOENGINE_BUFFER_SIZE_RANGE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(7i32);
pub const KSPROPERTY_AUDIOENGINE_LOOPBACK_PROTECTION: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(8i32);
pub const KSPROPERTY_AUDIOENGINE_VOLUMELEVEL: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(9i32);
impl ::core::convert::From<i32> for KSPROPERTY_AUDIOENGINE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AUDIOENGINE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AUDIOMODULE(pub i32);
pub const KSPROPERTY_AUDIOMODULE_DESCRIPTORS: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(1i32);
pub const KSPROPERTY_AUDIOMODULE_COMMAND: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(2i32);
pub const KSPROPERTY_AUDIOMODULE_NOTIFICATION_DEVICE_ID: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(3i32);
impl ::core::convert::From<i32> for KSPROPERTY_AUDIOMODULE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AUDIOMODULE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AUDIOPOSTURE(pub i32);
pub const KSPROPERTY_AUDIOPOSTURE_ORIENTATION: KSPROPERTY_AUDIOPOSTURE = KSPROPERTY_AUDIOPOSTURE(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_AUDIOPOSTURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AUDIOPOSTURE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AUDIORESOURCEMANAGEMENT(pub i32);
pub const KSPROPERTY_AUDIORESOURCEMANAGEMENT_RESOURCEGROUP: KSPROPERTY_AUDIORESOURCEMANAGEMENT = KSPROPERTY_AUDIORESOURCEMANAGEMENT(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_AUDIORESOURCEMANAGEMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AUDIORESOURCEMANAGEMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AUDIOSIGNALPROCESSING(pub i32);
pub const KSPROPERTY_AUDIOSIGNALPROCESSING_MODES: KSPROPERTY_AUDIOSIGNALPROCESSING = KSPROPERTY_AUDIOSIGNALPROCESSING(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_AUDIOSIGNALPROCESSING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AUDIOSIGNALPROCESSING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_AUDIO_BUFFER_DURATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_BIBLIOGRAPHIC(pub i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_LEADER: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(1380207648i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_LCCN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808529952i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ISBN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808595488i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ISSN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(842149920i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_CATALOGINGSOURCE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808726560i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808464672i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINCORPORATEBODY: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808530208i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINMEETINGNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825307424i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808661280i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_UNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808727072i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_TITLESTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892613152i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_VARYINGFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(909390368i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_PUBLICATION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808858144i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_PHYSICALDESCRIPTION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465184i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808727584i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(809055264i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_GENERALNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465696i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_BIBLIOGRAPHYNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(875574560i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_CONTENTSNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892351776i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_CREATIONCREDIT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(942683424i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_CITATION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808531232i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_PARTICIPANT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825308448i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SUMMARY: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808596768i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_TARGETAUDIENCE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825373984i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDFORMAVAILABLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808662304i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SYSTEMDETAILS: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(942880032i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_AWARDS: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(909653280i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465952i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTOPICALTERM: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808793632i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYGEOGRAPHIC: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825570848i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMGENRE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892679712i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMCURRICULUM: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(943011360i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808662816i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYRELATED: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808728352i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808466464i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808663072i32);
impl ::core::convert::From<i32> for KSPROPERTY_BIBLIOGRAPHIC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_BIBLIOGRAPHIC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSPROPERTY_BOUNDS_LONG {
    pub Anonymous1: KSPROPERTY_BOUNDS_LONG_0,
    pub Anonymous2: KSPROPERTY_BOUNDS_LONG_1,
}
impl KSPROPERTY_BOUNDS_LONG {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONG {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_BOUNDS_LONG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_BOUNDS_LONG_0 {
    pub SignedMinimum: i32,
    pub SignedMaximum: i32,
}
impl KSPROPERTY_BOUNDS_LONG_0 {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONG_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_BOUNDS_LONG_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("SignedMinimum", &self.SignedMinimum).field("SignedMaximum", &self.SignedMaximum).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONG_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SignedMinimum == other.SignedMinimum && self.SignedMaximum == other.SignedMaximum
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONG_0 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_BOUNDS_LONG_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_BOUNDS_LONG_1 {
    pub UnsignedMinimum: u32,
    pub UnsignedMaximum: u32,
}
impl KSPROPERTY_BOUNDS_LONG_1 {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONG_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_BOUNDS_LONG_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("UnsignedMinimum", &self.UnsignedMinimum).field("UnsignedMaximum", &self.UnsignedMaximum).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONG_1 {
    fn eq(&self, other: &Self) -> bool {
        self.UnsignedMinimum == other.UnsignedMinimum && self.UnsignedMaximum == other.UnsignedMaximum
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONG_1 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_BOUNDS_LONG_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSPROPERTY_BOUNDS_LONGLONG {
    pub Anonymous1: KSPROPERTY_BOUNDS_LONGLONG_0,
    pub Anonymous2: KSPROPERTY_BOUNDS_LONGLONG_1,
}
impl KSPROPERTY_BOUNDS_LONGLONG {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONGLONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONGLONG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONGLONG {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_BOUNDS_LONGLONG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_BOUNDS_LONGLONG_0 {
    pub SignedMinimum: i64,
    pub SignedMaximum: i64,
}
impl KSPROPERTY_BOUNDS_LONGLONG_0 {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONGLONG_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_BOUNDS_LONGLONG_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("SignedMinimum", &self.SignedMinimum).field("SignedMaximum", &self.SignedMaximum).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONGLONG_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SignedMinimum == other.SignedMinimum && self.SignedMaximum == other.SignedMaximum
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONGLONG_0 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_BOUNDS_LONGLONG_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_BOUNDS_LONGLONG_1 {
    pub UnsignedMinimum: u64,
    pub UnsignedMaximum: u64,
}
impl KSPROPERTY_BOUNDS_LONGLONG_1 {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONGLONG_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_BOUNDS_LONGLONG_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("UnsignedMinimum", &self.UnsignedMinimum).field("UnsignedMaximum", &self.UnsignedMaximum).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONGLONG_1 {
    fn eq(&self, other: &Self) -> bool {
        self.UnsignedMinimum == other.UnsignedMinimum && self.UnsignedMaximum == other.UnsignedMaximum
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONGLONG_1 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_BOUNDS_LONGLONG_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_BTAUDIO(pub i32);
pub const KSPROPERTY_ONESHOT_RECONNECT: KSPROPERTY_BTAUDIO = KSPROPERTY_BTAUDIO(0i32);
pub const KSPROPERTY_ONESHOT_DISCONNECT: KSPROPERTY_BTAUDIO = KSPROPERTY_BTAUDIO(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_BTAUDIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_BTAUDIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(pub i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(0i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(1i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMAXFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(2i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTRIGGERTIME: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(3i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WARMSTART: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(4i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MAXVIDFPS_PHOTORES: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(5i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTHUMBNAIL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(6i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SCENEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(7i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_TORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(8i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FLASHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(9i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OPTIMIZATIONHINT: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(10i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WHITEBALANCEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(11i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EXPOSUREMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(12i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(13i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(14i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FIELDOFVIEW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(15i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EVCOMPENSATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(16i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_CAMERAANGLEOFFSET: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(17i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_METADATA: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(18i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSPRIORITY: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(19i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSSTATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(20i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(21i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_ISPCONTROL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(22i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOCONFIRMATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(23i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ZOOM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(24i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MCC: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(25i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO_ADVANCED: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(26i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOSTABILIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(27i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VFR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(28i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEDETECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(29i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOHDR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(30i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_HISTOGRAM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(31i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OIS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(32i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ADVANCEDPHOTO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(33i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PROFILE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(34i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEAUTH_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(35i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SECURE_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(36i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOTEMPORALDENOISING: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(37i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_IRTORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(38i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_RELATIVEPANELOPTIMIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(39i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EYEGAZECORRECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(40i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_BACKGROUNDSEGMENTATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(41i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(42i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(43i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(44i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END2: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(44i32);
impl ::core::convert::From<i32> for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ABSOLUTE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ASYNCHRONOUS: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_MANUAL: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLAGS_RELATIVE: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CAMERACONTROL_FLASH(pub i32);
pub const KSPROPERTY_CAMERACONTROL_FLASH_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_FLASH = KSPROPERTY_CAMERACONTROL_FLASH(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_CAMERACONTROL_FLASH {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_FLASH {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_AUTO: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_MANUAL: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_OFF: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_ON: i32 = 1i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_FLASH_S {
    pub Flash: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_CAMERACONTROL_FLASH_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_CAMERACONTROL_FLASH_S").field("Flash", &self.Flash).field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn eq(&self, other: &Self) -> bool {
        self.Flash == other.Flash && self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_FLASH_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_FLASH_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    pub Property: KSIDENTIFIER,
    pub lOcularFocalLength: i32,
    pub lObjectiveFocalLengthMin: i32,
    pub lObjectiveFocalLengthMax: i32,
}
impl KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY(pub i32);
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY = KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_EXCLUSIVE_WITH_RECORD: i32 = 1i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    pub Capabilities: u32,
    pub Reserved0: u32,
}
impl KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S").field("Capabilities", &self.Capabilities).field("Reserved0", &self.Reserved0).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_SEQUENCE_EXCLUSIVE_WITH_RECORD: i32 = 2i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    pub NodeProperty: KSNODEPROPERTY,
    pub lOcularFocalLength: i32,
    pub lObjectiveFocalLengthMin: i32,
    pub lObjectiveFocalLengthMax: i32,
}
impl KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_CAMERACONTROL_NODE_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_NODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_NODE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_NODE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_NODE_S2 {
    pub NodeProperty: KSP_NODE,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl KSPROPERTY_CAMERACONTROL_NODE_S2 {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_NODE_S2 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(pub i32);
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CAPABILITY: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(0i32);
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_SET: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(1i32);
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CLEAR: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(2i32);
impl ::core::convert::From<i32> for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST(pub i32);
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST = KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_EXPOSURE: i32 = 512i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_FOCUS: i32 = 256i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_WB: i32 = 1024i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONVERGEMODE: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_ASYNC: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_MANUAL: i32 = 2i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    pub FocusRect: super::super::Foundation::RECT,
    pub AutoFocusLock: super::super::Foundation::BOOL,
    pub AutoExposureLock: super::super::Foundation::BOOL,
    pub AutoWhitebalanceLock: super::super::Foundation::BOOL,
    pub Anonymous: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    pub Capabilities: u32,
    pub Configuration: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_CAMERACONTROL_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_S2 {
    pub Property: KSIDENTIFIER,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl KSPROPERTY_CAMERACONTROL_S2 {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_S2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_S2 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_S2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_CAMERACONTROL_S_EX {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub FocusRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_CAMERACONTROL_S_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_S_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_S_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_S_EX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_AUTO: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_MANUAL: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_HIGH: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_LOW: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_MEDIUM: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_OFF: i32 = 0i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    pub VideoStabilizationMode: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S").field("VideoStabilizationMode", &self.VideoStabilizationMode).field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn eq(&self, other: &Self) -> bool {
        self.VideoStabilizationMode == other.VideoStabilizationMode && self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE(pub i32);
pub const KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE = KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(pub i32);
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_CLEAR: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(0i32);
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_SET: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CLOCK(pub i32);
pub const KSPROPERTY_CLOCK_TIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(0i32);
pub const KSPROPERTY_CLOCK_PHYSICALTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(1i32);
pub const KSPROPERTY_CLOCK_CORRELATEDTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(2i32);
pub const KSPROPERTY_CLOCK_CORRELATEDPHYSICALTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(3i32);
pub const KSPROPERTY_CLOCK_RESOLUTION: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(4i32);
pub const KSPROPERTY_CLOCK_STATE: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(5i32);
impl ::core::convert::From<i32> for KSPROPERTY_CLOCK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CLOCK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CONNECTION(pub i32);
pub const KSPROPERTY_CONNECTION_STATE: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(0i32);
pub const KSPROPERTY_CONNECTION_PRIORITY: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(1i32);
pub const KSPROPERTY_CONNECTION_DATAFORMAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(2i32);
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(3i32);
pub const KSPROPERTY_CONNECTION_PROPOSEDATAFORMAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(4i32);
pub const KSPROPERTY_CONNECTION_ACQUIREORDERING: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(5i32);
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING_EX: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(6i32);
pub const KSPROPERTY_CONNECTION_STARTAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(7i32);
impl ::core::convert::From<i32> for KSPROPERTY_CONNECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CONNECTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_COPYPROT(pub i32);
pub const KSPROPERTY_DVDCOPY_CHLG_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(1i32);
pub const KSPROPERTY_DVDCOPY_DVD_KEY1: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(2i32);
pub const KSPROPERTY_DVDCOPY_DEC_KEY2: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(3i32);
pub const KSPROPERTY_DVDCOPY_TITLE_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(4i32);
pub const KSPROPERTY_COPY_MACROVISION: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(5i32);
pub const KSPROPERTY_DVDCOPY_REGION: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(6i32);
pub const KSPROPERTY_DVDCOPY_SET_COPY_STATE: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(7i32);
pub const KSPROPERTY_DVDCOPY_DISC_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(128i32);
impl ::core::convert::From<i32> for KSPROPERTY_COPYPROT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_COPYPROT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CROSSBAR_ACTIVE_S {
    pub Property: KSIDENTIFIER,
    pub IndexInputPin: u32,
    pub Active: u32,
}
impl KSPROPERTY_CROSSBAR_ACTIVE_S {}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_ACTIVE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CROSSBAR_ACTIVE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CROSSBAR_ACTIVE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CROSSBAR_ACTIVE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CROSSBAR_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfInputs: u32,
    pub NumberOfOutputs: u32,
}
impl KSPROPERTY_CROSSBAR_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CROSSBAR_CAPS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CROSSBAR_CAPS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CROSSBAR_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CROSSBAR_PININFO_S {
    pub Property: KSIDENTIFIER,
    pub Direction: KSPIN_DATAFLOW,
    pub Index: u32,
    pub PinType: u32,
    pub RelatedPinIndex: u32,
    pub Medium: KSIDENTIFIER,
}
impl KSPROPERTY_CROSSBAR_PININFO_S {}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_PININFO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CROSSBAR_PININFO_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CROSSBAR_PININFO_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CROSSBAR_PININFO_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_CROSSBAR_ROUTE_S {
    pub Property: KSIDENTIFIER,
    pub IndexInputPin: u32,
    pub IndexOutputPin: u32,
    pub CanRoute: u32,
}
impl KSPROPERTY_CROSSBAR_ROUTE_S {}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_ROUTE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CROSSBAR_ROUTE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CROSSBAR_ROUTE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CROSSBAR_ROUTE_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_CYCLIC(pub i32);
pub const KSPROPERTY_CYCLIC_POSITION: KSPROPERTY_CYCLIC = KSPROPERTY_CYCLIC(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_CYCLIC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_CYCLIC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_DESCRIPTION {
    pub AccessFlags: u32,
    pub DescriptionSize: u32,
    pub PropTypeSet: KSIDENTIFIER,
    pub MembersListCount: u32,
    pub Reserved: u32,
}
impl KSPROPERTY_DESCRIPTION {}
impl ::core::default::Default for KSPROPERTY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_DESCRIPTION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_DESCRIPTION {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_DESCRIPTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_DIRECTSOUND3DBUFFER(pub i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_ALL: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(0i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_POSITION: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(1i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_VELOCITY: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(2i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEANGLES: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(3i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEORIENTATION: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(4i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEOUTSIDEVOLUME: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(5i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MINDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(6i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MAXDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(7i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MODE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(8i32);
impl ::core::convert::From<i32> for KSPROPERTY_DIRECTSOUND3DBUFFER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_DIRECTSOUND3DBUFFER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_DIRECTSOUND3DLISTENER(pub i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALL: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(0i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_POSITION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(1i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_VELOCITY: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(2i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ORIENTATION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(3i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DISTANCEFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(4i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ROLLOFFFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(5i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DOPPLERFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(6i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_BATCH: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(7i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALLOCATION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(8i32);
impl ::core::convert::From<i32> for KSPROPERTY_DIRECTSOUND3DLISTENER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_DIRECTSOUND3DLISTENER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_DRMAUDIOSTREAM(pub i32);
pub const KSPROPERTY_DRMAUDIOSTREAM_CONTENTID: KSPROPERTY_DRMAUDIOSTREAM = KSPROPERTY_DRMAUDIOSTREAM(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_DRMAUDIOSTREAM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_DRMAUDIOSTREAM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    pub Property: KSIDENTIFIER,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub AverageFrameSize: u32,
}
impl KSPROPERTY_DROPPEDFRAMES_CURRENT_S {}
impl ::core::default::Default for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_DVDSUBPIC(pub i32);
pub const KSPROPERTY_DVDSUBPIC_PALETTE: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(0i32);
pub const KSPROPERTY_DVDSUBPIC_HLI: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(1i32);
pub const KSPROPERTY_DVDSUBPIC_COMPOSIT_ON: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(2i32);
impl ::core::convert::From<i32> for KSPROPERTY_DVDSUBPIC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_DVDSUBPIC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_EXTDEVICE(pub i32);
pub const KSPROPERTY_EXTDEVICE_ID: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(0i32);
pub const KSPROPERTY_EXTDEVICE_VERSION: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(1i32);
pub const KSPROPERTY_EXTDEVICE_POWER_STATE: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(2i32);
pub const KSPROPERTY_EXTDEVICE_PORT: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(3i32);
pub const KSPROPERTY_EXTDEVICE_CAPABILITIES: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(4i32);
impl ::core::convert::From<i32> for KSPROPERTY_EXTDEVICE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTDEVICE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_EXTDEVICE_S {
    pub Property: KSIDENTIFIER,
    pub u: KSPROPERTY_EXTDEVICE_S_0,
}
impl KSPROPERTY_EXTDEVICE_S {}
impl ::core::default::Default for KSPROPERTY_EXTDEVICE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_EXTDEVICE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_EXTDEVICE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTDEVICE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSPROPERTY_EXTDEVICE_S_0 {
    pub Capabilities: DEVCAPS,
    pub DevPort: u32,
    pub PowerState: u32,
    pub pawchString: [u16; 260],
    pub NodeUniqueID: [u32; 2],
}
impl KSPROPERTY_EXTDEVICE_S_0 {}
impl ::core::default::Default for KSPROPERTY_EXTDEVICE_S_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_EXTDEVICE_S_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_EXTDEVICE_S_0 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTDEVICE_S_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_EXTENSION_UNIT(pub i32);
pub const KSPROPERTY_EXTENSION_UNIT_INFO: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(0i32);
pub const KSPROPERTY_EXTENSION_UNIT_CONTROL: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(1i32);
pub const KSPROPERTY_EXTENSION_UNIT_PASS_THROUGH: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(65535i32);
impl ::core::convert::From<i32> for KSPROPERTY_EXTENSION_UNIT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTENSION_UNIT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_EXTXPORT(pub i32);
pub const KSPROPERTY_EXTXPORT_CAPABILITIES: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(0i32);
pub const KSPROPERTY_EXTXPORT_INPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(1i32);
pub const KSPROPERTY_EXTXPORT_OUTPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(2i32);
pub const KSPROPERTY_EXTXPORT_LOAD_MEDIUM: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(3i32);
pub const KSPROPERTY_EXTXPORT_MEDIUM_INFO: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(4i32);
pub const KSPROPERTY_EXTXPORT_STATE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(5i32);
pub const KSPROPERTY_EXTXPORT_STATE_NOTIFY: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(6i32);
pub const KSPROPERTY_EXTXPORT_TIMECODE_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(7i32);
pub const KSPROPERTY_EXTXPORT_ATN_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(8i32);
pub const KSPROPERTY_EXTXPORT_RTC_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(9i32);
pub const KSPROPERTY_RAW_AVC_CMD: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(10i32);
impl ::core::convert::From<i32> for KSPROPERTY_EXTXPORT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_EXTXPORT_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub u: KSPROPERTY_EXTXPORT_NODE_S_0,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_EXTXPORT_NODE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_NODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_NODE_S {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT_NODE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl KSPROPERTY_EXTXPORT_NODE_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_NODE_S_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_NODE_S_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT_NODE_S_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    pub PayloadSize: u32,
    pub Payload: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_EXTXPORT_NODE_S_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_RawAVC_e__Struct").field("PayloadSize", &self.PayloadSize).field("Payload", &self.Payload).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Payload == other.Payload
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_NODE_S_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_EXTXPORT_NODE_S_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Timecode_e__Struct").field("frame", &self.frame).field("second", &self.second).field("minute", &self.minute).field("hour", &self.hour).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.frame == other.frame && self.second == other.second && self.minute == other.minute && self.hour == other.hour
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_NODE_S_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_EXTXPORT_S {
    pub Property: KSIDENTIFIER,
    pub u: KSPROPERTY_EXTXPORT_S_0,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_EXTXPORT_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_S {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl KSPROPERTY_EXTXPORT_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_S_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_S_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT_S_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_S_0_0 {
    pub PayloadSize: u32,
    pub Payload: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_EXTXPORT_S_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT_S_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_RawAVC_e__Struct").field("PayloadSize", &self.PayloadSize).field("Payload", &self.Payload).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_S_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Payload == other.Payload
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_S_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT_S_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_S_0_1 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_EXTXPORT_S_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT_S_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Timecode_e__Struct").field("frame", &self.frame).field("second", &self.second).field("minute", &self.minute).field("hour", &self.hour).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_S_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.frame == other.frame && self.second == other.second && self.minute == other.minute && self.hour == other.hour
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_S_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_EXTXPORT_S_0_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_FMRX_CONTROL(pub i32);
pub const KSPROPERTY_FMRX_STATE: KSPROPERTY_FMRX_CONTROL = KSPROPERTY_FMRX_CONTROL(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_FMRX_CONTROL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_FMRX_CONTROL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_FMRX_TOPOLOGY(pub i32);
pub const KSPROPERTY_FMRX_ENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(0i32);
pub const KSPROPERTY_FMRX_VOLUME: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(1i32);
pub const KSPROPERTY_FMRX_ANTENNAENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(2i32);
impl ::core::convert::From<i32> for KSPROPERTY_FMRX_TOPOLOGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_FMRX_TOPOLOGY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_GENERAL(pub i32);
pub const KSPROPERTY_GENERAL_COMPONENTID: KSPROPERTY_GENERAL = KSPROPERTY_GENERAL(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_GENERAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_GENERAL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_HRTF3D(pub i32);
pub const KSPROPERTY_HRTF3D_PARAMS: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(0i32);
pub const KSPROPERTY_HRTF3D_INITIALIZE: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(1i32);
pub const KSPROPERTY_HRTF3D_FILTER_FORMAT: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(2i32);
impl ::core::convert::From<i32> for KSPROPERTY_HRTF3D {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_HRTF3D {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_INTERLEAVEDAUDIO(pub i32);
pub const KSPROPERTY_INTERLEAVEDAUDIO_FORMATINFORMATION: KSPROPERTY_INTERLEAVEDAUDIO = KSPROPERTY_INTERLEAVEDAUDIO(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_INTERLEAVEDAUDIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_INTERLEAVEDAUDIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_ITD3D(pub i32);
pub const KSPROPERTY_ITD3D_PARAMS: KSPROPERTY_ITD3D = KSPROPERTY_ITD3D(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_ITD3D {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_ITD3D {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_JACK(pub i32);
pub const KSPROPERTY_JACK_DESCRIPTION: KSPROPERTY_JACK = KSPROPERTY_JACK(1i32);
pub const KSPROPERTY_JACK_DESCRIPTION2: KSPROPERTY_JACK = KSPROPERTY_JACK(2i32);
pub const KSPROPERTY_JACK_SINK_INFO: KSPROPERTY_JACK = KSPROPERTY_JACK(3i32);
pub const KSPROPERTY_JACK_CONTAINERID: KSPROPERTY_JACK = KSPROPERTY_JACK(4i32);
impl ::core::convert::From<i32> for KSPROPERTY_JACK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_JACK {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_MEDIAAVAILABLE {
    pub Earliest: i64,
    pub Latest: i64,
}
impl KSPROPERTY_MEDIAAVAILABLE {}
impl ::core::default::Default for KSPROPERTY_MEDIAAVAILABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_MEDIAAVAILABLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_MEDIAAVAILABLE").field("Earliest", &self.Earliest).field("Latest", &self.Latest).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_MEDIAAVAILABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Earliest == other.Earliest && self.Latest == other.Latest
    }
}
impl ::core::cmp::Eq for KSPROPERTY_MEDIAAVAILABLE {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_MEDIAAVAILABLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_MEDIASEEKING(pub i32);
pub const KSPROPERTY_MEDIASEEKING_CAPABILITIES: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(0i32);
pub const KSPROPERTY_MEDIASEEKING_FORMATS: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(1i32);
pub const KSPROPERTY_MEDIASEEKING_TIMEFORMAT: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(2i32);
pub const KSPROPERTY_MEDIASEEKING_POSITION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(3i32);
pub const KSPROPERTY_MEDIASEEKING_STOPPOSITION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(4i32);
pub const KSPROPERTY_MEDIASEEKING_POSITIONS: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(5i32);
pub const KSPROPERTY_MEDIASEEKING_DURATION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(6i32);
pub const KSPROPERTY_MEDIASEEKING_AVAILABLE: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(7i32);
pub const KSPROPERTY_MEDIASEEKING_PREROLL: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(8i32);
pub const KSPROPERTY_MEDIASEEKING_CONVERTTIMEFORMAT: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(9i32);
impl ::core::convert::From<i32> for KSPROPERTY_MEDIASEEKING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_MEDIASEEKING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_MEMBERSHEADER {
    pub MembersFlags: u32,
    pub MembersSize: u32,
    pub MembersCount: u32,
    pub Flags: u32,
}
impl KSPROPERTY_MEMBERSHEADER {}
impl ::core::default::Default for KSPROPERTY_MEMBERSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_MEMBERSHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_MEMBERSHEADER").field("MembersFlags", &self.MembersFlags).field("MembersSize", &self.MembersSize).field("MembersCount", &self.MembersCount).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_MEMBERSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.MembersFlags == other.MembersFlags && self.MembersSize == other.MembersSize && self.MembersCount == other.MembersCount && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSPROPERTY_MEMBERSHEADER {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_MEMBERSHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_MEMBER_FLAG_BASICSUPPORT_MULTICHANNEL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_MEMBER_FLAG_BASICSUPPORT_UNIFORM: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_MEMBER_FLAG_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_MEMBER_RANGES: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_MEMBER_STEPPEDRANGES: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_MEMBER_VALUES: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_MEMORY_TRANSPORT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_MPEG2VID(pub i32);
pub const KSPROPERTY_MPEG2VID_MODES: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(0i32);
pub const KSPROPERTY_MPEG2VID_CUR_MODE: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(1i32);
pub const KSPROPERTY_MPEG2VID_4_3_RECT: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(2i32);
pub const KSPROPERTY_MPEG2VID_16_9_RECT: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(3i32);
pub const KSPROPERTY_MPEG2VID_16_9_PANSCAN: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(4i32);
impl ::core::convert::From<i32> for KSPROPERTY_MPEG2VID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_MPEG2VID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES(pub i32);
pub const KSPROPERTY_MPEG4_MEDIATYPE_SD_BOX: KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES = KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub EventFilter: [u16; 1],
}
impl KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO").field("Header", &self.Header).field("EventFilter", &self.EventFilter).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.EventFilter == other.EventFilter
    }
}
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    pub MetadataItems: u32,
    pub Size: u32,
    pub PTZStatus: super::super::Foundation::BOOL,
    pub Events: super::super::Foundation::BOOL,
    pub Analytics: super::super::Foundation::BOOL,
    pub Reserved: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO").field("MetadataItems", &self.MetadataItems).field("Size", &self.Size).field("PTZStatus", &self.PTZStatus).field("Events", &self.Events).field("Analytics", &self.Analytics).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MetadataItems == other.MetadataItems && self.Size == other.Size && self.PTZStatus == other.PTZStatus && self.Events == other.Events && self.Analytics == other.Analytics && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE(pub i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE_EVENTSINFO: KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    pub Size: u32,
    pub Type: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE,
}
impl KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER").field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(pub i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_DISABLE: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(0i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_HOSTNTP: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(1i32);
pub const KSPROPERYT_NETWORKCAMERACONTROL_NTPINFO_TYPE_CUSTOM: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(2i32);
impl ::core::convert::From<i32> for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(pub i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTP: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(0i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_URI: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(1i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(2i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_EVENTTOPICS_XML: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(3i32);
impl ::core::convert::From<i32> for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_OVERLAYUPDATE(pub i32);
pub const KSPROPERTY_OVERLAYUPDATE_INTERESTS: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(0i32);
pub const KSPROPERTY_OVERLAYUPDATE_CLIPLIST: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(1i32);
pub const KSPROPERTY_OVERLAYUPDATE_PALETTE: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(2i32);
pub const KSPROPERTY_OVERLAYUPDATE_COLORKEY: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(4i32);
pub const KSPROPERTY_OVERLAYUPDATE_VIDEOPOSITION: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(8i32);
pub const KSPROPERTY_OVERLAYUPDATE_DISPLAYCHANGE: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(16i32);
pub const KSPROPERTY_OVERLAYUPDATE_COLORREF: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(268435456i32);
impl ::core::convert::From<i32> for KSPROPERTY_OVERLAYUPDATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_OVERLAYUPDATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_PIN(pub i32);
pub const KSPROPERTY_PIN_CINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(0i32);
pub const KSPROPERTY_PIN_CTYPES: KSPROPERTY_PIN = KSPROPERTY_PIN(1i32);
pub const KSPROPERTY_PIN_DATAFLOW: KSPROPERTY_PIN = KSPROPERTY_PIN(2i32);
pub const KSPROPERTY_PIN_DATARANGES: KSPROPERTY_PIN = KSPROPERTY_PIN(3i32);
pub const KSPROPERTY_PIN_DATAINTERSECTION: KSPROPERTY_PIN = KSPROPERTY_PIN(4i32);
pub const KSPROPERTY_PIN_INTERFACES: KSPROPERTY_PIN = KSPROPERTY_PIN(5i32);
pub const KSPROPERTY_PIN_MEDIUMS: KSPROPERTY_PIN = KSPROPERTY_PIN(6i32);
pub const KSPROPERTY_PIN_COMMUNICATION: KSPROPERTY_PIN = KSPROPERTY_PIN(7i32);
pub const KSPROPERTY_PIN_GLOBALCINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(8i32);
pub const KSPROPERTY_PIN_NECESSARYINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(9i32);
pub const KSPROPERTY_PIN_PHYSICALCONNECTION: KSPROPERTY_PIN = KSPROPERTY_PIN(10i32);
pub const KSPROPERTY_PIN_CATEGORY: KSPROPERTY_PIN = KSPROPERTY_PIN(11i32);
pub const KSPROPERTY_PIN_NAME: KSPROPERTY_PIN = KSPROPERTY_PIN(12i32);
pub const KSPROPERTY_PIN_CONSTRAINEDDATARANGES: KSPROPERTY_PIN = KSPROPERTY_PIN(13i32);
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT: KSPROPERTY_PIN = KSPROPERTY_PIN(14i32);
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT2: KSPROPERTY_PIN = KSPROPERTY_PIN(15i32);
pub const KSPROPERTY_PIN_MODEDATAFORMATS: KSPROPERTY_PIN = KSPROPERTY_PIN(16i32);
impl ::core::convert::From<i32> for KSPROPERTY_PIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_PIN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_PIN_FLAGS_ATTRIBUTE_RANGE_AWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_PIN_FLAGS_MASK: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_POSITIONS {
    pub Current: i64,
    pub Stop: i64,
    pub CurrentFlags: KS_SEEKING_FLAGS,
    pub StopFlags: KS_SEEKING_FLAGS,
}
impl KSPROPERTY_POSITIONS {}
impl ::core::default::Default for KSPROPERTY_POSITIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_POSITIONS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_POSITIONS").field("Current", &self.Current).field("Stop", &self.Stop).field("CurrentFlags", &self.CurrentFlags).field("StopFlags", &self.StopFlags).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_POSITIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Current == other.Current && self.Stop == other.Stop && self.CurrentFlags == other.CurrentFlags && self.StopFlags == other.StopFlags
    }
}
impl ::core::cmp::Eq for KSPROPERTY_POSITIONS {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_POSITIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_QUALITY(pub i32);
pub const KSPROPERTY_QUALITY_REPORT: KSPROPERTY_QUALITY = KSPROPERTY_QUALITY(0i32);
pub const KSPROPERTY_QUALITY_ERROR: KSPROPERTY_QUALITY = KSPROPERTY_QUALITY(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_QUALITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_QUALITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_RTAUDIO(pub i32);
pub const KSPROPERTY_RTAUDIO_GETPOSITIONFUNCTION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(0i32);
pub const KSPROPERTY_RTAUDIO_BUFFER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(1i32);
pub const KSPROPERTY_RTAUDIO_HWLATENCY: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(2i32);
pub const KSPROPERTY_RTAUDIO_POSITIONREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(3i32);
pub const KSPROPERTY_RTAUDIO_CLOCKREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(4i32);
pub const KSPROPERTY_RTAUDIO_BUFFER_WITH_NOTIFICATION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(5i32);
pub const KSPROPERTY_RTAUDIO_REGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(6i32);
pub const KSPROPERTY_RTAUDIO_UNREGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(7i32);
pub const KSPROPERTY_RTAUDIO_QUERY_NOTIFICATION_SUPPORT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(8i32);
pub const KSPROPERTY_RTAUDIO_PACKETCOUNT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(9i32);
pub const KSPROPERTY_RTAUDIO_PRESENTATION_POSITION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(10i32);
pub const KSPROPERTY_RTAUDIO_GETREADPACKET: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(11i32);
pub const KSPROPERTY_RTAUDIO_SETWRITEPACKET: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(12i32);
pub const KSPROPERTY_RTAUDIO_PACKETVREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(13i32);
impl ::core::convert::From<i32> for KSPROPERTY_RTAUDIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_RTAUDIO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_SELECTOR_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_SELECTOR_NODE_S {}
impl ::core::default::Default for KSPROPERTY_SELECTOR_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_SELECTOR_NODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SELECTOR_NODE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_SELECTOR_NODE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_SELECTOR_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_SELECTOR_S {}
impl ::core::default::Default for KSPROPERTY_SELECTOR_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_SELECTOR_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SELECTOR_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_SELECTOR_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_SERIAL {
    pub PropTypeSet: KSIDENTIFIER,
    pub Id: u32,
    pub PropertyLength: u32,
}
impl KSPROPERTY_SERIAL {}
impl ::core::default::Default for KSPROPERTY_SERIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_SERIAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SERIAL {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_SERIAL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_SERIALHDR {
    pub PropertySet: ::windows::runtime::GUID,
    pub Count: u32,
}
impl KSPROPERTY_SERIALHDR {}
impl ::core::default::Default for KSPROPERTY_SERIALHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_SERIALHDR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SERIALHDR {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_SERIALHDR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_SOUNDDETECTOR(pub i32);
pub const KSPROPERTY_SOUNDDETECTOR_SUPPORTEDPATTERNS: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(1i32);
pub const KSPROPERTY_SOUNDDETECTOR_PATTERNS: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(2i32);
pub const KSPROPERTY_SOUNDDETECTOR_ARMED: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(3i32);
pub const KSPROPERTY_SOUNDDETECTOR_MATCHRESULT: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(4i32);
pub const KSPROPERTY_SOUNDDETECTOR_RESET: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(5i32);
pub const KSPROPERTY_SOUNDDETECTOR_STREAMINGSUPPORT: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(6i32);
impl ::core::convert::From<i32> for KSPROPERTY_SOUNDDETECTOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_SOUNDDETECTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSPROPERTY_SPHLI {}
impl ::core::default::Default for KSPROPERTY_SPHLI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_SPHLI {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_SPHLI")
            .field("HLISS", &self.HLISS)
            .field("Reserved", &self.Reserved)
            .field("StartPTM", &self.StartPTM)
            .field("EndPTM", &self.EndPTM)
            .field("StartX", &self.StartX)
            .field("StartY", &self.StartY)
            .field("StopX", &self.StopX)
            .field("StopY", &self.StopY)
            .field("ColCon", &self.ColCon)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_SPHLI {
    fn eq(&self, other: &Self) -> bool {
        self.HLISS == other.HLISS && self.Reserved == other.Reserved && self.StartPTM == other.StartPTM && self.EndPTM == other.EndPTM && self.StartX == other.StartX && self.StartY == other.StartY && self.StopX == other.StopX && self.StopY == other.StopY && self.ColCon == other.ColCon
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SPHLI {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_SPHLI {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_SPPAL {
    pub sppal: [KS_DVD_YUV; 16],
}
impl KSPROPERTY_SPPAL {}
impl ::core::default::Default for KSPROPERTY_SPPAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSPROPERTY_SPPAL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSPROPERTY_SPPAL").field("sppal", &self.sppal).finish()
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_SPPAL {
    fn eq(&self, other: &Self) -> bool {
        self.sppal == other.sppal
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SPPAL {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_SPPAL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_STEPPING_LONG {
    pub SteppingDelta: u32,
    pub Reserved: u32,
    pub Bounds: KSPROPERTY_BOUNDS_LONG,
}
impl KSPROPERTY_STEPPING_LONG {}
impl ::core::default::Default for KSPROPERTY_STEPPING_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_STEPPING_LONG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_STEPPING_LONG {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_STEPPING_LONG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_STEPPING_LONGLONG {
    pub SteppingDelta: u64,
    pub Bounds: KSPROPERTY_BOUNDS_LONGLONG,
}
impl KSPROPERTY_STEPPING_LONGLONG {}
impl ::core::default::Default for KSPROPERTY_STEPPING_LONGLONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_STEPPING_LONGLONG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_STEPPING_LONGLONG {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_STEPPING_LONGLONG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_STREAM(pub i32);
pub const KSPROPERTY_STREAM_ALLOCATOR: KSPROPERTY_STREAM = KSPROPERTY_STREAM(0i32);
pub const KSPROPERTY_STREAM_QUALITY: KSPROPERTY_STREAM = KSPROPERTY_STREAM(1i32);
pub const KSPROPERTY_STREAM_DEGRADATION: KSPROPERTY_STREAM = KSPROPERTY_STREAM(2i32);
pub const KSPROPERTY_STREAM_MASTERCLOCK: KSPROPERTY_STREAM = KSPROPERTY_STREAM(3i32);
pub const KSPROPERTY_STREAM_TIMEFORMAT: KSPROPERTY_STREAM = KSPROPERTY_STREAM(4i32);
pub const KSPROPERTY_STREAM_PRESENTATIONTIME: KSPROPERTY_STREAM = KSPROPERTY_STREAM(5i32);
pub const KSPROPERTY_STREAM_PRESENTATIONEXTENT: KSPROPERTY_STREAM = KSPROPERTY_STREAM(6i32);
pub const KSPROPERTY_STREAM_FRAMETIME: KSPROPERTY_STREAM = KSPROPERTY_STREAM(7i32);
pub const KSPROPERTY_STREAM_RATECAPABILITY: KSPROPERTY_STREAM = KSPROPERTY_STREAM(8i32);
pub const KSPROPERTY_STREAM_RATE: KSPROPERTY_STREAM = KSPROPERTY_STREAM(9i32);
pub const KSPROPERTY_STREAM_PIPE_ID: KSPROPERTY_STREAM = KSPROPERTY_STREAM(10i32);
impl ::core::convert::From<i32> for KSPROPERTY_STREAM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_STREAM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_STREAMINTERFACE(pub i32);
pub const KSPROPERTY_STREAMINTERFACE_HEADERSIZE: KSPROPERTY_STREAMINTERFACE = KSPROPERTY_STREAMINTERFACE(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_STREAMINTERFACE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_STREAMINTERFACE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_TELEPHONY_CONTROL(pub i32);
pub const KSPROPERTY_TELEPHONY_PROVIDERID: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(0i32);
pub const KSPROPERTY_TELEPHONY_CALLINFO: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(1i32);
pub const KSPROPERTY_TELEPHONY_CALLCONTROL: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(2i32);
pub const KSPROPERTY_TELEPHONY_PROVIDERCHANGE: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(3i32);
pub const KSPROPERTY_TELEPHONY_CALLHOLD: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(4i32);
pub const KSPROPERTY_TELEPHONY_MUTE_TX: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(5i32);
impl ::core::convert::From<i32> for KSPROPERTY_TELEPHONY_CONTROL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TELEPHONY_CONTROL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_TELEPHONY_TOPOLOGY(pub i32);
pub const KSPROPERTY_TELEPHONY_ENDPOINTIDPAIR: KSPROPERTY_TELEPHONY_TOPOLOGY = KSPROPERTY_TELEPHONY_TOPOLOGY(0i32);
pub const KSPROPERTY_TELEPHONY_VOLUME: KSPROPERTY_TELEPHONY_TOPOLOGY = KSPROPERTY_TELEPHONY_TOPOLOGY(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_TELEPHONY_TOPOLOGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TELEPHONY_TOPOLOGY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_TIMECODE(pub i32);
pub const KSPROPERTY_TIMECODE_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(0i32);
pub const KSPROPERTY_ATN_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(1i32);
pub const KSPROPERTY_RTC_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(2i32);
impl ::core::convert::From<i32> for KSPROPERTY_TIMECODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TIMECODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TIMECODE_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub TimecodeSamp: super::TIMECODE_SAMPLE,
}
impl KSPROPERTY_TIMECODE_NODE_S {}
impl ::core::default::Default for KSPROPERTY_TIMECODE_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TIMECODE_NODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TIMECODE_NODE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TIMECODE_NODE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TIMECODE_S {
    pub Property: KSIDENTIFIER,
    pub TimecodeSamp: super::TIMECODE_SAMPLE,
}
impl KSPROPERTY_TIMECODE_S {}
impl ::core::default::Default for KSPROPERTY_TIMECODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TIMECODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TIMECODE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TIMECODE_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_TOPOLOGY(pub i32);
pub const KSPROPERTY_TOPOLOGY_CATEGORIES: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(0i32);
pub const KSPROPERTY_TOPOLOGY_NODES: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(1i32);
pub const KSPROPERTY_TOPOLOGY_CONNECTIONS: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(2i32);
pub const KSPROPERTY_TOPOLOGY_NAME: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(3i32);
impl ::core::convert::From<i32> for KSPROPERTY_TOPOLOGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TOPOLOGY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_TOPOLOGYNODE(pub i32);
pub const KSPROPERTY_TOPOLOGYNODE_ENABLE: KSPROPERTY_TOPOLOGYNODE = KSPROPERTY_TOPOLOGYNODE(1i32);
pub const KSPROPERTY_TOPOLOGYNODE_RESET: KSPROPERTY_TOPOLOGYNODE = KSPROPERTY_TOPOLOGYNODE(2i32);
impl ::core::convert::From<i32> for KSPROPERTY_TOPOLOGYNODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TOPOLOGYNODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_TUNER(pub i32);
pub const KSPROPERTY_TUNER_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(0i32);
pub const KSPROPERTY_TUNER_MODE_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(1i32);
pub const KSPROPERTY_TUNER_MODE: KSPROPERTY_TUNER = KSPROPERTY_TUNER(2i32);
pub const KSPROPERTY_TUNER_STANDARD: KSPROPERTY_TUNER = KSPROPERTY_TUNER(3i32);
pub const KSPROPERTY_TUNER_FREQUENCY: KSPROPERTY_TUNER = KSPROPERTY_TUNER(4i32);
pub const KSPROPERTY_TUNER_INPUT: KSPROPERTY_TUNER = KSPROPERTY_TUNER(5i32);
pub const KSPROPERTY_TUNER_STATUS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(6i32);
pub const KSPROPERTY_TUNER_IF_MEDIUM: KSPROPERTY_TUNER = KSPROPERTY_TUNER(7i32);
pub const KSPROPERTY_TUNER_SCAN_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(8i32);
pub const KSPROPERTY_TUNER_SCAN_STATUS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(9i32);
pub const KSPROPERTY_TUNER_STANDARD_MODE: KSPROPERTY_TUNER = KSPROPERTY_TUNER(10i32);
pub const KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(11i32);
impl ::core::convert::From<i32> for KSPROPERTY_TUNER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TUNER_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub ModesSupported: u32,
    pub VideoMedium: KSIDENTIFIER,
    pub TVAudioMedium: KSIDENTIFIER,
    pub RadioAudioMedium: KSIDENTIFIER,
}
impl KSPROPERTY_TUNER_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_CAPS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_CAPS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSPROPERTY_TUNER_FREQUENCY_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_FREQUENCY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_FREQUENCY_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_FREQUENCY_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_FREQUENCY_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TUNER_IF_MEDIUM_S {
    pub Property: KSIDENTIFIER,
    pub IFMedium: KSIDENTIFIER,
}
impl KSPROPERTY_TUNER_IF_MEDIUM_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_IF_MEDIUM_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_IF_MEDIUM_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_IF_MEDIUM_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_IF_MEDIUM_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TUNER_INPUT_S {
    pub Property: KSIDENTIFIER,
    pub InputIndex: u32,
}
impl KSPROPERTY_TUNER_INPUT_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_INPUT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_INPUT_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_INPUT_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_INPUT_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_TUNER_MODES(pub i32);
pub const KSPROPERTY_TUNER_MODE_TV: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(1i32);
pub const KSPROPERTY_TUNER_MODE_FM_RADIO: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(2i32);
pub const KSPROPERTY_TUNER_MODE_AM_RADIO: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(4i32);
pub const KSPROPERTY_TUNER_MODE_DSS: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(8i32);
pub const KSPROPERTY_TUNER_MODE_ATSC: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(16i32);
impl ::core::convert::From<i32> for KSPROPERTY_TUNER_MODES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_MODES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSPROPERTY_TUNER_MODE_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_MODE_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_MODE_CAPS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_MODE_CAPS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_MODE_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TUNER_MODE_S {
    pub Property: KSIDENTIFIER,
    pub Mode: u32,
}
impl KSPROPERTY_TUNER_MODE_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_MODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_MODE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_MODE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub NetworkType: ::windows::runtime::GUID,
    pub BufferSize: u32,
    pub NetworkTunerCapabilities: *mut ::core::ffi::c_void,
}
impl KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_TUNER_SCAN_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub fSupportsHardwareAssistedScanning: super::super::Foundation::BOOL,
    pub SupportedBroadcastStandards: u32,
    pub GUIDBucket: *mut ::core::ffi::c_void,
    pub lengthofBucket: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_TUNER_SCAN_CAPS_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_TUNER_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_SCAN_CAPS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_TUNER_SCAN_CAPS_S {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_SCAN_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TUNER_SCAN_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub LockStatus: _TunerDecoderLockType,
    pub CurrentFrequency: u32,
}
impl KSPROPERTY_TUNER_SCAN_STATUS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_SCAN_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_SCAN_STATUS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_SCAN_STATUS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_SCAN_STATUS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_TUNER_STANDARD_MODE_S {
    pub Property: KSIDENTIFIER,
    pub AutoDetect: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_TUNER_STANDARD_MODE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_TUNER_STANDARD_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_STANDARD_MODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_TUNER_STANDARD_MODE_S {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_STANDARD_MODE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TUNER_STANDARD_S {
    pub Property: KSIDENTIFIER,
    pub Standard: u32,
}
impl KSPROPERTY_TUNER_STANDARD_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_STANDARD_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_STANDARD_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_STANDARD_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_STANDARD_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TUNER_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub CurrentFrequency: u32,
    pub PLLOffset: u32,
    pub SignalStrength: u32,
    pub Busy: u32,
}
impl KSPROPERTY_TUNER_STATUS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_STATUS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_STATUS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TUNER_STATUS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TVAUDIO_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub Capabilities: u32,
    pub InputMedium: KSIDENTIFIER,
    pub OutputMedium: KSIDENTIFIER,
}
impl KSPROPERTY_TVAUDIO_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_TVAUDIO_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TVAUDIO_CAPS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TVAUDIO_CAPS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TVAUDIO_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_TVAUDIO_S {
    pub Property: KSIDENTIFIER,
    pub Mode: u32,
}
impl KSPROPERTY_TVAUDIO_S {}
impl ::core::default::Default for KSPROPERTY_TVAUDIO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_TVAUDIO_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TVAUDIO_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_TVAUDIO_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_BASICSUPPORT: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_COPYPAYLOAD: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_DEFAULTVALUES: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_FSFILTERSCOPE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_GET: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_GETPAYLOADSIZE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_HIGHPRIORITY: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_RELATIONS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_SERIALIZERAW: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_SERIALIZESET: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_SERIALIZESIZE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_SET: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_SETSUPPORT: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_TOPOLOGY: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_UNSERIALIZERAW: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_TYPE_UNSERIALIZESET: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VBICAP(pub i32);
pub const KSPROPERTY_VBICAP_PROPERTIES_PROTECTION: KSPROPERTY_VBICAP = KSPROPERTY_VBICAP(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_VBICAP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICAP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VBICODECFILTERING(pub i32);
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(1i32);
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(2i32);
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(3i32);
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(4i32);
pub const KSPROPERTY_VBICODECFILTERING_STATISTICS: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(5i32);
impl ::core::convert::From<i32> for KSPROPERTY_VBICODECFILTERING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    pub Property: KSIDENTIFIER,
    pub Substreams: VBICODECFILTERING_CC_SUBSTREAMS,
}
impl KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    pub Property: KSIDENTIFIER,
    pub Substreams: VBICODECFILTERING_NABTS_SUBSTREAMS,
}
impl KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    pub Property: KSIDENTIFIER,
    pub Scanlines: VBICODECFILTERING_SCANLINES,
}
impl KSPROPERTY_VBICODECFILTERING_SCANLINES_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_CC_PIN,
}
impl KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_CC,
}
impl KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_COMMON,
}
impl KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_NABTS_PIN,
}
impl KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_NABTS,
}
impl KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_CAMERACONTROL(pub i32);
pub const KSPROPERTY_CAMERACONTROL_PAN: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(0i32);
pub const KSPROPERTY_CAMERACONTROL_TILT: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(1i32);
pub const KSPROPERTY_CAMERACONTROL_ROLL: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(2i32);
pub const KSPROPERTY_CAMERACONTROL_ZOOM: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(3i32);
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(4i32);
pub const KSPROPERTY_CAMERACONTROL_IRIS: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(5i32);
pub const KSPROPERTY_CAMERACONTROL_FOCUS: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(6i32);
pub const KSPROPERTY_CAMERACONTROL_SCANMODE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(7i32);
pub const KSPROPERTY_CAMERACONTROL_PRIVACY: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(8i32);
pub const KSPROPERTY_CAMERACONTROL_PANTILT: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(9i32);
pub const KSPROPERTY_CAMERACONTROL_PAN_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(10i32);
pub const KSPROPERTY_CAMERACONTROL_TILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(11i32);
pub const KSPROPERTY_CAMERACONTROL_ROLL_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(12i32);
pub const KSPROPERTY_CAMERACONTROL_ZOOM_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(13i32);
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(14i32);
pub const KSPROPERTY_CAMERACONTROL_IRIS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(15i32);
pub const KSPROPERTY_CAMERACONTROL_FOCUS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(16i32);
pub const KSPROPERTY_CAMERACONTROL_PANTILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(17i32);
pub const KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(18i32);
pub const KSPROPERTY_CAMERACONTROL_AUTO_EXPOSURE_PRIORITY: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(19i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_CAMERACONTROL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_CAMERACONTROL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_CROSSBAR(pub i32);
pub const KSPROPERTY_CROSSBAR_CAPS: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(0i32);
pub const KSPROPERTY_CROSSBAR_PININFO: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(1i32);
pub const KSPROPERTY_CROSSBAR_CAN_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(2i32);
pub const KSPROPERTY_CROSSBAR_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(3i32);
pub const KSPROPERTY_CROSSBAR_INPUT_ACTIVE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(4i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_CROSSBAR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_CROSSBAR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_DROPPEDFRAMES(pub i32);
pub const KSPROPERTY_DROPPEDFRAMES_CURRENT: KSPROPERTY_VIDCAP_DROPPEDFRAMES = KSPROPERTY_VIDCAP_DROPPEDFRAMES(0i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_DROPPEDFRAMES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_DROPPEDFRAMES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_SELECTOR(pub i32);
pub const KSPROPERTY_SELECTOR_SOURCE_NODE_ID: KSPROPERTY_VIDCAP_SELECTOR = KSPROPERTY_VIDCAP_SELECTOR(0i32);
pub const KSPROPERTY_SELECTOR_NUM_SOURCES: KSPROPERTY_VIDCAP_SELECTOR = KSPROPERTY_VIDCAP_SELECTOR(1i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_SELECTOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_SELECTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_TVAUDIO(pub i32);
pub const KSPROPERTY_TVAUDIO_CAPS: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(0i32);
pub const KSPROPERTY_TVAUDIO_MODE: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(1i32);
pub const KSPROPERTY_TVAUDIO_CURRENTLY_AVAILABLE_MODES: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(2i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_TVAUDIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_TVAUDIO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(pub i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_GETINFO: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(0i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_KEYFRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(1i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_PFRAMES_PER_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(2i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_QUALITY: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(3i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(4i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_FRAME_SIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(5i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_WINDOWSIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(6i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_VIDEOCONTROL(pub i32);
pub const KSPROPERTY_VIDEOCONTROL_CAPS: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(0i32);
pub const KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(1i32);
pub const KSPROPERTY_VIDEOCONTROL_FRAME_RATES: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(2i32);
pub const KSPROPERTY_VIDEOCONTROL_MODE: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(3i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_VIDEOCONTROL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_VIDEOCONTROL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_VIDEODECODER(pub i32);
pub const KSPROPERTY_VIDEODECODER_CAPS: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(0i32);
pub const KSPROPERTY_VIDEODECODER_STANDARD: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(1i32);
pub const KSPROPERTY_VIDEODECODER_STATUS: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(2i32);
pub const KSPROPERTY_VIDEODECODER_OUTPUT_ENABLE: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(3i32);
pub const KSPROPERTY_VIDEODECODER_VCR_TIMING: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(4i32);
pub const KSPROPERTY_VIDEODECODER_STATUS2: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(5i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_VIDEODECODER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_VIDEODECODER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_VIDEOENCODER(pub i32);
pub const KSPROPERTY_VIDEOENCODER_CAPS: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(0i32);
pub const KSPROPERTY_VIDEOENCODER_STANDARD: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(1i32);
pub const KSPROPERTY_VIDEOENCODER_COPYPROTECTION: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(2i32);
pub const KSPROPERTY_VIDEOENCODER_CC_ENABLE: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(3i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_VIDEOENCODER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_VIDEOENCODER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDCAP_VIDEOPROCAMP(pub i32);
pub const KSPROPERTY_VIDEOPROCAMP_BRIGHTNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(0i32);
pub const KSPROPERTY_VIDEOPROCAMP_CONTRAST: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(1i32);
pub const KSPROPERTY_VIDEOPROCAMP_HUE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(2i32);
pub const KSPROPERTY_VIDEOPROCAMP_SATURATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(3i32);
pub const KSPROPERTY_VIDEOPROCAMP_SHARPNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(4i32);
pub const KSPROPERTY_VIDEOPROCAMP_GAMMA: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(5i32);
pub const KSPROPERTY_VIDEOPROCAMP_COLORENABLE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(6i32);
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(7i32);
pub const KSPROPERTY_VIDEOPROCAMP_BACKLIGHT_COMPENSATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(8i32);
pub const KSPROPERTY_VIDEOPROCAMP_GAIN: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(9i32);
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(10i32);
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER_LIMIT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(11i32);
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE_COMPONENT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(12i32);
pub const KSPROPERTY_VIDEOPROCAMP_POWERLINE_FREQUENCY: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(13i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDCAP_VIDEOPROCAMP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDCAP_VIDEOPROCAMP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub DefaultKeyFrameRate: i32,
    pub DefaultPFrameRate: i32,
    pub DefaultQuality: i32,
    pub NumberOfQualitySettings: i32,
    pub Capabilities: i32,
}
impl KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOCOMPRESSION_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Value: i32,
}
impl KSPROPERTY_VIDEOCOMPRESSION_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCOMPRESSION_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCOMPRESSION_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOCOMPRESSION_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOCOMPRESSION_S1 {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Value: i32,
    pub Flags: u32,
}
impl KSPROPERTY_VIDEOCOMPRESSION_S1 {}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCOMPRESSION_S1 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::super::Foundation::SIZE,
    pub CurrentActualFrameRate: i64,
    pub CurrentMaxAvailableFrameRate: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOCONTROL_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub VideoControlCaps: u32,
}
impl KSPROPERTY_VIDEOCONTROL_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCONTROL_CAPS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOCONTROL_MODE_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Mode: i32,
}
impl KSPROPERTY_VIDEOCONTROL_MODE_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCONTROL_MODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCONTROL_MODE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOCONTROL_MODE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEODECODER_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub StandardsSupported: u32,
    pub Capabilities: u32,
    pub SettlingTime: u32,
    pub HSyncPerVSync: u32,
}
impl KSPROPERTY_VIDEODECODER_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEODECODER_CAPS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEODECODER_CAPS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEODECODER_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEODECODER_S {
    pub Property: KSIDENTIFIER,
    pub Value: u32,
}
impl KSPROPERTY_VIDEODECODER_S {}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEODECODER_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEODECODER_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEODECODER_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEODECODER_STATUS2_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfLines: u32,
    pub SignalLocked: u32,
    pub ChromaLock: u32,
}
impl KSPROPERTY_VIDEODECODER_STATUS2_S {}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_STATUS2_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEODECODER_STATUS2_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEODECODER_STATUS2_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEODECODER_STATUS2_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEODECODER_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfLines: u32,
    pub SignalLocked: u32,
}
impl KSPROPERTY_VIDEODECODER_STATUS_S {}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEODECODER_STATUS_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEODECODER_STATUS_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEODECODER_STATUS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOENCODER_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_VIDEOENCODER_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOENCODER_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOENCODER_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOENCODER_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOENCODER_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_MANUAL: i32 = 2i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_VIDEOPROCAMP_NODE_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOPROCAMP_NODE_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    pub NodeProperty: KSP_NODE,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl KSPROPERTY_VIDEOPROCAMP_NODE_S2 {}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOPROCAMP_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl KSPROPERTY_VIDEOPROCAMP_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOPROCAMP_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOPROCAMP_S {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOPROCAMP_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSPROPERTY_VIDEOPROCAMP_S2 {
    pub Property: KSIDENTIFIER,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl KSPROPERTY_VIDEOPROCAMP_S2 {}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOPROCAMP_S2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOPROCAMP_S2 {}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDEOPROCAMP_S2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VIDMEM_TRANSPORT(pub i32);
pub const KSPROPERTY_DISPLAY_ADAPTER_GUID: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(1i32);
pub const KSPROPERTY_PREFERRED_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(2i32);
pub const KSPROPERTY_CURRENT_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(3i32);
pub const KSPROPERTY_MAP_CAPTURE_HANDLE_TO_VRAM_ADDRESS: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(4i32);
impl ::core::convert::From<i32> for KSPROPERTY_VIDMEM_TRANSPORT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VIDMEM_TRANSPORT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_VPCONFIG(pub i32);
pub const KSPROPERTY_VPCONFIG_NUMCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(0i32);
pub const KSPROPERTY_VPCONFIG_GETCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(1i32);
pub const KSPROPERTY_VPCONFIG_SETCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(2i32);
pub const KSPROPERTY_VPCONFIG_VPDATAINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(3i32);
pub const KSPROPERTY_VPCONFIG_MAXPIXELRATE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(4i32);
pub const KSPROPERTY_VPCONFIG_INFORMVPINPUT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(5i32);
pub const KSPROPERTY_VPCONFIG_NUMVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(6i32);
pub const KSPROPERTY_VPCONFIG_GETVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(7i32);
pub const KSPROPERTY_VPCONFIG_SETVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(8i32);
pub const KSPROPERTY_VPCONFIG_INVERTPOLARITY: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(9i32);
pub const KSPROPERTY_VPCONFIG_DECIMATIONCAPABILITY: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(10i32);
pub const KSPROPERTY_VPCONFIG_SCALEFACTOR: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(11i32);
pub const KSPROPERTY_VPCONFIG_DDRAWHANDLE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(12i32);
pub const KSPROPERTY_VPCONFIG_VIDEOPORTID: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(13i32);
pub const KSPROPERTY_VPCONFIG_DDRAWSURFACEHANDLE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(14i32);
pub const KSPROPERTY_VPCONFIG_SURFACEPARAMS: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(15i32);
impl ::core::convert::From<i32> for KSPROPERTY_VPCONFIG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_VPCONFIG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_WAVE(pub i32);
pub const KSPROPERTY_WAVE_COMPATIBLE_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(0i32);
pub const KSPROPERTY_WAVE_INPUT_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(1i32);
pub const KSPROPERTY_WAVE_OUTPUT_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(2i32);
pub const KSPROPERTY_WAVE_BUFFER: KSPROPERTY_WAVE = KSPROPERTY_WAVE(3i32);
pub const KSPROPERTY_WAVE_FREQUENCY: KSPROPERTY_WAVE = KSPROPERTY_WAVE(4i32);
pub const KSPROPERTY_WAVE_VOLUME: KSPROPERTY_WAVE = KSPROPERTY_WAVE(5i32);
pub const KSPROPERTY_WAVE_PAN: KSPROPERTY_WAVE = KSPROPERTY_WAVE(6i32);
impl ::core::convert::From<i32> for KSPROPERTY_WAVE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_WAVE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_WAVE_QUEUED_POSITION: u32 = 1u32;
pub const KSPROPSETID_AC3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3215714080, 28191, 4560, [188, 242, 68, 69, 83, 84, 0, 0]);
pub const KSPROPSETID_Audio: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1174383264, 28187, 4560, [188, 242, 68, 69, 83, 84, 0, 0]);
pub const KSPROPSETID_AudioBufferDuration: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1316208767, 9164, 18773, [167, 234, 61, 165, 2, 73, 98, 144]);
pub const KSPROPSETID_AudioDecoderOut: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1822875680, 17341, 4560, [189, 106, 0, 53, 5, 193, 3, 169]);
pub const KSPROPSETID_AudioEngine: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(976192220, 34927, 19370, [158, 180, 8, 43, 144, 37, 197, 54]);
pub const KSPROPSETID_AudioModule: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3224698288, 65397, 18376, [170, 60, 238, 70, 113, 107, 80, 198]);
pub const KSPROPSETID_AudioPosture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2751167245, 18254, 20305, [163, 121, 81, 40, 45, 212, 250, 143]);
pub const KSPROPSETID_AudioResourceManagement: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3501393377, 45772, 18508, [143, 35, 229, 210, 138, 217, 207, 136]);
pub const KSPROPSETID_AudioSignalProcessing: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1332196648, 12489, 16606, [178, 251, 133, 157, 221, 31, 52, 112]);
pub const KSPROPSETID_Bibliographic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(129635598, 58033, 4560, [172, 23, 0, 160, 201, 34, 49, 150]);
pub const KSPROPSETID_BtAudio: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2141219904, 47350, 19582, [133, 86, 232, 195, 58, 18, 229, 77]);
pub const KSPROPSETID_Clock: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3742540992, 44055, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSPROPSETID_Connection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(492357920, 44187, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSPROPSETID_CopyProt: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(243927616, 27375, 4560, [158, 208, 0, 160, 36, 202, 25, 179]);
pub const KSPROPSETID_Cyclic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1073655456, 11246, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSPROPSETID_DirectSound3DBuffer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1132147729, 53344, 4560, [133, 131, 0, 192, 79, 217, 186, 243]);
pub const KSPROPSETID_DirectSound3DListener: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1132147732, 53344, 4560, [133, 131, 0, 192, 79, 217, 186, 243]);
pub const KSPROPSETID_DrmAudioStream: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(791449053, 16792, 20396, [186, 41, 97, 187, 5, 183, 222, 6]);
pub const KSPROPSETID_DvdSubPic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2889417824, 17327, 4560, [189, 106, 0, 53, 5, 193, 3, 169]);
pub const KSPROPSETID_FMRXControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2491136570, 59630, 18310, [144, 196, 132, 40, 24, 95, 5, 190]);
pub const KSPROPSETID_FMRXTopology: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(205966991, 56365, 16900, [157, 201, 245, 137, 99, 54, 101, 99]);
pub const KSPROPSETID_General: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(342158757, 27279, 4561, [154, 167, 0, 160, 201, 34, 49, 150]);
pub const KSPROPSETID_Hrtf3d: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3060657328, 41091, 4560, [133, 30, 0, 192, 79, 217, 186, 243]);
pub const KSPROPSETID_InterleavedAudio: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3924550992, 54809, 19466, [151, 107, 112, 98, 50, 43, 48, 6]);
pub const KSPROPSETID_Itd3d: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1680470160, 40921, 4560, [167, 91, 0, 160, 201, 3, 101, 227]);
pub const KSPROPSETID_Jack: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1158281047, 11590, 17975, [142, 98, 206, 125, 185, 68, 245, 123]);
pub const KSPROPSETID_MPEG4_MediaType_Attributes: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4285287418, 1961, 19579, [162, 55, 103, 47, 157, 104, 6, 95]);
pub const KSPROPSETID_MediaSeeking: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4002434828, 53403, 4560, [171, 233, 0, 160, 201, 34, 49, 150]);
pub const KSPROPSETID_MemoryTransport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(171777117, 21059, 18457, [158, 208, 174, 232, 4, 76, 238, 43]);
pub const KSPROPSETID_Mpeg2Vid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3370195808, 3273, 4560, [189, 105, 0, 53, 5, 193, 3, 169]);
pub const KSPROPSETID_OverlayUpdate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1225696719, 30337, 4561, [162, 28, 0, 160, 201, 34, 49, 150]);
pub const KSPROPSETID_Pin: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2350074208, 20909, 4559, [135, 138, 148, 248, 1, 193, 0, 0]);
pub const KSPROPSETID_PinMDLCacheClearProp: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3178334843, 38908, 16583, [136, 206, 211, 255, 6, 245, 91, 22]);
pub const KSPROPSETID_Quality: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3513439104, 44058, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSPROPSETID_RtAudio: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2824184972, 12152, 18217, [144, 81, 25, 104, 116, 107, 158, 239]);
pub const KSPROPSETID_SoundDetector: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(289161822, 64791, 16471, [180, 34, 237, 64, 116, 241, 175, 223]);
pub const KSPROPSETID_SoundDetector2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4261929762, 17676, 19413, [132, 202, 169, 72, 80, 14, 166, 170]);
pub const KSPROPSETID_Stream: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1705687648, 39086, 4559, [161, 13, 0, 32, 175, 209, 86, 228]);
pub const KSPROPSETID_StreamAllocator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3480109890, 60551, 4559, [161, 48, 0, 32, 175, 209, 86, 228]);
pub const KSPROPSETID_StreamInterface: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(534613729, 40147, 4560, [130, 170, 0, 0, 248, 34, 254, 138]);
pub const KSPROPSETID_TSRateChange: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2768487872, 7453, 4561, [173, 128, 68, 69, 83, 84, 0, 0]);
pub const KSPROPSETID_TelephonyControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3068100273, 53401, 18591, [166, 160, 192, 16, 111, 8, 135, 167]);
pub const KSPROPSETID_TelephonyTopology: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2884787326, 3684, 20018, [177, 144, 208, 246, 215, 197, 62, 151]);
pub const KSPROPSETID_Topology: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1913473728, 30003, 4560, [165, 214, 40, 219, 4, 193, 0, 0]);
pub const KSPROPSETID_TopologyNode: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1174383265, 28187, 4560, [188, 242, 68, 69, 83, 84, 0, 0]);
pub const KSPROPSETID_VBICAP_PROPERTIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4049782279, 31541, 18799, [173, 127, 45, 202, 59, 70, 183, 24]);
pub const KSPROPSETID_VBICodecFiltering: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3405689034, 34581, 4560, [189, 106, 0, 53, 192, 237, 186, 190]);
pub const KSPROPSETID_VPConfig: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3156846176, 12515, 4560, [158, 105, 0, 192, 79, 215, 193, 91]);
pub const KSPROPSETID_VPVBIConfig: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3964836608, 6687, 4561, [186, 217, 0, 96, 151, 68, 17, 26]);
pub const KSPROPSETID_VramCapture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3879709923, 10368, 18690, [183, 153, 136, 208, 205, 99, 78, 15]);
pub const KSPROPSETID_Wave: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2454607024, 25359, 4559, [173, 167, 8, 0, 62, 48, 73, 74]);
pub const KSPROPTYPESETID_General: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2548669344, 48618, 4559, [165, 214, 40, 219, 4, 193, 0, 0]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSP_NODE {
    pub Property: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl KSP_NODE {}
impl ::core::default::Default for KSP_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSP_NODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSP_NODE {}
unsafe impl ::windows::runtime::Abi for KSP_NODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSP_PIN {
    pub Property: KSIDENTIFIER,
    pub PinId: u32,
    pub Anonymous: KSP_PIN_0,
}
impl KSP_PIN {}
impl ::core::default::Default for KSP_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSP_PIN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSP_PIN {}
unsafe impl ::windows::runtime::Abi for KSP_PIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSP_PIN_0 {
    pub Reserved: u32,
    pub Flags: u32,
}
impl KSP_PIN_0 {}
impl ::core::default::Default for KSP_PIN_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSP_PIN_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSP_PIN_0 {}
unsafe impl ::windows::runtime::Abi for KSP_PIN_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSP_TIMEFORMAT {
    pub Property: KSIDENTIFIER,
    pub SourceFormat: ::windows::runtime::GUID,
    pub TargetFormat: ::windows::runtime::GUID,
    pub Time: i64,
}
impl KSP_TIMEFORMAT {}
impl ::core::default::Default for KSP_TIMEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSP_TIMEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSP_TIMEFORMAT {}
unsafe impl ::windows::runtime::Abi for KSP_TIMEFORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSQUALITY {
    pub Context: *mut ::core::ffi::c_void,
    pub Proportion: u32,
    pub DeltaTime: i64,
}
impl KSQUALITY {}
impl ::core::default::Default for KSQUALITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSQUALITY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSQUALITY").field("Context", &self.Context).field("Proportion", &self.Proportion).field("DeltaTime", &self.DeltaTime).finish()
    }
}
impl ::core::cmp::PartialEq for KSQUALITY {
    fn eq(&self, other: &Self) -> bool {
        self.Context == other.Context && self.Proportion == other.Proportion && self.DeltaTime == other.DeltaTime
    }
}
impl ::core::cmp::Eq for KSQUALITY {}
unsafe impl ::windows::runtime::Abi for KSQUALITY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSQUALITY_MANAGER {
    pub QualityManager: super::super::Foundation::HANDLE,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl KSQUALITY_MANAGER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSQUALITY_MANAGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSQUALITY_MANAGER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSQUALITY_MANAGER").field("QualityManager", &self.QualityManager).field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSQUALITY_MANAGER {
    fn eq(&self, other: &Self) -> bool {
        self.QualityManager == other.QualityManager && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSQUALITY_MANAGER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSQUALITY_MANAGER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSQUERYBUFFER {
    pub Event: KSIDENTIFIER,
    pub EventData: *mut KSEVENTDATA,
    pub Reserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl KSQUERYBUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSQUERYBUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSQUERYBUFFER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSQUERYBUFFER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSQUERYBUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRATE {
    pub PresentationStart: i64,
    pub Duration: i64,
    pub Interface: KSIDENTIFIER,
    pub Rate: i32,
    pub Flags: u32,
}
impl KSRATE {}
impl ::core::default::Default for KSRATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRATE {}
unsafe impl ::windows::runtime::Abi for KSRATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRATE_CAPABILITY {
    pub Property: KSIDENTIFIER,
    pub Rate: KSRATE,
}
impl KSRATE_CAPABILITY {}
impl ::core::default::Default for KSRATE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRATE_CAPABILITY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRATE_CAPABILITY {}
unsafe impl ::windows::runtime::Abi for KSRATE_CAPABILITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSRATE_NOPRESENTATIONDURATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSRATE_NOPRESENTATIONSTART: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSRELATIVEEVENT {
    pub Size: u32,
    pub Flags: u32,
    pub Anonymous: KSRELATIVEEVENT_0,
    pub Reserved: *mut ::core::ffi::c_void,
    pub Event: KSIDENTIFIER,
    pub EventData: KSEVENTDATA,
}
#[cfg(feature = "Win32_Foundation")]
impl KSRELATIVEEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRELATIVEEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRELATIVEEVENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRELATIVEEVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSRELATIVEEVENT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union KSRELATIVEEVENT_0 {
    pub ObjectHandle: super::super::Foundation::HANDLE,
    pub ObjectPointer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl KSRELATIVEEVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRELATIVEEVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRELATIVEEVENT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRELATIVEEVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSRELATIVEEVENT_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSRELATIVEEVENT_FLAG_HANDLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSRELATIVEEVENT_FLAG_POINTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSRESET(pub i32);
pub const KSRESET_BEGIN: KSRESET = KSRESET(0i32);
pub const KSRESET_END: KSRESET = KSRESET(1i32);
impl ::core::convert::From<i32> for KSRESET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSRESET {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRESOLUTION {
    pub Granularity: i64,
    pub Error: i64,
}
impl KSRESOLUTION {}
impl ::core::default::Default for KSRESOLUTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSRESOLUTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRESOLUTION").field("Granularity", &self.Granularity).field("Error", &self.Error).finish()
    }
}
impl ::core::cmp::PartialEq for KSRESOLUTION {
    fn eq(&self, other: &Self) -> bool {
        self.Granularity == other.Granularity && self.Error == other.Error
    }
}
impl ::core::cmp::Eq for KSRESOLUTION {}
unsafe impl ::windows::runtime::Abi for KSRESOLUTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSRTAUDIO_BUFFER {
    pub BufferAddress: *mut ::core::ffi::c_void,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSRTAUDIO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_BUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRTAUDIO_BUFFER").field("BufferAddress", &self.BufferAddress).field("ActualBufferSize", &self.ActualBufferSize).field("CallMemoryBarrier", &self.CallMemoryBarrier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferAddress == other.BufferAddress && self.ActualBufferSize == other.ActualBufferSize && self.CallMemoryBarrier == other.CallMemoryBarrier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_BUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSRTAUDIO_BUFFER32 {
    pub BufferAddress: u32,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSRTAUDIO_BUFFER32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_BUFFER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_BUFFER32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRTAUDIO_BUFFER32").field("BufferAddress", &self.BufferAddress).field("ActualBufferSize", &self.ActualBufferSize).field("CallMemoryBarrier", &self.CallMemoryBarrier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER32 {
    fn eq(&self, other: &Self) -> bool {
        self.BufferAddress == other.BufferAddress && self.ActualBufferSize == other.ActualBufferSize && self.CallMemoryBarrier == other.CallMemoryBarrier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER32 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_BUFFER32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_BUFFER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub RequestedBufferSize: u32,
}
impl KSRTAUDIO_BUFFER_PROPERTY {}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER_PROPERTY {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_BUFFER_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_BUFFER_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
    pub RequestedBufferSize: u32,
}
impl KSRTAUDIO_BUFFER_PROPERTY32 {}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER_PROPERTY32 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER_PROPERTY32 {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_BUFFER_PROPERTY32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub RequestedBufferSize: u32,
    pub NotificationCount: u32,
}
impl KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
    pub RequestedBufferSize: u32,
    pub NotificationCount: u32,
}
impl KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSRTAUDIO_GETREADPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub PerformanceCounterValue: u64,
    pub MoreData: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl KSRTAUDIO_GETREADPACKET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_GETREADPACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_GETREADPACKET_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRTAUDIO_GETREADPACKET_INFO").field("PacketNumber", &self.PacketNumber).field("Flags", &self.Flags).field("PerformanceCounterValue", &self.PerformanceCounterValue).field("MoreData", &self.MoreData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_GETREADPACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PacketNumber == other.PacketNumber && self.Flags == other.Flags && self.PerformanceCounterValue == other.PerformanceCounterValue && self.MoreData == other.MoreData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_GETREADPACKET_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_GETREADPACKET_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_HWLATENCY {
    pub FifoSize: u32,
    pub ChipsetDelay: u32,
    pub CodecDelay: u32,
}
impl KSRTAUDIO_HWLATENCY {}
impl ::core::default::Default for KSRTAUDIO_HWLATENCY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSRTAUDIO_HWLATENCY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRTAUDIO_HWLATENCY").field("FifoSize", &self.FifoSize).field("ChipsetDelay", &self.ChipsetDelay).field("CodecDelay", &self.CodecDelay).finish()
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWLATENCY {
    fn eq(&self, other: &Self) -> bool {
        self.FifoSize == other.FifoSize && self.ChipsetDelay == other.ChipsetDelay && self.CodecDelay == other.CodecDelay
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWLATENCY {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_HWLATENCY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_HWREGISTER {
    pub Register: *mut ::core::ffi::c_void,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
impl KSRTAUDIO_HWREGISTER {}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSRTAUDIO_HWREGISTER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRTAUDIO_HWREGISTER").field("Register", &self.Register).field("Width", &self.Width).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).field("Accuracy", &self.Accuracy).finish()
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER {
    fn eq(&self, other: &Self) -> bool {
        self.Register == other.Register && self.Width == other.Width && self.Numerator == other.Numerator && self.Denominator == other.Denominator && self.Accuracy == other.Accuracy
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_HWREGISTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_HWREGISTER32 {
    pub Register: u32,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
impl KSRTAUDIO_HWREGISTER32 {}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSRTAUDIO_HWREGISTER32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRTAUDIO_HWREGISTER32").field("Register", &self.Register).field("Width", &self.Width).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).field("Accuracy", &self.Accuracy).finish()
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER32 {
    fn eq(&self, other: &Self) -> bool {
        self.Register == other.Register && self.Width == other.Width && self.Numerator == other.Numerator && self.Denominator == other.Denominator && self.Accuracy == other.Accuracy
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER32 {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_HWREGISTER32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_HWREGISTER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut ::core::ffi::c_void,
}
impl KSRTAUDIO_HWREGISTER_PROPERTY {}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER_PROPERTY {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_HWREGISTER_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_HWREGISTER_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
}
impl KSRTAUDIO_HWREGISTER_PROPERTY32 {}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER_PROPERTY32 {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub NotificationEvent: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub NotificationEvent: u32,
}
impl KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {}
impl ::core::default::Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_PACKETVREGISTER {
    pub CompletedPacketCount: *mut u64,
    pub CompletedPacketQPC: *mut u64,
    pub CompletedPacketHash: *mut u64,
}
impl KSRTAUDIO_PACKETVREGISTER {}
impl ::core::default::Default for KSRTAUDIO_PACKETVREGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSRTAUDIO_PACKETVREGISTER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRTAUDIO_PACKETVREGISTER").field("CompletedPacketCount", &self.CompletedPacketCount).field("CompletedPacketQPC", &self.CompletedPacketQPC).field("CompletedPacketHash", &self.CompletedPacketHash).finish()
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_PACKETVREGISTER {
    fn eq(&self, other: &Self) -> bool {
        self.CompletedPacketCount == other.CompletedPacketCount && self.CompletedPacketQPC == other.CompletedPacketQPC && self.CompletedPacketHash == other.CompletedPacketHash
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_PACKETVREGISTER {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_PACKETVREGISTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut ::core::ffi::c_void,
}
impl KSRTAUDIO_PACKETVREGISTER_PROPERTY {}
impl ::core::default::Default for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_PACKETVREGISTER_PROPERTY {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSRTAUDIO_SETWRITEPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub EosPacketLength: u32,
}
impl KSRTAUDIO_SETWRITEPACKET_INFO {}
impl ::core::default::Default for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSRTAUDIO_SETWRITEPACKET_INFO").field("PacketNumber", &self.PacketNumber).field("Flags", &self.Flags).field("EosPacketLength", &self.EosPacketLength).finish()
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PacketNumber == other.PacketNumber && self.Flags == other.Flags && self.EosPacketLength == other.EosPacketLength
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_SETWRITEPACKET_INFO {}
unsafe impl ::windows::runtime::Abi for KSRTAUDIO_SETWRITEPACKET_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSSOUNDDETECTORPROPERTY {
    pub Property: KSIDENTIFIER,
    pub EventId: ::windows::runtime::GUID,
}
impl KSSOUNDDETECTORPROPERTY {}
impl ::core::default::Default for KSSOUNDDETECTORPROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSSOUNDDETECTORPROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSSOUNDDETECTORPROPERTY {}
unsafe impl ::windows::runtime::Abi for KSSOUNDDETECTORPROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSSTATE(pub i32);
pub const KSSTATE_STOP: KSSTATE = KSSTATE(0i32);
pub const KSSTATE_ACQUIRE: KSSTATE = KSSTATE(1i32);
pub const KSSTATE_PAUSE: KSSTATE = KSSTATE(2i32);
pub const KSSTATE_RUN: KSSTATE = KSSTATE(3i32);
impl ::core::convert::From<i32> for KSSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSSTATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSSTREAMALLOCATOR_STATUS {
    pub Framing: KSALLOCATOR_FRAMING,
    pub AllocatedFrames: u32,
    pub Reserved: u32,
}
impl KSSTREAMALLOCATOR_STATUS {}
impl ::core::default::Default for KSSTREAMALLOCATOR_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSSTREAMALLOCATOR_STATUS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSSTREAMALLOCATOR_STATUS {}
unsafe impl ::windows::runtime::Abi for KSSTREAMALLOCATOR_STATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSSTREAMALLOCATOR_STATUS_EX {
    pub Framing: KSALLOCATOR_FRAMING_EX,
    pub AllocatedFrames: u32,
    pub Reserved: u32,
}
impl KSSTREAMALLOCATOR_STATUS_EX {}
impl ::core::default::Default for KSSTREAMALLOCATOR_STATUS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSSTREAMALLOCATOR_STATUS_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSSTREAMALLOCATOR_STATUS_EX {}
unsafe impl ::windows::runtime::Abi for KSSTREAMALLOCATOR_STATUS_EX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_FAILUREEXCEPTION: u32 = 8192u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl KSSTREAM_HEADER {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for KSSTREAM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::fmt::Debug for KSSTREAM_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSSTREAM_HEADER")
            .field("Size", &self.Size)
            .field("TypeSpecificFlags", &self.TypeSpecificFlags)
            .field("PresentationTime", &self.PresentationTime)
            .field("Duration", &self.Duration)
            .field("FrameExtent", &self.FrameExtent)
            .field("DataUsed", &self.DataUsed)
            .field("Data", &self.Data)
            .field("OptionsFlags", &self.OptionsFlags)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for KSSTREAM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.TypeSpecificFlags == other.TypeSpecificFlags && self.PresentationTime == other.PresentationTime && self.Duration == other.Duration && self.FrameExtent == other.FrameExtent && self.DataUsed == other.DataUsed && self.Data == other.Data && self.OptionsFlags == other.OptionsFlags && self.Reserved == other.Reserved
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for KSSTREAM_HEADER {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::runtime::Abi for KSSTREAM_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
#[cfg(any(target_arch = "x86",))]
impl KSSTREAM_HEADER {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for KSSTREAM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::fmt::Debug for KSSTREAM_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSSTREAM_HEADER")
            .field("Size", &self.Size)
            .field("TypeSpecificFlags", &self.TypeSpecificFlags)
            .field("PresentationTime", &self.PresentationTime)
            .field("Duration", &self.Duration)
            .field("FrameExtent", &self.FrameExtent)
            .field("DataUsed", &self.DataUsed)
            .field("Data", &self.Data)
            .field("OptionsFlags", &self.OptionsFlags)
            .finish()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for KSSTREAM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.TypeSpecificFlags == other.TypeSpecificFlags && self.PresentationTime == other.PresentationTime && self.Duration == other.Duration && self.FrameExtent == other.FrameExtent && self.DataUsed == other.DataUsed && self.Data == other.Data && self.OptionsFlags == other.OptionsFlags
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for KSSTREAM_HEADER {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::runtime::Abi for KSSTREAM_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_BUFFEREDTRANSFER: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_DATADISCONTINUITY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_DURATIONVALID: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_ENDOFPHOTOSEQUENCE: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_ENDOFSTREAM: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_FLUSHONPAUSE: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_FRAMEINFO: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_LOOPEDDATA: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_METADATA: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_PERSIST_SAMPLE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_PREROLL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_SAMPLE_PERSISTED: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_SECUREBUFFERTRANSFER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_SPLICEPOINT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_TIMEDISCONTINUITY: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_TIMEVALID: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_TYPECHANGED: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_OPTIONSF_VRAM_DATA_TRANSFER: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_HEADER_TRACK_COMPLETION_NUMBERS: u32 = 131072u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSSTREAM_METADATA_INFO {
    pub BufferSize: u32,
    pub UsedSize: u32,
    pub Data: *mut ::core::ffi::c_void,
    pub SystemVa: *mut ::core::ffi::c_void,
    pub Flags: u32,
    pub Reserved: u32,
}
impl KSSTREAM_METADATA_INFO {}
impl ::core::default::Default for KSSTREAM_METADATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSSTREAM_METADATA_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSSTREAM_METADATA_INFO").field("BufferSize", &self.BufferSize).field("UsedSize", &self.UsedSize).field("Data", &self.Data).field("SystemVa", &self.SystemVa).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSSTREAM_METADATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.UsedSize == other.UsedSize && self.Data == other.Data && self.SystemVa == other.SystemVa && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSSTREAM_METADATA_INFO {}
unsafe impl ::windows::runtime::Abi for KSSTREAM_METADATA_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_NONPAGED_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_PAGED_DATA: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_READ: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_SYNCHRONOUS: u32 = 4096u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSSTREAM_UVC_METADATA {
    pub StartOfFrameTimestamp: KSSTREAM_UVC_METADATATYPE_TIMESTAMP,
    pub EndOfFrameTimestamp: KSSTREAM_UVC_METADATATYPE_TIMESTAMP,
}
impl KSSTREAM_UVC_METADATA {}
impl ::core::default::Default for KSSTREAM_UVC_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSSTREAM_UVC_METADATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSSTREAM_UVC_METADATA {}
unsafe impl ::windows::runtime::Abi for KSSTREAM_UVC_METADATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    pub PresentationTimeStamp: u32,
    pub SourceClockReference: u32,
    pub Anonymous: KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0,
    pub Reserved0: u16,
    pub Reserved1: u32,
}
impl KSSTREAM_UVC_METADATATYPE_TIMESTAMP {}
impl ::core::default::Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {}
unsafe impl ::windows::runtime::Abi for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    pub Anonymous: KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0,
    pub SCRToken: u16,
}
impl KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {}
impl ::core::default::Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {}
unsafe impl ::windows::runtime::Abi for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    pub _bitfield: u16,
}
impl KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {}
impl ::core::default::Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {}
unsafe impl ::windows::runtime::Abi for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_UVC_SECURE_ATTRIBUTE_SIZE: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_WRITE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSTELEPHONY_CALLCONTROL {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallControlOp: TELEPHONY_CALLCONTROLOP,
}
impl KSTELEPHONY_CALLCONTROL {}
impl ::core::default::Default for KSTELEPHONY_CALLCONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSTELEPHONY_CALLCONTROL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSTELEPHONY_CALLCONTROL").field("CallType", &self.CallType).field("CallControlOp", &self.CallControlOp).finish()
    }
}
impl ::core::cmp::PartialEq for KSTELEPHONY_CALLCONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.CallType == other.CallType && self.CallControlOp == other.CallControlOp
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_CALLCONTROL {}
unsafe impl ::windows::runtime::Abi for KSTELEPHONY_CALLCONTROL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSTELEPHONY_CALLINFO {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallState: TELEPHONY_CALLSTATE,
}
impl KSTELEPHONY_CALLINFO {}
impl ::core::default::Default for KSTELEPHONY_CALLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSTELEPHONY_CALLINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSTELEPHONY_CALLINFO").field("CallType", &self.CallType).field("CallState", &self.CallState).finish()
    }
}
impl ::core::cmp::PartialEq for KSTELEPHONY_CALLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.CallType == other.CallType && self.CallState == other.CallState
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_CALLINFO {}
unsafe impl ::windows::runtime::Abi for KSTELEPHONY_CALLINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSTELEPHONY_PROVIDERCHANGE {
    pub CallType: TELEPHONY_CALLTYPE,
    pub ProviderChangeOp: TELEPHONY_PROVIDERCHANGEOP,
}
impl KSTELEPHONY_PROVIDERCHANGE {}
impl ::core::default::Default for KSTELEPHONY_PROVIDERCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSTELEPHONY_PROVIDERCHANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSTELEPHONY_PROVIDERCHANGE").field("CallType", &self.CallType).field("ProviderChangeOp", &self.ProviderChangeOp).finish()
    }
}
impl ::core::cmp::PartialEq for KSTELEPHONY_PROVIDERCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.CallType == other.CallType && self.ProviderChangeOp == other.ProviderChangeOp
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_PROVIDERCHANGE {}
unsafe impl ::windows::runtime::Abi for KSTELEPHONY_PROVIDERCHANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSTIME {
    pub Time: i64,
    pub Numerator: u32,
    pub Denominator: u32,
}
impl KSTIME {}
impl ::core::default::Default for KSTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSTIME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSTIME").field("Time", &self.Time).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::cmp::PartialEq for KSTIME {
    fn eq(&self, other: &Self) -> bool {
        self.Time == other.Time && self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for KSTIME {}
unsafe impl ::windows::runtime::Abi for KSTIME {
    type Abi = Self;
}
pub const KSTIME_FORMAT_BYTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2071483761, 35970, 4559, [188, 12, 0, 170, 0, 172, 116, 246]);
pub const KSTIME_FORMAT_FIELD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2071483763, 35970, 4559, [188, 12, 0, 170, 0, 172, 116, 246]);
pub const KSTIME_FORMAT_FRAME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2071483760, 35970, 4559, [188, 12, 0, 170, 0, 172, 116, 246]);
pub const KSTIME_FORMAT_MEDIA_TIME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2071483764, 35970, 4559, [188, 12, 0, 170, 0, 172, 116, 246]);
pub const KSTIME_FORMAT_SAMPLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2071483762, 35970, 4559, [188, 12, 0, 170, 0, 172, 116, 246]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSTOPOLOGY {
    pub CategoriesCount: u32,
    pub Categories: *mut ::windows::runtime::GUID,
    pub TopologyNodesCount: u32,
    pub TopologyNodes: *mut ::windows::runtime::GUID,
    pub TopologyConnectionsCount: u32,
    pub TopologyConnections: *mut KSTOPOLOGY_CONNECTION,
    pub TopologyNodesNames: *mut ::windows::runtime::GUID,
    pub Reserved: u32,
}
impl KSTOPOLOGY {}
impl ::core::default::Default for KSTOPOLOGY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSTOPOLOGY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSTOPOLOGY")
            .field("CategoriesCount", &self.CategoriesCount)
            .field("Categories", &self.Categories)
            .field("TopologyNodesCount", &self.TopologyNodesCount)
            .field("TopologyNodes", &self.TopologyNodes)
            .field("TopologyConnectionsCount", &self.TopologyConnectionsCount)
            .field("TopologyConnections", &self.TopologyConnections)
            .field("TopologyNodesNames", &self.TopologyNodesNames)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSTOPOLOGY {
    fn eq(&self, other: &Self) -> bool {
        self.CategoriesCount == other.CategoriesCount && self.Categories == other.Categories && self.TopologyNodesCount == other.TopologyNodesCount && self.TopologyNodes == other.TopologyNodes && self.TopologyConnectionsCount == other.TopologyConnectionsCount && self.TopologyConnections == other.TopologyConnections && self.TopologyNodesNames == other.TopologyNodesNames && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY {}
unsafe impl ::windows::runtime::Abi for KSTOPOLOGY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSTOPOLOGY_CONNECTION {
    pub FromNode: u32,
    pub FromNodePin: u32,
    pub ToNode: u32,
    pub ToNodePin: u32,
}
impl KSTOPOLOGY_CONNECTION {}
impl ::core::default::Default for KSTOPOLOGY_CONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSTOPOLOGY_CONNECTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSTOPOLOGY_CONNECTION").field("FromNode", &self.FromNode).field("FromNodePin", &self.FromNodePin).field("ToNode", &self.ToNode).field("ToNodePin", &self.ToNodePin).finish()
    }
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.FromNode == other.FromNode && self.FromNodePin == other.FromNodePin && self.ToNode == other.ToNode && self.ToNodePin == other.ToNodePin
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_CONNECTION {}
unsafe impl ::windows::runtime::Abi for KSTOPOLOGY_CONNECTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSTOPOLOGY_ENDPOINTID {
    pub TopologyName: [u16; 260],
    pub PinId: u32,
}
impl KSTOPOLOGY_ENDPOINTID {}
impl ::core::default::Default for KSTOPOLOGY_ENDPOINTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSTOPOLOGY_ENDPOINTID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSTOPOLOGY_ENDPOINTID").field("TopologyName", &self.TopologyName).field("PinId", &self.PinId).finish()
    }
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_ENDPOINTID {
    fn eq(&self, other: &Self) -> bool {
        self.TopologyName == other.TopologyName && self.PinId == other.PinId
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_ENDPOINTID {}
unsafe impl ::windows::runtime::Abi for KSTOPOLOGY_ENDPOINTID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSTOPOLOGY_ENDPOINTIDPAIR {
    pub RenderEndpoint: KSTOPOLOGY_ENDPOINTID,
    pub CaptureEndpoint: KSTOPOLOGY_ENDPOINTID,
}
impl KSTOPOLOGY_ENDPOINTIDPAIR {}
impl ::core::default::Default for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSTOPOLOGY_ENDPOINTIDPAIR").field("RenderEndpoint", &self.RenderEndpoint).field("CaptureEndpoint", &self.CaptureEndpoint).finish()
    }
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.RenderEndpoint == other.RenderEndpoint && self.CaptureEndpoint == other.CaptureEndpoint
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_ENDPOINTIDPAIR {}
unsafe impl ::windows::runtime::Abi for KSTOPOLOGY_ENDPOINTIDPAIR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSVPMAXPIXELRATE {
    pub Size: KS_AMVPSIZE,
    pub MaxPixelsPerSecond: u32,
    pub Reserved: u32,
}
impl KSVPMAXPIXELRATE {}
impl ::core::default::Default for KSVPMAXPIXELRATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSVPMAXPIXELRATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSVPMAXPIXELRATE").field("Size", &self.Size).field("MaxPixelsPerSecond", &self.MaxPixelsPerSecond).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KSVPMAXPIXELRATE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.MaxPixelsPerSecond == other.MaxPixelsPerSecond && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSVPMAXPIXELRATE {}
unsafe impl ::windows::runtime::Abi for KSVPMAXPIXELRATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSVPSIZE_PROP {
    pub Property: KSIDENTIFIER,
    pub Size: KS_AMVPSIZE,
}
impl KSVPSIZE_PROP {}
impl ::core::default::Default for KSVPSIZE_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSVPSIZE_PROP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KSVPSIZE_PROP {}
unsafe impl ::windows::runtime::Abi for KSVPSIZE_PROP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSVPSURFACEPARAMS {
    pub dwPitch: u32,
    pub dwXOrigin: u32,
    pub dwYOrigin: u32,
}
impl KSVPSURFACEPARAMS {}
impl ::core::default::Default for KSVPSURFACEPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSVPSURFACEPARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSVPSURFACEPARAMS").field("dwPitch", &self.dwPitch).field("dwXOrigin", &self.dwXOrigin).field("dwYOrigin", &self.dwYOrigin).finish()
    }
}
impl ::core::cmp::PartialEq for KSVPSURFACEPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwPitch == other.dwPitch && self.dwXOrigin == other.dwXOrigin && self.dwYOrigin == other.dwYOrigin
    }
}
impl ::core::cmp::Eq for KSVPSURFACEPARAMS {}
unsafe impl ::windows::runtime::Abi for KSVPSURFACEPARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KSWAVETABLE_WAVE_DESC {
    pub Identifier: KSIDENTIFIER,
    pub Size: u32,
    pub Looped: super::super::Foundation::BOOL,
    pub LoopPoint: u32,
    pub InROM: super::super::Foundation::BOOL,
    pub Format: KSDATAFORMAT,
}
#[cfg(feature = "Win32_Foundation")]
impl KSWAVETABLE_WAVE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSWAVETABLE_WAVE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSWAVETABLE_WAVE_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSWAVETABLE_WAVE_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KSWAVETABLE_WAVE_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSWAVE_BUFFER {
    pub Attributes: u32,
    pub BufferSize: u32,
    pub BufferAddress: *mut ::core::ffi::c_void,
}
impl KSWAVE_BUFFER {}
impl ::core::default::Default for KSWAVE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSWAVE_BUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSWAVE_BUFFER").field("Attributes", &self.Attributes).field("BufferSize", &self.BufferSize).field("BufferAddress", &self.BufferAddress).finish()
    }
}
impl ::core::cmp::PartialEq for KSWAVE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.Attributes == other.Attributes && self.BufferSize == other.BufferSize && self.BufferAddress == other.BufferAddress
    }
}
impl ::core::cmp::Eq for KSWAVE_BUFFER {}
unsafe impl ::windows::runtime::Abi for KSWAVE_BUFFER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSWAVE_BUFFER_ATTRIBUTEF_LOOPING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSWAVE_BUFFER_ATTRIBUTEF_STATIC: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSWAVE_COMPATCAPS {
    pub ulDeviceType: u32,
}
impl KSWAVE_COMPATCAPS {}
impl ::core::default::Default for KSWAVE_COMPATCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSWAVE_COMPATCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSWAVE_COMPATCAPS").field("ulDeviceType", &self.ulDeviceType).finish()
    }
}
impl ::core::cmp::PartialEq for KSWAVE_COMPATCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.ulDeviceType == other.ulDeviceType
    }
}
impl ::core::cmp::Eq for KSWAVE_COMPATCAPS {}
unsafe impl ::windows::runtime::Abi for KSWAVE_COMPATCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSWAVE_COMPATCAPS_INPUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSWAVE_COMPATCAPS_OUTPUT: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSWAVE_INPUT_CAPABILITIES {
    pub MaximumChannelsPerConnection: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
    pub TotalConnections: u32,
    pub ActiveConnections: u32,
}
impl KSWAVE_INPUT_CAPABILITIES {}
impl ::core::default::Default for KSWAVE_INPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSWAVE_INPUT_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSWAVE_INPUT_CAPABILITIES")
            .field("MaximumChannelsPerConnection", &self.MaximumChannelsPerConnection)
            .field("MinimumBitsPerSample", &self.MinimumBitsPerSample)
            .field("MaximumBitsPerSample", &self.MaximumBitsPerSample)
            .field("MinimumSampleFrequency", &self.MinimumSampleFrequency)
            .field("MaximumSampleFrequency", &self.MaximumSampleFrequency)
            .field("TotalConnections", &self.TotalConnections)
            .field("ActiveConnections", &self.ActiveConnections)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSWAVE_INPUT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumChannelsPerConnection == other.MaximumChannelsPerConnection && self.MinimumBitsPerSample == other.MinimumBitsPerSample && self.MaximumBitsPerSample == other.MaximumBitsPerSample && self.MinimumSampleFrequency == other.MinimumSampleFrequency && self.MaximumSampleFrequency == other.MaximumSampleFrequency && self.TotalConnections == other.TotalConnections && self.ActiveConnections == other.ActiveConnections
    }
}
impl ::core::cmp::Eq for KSWAVE_INPUT_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for KSWAVE_INPUT_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KSWAVE_OUTPUT_CAPABILITIES {}
impl ::core::default::Default for KSWAVE_OUTPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSWAVE_OUTPUT_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSWAVE_OUTPUT_CAPABILITIES")
            .field("MaximumChannelsPerConnection", &self.MaximumChannelsPerConnection)
            .field("MinimumBitsPerSample", &self.MinimumBitsPerSample)
            .field("MaximumBitsPerSample", &self.MaximumBitsPerSample)
            .field("MinimumSampleFrequency", &self.MinimumSampleFrequency)
            .field("MaximumSampleFrequency", &self.MaximumSampleFrequency)
            .field("TotalConnections", &self.TotalConnections)
            .field("StaticConnections", &self.StaticConnections)
            .field("StreamingConnections", &self.StreamingConnections)
            .field("ActiveConnections", &self.ActiveConnections)
            .field("ActiveStaticConnections", &self.ActiveStaticConnections)
            .field("ActiveStreamingConnections", &self.ActiveStreamingConnections)
            .field("Total3DConnections", &self.Total3DConnections)
            .field("Static3DConnections", &self.Static3DConnections)
            .field("Streaming3DConnections", &self.Streaming3DConnections)
            .field("Active3DConnections", &self.Active3DConnections)
            .field("ActiveStatic3DConnections", &self.ActiveStatic3DConnections)
            .field("ActiveStreaming3DConnections", &self.ActiveStreaming3DConnections)
            .field("TotalSampleMemory", &self.TotalSampleMemory)
            .field("FreeSampleMemory", &self.FreeSampleMemory)
            .field("LargestFreeContiguousSampleMemory", &self.LargestFreeContiguousSampleMemory)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KSWAVE_OUTPUT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumChannelsPerConnection == other.MaximumChannelsPerConnection
            && self.MinimumBitsPerSample == other.MinimumBitsPerSample
            && self.MaximumBitsPerSample == other.MaximumBitsPerSample
            && self.MinimumSampleFrequency == other.MinimumSampleFrequency
            && self.MaximumSampleFrequency == other.MaximumSampleFrequency
            && self.TotalConnections == other.TotalConnections
            && self.StaticConnections == other.StaticConnections
            && self.StreamingConnections == other.StreamingConnections
            && self.ActiveConnections == other.ActiveConnections
            && self.ActiveStaticConnections == other.ActiveStaticConnections
            && self.ActiveStreamingConnections == other.ActiveStreamingConnections
            && self.Total3DConnections == other.Total3DConnections
            && self.Static3DConnections == other.Static3DConnections
            && self.Streaming3DConnections == other.Streaming3DConnections
            && self.Active3DConnections == other.Active3DConnections
            && self.ActiveStatic3DConnections == other.ActiveStatic3DConnections
            && self.ActiveStreaming3DConnections == other.ActiveStreaming3DConnections
            && self.TotalSampleMemory == other.TotalSampleMemory
            && self.FreeSampleMemory == other.FreeSampleMemory
            && self.LargestFreeContiguousSampleMemory == other.LargestFreeContiguousSampleMemory
    }
}
impl ::core::cmp::Eq for KSWAVE_OUTPUT_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for KSWAVE_OUTPUT_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KSWAVE_VOLUME {
    pub LeftAttenuation: i32,
    pub RightAttenuation: i32,
}
impl KSWAVE_VOLUME {}
impl ::core::default::Default for KSWAVE_VOLUME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KSWAVE_VOLUME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KSWAVE_VOLUME").field("LeftAttenuation", &self.LeftAttenuation).field("RightAttenuation", &self.RightAttenuation).finish()
    }
}
impl ::core::cmp::PartialEq for KSWAVE_VOLUME {
    fn eq(&self, other: &Self) -> bool {
        self.LeftAttenuation == other.LeftAttenuation && self.RightAttenuation == other.RightAttenuation
    }
}
impl ::core::cmp::Eq for KSWAVE_VOLUME {}
unsafe impl ::windows::runtime::Abi for KSWAVE_VOLUME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AMCONTROL_COLORINFO_PRESENT: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AMCONTROL_PAD_TO_16x9: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AMCONTROL_PAD_TO_4x3: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AMCONTROL_USED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_AMPixAspectRatio(pub i32);
pub const KS_PixAspectRatio_NTSC4x3: KS_AMPixAspectRatio = KS_AMPixAspectRatio(0i32);
pub const KS_PixAspectRatio_NTSC16x9: KS_AMPixAspectRatio = KS_AMPixAspectRatio(1i32);
pub const KS_PixAspectRatio_PAL4x3: KS_AMPixAspectRatio = KS_AMPixAspectRatio(2i32);
pub const KS_PixAspectRatio_PAL16x9: KS_AMPixAspectRatio = KS_AMPixAspectRatio(3i32);
impl ::core::convert::From<i32> for KS_AMPixAspectRatio {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_AMPixAspectRatio {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_AMVPDATAINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_AMVPDATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_AMVPDATAINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_AMVPDATAINFO")
            .field("dwSize", &self.dwSize)
            .field("dwMicrosecondsPerField", &self.dwMicrosecondsPerField)
            .field("amvpDimInfo", &self.amvpDimInfo)
            .field("dwPictAspectRatioX", &self.dwPictAspectRatioX)
            .field("dwPictAspectRatioY", &self.dwPictAspectRatioY)
            .field("bEnableDoubleClock", &self.bEnableDoubleClock)
            .field("bEnableVACT", &self.bEnableVACT)
            .field("bDataIsInterlaced", &self.bDataIsInterlaced)
            .field("lHalfLinesOdd", &self.lHalfLinesOdd)
            .field("bFieldPolarityInverted", &self.bFieldPolarityInverted)
            .field("dwNumLinesInVREF", &self.dwNumLinesInVREF)
            .field("lHalfLinesEven", &self.lHalfLinesEven)
            .field("dwReserved1", &self.dwReserved1)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_AMVPDATAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwMicrosecondsPerField == other.dwMicrosecondsPerField
            && self.amvpDimInfo == other.amvpDimInfo
            && self.dwPictAspectRatioX == other.dwPictAspectRatioX
            && self.dwPictAspectRatioY == other.dwPictAspectRatioY
            && self.bEnableDoubleClock == other.bEnableDoubleClock
            && self.bEnableVACT == other.bEnableVACT
            && self.bDataIsInterlaced == other.bDataIsInterlaced
            && self.lHalfLinesOdd == other.lHalfLinesOdd
            && self.bFieldPolarityInverted == other.bFieldPolarityInverted
            && self.dwNumLinesInVREF == other.dwNumLinesInVREF
            && self.lHalfLinesEven == other.lHalfLinesEven
            && self.dwReserved1 == other.dwReserved1
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_AMVPDATAINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_AMVPDATAINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_AMVPDIMINFO {
    pub dwFieldWidth: u32,
    pub dwFieldHeight: u32,
    pub dwVBIWidth: u32,
    pub dwVBIHeight: u32,
    pub rcValidRegion: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_AMVPDIMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_AMVPDIMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_AMVPDIMINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_AMVPDIMINFO").field("dwFieldWidth", &self.dwFieldWidth).field("dwFieldHeight", &self.dwFieldHeight).field("dwVBIWidth", &self.dwVBIWidth).field("dwVBIHeight", &self.dwVBIHeight).field("rcValidRegion", &self.rcValidRegion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_AMVPDIMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFieldWidth == other.dwFieldWidth && self.dwFieldHeight == other.dwFieldHeight && self.dwVBIWidth == other.dwVBIWidth && self.dwVBIHeight == other.dwVBIHeight && self.rcValidRegion == other.rcValidRegion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_AMVPDIMINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_AMVPDIMINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_AMVPSIZE {
    pub dwWidth: u32,
    pub dwHeight: u32,
}
impl KS_AMVPSIZE {}
impl ::core::default::Default for KS_AMVPSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_AMVPSIZE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_AMVPSIZE").field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).finish()
    }
}
impl ::core::cmp::PartialEq for KS_AMVPSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight
    }
}
impl ::core::cmp::Eq for KS_AMVPSIZE {}
unsafe impl ::windows::runtime::Abi for KS_AMVPSIZE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_AMVP_MODE(pub i32);
pub const KS_AMVP_MODE_WEAVE: KS_AMVP_MODE = KS_AMVP_MODE(0i32);
pub const KS_AMVP_MODE_BOBINTERLEAVED: KS_AMVP_MODE = KS_AMVP_MODE(1i32);
pub const KS_AMVP_MODE_BOBNONINTERLEAVED: KS_AMVP_MODE = KS_AMVP_MODE(2i32);
pub const KS_AMVP_MODE_SKIPEVEN: KS_AMVP_MODE = KS_AMVP_MODE(3i32);
pub const KS_AMVP_MODE_SKIPODD: KS_AMVP_MODE = KS_AMVP_MODE(4i32);
impl ::core::convert::From<i32> for KS_AMVP_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_AMVP_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_AMVP_SELECTFORMATBY(pub i32);
pub const KS_AMVP_DO_NOT_CARE: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(0i32);
pub const KS_AMVP_BEST_BANDWIDTH: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(1i32);
pub const KS_AMVP_INPUT_SAME_AS_OUTPUT: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(2i32);
impl ::core::convert::From<i32> for KS_AMVP_SELECTFORMATBY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_AMVP_SELECTFORMATBY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_AM_ExactRateChange {
    pub OutputZeroTime: i64,
    pub Rate: i32,
}
impl KS_AM_ExactRateChange {}
impl ::core::default::Default for KS_AM_ExactRateChange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_AM_ExactRateChange {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_AM_ExactRateChange").field("OutputZeroTime", &self.OutputZeroTime).field("Rate", &self.Rate).finish()
    }
}
impl ::core::cmp::PartialEq for KS_AM_ExactRateChange {
    fn eq(&self, other: &Self) -> bool {
        self.OutputZeroTime == other.OutputZeroTime && self.Rate == other.Rate
    }
}
impl ::core::cmp::Eq for KS_AM_ExactRateChange {}
unsafe impl ::windows::runtime::Abi for KS_AM_ExactRateChange {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_AM_PROPERTY_TS_RATE_CHANGE(pub i32);
pub const KS_AM_RATE_SimpleRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(1i32);
pub const KS_AM_RATE_ExactRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(2i32);
pub const KS_AM_RATE_MaxFullDataRate: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(3i32);
pub const KS_AM_RATE_Step: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(4i32);
impl ::core::convert::From<i32> for KS_AM_PROPERTY_TS_RATE_CHANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_AM_PROPERTY_TS_RATE_CHANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_AM_SimpleRateChange {
    pub StartTime: i64,
    pub Rate: i32,
}
impl KS_AM_SimpleRateChange {}
impl ::core::default::Default for KS_AM_SimpleRateChange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_AM_SimpleRateChange {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_AM_SimpleRateChange").field("StartTime", &self.StartTime).field("Rate", &self.Rate).finish()
    }
}
impl ::core::cmp::PartialEq for KS_AM_SimpleRateChange {
    fn eq(&self, other: &Self) -> bool {
        self.StartTime == other.StartTime && self.Rate == other.Rate
    }
}
impl ::core::cmp::Eq for KS_AM_SimpleRateChange {}
unsafe impl ::windows::runtime::Abi for KS_AM_SimpleRateChange {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AM_UseNewCSSKey: i32 = 1i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_AnalogVideoInfo {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_AnalogVideoInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_AnalogVideoInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_AnalogVideoInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_AnalogVideoInfo").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwActiveWidth", &self.dwActiveWidth).field("dwActiveHeight", &self.dwActiveHeight).field("AvgTimePerFrame", &self.AvgTimePerFrame).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_AnalogVideoInfo {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwActiveWidth == other.dwActiveWidth && self.dwActiveHeight == other.dwActiveHeight && self.AvgTimePerFrame == other.AvgTimePerFrame
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_AnalogVideoInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_AnalogVideoInfo {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_AnalogVideoStandard(pub i32);
pub const KS_AnalogVideo_None: KS_AnalogVideoStandard = KS_AnalogVideoStandard(0i32);
pub const KS_AnalogVideo_NTSC_M: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1i32);
pub const KS_AnalogVideo_NTSC_M_J: KS_AnalogVideoStandard = KS_AnalogVideoStandard(2i32);
pub const KS_AnalogVideo_NTSC_433: KS_AnalogVideoStandard = KS_AnalogVideoStandard(4i32);
pub const KS_AnalogVideo_PAL_B: KS_AnalogVideoStandard = KS_AnalogVideoStandard(16i32);
pub const KS_AnalogVideo_PAL_D: KS_AnalogVideoStandard = KS_AnalogVideoStandard(32i32);
pub const KS_AnalogVideo_PAL_G: KS_AnalogVideoStandard = KS_AnalogVideoStandard(64i32);
pub const KS_AnalogVideo_PAL_H: KS_AnalogVideoStandard = KS_AnalogVideoStandard(128i32);
pub const KS_AnalogVideo_PAL_I: KS_AnalogVideoStandard = KS_AnalogVideoStandard(256i32);
pub const KS_AnalogVideo_PAL_M: KS_AnalogVideoStandard = KS_AnalogVideoStandard(512i32);
pub const KS_AnalogVideo_PAL_N: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1024i32);
pub const KS_AnalogVideo_PAL_60: KS_AnalogVideoStandard = KS_AnalogVideoStandard(2048i32);
pub const KS_AnalogVideo_SECAM_B: KS_AnalogVideoStandard = KS_AnalogVideoStandard(4096i32);
pub const KS_AnalogVideo_SECAM_D: KS_AnalogVideoStandard = KS_AnalogVideoStandard(8192i32);
pub const KS_AnalogVideo_SECAM_G: KS_AnalogVideoStandard = KS_AnalogVideoStandard(16384i32);
pub const KS_AnalogVideo_SECAM_H: KS_AnalogVideoStandard = KS_AnalogVideoStandard(32768i32);
pub const KS_AnalogVideo_SECAM_K: KS_AnalogVideoStandard = KS_AnalogVideoStandard(65536i32);
pub const KS_AnalogVideo_SECAM_K1: KS_AnalogVideoStandard = KS_AnalogVideoStandard(131072i32);
pub const KS_AnalogVideo_SECAM_L: KS_AnalogVideoStandard = KS_AnalogVideoStandard(262144i32);
pub const KS_AnalogVideo_SECAM_L1: KS_AnalogVideoStandard = KS_AnalogVideoStandard(524288i32);
pub const KS_AnalogVideo_PAL_N_COMBO: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1048576i32);
impl ::core::convert::From<i32> for KS_AnalogVideoStandard {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_AnalogVideoStandard {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AnalogVideo_NTSC_Mask: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AnalogVideo_PAL_Mask: u32 = 1052656u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AnalogVideo_SECAM_Mask: u32 = 1044480u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KS_BITMAPINFOHEADER {}
impl ::core::default::Default for KS_BITMAPINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_BITMAPINFOHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_BITMAPINFOHEADER")
            .field("biSize", &self.biSize)
            .field("biWidth", &self.biWidth)
            .field("biHeight", &self.biHeight)
            .field("biPlanes", &self.biPlanes)
            .field("biBitCount", &self.biBitCount)
            .field("biCompression", &self.biCompression)
            .field("biSizeImage", &self.biSizeImage)
            .field("biXPelsPerMeter", &self.biXPelsPerMeter)
            .field("biYPelsPerMeter", &self.biYPelsPerMeter)
            .field("biClrUsed", &self.biClrUsed)
            .field("biClrImportant", &self.biClrImportant)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KS_BITMAPINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.biSize == other.biSize && self.biWidth == other.biWidth && self.biHeight == other.biHeight && self.biPlanes == other.biPlanes && self.biBitCount == other.biBitCount && self.biCompression == other.biCompression && self.biSizeImage == other.biSizeImage && self.biXPelsPerMeter == other.biXPelsPerMeter && self.biYPelsPerMeter == other.biYPelsPerMeter && self.biClrUsed == other.biClrUsed && self.biClrImportant == other.biClrImportant
    }
}
impl ::core::cmp::Eq for KS_BITMAPINFOHEADER {}
unsafe impl ::windows::runtime::Abi for KS_BITMAPINFOHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_BI_BITFIELDS: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_BI_JPEG: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_BI_RGB: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_BI_RLE4: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_BI_RLE8: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_EVEN: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_FIELD1_MASK: i32 = 240i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_FIELD2_MASK: i32 = 7936i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_ODD: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_CC1: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_CC2: i32 = 32i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_CC3: i32 = 256i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_CC4: i32 = 512i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_T1: i32 = 64i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_T2: i32 = 128i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_T3: i32 = 1024i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_T4: i32 = 2048i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_CC_SUBSTREAM_SERVICE_XDS: i32 = 4096i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_COLCON {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
}
impl KS_COLCON {}
impl ::core::default::Default for KS_COLCON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_COLCON {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_COLCON").field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).field("_bitfield3", &self._bitfield3).field("_bitfield4", &self._bitfield4).finish()
    }
}
impl ::core::cmp::PartialEq for KS_COLCON {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2 && self._bitfield3 == other._bitfield3 && self._bitfield4 == other._bitfield4
    }
}
impl ::core::cmp::Eq for KS_COLCON {}
unsafe impl ::windows::runtime::Abi for KS_COLCON {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_COMPRESSION {
    pub RatioNumerator: u32,
    pub RatioDenominator: u32,
    pub RatioConstantMargin: u32,
}
impl KS_COMPRESSION {}
impl ::core::default::Default for KS_COMPRESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_COMPRESSION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_COMPRESSION").field("RatioNumerator", &self.RatioNumerator).field("RatioDenominator", &self.RatioDenominator).field("RatioConstantMargin", &self.RatioConstantMargin).finish()
    }
}
impl ::core::cmp::PartialEq for KS_COMPRESSION {
    fn eq(&self, other: &Self) -> bool {
        self.RatioNumerator == other.RatioNumerator && self.RatioDenominator == other.RatioDenominator && self.RatioConstantMargin == other.RatioConstantMargin
    }
}
impl ::core::cmp::Eq for KS_COMPRESSION {}
unsafe impl ::windows::runtime::Abi for KS_COMPRESSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_COPYPROTECT_RestrictDuplication: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_COPY_MACROVISION {
    pub MACROVISIONLevel: u32,
}
impl KS_COPY_MACROVISION {}
impl ::core::default::Default for KS_COPY_MACROVISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_COPY_MACROVISION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_COPY_MACROVISION").field("MACROVISIONLevel", &self.MACROVISIONLevel).finish()
    }
}
impl ::core::cmp::PartialEq for KS_COPY_MACROVISION {
    fn eq(&self, other: &Self) -> bool {
        self.MACROVISIONLevel == other.MACROVISIONLevel
    }
}
impl ::core::cmp::Eq for KS_COPY_MACROVISION {}
unsafe impl ::windows::runtime::Abi for KS_COPY_MACROVISION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_COPY_MACROVISION_LEVEL(pub i32);
pub const KS_MACROVISION_DISABLED: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(0i32);
pub const KS_MACROVISION_LEVEL1: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(1i32);
pub const KS_MACROVISION_LEVEL2: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(2i32);
pub const KS_MACROVISION_LEVEL3: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(3i32);
impl ::core::convert::From<i32> for KS_COPY_MACROVISION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_COPY_MACROVISION_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_CameraControlAsyncOperation(pub i32);
pub const KS_CAMERACONTROL_ASYNC_START: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(1i32);
pub const KS_CAMERACONTROL_ASYNC_STOP: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(2i32);
pub const KS_CAMERACONTROL_ASYNC_RESET: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(3i32);
impl ::core::convert::From<i32> for KS_CameraControlAsyncOperation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_CameraControlAsyncOperation {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_CompressionCaps(pub i32);
pub const KS_CompressionCaps_CanQuality: KS_CompressionCaps = KS_CompressionCaps(1i32);
pub const KS_CompressionCaps_CanCrunch: KS_CompressionCaps = KS_CompressionCaps(2i32);
pub const KS_CompressionCaps_CanKeyFrame: KS_CompressionCaps = KS_CompressionCaps(4i32);
pub const KS_CompressionCaps_CanBFrame: KS_CompressionCaps = KS_CompressionCaps(8i32);
pub const KS_CompressionCaps_CanWindow: KS_CompressionCaps = KS_CompressionCaps(16i32);
impl ::core::convert::From<i32> for KS_CompressionCaps {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_CompressionCaps {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DATAFORMAT_H264VIDEOINFO {
    pub DataFormat: KSDATAFORMAT,
    pub H264VideoInfoHeader: KS_H264VIDEOINFO,
}
impl KS_DATAFORMAT_H264VIDEOINFO {}
impl ::core::default::Default for KS_DATAFORMAT_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DATAFORMAT_H264VIDEOINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KS_DATAFORMAT_H264VIDEOINFO {}
unsafe impl ::windows::runtime::Abi for KS_DATAFORMAT_H264VIDEOINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DATAFORMAT_IMAGEINFO {
    pub DataFormat: KSDATAFORMAT,
    pub ImageInfoHeader: KS_BITMAPINFOHEADER,
}
impl KS_DATAFORMAT_IMAGEINFO {}
impl ::core::default::Default for KS_DATAFORMAT_IMAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DATAFORMAT_IMAGEINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KS_DATAFORMAT_IMAGEINFO {}
unsafe impl ::windows::runtime::Abi for KS_DATAFORMAT_IMAGEINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_DATAFORMAT_MPEGVIDEOINFO2 {
    pub DataFormat: KSDATAFORMAT,
    pub MpegVideoInfoHeader2: KS_MPEGVIDEOINFO2,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_DATAFORMAT_MPEGVIDEOINFO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATAFORMAT_MPEGVIDEOINFO2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DATAFORMAT_VBIINFOHEADER {
    pub DataFormat: KSDATAFORMAT,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
impl KS_DATAFORMAT_VBIINFOHEADER {}
impl ::core::default::Default for KS_DATAFORMAT_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DATAFORMAT_VBIINFOHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KS_DATAFORMAT_VBIINFOHEADER {}
unsafe impl ::windows::runtime::Abi for KS_DATAFORMAT_VBIINFOHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_DATAFORMAT_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATAFORMAT_VIDEOINFOHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATAFORMAT_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATAFORMAT_VIDEOINFOHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER2 {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfoHeader2: KS_VIDEOINFOHEADER2,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_DATAFORMAT_VIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATAFORMAT_VIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_DATAFORMAT_VIDEOINFO_PALETTE {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfo: KS_VIDEOINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_DATAFORMAT_VIDEOINFO_PALETTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATAFORMAT_VIDEOINFO_PALETTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_DATARANGE_ANALOGVIDEO {
    pub DataRange: KSDATAFORMAT,
    pub AnalogVideoInfo: KS_AnalogVideoInfo,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_DATARANGE_ANALOGVIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_ANALOGVIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_ANALOGVIDEO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_ANALOGVIDEO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_ANALOGVIDEO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_DATARANGE_H264_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_H264_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_H264_VIDEO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_H264_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_H264_VIDEO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_DATARANGE_IMAGE {
    pub DataRange: KSDATAFORMAT,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub ImageInfoHeader: KS_BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_DATARANGE_IMAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_IMAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_IMAGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_IMAGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_DATARANGE_MPEG1_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_MPEG1_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_MPEG1_VIDEO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_MPEG1_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_MPEG1_VIDEO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_DATARANGE_MPEG2_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_MPEG2_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_MPEG2_VIDEO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_MPEG2_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_MPEG2_VIDEO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_DATARANGE_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_VIDEO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_VIDEO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_DATARANGE_VIDEO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_VIDEO2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_VIDEO2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_VIDEO2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_DATARANGE_VIDEO_PALETTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO_PALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_VIDEO_PALETTE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_VIDEO_PALETTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_VIDEO_PALETTE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_DATARANGE_VIDEO_VBI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO_VBI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_VIDEO_VBI {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_VIDEO_VBI {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_DATARANGE_VIDEO_VBI {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_DVDCOPYSTATE(pub i32);
pub const KS_DVDCOPYSTATE_INITIALIZE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(0i32);
pub const KS_DVDCOPYSTATE_INITIALIZE_TITLE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(1i32);
pub const KS_DVDCOPYSTATE_AUTHENTICATION_NOT_REQUIRED: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(2i32);
pub const KS_DVDCOPYSTATE_AUTHENTICATION_REQUIRED: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(3i32);
pub const KS_DVDCOPYSTATE_DONE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(4i32);
impl ::core::convert::From<i32> for KS_DVDCOPYSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_DVDCOPYSTATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DVDCOPY_BUSKEY {
    pub BusKey: [u8; 5],
    pub Reserved: [u8; 1],
}
impl KS_DVDCOPY_BUSKEY {}
impl ::core::default::Default for KS_DVDCOPY_BUSKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_DVDCOPY_BUSKEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_DVDCOPY_BUSKEY").field("BusKey", &self.BusKey).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_BUSKEY {
    fn eq(&self, other: &Self) -> bool {
        self.BusKey == other.BusKey && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_BUSKEY {}
unsafe impl ::windows::runtime::Abi for KS_DVDCOPY_BUSKEY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DVDCOPY_CHLGKEY {
    pub ChlgKey: [u8; 10],
    pub Reserved: [u8; 2],
}
impl KS_DVDCOPY_CHLGKEY {}
impl ::core::default::Default for KS_DVDCOPY_CHLGKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_DVDCOPY_CHLGKEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_DVDCOPY_CHLGKEY").field("ChlgKey", &self.ChlgKey).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_CHLGKEY {
    fn eq(&self, other: &Self) -> bool {
        self.ChlgKey == other.ChlgKey && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_CHLGKEY {}
unsafe impl ::windows::runtime::Abi for KS_DVDCOPY_CHLGKEY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DVDCOPY_DISCKEY {
    pub DiscKey: [u8; 2048],
}
impl KS_DVDCOPY_DISCKEY {}
impl ::core::default::Default for KS_DVDCOPY_DISCKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_DVDCOPY_DISCKEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_DVDCOPY_DISCKEY").field("DiscKey", &self.DiscKey).finish()
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_DISCKEY {
    fn eq(&self, other: &Self) -> bool {
        self.DiscKey == other.DiscKey
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_DISCKEY {}
unsafe impl ::windows::runtime::Abi for KS_DVDCOPY_DISCKEY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DVDCOPY_REGION {
    pub Reserved: u8,
    pub RegionData: u8,
    pub Reserved2: [u8; 2],
}
impl KS_DVDCOPY_REGION {}
impl ::core::default::Default for KS_DVDCOPY_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_DVDCOPY_REGION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_DVDCOPY_REGION").field("Reserved", &self.Reserved).field("RegionData", &self.RegionData).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.RegionData == other.RegionData && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_REGION {}
unsafe impl ::windows::runtime::Abi for KS_DVDCOPY_REGION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DVDCOPY_SET_COPY_STATE {
    pub DVDCopyState: u32,
}
impl KS_DVDCOPY_SET_COPY_STATE {}
impl ::core::default::Default for KS_DVDCOPY_SET_COPY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_DVDCOPY_SET_COPY_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_DVDCOPY_SET_COPY_STATE").field("DVDCopyState", &self.DVDCopyState).finish()
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_SET_COPY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.DVDCopyState == other.DVDCopyState
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_SET_COPY_STATE {}
unsafe impl ::windows::runtime::Abi for KS_DVDCOPY_SET_COPY_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DVDCOPY_TITLEKEY {
    pub KeyFlags: u32,
    pub ReservedNT: [u32; 2],
    pub TitleKey: [u8; 6],
    pub Reserved: [u8; 2],
}
impl KS_DVDCOPY_TITLEKEY {}
impl ::core::default::Default for KS_DVDCOPY_TITLEKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_DVDCOPY_TITLEKEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_DVDCOPY_TITLEKEY").field("KeyFlags", &self.KeyFlags).field("ReservedNT", &self.ReservedNT).field("TitleKey", &self.TitleKey).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_TITLEKEY {
    fn eq(&self, other: &Self) -> bool {
        self.KeyFlags == other.KeyFlags && self.ReservedNT == other.ReservedNT && self.TitleKey == other.TitleKey && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_TITLEKEY {}
unsafe impl ::windows::runtime::Abi for KS_DVDCOPY_TITLEKEY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_CGMS_COPY_ONCE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_CGMS_COPY_PERMITTED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_CGMS_COPY_PROTECT_MASK: u32 = 24u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_CGMS_NO_COPY: u32 = 24u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_CGMS_RESERVED_MASK: u32 = 120u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_COPYRIGHTED: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_COPYRIGHT_MASK: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_NOT_COPYRIGHTED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_SECTOR_NOT_PROTECTED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_SECTOR_PROTECTED: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_DVD_SECTOR_PROTECT_MASK: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DVD_YCrCb {
    pub Reserved: u8,
    pub Y: u8,
    pub Cr: u8,
    pub Cb: u8,
}
impl KS_DVD_YCrCb {}
impl ::core::default::Default for KS_DVD_YCrCb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_DVD_YCrCb {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_DVD_YCrCb").field("Reserved", &self.Reserved).field("Y", &self.Y).field("Cr", &self.Cr).field("Cb", &self.Cb).finish()
    }
}
impl ::core::cmp::PartialEq for KS_DVD_YCrCb {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.Y == other.Y && self.Cr == other.Cr && self.Cb == other.Cb
    }
}
impl ::core::cmp::Eq for KS_DVD_YCrCb {}
unsafe impl ::windows::runtime::Abi for KS_DVD_YCrCb {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_DVD_YUV {
    pub Reserved: u8,
    pub Y: u8,
    pub V: u8,
    pub U: u8,
}
impl KS_DVD_YUV {}
impl ::core::default::Default for KS_DVD_YUV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_DVD_YUV {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_DVD_YUV").field("Reserved", &self.Reserved).field("Y", &self.Y).field("V", &self.V).field("U", &self.U).finish()
    }
}
impl ::core::cmp::PartialEq for KS_DVD_YUV {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.Y == other.Y && self.V == other.V && self.U == other.U
    }
}
impl ::core::cmp::Eq for KS_DVD_YUV {}
unsafe impl ::windows::runtime::Abi for KS_DVD_YUV {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_FRAME_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_FRAME_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union KS_FRAME_INFO_0 {
    pub lSurfacePitch: i32,
    pub Reserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_FRAME_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_FRAME_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_FRAME_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_FRAME_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union KS_FRAME_INFO_1 {
    pub Anonymous: KS_FRAME_INFO_1_0,
    pub FrameCompletionNumber: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_FRAME_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_FRAME_INFO_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_FRAME_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_FRAME_INFO_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KS_FRAME_INFO_1_0 {
    pub Reserved3: u32,
    pub Reserved4: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_FRAME_INFO_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_FRAME_INFO_1_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_FRAME_INFO_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved3 == other.Reserved3 && self.Reserved4 == other.Reserved4
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_FRAME_INFO_1_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_FRAME_INFO_1_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_FRAMING_ITEM {
    pub MemoryType: ::windows::runtime::GUID,
    pub BusType: ::windows::runtime::GUID,
    pub MemoryFlags: u32,
    pub BusFlags: u32,
    pub Flags: u32,
    pub Frames: u32,
    pub Anonymous: KS_FRAMING_ITEM_0,
    pub MemoryTypeWeight: u32,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub FramingRange: KS_FRAMING_RANGE_WEIGHTED,
}
impl KS_FRAMING_ITEM {}
impl ::core::default::Default for KS_FRAMING_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_FRAMING_ITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KS_FRAMING_ITEM {}
unsafe impl ::windows::runtime::Abi for KS_FRAMING_ITEM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union KS_FRAMING_ITEM_0 {
    pub FileAlignment: u32,
    pub FramePitch: i32,
}
impl KS_FRAMING_ITEM_0 {}
impl ::core::default::Default for KS_FRAMING_ITEM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_FRAMING_ITEM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for KS_FRAMING_ITEM_0 {}
unsafe impl ::windows::runtime::Abi for KS_FRAMING_ITEM_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_FRAMING_RANGE {
    pub MinFrameSize: u32,
    pub MaxFrameSize: u32,
    pub Stepping: u32,
}
impl KS_FRAMING_RANGE {}
impl ::core::default::Default for KS_FRAMING_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_FRAMING_RANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_FRAMING_RANGE").field("MinFrameSize", &self.MinFrameSize).field("MaxFrameSize", &self.MaxFrameSize).field("Stepping", &self.Stepping).finish()
    }
}
impl ::core::cmp::PartialEq for KS_FRAMING_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.MinFrameSize == other.MinFrameSize && self.MaxFrameSize == other.MaxFrameSize && self.Stepping == other.Stepping
    }
}
impl ::core::cmp::Eq for KS_FRAMING_RANGE {}
unsafe impl ::windows::runtime::Abi for KS_FRAMING_RANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_FRAMING_RANGE_WEIGHTED {
    pub Range: KS_FRAMING_RANGE,
    pub InPlaceWeight: u32,
    pub NotInPlaceWeight: u32,
}
impl KS_FRAMING_RANGE_WEIGHTED {}
impl ::core::default::Default for KS_FRAMING_RANGE_WEIGHTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_FRAMING_RANGE_WEIGHTED {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_FRAMING_RANGE_WEIGHTED").field("Range", &self.Range).field("InPlaceWeight", &self.InPlaceWeight).field("NotInPlaceWeight", &self.NotInPlaceWeight).finish()
    }
}
impl ::core::cmp::PartialEq for KS_FRAMING_RANGE_WEIGHTED {
    fn eq(&self, other: &Self) -> bool {
        self.Range == other.Range && self.InPlaceWeight == other.InPlaceWeight && self.NotInPlaceWeight == other.NotInPlaceWeight
    }
}
impl ::core::cmp::Eq for KS_FRAMING_RANGE_WEIGHTED {}
unsafe impl ::windows::runtime::Abi for KS_FRAMING_RANGE_WEIGHTED {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KS_H264VIDEOINFO {}
impl ::core::default::Default for KS_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_H264VIDEOINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_H264VIDEOINFO")
            .field("wWidth", &self.wWidth)
            .field("wHeight", &self.wHeight)
            .field("wSARwidth", &self.wSARwidth)
            .field("wSARheight", &self.wSARheight)
            .field("wProfile", &self.wProfile)
            .field("bLevelIDC", &self.bLevelIDC)
            .field("wConstrainedToolset", &self.wConstrainedToolset)
            .field("bmSupportedUsages", &self.bmSupportedUsages)
            .field("bmCapabilities", &self.bmCapabilities)
            .field("bmSVCCapabilities", &self.bmSVCCapabilities)
            .field("bmMVCCapabilities", &self.bmMVCCapabilities)
            .field("dwFrameInterval", &self.dwFrameInterval)
            .field("bMaxCodecConfigDelay", &self.bMaxCodecConfigDelay)
            .field("bmSupportedSliceModes", &self.bmSupportedSliceModes)
            .field("bmSupportedSyncFrameTypes", &self.bmSupportedSyncFrameTypes)
            .field("bResolutionScaling", &self.bResolutionScaling)
            .field("bSimulcastSupport", &self.bSimulcastSupport)
            .field("bmSupportedRateControlModes", &self.bmSupportedRateControlModes)
            .field("wMaxMBperSecOneResolutionNoScalability", &self.wMaxMBperSecOneResolutionNoScalability)
            .field("wMaxMBperSecTwoResolutionsNoScalability", &self.wMaxMBperSecTwoResolutionsNoScalability)
            .field("wMaxMBperSecThreeResolutionsNoScalability", &self.wMaxMBperSecThreeResolutionsNoScalability)
            .field("wMaxMBperSecFourResolutionsNoScalability", &self.wMaxMBperSecFourResolutionsNoScalability)
            .field("wMaxMBperSecOneResolutionTemporalScalability", &self.wMaxMBperSecOneResolutionTemporalScalability)
            .field("wMaxMBperSecTwoResolutionsTemporalScalablility", &self.wMaxMBperSecTwoResolutionsTemporalScalablility)
            .field("wMaxMBperSecThreeResolutionsTemporalScalability", &self.wMaxMBperSecThreeResolutionsTemporalScalability)
            .field("wMaxMBperSecFourResolutionsTemporalScalability", &self.wMaxMBperSecFourResolutionsTemporalScalability)
            .field("wMaxMBperSecOneResolutionTemporalQualityScalability", &self.wMaxMBperSecOneResolutionTemporalQualityScalability)
            .field("wMaxMBperSecTwoResolutionsTemporalQualityScalability", &self.wMaxMBperSecTwoResolutionsTemporalQualityScalability)
            .field("wMaxMBperSecThreeResolutionsTemporalQualityScalablity", &self.wMaxMBperSecThreeResolutionsTemporalQualityScalablity)
            .field("wMaxMBperSecFourResolutionsTemporalQualityScalability", &self.wMaxMBperSecFourResolutionsTemporalQualityScalability)
            .field("wMaxMBperSecOneResolutionTemporalSpatialScalability", &self.wMaxMBperSecOneResolutionTemporalSpatialScalability)
            .field("wMaxMBperSecTwoResolutionsTemporalSpatialScalability", &self.wMaxMBperSecTwoResolutionsTemporalSpatialScalability)
            .field("wMaxMBperSecThreeResolutionsTemporalSpatialScalablity", &self.wMaxMBperSecThreeResolutionsTemporalSpatialScalablity)
            .field("wMaxMBperSecFourResolutionsTemporalSpatialScalability", &self.wMaxMBperSecFourResolutionsTemporalSpatialScalability)
            .field("wMaxMBperSecOneResolutionFullScalability", &self.wMaxMBperSecOneResolutionFullScalability)
            .field("wMaxMBperSecTwoResolutionsFullScalability", &self.wMaxMBperSecTwoResolutionsFullScalability)
            .field("wMaxMBperSecThreeResolutionsFullScalability", &self.wMaxMBperSecThreeResolutionsFullScalability)
            .field("wMaxMBperSecFourResolutionsFullScalability", &self.wMaxMBperSecFourResolutionsFullScalability)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KS_H264VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wWidth == other.wWidth
            && self.wHeight == other.wHeight
            && self.wSARwidth == other.wSARwidth
            && self.wSARheight == other.wSARheight
            && self.wProfile == other.wProfile
            && self.bLevelIDC == other.bLevelIDC
            && self.wConstrainedToolset == other.wConstrainedToolset
            && self.bmSupportedUsages == other.bmSupportedUsages
            && self.bmCapabilities == other.bmCapabilities
            && self.bmSVCCapabilities == other.bmSVCCapabilities
            && self.bmMVCCapabilities == other.bmMVCCapabilities
            && self.dwFrameInterval == other.dwFrameInterval
            && self.bMaxCodecConfigDelay == other.bMaxCodecConfigDelay
            && self.bmSupportedSliceModes == other.bmSupportedSliceModes
            && self.bmSupportedSyncFrameTypes == other.bmSupportedSyncFrameTypes
            && self.bResolutionScaling == other.bResolutionScaling
            && self.bSimulcastSupport == other.bSimulcastSupport
            && self.bmSupportedRateControlModes == other.bmSupportedRateControlModes
            && self.wMaxMBperSecOneResolutionNoScalability == other.wMaxMBperSecOneResolutionNoScalability
            && self.wMaxMBperSecTwoResolutionsNoScalability == other.wMaxMBperSecTwoResolutionsNoScalability
            && self.wMaxMBperSecThreeResolutionsNoScalability == other.wMaxMBperSecThreeResolutionsNoScalability
            && self.wMaxMBperSecFourResolutionsNoScalability == other.wMaxMBperSecFourResolutionsNoScalability
            && self.wMaxMBperSecOneResolutionTemporalScalability == other.wMaxMBperSecOneResolutionTemporalScalability
            && self.wMaxMBperSecTwoResolutionsTemporalScalablility == other.wMaxMBperSecTwoResolutionsTemporalScalablility
            && self.wMaxMBperSecThreeResolutionsTemporalScalability == other.wMaxMBperSecThreeResolutionsTemporalScalability
            && self.wMaxMBperSecFourResolutionsTemporalScalability == other.wMaxMBperSecFourResolutionsTemporalScalability
            && self.wMaxMBperSecOneResolutionTemporalQualityScalability == other.wMaxMBperSecOneResolutionTemporalQualityScalability
            && self.wMaxMBperSecTwoResolutionsTemporalQualityScalability == other.wMaxMBperSecTwoResolutionsTemporalQualityScalability
            && self.wMaxMBperSecThreeResolutionsTemporalQualityScalablity == other.wMaxMBperSecThreeResolutionsTemporalQualityScalablity
            && self.wMaxMBperSecFourResolutionsTemporalQualityScalability == other.wMaxMBperSecFourResolutionsTemporalQualityScalability
            && self.wMaxMBperSecOneResolutionTemporalSpatialScalability == other.wMaxMBperSecOneResolutionTemporalSpatialScalability
            && self.wMaxMBperSecTwoResolutionsTemporalSpatialScalability == other.wMaxMBperSecTwoResolutionsTemporalSpatialScalability
            && self.wMaxMBperSecThreeResolutionsTemporalSpatialScalablity == other.wMaxMBperSecThreeResolutionsTemporalSpatialScalablity
            && self.wMaxMBperSecFourResolutionsTemporalSpatialScalability == other.wMaxMBperSecFourResolutionsTemporalSpatialScalability
            && self.wMaxMBperSecOneResolutionFullScalability == other.wMaxMBperSecOneResolutionFullScalability
            && self.wMaxMBperSecTwoResolutionsFullScalability == other.wMaxMBperSecTwoResolutionsFullScalability
            && self.wMaxMBperSecThreeResolutionsFullScalability == other.wMaxMBperSecThreeResolutionsFullScalability
            && self.wMaxMBperSecFourResolutionsFullScalability == other.wMaxMBperSecFourResolutionsFullScalability
    }
}
impl ::core::cmp::Eq for KS_H264VIDEOINFO {}
unsafe impl ::windows::runtime::Abi for KS_H264VIDEOINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_1FieldPerSample: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_DisplayModeBobOnly: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_DisplayModeBobOrWeave: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_DisplayModeMask: u32 = 192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_DisplayModeWeaveOnly: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_Field1First: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_FieldPatBothIrregular: u32 = 48u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_FieldPatBothRegular: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_FieldPatField1Only: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_FieldPatField2Only: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_FieldPatternMask: u32 = 48u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_IsInterlaced: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_INTERLACE_UNUSED: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_LogicalMemoryType(pub i32);
pub const KS_MemoryTypeDontCare: KS_LogicalMemoryType = KS_LogicalMemoryType(0i32);
pub const KS_MemoryTypeKernelPaged: KS_LogicalMemoryType = KS_LogicalMemoryType(1i32);
pub const KS_MemoryTypeKernelNonPaged: KS_LogicalMemoryType = KS_LogicalMemoryType(2i32);
pub const KS_MemoryTypeDeviceHostMapped: KS_LogicalMemoryType = KS_LogicalMemoryType(3i32);
pub const KS_MemoryTypeDeviceSpecific: KS_LogicalMemoryType = KS_LogicalMemoryType(4i32);
pub const KS_MemoryTypeUser: KS_LogicalMemoryType = KS_LogicalMemoryType(5i32);
pub const KS_MemoryTypeAnyHost: KS_LogicalMemoryType = KS_LogicalMemoryType(6i32);
impl ::core::convert::From<i32> for KS_LogicalMemoryType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_LogicalMemoryType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_MPEAUDIOINFO {
    pub dwFlags: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
}
impl KS_MPEAUDIOINFO {}
impl ::core::default::Default for KS_MPEAUDIOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_MPEAUDIOINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_MPEAUDIOINFO").field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).finish()
    }
}
impl ::core::cmp::PartialEq for KS_MPEAUDIOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
impl ::core::cmp::Eq for KS_MPEAUDIOINFO {}
unsafe impl ::windows::runtime::Abi for KS_MPEAUDIOINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_MPEG1VIDEOINFO {
    pub hdr: KS_VIDEOINFOHEADER,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub bSequenceHeader: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl KS_MPEG1VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_MPEG1VIDEOINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_MPEG1VIDEOINFO").field("hdr", &self.hdr).field("dwStartTimeCode", &self.dwStartTimeCode).field("cbSequenceHeader", &self.cbSequenceHeader).field("bSequenceHeader", &self.bSequenceHeader).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_MPEG1VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwStartTimeCode == other.dwStartTimeCode && self.cbSequenceHeader == other.cbSequenceHeader && self.bSequenceHeader == other.bSequenceHeader
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_MPEG1VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_MPEG1VIDEOINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_MPEG2Level(pub i32);
pub const KS_MPEG2Level_Low: KS_MPEG2Level = KS_MPEG2Level(0i32);
pub const KS_MPEG2Level_Main: KS_MPEG2Level = KS_MPEG2Level(1i32);
pub const KS_MPEG2Level_High1440: KS_MPEG2Level = KS_MPEG2Level(2i32);
pub const KS_MPEG2Level_High: KS_MPEG2Level = KS_MPEG2Level(3i32);
impl ::core::convert::From<i32> for KS_MPEG2Level {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_MPEG2Level {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_MPEG2Profile(pub i32);
pub const KS_MPEG2Profile_Simple: KS_MPEG2Profile = KS_MPEG2Profile(0i32);
pub const KS_MPEG2Profile_Main: KS_MPEG2Profile = KS_MPEG2Profile(1i32);
pub const KS_MPEG2Profile_SNRScalable: KS_MPEG2Profile = KS_MPEG2Profile(2i32);
pub const KS_MPEG2Profile_SpatiallyScalable: KS_MPEG2Profile = KS_MPEG2Profile(3i32);
pub const KS_MPEG2Profile_High: KS_MPEG2Profile = KS_MPEG2Profile(4i32);
impl ::core::convert::From<i32> for KS_MPEG2Profile {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_MPEG2Profile {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_27MhzTimebase: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_DSS_UserData: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_DVB_UserData: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_DVDLine21Field1: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_DVDLine21Field2: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_DoPanScan: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_FilmCameraMode: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_LetterboxAnalogOut: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_SourceIsLetterboxed: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEG2_WidescreenAnalogOut: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MPEGAUDIOINFO_27MhzTimebase: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_MPEGVIDEOINFO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_MPEGVIDEOINFO2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_MPEGVIDEOINFO2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_MPEGVIDEOINFO2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_ADVERTISER_BASE: u32 = 2224u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_CONTENT_BASE: u32 = 2208u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_MICROSOFT_RESERVED_TEST_DATA_BASE: u32 = 2288u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_NETWORK_WIDE_ADVERTISER_BASE: u32 = 2160u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_NETWORK_WIDE_CONTENT_BASE: u32 = 2144u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_ADVERTISER_BASE: u32 = 2064u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_BASE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_ADVERTISER_BASE: u32 = 2096u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_CONTENT_BASE: u32 = 2080u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_ADVERTISER_BASE: u32 = 2128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_CONTENT_BASE: u32 = 2112u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_TELEVISION_STATION_ADVERTISER_BASE: u32 = 2192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_NABTS_GROUPID_TELEVISION_STATION_CONTENT_BASE: u32 = 2176u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_PhysicalConnectorType(pub i32);
pub const KS_PhysConn_Video_Tuner: KS_PhysicalConnectorType = KS_PhysicalConnectorType(1i32);
pub const KS_PhysConn_Video_Composite: KS_PhysicalConnectorType = KS_PhysicalConnectorType(2i32);
pub const KS_PhysConn_Video_SVideo: KS_PhysicalConnectorType = KS_PhysicalConnectorType(3i32);
pub const KS_PhysConn_Video_RGB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4i32);
pub const KS_PhysConn_Video_YRYBY: KS_PhysicalConnectorType = KS_PhysicalConnectorType(5i32);
pub const KS_PhysConn_Video_SerialDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(6i32);
pub const KS_PhysConn_Video_ParallelDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(7i32);
pub const KS_PhysConn_Video_SCSI: KS_PhysicalConnectorType = KS_PhysicalConnectorType(8i32);
pub const KS_PhysConn_Video_AUX: KS_PhysicalConnectorType = KS_PhysicalConnectorType(9i32);
pub const KS_PhysConn_Video_1394: KS_PhysicalConnectorType = KS_PhysicalConnectorType(10i32);
pub const KS_PhysConn_Video_USB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(11i32);
pub const KS_PhysConn_Video_VideoDecoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(12i32);
pub const KS_PhysConn_Video_VideoEncoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(13i32);
pub const KS_PhysConn_Video_SCART: KS_PhysicalConnectorType = KS_PhysicalConnectorType(14i32);
pub const KS_PhysConn_Audio_Tuner: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4096i32);
pub const KS_PhysConn_Audio_Line: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4097i32);
pub const KS_PhysConn_Audio_Mic: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4098i32);
pub const KS_PhysConn_Audio_AESDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4099i32);
pub const KS_PhysConn_Audio_SPDIFDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4100i32);
pub const KS_PhysConn_Audio_SCSI: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4101i32);
pub const KS_PhysConn_Audio_AUX: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4102i32);
pub const KS_PhysConn_Audio_1394: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4103i32);
pub const KS_PhysConn_Audio_USB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4104i32);
pub const KS_PhysConn_Audio_AudioDecoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4105i32);
impl ::core::convert::From<i32> for KS_PhysicalConnectorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_PhysicalConnectorType {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
impl KS_RGBQUAD {}
impl ::core::default::Default for KS_RGBQUAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_RGBQUAD {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_RGBQUAD").field("rgbBlue", &self.rgbBlue).field("rgbGreen", &self.rgbGreen).field("rgbRed", &self.rgbRed).field("rgbReserved", &self.rgbReserved).finish()
    }
}
impl ::core::cmp::PartialEq for KS_RGBQUAD {
    fn eq(&self, other: &Self) -> bool {
        self.rgbBlue == other.rgbBlue && self.rgbGreen == other.rgbGreen && self.rgbRed == other.rgbRed && self.rgbReserved == other.rgbReserved
    }
}
impl ::core::cmp::Eq for KS_RGBQUAD {}
unsafe impl ::windows::runtime::Abi for KS_RGBQUAD {
    type Abi = Self;
}
pub const KS_SECURE_CAMERA_SCENARIO_ID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2924739694, 36233, 17544, [157, 46, 77, 0, 135, 49, 197, 253]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_SEEKING_CAPABILITIES(pub i32);
pub const KS_SEEKING_CanSeekAbsolute: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(1i32);
pub const KS_SEEKING_CanSeekForwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(2i32);
pub const KS_SEEKING_CanSeekBackwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(4i32);
pub const KS_SEEKING_CanGetCurrentPos: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(8i32);
pub const KS_SEEKING_CanGetStopPos: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(16i32);
pub const KS_SEEKING_CanGetDuration: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(32i32);
pub const KS_SEEKING_CanPlayBackwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(64i32);
impl ::core::convert::From<i32> for KS_SEEKING_CAPABILITIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_SEEKING_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_SEEKING_FLAGS(pub i32);
pub const KS_SEEKING_NoPositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(0i32);
pub const KS_SEEKING_AbsolutePositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(1i32);
pub const KS_SEEKING_RelativePositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(2i32);
pub const KS_SEEKING_IncrementalPositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(3i32);
pub const KS_SEEKING_PositioningBitsMask: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(3i32);
pub const KS_SEEKING_SeekToKeyFrame: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(4i32);
pub const KS_SEEKING_ReturnTime: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(8i32);
impl ::core::convert::From<i32> for KS_SEEKING_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_SEEKING_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_TRUECOLORINFO {
    pub dwBitMasks: [u32; 3],
    pub bmiColors: [KS_RGBQUAD; 256],
}
impl KS_TRUECOLORINFO {}
impl ::core::default::Default for KS_TRUECOLORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_TRUECOLORINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_TRUECOLORINFO").field("dwBitMasks", &self.dwBitMasks).field("bmiColors", &self.bmiColors).finish()
    }
}
impl ::core::cmp::PartialEq for KS_TRUECOLORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwBitMasks == other.dwBitMasks && self.bmiColors == other.bmiColors
    }
}
impl ::core::cmp::Eq for KS_TRUECOLORINFO {}
unsafe impl ::windows::runtime::Abi for KS_TRUECOLORINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_TUNER_STRATEGY(pub i32);
pub const KS_TUNER_STRATEGY_PLL: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(1i32);
pub const KS_TUNER_STRATEGY_SIGNAL_STRENGTH: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(2i32);
pub const KS_TUNER_STRATEGY_DRIVER_TUNES: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(4i32);
impl ::core::convert::From<i32> for KS_TUNER_STRATEGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_TUNER_STRATEGY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_TUNER_TUNING_FLAGS(pub i32);
pub const KS_TUNER_TUNING_EXACT: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(1i32);
pub const KS_TUNER_TUNING_FINE: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(2i32);
pub const KS_TUNER_TUNING_COARSE: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(3i32);
impl ::core::convert::From<i32> for KS_TUNER_TUNING_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_TUNER_TUNING_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_MODE_LANG_A: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_MODE_LANG_B: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_MODE_LANG_C: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_MODE_MONO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_MODE_STEREO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_PRESET_LANG_A: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_PRESET_LANG_B: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_PRESET_LANG_C: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVAUDIO_PRESET_STEREO: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_TVTUNER_CHANGE_END_TUNE: i32 = 2i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_TVTUNER_CHANGE_INFO {
    pub dwFlags: u32,
    pub dwCountryCode: u32,
    pub dwAnalogVideoStandard: u32,
    pub dwChannel: u32,
}
impl KS_TVTUNER_CHANGE_INFO {}
impl ::core::default::Default for KS_TVTUNER_CHANGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_TVTUNER_CHANGE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_TVTUNER_CHANGE_INFO").field("dwFlags", &self.dwFlags).field("dwCountryCode", &self.dwCountryCode).field("dwAnalogVideoStandard", &self.dwAnalogVideoStandard).field("dwChannel", &self.dwChannel).finish()
    }
}
impl ::core::cmp::PartialEq for KS_TVTUNER_CHANGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwCountryCode == other.dwCountryCode && self.dwAnalogVideoStandard == other.dwAnalogVideoStandard && self.dwChannel == other.dwChannel
    }
}
impl ::core::cmp::Eq for KS_TVTUNER_CHANGE_INFO {}
unsafe impl ::windows::runtime::Abi for KS_TVTUNER_CHANGE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBICAP_PROTECTION_MV_DETECTED: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBICAP_PROTECTION_MV_HARDWARE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBICAP_PROTECTION_MV_PRESENT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBIDATARATE_CC: i32 = 503493i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBIDATARATE_NABTS: i32 = 5727272i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl KS_VBIINFOHEADER {}
impl ::core::default::Default for KS_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_VBIINFOHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_VBIINFOHEADER")
            .field("StartLine", &self.StartLine)
            .field("EndLine", &self.EndLine)
            .field("SamplingFrequency", &self.SamplingFrequency)
            .field("MinLineStartTime", &self.MinLineStartTime)
            .field("MaxLineStartTime", &self.MaxLineStartTime)
            .field("ActualLineStartTime", &self.ActualLineStartTime)
            .field("ActualLineEndTime", &self.ActualLineEndTime)
            .field("VideoStandard", &self.VideoStandard)
            .field("SamplesPerLine", &self.SamplesPerLine)
            .field("StrideInBytes", &self.StrideInBytes)
            .field("BufferSize", &self.BufferSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KS_VBIINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.StartLine == other.StartLine
            && self.EndLine == other.EndLine
            && self.SamplingFrequency == other.SamplingFrequency
            && self.MinLineStartTime == other.MinLineStartTime
            && self.MaxLineStartTime == other.MaxLineStartTime
            && self.ActualLineStartTime == other.ActualLineStartTime
            && self.ActualLineEndTime == other.ActualLineEndTime
            && self.VideoStandard == other.VideoStandard
            && self.SamplesPerLine == other.SamplesPerLine
            && self.StrideInBytes == other.StrideInBytes
            && self.BufferSize == other.BufferSize
    }
}
impl ::core::cmp::Eq for KS_VBIINFOHEADER {}
unsafe impl ::windows::runtime::Abi for KS_VBIINFOHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBI_FLAG_FIELD1: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBI_FLAG_FIELD2: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBI_FLAG_FRAME: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBI_FLAG_MV_DETECTED: i32 = 1024i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBI_FLAG_MV_HARDWARE: i32 = 512i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBI_FLAG_MV_PRESENT: i32 = 256i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBI_FLAG_TVTUNER_CHANGE: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VBI_FLAG_VBIINFOHEADER_CHANGE: i32 = 32i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct KS_VBI_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub dwSamplingFrequency: u32,
    pub TvTunerChangeInfo: KS_TVTUNER_CHANGE_INFO,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
impl KS_VBI_FRAME_INFO {}
impl ::core::default::Default for KS_VBI_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for KS_VBI_FRAME_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_VBI_FRAME_INFO")
            .field("ExtendedHeaderSize", &self.ExtendedHeaderSize)
            .field("dwFrameFlags", &self.dwFrameFlags)
            .field("PictureNumber", &self.PictureNumber)
            .field("DropCount", &self.DropCount)
            .field("dwSamplingFrequency", &self.dwSamplingFrequency)
            .field("TvTunerChangeInfo", &self.TvTunerChangeInfo)
            .field("VBIInfoHeader", &self.VBIInfoHeader)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KS_VBI_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExtendedHeaderSize == other.ExtendedHeaderSize && self.dwFrameFlags == other.dwFrameFlags && self.PictureNumber == other.PictureNumber && self.DropCount == other.DropCount && self.dwSamplingFrequency == other.dwSamplingFrequency && self.TvTunerChangeInfo == other.TvTunerChangeInfo && self.VBIInfoHeader == other.VBIInfoHeader
    }
}
impl ::core::cmp::Eq for KS_VBI_FRAME_INFO {}
unsafe impl ::windows::runtime::Abi for KS_VBI_FRAME_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_VIDEODECODER_FLAGS(pub i32);
pub const KS_VIDEODECODER_FLAGS_CAN_DISABLE_OUTPUT: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(1i32);
pub const KS_VIDEODECODER_FLAGS_CAN_USE_VCR_LOCKING: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(2i32);
pub const KS_VIDEODECODER_FLAGS_CAN_INDICATE_LOCKED: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(4i32);
impl ::core::convert::From<i32> for KS_VIDEODECODER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_VIDEODECODER_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_VIDEOINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union KS_VIDEOINFO_0 {
    pub bmiColors: [KS_RGBQUAD; 256],
    pub dwBitMasks: [u32; 3],
    pub TrueColorInfo: KS_TRUECOLORINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_VIDEOINFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_VIDEOINFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_VIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: KS_BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_VIDEOINFOHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_VIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_VIDEOINFOHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
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
impl KS_VIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFOHEADER2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_VIDEOINFOHEADER2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union KS_VIDEOINFOHEADER2_0 {
    pub dwControlFlags: u32,
    pub dwReserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KS_VIDEOINFOHEADER2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFOHEADER2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFOHEADER2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFOHEADER2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_VIDEOINFOHEADER2_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_CAPTURE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_CC: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_EDS: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_IS_VPE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_NABTS: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_PREVIEW: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_STILL: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_TELETEXT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEOSTREAM_VBI: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_ALLOC_VPE_AGP: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_ALLOC_VPE_DISPLAY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_ALLOC_VPE_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_B_FRAME: i32 = 32i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_FIELD1: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_FIELD1FIRST: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_FIELD2: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_FIELD_MASK: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_FRAME: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_IPB_MASK: i32 = 48i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_I_FRAME: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_P_FRAME: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_REPEAT_FIELD: i32 = 64i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_VIDEO_FLAG_WEAVE: i32 = 8i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct KS_VIDEO_STREAM_CONFIG_CAPS {
    pub guid: ::windows::runtime::GUID,
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
impl KS_VIDEO_STREAM_CONFIG_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("KS_VIDEO_STREAM_CONFIG_CAPS")
            .field("guid", &self.guid)
            .field("VideoStandard", &self.VideoStandard)
            .field("InputSize", &self.InputSize)
            .field("MinCroppingSize", &self.MinCroppingSize)
            .field("MaxCroppingSize", &self.MaxCroppingSize)
            .field("CropGranularityX", &self.CropGranularityX)
            .field("CropGranularityY", &self.CropGranularityY)
            .field("CropAlignX", &self.CropAlignX)
            .field("CropAlignY", &self.CropAlignY)
            .field("MinOutputSize", &self.MinOutputSize)
            .field("MaxOutputSize", &self.MaxOutputSize)
            .field("OutputGranularityX", &self.OutputGranularityX)
            .field("OutputGranularityY", &self.OutputGranularityY)
            .field("StretchTapsX", &self.StretchTapsX)
            .field("StretchTapsY", &self.StretchTapsY)
            .field("ShrinkTapsX", &self.ShrinkTapsX)
            .field("ShrinkTapsY", &self.ShrinkTapsY)
            .field("MinFrameInterval", &self.MinFrameInterval)
            .field("MaxFrameInterval", &self.MaxFrameInterval)
            .field("MinBitsPerSecond", &self.MinBitsPerSecond)
            .field("MaxBitsPerSecond", &self.MaxBitsPerSecond)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
            && self.VideoStandard == other.VideoStandard
            && self.InputSize == other.InputSize
            && self.MinCroppingSize == other.MinCroppingSize
            && self.MaxCroppingSize == other.MaxCroppingSize
            && self.CropGranularityX == other.CropGranularityX
            && self.CropGranularityY == other.CropGranularityY
            && self.CropAlignX == other.CropAlignX
            && self.CropAlignY == other.CropAlignY
            && self.MinOutputSize == other.MinOutputSize
            && self.MaxOutputSize == other.MaxOutputSize
            && self.OutputGranularityX == other.OutputGranularityX
            && self.OutputGranularityY == other.OutputGranularityY
            && self.StretchTapsX == other.StretchTapsX
            && self.StretchTapsY == other.StretchTapsY
            && self.ShrinkTapsX == other.ShrinkTapsX
            && self.ShrinkTapsY == other.ShrinkTapsY
            && self.MinFrameInterval == other.MinFrameInterval
            && self.MaxFrameInterval == other.MaxFrameInterval
            && self.MinBitsPerSecond == other.MinBitsPerSecond
            && self.MaxBitsPerSecond == other.MaxBitsPerSecond
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEO_STREAM_CONFIG_CAPS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KS_VIDEO_STREAM_CONFIG_CAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_VideoControlFlags(pub i32);
pub const KS_VideoControlFlag_FlipHorizontal: KS_VideoControlFlags = KS_VideoControlFlags(1i32);
pub const KS_VideoControlFlag_FlipVertical: KS_VideoControlFlags = KS_VideoControlFlags(2i32);
pub const KS_Obsolete_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = KS_VideoControlFlags(16i32);
pub const KS_Obsolete_VideoControlFlag_Trigger: KS_VideoControlFlags = KS_VideoControlFlags(32i32);
pub const KS_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = KS_VideoControlFlags(4i32);
pub const KS_VideoControlFlag_Trigger: KS_VideoControlFlags = KS_VideoControlFlags(8i32);
pub const KS_VideoControlFlag_IndependentImagePin: KS_VideoControlFlags = KS_VideoControlFlags(64i32);
pub const KS_VideoControlFlag_StillCapturePreviewFrame: KS_VideoControlFlags = KS_VideoControlFlags(128i32);
pub const KS_VideoControlFlag_StartPhotoSequenceCapture: KS_VideoControlFlags = KS_VideoControlFlags(256i32);
pub const KS_VideoControlFlag_StopPhotoSequenceCapture: KS_VideoControlFlags = KS_VideoControlFlags(512i32);
impl ::core::convert::From<i32> for KS_VideoControlFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_VideoControlFlags {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KS_VideoStreamingHints(pub i32);
pub const KS_StreamingHint_FrameInterval: KS_VideoStreamingHints = KS_VideoStreamingHints(256i32);
pub const KS_StreamingHint_KeyFrameRate: KS_VideoStreamingHints = KS_VideoStreamingHints(512i32);
pub const KS_StreamingHint_PFrameRate: KS_VideoStreamingHints = KS_VideoStreamingHints(1024i32);
pub const KS_StreamingHint_CompQuality: KS_VideoStreamingHints = KS_VideoStreamingHints(2048i32);
pub const KS_StreamingHint_CompWindowSize: KS_VideoStreamingHints = KS_VideoStreamingHints(4096i32);
impl ::core::convert::From<i32> for KS_VideoStreamingHints {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KS_VideoStreamingHints {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iBLUE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iEGA_COLORS: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iGREEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iMASK_COLORS: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iMAXBITS: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iPALETTE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iPALETTE_COLORS: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iRED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_iTRUECOLOR: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateAllocator<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(connectionhandle: Param0, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KsCreateAllocator(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(KsCreateAllocator(connectionhandle.into_param().abi(), ::core::mem::transmute(allocatorframing), ::core::mem::transmute(allocatorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateAllocator2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(connectionhandle: Param0, allocatorframing: *const KSALLOCATOR_FRAMING) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KsCreateAllocator2(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        KsCreateAllocator2(connectionhandle.into_param().abi(), ::core::mem::transmute(allocatorframing), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(connectionhandle: Param0, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KsCreateClock(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(KsCreateClock(connectionhandle.into_param().abi(), ::core::mem::transmute(clockcreate), ::core::mem::transmute(clockhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateClock2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(connectionhandle: Param0, clockcreate: *const KSCLOCK_CREATE) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KsCreateClock2(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        KsCreateClock2(connectionhandle.into_param().abi(), ::core::mem::transmute(clockcreate), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreatePin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filterhandle: Param0, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KsCreatePin(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(KsCreatePin(filterhandle.into_param().abi(), ::core::mem::transmute(connect), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(connectionhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreatePin2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filterhandle: Param0, connect: *const KSPIN_CONNECT, desiredaccess: u32) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KsCreatePin2(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        KsCreatePin2(filterhandle.into_param().abi(), ::core::mem::transmute(connect), ::core::mem::transmute(desiredaccess), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateTopologyNode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(parenthandle: Param0, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KsCreateTopologyNode(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(KsCreateTopologyNode(parenthandle.into_param().abi(), ::core::mem::transmute(nodecreate), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(nodehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateTopologyNode2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(parenthandle: Param0, nodecreate: *const KSNODE_CREATE, desiredaccess: u32) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KsCreateTopologyNode2(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        KsCreateTopologyNode2(parenthandle.into_param().abi(), ::core::mem::transmute(nodecreate), ::core::mem::transmute(desiredaccess), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct LOOPEDSTREAMING_POSITION_EVENT_DATA {
    pub KsEventData: KSEVENTDATA,
    pub Position: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl LOOPEDSTREAMING_POSITION_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOOPEDSTREAMING_POSITION_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MAX_NABTS_VBI_LINES_PER_FIELD: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MAX_RESOURCEGROUPID_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MAX_SINK_DESCRIPTION_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MAX_WST_VBI_LINES_PER_FIELD: u32 = 17u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub struct MEDIUM_INFO {
    pub MediaPresent: super::super::Foundation::BOOL,
    pub MediaType: u32,
    pub RecordInhibit: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl MEDIUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MEDIUM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MEDIUM_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MEDIUM_INFO").field("MediaPresent", &self.MediaPresent).field("MediaType", &self.MediaType).field("RecordInhibit", &self.RecordInhibit).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MEDIUM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MediaPresent == other.MediaPresent && self.MediaType == other.MediaType && self.RecordInhibit == other.RecordInhibit
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MEDIUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MEDIUM_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub union MF_MDL_SHARED_PAYLOAD_KEY {
    pub combined: MF_MDL_SHARED_PAYLOAD_KEY_0,
    pub GMDLHandle: ::windows::runtime::GUID,
}
impl MF_MDL_SHARED_PAYLOAD_KEY {}
impl ::core::default::Default for MF_MDL_SHARED_PAYLOAD_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_MDL_SHARED_PAYLOAD_KEY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MF_MDL_SHARED_PAYLOAD_KEY {}
unsafe impl ::windows::runtime::Abi for MF_MDL_SHARED_PAYLOAD_KEY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct MF_MDL_SHARED_PAYLOAD_KEY_0 {
    pub pHandle: u32,
    pub fHandle: u32,
    pub uPayload: u64,
}
impl MF_MDL_SHARED_PAYLOAD_KEY_0 {}
impl ::core::default::Default for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_combined_e__Struct").field("pHandle", &self.pHandle).field("fHandle", &self.fHandle).field("uPayload", &self.uPayload).finish()
    }
}
impl ::core::cmp::PartialEq for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pHandle == other.pHandle && self.fHandle == other.fHandle && self.uPayload == other.uPayload
    }
}
impl ::core::cmp::Eq for MF_MDL_SHARED_PAYLOAD_KEY_0 {}
unsafe impl ::windows::runtime::Abi for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MIN_DEV_VER_FOR_FLAGS: u32 = 272u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MIN_DEV_VER_FOR_QI: u32 = 256u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct NABTSFEC_BUFFER {
    pub dataSize: u32,
    pub groupID: u16,
    pub Reserved: u16,
    pub data: [u8; 448],
}
impl NABTSFEC_BUFFER {}
impl ::core::default::Default for NABTSFEC_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for NABTSFEC_BUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("NABTSFEC_BUFFER").field("dataSize", &self.dataSize).field("groupID", &self.groupID).field("Reserved", &self.Reserved).field("data", &self.data).finish()
    }
}
impl ::core::cmp::PartialEq for NABTSFEC_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dataSize == other.dataSize && self.groupID == other.groupID && self.Reserved == other.Reserved && self.data == other.data
    }
}
impl ::core::cmp::Eq for NABTSFEC_BUFFER {}
unsafe impl ::windows::runtime::Abi for NABTSFEC_BUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct NABTS_BUFFER {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub PictureNumber: i64,
    pub NabtsLines: [NABTS_BUFFER_LINE; 11],
}
impl NABTS_BUFFER {}
impl ::core::default::Default for NABTS_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NABTS_BUFFER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for NABTS_BUFFER {}
unsafe impl ::windows::runtime::Abi for NABTS_BUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct NABTS_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 36],
}
impl NABTS_BUFFER_LINE {}
impl ::core::default::Default for NABTS_BUFFER_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for NABTS_BUFFER_LINE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("NABTS_BUFFER_LINE").field("Confidence", &self.Confidence).field("Bytes", &self.Bytes).finish()
    }
}
impl ::core::cmp::PartialEq for NABTS_BUFFER_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Confidence == other.Confidence && self.Bytes == other.Bytes
    }
}
impl ::core::cmp::Eq for NABTS_BUFFER_LINE {}
unsafe impl ::windows::runtime::Abi for NABTS_BUFFER_LINE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const NABTS_BUFFER_PICTURENUMBER_SUPPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const NABTS_BYTES_PER_LINE: u32 = 36u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const NABTS_LINES_PER_BUNDLE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const NABTS_PAYLOAD_PER_LINE: u32 = 28u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const NANOSECONDS: u32 = 10000000u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct OPTIMAL_WEIGHT_TOTALS {
    pub MinTotalNominator: i64,
    pub MaxTotalNominator: i64,
    pub TotalDenominator: i64,
}
impl OPTIMAL_WEIGHT_TOTALS {}
impl ::core::default::Default for OPTIMAL_WEIGHT_TOTALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OPTIMAL_WEIGHT_TOTALS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OPTIMAL_WEIGHT_TOTALS").field("MinTotalNominator", &self.MinTotalNominator).field("MaxTotalNominator", &self.MaxTotalNominator).field("TotalDenominator", &self.TotalDenominator).finish()
    }
}
impl ::core::cmp::PartialEq for OPTIMAL_WEIGHT_TOTALS {
    fn eq(&self, other: &Self) -> bool {
        self.MinTotalNominator == other.MinTotalNominator && self.MaxTotalNominator == other.MaxTotalNominator && self.TotalDenominator == other.TotalDenominator
    }
}
impl ::core::cmp::Eq for OPTIMAL_WEIGHT_TOTALS {}
unsafe impl ::windows::runtime::Abi for OPTIMAL_WEIGHT_TOTALS {
    type Abi = Self;
}
pub const PINNAME_DISPLAYPORT_OUT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(570143529, 6730, 18650, [160, 118, 35, 24, 163, 197, 155, 38]);
pub const PINNAME_HDMI_OUT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(947649539, 59375, 18689, [134, 224, 53, 183, 195, 43, 0, 239]);
pub const PINNAME_IMAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(950062488, 54427, 19688, [180, 138, 52, 70, 103, 161, 120, 48]);
pub const PINNAME_SPDIF_IN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(366776357, 8877, 16819, [136, 117, 244, 206, 176, 41, 158, 32]);
pub const PINNAME_SPDIF_OUT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(975586433, 58668, 19330, [142, 122, 200, 226, 249, 29, 195, 128]);
pub const PINNAME_VIDEO_ANALOGVIDEOIN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176131, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176129, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_CC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176137, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_CC_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(447578209, 301, 4562, [180, 177, 0, 160, 209, 2, 207, 190]);
pub const PINNAME_VIDEO_EDS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176135, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_NABTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176134, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_NABTS_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(695219808, 18826, 4562, [180, 177, 0, 160, 209, 2, 207, 190]);
pub const PINNAME_VIDEO_PREVIEW: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176130, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_STILL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176138, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_TELETEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176136, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_TIMECODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176139, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_VBI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176132, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_VIDEOPORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176133, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
pub const PINNAME_VIDEO_VIDEOPORT_VBI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218176140, 851, 4561, [144, 95, 0, 0, 192, 204, 22, 186]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PIPE_ALLOCATOR_PLACE(pub i32);
pub const Pipe_Allocator_None: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(0i32);
pub const Pipe_Allocator_FirstPin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(1i32);
pub const Pipe_Allocator_LastPin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(2i32);
pub const Pipe_Allocator_MiddlePin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(3i32);
impl ::core::convert::From<i32> for PIPE_ALLOCATOR_PLACE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PIPE_ALLOCATOR_PLACE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct PIPE_DIMENSIONS {
    pub AllocatorPin: KS_COMPRESSION,
    pub MaxExpansionPin: KS_COMPRESSION,
    pub EndPin: KS_COMPRESSION,
}
impl PIPE_DIMENSIONS {}
impl ::core::default::Default for PIPE_DIMENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PIPE_DIMENSIONS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PIPE_DIMENSIONS").field("AllocatorPin", &self.AllocatorPin).field("MaxExpansionPin", &self.MaxExpansionPin).field("EndPin", &self.EndPin).finish()
    }
}
impl ::core::cmp::PartialEq for PIPE_DIMENSIONS {
    fn eq(&self, other: &Self) -> bool {
        self.AllocatorPin == other.AllocatorPin && self.MaxExpansionPin == other.MaxExpansionPin && self.EndPin == other.EndPin
    }
}
impl ::core::cmp::Eq for PIPE_DIMENSIONS {}
unsafe impl ::windows::runtime::Abi for PIPE_DIMENSIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PIPE_STATE(pub i32);
pub const PipeState_DontCare: PIPE_STATE = PIPE_STATE(0i32);
pub const PipeState_RangeNotFixed: PIPE_STATE = PIPE_STATE(1i32);
pub const PipeState_RangeFixed: PIPE_STATE = PIPE_STATE(2i32);
pub const PipeState_CompressionUnknown: PIPE_STATE = PIPE_STATE(3i32);
pub const PipeState_Finalized: PIPE_STATE = PIPE_STATE(4i32);
impl ::core::convert::From<i32> for PIPE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PIPE_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct PIPE_TERMINATION {
    pub Flags: u32,
    pub OutsideFactors: u32,
    pub Weigth: u32,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub OptimalRange: KS_FRAMING_RANGE_WEIGHTED,
    pub Compression: KS_COMPRESSION,
}
impl PIPE_TERMINATION {}
impl ::core::default::Default for PIPE_TERMINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PIPE_TERMINATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PIPE_TERMINATION").field("Flags", &self.Flags).field("OutsideFactors", &self.OutsideFactors).field("Weigth", &self.Weigth).field("PhysicalRange", &self.PhysicalRange).field("OptimalRange", &self.OptimalRange).field("Compression", &self.Compression).finish()
    }
}
impl ::core::cmp::PartialEq for PIPE_TERMINATION {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.OutsideFactors == other.OutsideFactors && self.Weigth == other.Weigth && self.PhysicalRange == other.PhysicalRange && self.OptimalRange == other.OptimalRange && self.Compression == other.Compression
    }
}
impl ::core::cmp::Eq for PIPE_TERMINATION {}
unsafe impl ::windows::runtime::Abi for PIPE_TERMINATION {
    type Abi = Self;
}
pub const PROPSETID_ALLOCATOR_CONTROL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1394022752, 5262, 4562, [153, 121, 0, 0, 192, 204, 22, 186]);
pub const PROPSETID_EXT_DEVICE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3044215440, 6700, 4559, [140, 35, 0, 170, 0, 107, 104, 20]);
pub const PROPSETID_EXT_TRANSPORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2688341488, 12357, 4559, [140, 68, 0, 170, 0, 107, 104, 20]);
pub const PROPSETID_TIMECODE_READER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2605280481, 33051, 4559, [140, 119, 0, 170, 0, 107, 104, 20]);
pub const PROPSETID_TUNER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401093, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_CAMERACONTROL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336647536, 12460, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_CAMERACONTROL_FLASH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2019462985, 25506, 16708, [171, 112, 255, 178, 120, 250, 38, 206]);
pub const PROPSETID_VIDCAP_CAMERACONTROL_IMAGE_PIN_CAPABILITY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2638052287, 23661, 16696, [187, 0, 88, 78, 221, 32, 247, 197]);
pub const PROPSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2635256216, 63596, 20461, [176, 35, 93, 135, 101, 61, 167, 147]);
pub const PROPSETID_VIDCAP_CAMERACONTROL_VIDEO_STABILIZATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1133923283, 30486, 16462, [139, 225, 210, 153, 178, 14, 80, 253]);
pub const PROPSETID_VIDCAP_CROSSBAR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401152, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_DROPPEDFRAMES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336647492, 12460, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_SELECTOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(448638666, 26806, 20355, [147, 113, 180, 19, 144, 124, 123, 159]);
pub const PROPSETID_VIDCAP_TVAUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401168, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_VIDEOCOMPRESSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336647491, 12460, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_VIDEOCONTROL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401200, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_VIDEODECODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336647504, 12460, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_VIDEOENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1781401104, 10468, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
pub const PROPSETID_VIDCAP_VIDEOPROCAMP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336647520, 12460, 4560, [161, 140, 0, 160, 201, 17, 137, 86]);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_Align: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_Buffers: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_FixedCompression: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_Flags: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_LogicalEnd: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_MemoryTypes: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_None: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_OptimalRanges: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_PhysicalEnd: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_PhysicalRanges: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_UnknownCompression: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_UserModeDownstream: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const PipeFactor_UserModeUpstream: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub const RT_RCDATA: super::super::Foundation::PWSTR = super::super::Foundation::PWSTR(10i32 as _);
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
pub const RT_STRING: super::super::Foundation::PWSTR = super::super::Foundation::PWSTR(6i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct SECURE_BUFFER_INFO {
    pub guidBufferIdentifier: ::windows::runtime::GUID,
    pub cbBufferSize: u32,
    pub cbCaptured: u32,
    pub ullReserved: [u64; 16],
}
impl SECURE_BUFFER_INFO {}
impl ::core::default::Default for SECURE_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SECURE_BUFFER_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SECURE_BUFFER_INFO").field("guidBufferIdentifier", &self.guidBufferIdentifier).field("cbBufferSize", &self.cbBufferSize).field("cbCaptured", &self.cbCaptured).field("ullReserved", &self.ullReserved).finish()
    }
}
impl ::core::cmp::PartialEq for SECURE_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.guidBufferIdentifier == other.guidBufferIdentifier && self.cbBufferSize == other.cbBufferSize && self.cbCaptured == other.cbCaptured && self.ullReserved == other.ullReserved
    }
}
impl ::core::cmp::Eq for SECURE_BUFFER_INFO {}
unsafe impl ::windows::runtime::Abi for SECURE_BUFFER_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct SOUNDDETECTOR_PATTERNHEADER {
    pub Size: u32,
    pub PatternType: ::windows::runtime::GUID,
}
impl SOUNDDETECTOR_PATTERNHEADER {}
impl ::core::default::Default for SOUNDDETECTOR_PATTERNHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SOUNDDETECTOR_PATTERNHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SOUNDDETECTOR_PATTERNHEADER").field("Size", &self.Size).field("PatternType", &self.PatternType).finish()
    }
}
impl ::core::cmp::PartialEq for SOUNDDETECTOR_PATTERNHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.PatternType == other.PatternType
    }
}
impl ::core::cmp::Eq for SOUNDDETECTOR_PATTERNHEADER {}
unsafe impl ::windows::runtime::Abi for SOUNDDETECTOR_PATTERNHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_ALL: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_BACK_CENTER: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_BACK_LEFT: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_BACK_RIGHT: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_FRONT_CENTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_FRONT_LEFT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_FRONT_LEFT_OF_CENTER: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_FRONT_RIGHT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_LOW_FREQUENCY: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_RESERVED: u32 = 2147221504u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_SIDE_LEFT: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_SIDE_RIGHT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_TOP_BACK_CENTER: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_TOP_BACK_LEFT: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_TOP_BACK_RIGHT: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_TOP_CENTER: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_TOP_FRONT_CENTER: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_TOP_FRONT_LEFT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SPEAKER_TOP_FRONT_RIGHT: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SYSAUDIO_FLAGS_CLEAR_PREFERRED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const SYSAUDIO_FLAGS_DONT_COMBINE_PINS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TELEPHONY_CALLCONTROLOP(pub i32);
pub const TELEPHONY_CALLCONTROLOP_DISABLE: TELEPHONY_CALLCONTROLOP = TELEPHONY_CALLCONTROLOP(0i32);
pub const TELEPHONY_CALLCONTROLOP_ENABLE: TELEPHONY_CALLCONTROLOP = TELEPHONY_CALLCONTROLOP(1i32);
impl ::core::convert::From<i32> for TELEPHONY_CALLCONTROLOP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TELEPHONY_CALLCONTROLOP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TELEPHONY_CALLSTATE(pub i32);
pub const TELEPHONY_CALLSTATE_DISABLED: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(0i32);
pub const TELEPHONY_CALLSTATE_ENABLED: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(1i32);
pub const TELEPHONY_CALLSTATE_HOLD: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(2i32);
pub const TELEPHONY_CALLSTATE_PROVIDERTRANSITION: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(3i32);
impl ::core::convert::From<i32> for TELEPHONY_CALLSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TELEPHONY_CALLSTATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TELEPHONY_CALLTYPE(pub i32);
pub const TELEPHONY_CALLTYPE_CIRCUITSWITCHED: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(0i32);
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_LTE: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(1i32);
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_WLAN: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(2i32);
impl ::core::convert::From<i32> for TELEPHONY_CALLTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TELEPHONY_CALLTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TELEPHONY_PROVIDERCHANGEOP(pub i32);
pub const TELEPHONY_PROVIDERCHANGEOP_END: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(0i32);
pub const TELEPHONY_PROVIDERCHANGEOP_BEGIN: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(1i32);
pub const TELEPHONY_PROVIDERCHANGEOP_CANCEL: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(2i32);
impl ::core::convert::From<i32> for TELEPHONY_PROVIDERCHANGEOP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TELEPHONY_PROVIDERCHANGEOP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct TRANSPORTAUDIOPARMS {
    pub EnableOutput: i32,
    pub EnableRecord: i32,
    pub EnableSelsync: i32,
    pub Input: i32,
    pub MonitorSource: i32,
}
impl TRANSPORTAUDIOPARMS {}
impl ::core::default::Default for TRANSPORTAUDIOPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRANSPORTAUDIOPARMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRANSPORTAUDIOPARMS").field("EnableOutput", &self.EnableOutput).field("EnableRecord", &self.EnableRecord).field("EnableSelsync", &self.EnableSelsync).field("Input", &self.Input).field("MonitorSource", &self.MonitorSource).finish()
    }
}
impl ::core::cmp::PartialEq for TRANSPORTAUDIOPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.EnableOutput == other.EnableOutput && self.EnableRecord == other.EnableRecord && self.EnableSelsync == other.EnableSelsync && self.Input == other.Input && self.MonitorSource == other.MonitorSource
    }
}
impl ::core::cmp::Eq for TRANSPORTAUDIOPARMS {}
unsafe impl ::windows::runtime::Abi for TRANSPORTAUDIOPARMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl TRANSPORTBASICPARMS {}
impl ::core::default::Default for TRANSPORTBASICPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRANSPORTBASICPARMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRANSPORTBASICPARMS")
            .field("TimeFormat", &self.TimeFormat)
            .field("TimeReference", &self.TimeReference)
            .field("Superimpose", &self.Superimpose)
            .field("EndStopAction", &self.EndStopAction)
            .field("RecordFormat", &self.RecordFormat)
            .field("StepFrames", &self.StepFrames)
            .field("SetpField", &self.SetpField)
            .field("Preroll", &self.Preroll)
            .field("RecPreroll", &self.RecPreroll)
            .field("Postroll", &self.Postroll)
            .field("EditDelay", &self.EditDelay)
            .field("PlayTCDelay", &self.PlayTCDelay)
            .field("RecTCDelay", &self.RecTCDelay)
            .field("EditField", &self.EditField)
            .field("FrameServo", &self.FrameServo)
            .field("ColorFrameServo", &self.ColorFrameServo)
            .field("ServoRef", &self.ServoRef)
            .field("WarnGenlock", &self.WarnGenlock)
            .field("SetTracking", &self.SetTracking)
            .field("VolumeName", &self.VolumeName)
            .field("Ballistic", &self.Ballistic)
            .field("Speed", &self.Speed)
            .field("CounterFormat", &self.CounterFormat)
            .field("TunerChannel", &self.TunerChannel)
            .field("TunerNumber", &self.TunerNumber)
            .field("TimerEvent", &self.TimerEvent)
            .field("TimerStartDay", &self.TimerStartDay)
            .field("TimerStartTime", &self.TimerStartTime)
            .field("TimerStopDay", &self.TimerStopDay)
            .field("TimerStopTime", &self.TimerStopTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSPORTBASICPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.TimeFormat == other.TimeFormat
            && self.TimeReference == other.TimeReference
            && self.Superimpose == other.Superimpose
            && self.EndStopAction == other.EndStopAction
            && self.RecordFormat == other.RecordFormat
            && self.StepFrames == other.StepFrames
            && self.SetpField == other.SetpField
            && self.Preroll == other.Preroll
            && self.RecPreroll == other.RecPreroll
            && self.Postroll == other.Postroll
            && self.EditDelay == other.EditDelay
            && self.PlayTCDelay == other.PlayTCDelay
            && self.RecTCDelay == other.RecTCDelay
            && self.EditField == other.EditField
            && self.FrameServo == other.FrameServo
            && self.ColorFrameServo == other.ColorFrameServo
            && self.ServoRef == other.ServoRef
            && self.WarnGenlock == other.WarnGenlock
            && self.SetTracking == other.SetTracking
            && self.VolumeName == other.VolumeName
            && self.Ballistic == other.Ballistic
            && self.Speed == other.Speed
            && self.CounterFormat == other.CounterFormat
            && self.TunerChannel == other.TunerChannel
            && self.TunerNumber == other.TunerNumber
            && self.TimerEvent == other.TimerEvent
            && self.TimerStartDay == other.TimerStartDay
            && self.TimerStartTime == other.TimerStartTime
            && self.TimerStopDay == other.TimerStopDay
            && self.TimerStopTime == other.TimerStopTime
    }
}
impl ::core::cmp::Eq for TRANSPORTBASICPARMS {}
unsafe impl ::windows::runtime::Abi for TRANSPORTBASICPARMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl TRANSPORTSTATUS {}
impl ::core::default::Default for TRANSPORTSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRANSPORTSTATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRANSPORTSTATUS")
            .field("Mode", &self.Mode)
            .field("LastError", &self.LastError)
            .field("RecordInhibit", &self.RecordInhibit)
            .field("ServoLock", &self.ServoLock)
            .field("MediaPresent", &self.MediaPresent)
            .field("MediaLength", &self.MediaLength)
            .field("MediaSize", &self.MediaSize)
            .field("MediaTrackCount", &self.MediaTrackCount)
            .field("MediaTrackLength", &self.MediaTrackLength)
            .field("MediaTrackSide", &self.MediaTrackSide)
            .field("MediaType", &self.MediaType)
            .field("LinkMode", &self.LinkMode)
            .field("NotifyOn", &self.NotifyOn)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSPORTSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode
            && self.LastError == other.LastError
            && self.RecordInhibit == other.RecordInhibit
            && self.ServoLock == other.ServoLock
            && self.MediaPresent == other.MediaPresent
            && self.MediaLength == other.MediaLength
            && self.MediaSize == other.MediaSize
            && self.MediaTrackCount == other.MediaTrackCount
            && self.MediaTrackLength == other.MediaTrackLength
            && self.MediaTrackSide == other.MediaTrackSide
            && self.MediaType == other.MediaType
            && self.LinkMode == other.LinkMode
            && self.NotifyOn == other.NotifyOn
    }
}
impl ::core::cmp::Eq for TRANSPORTSTATUS {}
unsafe impl ::windows::runtime::Abi for TRANSPORTSTATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct TRANSPORTVIDEOPARMS {
    pub OutputMode: i32,
    pub Input: i32,
}
impl TRANSPORTVIDEOPARMS {}
impl ::core::default::Default for TRANSPORTVIDEOPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRANSPORTVIDEOPARMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRANSPORTVIDEOPARMS").field("OutputMode", &self.OutputMode).field("Input", &self.Input).finish()
    }
}
impl ::core::cmp::PartialEq for TRANSPORTVIDEOPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.OutputMode == other.OutputMode && self.Input == other.Input
    }
}
impl ::core::cmp::Eq for TRANSPORTVIDEOPARMS {}
unsafe impl ::windows::runtime::Abi for TRANSPORTVIDEOPARMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct TRANSPORT_STATE {
    pub Mode: u32,
    pub State: u32,
}
impl TRANSPORT_STATE {}
impl ::core::default::Default for TRANSPORT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRANSPORT_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRANSPORT_STATE").field("Mode", &self.Mode).field("State", &self.State).finish()
    }
}
impl ::core::cmp::PartialEq for TRANSPORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.State == other.State
    }
}
impl ::core::cmp::Eq for TRANSPORT_STATE {}
unsafe impl ::windows::runtime::Abi for TRANSPORT_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl TUNER_ANALOG_CAPS_S {}
impl ::core::default::Default for TUNER_ANALOG_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TUNER_ANALOG_CAPS_S {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TUNER_ANALOG_CAPS_S")
            .field("Mode", &self.Mode)
            .field("StandardsSupported", &self.StandardsSupported)
            .field("MinFrequency", &self.MinFrequency)
            .field("MaxFrequency", &self.MaxFrequency)
            .field("TuningGranularity", &self.TuningGranularity)
            .field("SettlingTime", &self.SettlingTime)
            .field("ScanSensingRange", &self.ScanSensingRange)
            .field("FineTuneSensingRange", &self.FineTuneSensingRange)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TUNER_ANALOG_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.StandardsSupported == other.StandardsSupported && self.MinFrequency == other.MinFrequency && self.MaxFrequency == other.MaxFrequency && self.TuningGranularity == other.TuningGranularity && self.SettlingTime == other.SettlingTime && self.ScanSensingRange == other.ScanSensingRange && self.FineTuneSensingRange == other.FineTuneSensingRange
    }
}
impl ::core::cmp::Eq for TUNER_ANALOG_CAPS_S {}
unsafe impl ::windows::runtime::Abi for TUNER_ANALOG_CAPS_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICAP_PROPERTIES_PROTECTION_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Status: u32,
}
impl VBICAP_PROPERTIES_PROTECTION_S {}
impl ::core::default::Default for VBICAP_PROPERTIES_PROTECTION_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICAP_PROPERTIES_PROTECTION_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VBICAP_PROPERTIES_PROTECTION_S {}
unsafe impl ::windows::runtime::Abi for VBICAP_PROPERTIES_PROTECTION_S {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICODECFILTERING_CC_SUBSTREAMS {
    pub SubstreamMask: u32,
}
impl VBICODECFILTERING_CC_SUBSTREAMS {}
impl ::core::default::Default for VBICODECFILTERING_CC_SUBSTREAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_CC_SUBSTREAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_CC_SUBSTREAMS").field("SubstreamMask", &self.SubstreamMask).finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_CC_SUBSTREAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SubstreamMask == other.SubstreamMask
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_CC_SUBSTREAMS {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_CC_SUBSTREAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICODECFILTERING_NABTS_SUBSTREAMS {
    pub SubstreamMask: [u32; 128],
}
impl VBICODECFILTERING_NABTS_SUBSTREAMS {}
impl ::core::default::Default for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_NABTS_SUBSTREAMS").field("SubstreamMask", &self.SubstreamMask).finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SubstreamMask == other.SubstreamMask
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_NABTS_SUBSTREAMS {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_NABTS_SUBSTREAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICODECFILTERING_SCANLINES {
    pub DwordBitArray: [u32; 32],
}
impl VBICODECFILTERING_SCANLINES {}
impl ::core::default::Default for VBICODECFILTERING_SCANLINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_SCANLINES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_SCANLINES").field("DwordBitArray", &self.DwordBitArray).finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_SCANLINES {
    fn eq(&self, other: &Self) -> bool {
        self.DwordBitArray == other.DwordBitArray
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_SCANLINES {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_SCANLINES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICODECFILTERING_STATISTICS_CC {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
impl VBICODECFILTERING_STATISTICS_CC {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_CC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_CC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_STATISTICS_CC").field("Common", &self.Common).finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_CC {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_CC {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_STATISTICS_CC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICODECFILTERING_STATISTICS_CC_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl VBICODECFILTERING_STATISTICS_CC_PIN {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_STATISTICS_CC_PIN").field("Common", &self.Common).finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_CC_PIN {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_STATISTICS_CC_PIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl VBICODECFILTERING_STATISTICS_COMMON {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_COMMON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_COMMON {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_STATISTICS_COMMON")
            .field("InputSRBsProcessed", &self.InputSRBsProcessed)
            .field("OutputSRBsProcessed", &self.OutputSRBsProcessed)
            .field("SRBsIgnored", &self.SRBsIgnored)
            .field("InputSRBsMissing", &self.InputSRBsMissing)
            .field("OutputSRBsMissing", &self.OutputSRBsMissing)
            .field("OutputFailures", &self.OutputFailures)
            .field("InternalErrors", &self.InternalErrors)
            .field("ExternalErrors", &self.ExternalErrors)
            .field("InputDiscontinuities", &self.InputDiscontinuities)
            .field("DSPFailures", &self.DSPFailures)
            .field("TvTunerChanges", &self.TvTunerChanges)
            .field("VBIHeaderChanges", &self.VBIHeaderChanges)
            .field("LineConfidenceAvg", &self.LineConfidenceAvg)
            .field("BytesOutput", &self.BytesOutput)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_COMMON {
    fn eq(&self, other: &Self) -> bool {
        self.InputSRBsProcessed == other.InputSRBsProcessed
            && self.OutputSRBsProcessed == other.OutputSRBsProcessed
            && self.SRBsIgnored == other.SRBsIgnored
            && self.InputSRBsMissing == other.InputSRBsMissing
            && self.OutputSRBsMissing == other.OutputSRBsMissing
            && self.OutputFailures == other.OutputFailures
            && self.InternalErrors == other.InternalErrors
            && self.ExternalErrors == other.ExternalErrors
            && self.InputDiscontinuities == other.InputDiscontinuities
            && self.DSPFailures == other.DSPFailures
            && self.TvTunerChanges == other.TvTunerChanges
            && self.VBIHeaderChanges == other.VBIHeaderChanges
            && self.LineConfidenceAvg == other.LineConfidenceAvg
            && self.BytesOutput == other.BytesOutput
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_COMMON {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_STATISTICS_COMMON {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl VBICODECFILTERING_STATISTICS_COMMON_PIN {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_STATISTICS_COMMON_PIN")
            .field("SRBsProcessed", &self.SRBsProcessed)
            .field("SRBsIgnored", &self.SRBsIgnored)
            .field("SRBsMissing", &self.SRBsMissing)
            .field("InternalErrors", &self.InternalErrors)
            .field("ExternalErrors", &self.ExternalErrors)
            .field("Discontinuities", &self.Discontinuities)
            .field("LineConfidenceAvg", &self.LineConfidenceAvg)
            .field("BytesOutput", &self.BytesOutput)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn eq(&self, other: &Self) -> bool {
        self.SRBsProcessed == other.SRBsProcessed && self.SRBsIgnored == other.SRBsIgnored && self.SRBsMissing == other.SRBsMissing && self.InternalErrors == other.InternalErrors && self.ExternalErrors == other.ExternalErrors && self.Discontinuities == other.Discontinuities && self.LineConfidenceAvg == other.LineConfidenceAvg && self.BytesOutput == other.BytesOutput
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_COMMON_PIN {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl VBICODECFILTERING_STATISTICS_NABTS {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_NABTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_NABTS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_STATISTICS_NABTS")
            .field("Common", &self.Common)
            .field("FECBundleBadLines", &self.FECBundleBadLines)
            .field("FECQueueOverflows", &self.FECQueueOverflows)
            .field("FECCorrectedLines", &self.FECCorrectedLines)
            .field("FECUncorrectableLines", &self.FECUncorrectableLines)
            .field("BundlesProcessed", &self.BundlesProcessed)
            .field("BundlesSent2IP", &self.BundlesSent2IP)
            .field("FilteredLines", &self.FilteredLines)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_NABTS {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common && self.FECBundleBadLines == other.FECBundleBadLines && self.FECQueueOverflows == other.FECQueueOverflows && self.FECCorrectedLines == other.FECCorrectedLines && self.FECUncorrectableLines == other.FECUncorrectableLines && self.BundlesProcessed == other.BundlesProcessed && self.BundlesSent2IP == other.BundlesSent2IP && self.FilteredLines == other.FilteredLines
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_NABTS {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_STATISTICS_NABTS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICODECFILTERING_STATISTICS_NABTS_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl VBICODECFILTERING_STATISTICS_NABTS_PIN {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_STATISTICS_NABTS_PIN").field("Common", &self.Common).finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_NABTS_PIN {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
impl VBICODECFILTERING_STATISTICS_TELETEXT {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_STATISTICS_TELETEXT").field("Common", &self.Common).finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_TELETEXT {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_STATISTICS_TELETEXT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl VBICODECFILTERING_STATISTICS_TELETEXT_PIN {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBICODECFILTERING_STATISTICS_TELETEXT_PIN").field("Common", &self.Common).finish()
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {}
unsafe impl ::windows::runtime::Abi for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
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
impl VRAM_SURFACE_INFO {}
impl ::core::default::Default for VRAM_SURFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VRAM_SURFACE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VRAM_SURFACE_INFO")
            .field("hSurface", &self.hSurface)
            .field("VramPhysicalAddress", &self.VramPhysicalAddress)
            .field("cbCaptured", &self.cbCaptured)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwLinearSize", &self.dwLinearSize)
            .field("lPitch", &self.lPitch)
            .field("ullReserved", &self.ullReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VRAM_SURFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hSurface == other.hSurface && self.VramPhysicalAddress == other.VramPhysicalAddress && self.cbCaptured == other.cbCaptured && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwLinearSize == other.dwLinearSize && self.lPitch == other.lPitch && self.ullReserved == other.ullReserved
    }
}
impl ::core::cmp::Eq for VRAM_SURFACE_INFO {}
unsafe impl ::windows::runtime::Abi for VRAM_SURFACE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct VRAM_SURFACE_INFO_PROPERTY_S {
    pub Property: KSIDENTIFIER,
    pub pVramSurfaceInfo: *mut VRAM_SURFACE_INFO,
}
impl VRAM_SURFACE_INFO_PROPERTY_S {}
impl ::core::default::Default for VRAM_SURFACE_INFO_PROPERTY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VRAM_SURFACE_INFO_PROPERTY_S {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VRAM_SURFACE_INFO_PROPERTY_S {}
unsafe impl ::windows::runtime::Abi for VRAM_SURFACE_INFO_PROPERTY_S {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const WAVE_FORMAT_EXTENSIBLE: u32 = 65534u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct WNF_KSCAMERA_STREAMSTATE_INFO {
    pub ProcessId: u32,
    pub SessionId: u32,
    pub StreamState: u32,
    pub Reserved: u32,
}
impl WNF_KSCAMERA_STREAMSTATE_INFO {}
impl ::core::default::Default for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WNF_KSCAMERA_STREAMSTATE_INFO").field("ProcessId", &self.ProcessId).field("SessionId", &self.SessionId).field("StreamState", &self.StreamState).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessId == other.ProcessId && self.SessionId == other.SessionId && self.StreamState == other.StreamState && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WNF_KSCAMERA_STREAMSTATE_INFO {}
unsafe impl ::windows::runtime::Abi for WNF_KSCAMERA_STREAMSTATE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct WST_BUFFER {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub WstLines: [WST_BUFFER_LINE; 17],
}
impl WST_BUFFER {}
impl ::core::default::Default for WST_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WST_BUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WST_BUFFER").field("ScanlinesRequested", &self.ScanlinesRequested).field("WstLines", &self.WstLines).finish()
    }
}
impl ::core::cmp::PartialEq for WST_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ScanlinesRequested == other.ScanlinesRequested && self.WstLines == other.WstLines
    }
}
impl ::core::cmp::Eq for WST_BUFFER {}
unsafe impl ::windows::runtime::Abi for WST_BUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct WST_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 42],
}
impl WST_BUFFER_LINE {}
impl ::core::default::Default for WST_BUFFER_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WST_BUFFER_LINE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WST_BUFFER_LINE").field("Confidence", &self.Confidence).field("Bytes", &self.Bytes).finish()
    }
}
impl ::core::cmp::PartialEq for WST_BUFFER_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Confidence == other.Confidence && self.Bytes == other.Bytes
    }
}
impl ::core::cmp::Eq for WST_BUFFER_LINE {}
unsafe impl ::windows::runtime::Abi for WST_BUFFER_LINE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const WST_BYTES_PER_LINE: u32 = 42u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const WST_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const WST_TVTUNER_CHANGE_END_TUNE: i32 = 8192i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub struct _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    pub ProcessingMode: ::windows::runtime::GUID,
    pub SamplesPerProcessingPacket: u32,
    pub ProcessingPacketDurationInHns: u32,
}
impl _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {}
impl ::core::default::Default for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT").field("ProcessingMode", &self.ProcessingMode).field("SamplesPerProcessingPacket", &self.SamplesPerProcessingPacket).field("ProcessingPacketDurationInHns", &self.ProcessingPacketDurationInHns).finish()
    }
}
impl ::core::cmp::PartialEq for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessingMode == other.ProcessingMode && self.SamplesPerProcessingPacket == other.SamplesPerProcessingPacket && self.ProcessingPacketDurationInHns == other.ProcessingPacketDurationInHns
    }
}
impl ::core::cmp::Eq for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {}
unsafe impl ::windows::runtime::Abi for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _TunerDecoderLockType(pub i32);
pub const Tuner_LockType_None: _TunerDecoderLockType = _TunerDecoderLockType(0i32);
pub const Tuner_LockType_Within_Scan_Sensing_Range: _TunerDecoderLockType = _TunerDecoderLockType(1i32);
pub const Tuner_LockType_Locked: _TunerDecoderLockType = _TunerDecoderLockType(2i32);
impl ::core::convert::From<i32> for _TunerDecoderLockType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _TunerDecoderLockType {
    type Abi = Self;
}
