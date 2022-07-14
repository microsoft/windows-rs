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
    pub MemoryType: ::windows::core::GUID,
    pub BusType: ::windows::core::GUID,
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
impl ::core::fmt::Debug for ALLOCATOR_PROPERTIES_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ALLOCATOR_PROPERTIES_EX")
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
unsafe impl ::windows::core::Abi for ALLOCATOR_PROPERTIES_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ALLOCATOR_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ALLOCATOR_PROPERTIES_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for ALLOCATOR_PROPERTIES_EX {}
impl ::core::default::Default for ALLOCATOR_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const APO_CLASS_UUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5989fce8_9cd0_467d_8a6a_5419e31529d4);
pub const AUDIOENDPOINT_CLASS_UUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc166523c_fe0c_4a94_a586_f1a80cfbbf3e);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOMODULE_MAX_DATA_SIZE: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOMODULE_MAX_NAME_CCH_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIOPOSTURE_ORIENTATION(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOPOSTURE_ORIENTATION_NOTROTATED: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOPOSTURE_ORIENTATION_ROTATED90DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOPOSTURE_ORIENTATION_ROTATED180DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIOPOSTURE_ORIENTATION_ROTATED270DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(3i32);
impl ::core::marker::Copy for AUDIOPOSTURE_ORIENTATION {}
impl ::core::clone::Clone for AUDIOPOSTURE_ORIENTATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIOPOSTURE_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDIOPOSTURE_ORIENTATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIOPOSTURE_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIOPOSTURE_ORIENTATION").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIORESOURCEMANAGEMENT_RESOURCEGROUP").field("ResourceGroupAcquired", &self.ResourceGroupAcquired).field("ResourceGroupName", &self.ResourceGroupName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIORESOURCEMANAGEMENT_RESOURCEGROUP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIO_CURVE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIO_CURVE_TYPE_NONE: AUDIO_CURVE_TYPE = AUDIO_CURVE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const AUDIO_CURVE_TYPE_WINDOWS_FADE: AUDIO_CURVE_TYPE = AUDIO_CURVE_TYPE(1i32);
impl ::core::marker::Copy for AUDIO_CURVE_TYPE {}
impl ::core::clone::Clone for AUDIO_CURVE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_CURVE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_CURVE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIO_CURVE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_CURVE_TYPE").field(&self.0).finish()
    }
}
pub const AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adbe_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_AUTOMATIC_GAIN_CONTROL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc0_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BASS_BOOST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc5_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BASS_MANAGEMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adca_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BEAMFORMING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc1_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_CONSTANT_TONE_REMOVAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc2_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_DEEP_NOISE_SUPPRESSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64add0_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_DYNAMIC_RANGE_COMPRESSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adce_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_ENVIRONMENTAL_EFFECTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adcb_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_EQUALIZER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc3_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_FAR_FIELD_BEAMFORMING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adcf_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_LOUDNESS_EQUALIZER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc4_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_NOISE_SUPPRESSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adbf_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_ROOM_CORRECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc9_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_COMPENSATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adcd_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_FILL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc8_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_PROTECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adcc_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_VIRTUAL_HEADPHONES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc7_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_VIRTUAL_SURROUND: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f64adc6_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_SIGNALPROCESSINGMODE_COMMUNICATIONS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98951333_b9cd_48b1_a0a3_ff40682d73f7);
pub const AUDIO_SIGNALPROCESSINGMODE_DEFAULT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc18e2f7e_933d_4965_b7d1_1eef228d2af3);
pub const AUDIO_SIGNALPROCESSINGMODE_FAR_FIELD_SPEECH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28941cba_3be6_4a78_9a76_30fd91559b64);
pub const AUDIO_SIGNALPROCESSINGMODE_MEDIA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4780004e_7133_41d8_8c74_660dadd2c0ee);
pub const AUDIO_SIGNALPROCESSINGMODE_MOVIE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb26feb0d_ec94_477c_9494_d1ab8e753f6e);
pub const AUDIO_SIGNALPROCESSINGMODE_NOTIFICATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cf2a70b_f377_403b_bd6b_360863e0355c);
pub const AUDIO_SIGNALPROCESSINGMODE_RAW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e90ea20_b493_4fd1_a1a8_7e1361a956cf);
pub const AUDIO_SIGNALPROCESSINGMODE_SPEECH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc1cfc9b_b9d6_4cfa_b5e0_4bb2166878b2);
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
pub const BLUETOOTHLE_MIDI_SERVICE_UUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b80e5a_ede8_4b33_a751_6ce34ec4c700);
pub const BLUETOOTH_MIDI_DATAIO_CHARACTERISTIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7772e5db_3868_4112_a1a9_f2669d106bf3);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const BUS_INTERFACE_REFERENCE_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CAPTURE_MEMORY_ALLOCATION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_INVALID: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_SYSTEM: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_VRAM: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_SYSTEM_AGP: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_VRAM_MAPPED: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAPTURE_ALLOC_SECURE_BUFFER: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(16i32);
impl ::core::marker::Copy for CAPTURE_MEMORY_ALLOCATION_FLAGS {}
impl ::core::clone::Clone for CAPTURE_MEMORY_ALLOCATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CAPTURE_MEMORY_ALLOCATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CAPTURE_MEMORY_ALLOCATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CAPTURE_MEMORY_ALLOCATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CAPTURE_MEMORY_ALLOCATION_FLAGS").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for CC_BYTE_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CC_BYTE_PAIR").field("Decoded", &self.Decoded).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for CC_BYTE_PAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CC_BYTE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CC_BYTE_PAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for CC_BYTE_PAIR {}
impl ::core::default::Default for CC_BYTE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CC_HW_FIELD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CC_HW_FIELD").field("ScanlinesRequested", &self.ScanlinesRequested).field("fieldFlags", &self.fieldFlags).field("PictureNumber", &self.PictureNumber).field("Lines", &self.Lines).finish()
    }
}
unsafe impl ::windows::core::Abi for CC_HW_FIELD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CC_HW_FIELD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CC_HW_FIELD>()) == 0 }
    }
}
impl ::core::cmp::Eq for CC_HW_FIELD {}
impl ::core::default::Default for CC_HW_FIELD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const CC_MAX_HW_DECODE_LINES: u32 = 12u32;
pub const CLSID_KsIBasicAudioInterfaceHandler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f8ac3e_0f71_11d2_b72c_00c04fb6bd3d);
pub const CLSID_Proxy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cca71b_ecd7_11d0_b908_00a0c9223196);
pub const CODECAPI_ALLSETTINGS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a577e92_83e1_4113_adc2_4fcec32f83a1);
pub const CODECAPI_AUDIO_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9d19a3e_f897_429c_bc46_8138b7272b2d);
pub const CODECAPI_CHANGELISTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b12acf_f6b0_47d9_9456_96f22c4e0b9d);
pub const CODECAPI_CURRENTCHANGELIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cb14e83_7d72_4657_83fd_47a2c5b9d13d);
pub const CODECAPI_SETALLDEFAULTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c5e6a7c_acf8_4f55_a999_1a628109051b);
pub const CODECAPI_SUPPORTSEVENTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0581af97_7693_4dbd_9dca_3f9ebd6585a1);
pub const CODECAPI_VIDEO_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7112e8e1_3d03_47ef_8e60_03f1cf537301);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONSTRICTOR_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const CONSTRICTOR_OPTION_DISABLE: CONSTRICTOR_OPTION = CONSTRICTOR_OPTION(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const CONSTRICTOR_OPTION_MUTE: CONSTRICTOR_OPTION = CONSTRICTOR_OPTION(1i32);
impl ::core::marker::Copy for CONSTRICTOR_OPTION {}
impl ::core::clone::Clone for CONSTRICTOR_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONSTRICTOR_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONSTRICTOR_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONSTRICTOR_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSTRICTOR_OPTION").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for DEVCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVCAPS")
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
unsafe impl ::windows::core::Abi for DEVCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVCAPS {}
impl ::core::default::Default for DEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_KsAudio_Controller_DeviceInterface_Path: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x13e004d6_b066_43bd_913b_a415cd13da87), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_KsAudio_PacketSize_Constraints: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x13e004d6_b066_43bd_913b_a415cd13da87), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_KsAudio_PacketSize_Constraints2: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x9404f781_7191_409b_8b0b_80bf6ec229ae), pid: 2u32 };
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
unsafe impl ::windows::core::Abi for DS3DVECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DS3DVECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DS3DVECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DS3DVECTOR {}
impl ::core::default::Default for DS3DVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DS3DVECTOR_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DS3DVECTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DS3DVECTOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DS3DVECTOR_0 {}
impl ::core::default::Default for DS3DVECTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DS3DVECTOR_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DS3DVECTOR_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DS3DVECTOR_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for DS3DVECTOR_1 {}
impl ::core::default::Default for DS3DVECTOR_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DS3DVECTOR_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DS3DVECTOR_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DS3DVECTOR_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for DS3DVECTOR_2 {}
impl ::core::default::Default for DS3DVECTOR_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const ENCAPIPARAM_BITRATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49cc4c43_ca83_4ad4_a9af_f3696af666df);
pub const ENCAPIPARAM_BITRATE_MODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5fb25c_c713_40d1_9d58_c0d7241e250f);
pub const ENCAPIPARAM_PEAK_BITRATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x703f16a9_3d48_44a1_b077_018dff915d19);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPcxConnectionType(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeUnknown: EPcxConnectionType = EPcxConnectionType(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnType3Point5mm: EPcxConnectionType = EPcxConnectionType(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeQuarter: EPcxConnectionType = EPcxConnectionType(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeAtapiInternal: EPcxConnectionType = EPcxConnectionType(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeRCA: EPcxConnectionType = EPcxConnectionType(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeOptical: EPcxConnectionType = EPcxConnectionType(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeOtherDigital: EPcxConnectionType = EPcxConnectionType(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeOtherAnalog: EPcxConnectionType = EPcxConnectionType(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeMultichannelAnalogDIN: EPcxConnectionType = EPcxConnectionType(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeXlrProfessional: EPcxConnectionType = EPcxConnectionType(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeRJ11Modem: EPcxConnectionType = EPcxConnectionType(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eConnTypeCombination: EPcxConnectionType = EPcxConnectionType(11i32);
impl ::core::marker::Copy for EPcxConnectionType {}
impl ::core::clone::Clone for EPcxConnectionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EPcxConnectionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EPcxConnectionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EPcxConnectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPcxConnectionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPcxGenLocation(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGenLocPrimaryBox: EPcxGenLocation = EPcxGenLocation(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGenLocInternal: EPcxGenLocation = EPcxGenLocation(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGenLocSeparate: EPcxGenLocation = EPcxGenLocation(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGenLocOther: EPcxGenLocation = EPcxGenLocation(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const EPcxGenLocation_enum_count: EPcxGenLocation = EPcxGenLocation(4i32);
impl ::core::marker::Copy for EPcxGenLocation {}
impl ::core::clone::Clone for EPcxGenLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EPcxGenLocation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EPcxGenLocation {
    type Abi = Self;
}
impl ::core::fmt::Debug for EPcxGenLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPcxGenLocation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPcxGeoLocation(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocRear: EPcxGeoLocation = EPcxGeoLocation(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocFront: EPcxGeoLocation = EPcxGeoLocation(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocLeft: EPcxGeoLocation = EPcxGeoLocation(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocRight: EPcxGeoLocation = EPcxGeoLocation(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocTop: EPcxGeoLocation = EPcxGeoLocation(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocBottom: EPcxGeoLocation = EPcxGeoLocation(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocRearPanel: EPcxGeoLocation = EPcxGeoLocation(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocRiser: EPcxGeoLocation = EPcxGeoLocation(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocInsideMobileLid: EPcxGeoLocation = EPcxGeoLocation(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocDrivebay: EPcxGeoLocation = EPcxGeoLocation(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocHDMI: EPcxGeoLocation = EPcxGeoLocation(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocOutsideMobileLid: EPcxGeoLocation = EPcxGeoLocation(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocATAPI: EPcxGeoLocation = EPcxGeoLocation(13i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocNotApplicable: EPcxGeoLocation = EPcxGeoLocation(14i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const eGeoLocReserved6: EPcxGeoLocation = EPcxGeoLocation(15i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const EPcxGeoLocation_enum_count: EPcxGeoLocation = EPcxGeoLocation(16i32);
impl ::core::marker::Copy for EPcxGeoLocation {}
impl ::core::clone::Clone for EPcxGeoLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EPcxGeoLocation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EPcxGeoLocation {
    type Abi = Self;
}
impl ::core::fmt::Debug for EPcxGeoLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPcxGeoLocation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPxcPortConnection(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const ePortConnJack: EPxcPortConnection = EPxcPortConnection(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const ePortConnIntegratedDevice: EPxcPortConnection = EPxcPortConnection(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const ePortConnBothIntegratedAndJack: EPxcPortConnection = EPxcPortConnection(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const ePortConnUnknown: EPxcPortConnection = EPxcPortConnection(3i32);
impl ::core::marker::Copy for EPxcPortConnection {}
impl ::core::clone::Clone for EPxcPortConnection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EPxcPortConnection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EPxcPortConnection {
    type Abi = Self;
}
impl ::core::fmt::Debug for EPxcPortConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPxcPortConnection").field(&self.0).finish()
    }
}
pub const EVENTSETID_CROSSBAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0641_28e4_11d0_a18c_00a0c9118956);
pub const EVENTSETID_TUNER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0606_28e4_11d0_a18c_00a0c9118956);
pub const EVENTSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fdffc5d_c732_4ba6_b5df_6b4d7fc88b8b);
pub const EVENTSETID_VIDEODECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0621_28e4_11d0_a18c_00a0c9118956);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FRAMING_CACHE_OPS(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Framing_Cache_Update: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Framing_Cache_ReadLast: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Framing_Cache_ReadOrig: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Framing_Cache_Write: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(3i32);
impl ::core::marker::Copy for FRAMING_CACHE_OPS {}
impl ::core::clone::Clone for FRAMING_CACHE_OPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FRAMING_CACHE_OPS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FRAMING_CACHE_OPS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FRAMING_CACHE_OPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMING_CACHE_OPS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FRAMING_PROP(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FramingProp_Uninitialized: FRAMING_PROP = FRAMING_PROP(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FramingProp_None: FRAMING_PROP = FRAMING_PROP(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FramingProp_Old: FRAMING_PROP = FRAMING_PROP(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FramingProp_Ex: FRAMING_PROP = FRAMING_PROP(3i32);
impl ::core::marker::Copy for FRAMING_PROP {}
impl ::core::clone::Clone for FRAMING_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FRAMING_PROP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FRAMING_PROP {
    type Abi = Self;
}
impl ::core::fmt::Debug for FRAMING_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMING_PROP").field(&self.0).finish()
    }
}
pub const GUID_NULL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsAggregateControl(::windows::core::IUnknown);
impl IKsAggregateControl {
    pub unsafe fn KsAddAggregate(&self, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KsAddAggregate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(aggregateclass)).ok()
    }
    pub unsafe fn KsRemoveAggregate(&self, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KsRemoveAggregate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(aggregateclass)).ok()
    }
}
impl ::core::convert::From<IKsAggregateControl> for ::windows::core::IUnknown {
    fn from(value: IKsAggregateControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsAggregateControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsAggregateControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsAggregateControl> for ::windows::core::IUnknown {
    fn from(value: &IKsAggregateControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsAggregateControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsAggregateControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsAggregateControl {}
impl ::core::fmt::Debug for IKsAggregateControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsAggregateControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsAggregateControl {
    type Vtable = IKsAggregateControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f40eac0_3947_11d2_874e_00a0c9223196);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsAggregateControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub KsAddAggregate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub KsRemoveAggregate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct IKsAllocator(pub u8);
#[repr(C)]
pub struct IKsAllocatorEx(pub u8);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsControl(::windows::core::IUnknown);
impl IKsControl {
    pub unsafe fn KsProperty(&self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KsProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(property), propertylength, ::core::mem::transmute(propertydata), datalength, ::core::mem::transmute(bytesreturned)).ok()
    }
    pub unsafe fn KsMethod(&self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KsMethod)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(method), methodlength, ::core::mem::transmute(methoddata), datalength, ::core::mem::transmute(bytesreturned)).ok()
    }
    pub unsafe fn KsEvent(&self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KsEvent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(event), eventlength, ::core::mem::transmute(eventdata), datalength, ::core::mem::transmute(bytesreturned)).ok()
    }
}
impl ::core::convert::From<IKsControl> for ::windows::core::IUnknown {
    fn from(value: IKsControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsControl> for ::windows::core::IUnknown {
    fn from(value: &IKsControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsControl {}
impl ::core::fmt::Debug for IKsControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsControl {
    type Vtable = IKsControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28f54685_06fd_11d2_b27a_00a0c9223196);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub KsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT,
    pub KsMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT,
    pub KsEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsFormatSupport(::windows::core::IUnknown);
impl IKsFormatSupport {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFormatSupported(&self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsFormatSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pksformat), cbformat, ::core::mem::transmute(pbsupported)).ok()
    }
    pub unsafe fn GetDevicePreferredFormat(&self) -> ::windows::core::Result<*mut KSDATAFORMAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDevicePreferredFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut KSDATAFORMAT>(result__)
    }
}
impl ::core::convert::From<IKsFormatSupport> for ::windows::core::IUnknown {
    fn from(value: IKsFormatSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsFormatSupport> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsFormatSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsFormatSupport> for ::windows::core::IUnknown {
    fn from(value: &IKsFormatSupport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsFormatSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsFormatSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsFormatSupport {}
impl ::core::fmt::Debug for IKsFormatSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsFormatSupport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsFormatSupport {
    type Vtable = IKsFormatSupport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cb4a69d_bb6f_4d2b_95b7_452d2c155db5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsFormatSupport_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFormatSupported: usize,
    pub GetDevicePreferredFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsJackContainerId(::windows::core::IUnknown);
impl IKsJackContainerId {
    pub unsafe fn GetJackContainerId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJackContainerId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IKsJackContainerId> for ::windows::core::IUnknown {
    fn from(value: IKsJackContainerId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsJackContainerId> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsJackContainerId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsJackContainerId> for ::windows::core::IUnknown {
    fn from(value: &IKsJackContainerId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsJackContainerId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsJackContainerId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackContainerId {}
impl ::core::fmt::Debug for IKsJackContainerId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackContainerId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsJackContainerId {
    type Vtable = IKsJackContainerId_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99af463_d629_4ec4_8c00_e54d68154248);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackContainerId_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetJackContainerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjackcontainerid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsJackDescription(::windows::core::IUnknown);
impl IKsJackDescription {
    pub unsafe fn GetJackCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJackCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetJackDescription(&self, njack: u32) -> ::windows::core::Result<KSJACK_DESCRIPTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJackDescription)(::windows::core::Interface::as_raw(self), njack, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<KSJACK_DESCRIPTION>(result__)
    }
}
impl ::core::convert::From<IKsJackDescription> for ::windows::core::IUnknown {
    fn from(value: IKsJackDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsJackDescription> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsJackDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsJackDescription> for ::windows::core::IUnknown {
    fn from(value: &IKsJackDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsJackDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsJackDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackDescription {}
impl ::core::fmt::Debug for IKsJackDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsJackDescription {
    type Vtable = IKsJackDescription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4509f757_2d46_4637_8e62_ce7db944f57b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetJackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetJackDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetJackDescription: usize,
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsJackDescription2(::windows::core::IUnknown);
impl IKsJackDescription2 {
    pub unsafe fn GetJackCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJackCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetJackDescription2(&self, njack: u32) -> ::windows::core::Result<KSJACK_DESCRIPTION2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJackDescription2)(::windows::core::Interface::as_raw(self), njack, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<KSJACK_DESCRIPTION2>(result__)
    }
}
impl ::core::convert::From<IKsJackDescription2> for ::windows::core::IUnknown {
    fn from(value: IKsJackDescription2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsJackDescription2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsJackDescription2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsJackDescription2> for ::windows::core::IUnknown {
    fn from(value: &IKsJackDescription2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsJackDescription2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsJackDescription2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackDescription2 {}
impl ::core::fmt::Debug for IKsJackDescription2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackDescription2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsJackDescription2 {
    type Vtable = IKsJackDescription2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x478f3a9b_e0c9_4827_9228_6f5505ffe76a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetJackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT,
    pub GetJackDescription2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsJackSinkInformation(::windows::core::IUnknown);
impl IKsJackSinkInformation {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetJackSinkInformation(&self) -> ::windows::core::Result<KSJACK_SINK_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJackSinkInformation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<KSJACK_SINK_INFORMATION>(result__)
    }
}
impl ::core::convert::From<IKsJackSinkInformation> for ::windows::core::IUnknown {
    fn from(value: IKsJackSinkInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsJackSinkInformation> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsJackSinkInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsJackSinkInformation> for ::windows::core::IUnknown {
    fn from(value: &IKsJackSinkInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsJackSinkInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsJackSinkInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackSinkInformation {}
impl ::core::fmt::Debug for IKsJackSinkInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackSinkInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsJackSinkInformation {
    type Vtable = IKsJackSinkInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9bd72ed_290f_4581_9ff3_61027a8fe532);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackSinkInformation_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetJackSinkInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetJackSinkInformation: usize,
}
#[repr(C)]
pub struct IKsPin(pub u8);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsPropertySet(::windows::core::IUnknown);
impl IKsPropertySet {
    pub unsafe fn Set(&self, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(propset), id, ::core::mem::transmute(instancedata), instancelength, ::core::mem::transmute(propertydata), datalength).ok()
    }
    pub unsafe fn Get(&self, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Get)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(propset), id, ::core::mem::transmute(instancedata), instancelength, ::core::mem::transmute(propertydata), datalength, ::core::mem::transmute(bytesreturned)).ok()
    }
    pub unsafe fn QuerySupported(&self, propset: *const ::windows::core::GUID, id: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QuerySupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(propset), id, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IKsPropertySet> for ::windows::core::IUnknown {
    fn from(value: IKsPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsPropertySet> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsPropertySet> for ::windows::core::IUnknown {
    fn from(value: &IKsPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsPropertySet {}
impl ::core::fmt::Debug for IKsPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsPropertySet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsPropertySet {
    type Vtable = IKsPropertySet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31efac30_515c_11d0_a9aa_00aa0061be93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsPropertySet_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::HRESULT,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT,
    pub QuerySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, typesupport: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
pub struct IKsTopology(::windows::core::IUnknown);
impl IKsTopology {
    pub unsafe fn CreateNodeInstance<'a, P0>(&self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: P0, interfaceid: *const ::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).CreateNodeInstance)(::windows::core::Interface::as_raw(self), nodeid, flags, desiredaccess, unkouter.into().abi(), ::core::mem::transmute(interfaceid), ::core::mem::transmute(interface)).ok()
    }
}
impl ::core::convert::From<IKsTopology> for ::windows::core::IUnknown {
    fn from(value: IKsTopology) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKsTopology> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKsTopology) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKsTopology> for ::windows::core::IUnknown {
    fn from(value: &IKsTopology) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKsTopology {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKsTopology {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsTopology {}
impl ::core::fmt::Debug for IKsTopology {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsTopology").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsTopology {
    type Vtable = IKsTopology_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28f54683_06fd_11d2_b27a_00a0c9223196);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsTopology_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateNodeInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
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
impl ::core::fmt::Debug for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERLEAVED_AUDIO_FORMAT_INFORMATION").field("Size", &self.Size).field("PrimaryChannelCount", &self.PrimaryChannelCount).field("PrimaryChannelStartPosition", &self.PrimaryChannelStartPosition).field("PrimaryChannelMask", &self.PrimaryChannelMask).field("InterleavedChannelCount", &self.InterleavedChannelCount).field("InterleavedChannelStartPosition", &self.InterleavedChannelStartPosition).field("InterleavedChannelMask", &self.InterleavedChannelMask).finish()
    }
}
unsafe impl ::windows::core::Abi for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERLEAVED_AUDIO_FORMAT_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for INTERLEAVED_AUDIO_FORMAT_INFORMATION {}
impl ::core::default::Default for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ALTERNATE_AUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_ALTERNATE_AUDIO").field("fStereo", &self.fStereo).field("DualMode", &self.DualMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAC3_ALTERNATE_AUDIO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ALTERNATE_AUDIO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAC3_ALTERNATE_AUDIO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ALTERNATE_AUDIO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ALTERNATE_AUDIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAC3_BIT_STREAM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_BIT_STREAM_MODE").field("BitStreamMode", &self.BitStreamMode).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAC3_BIT_STREAM_MODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAC3_BIT_STREAM_MODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAC3_BIT_STREAM_MODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAC3_BIT_STREAM_MODE {}
impl ::core::default::Default for KSAC3_BIT_STREAM_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAC3_DIALOGUE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_DIALOGUE_LEVEL").field("DialogueLevel", &self.DialogueLevel).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAC3_DIALOGUE_LEVEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAC3_DIALOGUE_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAC3_DIALOGUE_LEVEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAC3_DIALOGUE_LEVEL {}
impl ::core::default::Default for KSAC3_DIALOGUE_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_DOWNMIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_DOWNMIX").field("fDownMix", &self.fDownMix).field("fDolbySurround", &self.fDolbySurround).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAC3_DOWNMIX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_DOWNMIX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAC3_DOWNMIX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_DOWNMIX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_DOWNMIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ERROR_CONCEALMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_ERROR_CONCEALMENT").field("fRepeatPreviousBlock", &self.fRepeatPreviousBlock).field("fErrorInCurrentBlock", &self.fErrorInCurrentBlock).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAC3_ERROR_CONCEALMENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ERROR_CONCEALMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAC3_ERROR_CONCEALMENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ERROR_CONCEALMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ERROR_CONCEALMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ROOM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_ROOM_TYPE").field("fLargeRoom", &self.fLargeRoom).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAC3_ROOM_TYPE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ROOM_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAC3_ROOM_TYPE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ROOM_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ROOM_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
pub const KSALGORITHMINSTANCE_SYSTEM_ACOUSTIC_ECHO_CANCEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c22c56d_9879_4f5b_a389_27996ddc2810);
pub const KSALGORITHMINSTANCE_SYSTEM_AGC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x950e55b9_877c_4c67_be08_e47b5611130a);
pub const KSALGORITHMINSTANCE_SYSTEM_MICROPHONE_ARRAY_PROCESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6f5a0a0_9e61_4f8c_91e3_76cf0f3c471f);
pub const KSALGORITHMINSTANCE_SYSTEM_NOISE_SUPPRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ab0882e_7274_4516_877d_4eee99ba4fd0);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSALLOCATORMODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KsAllocatorMode_User: KSALLOCATORMODE = KSALLOCATORMODE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KsAllocatorMode_Kernel: KSALLOCATORMODE = KSALLOCATORMODE(1i32);
impl ::core::marker::Copy for KSALLOCATORMODE {}
impl ::core::clone::Clone for KSALLOCATORMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSALLOCATORMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSALLOCATORMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSALLOCATORMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSALLOCATORMODE").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSALLOCATOR_FRAMING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSALLOCATOR_FRAMING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSALLOCATOR_FRAMING>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSALLOCATOR_FRAMING {}
impl ::core::default::Default for KSALLOCATOR_FRAMING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSALLOCATOR_FRAMING_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSALLOCATOR_FRAMING_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSALLOCATOR_FRAMING_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSALLOCATOR_FRAMING_0 {}
impl ::core::default::Default for KSALLOCATOR_FRAMING_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSALLOCATOR_FRAMING_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSALLOCATOR_FRAMING_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSALLOCATOR_FRAMING_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSALLOCATOR_FRAMING_1 {}
impl ::core::default::Default for KSALLOCATOR_FRAMING_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSALLOCATOR_FRAMING_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSALLOCATOR_FRAMING_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSALLOCATOR_FRAMING_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSALLOCATOR_FRAMING_EX {}
impl ::core::default::Default for KSALLOCATOR_FRAMING_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub Attribute: ::windows::core::GUID,
}
impl ::core::marker::Copy for KSATTRIBUTE {}
impl ::core::clone::Clone for KSATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSATTRIBUTE").field("Size", &self.Size).field("Flags", &self.Flags).field("Attribute", &self.Attribute).finish()
    }
}
unsafe impl ::windows::core::Abi for KSATTRIBUTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSATTRIBUTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSATTRIBUTE {}
impl ::core::default::Default for KSATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSATTRIBUTEID_AUDIOSIGNALPROCESSING_MODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1f89eb5_5f46_419b_967b_ff6770b98401);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    pub AttributeHeader: KSATTRIBUTE,
    pub SignalProcessingMode: ::windows::core::GUID,
}
impl ::core::marker::Copy for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {}
impl ::core::clone::Clone for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE").field("AttributeHeader", &self.AttributeHeader).field("SignalProcessingMode", &self.SignalProcessingMode).finish()
    }
}
unsafe impl ::windows::core::Abi for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {}
impl ::core::default::Default for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
pub const KSAUDFNAME_3D_CENTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f0670b4_991f_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_3D_DEPTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63ff5747_991f_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_3D_STEREO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede2_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_ALTERNATE_MICROPHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc31d6b_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_AUX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedfe_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_AUX_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedfd_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_AUX_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedfc_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_BASS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede0_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedfb_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_IN_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf3_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedea_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede9_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_IN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf9_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_IN_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf4_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedec_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedeb_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MASTER_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede4_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MASTER_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede3_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MICROPHONE_BOOST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc31d6a_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MIC_IN_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf5_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIC_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedee_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIC_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185feded_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf8_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_IN_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf2_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede8_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede7_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDRANGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2cbe478_ae84_49a1_8b72_4ad09b78ed34);
pub const KSAUDFNAME_MONO_MIX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00dff078_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_MIX_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc31d69_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_MIX_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22b0eafe_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9b41dc3_96e2_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ad247ec_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ad247eb_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_PC_SPEAKER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedff_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PC_SPEAKER_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf1_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PC_SPEAKER_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf0_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PEAKMETER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57e24340_fc5b_4612_a562_72b11a29dfae);
pub const KSAUDFNAME_RECORDING_CONTROL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedfa_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_RECORDING_SOURCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedef_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_STEREO_MIX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00dff077_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_STEREO_MIX_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22b0eafd_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_STEREO_MIX_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ad247ed_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_TREBLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede1_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x915daec4_a434_11d2_ac52_00c04f8efb68);
pub const KSAUDFNAME_VIDEO_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b46e709_992a_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_VIDEO_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b46e708_992a_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_VOLUME_CONTROL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf7_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_IN_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fedf6_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede6_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_OUT_MIX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fee00_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185fede5_9905_11d1_95a9_00c04fb925d3);
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
impl ::core::fmt::Debug for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOENGINE_BUFFER_SIZE_RANGE").field("MinBufferBytes", &self.MinBufferBytes).field("MaxBufferBytes", &self.MaxBufferBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIOENGINE_BUFFER_SIZE_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_BUFFER_SIZE_RANGE {}
impl ::core::default::Default for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIOENGINE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOENGINE_DESCRIPTOR").field("nHostPinId", &self.nHostPinId).field("nOffloadPinId", &self.nOffloadPinId).field("nLoopbackPinId", &self.nLoopbackPinId).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIOENGINE_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIOENGINE_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_DESCRIPTOR {}
impl ::core::default::Default for KSAUDIOENGINE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIOENGINE_VOLUMELEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOENGINE_VOLUMELEVEL").field("TargetVolume", &self.TargetVolume).field("CurveType", &self.CurveType).field("CurveDuration", &self.CurveDuration).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIOENGINE_VOLUMELEVEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_VOLUMELEVEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIOENGINE_VOLUMELEVEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_VOLUMELEVEL {}
impl ::core::default::Default for KSAUDIOENGINE_VOLUMELEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOMODULE_DESCRIPTOR {
    pub ClassId: ::windows::core::GUID,
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
impl ::core::fmt::Debug for KSAUDIOMODULE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOMODULE_DESCRIPTOR").field("ClassId", &self.ClassId).field("InstanceId", &self.InstanceId).field("VersionMajor", &self.VersionMajor).field("VersionMinor", &self.VersionMinor).field("Name", &self.Name).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIOMODULE_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIOMODULE_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_DESCRIPTOR {}
impl ::core::default::Default for KSAUDIOMODULE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSAUDIOMODULE_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIOMODULE_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_NOTIFICATION {}
impl ::core::default::Default for KSAUDIOMODULE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSAUDIOMODULE_NOTIFICATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_NOTIFICATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIOMODULE_NOTIFICATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_NOTIFICATION_0 {}
impl ::core::default::Default for KSAUDIOMODULE_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOMODULE_NOTIFICATION_0_0 {
    pub DeviceId: ::windows::core::GUID,
    pub ClassId: ::windows::core::GUID,
    pub InstanceId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSAUDIOMODULE_NOTIFICATION_0_0 {}
impl ::core::clone::Clone for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOMODULE_NOTIFICATION_0_0").field("DeviceId", &self.DeviceId).field("ClassId", &self.ClassId).field("InstanceId", &self.InstanceId).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIOMODULE_NOTIFICATION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIOMODULE_NOTIFICATION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_NOTIFICATION_0_0 {}
impl ::core::default::Default for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSAUDIOMODULE_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub ClassId: ::windows::core::GUID,
    pub InstanceId: u32,
}
impl ::core::marker::Copy for KSAUDIOMODULE_PROPERTY {}
impl ::core::clone::Clone for KSAUDIOMODULE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KSAUDIOMODULE_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIOMODULE_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_PROPERTY {}
impl ::core::default::Default for KSAUDIOMODULE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_CHANNEL_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_CHANNEL_CONFIG").field("ActiveSpeakerPositions", &self.ActiveSpeakerPositions).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIO_CHANNEL_CONFIG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_CHANNEL_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_CHANNEL_CONFIG>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_CHANNEL_CONFIG {}
impl ::core::default::Default for KSAUDIO_CHANNEL_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAUDIO_COPY_PROTECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_COPY_PROTECTION").field("fCopyrighted", &self.fCopyrighted).field("fOriginal", &self.fOriginal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAUDIO_COPY_PROTECTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_COPY_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_COPY_PROTECTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_COPY_PROTECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_COPY_PROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_DYNAMIC_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_DYNAMIC_RANGE").field("QuietCompression", &self.QuietCompression).field("LoudCompression", &self.LoudCompression).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIO_DYNAMIC_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_DYNAMIC_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_DYNAMIC_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_DYNAMIC_RANGE {}
impl ::core::default::Default for KSAUDIO_DYNAMIC_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_MICROPHONE_COORDINATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_MICROPHONE_COORDINATES").field("usType", &self.usType).field("wXCoord", &self.wXCoord).field("wYCoord", &self.wYCoord).field("wZCoord", &self.wZCoord).field("wVerticalAngle", &self.wVerticalAngle).field("wHorizontalAngle", &self.wHorizontalAngle).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIO_MICROPHONE_COORDINATES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_MICROPHONE_COORDINATES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_MICROPHONE_COORDINATES>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_MICROPHONE_COORDINATES {}
impl ::core::default::Default for KSAUDIO_MICROPHONE_COORDINATES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_MIC_ARRAY_GEOMETRY")
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
unsafe impl ::windows::core::Abi for KSAUDIO_MIC_ARRAY_GEOMETRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_MIC_ARRAY_GEOMETRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_MIC_ARRAY_GEOMETRY {}
impl ::core::default::Default for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAUDIO_MIXCAP_TABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIXCAP_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_MIXCAP_TABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIXCAP_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIXCAP_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAUDIO_MIXLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_MIXLEVEL").field("Mute", &self.Mute).field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAUDIO_MIXLEVEL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIXLEVEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_MIXLEVEL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIXLEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIXLEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAUDIO_MIX_CAPS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIX_CAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_MIX_CAPS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIX_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIX_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSAUDIO_MIX_CAPS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIX_CAPS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_MIX_CAPS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIX_CAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIX_CAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_PACKETSIZE_CONSTRAINTS").field("MinPacketPeriodInHns", &self.MinPacketPeriodInHns).field("PacketSizeFileAlignment", &self.PacketSizeFileAlignment).field("Reserved", &self.Reserved).field("NumProcessingModeConstraints", &self.NumProcessingModeConstraints).field("ProcessingModeConstraints", &self.ProcessingModeConstraints).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_PACKETSIZE_CONSTRAINTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_PACKETSIZE_CONSTRAINTS {}
impl ::core::default::Default for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_PACKETSIZE_CONSTRAINTS2").field("MinPacketPeriodInHns", &self.MinPacketPeriodInHns).field("PacketSizeFileAlignment", &self.PacketSizeFileAlignment).field("MaxPacketSizeInBytes", &self.MaxPacketSizeInBytes).field("NumProcessingModeConstraints", &self.NumProcessingModeConstraints).field("ProcessingModeConstraints", &self.ProcessingModeConstraints).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_PACKETSIZE_CONSTRAINTS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {}
impl ::core::default::Default for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_POSITION").field("PlayOffset", &self.PlayOffset).field("WriteOffset", &self.WriteOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIO_POSITION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_POSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_POSITION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_POSITION {}
impl ::core::default::Default for KSAUDIO_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_POSITIONEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_POSITIONEX").field("TimerFrequency", &self.TimerFrequency).field("TimeStamp1", &self.TimeStamp1).field("Position", &self.Position).field("TimeStamp2", &self.TimeStamp2).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIO_POSITIONEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_POSITIONEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_POSITIONEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_POSITIONEX {}
impl ::core::default::Default for KSAUDIO_POSITIONEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSAUDIO_PRESENTATION_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_PRESENTATION_POSITION").field("u64PositionInBlocks", &self.u64PositionInBlocks).field("u64QPCPosition", &self.u64QPCPosition).finish()
    }
}
unsafe impl ::windows::core::Abi for KSAUDIO_PRESENTATION_POSITION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSAUDIO_PRESENTATION_POSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSAUDIO_PRESENTATION_POSITION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSAUDIO_PRESENTATION_POSITION {}
impl ::core::default::Default for KSAUDIO_PRESENTATION_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
pub const KSCAMERAPROFILE_BalancedVideoAndPhoto: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b52b017_42c7_4a21_bfe3_23f009149887);
pub const KSCAMERAPROFILE_CompressedCamera: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e34cdc1_27ad_437f_abde_02b629f37b44);
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
pub const KSCAMERAPROFILE_FaceAuth_Mode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81361b22_700b_4546_a2d4_c52e907bfc27);
pub const KSCAMERAPROFILE_HDRWithWCGPhoto: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bf6f1ff_b555_4625_b326_a46def318fb7);
pub const KSCAMERAPROFILE_HDRWithWCGVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b27c336_4924_4989_b994_fdaf1dc7cd85);
pub const KSCAMERAPROFILE_HighFrameRate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x566e6113_8c35_48e7_b89f_d23fdc1219dc);
pub const KSCAMERAPROFILE_HighQualityPhoto: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32440725_961b_4ca3_b5b2_854e719d9e1b);
pub const KSCAMERAPROFILE_Legacy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4894d81_62b7_4eec_8740_80658c4a9d3e);
pub const KSCAMERAPROFILE_PhotoSequence: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02399d9d_4ee8_49ba_bc07_5ff156531413);
pub const KSCAMERAPROFILE_VariablePhotoSequence: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ff2cb56_e75a_49b1_a928_9985d5946f87);
pub const KSCAMERAPROFILE_VideoConferencing: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5444a88_e1bf_4597_b2dd_9e1ead864bb8);
pub const KSCAMERAPROFILE_VideoHDR8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4f3f4ec_bdff_4314_b1d4_008e281f74e7);
pub const KSCAMERAPROFILE_VideoRecording: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e517e8_8f8c_4f6f_9a57_46fc2f647ec0);
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
    pub SubType: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS").field("Resolution", &self.Resolution).field("MaxFrameRate", &self.MaxFrameRate).field("MaskResolution", &self.MaskResolution).field("SubType", &self.SubType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_CAMERAOFFSET").field("PitchAngle", &self.PitchAngle).field("YawAngle", &self.YawAngle).field("Flag", &self.Flag).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_CAMERAOFFSET>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS")
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
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING").field("OriginX", &self.OriginX).field("OriginY", &self.OriginY).field("WindowSize", &self.WindowSize).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_EVCOMPENSATION").field("Mode", &self.Mode).field("Min", &self.Min).field("Max", &self.Max).field("Value", &self.Value).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_EVCOMPENSATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_FIELDOFVIEW").field("NormalizedFocalLengthX", &self.NormalizedFocalLengthX).field("NormalizedFocalLengthY", &self.NormalizedFocalLengthY).field("Flag", &self.Flag).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_FIELDOFVIEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSCAMERA_EXTENDEDPROP_FOCUSSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_UNINITIALIZED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_LOST: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_SEARCHING: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FOCUSED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FAILED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(4i32);
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_FOCUSSTATE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_HEADER").field("Version", &self.Version).field("PinId", &self.PinId).field("Size", &self.Size).field("Result", &self.Result).field("Flags", &self.Flags).field("Capability", &self.Capability).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_HEADER {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_METADATAINFO").field("BufferAlignment", &self.BufferAlignment).field("MaxMetadataBufferSize", &self.MaxMetadataBufferSize).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_METADATAINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_METADATAINFO {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_ALIGNMENTREQUIRED: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_MEMORYTYPE_MASK: u64 = 255u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_SYSTEMMEMORY: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSCAMERA_EXTENDEDPROP_MetadataAlignment(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_16: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_32: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_64: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_128: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_256: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_512: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_1024: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_2048: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_4096: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_8192: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(13i32);
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_MetadataAlignment {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_MetadataAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_MetadataAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_MetadataAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_MetadataAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_MetadataAlignment").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_PHOTOMODE").field("RequestedHistoryFrames", &self.RequestedHistoryFrames).field("MaxHistoryFrames", &self.MaxHistoryFrames).field("SubMode", &self.SubMode).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_PHOTOMODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_PHOTOMODE {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub ProfileId: ::windows::core::GUID,
    pub Index: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_PROFILE {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_PROFILE").field("ProfileId", &self.ProfileId).field("Index", &self.Index).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_PROFILE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_PROFILE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_PROFILE {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_DYNAMIC: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_OFF: u64 = 0u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_ON: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSCAMERA_EXTENDEDPROP_ROITYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_UNKNOWN: KSCAMERA_EXTENDEDPROP_ROITYPE = KSCAMERA_EXTENDEDPROP_ROITYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_FACE: KSCAMERA_EXTENDEDPROP_ROITYPE = KSCAMERA_EXTENDEDPROP_ROITYPE(1i32);
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_ROITYPE {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_ROITYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROITYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROITYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROITYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_ROITYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS").field("ControlId", &self.ControlId).field("MaxNumberOfROIs", &self.MaxNumberOfROIs).field("Capability", &self.Capability).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER").field("Size", &self.Size).field("ConfigCapCount", &self.ConfigCapCount).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_FOCUS").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_ROI_FOCUS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_INFO").field("Region", &self.Region).field("Flags", &self.Flags).field("Weight", &self.Weight).field("RegionOfInterestType", &self.RegionOfInterestType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_ROI_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL").field("ControlId", &self.ControlId).field("ROICount", &self.ROICount).field("Result", &self.Result).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER").field("Size", &self.Size).field("ControlCount", &self.ControlCount).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_VALUE {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_VALUE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_VALUE_0 {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSCAMERA_EXTENDEDPROP_WBPRESET(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CLOUDY: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_DAYLIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLASH: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLUORESCENT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_TUNGSTEN: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CANDLELIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(6i32);
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_WBPRESET {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_WBPRESET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_WBPRESET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_WBPRESET {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_WBPRESET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_WBPRESET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_TEMPERATURE: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_PRESET: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(2i32);
impl ::core::marker::Copy for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {}
impl ::core::clone::Clone for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_MAXVIDEOFPS_FORPHOTORES").field("PhotoResWidth", &self.PhotoResWidth).field("PhotoResHeight", &self.PhotoResHeight).field("PreviewFPSNum", &self.PreviewFPSNum).field("PreviewFPSDenom", &self.PreviewFPSDenom).field("CaptureFPSNum", &self.CaptureFPSNum).field("CaptureFPSDenom", &self.CaptureFPSDenom).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_MAXVIDEOFPS_FORPHOTORES>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {}
impl ::core::default::Default for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK").field("Header", &self.Header).field("MaskCoverageBoundingBox", &self.MaskCoverageBoundingBox).field("MaskResolution", &self.MaskResolution).field("ForegroundBoundingBox", &self.ForegroundBoundingBox).field("MaskData", &self.MaskData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_METADATA_CAPTURESTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_CAPTURESTATS")
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
unsafe impl ::windows::core::Abi for KSCAMERA_METADATA_CAPTURESTATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_CAPTURESTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_METADATA_CAPTURESTATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_CAPTURESTATS {}
impl ::core::default::Default for KSCAMERA_METADATA_CAPTURESTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_METADATA_DIGITALWINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_DIGITALWINDOW").field("Header", &self.Header).field("Window", &self.Window).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_METADATA_DIGITALWINDOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_DIGITALWINDOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_METADATA_DIGITALWINDOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_DIGITALWINDOW {}
impl ::core::default::Default for KSCAMERA_METADATA_DIGITALWINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_FRAMEILLUMINATION").field("Header", &self.Header).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_METADATA_FRAMEILLUMINATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_METADATA_FRAMEILLUMINATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_FRAMEILLUMINATION {}
impl ::core::default::Default for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_METADATA_ITEMHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_ITEMHEADER").field("MetadataId", &self.MetadataId).field("Size", &self.Size).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_METADATA_ITEMHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_ITEMHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_METADATA_ITEMHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_ITEMHEADER {}
impl ::core::default::Default for KSCAMERA_METADATA_ITEMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_PHOTOCONFIRMATION").field("Header", &self.Header).field("PhotoConfirmationIndex", &self.PhotoConfirmationIndex).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_METADATA_PHOTOCONFIRMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_PHOTOCONFIRMATION {}
impl ::core::default::Default for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSCAMERA_MetadataId(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_Standard_Start: KSCAMERA_MetadataId = KSCAMERA_MetadataId(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_PhotoConfirmation: KSCAMERA_MetadataId = KSCAMERA_MetadataId(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_UsbVideoHeader: KSCAMERA_MetadataId = KSCAMERA_MetadataId(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_CaptureStats: KSCAMERA_MetadataId = KSCAMERA_MetadataId(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_CameraExtrinsics: KSCAMERA_MetadataId = KSCAMERA_MetadataId(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_CameraIntrinsics: KSCAMERA_MetadataId = KSCAMERA_MetadataId(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_FrameIllumination: KSCAMERA_MetadataId = KSCAMERA_MetadataId(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_DigitalWindow: KSCAMERA_MetadataId = KSCAMERA_MetadataId(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_BackgroundSegmentationMask: KSCAMERA_MetadataId = KSCAMERA_MetadataId(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_Standard_End: KSCAMERA_MetadataId = KSCAMERA_MetadataId(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const MetadataId_Custom_Start: KSCAMERA_MetadataId = KSCAMERA_MetadataId(-2147483648i32);
impl ::core::marker::Copy for KSCAMERA_MetadataId {}
impl ::core::clone::Clone for KSCAMERA_MetadataId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSCAMERA_MetadataId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_MetadataId {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSCAMERA_MetadataId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_MetadataId").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_CAP_HEADER").field("Size", &self.Size).field("ItemCount", &self.ItemCount).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PERFRAMESETTING_CAP_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CAP_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER").field("Size", &self.Size).field("Type", &self.Type).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    pub Size: u32,
    pub Reserved: u32,
    pub Id: ::windows::core::GUID,
}
impl ::core::marker::Copy for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {}
impl ::core::clone::Clone for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM").field("Size", &self.Size).field("Reserved", &self.Reserved).field("Id", &self.Id).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_FRAME_HEADER").field("Size", &self.Size).field("Id", &self.Id).field("ItemCount", &self.ItemCount).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PERFRAMESETTING_FRAME_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PERFRAMESETTING_HEADER {
    pub Size: u32,
    pub FrameCount: u32,
    pub Id: ::windows::core::GUID,
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
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_HEADER").field("Size", &self.Size).field("FrameCount", &self.FrameCount).field("Id", &self.Id).field("Flags", &self.Flags).field("LoopCount", &self.LoopCount).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PERFRAMESETTING_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PERFRAMESETTING_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_ITEM_HEADER").field("Size", &self.Size).field("Type", &self.Type).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PERFRAMESETTING_ITEM_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSCAMERA_PERFRAMESETTING_ITEM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_TIME: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_FLASH: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_COMPENSATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_ISO: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_FOCUS: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_PHOTOCONFIRMATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_ITEM_CUSTOM: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(7i32);
impl ::core::marker::Copy for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {}
impl ::core::clone::Clone for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_PERFRAMESETTING_ITEM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSCAMERA_PERFRAMESETTING_MANUAL: u64 = 8589934592u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_CONCURRENCYINFO {
    pub ReferenceGuid: ::windows::core::GUID,
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
impl ::core::fmt::Debug for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_CONCURRENCYINFO").field("ReferenceGuid", &self.ReferenceGuid).field("Reserved", &self.Reserved).field("ProfileCount", &self.ProfileCount).field("Profiles", &self.Profiles).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PROFILE_CONCURRENCYINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PROFILE_CONCURRENCYINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_CONCURRENCYINFO {}
impl ::core::default::Default for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_INFO {
    pub ProfileId: ::windows::core::GUID,
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
impl ::core::fmt::Debug for KSCAMERA_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_INFO").field("ProfileId", &self.ProfileId).field("Index", &self.Index).field("PinCount", &self.PinCount).field("Pins", &self.Pins).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PROFILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PROFILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_INFO {}
impl ::core::default::Default for KSCAMERA_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_MEDIAINFO").field("Resolution", &self.Resolution).field("MaxFrameRate", &self.MaxFrameRate).field("Flags", &self.Flags).field("Data0", &self.Data0).field("Data1", &self.Data1).field("Data2", &self.Data2).field("Data3", &self.Data3).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PROFILE_MEDIAINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PROFILE_MEDIAINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO {}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_MEDIAINFO_0").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PROFILE_MEDIAINFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PROFILE_MEDIAINFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO_0 {}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_MEDIAINFO_1").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PROFILE_MEDIAINFO_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PROFILE_MEDIAINFO_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO_1 {}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCAMERA_PROFILE_PININFO {
    pub PinCategory: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for KSCAMERA_PROFILE_PININFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_PININFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PROFILE_PININFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_PININFO {}
impl ::core::default::Default for KSCAMERA_PROFILE_PININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSCAMERA_PROFILE_PININFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_PININFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PROFILE_PININFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_PININFO_0 {}
impl ::core::default::Default for KSCAMERA_PROFILE_PININFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSCAMERA_PROFILE_PININFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_PININFO_0_0").field("PinIndex", &self.PinIndex).field("ProfileSensorType", &self.ProfileSensorType).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCAMERA_PROFILE_PININFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_PININFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCAMERA_PROFILE_PININFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_PININFO_0_0 {}
impl ::core::default::Default for KSCAMERA_PROFILE_PININFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const KSCATEGORY_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6994ad04_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_BRIDGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x085aff00_62ce_11cf_a5d6_28db04c10000);
pub const KSCATEGORY_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65e8773d_8f56_11d0_a3b9_00a0c9223196);
pub const KSCATEGORY_CLOCK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53172480_4791_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_COMMUNICATIONSTRANSFORM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf1dda2c_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_CROSSBAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa799a801_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_DATACOMPRESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e84c900_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_DATADECOMPRESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2721ae20_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_DATATRANSFORM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2eb07ea0_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19689bf6_c384_48fd_ad51_90e58c79f70b);
pub const KSCATEGORY_ESCALANTE_PLATFORM_DRIVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74f3aea8_9768_11d1_8e07_00a0c95ec22e);
pub const KSCATEGORY_FILESYSTEM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x760fed5e_9357_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_INTERFACETRANSFORM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf1dda2d_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_MEDIUMTRANSFORM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf1dda2e_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_MICROPHONE_ARRAY_PROCESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x830a44f2_a32d_476b_be97_42845673b35a);
pub const KSCATEGORY_MIXER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad809c00_7b88_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_MULTIPLEXER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a5de1d3_01a1_452c_b481_4fa2b96271e8);
pub const KSCATEGORY_NETWORK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67c9cc3c_69c4_11d2_8759_00a0c9223196);
pub const KSCATEGORY_NETWORK_CAMERA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8238652_b500_41eb_b4f3_4234f7f5ae99);
pub const KSCATEGORY_PROXY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97ebaaca_95bd_11d0_a3ea_00a0c9223196);
pub const KSCATEGORY_QUALITY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97ebaacb_95bd_11d0_a3ea_00a0c9223196);
pub const KSCATEGORY_REALTIME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb115ffc_10c8_4964_831d_6dcb02e6f23f);
pub const KSCATEGORY_RENDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65e8773e_8f56_11d0_a3b9_00a0c9223196);
pub const KSCATEGORY_SENSOR_CAMERA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24e552d7_6523_47f7_a647_d3465bf1f5ca);
pub const KSCATEGORY_SENSOR_GROUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x669c7214_0a88_4311_a7f3_4e79820e33bd);
pub const KSCATEGORY_SPLITTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a4252a0_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_TEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6994ad06_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_TOPOLOGY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdda54a40_1e4c_11d1_a050_405705c10000);
pub const KSCATEGORY_TVAUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa799a802_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_TVTUNER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa799a800_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_VBICODEC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07dad660_22f1_11d1_a9f4_00c04fbbde8f);
pub const KSCATEGORY_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6994ad05_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_VIDEO_CAMERA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5323777_f976_4f5b_9b55_b94699c46e44);
pub const KSCATEGORY_VIRTUAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3503eac4_1f26_11d1_8ab0_00a0c9223196);
pub const KSCATEGORY_VPMUX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa799a803_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_WDMAUD_USE_PIN_NAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47a4fa20_a251_11d1_a050_0000f8004788);
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
impl ::core::fmt::Debug for KSCLOCK_CREATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCLOCK_CREATE").field("CreateFlags", &self.CreateFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCLOCK_CREATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCLOCK_CREATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCLOCK_CREATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCLOCK_CREATE {}
impl ::core::default::Default for KSCLOCK_CREATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSCOMPONENTID {
    pub Manufacturer: ::windows::core::GUID,
    pub Product: ::windows::core::GUID,
    pub Component: ::windows::core::GUID,
    pub Name: ::windows::core::GUID,
    pub Version: u32,
    pub Revision: u32,
}
impl ::core::marker::Copy for KSCOMPONENTID {}
impl ::core::clone::Clone for KSCOMPONENTID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSCOMPONENTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCOMPONENTID").field("Manufacturer", &self.Manufacturer).field("Product", &self.Product).field("Component", &self.Component).field("Name", &self.Name).field("Version", &self.Version).field("Revision", &self.Revision).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCOMPONENTID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCOMPONENTID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCOMPONENTID>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCOMPONENTID {}
impl ::core::default::Default for KSCOMPONENTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSCOMPONENTID_USBAUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1275f0_26e9_4264_ba4d_39fff01d94aa);
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
impl ::core::fmt::Debug for KSCORRELATED_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCORRELATED_TIME").field("Time", &self.Time).field("SystemTime", &self.SystemTime).finish()
    }
}
unsafe impl ::windows::core::Abi for KSCORRELATED_TIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSCORRELATED_TIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSCORRELATED_TIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSCORRELATED_TIME {}
impl ::core::default::Default for KSCORRELATED_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSDATAFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDATAFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDATAFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDATAFORMAT {}
impl ::core::default::Default for KSDATAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSDATAFORMAT_0 {
    pub FormatSize: u32,
    pub Flags: u32,
    pub SampleSize: u32,
    pub Reserved: u32,
    pub MajorFormat: ::windows::core::GUID,
    pub SubFormat: ::windows::core::GUID,
    pub Specifier: ::windows::core::GUID,
}
impl ::core::marker::Copy for KSDATAFORMAT_0 {}
impl ::core::clone::Clone for KSDATAFORMAT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSDATAFORMAT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDATAFORMAT_0").field("FormatSize", &self.FormatSize).field("Flags", &self.Flags).field("SampleSize", &self.SampleSize).field("Reserved", &self.Reserved).field("MajorFormat", &self.MajorFormat).field("SubFormat", &self.SubFormat).field("Specifier", &self.Specifier).finish()
    }
}
unsafe impl ::windows::core::Abi for KSDATAFORMAT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDATAFORMAT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDATAFORMAT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDATAFORMAT_0 {}
impl ::core::default::Default for KSDATAFORMAT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDATAFORMAT_BIT_ATTRIBUTES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDATAFORMAT_BIT_TEMPORAL_COMPRESSION: u32 = 0u32;
pub const KSDATAFORMAT_SPECIFIER_AC3_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d80e4_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_ANALOGVIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0482dde0_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_AC3_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b35_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b32_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b31_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b34_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b33_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DSOUND: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x518590a2_a184_11d0_8522_00c04fd9baf3);
pub const KSDATAFORMAT_SPECIFIER_FILEHANDLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65e8773c_8f56_11d0_a3b9_00a0c9223196);
pub const KSDATAFORMAT_SPECIFIER_FILENAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa797b40_e974_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SPECIFIER_H264_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2017be05_6629_4248_aaed_7e1a47bc9b9c);
pub const KSDATAFORMAT_SPECIFIER_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x692fa379_d3e8_4651_b5b4_0b94b013eeaf);
pub const KSDATAFORMAT_SPECIFIER_JPEG_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x692fa379_d3e8_4651_b5b4_0b94b013eeaf);
pub const KSDATAFORMAT_SPECIFIER_LPCM_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d80e6_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_MPEG1_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f82_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SPECIFIER_MPEG2_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d80e5_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_MPEG2_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_NONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f6417d6_c318_11d0_a43f_00a0c9223196);
pub const KSDATAFORMAT_SPECIFIER_VBI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72a76e0_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SPECIFIER_VC_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad98d184_aac3_11d0_a41c_00a0c9223196);
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72a76a0_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SPECIFIER_WAVEFORMATEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SUBTYPE_AC3_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d802c_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_ANALOG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dba3190_67bd_11cf_a0f7_0020afd156e4);
pub const KSDATAFORMAT_SUBTYPE_CC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33214cc1_011f_11d2_b4b1_00a0d102cfbe);
pub const KSDATAFORMAT_SUBTYPE_D16: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000050_0004_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_DSS_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0af4f82_e163_11d0_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_DSS_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0af4f81_e163_11d0_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_DTS_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8033_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_AAC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000006_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ATRAC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000008_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000092_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000a_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS_ATMOS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010a_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT20: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000030c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MLP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000d_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000008_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000030b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS_HD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000003_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000004_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000005_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ONE_BIT_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000009_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_WMA_PRO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000164_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IMAGE_RGB32: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000016_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_JPEG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const KSDATAFORMAT_SUBTYPE_L16: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000051_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L16_CUSTOM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000051_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L16_IR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000051_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000032_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8_CUSTOM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000032_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8_IR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000032_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_LPCM_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8032_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_Line21_BytePair: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e8d4a22_310c_11d0_b79a_00aa003767a7);
pub const KSDATAFORMAT_SUBTYPE_Line21_GOPPacket: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e8d4a23_310c_11d0_b79a_00aa003767a7);
pub const KSDATAFORMAT_SUBTYPE_MIDI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d262760_e957_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SUBTYPE_MIDI_BUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ca15fa0_6cfe_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SUBTYPE_MJPG_CUSTOM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47504a4d_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MJPG_DEPTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47504a4d_0004_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MJPG_IR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47504a4d_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Packet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb80_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Payload: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb81_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb86_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG2_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d802b_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_MPEG2_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_MPEGLAYER3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MPEG_HEAAC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00001610_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_NABTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72a76e2_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SUBTYPE_NABTS_FEC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe757bca1_39ac_11d1_a9f5_00c04fbbde8f);
pub const KSDATAFORMAT_SUBTYPE_NONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb8e_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_OVERLAY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7f_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_PCM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_RAW8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca20d9a0_3e3e_11d1_9bf9_00c04fbbdebf);
pub const KSDATAFORMAT_SUBTYPE_RIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4995daee_9ee6_11d0_a40e_00a0c9223196);
pub const KSDATAFORMAT_SUBTYPE_RIFFMIDI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4995daf0_9ee6_11d0_a40e_00a0c9223196);
pub const KSDATAFORMAT_SUBTYPE_RIFFWAVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb8b_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_SDDS_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8034_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_AC3_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b25_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b22_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b21_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b24_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b23_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_SUBPICTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d802d_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_TELETEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72a76e3_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SUBTYPE_VPVBI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a9b6a41_1a22_11d1_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_VPVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a9b6a40_1a22_11d1_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_WAVEFORMATEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO_LOSSLESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_ANALOGAUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0482dee1_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_TYPE_ANALOGVIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0482dde1_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_TYPE_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_AUXLine21Data: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x670aea80_3a82_11d0_b79b_00aa003767a7);
pub const KSDATAFORMAT_TYPE_DVD_ENCRYPTED_PACK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed0b916a_044d_11d1_aa78_00c04fc31d60);
pub const KSDATAFORMAT_TYPE_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72178c23_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const KSDATAFORMAT_TYPE_MIDI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7364696d_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_MPEG2_PES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8020_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MPEG2_PROGRAM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8022_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MPEG2_TRANSPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8023_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MUSIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe725d360_62cc_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_TYPE_NABTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe757bca0_39ac_11d1_a9f5_00c04fbbde8f);
pub const KSDATAFORMAT_TYPE_STANDARD_ELEMENTARY_STREAM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b11_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_STANDARD_PACK_HEADER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b13_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_STANDARD_PES_PACKET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36523b12_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_STREAM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb83_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_TYPE_TEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73747874_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_VBI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72a76e1_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_TYPE_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
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
unsafe impl ::windows::core::Abi for KSDATARANGE_AUDIO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDATARANGE_AUDIO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDATARANGE_AUDIO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDATARANGE_AUDIO {}
impl ::core::default::Default for KSDATARANGE_AUDIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub Technology: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for KSDATARANGE_MUSIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDATARANGE_MUSIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDATARANGE_MUSIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDATARANGE_MUSIC {}
impl ::core::default::Default for KSDATARANGE_MUSIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSDEGRADESETID_Standard: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f564180_704c_11d0_a5d6_28db04c10000);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSDEGRADE_STANDARD(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEGRADE_STANDARD_SAMPLE: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEGRADE_STANDARD_QUALITY: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEGRADE_STANDARD_COMPUTATION: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEGRADE_STANDARD_SKIP: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(3i32);
impl ::core::marker::Copy for KSDEGRADE_STANDARD {}
impl ::core::clone::Clone for KSDEGRADE_STANDARD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSDEGRADE_STANDARD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSDEGRADE_STANDARD {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSDEGRADE_STANDARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDEGRADE_STANDARD").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSDEVICE_PROFILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDEVICE_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDEVICE_PROFILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDEVICE_PROFILE_INFO {}
impl ::core::default::Default for KSDEVICE_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSDEVICE_PROFILE_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDEVICE_PROFILE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDEVICE_PROFILE_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDEVICE_PROFILE_INFO_0 {}
impl ::core::default::Default for KSDEVICE_PROFILE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSDEVICE_PROFILE_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDEVICE_PROFILE_INFO_0_0").field("Info", &self.Info).field("Reserved", &self.Reserved).field("ConcurrencyCount", &self.ConcurrencyCount).field("Concurrency", &self.Concurrency).finish()
    }
}
unsafe impl ::windows::core::Abi for KSDEVICE_PROFILE_INFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDEVICE_PROFILE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDEVICE_PROFILE_INFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDEVICE_PROFILE_INFO_0_0 {}
impl ::core::default::Default for KSDEVICE_PROFILE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_PROFILE_TYPE_CAMERA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_PROFILE_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSDEVICE_THERMAL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_THERMAL_STATE_LOW: KSDEVICE_THERMAL_STATE = KSDEVICE_THERMAL_STATE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDEVICE_THERMAL_STATE_HIGH: KSDEVICE_THERMAL_STATE = KSDEVICE_THERMAL_STATE(1i32);
impl ::core::marker::Copy for KSDEVICE_THERMAL_STATE {}
impl ::core::clone::Clone for KSDEVICE_THERMAL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSDEVICE_THERMAL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSDEVICE_THERMAL_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSDEVICE_THERMAL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDEVICE_THERMAL_STATE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSDISPLAYCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDISPLAYCHANGE").field("PelsWidth", &self.PelsWidth).field("PelsHeight", &self.PelsHeight).field("BitsPerPel", &self.BitsPerPel).field("DeviceID", &self.DeviceID).finish()
    }
}
unsafe impl ::windows::core::Abi for KSDISPLAYCHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDISPLAYCHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDISPLAYCHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDISPLAYCHANGE {}
impl ::core::default::Default for KSDISPLAYCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSDS3D_BUFFER_ALL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDS3D_BUFFER_ALL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_BUFFER_ALL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDS3D_BUFFER_ALL {}
impl ::core::default::Default for KSDS3D_BUFFER_ALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSDS3D_BUFFER_CONE_ANGLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_BUFFER_CONE_ANGLES").field("InsideConeAngle", &self.InsideConeAngle).field("OutsideConeAngle", &self.OutsideConeAngle).finish()
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_BUFFER_CONE_ANGLES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDS3D_BUFFER_CONE_ANGLES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_BUFFER_CONE_ANGLES>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDS3D_BUFFER_CONE_ANGLES {}
impl ::core::default::Default for KSDS3D_BUFFER_CONE_ANGLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSDS3D_HRTF_COEFF_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FLOAT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const SHORT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDS3D_COEFF_COUNT: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(2i32);
impl ::core::marker::Copy for KSDS3D_HRTF_COEFF_FORMAT {}
impl ::core::clone::Clone for KSDS3D_HRTF_COEFF_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSDS3D_HRTF_COEFF_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_HRTF_COEFF_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSDS3D_HRTF_COEFF_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDS3D_HRTF_COEFF_FORMAT").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_HRTF_FILTER_FORMAT_MSG").field("FilterMethod", &self.FilterMethod).field("CoeffFormat", &self.CoeffFormat).field("Version", &self.Version).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_HRTF_FILTER_FORMAT_MSG>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDS3D_HRTF_FILTER_FORMAT_MSG {}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSDS3D_HRTF_FILTER_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const DIRECT_FORM: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const CASCADE_FORM: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDS3D_FILTER_METHOD_COUNT: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(2i32);
impl ::core::marker::Copy for KSDS3D_HRTF_FILTER_METHOD {}
impl ::core::clone::Clone for KSDS3D_HRTF_FILTER_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_HRTF_FILTER_METHOD {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDS3D_HRTF_FILTER_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSDS3D_HRTF_FILTER_QUALITY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const FULL_FILTER: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const LIGHT_FILTER: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSDS3D_FILTER_QUALITY_COUNT: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(2i32);
impl ::core::marker::Copy for KSDS3D_HRTF_FILTER_QUALITY {}
impl ::core::clone::Clone for KSDS3D_HRTF_FILTER_QUALITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_HRTF_FILTER_QUALITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDS3D_HRTF_FILTER_QUALITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSDS3D_HRTF_FILTER_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const DS3D_HRTF_VERSION_1: KSDS3D_HRTF_FILTER_VERSION = KSDS3D_HRTF_FILTER_VERSION(0i32);
impl ::core::marker::Copy for KSDS3D_HRTF_FILTER_VERSION {}
impl ::core::clone::Clone for KSDS3D_HRTF_FILTER_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_HRTF_FILTER_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDS3D_HRTF_FILTER_VERSION").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSDS3D_HRTF_INIT_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_HRTF_INIT_MSG").field("Size", &self.Size).field("Quality", &self.Quality).field("SampleRate", &self.SampleRate).field("MaxFilterSize", &self.MaxFilterSize).field("FilterTransientMuteLength", &self.FilterTransientMuteLength).field("FilterOverlapBufferLength", &self.FilterOverlapBufferLength).field("OutputOverlapBufferLength", &self.OutputOverlapBufferLength).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_HRTF_INIT_MSG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDS3D_HRTF_INIT_MSG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_HRTF_INIT_MSG>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDS3D_HRTF_INIT_MSG {}
impl ::core::default::Default for KSDS3D_HRTF_INIT_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSDS3D_HRTF_PARAMS_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_HRTF_PARAMS_MSG").field("Size", &self.Size).field("Enabled", &self.Enabled).field("SwapChannels", &self.SwapChannels).field("ZeroAzimuth", &self.ZeroAzimuth).field("CrossFadeOutput", &self.CrossFadeOutput).field("FilterSize", &self.FilterSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSDS3D_HRTF_PARAMS_MSG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSDS3D_HRTF_PARAMS_MSG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_HRTF_PARAMS_MSG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSDS3D_HRTF_PARAMS_MSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSDS3D_HRTF_PARAMS_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSDS3D_ITD_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_ITD_PARAMS").field("Channel", &self.Channel).field("VolSmoothScale", &self.VolSmoothScale).field("TotalDryAttenuation", &self.TotalDryAttenuation).field("TotalWetAttenuation", &self.TotalWetAttenuation).field("SmoothFrequency", &self.SmoothFrequency).field("Delay", &self.Delay).finish()
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_ITD_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDS3D_ITD_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_ITD_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDS3D_ITD_PARAMS {}
impl ::core::default::Default for KSDS3D_ITD_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSDS3D_ITD_PARAMS_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_ITD_PARAMS_MSG").field("Enabled", &self.Enabled).field("LeftParams", &self.LeftParams).field("RightParams", &self.RightParams).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSDS3D_ITD_PARAMS_MSG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDS3D_ITD_PARAMS_MSG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_ITD_PARAMS_MSG>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDS3D_ITD_PARAMS_MSG {}
impl ::core::default::Default for KSDS3D_ITD_PARAMS_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSDS3D_LISTENER_ALL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDS3D_LISTENER_ALL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_LISTENER_ALL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDS3D_LISTENER_ALL {}
impl ::core::default::Default for KSDS3D_LISTENER_ALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSDS3D_LISTENER_ORIENTATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSDS3D_LISTENER_ORIENTATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSDS3D_LISTENER_ORIENTATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSDS3D_LISTENER_ORIENTATION {}
impl ::core::default::Default for KSDS3D_LISTENER_ORIENTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSERROR").field("Context", &self.Context).field("Status", &self.Status).finish()
    }
}
unsafe impl ::windows::core::Abi for KSERROR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSERROR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSERROR>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSERROR {}
impl ::core::default::Default for KSERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSEVENTDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSEVENTDATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSEVENTDATA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSEVENTDATA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSEVENTDATA_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSEVENTDATA_0_0").field("Unused", &self.Unused).field("Alignment", &self.Alignment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSEVENTDATA_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSEVENTDATA_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSEVENTDATA_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSEVENTDATA_0_1").field("Event", &self.Event).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSEVENTDATA_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSEVENTDATA_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSEVENTDATA_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSEVENTDATA_0_2").field("Semaphore", &self.Semaphore).field("Reserved", &self.Reserved).field("Adjustment", &self.Adjustment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSEVENTDATA_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENTDATA_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSEVENTDATA_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENTDATA_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
pub const KSEVENTSETID_AudioControlChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe85e9698_fa2f_11d1_95bd_00c04fb925d3);
pub const KSEVENTSETID_CameraAsyncControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22a11754_9701_4088_b33f_6b9cbc52df5e);
pub const KSEVENTSETID_CameraEvent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7899b2e0_6b43_4964_9d2a_a21f4061f576);
pub const KSEVENTSETID_Clock: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x364d8e20_62c7_11cf_a5d6_28db04c10000);
pub const KSEVENTSETID_Connection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f4bcbe0_9ea5_11cf_a5d6_28db04c10000);
pub const KSEVENTSETID_Device: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x288296ec_9f94_41b4_a153_aa31aeecb33f);
pub const KSEVENTSETID_DynamicFormatChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x162ac456_83d7_4239_96df_c75ffa138bc6);
pub const KSEVENTSETID_EXTDEV_Command: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x109c7988_b3cb_11d2_b48e_006097b3391b);
pub const KSEVENTSETID_ExtendedCameraControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x571c92c9_13a2_47e3_a649_d2a778166384);
pub const KSEVENTSETID_LoopedStreaming: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4682b940_c6ef_11d0_96d8_00aa0051e51d);
pub const KSEVENTSETID_PinCapsChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd4f192e_3b78_49ad_a534_2c315b822000);
pub const KSEVENTSETID_SoundDetector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69785c9b_fc2d_49d6_ac32_4799f87de9f6);
pub const KSEVENTSETID_StreamAllocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d95571_073c_11d0_a161_0020afd156e4);
pub const KSEVENTSETID_Telephony: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb77f12b4_ceb4_4484_8d5e_52c1e7d8762d);
pub const KSEVENTSETID_VIDCAPTOSTI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb47de20_f628_11d1_ba41_00a0c90d2b05);
pub const KSEVENTSETID_VIDCAP_TVAUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0651_28e4_11d0_a18c_00a0c9118956);
pub const KSEVENTSETID_VPNotify: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20c5598e_d3c8_11d0_8dfc_00c04fd7c08b);
pub const KSEVENTSETID_VPVBINotify: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec529b01_1a1f_11d1_bad9_00609744111a);
pub const KSEVENTSETID_VolumeLimit: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda168465_3a7c_4858_9d4a_3e8e24701aef);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_AUDIO_CONTROL_CHANGE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONTROL_CHANGE: KSEVENT_AUDIO_CONTROL_CHANGE = KSEVENT_AUDIO_CONTROL_CHANGE(0i32);
impl ::core::marker::Copy for KSEVENT_AUDIO_CONTROL_CHANGE {}
impl ::core::clone::Clone for KSEVENT_AUDIO_CONTROL_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_AUDIO_CONTROL_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_AUDIO_CONTROL_CHANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_AUDIO_CONTROL_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_AUDIO_CONTROL_CHANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_CAMERACONTROL(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CAMERACONTROL_FOCUS: KSEVENT_CAMERACONTROL = KSEVENT_CAMERACONTROL(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CAMERACONTROL_ZOOM: KSEVENT_CAMERACONTROL = KSEVENT_CAMERACONTROL(1i32);
impl ::core::marker::Copy for KSEVENT_CAMERACONTROL {}
impl ::core::clone::Clone for KSEVENT_CAMERACONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_CAMERACONTROL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_CAMERACONTROL {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_CAMERACONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CAMERACONTROL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_CAMERAEVENT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_PHOTO_SAMPLE_SCANNED: KSEVENT_CAMERAEVENT = KSEVENT_CAMERAEVENT(0i32);
impl ::core::marker::Copy for KSEVENT_CAMERAEVENT {}
impl ::core::clone::Clone for KSEVENT_CAMERAEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_CAMERAEVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_CAMERAEVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_CAMERAEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CAMERAEVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_CLOCK_POSITION(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CLOCK_INTERVAL_MARK: KSEVENT_CLOCK_POSITION = KSEVENT_CLOCK_POSITION(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CLOCK_POSITION_MARK: KSEVENT_CLOCK_POSITION = KSEVENT_CLOCK_POSITION(1i32);
impl ::core::marker::Copy for KSEVENT_CLOCK_POSITION {}
impl ::core::clone::Clone for KSEVENT_CLOCK_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_CLOCK_POSITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_CLOCK_POSITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_CLOCK_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CLOCK_POSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_CONNECTION(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_POSITIONUPDATE: KSEVENT_CONNECTION = KSEVENT_CONNECTION(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_DATADISCONTINUITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_TIMEDISCONTINUITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_PRIORITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CONNECTION_ENDOFSTREAM: KSEVENT_CONNECTION = KSEVENT_CONNECTION(4i32);
impl ::core::marker::Copy for KSEVENT_CONNECTION {}
impl ::core::clone::Clone for KSEVENT_CONNECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_CONNECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_CONNECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CONNECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_CROSSBAR(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_CROSSBAR_CHANGED: KSEVENT_CROSSBAR = KSEVENT_CROSSBAR(0i32);
impl ::core::marker::Copy for KSEVENT_CROSSBAR {}
impl ::core::clone::Clone for KSEVENT_CROSSBAR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_CROSSBAR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_CROSSBAR {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_CROSSBAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CROSSBAR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_DEVCMD(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_COMMAND_NOTIFY_INTERIM_READY: KSEVENT_DEVCMD = KSEVENT_DEVCMD(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_COMMAND_CONTROL_INTERIM_READY: KSEVENT_DEVCMD = KSEVENT_DEVCMD(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_COMMAND_BUSRESET: KSEVENT_DEVCMD = KSEVENT_DEVCMD(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_TIMECODE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_OPERATION_MODE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_TRANSPORT_STATE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_NOTIFY_REMOVAL: KSEVENT_DEVCMD = KSEVENT_DEVCMD(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_EXTDEV_NOTIFY_MEDIUM_CHANGE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(7i32);
impl ::core::marker::Copy for KSEVENT_DEVCMD {}
impl ::core::clone::Clone for KSEVENT_DEVCMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_DEVCMD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_DEVCMD {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_DEVCMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_DEVCMD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_DEVICE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DEVICE_LOST: KSEVENT_DEVICE = KSEVENT_DEVICE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DEVICE_PREEMPTED: KSEVENT_DEVICE = KSEVENT_DEVICE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DEVICE_THERMAL_HIGH: KSEVENT_DEVICE = KSEVENT_DEVICE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DEVICE_THERMAL_LOW: KSEVENT_DEVICE = KSEVENT_DEVICE(3i32);
impl ::core::marker::Copy for KSEVENT_DEVICE {}
impl ::core::clone::Clone for KSEVENT_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_DEVICE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_DEVICE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_DYNAMICFORMATCHANGE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_DYNAMIC_FORMAT_CHANGE: KSEVENT_DYNAMICFORMATCHANGE = KSEVENT_DYNAMICFORMATCHANGE(0i32);
impl ::core::marker::Copy for KSEVENT_DYNAMICFORMATCHANGE {}
impl ::core::clone::Clone for KSEVENT_DYNAMICFORMATCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_DYNAMICFORMATCHANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_DYNAMICFORMATCHANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_DYNAMICFORMATCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_DYNAMICFORMATCHANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_ENTRY_BUFFERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_ENTRY_DELETED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_ENTRY_ONESHOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_LOOPEDSTREAMING(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_LOOPEDSTREAMING_POSITION: KSEVENT_LOOPEDSTREAMING = KSEVENT_LOOPEDSTREAMING(0i32);
impl ::core::marker::Copy for KSEVENT_LOOPEDSTREAMING {}
impl ::core::clone::Clone for KSEVENT_LOOPEDSTREAMING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_LOOPEDSTREAMING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_LOOPEDSTREAMING {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_LOOPEDSTREAMING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_LOOPEDSTREAMING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_PINCAPS_CHANGENOTIFICATIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_PINCAPS_FORMATCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = KSEVENT_PINCAPS_CHANGENOTIFICATIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_PINCAPS_JACKINFOCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = KSEVENT_PINCAPS_CHANGENOTIFICATIONS(1i32);
impl ::core::marker::Copy for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {}
impl ::core::clone::Clone for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_PINCAPS_CHANGENOTIFICATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_SOUNDDETECTOR(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_SOUNDDETECTOR_MATCHDETECTED: KSEVENT_SOUNDDETECTOR = KSEVENT_SOUNDDETECTOR(1i32);
impl ::core::marker::Copy for KSEVENT_SOUNDDETECTOR {}
impl ::core::clone::Clone for KSEVENT_SOUNDDETECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_SOUNDDETECTOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_SOUNDDETECTOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_SOUNDDETECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_SOUNDDETECTOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_STREAMALLOCATOR(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_STREAMALLOCATOR_INTERNAL_FREEFRAME: KSEVENT_STREAMALLOCATOR = KSEVENT_STREAMALLOCATOR(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_STREAMALLOCATOR_FREEFRAME: KSEVENT_STREAMALLOCATOR = KSEVENT_STREAMALLOCATOR(1i32);
impl ::core::marker::Copy for KSEVENT_STREAMALLOCATOR {}
impl ::core::clone::Clone for KSEVENT_STREAMALLOCATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_STREAMALLOCATOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_STREAMALLOCATOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_STREAMALLOCATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_STREAMALLOCATOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_TELEPHONY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TELEPHONY_ENDPOINTPAIRS_CHANGED: KSEVENT_TELEPHONY = KSEVENT_TELEPHONY(0i32);
impl ::core::marker::Copy for KSEVENT_TELEPHONY {}
impl ::core::clone::Clone for KSEVENT_TELEPHONY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_TELEPHONY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_TELEPHONY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_TELEPHONY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_TELEPHONY").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSEVENT_TIME_INTERVAL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENT_TIME_INTERVAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSEVENT_TIME_INTERVAL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENT_TIME_INTERVAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TIME_INTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSEVENT_TIME_MARK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENT_TIME_MARK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSEVENT_TIME_MARK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENT_TIME_MARK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TIME_MARK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_TUNER(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TUNER_CHANGED: KSEVENT_TUNER = KSEVENT_TUNER(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TUNER_INITIATE_SCAN: KSEVENT_TUNER = KSEVENT_TUNER(1i32);
impl ::core::marker::Copy for KSEVENT_TUNER {}
impl ::core::clone::Clone for KSEVENT_TUNER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_TUNER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_TUNER {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_TUNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_TUNER").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSEVENT_TUNER_INITIATE_SCAN_S {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSEVENT_TUNER_INITIATE_SCAN_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSEVENT_TUNER_INITIATE_SCAN_S>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSEVENT_TUNER_INITIATE_SCAN_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TUNER_INITIATE_SCAN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_TVAUDIO(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_TVAUDIO_CHANGED: KSEVENT_TVAUDIO = KSEVENT_TVAUDIO(0i32);
impl ::core::marker::Copy for KSEVENT_TVAUDIO {}
impl ::core::clone::Clone for KSEVENT_TVAUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_TVAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_TVAUDIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_TVAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_TVAUDIO").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_VIDCAPTOSTI(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VIDCAPTOSTI_EXT_TRIGGER: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VIDCAP_AUTO_UPDATE: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VIDCAP_SEARCH: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(2i32);
impl ::core::marker::Copy for KSEVENT_VIDCAPTOSTI {}
impl ::core::clone::Clone for KSEVENT_VIDCAPTOSTI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_VIDCAPTOSTI {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_VIDCAPTOSTI {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_VIDCAPTOSTI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VIDCAPTOSTI").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_VIDEODECODER(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VIDEODECODER_CHANGED: KSEVENT_VIDEODECODER = KSEVENT_VIDEODECODER(0i32);
impl ::core::marker::Copy for KSEVENT_VIDEODECODER {}
impl ::core::clone::Clone for KSEVENT_VIDEODECODER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_VIDEODECODER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_VIDEODECODER {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_VIDEODECODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VIDEODECODER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_VOLUMELIMIT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VOLUMELIMIT_CHANGED: KSEVENT_VOLUMELIMIT = KSEVENT_VOLUMELIMIT(0i32);
impl ::core::marker::Copy for KSEVENT_VOLUMELIMIT {}
impl ::core::clone::Clone for KSEVENT_VOLUMELIMIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_VOLUMELIMIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_VOLUMELIMIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_VOLUMELIMIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VOLUMELIMIT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_VPNOTIFY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VPNOTIFY_FORMATCHANGE: KSEVENT_VPNOTIFY = KSEVENT_VPNOTIFY(0i32);
impl ::core::marker::Copy for KSEVENT_VPNOTIFY {}
impl ::core::clone::Clone for KSEVENT_VPNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_VPNOTIFY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_VPNOTIFY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_VPNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VPNOTIFY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEVENT_VPVBINOTIFY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSEVENT_VPVBINOTIFY_FORMATCHANGE: KSEVENT_VPVBINOTIFY = KSEVENT_VPVBINOTIFY(0i32);
impl ::core::marker::Copy for KSEVENT_VPVBINOTIFY {}
impl ::core::clone::Clone for KSEVENT_VPVBINOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSEVENT_VPVBINOTIFY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSEVENT_VPVBINOTIFY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSEVENT_VPVBINOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VPVBINOTIFY").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSE_NODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSE_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSE_NODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSE_NODE {}
impl ::core::default::Default for KSE_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSE_PIN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSE_PIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSE_PIN>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSE_PIN {}
impl ::core::default::Default for KSE_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSFRAMETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSFRAMETIME").field("Duration", &self.Duration).field("FrameFlags", &self.FrameFlags).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSFRAMETIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSFRAMETIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSFRAMETIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSFRAMETIME {}
impl ::core::default::Default for KSFRAMETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSGOP_USERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSGOP_USERDATA").field("sc", &self.sc).field("reserved1", &self.reserved1).field("cFields", &self.cFields).field("l21Data", &self.l21Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSGOP_USERDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSGOP_USERDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSGOP_USERDATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSGOP_USERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSGOP_USERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSIDENTIFIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSIDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSIDENTIFIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSIDENTIFIER {}
impl ::core::default::Default for KSIDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSIDENTIFIER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSIDENTIFIER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSIDENTIFIER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSIDENTIFIER_0 {}
impl ::core::default::Default for KSIDENTIFIER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSIDENTIFIER_0_0 {
    pub Set: ::windows::core::GUID,
    pub Id: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for KSIDENTIFIER_0_0 {}
impl ::core::clone::Clone for KSIDENTIFIER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSIDENTIFIER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSIDENTIFIER_0_0").field("Set", &self.Set).field("Id", &self.Id).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for KSIDENTIFIER_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSIDENTIFIER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSIDENTIFIER_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSIDENTIFIER_0_0 {}
impl ::core::default::Default for KSIDENTIFIER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSINTERFACESETID_FileIo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c6f932c_e771_11d0_b8ff_00a0c9223196);
pub const KSINTERFACESETID_Media: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a13eb40_30a7_11d0_a5d6_28db04c10000);
pub const KSINTERFACESETID_Standard: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a8766a0_62ce_11cf_a5d6_28db04c10000);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSINTERFACE_FILEIO(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_FILEIO_STREAMING: KSINTERFACE_FILEIO = KSINTERFACE_FILEIO(0i32);
impl ::core::marker::Copy for KSINTERFACE_FILEIO {}
impl ::core::clone::Clone for KSINTERFACE_FILEIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSINTERFACE_FILEIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSINTERFACE_FILEIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSINTERFACE_FILEIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSINTERFACE_FILEIO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSINTERFACE_MEDIA(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_MEDIA_MUSIC: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_MEDIA_WAVE_BUFFERED: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_MEDIA_WAVE_QUEUED: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(2i32);
impl ::core::marker::Copy for KSINTERFACE_MEDIA {}
impl ::core::clone::Clone for KSINTERFACE_MEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSINTERFACE_MEDIA {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSINTERFACE_MEDIA {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSINTERFACE_MEDIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSINTERFACE_MEDIA").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSINTERFACE_STANDARD(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_STANDARD_STREAMING: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_STANDARD_LOOPED_STREAMING: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSINTERFACE_STANDARD_CONTROL: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(2i32);
impl ::core::marker::Copy for KSINTERFACE_STANDARD {}
impl ::core::clone::Clone for KSINTERFACE_STANDARD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSINTERFACE_STANDARD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSINTERFACE_STANDARD {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSINTERFACE_STANDARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSINTERFACE_STANDARD").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSINTERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSINTERVAL").field("TimeBase", &self.TimeBase).field("Interval", &self.Interval).finish()
    }
}
unsafe impl ::windows::core::Abi for KSINTERVAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSINTERVAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSINTERVAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSINTERVAL {}
impl ::core::default::Default for KSINTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSJACK_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSJACK_DESCRIPTION").field("ChannelMapping", &self.ChannelMapping).field("Color", &self.Color).field("ConnectionType", &self.ConnectionType).field("GeoLocation", &self.GeoLocation).field("GenLocation", &self.GenLocation).field("PortConnection", &self.PortConnection).field("IsConnected", &self.IsConnected).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSJACK_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSJACK_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSJACK_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSJACK_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSJACK_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSJACK_DESCRIPTION2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSJACK_DESCRIPTION2").field("DeviceStateInfo", &self.DeviceStateInfo).field("JackCapabilities", &self.JackCapabilities).finish()
    }
}
unsafe impl ::windows::core::Abi for KSJACK_DESCRIPTION2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSJACK_DESCRIPTION2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSJACK_DESCRIPTION2>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSJACK_DESCRIPTION2 {}
impl ::core::default::Default for KSJACK_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSJACK_SINK_CONNECTIONTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSJACK_SINK_CONNECTIONTYPE_HDMI: KSJACK_SINK_CONNECTIONTYPE = KSJACK_SINK_CONNECTIONTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSJACK_SINK_CONNECTIONTYPE_DISPLAYPORT: KSJACK_SINK_CONNECTIONTYPE = KSJACK_SINK_CONNECTIONTYPE(1i32);
impl ::core::marker::Copy for KSJACK_SINK_CONNECTIONTYPE {}
impl ::core::clone::Clone for KSJACK_SINK_CONNECTIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSJACK_SINK_CONNECTIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSJACK_SINK_CONNECTIONTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSJACK_SINK_CONNECTIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSJACK_SINK_CONNECTIONTYPE").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSJACK_SINK_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSJACK_SINK_INFORMATION").field("ConnType", &self.ConnType).field("ManufacturerId", &self.ManufacturerId).field("ProductId", &self.ProductId).field("AudioLatency", &self.AudioLatency).field("HDCPCapable", &self.HDCPCapable).field("AICapable", &self.AICapable).field("SinkDescriptionLength", &self.SinkDescriptionLength).field("SinkDescription", &self.SinkDescription).field("PortId", &self.PortId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSJACK_SINK_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSJACK_SINK_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSJACK_SINK_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSJACK_SINK_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSJACK_SINK_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSMEDIUMSETID_MidiBus: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05908040_3246_11d0_a5d6_28db04c10000);
pub const KSMEDIUMSETID_Standard: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4747b320_62ce_11cf_a5d6_28db04c10000);
pub const KSMEDIUMSETID_VPBus: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa18c15ec_ce43_11d0_abe7_00a0c9223196);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMEDIUM_STANDARD_DEVIO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMEDIUM_TYPE_ANYINSTANCE: u32 = 0u32;
pub const KSMEMORY_TYPE_DEVICE_UNKNOWN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x091bb639_603f_11d1_b067_00a0c9062802);
pub const KSMEMORY_TYPE_KERNEL_NONPAGED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a6d5fc4_7895_11d1_b069_00a0c9062802);
pub const KSMEMORY_TYPE_KERNEL_PAGED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd833f8f8_7894_11d1_b069_00a0c9062802);
pub const KSMEMORY_TYPE_SYSTEM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x091bb638_603f_11d1_b067_00a0c9062802);
pub const KSMEMORY_TYPE_USER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb0fc28_7893_11d1_b069_00a0c9062802);
pub const KSMETHODSETID_StreamAllocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf6e4341_ec87_11cf_a130_0020afd156e4);
pub const KSMETHODSETID_StreamIo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65d003ca_1523_11d2_b27a_00a0c9223196);
pub const KSMETHODSETID_Wavetable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcef31eb_d907_11d0_9583_00c04fb925d3);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSMETHOD_STREAMALLOCATOR(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_STREAMALLOCATOR_ALLOC: KSMETHOD_STREAMALLOCATOR = KSMETHOD_STREAMALLOCATOR(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_STREAMALLOCATOR_FREE: KSMETHOD_STREAMALLOCATOR = KSMETHOD_STREAMALLOCATOR(1i32);
impl ::core::marker::Copy for KSMETHOD_STREAMALLOCATOR {}
impl ::core::clone::Clone for KSMETHOD_STREAMALLOCATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSMETHOD_STREAMALLOCATOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSMETHOD_STREAMALLOCATOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSMETHOD_STREAMALLOCATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMETHOD_STREAMALLOCATOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSMETHOD_STREAMIO(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_STREAMIO_READ: KSMETHOD_STREAMIO = KSMETHOD_STREAMIO(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_STREAMIO_WRITE: KSMETHOD_STREAMIO = KSMETHOD_STREAMIO(1i32);
impl ::core::marker::Copy for KSMETHOD_STREAMIO {}
impl ::core::clone::Clone for KSMETHOD_STREAMIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSMETHOD_STREAMIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSMETHOD_STREAMIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSMETHOD_STREAMIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMETHOD_STREAMIO").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSMETHOD_WAVETABLE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVETABLE_WAVE_ALLOC: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVETABLE_WAVE_FREE: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVETABLE_WAVE_FIND: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVETABLE_WAVE_WRITE: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(3i32);
impl ::core::marker::Copy for KSMETHOD_WAVETABLE {}
impl ::core::clone::Clone for KSMETHOD_WAVETABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSMETHOD_WAVETABLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSMETHOD_WAVETABLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSMETHOD_WAVETABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMETHOD_WAVETABLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMETHOD_WAVE_QUEUED_BREAKLOOP: u32 = 1u32;
pub const KSMFT_CATEGORY_AUDIO_DECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ea73fb4_ef7a_4559_8d5d_719d8f0426c7);
pub const KSMFT_CATEGORY_AUDIO_EFFECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11064c48_3648_4ed0_932e_05ce8ac811b7);
pub const KSMFT_CATEGORY_AUDIO_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91c64bd0_f91e_4d8c_9276_db248279d975);
pub const KSMFT_CATEGORY_DEMULTIPLEXER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8700a7a_939b_44c5_99d7_76226b23b3f1);
pub const KSMFT_CATEGORY_MULTIPLEXER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x059c561e_05ae_4b61_b69d_55b61ee54a7b);
pub const KSMFT_CATEGORY_OTHER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90175d57_b7ea_4901_aeb3_933a8747756f);
pub const KSMFT_CATEGORY_VIDEO_DECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6c02d4b_6833_45b4_971a_05a4b04bab91);
pub const KSMFT_CATEGORY_VIDEO_EFFECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12e17c21_532c_4a6e_8a1c_40825a736397);
pub const KSMFT_CATEGORY_VIDEO_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf79eac7d_e545_4387_bdee_d647d7bde42a);
pub const KSMFT_CATEGORY_VIDEO_PROCESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x302ea3fc_aa5f_47f9_9f7a_c2188bb16302);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSMICARRAY_MICARRAYTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICARRAYTYPE_LINEAR: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICARRAYTYPE_PLANAR: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICARRAYTYPE_3D: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(2i32);
impl ::core::marker::Copy for KSMICARRAY_MICARRAYTYPE {}
impl ::core::clone::Clone for KSMICARRAY_MICARRAYTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSMICARRAY_MICARRAYTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSMICARRAY_MICARRAYTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSMICARRAY_MICARRAYTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMICARRAY_MICARRAYTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSMICARRAY_MICTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_OMNIDIRECTIONAL: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_SUBCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_CARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_SUPERCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_HYPERCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_8SHAPED: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSMICARRAY_MICTYPE_VENDORDEFINED: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(15i32);
impl ::core::marker::Copy for KSMICARRAY_MICTYPE {}
impl ::core::clone::Clone for KSMICARRAY_MICTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSMICARRAY_MICTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSMICARRAY_MICTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSMICARRAY_MICTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMICARRAY_MICTYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSMPEGVID_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSMPEGVID_RECT").field("StartX", &self.StartX).field("StartY", &self.StartY).field("EndX", &self.EndX).field("EndY", &self.EndY).finish()
    }
}
unsafe impl ::windows::core::Abi for KSMPEGVID_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSMPEGVID_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSMPEGVID_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSMPEGVID_RECT {}
impl ::core::default::Default for KSMPEGVID_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSMULTIPLE_DATA_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSMULTIPLE_DATA_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSMULTIPLE_DATA_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSMULTIPLE_DATA_PROP {}
impl ::core::default::Default for KSMULTIPLE_DATA_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSMULTIPLE_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSMULTIPLE_ITEM").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
unsafe impl ::windows::core::Abi for KSMULTIPLE_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSMULTIPLE_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSMULTIPLE_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSMULTIPLE_ITEM {}
impl ::core::default::Default for KSMULTIPLE_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSMUSICFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSMUSICFORMAT").field("TimeDeltaMs", &self.TimeDeltaMs).field("ByteCount", &self.ByteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for KSMUSICFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSMUSICFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSMUSICFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSMUSICFORMAT {}
impl ::core::default::Default for KSMUSICFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSMUSIC_TECHNOLOGY_FMSYNTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x252c5c80_62e9_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86c92e60_62e8_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_SQSYNTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ecf4380_62e9_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_SWSYNTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37407736_3620_11d1_85d3_0000f8754380);
pub const KSMUSIC_TECHNOLOGY_WAVETABLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x394ec7c0_62e9_11cf_a5d6_28db04c10000);
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
unsafe impl ::windows::core::Abi for KSM_NODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSM_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSM_NODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSM_NODE {}
impl ::core::default::Default for KSM_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSNAME_Allocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x642f5d00_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_Clock: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53172480_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_Filter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b365890_165f_11d0_a195_0020afd156e4);
pub const KSNAME_Pin: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146f1a80_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_TopologyNode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0621061a_ee75_11d0_b915_00a0c9223196);
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
unsafe impl ::windows::core::Abi for KSNODEPROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSNODEPROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSNODEPROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSNODEPROPERTY {}
impl ::core::default::Default for KSNODEPROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSNODEPROPERTY_AUDIO_3D_LISTENER>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_3D_LISTENER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSNODEPROPERTY_AUDIO_3D_LISTENER>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_3D_LISTENER {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSNODEPROPERTY_AUDIO_CHANNEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSNODEPROPERTY_AUDIO_CHANNEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_CHANNEL {}
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSNODEPROPERTY_AUDIO_DEV_SPECIFIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {}
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for KSNODEPROPERTY_AUDIO_PROPERTY {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSNODEPROPERTY_AUDIO_PROPERTY>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_PROPERTY {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for KSNODEPROPERTY_AUDIO_PROPERTY {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSNODEPROPERTY_AUDIO_PROPERTY>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for KSNODEPROPERTY_AUDIO_PROPERTY {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSNODETYPE_1394_DA_STREAM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21fe6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_1394_DV_STREAM_SOUNDTRACK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21fe7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_3D_EFFECTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55515860_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ADC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d837fe0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_AGC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ANALOG_CONNECTOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21fe1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_ANALOG_TAPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_AUDIO_ENGINE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35caf6e4_f3b3_4168_bb4b_55e77a461c7e);
pub const KSNODETYPE_AUDIO_KEYWORDDETECTOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3817e0b8_df58_4375_b669_c49634331f9d);
pub const KSNODETYPE_AUDIO_LOOPBACK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f42c0b2_91ce_4bcf_9ccd_0e599037ab35);
pub const KSNODETYPE_AUDIO_MODULE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45aab42e_caeb_4052_8aa9_b38cb5109619);
pub const KSNODETYPE_BIDIRECTIONAL_UNDEFINED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21de0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CABLE_TUNER_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220ee_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CD_PLAYER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CHORUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20173f20_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_COMMUNICATION_SPEAKER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ce6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DAC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507ae360_c554_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DAT_IO_DIGITAL_AUDIO_TAPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DCC_IO_DIGITAL_COMPACT_CASSETTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DELAY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x144981e0_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DEMUX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0eb67d4_e807_11d0_958a_00c04fb925d3);
pub const KSNODETYPE_DESKTOP_MICROPHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21be2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DESKTOP_SPEAKER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ce4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DEV_SPECIFIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x941c7ac0_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DIGITAL_AUDIO_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21fe2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DISPLAYPORT_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe47e4031_3ea6_418d_8f9b_b73843ccba97);
pub const KSNODETYPE_DOWN_LINE_PHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ee3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DRM_DESCRAMBLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffbb6e3f_ccfe_4d84_90d9_421418b03a8e);
pub const KSNODETYPE_DSS_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220ef_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DVD_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220eb_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DYN_RANGE_COMPRESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c8a6a8_601f_4af8_8793_d905ff4ca97d);
pub const KSNODETYPE_ECHO_CANCELING_SPEAKERPHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21de5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_ECHO_SUPPRESSING_SPEAKERPHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21de4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EMBEDDED_UNDEFINED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EQUALIZATION_NOISE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EQUALIZER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d41b4a0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_EXTERNAL_UNDEFINED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21fe0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_FM_RX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x834a733c_f485_41c0_a62b_513025014e40);
pub const KSNODETYPE_HANDSET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21de1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HDMI_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1b9cc2a_f519_417f_91c9_55fa65481001);
pub const KSNODETYPE_HEADPHONES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ce2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HEADSET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21de2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HEAD_MOUNTED_DISPLAY_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ce3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_INPUT_UNDEFINED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21be0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LEGACY_AUDIO_CONNECTOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21fe4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LEVEL_CALIBRATION_NOISE_SOURCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LINE_CONNECTOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21fe3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LOUDNESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41887440_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_LOW_FREQUENCY_EFFECTS_SPEAKER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ce7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MICROPHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21be1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MICROPHONE_ARRAY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21be5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MIDI_ELEMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01c6fe66_6e48_4c65_ac9b_52db5d656c7e);
pub const KSNODETYPE_MIDI_JACK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x265e0c3f_fa39_4df3_ab04_be01b91e299a);
pub const KSNODETYPE_MINIDISK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MULTITRACK_RECORDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220f2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b223c0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_MUX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ceaf780_c556_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_NOISE_SUPPRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const KSNODETYPE_OMNI_DIRECTIONAL_MICROPHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21be4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_OUTPUT_UNDEFINED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ce0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PARAMETRIC_EQUALIZER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19bb3a6a_ce2b_4442_87ec_6727c3cab477);
pub const KSNODETYPE_PEAKMETER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa085651e_5f0d_4b36_a869_d195d6ab4b9e);
pub const KSNODETYPE_PERSONAL_MICROPHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21be3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PHONE_LINE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ee1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PHONOGRAPH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e8_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PROCESSING_MICROPHONE_ARRAY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21be6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PROLOGIC_DECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x831c2c80_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_PROLOGIC_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8074c5b2_3c66_11d2_b45a_3078302c2030);
pub const KSNODETYPE_RADIO_RECEIVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220f0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_RADIO_TRANSMITTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220f1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_REVERB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef0328e0_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ROOM_SPEAKER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ce5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SATELLITE_RECEIVER_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220ed_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPDIF_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21fe5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ce1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKERPHONE_NO_ECHO_REDUCTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21de3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKERS_STATIC_JACK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28e04f87_4dbe_4f8d_8589_025d209dfb4a);
pub const KSNODETYPE_SRC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9db7b9e0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_STEREO_WIDE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9e69800_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda441a60_c556_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SUPERMIX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe573adc0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SYNTHESIZER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220f3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TELEPHONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ee2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TELEPHONY_BIDI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x686d7cc0_d903_4258_b443_3a3d3580741c);
pub const KSNODETYPE_TELEPHONY_UNDEFINED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff21ee0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7607e580_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_TV_TUNER_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220ec_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_UPDOWN_MIX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7edc5cf_7b63_4ee2_a100_29ee2cb6b2de);
pub const KSNODETYPE_VCR_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220e9_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_CAMERA_TERMINAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff229e6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_DISC_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff220ea_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_INPUT_MTT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff229e7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_INPUT_TERMINAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff229e2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_OUTPUT_MTT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff229e8_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_OUTPUT_TERMINAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff229e3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_PROCESSING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff229e5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_SELECTOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff229e4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_STREAMING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdff229e1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a5acc00_c557_11d0_8a2b_00a0c9255ac1);
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
impl ::core::fmt::Debug for KSNODE_CREATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSNODE_CREATE").field("CreateFlags", &self.CreateFlags).field("Node", &self.Node).finish()
    }
}
unsafe impl ::windows::core::Abi for KSNODE_CREATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSNODE_CREATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSNODE_CREATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSNODE_CREATE {}
impl ::core::default::Default for KSNODE_CREATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSNOTIFICATIONID_AudioModule: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c2220f0_d9a6_4d5c_a036_573857fd50d2);
pub const KSNOTIFICATIONID_SoundDetector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6389d844_bb32_4c4c_a802_f4b4b77afead);
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
impl ::core::fmt::Debug for KSPIN_CINSTANCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPIN_CINSTANCES").field("PossibleCount", &self.PossibleCount).field("CurrentCount", &self.CurrentCount).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPIN_CINSTANCES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPIN_CINSTANCES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPIN_CINSTANCES>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPIN_CINSTANCES {}
impl ::core::default::Default for KSPIN_CINSTANCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPIN_COMMUNICATION(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_NONE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_SINK: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_SOURCE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_BOTH: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_COMMUNICATION_BRIDGE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(4i32);
impl ::core::marker::Copy for KSPIN_COMMUNICATION {}
impl ::core::clone::Clone for KSPIN_COMMUNICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPIN_COMMUNICATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPIN_COMMUNICATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPIN_COMMUNICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPIN_COMMUNICATION").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPIN_CONNECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPIN_CONNECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPIN_CONNECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPIN_CONNECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPIN_CONNECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPIN_DATAFLOW(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_DATAFLOW_IN: KSPIN_DATAFLOW = KSPIN_DATAFLOW(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_DATAFLOW_OUT: KSPIN_DATAFLOW = KSPIN_DATAFLOW(2i32);
impl ::core::marker::Copy for KSPIN_DATAFLOW {}
impl ::core::clone::Clone for KSPIN_DATAFLOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPIN_DATAFLOW {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPIN_DATAFLOW {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPIN_DATAFLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPIN_DATAFLOW").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPIN_MDL_CACHING_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANUP: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_WAIT: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_NOWAIT: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPIN_MDL_CACHING_NOTIFY_ADDSAMPLE: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(3i32);
impl ::core::marker::Copy for KSPIN_MDL_CACHING_EVENT {}
impl ::core::clone::Clone for KSPIN_MDL_CACHING_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPIN_MDL_CACHING_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPIN_MDL_CACHING_EVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPIN_MDL_CACHING_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPIN_MDL_CACHING_EVENT").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPIN_MDL_CACHING_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPIN_MDL_CACHING_NOTIFICATION").field("Event", &self.Event).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPIN_MDL_CACHING_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPIN_MDL_CACHING_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPIN_MDL_CACHING_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPIN_MDL_CACHING_NOTIFICATION {}
impl ::core::default::Default for KSPIN_MDL_CACHING_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPIN_MDL_CACHING_NOTIFICATION32").field("Event", &self.Event).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPIN_MDL_CACHING_NOTIFICATION32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPIN_MDL_CACHING_NOTIFICATION32>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPIN_MDL_CACHING_NOTIFICATION32 {}
impl ::core::default::Default for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPIN_PHYSICALCONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPIN_PHYSICALCONNECTION").field("Size", &self.Size).field("Pin", &self.Pin).field("SymbolicLinkName", &self.SymbolicLinkName).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPIN_PHYSICALCONNECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPIN_PHYSICALCONNECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPIN_PHYSICALCONNECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPIN_PHYSICALCONNECTION {}
impl ::core::default::Default for KSPIN_PHYSICALCONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPPROPERTY_ALLOCATOR_MDLCACHING(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CLEANUP_CACHEDMDLPAGES: KSPPROPERTY_ALLOCATOR_MDLCACHING = KSPPROPERTY_ALLOCATOR_MDLCACHING(1i32);
impl ::core::marker::Copy for KSPPROPERTY_ALLOCATOR_MDLCACHING {}
impl ::core::clone::Clone for KSPPROPERTY_ALLOCATOR_MDLCACHING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPPROPERTY_ALLOCATOR_MDLCACHING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPPROPERTY_ALLOCATOR_MDLCACHING {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPPROPERTY_ALLOCATOR_MDLCACHING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPPROPERTY_ALLOCATOR_MDLCACHING").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPRIORITY").field("PriorityClass", &self.PriorityClass).field("PrioritySubClass", &self.PrioritySubClass).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPRIORITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPRIORITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPRIORITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPRIORITY {}
impl ::core::default::Default for KSPRIORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
pub const KSPROPERTYSETID_ExtendedCameraControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cb79112_c0d2_4213_9ca6_cd4fdb927972);
pub const KSPROPERTYSETID_NetworkCameraControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e780f09_5745_4e3a_bc9f_f226ea43a6ec);
pub const KSPROPERTYSETID_PerFrameSettingControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1f3e261_dee6_4537_bff5_ee206db54aac);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_AC3(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_ERROR_CONCEALMENT: KSPROPERTY_AC3 = KSPROPERTY_AC3(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_ALTERNATE_AUDIO: KSPROPERTY_AC3 = KSPROPERTY_AC3(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_DOWNMIX: KSPROPERTY_AC3 = KSPROPERTY_AC3(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_BIT_STREAM_MODE: KSPROPERTY_AC3 = KSPROPERTY_AC3(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_DIALOGUE_LEVEL: KSPROPERTY_AC3 = KSPROPERTY_AC3(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_LANGUAGE_CODE: KSPROPERTY_AC3 = KSPROPERTY_AC3(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AC3_ROOM_TYPE: KSPROPERTY_AC3 = KSPROPERTY_AC3(7i32);
impl ::core::marker::Copy for KSPROPERTY_AC3 {}
impl ::core::clone::Clone for KSPROPERTY_AC3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_AC3 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_AC3 {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_AC3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AC3").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CONTROL_HONOR_COUNT: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(3i32);
impl ::core::marker::Copy for KSPROPERTY_ALLOCATOR_CONTROL {}
impl ::core::clone::Clone for KSPROPERTY_ALLOCATOR_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_ALLOCATOR_CONTROL {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_ALLOCATOR_CONTROL").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S").field("InterleavedCapSupported", &self.InterleavedCapSupported).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S").field("InterleavedCapPossible", &self.InterleavedCapPossible).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S").field("CX", &self.CX).field("CY", &self.CY).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_AUDDECOUT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDDECOUT_MODES: KSPROPERTY_AUDDECOUT = KSPROPERTY_AUDDECOUT(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDDECOUT_CUR_MODE: KSPROPERTY_AUDDECOUT = KSPROPERTY_AUDDECOUT(1i32);
impl ::core::marker::Copy for KSPROPERTY_AUDDECOUT {}
impl ::core::clone::Clone for KSPROPERTY_AUDDECOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_AUDDECOUT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_AUDDECOUT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_AUDDECOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDDECOUT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_AUDIO(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_LATENCY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_COPY_PROTECTION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CHANNEL_CONFIG: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_VOLUMELEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DYNAMIC_RANGE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_QUALITY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_SAMPLING_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DYNAMIC_SAMPLING_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIX_LEVEL_TABLE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIX_LEVEL_CAPS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MUX_SOURCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MUTE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(13i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_BASS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(14i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(15i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_TREBLE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(16i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_BASS_BOOST: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(17i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_EQ_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(18i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_NUM_EQ_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(19i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_EQ_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(20i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_AGC: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(21i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DELAY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(22i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_LOUDNESS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(23i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_WIDE_MODE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(24i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_WIDENESS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(25i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_REVERB_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(26i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CHORUS_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(27i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DEV_SPECIFIC: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(28i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_DEMUX_DEST: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(29i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_STEREO_ENHANCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(30i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MANUFACTURE_GUID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(31i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PRODUCT_GUID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(32i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CPU_RESOURCES: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(33i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_STEREO_SPEAKER_GEOMETRY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(34i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_SURROUND_ENCODE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(35i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_3D_INTERFACE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(36i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEAKMETER: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(37i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_ALGORITHM_INSTANCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(38i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_FILTER_STATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(39i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PREFERRED_STATUS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(40i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_MAX_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(41i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_NUM_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(42i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_BAND_CENTER_FREQ: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(43i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_BAND_Q_FACTOR: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(44i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEQ_BAND_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(45i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(46i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_DEPTH: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(47i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_REVERB_TIME: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(48i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_REVERB_DELAY_FEEDBACK: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(49i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_POSITIONEX: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(50i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIC_ARRAY_GEOMETRY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(51i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PRESENTATION_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(52i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(53i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_LINEAR_BUFFER_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(54i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_PEAKMETER2: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(55i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_LASTBUFFER_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(56i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_VOLUMELIMIT_ENGAGED: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(57i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(58i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIC_SNR: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(59i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY2: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(60i32);
impl ::core::marker::Copy for KSPROPERTY_AUDIO {}
impl ::core::clone::Clone for KSPROPERTY_AUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_AUDIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_AUDIOENGINE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_LFXENABLE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_GFXENABLE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_MIXFORMAT: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_DEVICEFORMAT: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_SUPPORTEDDEVICEFORMATS: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_DESCRIPTOR: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_BUFFER_SIZE_RANGE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_LOOPBACK_PROTECTION: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOENGINE_VOLUMELEVEL: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(9i32);
impl ::core::marker::Copy for KSPROPERTY_AUDIOENGINE {}
impl ::core::clone::Clone for KSPROPERTY_AUDIOENGINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIOENGINE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_AUDIOENGINE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIOENGINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIOENGINE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_AUDIOMODULE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOMODULE_DESCRIPTORS: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOMODULE_COMMAND: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOMODULE_NOTIFICATION_DEVICE_ID: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(3i32);
impl ::core::marker::Copy for KSPROPERTY_AUDIOMODULE {}
impl ::core::clone::Clone for KSPROPERTY_AUDIOMODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIOMODULE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_AUDIOMODULE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIOMODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIOMODULE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_AUDIOPOSTURE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOPOSTURE_ORIENTATION: KSPROPERTY_AUDIOPOSTURE = KSPROPERTY_AUDIOPOSTURE(1i32);
impl ::core::marker::Copy for KSPROPERTY_AUDIOPOSTURE {}
impl ::core::clone::Clone for KSPROPERTY_AUDIOPOSTURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIOPOSTURE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_AUDIOPOSTURE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIOPOSTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIOPOSTURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_AUDIORESOURCEMANAGEMENT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIORESOURCEMANAGEMENT_RESOURCEGROUP: KSPROPERTY_AUDIORESOURCEMANAGEMENT = KSPROPERTY_AUDIORESOURCEMANAGEMENT(0i32);
impl ::core::marker::Copy for KSPROPERTY_AUDIORESOURCEMANAGEMENT {}
impl ::core::clone::Clone for KSPROPERTY_AUDIORESOURCEMANAGEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIORESOURCEMANAGEMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_AUDIORESOURCEMANAGEMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIORESOURCEMANAGEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIORESOURCEMANAGEMENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_AUDIOSIGNALPROCESSING(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIOSIGNALPROCESSING_MODES: KSPROPERTY_AUDIOSIGNALPROCESSING = KSPROPERTY_AUDIOSIGNALPROCESSING(0i32);
impl ::core::marker::Copy for KSPROPERTY_AUDIOSIGNALPROCESSING {}
impl ::core::clone::Clone for KSPROPERTY_AUDIOSIGNALPROCESSING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIOSIGNALPROCESSING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_AUDIOSIGNALPROCESSING {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIOSIGNALPROCESSING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIOSIGNALPROCESSING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_AUDIO_BUFFER_DURATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_BIBLIOGRAPHIC(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_LEADER: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(1380207648i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_LCCN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808529952i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ISBN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808595488i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ISSN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(842149920i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_CATALOGINGSOURCE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808726560i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808464672i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINCORPORATEBODY: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808530208i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINMEETINGNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825307424i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808661280i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_UNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808727072i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_TITLESTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892613152i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_VARYINGFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(909390368i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_PUBLICATION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808858144i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_PHYSICALDESCRIPTION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465184i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808727584i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(809055264i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_GENERALNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465696i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_BIBLIOGRAPHYNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(875574560i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_CONTENTSNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892351776i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_CREATIONCREDIT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(942683424i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_CITATION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808531232i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_PARTICIPANT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825308448i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SUMMARY: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808596768i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_TARGETAUDIENCE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825373984i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDFORMAVAILABLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808662304i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SYSTEMDETAILS: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(942880032i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_AWARDS: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(909653280i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465952i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTOPICALTERM: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808793632i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYGEOGRAPHIC: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825570848i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMGENRE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892679712i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMCURRICULUM: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(943011360i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808662816i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYRELATED: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808728352i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808466464i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808663072i32);
impl ::core::marker::Copy for KSPROPERTY_BIBLIOGRAPHIC {}
impl ::core::clone::Clone for KSPROPERTY_BIBLIOGRAPHIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_BIBLIOGRAPHIC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_BIBLIOGRAPHIC {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_BIBLIOGRAPHIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_BIBLIOGRAPHIC").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_BOUNDS_LONG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_BOUNDS_LONG>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONG {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPROPERTY_BOUNDS_LONG_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_BOUNDS_LONG_0").field("SignedMinimum", &self.SignedMinimum).field("SignedMaximum", &self.SignedMaximum).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_BOUNDS_LONG_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONG_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_BOUNDS_LONG_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONG_0 {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONG_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPROPERTY_BOUNDS_LONG_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_BOUNDS_LONG_1").field("UnsignedMinimum", &self.UnsignedMinimum).field("UnsignedMaximum", &self.UnsignedMaximum).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_BOUNDS_LONG_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONG_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_BOUNDS_LONG_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONG_1 {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONG_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_BOUNDS_LONGLONG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONGLONG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_BOUNDS_LONGLONG>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONGLONG {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONGLONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPROPERTY_BOUNDS_LONGLONG_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_BOUNDS_LONGLONG_0").field("SignedMinimum", &self.SignedMinimum).field("SignedMaximum", &self.SignedMaximum).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_BOUNDS_LONGLONG_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONGLONG_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_BOUNDS_LONGLONG_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONGLONG_0 {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONGLONG_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPROPERTY_BOUNDS_LONGLONG_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_BOUNDS_LONGLONG_1").field("UnsignedMinimum", &self.UnsignedMinimum).field("UnsignedMaximum", &self.UnsignedMaximum).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_BOUNDS_LONGLONG_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_BOUNDS_LONGLONG_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_BOUNDS_LONGLONG_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_BOUNDS_LONGLONG_1 {}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONGLONG_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_BTAUDIO(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ONESHOT_RECONNECT: KSPROPERTY_BTAUDIO = KSPROPERTY_BTAUDIO(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ONESHOT_DISCONNECT: KSPROPERTY_BTAUDIO = KSPROPERTY_BTAUDIO(1i32);
impl ::core::marker::Copy for KSPROPERTY_BTAUDIO {}
impl ::core::clone::Clone for KSPROPERTY_BTAUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_BTAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_BTAUDIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_BTAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_BTAUDIO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMAXFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTRIGGERTIME: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WARMSTART: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MAXVIDFPS_PHOTORES: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTHUMBNAIL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SCENEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_TORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FLASHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OPTIMIZATIONHINT: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WHITEBALANCEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EXPOSUREMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(13i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(14i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FIELDOFVIEW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(15i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EVCOMPENSATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(16i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_CAMERAANGLEOFFSET: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(17i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_METADATA: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(18i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSPRIORITY: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(19i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSSTATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(20i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(21i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_ISPCONTROL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(22i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOCONFIRMATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(23i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ZOOM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(24i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MCC: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(25i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO_ADVANCED: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(26i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOSTABILIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(27i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VFR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(28i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEDETECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(29i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOHDR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(30i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_HISTOGRAM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(31i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OIS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(32i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ADVANCEDPHOTO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(33i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PROFILE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(34i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEAUTH_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(35i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SECURE_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(36i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOTEMPORALDENOISING: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(37i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_IRTORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(38i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_RELATIVEPANELOPTIMIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(39i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EYEGAZECORRECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(40i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_BACKGROUNDSEGMENTATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(41i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(42i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(43i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(44i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END2: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(44i32);
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CAMERACONTROL_FLASH(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FLASH_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_FLASH = KSPROPERTY_CAMERACONTROL_FLASH(0i32);
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_FLASH {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_FLASH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_FLASH {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_FLASH {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_FLASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_FLASH").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_CAMERACONTROL_FLASH_S").field("Flash", &self.Flash).field("Capabilities", &self.Capabilities).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_FLASH_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_FLASH_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_FLASH_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY = KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY(0i32);
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S").field("Capabilities", &self.Capabilities).field("Reserved0", &self.Reserved0).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_NODE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_NODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_NODE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_NODE_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_NODE_S2>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_NODE_S2 {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CAPABILITY: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_SET: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CLEAR: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(2i32);
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST = KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST(0i32);
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_S2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_S2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_S2>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_S2 {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_S_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_S_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_S_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_S_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S").field("VideoStabilizationMode", &self.VideoStabilizationMode).field("Capabilities", &self.Capabilities).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE = KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE(0i32);
impl ::core::marker::Copy for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {}
impl ::core::clone::Clone for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_CLEAR: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_SET: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(1i32);
impl ::core::marker::Copy for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {}
impl ::core::clone::Clone for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CLOCK(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_TIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_PHYSICALTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_CORRELATEDTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_CORRELATEDPHYSICALTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_RESOLUTION: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CLOCK_STATE: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(5i32);
impl ::core::marker::Copy for KSPROPERTY_CLOCK {}
impl ::core::clone::Clone for KSPROPERTY_CLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CLOCK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CLOCK {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CLOCK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CONNECTION(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_STATE: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_PRIORITY: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_DATAFORMAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_PROPOSEDATAFORMAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_ACQUIREORDERING: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING_EX: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CONNECTION_STARTAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(7i32);
impl ::core::marker::Copy for KSPROPERTY_CONNECTION {}
impl ::core::clone::Clone for KSPROPERTY_CONNECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CONNECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CONNECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CONNECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_COPYPROT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_CHLG_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_DVD_KEY1: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_DEC_KEY2: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_TITLE_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_COPY_MACROVISION: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_REGION: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_SET_COPY_STATE: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDCOPY_DISC_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(128i32);
impl ::core::marker::Copy for KSPROPERTY_COPYPROT {}
impl ::core::clone::Clone for KSPROPERTY_COPYPROT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_COPYPROT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_COPYPROT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_COPYPROT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_COPYPROT").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CROSSBAR_ACTIVE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CROSSBAR_ACTIVE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CROSSBAR_ACTIVE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CROSSBAR_ACTIVE_S {}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_ACTIVE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CROSSBAR_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CROSSBAR_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CROSSBAR_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CROSSBAR_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CROSSBAR_PININFO_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CROSSBAR_PININFO_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CROSSBAR_PININFO_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CROSSBAR_PININFO_S {}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_PININFO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_CROSSBAR_ROUTE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_CROSSBAR_ROUTE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_CROSSBAR_ROUTE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CROSSBAR_ROUTE_S {}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_ROUTE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_CYCLIC(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CYCLIC_POSITION: KSPROPERTY_CYCLIC = KSPROPERTY_CYCLIC(0i32);
impl ::core::marker::Copy for KSPROPERTY_CYCLIC {}
impl ::core::clone::Clone for KSPROPERTY_CYCLIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_CYCLIC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_CYCLIC {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_CYCLIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CYCLIC").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_DESCRIPTION {}
impl ::core::default::Default for KSPROPERTY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_DIRECTSOUND3DBUFFER(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_ALL: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_POSITION: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_VELOCITY: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEANGLES: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEORIENTATION: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEOUTSIDEVOLUME: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MINDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MAXDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MODE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(8i32);
impl ::core::marker::Copy for KSPROPERTY_DIRECTSOUND3DBUFFER {}
impl ::core::clone::Clone for KSPROPERTY_DIRECTSOUND3DBUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_DIRECTSOUND3DBUFFER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_DIRECTSOUND3DBUFFER {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_DIRECTSOUND3DBUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_DIRECTSOUND3DBUFFER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_DIRECTSOUND3DLISTENER(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALL: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_POSITION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_VELOCITY: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ORIENTATION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DISTANCEFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ROLLOFFFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DOPPLERFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_BATCH: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALLOCATION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(8i32);
impl ::core::marker::Copy for KSPROPERTY_DIRECTSOUND3DLISTENER {}
impl ::core::clone::Clone for KSPROPERTY_DIRECTSOUND3DLISTENER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_DIRECTSOUND3DLISTENER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_DIRECTSOUND3DLISTENER {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_DIRECTSOUND3DLISTENER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_DIRECTSOUND3DLISTENER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_DRMAUDIOSTREAM(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DRMAUDIOSTREAM_CONTENTID: KSPROPERTY_DRMAUDIOSTREAM = KSPROPERTY_DRMAUDIOSTREAM(0i32);
impl ::core::marker::Copy for KSPROPERTY_DRMAUDIOSTREAM {}
impl ::core::clone::Clone for KSPROPERTY_DRMAUDIOSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_DRMAUDIOSTREAM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_DRMAUDIOSTREAM {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_DRMAUDIOSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_DRMAUDIOSTREAM").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_DROPPEDFRAMES_CURRENT_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {}
impl ::core::default::Default for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_DVDSUBPIC(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDSUBPIC_PALETTE: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDSUBPIC_HLI: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DVDSUBPIC_COMPOSIT_ON: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(2i32);
impl ::core::marker::Copy for KSPROPERTY_DVDSUBPIC {}
impl ::core::clone::Clone for KSPROPERTY_DVDSUBPIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_DVDSUBPIC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_DVDSUBPIC {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_DVDSUBPIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_DVDSUBPIC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_EXTDEVICE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_ID: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_VERSION: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_POWER_STATE: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_PORT: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTDEVICE_CAPABILITIES: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(4i32);
impl ::core::marker::Copy for KSPROPERTY_EXTDEVICE {}
impl ::core::clone::Clone for KSPROPERTY_EXTDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_EXTDEVICE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTDEVICE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_EXTDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_EXTDEVICE").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTDEVICE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_EXTDEVICE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTDEVICE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_EXTDEVICE_S {}
impl ::core::default::Default for KSPROPERTY_EXTDEVICE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTDEVICE_S_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_EXTDEVICE_S_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTDEVICE_S_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_EXTDEVICE_S_0 {}
impl ::core::default::Default for KSPROPERTY_EXTDEVICE_S_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_EXTENSION_UNIT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTENSION_UNIT_INFO: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTENSION_UNIT_CONTROL: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTENSION_UNIT_PASS_THROUGH: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(65535i32);
impl ::core::marker::Copy for KSPROPERTY_EXTENSION_UNIT {}
impl ::core::clone::Clone for KSPROPERTY_EXTENSION_UNIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_EXTENSION_UNIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTENSION_UNIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_EXTENSION_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_EXTENSION_UNIT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_EXTXPORT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_CAPABILITIES: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_INPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_OUTPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_LOAD_MEDIUM: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_MEDIUM_INFO: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_STATE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_STATE_NOTIFY: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_TIMECODE_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_ATN_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_EXTXPORT_RTC_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RAW_AVC_CMD: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(10i32);
impl ::core::marker::Copy for KSPROPERTY_EXTXPORT {}
impl ::core::clone::Clone for KSPROPERTY_EXTXPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_EXTXPORT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_EXTXPORT").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT_NODE_S {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_NODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTXPORT_NODE_S>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_NODE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT_NODE_S_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_NODE_S_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTXPORT_NODE_S_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_NODE_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_EXTXPORT_NODE_S_0_0").field("PayloadSize", &self.PayloadSize).field("Payload", &self.Payload).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTXPORT_NODE_S_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_NODE_S_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_EXTXPORT_NODE_S_0_1").field("frame", &self.frame).field("second", &self.second).field("minute", &self.minute).field("hour", &self.hour).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTXPORT_NODE_S_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_NODE_S_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT_S {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTXPORT_S>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT_S_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_S_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTXPORT_S_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_S_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT_S_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_EXTXPORT_S_0_0").field("PayloadSize", &self.PayloadSize).field("Payload", &self.Payload).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT_S_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_S_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTXPORT_S_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_S_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT_S_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_EXTXPORT_S_0_1").field("frame", &self.frame).field("second", &self.second).field("minute", &self.minute).field("hour", &self.hour).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_EXTXPORT_S_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_EXTXPORT_S_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_EXTXPORT_S_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_EXTXPORT_S_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_FMRX_CONTROL(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_FMRX_STATE: KSPROPERTY_FMRX_CONTROL = KSPROPERTY_FMRX_CONTROL(0i32);
impl ::core::marker::Copy for KSPROPERTY_FMRX_CONTROL {}
impl ::core::clone::Clone for KSPROPERTY_FMRX_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_FMRX_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_FMRX_CONTROL {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_FMRX_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_FMRX_CONTROL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_FMRX_TOPOLOGY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_FMRX_ENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_FMRX_VOLUME: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_FMRX_ANTENNAENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(2i32);
impl ::core::marker::Copy for KSPROPERTY_FMRX_TOPOLOGY {}
impl ::core::clone::Clone for KSPROPERTY_FMRX_TOPOLOGY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_FMRX_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_FMRX_TOPOLOGY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_FMRX_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_FMRX_TOPOLOGY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_GENERAL(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_GENERAL_COMPONENTID: KSPROPERTY_GENERAL = KSPROPERTY_GENERAL(0i32);
impl ::core::marker::Copy for KSPROPERTY_GENERAL {}
impl ::core::clone::Clone for KSPROPERTY_GENERAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_GENERAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_GENERAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_GENERAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_GENERAL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_HRTF3D(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_HRTF3D_PARAMS: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_HRTF3D_INITIALIZE: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_HRTF3D_FILTER_FORMAT: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(2i32);
impl ::core::marker::Copy for KSPROPERTY_HRTF3D {}
impl ::core::clone::Clone for KSPROPERTY_HRTF3D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_HRTF3D {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_HRTF3D {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_HRTF3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_HRTF3D").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_INTERLEAVEDAUDIO(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_INTERLEAVEDAUDIO_FORMATINFORMATION: KSPROPERTY_INTERLEAVEDAUDIO = KSPROPERTY_INTERLEAVEDAUDIO(1i32);
impl ::core::marker::Copy for KSPROPERTY_INTERLEAVEDAUDIO {}
impl ::core::clone::Clone for KSPROPERTY_INTERLEAVEDAUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_INTERLEAVEDAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_INTERLEAVEDAUDIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_INTERLEAVEDAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_INTERLEAVEDAUDIO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_ITD3D(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ITD3D_PARAMS: KSPROPERTY_ITD3D = KSPROPERTY_ITD3D(0i32);
impl ::core::marker::Copy for KSPROPERTY_ITD3D {}
impl ::core::clone::Clone for KSPROPERTY_ITD3D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_ITD3D {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_ITD3D {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_ITD3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_ITD3D").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_JACK(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_JACK_DESCRIPTION: KSPROPERTY_JACK = KSPROPERTY_JACK(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_JACK_DESCRIPTION2: KSPROPERTY_JACK = KSPROPERTY_JACK(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_JACK_SINK_INFO: KSPROPERTY_JACK = KSPROPERTY_JACK(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_JACK_CONTAINERID: KSPROPERTY_JACK = KSPROPERTY_JACK(4i32);
impl ::core::marker::Copy for KSPROPERTY_JACK {}
impl ::core::clone::Clone for KSPROPERTY_JACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_JACK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_JACK {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_JACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_JACK").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_MEDIAAVAILABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_MEDIAAVAILABLE").field("Earliest", &self.Earliest).field("Latest", &self.Latest).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_MEDIAAVAILABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_MEDIAAVAILABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_MEDIAAVAILABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_MEDIAAVAILABLE {}
impl ::core::default::Default for KSPROPERTY_MEDIAAVAILABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_MEDIASEEKING(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_CAPABILITIES: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_FORMATS: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_TIMEFORMAT: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_POSITION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_STOPPOSITION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_POSITIONS: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_DURATION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_AVAILABLE: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_PREROLL: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MEDIASEEKING_CONVERTTIMEFORMAT: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(9i32);
impl ::core::marker::Copy for KSPROPERTY_MEDIASEEKING {}
impl ::core::clone::Clone for KSPROPERTY_MEDIASEEKING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_MEDIASEEKING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_MEDIASEEKING {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_MEDIASEEKING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_MEDIASEEKING").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_MEMBERSHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_MEMBERSHEADER").field("MembersFlags", &self.MembersFlags).field("MembersSize", &self.MembersSize).field("MembersCount", &self.MembersCount).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_MEMBERSHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_MEMBERSHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_MEMBERSHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_MEMBERSHEADER {}
impl ::core::default::Default for KSPROPERTY_MEMBERSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_MPEG2VID(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_MODES: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_CUR_MODE: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_4_3_RECT: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_16_9_RECT: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG2VID_16_9_PANSCAN: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(4i32);
impl ::core::marker::Copy for KSPROPERTY_MPEG2VID {}
impl ::core::clone::Clone for KSPROPERTY_MPEG2VID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_MPEG2VID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_MPEG2VID {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_MPEG2VID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_MPEG2VID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MPEG4_MEDIATYPE_SD_BOX: KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES = KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES(1i32);
impl ::core::marker::Copy for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {}
impl ::core::clone::Clone for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO").field("Header", &self.Header).field("EventFilter", &self.EventFilter).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO").field("MetadataItems", &self.MetadataItems).field("Size", &self.Size).field("PTZStatus", &self.PTZStatus).field("Events", &self.Events).field("Analytics", &self.Analytics).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE_EVENTSINFO: KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE(0i32);
impl ::core::marker::Copy for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {}
impl ::core::clone::Clone for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER").field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_DISABLE: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_HOSTNTP: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_CUSTOM: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(2i32);
impl ::core::marker::Copy for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {}
impl ::core::clone::Clone for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTP: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_URI: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_NETWORKCAMERACONTROL_EVENTTOPICS_XML: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(3i32);
impl ::core::marker::Copy for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {}
impl ::core::clone::Clone for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_OVERLAYUPDATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_INTERESTS: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_CLIPLIST: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_PALETTE: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_COLORKEY: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_VIDEOPOSITION: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_DISPLAYCHANGE: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(16i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_OVERLAYUPDATE_COLORREF: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(268435456i32);
impl ::core::marker::Copy for KSPROPERTY_OVERLAYUPDATE {}
impl ::core::clone::Clone for KSPROPERTY_OVERLAYUPDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_OVERLAYUPDATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_OVERLAYUPDATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_OVERLAYUPDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_OVERLAYUPDATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_PIN(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_CINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_CTYPES: KSPROPERTY_PIN = KSPROPERTY_PIN(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_DATAFLOW: KSPROPERTY_PIN = KSPROPERTY_PIN(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_DATARANGES: KSPROPERTY_PIN = KSPROPERTY_PIN(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_DATAINTERSECTION: KSPROPERTY_PIN = KSPROPERTY_PIN(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_INTERFACES: KSPROPERTY_PIN = KSPROPERTY_PIN(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_MEDIUMS: KSPROPERTY_PIN = KSPROPERTY_PIN(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_COMMUNICATION: KSPROPERTY_PIN = KSPROPERTY_PIN(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_GLOBALCINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_NECESSARYINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_PHYSICALCONNECTION: KSPROPERTY_PIN = KSPROPERTY_PIN(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_CATEGORY: KSPROPERTY_PIN = KSPROPERTY_PIN(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_NAME: KSPROPERTY_PIN = KSPROPERTY_PIN(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_CONSTRAINEDDATARANGES: KSPROPERTY_PIN = KSPROPERTY_PIN(13i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT: KSPROPERTY_PIN = KSPROPERTY_PIN(14i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT2: KSPROPERTY_PIN = KSPROPERTY_PIN(15i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PIN_MODEDATAFORMATS: KSPROPERTY_PIN = KSPROPERTY_PIN(16i32);
impl ::core::marker::Copy for KSPROPERTY_PIN {}
impl ::core::clone::Clone for KSPROPERTY_PIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_PIN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_PIN {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_PIN").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_POSITIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_POSITIONS").field("Current", &self.Current).field("Stop", &self.Stop).field("CurrentFlags", &self.CurrentFlags).field("StopFlags", &self.StopFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_POSITIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_POSITIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_POSITIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_POSITIONS {}
impl ::core::default::Default for KSPROPERTY_POSITIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_QUALITY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_QUALITY_REPORT: KSPROPERTY_QUALITY = KSPROPERTY_QUALITY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_QUALITY_ERROR: KSPROPERTY_QUALITY = KSPROPERTY_QUALITY(1i32);
impl ::core::marker::Copy for KSPROPERTY_QUALITY {}
impl ::core::clone::Clone for KSPROPERTY_QUALITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_QUALITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_QUALITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_RTAUDIO(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_GETPOSITIONFUNCTION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_BUFFER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_HWLATENCY: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_POSITIONREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_CLOCKREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_BUFFER_WITH_NOTIFICATION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_REGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_UNREGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_QUERY_NOTIFICATION_SUPPORT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_PACKETCOUNT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_PRESENTATION_POSITION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_GETREADPACKET: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_SETWRITEPACKET: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTAUDIO_PACKETVREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(13i32);
impl ::core::marker::Copy for KSPROPERTY_RTAUDIO {}
impl ::core::clone::Clone for KSPROPERTY_RTAUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_RTAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_RTAUDIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_RTAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_RTAUDIO").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_SELECTOR_NODE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_SELECTOR_NODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_SELECTOR_NODE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SELECTOR_NODE_S {}
impl ::core::default::Default for KSPROPERTY_SELECTOR_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_SELECTOR_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_SELECTOR_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_SELECTOR_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SELECTOR_S {}
impl ::core::default::Default for KSPROPERTY_SELECTOR_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_SERIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_SERIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_SERIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SERIAL {}
impl ::core::default::Default for KSPROPERTY_SERIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_SERIALHDR {
    pub PropertySet: ::windows::core::GUID,
    pub Count: u32,
}
impl ::core::marker::Copy for KSPROPERTY_SERIALHDR {}
impl ::core::clone::Clone for KSPROPERTY_SERIALHDR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_SERIALHDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_SERIALHDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_SERIALHDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SERIALHDR {}
impl ::core::default::Default for KSPROPERTY_SERIALHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_SOUNDDETECTOR(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_SUPPORTEDPATTERNS: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_PATTERNS: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_ARMED: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_MATCHRESULT: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_RESET: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SOUNDDETECTOR_STREAMINGSUPPORT: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(6i32);
impl ::core::marker::Copy for KSPROPERTY_SOUNDDETECTOR {}
impl ::core::clone::Clone for KSPROPERTY_SOUNDDETECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_SOUNDDETECTOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_SOUNDDETECTOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_SOUNDDETECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_SOUNDDETECTOR").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSPROPERTY_SPHLI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_SPHLI").field("HLISS", &self.HLISS).field("Reserved", &self.Reserved).field("StartPTM", &self.StartPTM).field("EndPTM", &self.EndPTM).field("StartX", &self.StartX).field("StartY", &self.StartY).field("StopX", &self.StopX).field("StopY", &self.StopY).field("ColCon", &self.ColCon).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_SPHLI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_SPHLI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_SPHLI>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SPHLI {}
impl ::core::default::Default for KSPROPERTY_SPHLI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSPROPERTY_SPPAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_SPPAL").field("sppal", &self.sppal).finish()
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_SPPAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_SPPAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_SPPAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SPPAL {}
impl ::core::default::Default for KSPROPERTY_SPPAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_STEPPING_LONG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_STEPPING_LONG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_STEPPING_LONG>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_STEPPING_LONG {}
impl ::core::default::Default for KSPROPERTY_STEPPING_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_STEPPING_LONGLONG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_STEPPING_LONGLONG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_STEPPING_LONGLONG>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_STEPPING_LONGLONG {}
impl ::core::default::Default for KSPROPERTY_STEPPING_LONGLONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_STREAM(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_ALLOCATOR: KSPROPERTY_STREAM = KSPROPERTY_STREAM(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_QUALITY: KSPROPERTY_STREAM = KSPROPERTY_STREAM(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_DEGRADATION: KSPROPERTY_STREAM = KSPROPERTY_STREAM(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_MASTERCLOCK: KSPROPERTY_STREAM = KSPROPERTY_STREAM(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_TIMEFORMAT: KSPROPERTY_STREAM = KSPROPERTY_STREAM(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_PRESENTATIONTIME: KSPROPERTY_STREAM = KSPROPERTY_STREAM(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_PRESENTATIONEXTENT: KSPROPERTY_STREAM = KSPROPERTY_STREAM(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_FRAMETIME: KSPROPERTY_STREAM = KSPROPERTY_STREAM(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_RATECAPABILITY: KSPROPERTY_STREAM = KSPROPERTY_STREAM(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_RATE: KSPROPERTY_STREAM = KSPROPERTY_STREAM(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAM_PIPE_ID: KSPROPERTY_STREAM = KSPROPERTY_STREAM(10i32);
impl ::core::marker::Copy for KSPROPERTY_STREAM {}
impl ::core::clone::Clone for KSPROPERTY_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_STREAM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_STREAM {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_STREAM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_STREAMINTERFACE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_STREAMINTERFACE_HEADERSIZE: KSPROPERTY_STREAMINTERFACE = KSPROPERTY_STREAMINTERFACE(0i32);
impl ::core::marker::Copy for KSPROPERTY_STREAMINTERFACE {}
impl ::core::clone::Clone for KSPROPERTY_STREAMINTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_STREAMINTERFACE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_STREAMINTERFACE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_STREAMINTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_STREAMINTERFACE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_TELEPHONY_CONTROL(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_PROVIDERID: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_CALLINFO: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_CALLCONTROL: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_PROVIDERCHANGE: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_CALLHOLD: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_MUTE_TX: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(5i32);
impl ::core::marker::Copy for KSPROPERTY_TELEPHONY_CONTROL {}
impl ::core::clone::Clone for KSPROPERTY_TELEPHONY_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_TELEPHONY_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_TELEPHONY_CONTROL {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_TELEPHONY_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TELEPHONY_CONTROL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_TELEPHONY_TOPOLOGY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_ENDPOINTIDPAIR: KSPROPERTY_TELEPHONY_TOPOLOGY = KSPROPERTY_TELEPHONY_TOPOLOGY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TELEPHONY_VOLUME: KSPROPERTY_TELEPHONY_TOPOLOGY = KSPROPERTY_TELEPHONY_TOPOLOGY(1i32);
impl ::core::marker::Copy for KSPROPERTY_TELEPHONY_TOPOLOGY {}
impl ::core::clone::Clone for KSPROPERTY_TELEPHONY_TOPOLOGY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_TELEPHONY_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_TELEPHONY_TOPOLOGY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_TELEPHONY_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TELEPHONY_TOPOLOGY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_TIMECODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TIMECODE_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_ATN_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_RTC_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(2i32);
impl ::core::marker::Copy for KSPROPERTY_TIMECODE {}
impl ::core::clone::Clone for KSPROPERTY_TIMECODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_TIMECODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_TIMECODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_TIMECODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TIMECODE").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TIMECODE_NODE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TIMECODE_NODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TIMECODE_NODE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TIMECODE_NODE_S {}
impl ::core::default::Default for KSPROPERTY_TIMECODE_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TIMECODE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TIMECODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TIMECODE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TIMECODE_S {}
impl ::core::default::Default for KSPROPERTY_TIMECODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_TOPOLOGY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGY_CATEGORIES: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGY_NODES: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGY_CONNECTIONS: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGY_NAME: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(3i32);
impl ::core::marker::Copy for KSPROPERTY_TOPOLOGY {}
impl ::core::clone::Clone for KSPROPERTY_TOPOLOGY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_TOPOLOGY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TOPOLOGY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_TOPOLOGYNODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGYNODE_ENABLE: KSPROPERTY_TOPOLOGYNODE = KSPROPERTY_TOPOLOGYNODE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TOPOLOGYNODE_RESET: KSPROPERTY_TOPOLOGYNODE = KSPROPERTY_TOPOLOGYNODE(2i32);
impl ::core::marker::Copy for KSPROPERTY_TOPOLOGYNODE {}
impl ::core::clone::Clone for KSPROPERTY_TOPOLOGYNODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_TOPOLOGYNODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_TOPOLOGYNODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_TOPOLOGYNODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TOPOLOGYNODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_TUNER(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE: KSPROPERTY_TUNER = KSPROPERTY_TUNER(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_STANDARD: KSPROPERTY_TUNER = KSPROPERTY_TUNER(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_FREQUENCY: KSPROPERTY_TUNER = KSPROPERTY_TUNER(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_INPUT: KSPROPERTY_TUNER = KSPROPERTY_TUNER(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_STATUS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_IF_MEDIUM: KSPROPERTY_TUNER = KSPROPERTY_TUNER(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_SCAN_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_SCAN_STATUS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_STANDARD_MODE: KSPROPERTY_TUNER = KSPROPERTY_TUNER(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(11i32);
impl ::core::marker::Copy for KSPROPERTY_TUNER {}
impl ::core::clone::Clone for KSPROPERTY_TUNER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_TUNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TUNER").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_FREQUENCY_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_FREQUENCY_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_FREQUENCY_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_FREQUENCY_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_FREQUENCY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_IF_MEDIUM_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_IF_MEDIUM_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_IF_MEDIUM_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_IF_MEDIUM_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_IF_MEDIUM_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_INPUT_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_INPUT_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_INPUT_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_INPUT_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_INPUT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_TUNER_MODES(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_TV: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_FM_RADIO: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_AM_RADIO: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_DSS: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TUNER_MODE_ATSC: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(16i32);
impl ::core::marker::Copy for KSPROPERTY_TUNER_MODES {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_MODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_MODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_TUNER_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TUNER_MODES").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_MODE_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_MODE_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_MODE_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_MODE_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_MODE_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_MODE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_MODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_MODE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_MODE_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub NetworkType: ::windows::core::GUID,
    pub BufferSize: u32,
    pub NetworkTunerCapabilities: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {}
impl ::core::clone::Clone for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_SCAN_CAPS_S {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_SCAN_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_SCAN_CAPS_S>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_TUNER_SCAN_CAPS_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_TUNER_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_SCAN_STATUS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_SCAN_STATUS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_SCAN_STATUS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_SCAN_STATUS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_SCAN_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_STANDARD_MODE_S {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_STANDARD_MODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_STANDARD_MODE_S>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_TUNER_STANDARD_MODE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_TUNER_STANDARD_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_STANDARD_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_STANDARD_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_STANDARD_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_STANDARD_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_STANDARD_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TUNER_STATUS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TUNER_STATUS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TUNER_STATUS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TUNER_STATUS_S {}
impl ::core::default::Default for KSPROPERTY_TUNER_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TVAUDIO_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TVAUDIO_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TVAUDIO_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TVAUDIO_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_TVAUDIO_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_TVAUDIO_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_TVAUDIO_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_TVAUDIO_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_TVAUDIO_S {}
impl ::core::default::Default for KSPROPERTY_TVAUDIO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VBICAP(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICAP_PROPERTIES_PROTECTION: KSPROPERTY_VBICAP = KSPROPERTY_VBICAP(1i32);
impl ::core::marker::Copy for KSPROPERTY_VBICAP {}
impl ::core::clone::Clone for KSPROPERTY_VBICAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VBICAP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICAP {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VBICAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VBICAP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VBICODECFILTERING(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VBICODECFILTERING_STATISTICS: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(5i32);
impl ::core::marker::Copy for KSPROPERTY_VBICODECFILTERING {}
impl ::core::clone::Clone for KSPROPERTY_VBICODECFILTERING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VBICODECFILTERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VBICODECFILTERING").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_SCANLINES_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_CAMERACONTROL(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PAN: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_TILT: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_ROLL: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_ZOOM: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_IRIS: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FOCUS: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_SCANMODE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PRIVACY: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PANTILT: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PAN_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_TILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_ROLL_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_ZOOM_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(13i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(14i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_IRIS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(15i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FOCUS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(16i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_PANTILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(17i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(18i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CAMERACONTROL_AUTO_EXPOSURE_PRIORITY: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(19i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_CAMERACONTROL {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_CAMERACONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_CAMERACONTROL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_CAMERACONTROL {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_CAMERACONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_CAMERACONTROL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_CROSSBAR(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_CAPS: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_PININFO: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_CAN_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CROSSBAR_INPUT_ACTIVE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(4i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_CROSSBAR {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_CROSSBAR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_CROSSBAR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_CROSSBAR {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_CROSSBAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_CROSSBAR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_DROPPEDFRAMES(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DROPPEDFRAMES_CURRENT: KSPROPERTY_VIDCAP_DROPPEDFRAMES = KSPROPERTY_VIDCAP_DROPPEDFRAMES(0i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_DROPPEDFRAMES {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_DROPPEDFRAMES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_DROPPEDFRAMES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_DROPPEDFRAMES {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_DROPPEDFRAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_DROPPEDFRAMES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_SELECTOR(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SELECTOR_SOURCE_NODE_ID: KSPROPERTY_VIDCAP_SELECTOR = KSPROPERTY_VIDCAP_SELECTOR(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_SELECTOR_NUM_SOURCES: KSPROPERTY_VIDCAP_SELECTOR = KSPROPERTY_VIDCAP_SELECTOR(1i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_SELECTOR {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_SELECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_SELECTOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_SELECTOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_SELECTOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_TVAUDIO(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TVAUDIO_CAPS: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TVAUDIO_MODE: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_TVAUDIO_CURRENTLY_AVAILABLE_MODES: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(2i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_TVAUDIO {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_TVAUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_TVAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_TVAUDIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_TVAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_TVAUDIO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_GETINFO: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_KEYFRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_PFRAMES_PER_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_QUALITY: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_FRAME_SIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCOMPRESSION_WINDOWSIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(6i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEOCOMPRESSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_VIDEOCONTROL(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCONTROL_CAPS: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCONTROL_FRAME_RATES: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOCONTROL_MODE: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(3i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_VIDEOCONTROL {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_VIDEOCONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEOCONTROL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_VIDEOCONTROL {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEOCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEOCONTROL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_VIDEODECODER(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_CAPS: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_STANDARD: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_STATUS: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_OUTPUT_ENABLE: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_VCR_TIMING: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEODECODER_STATUS2: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(5i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_VIDEODECODER {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_VIDEODECODER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEODECODER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_VIDEODECODER {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEODECODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEODECODER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_VIDEOENCODER(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOENCODER_CAPS: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOENCODER_STANDARD: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOENCODER_COPYPROTECTION: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOENCODER_CC_ENABLE: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(3i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_VIDEOENCODER {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_VIDEOENCODER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEOENCODER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_VIDEOENCODER {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEOENCODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEOENCODER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDCAP_VIDEOPROCAMP(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_BRIGHTNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_CONTRAST: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_HUE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_SATURATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_SHARPNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_GAMMA: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_COLORENABLE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_BACKLIGHT_COMPENSATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_GAIN: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER_LIMIT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE_COMPONENT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VIDEOPROCAMP_POWERLINE_FREQUENCY: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(13i32);
impl ::core::marker::Copy for KSPROPERTY_VIDCAP_VIDEOPROCAMP {}
impl ::core::clone::Clone for KSPROPERTY_VIDCAP_VIDEOPROCAMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEOPROCAMP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDCAP_VIDEOPROCAMP {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEOPROCAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEOPROCAMP").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOCOMPRESSION_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCOMPRESSION_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOCOMPRESSION_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCOMPRESSION_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOCOMPRESSION_S1>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCOMPRESSION_S1 {}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOCONTROL_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCONTROL_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOCONTROL_MODE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOCONTROL_MODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOCONTROL_MODE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOCONTROL_MODE_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEODECODER_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEODECODER_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEODECODER_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEODECODER_CAPS_S {}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEODECODER_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEODECODER_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEODECODER_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEODECODER_S {}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEODECODER_STATUS2_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEODECODER_STATUS2_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEODECODER_STATUS2_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEODECODER_STATUS2_S {}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_STATUS2_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEODECODER_STATUS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEODECODER_STATUS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEODECODER_STATUS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEODECODER_STATUS_S {}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOENCODER_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOENCODER_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOENCODER_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOENCODER_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOENCODER_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOPROCAMP_NODE_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOPROCAMP_NODE_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOPROCAMP_NODE_S2>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOPROCAMP_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOPROCAMP_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOPROCAMP_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOPROCAMP_S {}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDEOPROCAMP_S2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSPROPERTY_VIDEOPROCAMP_S2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSPROPERTY_VIDEOPROCAMP_S2>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSPROPERTY_VIDEOPROCAMP_S2 {}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VIDMEM_TRANSPORT(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_DISPLAY_ADAPTER_GUID: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_PREFERRED_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_CURRENT_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_MAP_CAPTURE_HANDLE_TO_VRAM_ADDRESS: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(4i32);
impl ::core::marker::Copy for KSPROPERTY_VIDMEM_TRANSPORT {}
impl ::core::clone::Clone for KSPROPERTY_VIDMEM_TRANSPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VIDMEM_TRANSPORT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VIDMEM_TRANSPORT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VIDMEM_TRANSPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDMEM_TRANSPORT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_VPCONFIG(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_NUMCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_GETCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_SETCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_VPDATAINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_MAXPIXELRATE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_INFORMVPINPUT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_NUMVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_GETVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_SETVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_INVERTPOLARITY: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_DECIMATIONCAPABILITY: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_SCALEFACTOR: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_DDRAWHANDLE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_VIDEOPORTID: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(13i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_DDRAWSURFACEHANDLE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(14i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_VPCONFIG_SURFACEPARAMS: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(15i32);
impl ::core::marker::Copy for KSPROPERTY_VPCONFIG {}
impl ::core::clone::Clone for KSPROPERTY_VPCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_VPCONFIG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_VPCONFIG {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_VPCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VPCONFIG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSPROPERTY_WAVE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_COMPATIBLE_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_INPUT_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_OUTPUT_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_BUFFER: KSPROPERTY_WAVE = KSPROPERTY_WAVE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_FREQUENCY: KSPROPERTY_WAVE = KSPROPERTY_WAVE(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_VOLUME: KSPROPERTY_WAVE = KSPROPERTY_WAVE(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_PAN: KSPROPERTY_WAVE = KSPROPERTY_WAVE(6i32);
impl ::core::marker::Copy for KSPROPERTY_WAVE {}
impl ::core::clone::Clone for KSPROPERTY_WAVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSPROPERTY_WAVE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSPROPERTY_WAVE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSPROPERTY_WAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_WAVE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSPROPERTY_WAVE_QUEUED_POSITION: u32 = 1u32;
pub const KSPROPSETID_AC3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfabe720_6e1f_11d0_bcf2_444553540000);
pub const KSPROPSETID_Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45ffaaa0_6e1b_11d0_bcf2_444553540000);
pub const KSPROPSETID_AudioBufferDuration: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e73c07f_23cc_4955_a7ea_3da502496290);
pub const KSPROPSETID_AudioDecoderOut: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ca6e020_43bd_11d0_bd6a_003505c103a9);
pub const KSPROPSETID_AudioEngine: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a2f82dc_886f_4baa_9eb4_082b9025c536);
pub const KSPROPSETID_AudioModule: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc034fdb0_ff75_47c8_aa3c_ee46716b50c6);
pub const KSPROPSETID_AudioPosture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3fb7b0d_474e_4f51_a379_51282dd4fa8f);
pub const KSPROPSETID_AudioResourceManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0b305e1_b2cc_484c_8f23_e5d28ad9cf88);
pub const KSPROPSETID_AudioSignalProcessing: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f67b528_30c9_40de_b2fb_859ddd1f3470);
pub const KSPROPSETID_Bibliographic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07ba150e_e2b1_11d0_ac17_00a0c9223196);
pub const KSPROPSETID_BtAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa06c40_b8f6_4c7e_8556_e8c33a12e54d);
pub const KSPROPSETID_Clock: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf12a4c0_ac17_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_Connection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d58c920_ac9b_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_CopyProt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e8a0a40_6aef_11d0_9ed0_00a024ca19b3);
pub const KSPROPSETID_Cyclic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ffeaea0_2bee_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_DirectSound3DBuffer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x437b3411_d060_11d0_8583_00c04fd9baf3);
pub const KSPROPSETID_DirectSound3DListener: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x437b3414_d060_11d0_8583_00c04fd9baf3);
pub const KSPROPSETID_DrmAudioStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f2c8ddd_4198_4fac_ba29_61bb05b7de06);
pub const KSPROPSETID_DvdSubPic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac390460_43af_11d0_bd6a_003505c103a9);
pub const KSPROPSETID_FMRXControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x947bba3a_e8ee_4786_90c4_8428185f05be);
pub const KSPROPSETID_FMRXTopology: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c46ce8f_dc2d_4204_9dc9_f58963366563);
pub const KSPROPSETID_General: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1464eda5_6a8f_11d1_9aa7_00a0c9223196);
pub const KSPROPSETID_Hrtf3d: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb66decb0_a083_11d0_851e_00c04fd9baf3);
pub const KSPROPSETID_InterleavedAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9ebe550_d619_4c0a_976b_7062322b3006);
pub const KSPROPSETID_Itd3d: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6429f090_9fd9_11d0_a75b_00a0c90365e3);
pub const KSPROPSETID_Jack: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4509f757_2d46_4637_8e62_ce7db944f57b);
pub const KSPROPSETID_MPEG4_MediaType_Attributes: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff6c4bfa_07a9_4c7b_a237_672f9d68065f);
pub const KSPROPSETID_MediaSeeking: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee904f0c_d09b_11d0_abe9_00a0c9223196);
pub const KSPROPSETID_MemoryTransport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a3d1c5d_5243_4819_9ed0_aee8044cee2b);
pub const KSPROPSETID_Mpeg2Vid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8e11b60_0cc9_11d0_bd69_003505c103a9);
pub const KSPROPSETID_OverlayUpdate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x490ea5cf_7681_11d1_a21c_00a0c9223196);
pub const KSPROPSETID_Pin: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c134960_51ad_11cf_878a_94f801c10000);
pub const KSPROPSETID_PinMDLCacheClearProp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd718a7b_97fc_40c7_88ce_d3ff06f55b16);
pub const KSPROPSETID_Quality: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd16ad380_ac1a_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_RtAudio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa855a48c_2f78_4729_9051_1968746b9eef);
pub const KSPROPSETID_SoundDetector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x113c425e_fd17_4057_b422_ed4074f1afdf);
pub const KSPROPSETID_SoundDetector2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe07e322_450c_4bd5_84ca_a948500ea6aa);
pub const KSPROPSETID_Stream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65aaba60_98ae_11cf_a10d_0020afd156e4);
pub const KSPROPSETID_StreamAllocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf6e4342_ec87_11cf_a130_0020afd156e4);
pub const KSPROPSETID_StreamInterface: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fdd8ee1_9cd3_11d0_82aa_0000f822fe8a);
pub const KSPROPSETID_TSRateChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa503c5c0_1d1d_11d1_ad80_444553540000);
pub const KSPROPSETID_TelephonyControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6df7eb1_d099_489f_a6a0_c0106f0887a7);
pub const KSPROPSETID_TelephonyTopology: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabf25c7e_0e64_4e32_b190_d0f6d7c53e97);
pub const KSPROPSETID_Topology: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x720d4ac0_7533_11d0_a5d6_28db04c10000);
pub const KSPROPSETID_TopologyNode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45ffaaa1_6e1b_11d0_bcf2_444553540000);
pub const KSPROPSETID_VBICAP_PROPERTIES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf162c607_7b35_496f_ad7f_2dca3b46b718);
pub const KSPROPSETID_VBICodecFiltering: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcafeb0ca_8715_11d0_bd6a_0035c0edbabe);
pub const KSPROPSETID_VPConfig: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc29a660_30e3_11d0_9e69_00c04fd7c15b);
pub const KSPROPSETID_VPVBIConfig: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec529b00_1a1f_11d1_bad9_00609744111a);
pub const KSPROPSETID_VramCapture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe73face3_2880_4902_b799_88d0cd634e0f);
pub const KSPROPSETID_Wave: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x924e54b0_630f_11cf_ada7_08003e30494a);
pub const KSPROPTYPESETID_General: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97e99ba0_bdea_11cf_a5d6_28db04c10000);
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
unsafe impl ::windows::core::Abi for KSP_NODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSP_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSP_NODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSP_NODE {}
impl ::core::default::Default for KSP_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSP_PIN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSP_PIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSP_PIN>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSP_PIN {}
impl ::core::default::Default for KSP_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSP_PIN_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSP_PIN_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSP_PIN_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSP_PIN_0 {}
impl ::core::default::Default for KSP_PIN_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSP_TIMEFORMAT {
    pub Property: KSIDENTIFIER,
    pub SourceFormat: ::windows::core::GUID,
    pub TargetFormat: ::windows::core::GUID,
    pub Time: i64,
}
impl ::core::marker::Copy for KSP_TIMEFORMAT {}
impl ::core::clone::Clone for KSP_TIMEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KSP_TIMEFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSP_TIMEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSP_TIMEFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSP_TIMEFORMAT {}
impl ::core::default::Default for KSP_TIMEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSQUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSQUALITY").field("Context", &self.Context).field("Proportion", &self.Proportion).field("DeltaTime", &self.DeltaTime).finish()
    }
}
unsafe impl ::windows::core::Abi for KSQUALITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSQUALITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSQUALITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSQUALITY {}
impl ::core::default::Default for KSQUALITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSQUALITY_MANAGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSQUALITY_MANAGER").field("QualityManager", &self.QualityManager).field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSQUALITY_MANAGER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSQUALITY_MANAGER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSQUALITY_MANAGER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSQUALITY_MANAGER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSQUALITY_MANAGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSQUERYBUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSQUERYBUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSQUERYBUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSQUERYBUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSQUERYBUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRATE {}
impl ::core::default::Default for KSRATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRATE_CAPABILITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRATE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRATE_CAPABILITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRATE_CAPABILITY {}
impl ::core::default::Default for KSRATE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSRELATIVEEVENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRELATIVEEVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRELATIVEEVENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRELATIVEEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRELATIVEEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSRELATIVEEVENT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRELATIVEEVENT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRELATIVEEVENT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRELATIVEEVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRELATIVEEVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRELATIVEEVENT_FLAG_HANDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRELATIVEEVENT_FLAG_POINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSRESET(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRESET_BEGIN: KSRESET = KSRESET(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSRESET_END: KSRESET = KSRESET(1i32);
impl ::core::marker::Copy for KSRESET {}
impl ::core::clone::Clone for KSRESET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSRESET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSRESET {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSRESET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSRESET").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KSRESOLUTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRESOLUTION").field("Granularity", &self.Granularity).field("Error", &self.Error).finish()
    }
}
unsafe impl ::windows::core::Abi for KSRESOLUTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRESOLUTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRESOLUTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRESOLUTION {}
impl ::core::default::Default for KSRESOLUTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_BUFFER").field("BufferAddress", &self.BufferAddress).field("ActualBufferSize", &self.ActualBufferSize).field("CallMemoryBarrier", &self.CallMemoryBarrier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSRTAUDIO_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_BUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_BUFFER32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_BUFFER32").field("BufferAddress", &self.BufferAddress).field("ActualBufferSize", &self.ActualBufferSize).field("CallMemoryBarrier", &self.CallMemoryBarrier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSRTAUDIO_BUFFER32 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_BUFFER32>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_BUFFER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRTAUDIO_BUFFER_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_BUFFER_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER_PROPERTY {}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRTAUDIO_BUFFER_PROPERTY32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER_PROPERTY32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_BUFFER_PROPERTY32>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER_PROPERTY32 {}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_GETREADPACKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_GETREADPACKET_INFO").field("PacketNumber", &self.PacketNumber).field("Flags", &self.Flags).field("PerformanceCounterValue", &self.PerformanceCounterValue).field("MoreData", &self.MoreData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSRTAUDIO_GETREADPACKET_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_GETREADPACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_GETREADPACKET_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_GETREADPACKET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_GETREADPACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSRTAUDIO_HWLATENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_HWLATENCY").field("FifoSize", &self.FifoSize).field("ChipsetDelay", &self.ChipsetDelay).field("CodecDelay", &self.CodecDelay).finish()
    }
}
unsafe impl ::windows::core::Abi for KSRTAUDIO_HWLATENCY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWLATENCY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_HWLATENCY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWLATENCY {}
impl ::core::default::Default for KSRTAUDIO_HWLATENCY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSRTAUDIO_HWREGISTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_HWREGISTER").field("Register", &self.Register).field("Width", &self.Width).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).field("Accuracy", &self.Accuracy).finish()
    }
}
unsafe impl ::windows::core::Abi for KSRTAUDIO_HWREGISTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_HWREGISTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER {}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSRTAUDIO_HWREGISTER32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_HWREGISTER32").field("Register", &self.Register).field("Width", &self.Width).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).field("Accuracy", &self.Accuracy).finish()
    }
}
unsafe impl ::windows::core::Abi for KSRTAUDIO_HWREGISTER32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_HWREGISTER32>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER32 {}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRTAUDIO_HWREGISTER_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_HWREGISTER_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER_PROPERTY {}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_HWREGISTER_PROPERTY32>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER_PROPERTY32 {}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {}
impl ::core::default::Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSRTAUDIO_PACKETVREGISTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_PACKETVREGISTER").field("CompletedPacketCount", &self.CompletedPacketCount).field("CompletedPacketQPC", &self.CompletedPacketQPC).field("CompletedPacketHash", &self.CompletedPacketHash).finish()
    }
}
unsafe impl ::windows::core::Abi for KSRTAUDIO_PACKETVREGISTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_PACKETVREGISTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_PACKETVREGISTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_PACKETVREGISTER {}
impl ::core::default::Default for KSRTAUDIO_PACKETVREGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_PACKETVREGISTER_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_PACKETVREGISTER_PROPERTY {}
impl ::core::default::Default for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_SETWRITEPACKET_INFO").field("PacketNumber", &self.PacketNumber).field("Flags", &self.Flags).field("EosPacketLength", &self.EosPacketLength).finish()
    }
}
unsafe impl ::windows::core::Abi for KSRTAUDIO_SETWRITEPACKET_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSRTAUDIO_SETWRITEPACKET_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_SETWRITEPACKET_INFO {}
impl ::core::default::Default for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSSOUNDDETECTORPROPERTY {
    pub Property: KSIDENTIFIER,
    pub EventId: ::windows::core::GUID,
}
impl ::core::marker::Copy for KSSOUNDDETECTORPROPERTY {}
impl ::core::clone::Clone for KSSOUNDDETECTORPROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KSSOUNDDETECTORPROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSSOUNDDETECTORPROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSOUNDDETECTORPROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSSOUNDDETECTORPROPERTY {}
impl ::core::default::Default for KSSOUNDDETECTORPROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTATE_STOP: KSSTATE = KSSTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTATE_ACQUIRE: KSSTATE = KSSTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTATE_PAUSE: KSSTATE = KSSTATE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KSSTATE_RUN: KSSTATE = KSSTATE(3i32);
impl ::core::marker::Copy for KSSTATE {}
impl ::core::clone::Clone for KSSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KSSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KSSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KSSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSSTATE").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KSSTREAMALLOCATOR_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSSTREAMALLOCATOR_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAMALLOCATOR_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSSTREAMALLOCATOR_STATUS {}
impl ::core::default::Default for KSSTREAMALLOCATOR_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSSTREAMALLOCATOR_STATUS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSSTREAMALLOCATOR_STATUS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAMALLOCATOR_STATUS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSSTREAMALLOCATOR_STATUS_EX {}
impl ::core::default::Default for KSSTREAMALLOCATOR_STATUS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for KSSTREAM_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSSTREAM_HEADER").field("Size", &self.Size).field("TypeSpecificFlags", &self.TypeSpecificFlags).field("PresentationTime", &self.PresentationTime).field("Duration", &self.Duration).field("FrameExtent", &self.FrameExtent).field("DataUsed", &self.DataUsed).field("Data", &self.Data).field("OptionsFlags", &self.OptionsFlags).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for KSSTREAM_HEADER {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for KSSTREAM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAM_HEADER>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for KSSTREAM_HEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for KSSTREAM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for KSSTREAM_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSSTREAM_HEADER").field("Size", &self.Size).field("TypeSpecificFlags", &self.TypeSpecificFlags).field("PresentationTime", &self.PresentationTime).field("Duration", &self.Duration).field("FrameExtent", &self.FrameExtent).field("DataUsed", &self.DataUsed).field("Data", &self.Data).field("OptionsFlags", &self.OptionsFlags).finish()
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for KSSTREAM_HEADER {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for KSSTREAM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAM_HEADER>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for KSSTREAM_HEADER {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KSSTREAM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSSTREAM_METADATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSSTREAM_METADATA_INFO").field("BufferSize", &self.BufferSize).field("UsedSize", &self.UsedSize).field("Data", &self.Data).field("SystemVa", &self.SystemVa).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSSTREAM_METADATA_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSSTREAM_METADATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAM_METADATA_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSSTREAM_METADATA_INFO {}
impl ::core::default::Default for KSSTREAM_METADATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSSTREAM_UVC_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSSTREAM_UVC_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAM_UVC_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSSTREAM_UVC_METADATA {}
impl ::core::default::Default for KSSTREAM_UVC_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAM_UVC_METADATATYPE_TIMESTAMP>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {}
impl ::core::default::Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {}
impl ::core::default::Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {}
impl ::core::default::Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSTELEPHONY_CALLCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTELEPHONY_CALLCONTROL").field("CallType", &self.CallType).field("CallControlOp", &self.CallControlOp).finish()
    }
}
unsafe impl ::windows::core::Abi for KSTELEPHONY_CALLCONTROL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSTELEPHONY_CALLCONTROL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSTELEPHONY_CALLCONTROL>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_CALLCONTROL {}
impl ::core::default::Default for KSTELEPHONY_CALLCONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSTELEPHONY_CALLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTELEPHONY_CALLINFO").field("CallType", &self.CallType).field("CallState", &self.CallState).finish()
    }
}
unsafe impl ::windows::core::Abi for KSTELEPHONY_CALLINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSTELEPHONY_CALLINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSTELEPHONY_CALLINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_CALLINFO {}
impl ::core::default::Default for KSTELEPHONY_CALLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSTELEPHONY_PROVIDERCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTELEPHONY_PROVIDERCHANGE").field("CallType", &self.CallType).field("ProviderChangeOp", &self.ProviderChangeOp).finish()
    }
}
unsafe impl ::windows::core::Abi for KSTELEPHONY_PROVIDERCHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSTELEPHONY_PROVIDERCHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSTELEPHONY_PROVIDERCHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_PROVIDERCHANGE {}
impl ::core::default::Default for KSTELEPHONY_PROVIDERCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTIME").field("Time", &self.Time).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
unsafe impl ::windows::core::Abi for KSTIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSTIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSTIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSTIME {}
impl ::core::default::Default for KSTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KSTIME_FORMAT_BYTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b785571_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_FIELD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b785573_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_FRAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b785570_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_MEDIA_TIME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b785574_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_SAMPLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b785572_8c82_11cf_bc0c_00aa00ac74f6);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KSTOPOLOGY {
    pub CategoriesCount: u32,
    pub Categories: *const ::windows::core::GUID,
    pub TopologyNodesCount: u32,
    pub TopologyNodes: *const ::windows::core::GUID,
    pub TopologyConnectionsCount: u32,
    pub TopologyConnections: *const KSTOPOLOGY_CONNECTION,
    pub TopologyNodesNames: *const ::windows::core::GUID,
    pub Reserved: u32,
}
impl ::core::marker::Copy for KSTOPOLOGY {}
impl ::core::clone::Clone for KSTOPOLOGY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KSTOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTOPOLOGY").field("CategoriesCount", &self.CategoriesCount).field("Categories", &self.Categories).field("TopologyNodesCount", &self.TopologyNodesCount).field("TopologyNodes", &self.TopologyNodes).field("TopologyConnectionsCount", &self.TopologyConnectionsCount).field("TopologyConnections", &self.TopologyConnections).field("TopologyNodesNames", &self.TopologyNodesNames).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSTOPOLOGY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSTOPOLOGY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSTOPOLOGY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY {}
impl ::core::default::Default for KSTOPOLOGY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSTOPOLOGY_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTOPOLOGY_CONNECTION").field("FromNode", &self.FromNode).field("FromNodePin", &self.FromNodePin).field("ToNode", &self.ToNode).field("ToNodePin", &self.ToNodePin).finish()
    }
}
unsafe impl ::windows::core::Abi for KSTOPOLOGY_CONNECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSTOPOLOGY_CONNECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_CONNECTION {}
impl ::core::default::Default for KSTOPOLOGY_CONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSTOPOLOGY_ENDPOINTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTOPOLOGY_ENDPOINTID").field("TopologyName", &self.TopologyName).field("PinId", &self.PinId).finish()
    }
}
unsafe impl ::windows::core::Abi for KSTOPOLOGY_ENDPOINTID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_ENDPOINTID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSTOPOLOGY_ENDPOINTID>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_ENDPOINTID {}
impl ::core::default::Default for KSTOPOLOGY_ENDPOINTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTOPOLOGY_ENDPOINTIDPAIR").field("RenderEndpoint", &self.RenderEndpoint).field("CaptureEndpoint", &self.CaptureEndpoint).finish()
    }
}
unsafe impl ::windows::core::Abi for KSTOPOLOGY_ENDPOINTIDPAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSTOPOLOGY_ENDPOINTIDPAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_ENDPOINTIDPAIR {}
impl ::core::default::Default for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSVPMAXPIXELRATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSVPMAXPIXELRATE").field("Size", &self.Size).field("MaxPixelsPerSecond", &self.MaxPixelsPerSecond).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KSVPMAXPIXELRATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSVPMAXPIXELRATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSVPMAXPIXELRATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSVPMAXPIXELRATE {}
impl ::core::default::Default for KSVPMAXPIXELRATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KSVPSIZE_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSVPSIZE_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSVPSIZE_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSVPSIZE_PROP {}
impl ::core::default::Default for KSVPSIZE_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSVPSURFACEPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSVPSURFACEPARAMS").field("dwPitch", &self.dwPitch).field("dwXOrigin", &self.dwXOrigin).field("dwYOrigin", &self.dwYOrigin).finish()
    }
}
unsafe impl ::windows::core::Abi for KSVPSURFACEPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSVPSURFACEPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSVPSURFACEPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSVPSURFACEPARAMS {}
impl ::core::default::Default for KSVPSURFACEPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KSWAVETABLE_WAVE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSWAVETABLE_WAVE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSWAVETABLE_WAVE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSWAVETABLE_WAVE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSWAVETABLE_WAVE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSWAVE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_BUFFER").field("Attributes", &self.Attributes).field("BufferSize", &self.BufferSize).field("BufferAddress", &self.BufferAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for KSWAVE_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSWAVE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSWAVE_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSWAVE_BUFFER {}
impl ::core::default::Default for KSWAVE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSWAVE_COMPATCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_COMPATCAPS").field("ulDeviceType", &self.ulDeviceType).finish()
    }
}
unsafe impl ::windows::core::Abi for KSWAVE_COMPATCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSWAVE_COMPATCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSWAVE_COMPATCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSWAVE_COMPATCAPS {}
impl ::core::default::Default for KSWAVE_COMPATCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSWAVE_INPUT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_INPUT_CAPABILITIES").field("MaximumChannelsPerConnection", &self.MaximumChannelsPerConnection).field("MinimumBitsPerSample", &self.MinimumBitsPerSample).field("MaximumBitsPerSample", &self.MaximumBitsPerSample).field("MinimumSampleFrequency", &self.MinimumSampleFrequency).field("MaximumSampleFrequency", &self.MaximumSampleFrequency).field("TotalConnections", &self.TotalConnections).field("ActiveConnections", &self.ActiveConnections).finish()
    }
}
unsafe impl ::windows::core::Abi for KSWAVE_INPUT_CAPABILITIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSWAVE_INPUT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSWAVE_INPUT_CAPABILITIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSWAVE_INPUT_CAPABILITIES {}
impl ::core::default::Default for KSWAVE_INPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSWAVE_OUTPUT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_OUTPUT_CAPABILITIES")
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
unsafe impl ::windows::core::Abi for KSWAVE_OUTPUT_CAPABILITIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSWAVE_OUTPUT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSWAVE_OUTPUT_CAPABILITIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSWAVE_OUTPUT_CAPABILITIES {}
impl ::core::default::Default for KSWAVE_OUTPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KSWAVE_VOLUME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_VOLUME").field("LeftAttenuation", &self.LeftAttenuation).field("RightAttenuation", &self.RightAttenuation).finish()
    }
}
unsafe impl ::windows::core::Abi for KSWAVE_VOLUME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KSWAVE_VOLUME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KSWAVE_VOLUME>()) == 0 }
    }
}
impl ::core::cmp::Eq for KSWAVE_VOLUME {}
impl ::core::default::Default for KSWAVE_VOLUME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_AMPixAspectRatio(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PixAspectRatio_NTSC4x3: KS_AMPixAspectRatio = KS_AMPixAspectRatio(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PixAspectRatio_NTSC16x9: KS_AMPixAspectRatio = KS_AMPixAspectRatio(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PixAspectRatio_PAL4x3: KS_AMPixAspectRatio = KS_AMPixAspectRatio(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PixAspectRatio_PAL16x9: KS_AMPixAspectRatio = KS_AMPixAspectRatio(3i32);
impl ::core::marker::Copy for KS_AMPixAspectRatio {}
impl ::core::clone::Clone for KS_AMPixAspectRatio {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_AMPixAspectRatio {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_AMPixAspectRatio {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_AMPixAspectRatio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AMPixAspectRatio").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_AMVPDATAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AMVPDATAINFO")
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
unsafe impl ::windows::core::Abi for KS_AMVPDATAINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_AMVPDATAINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_AMVPDATAINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_AMVPDATAINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_AMVPDATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_AMVPDIMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AMVPDIMINFO").field("dwFieldWidth", &self.dwFieldWidth).field("dwFieldHeight", &self.dwFieldHeight).field("dwVBIWidth", &self.dwVBIWidth).field("dwVBIHeight", &self.dwVBIHeight).field("rcValidRegion", &self.rcValidRegion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_AMVPDIMINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_AMVPDIMINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_AMVPDIMINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_AMVPDIMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_AMVPDIMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_AMVPSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AMVPSIZE").field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_AMVPSIZE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_AMVPSIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_AMVPSIZE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_AMVPSIZE {}
impl ::core::default::Default for KS_AMVPSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_AMVP_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_WEAVE: KS_AMVP_MODE = KS_AMVP_MODE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_BOBINTERLEAVED: KS_AMVP_MODE = KS_AMVP_MODE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_BOBNONINTERLEAVED: KS_AMVP_MODE = KS_AMVP_MODE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_SKIPEVEN: KS_AMVP_MODE = KS_AMVP_MODE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_MODE_SKIPODD: KS_AMVP_MODE = KS_AMVP_MODE(4i32);
impl ::core::marker::Copy for KS_AMVP_MODE {}
impl ::core::clone::Clone for KS_AMVP_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_AMVP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_AMVP_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_AMVP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AMVP_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_AMVP_SELECTFORMATBY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_DO_NOT_CARE: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_BEST_BANDWIDTH: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AMVP_INPUT_SAME_AS_OUTPUT: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(2i32);
impl ::core::marker::Copy for KS_AMVP_SELECTFORMATBY {}
impl ::core::clone::Clone for KS_AMVP_SELECTFORMATBY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_AMVP_SELECTFORMATBY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_AMVP_SELECTFORMATBY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_AMVP_SELECTFORMATBY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AMVP_SELECTFORMATBY").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KS_AM_ExactRateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AM_ExactRateChange").field("OutputZeroTime", &self.OutputZeroTime).field("Rate", &self.Rate).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_AM_ExactRateChange {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_AM_ExactRateChange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_AM_ExactRateChange>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_AM_ExactRateChange {}
impl ::core::default::Default for KS_AM_ExactRateChange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_AM_PROPERTY_TS_RATE_CHANGE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_RATE_SimpleRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_RATE_ExactRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_RATE_MaxFullDataRate: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AM_RATE_Step: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(4i32);
impl ::core::marker::Copy for KS_AM_PROPERTY_TS_RATE_CHANGE {}
impl ::core::clone::Clone for KS_AM_PROPERTY_TS_RATE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_AM_PROPERTY_TS_RATE_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_AM_PROPERTY_TS_RATE_CHANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_AM_PROPERTY_TS_RATE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AM_PROPERTY_TS_RATE_CHANGE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KS_AM_SimpleRateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AM_SimpleRateChange").field("StartTime", &self.StartTime).field("Rate", &self.Rate).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_AM_SimpleRateChange {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_AM_SimpleRateChange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_AM_SimpleRateChange>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_AM_SimpleRateChange {}
impl ::core::default::Default for KS_AM_SimpleRateChange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_AnalogVideoInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AnalogVideoInfo").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwActiveWidth", &self.dwActiveWidth).field("dwActiveHeight", &self.dwActiveHeight).field("AvgTimePerFrame", &self.AvgTimePerFrame).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_AnalogVideoInfo {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_AnalogVideoInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_AnalogVideoInfo>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_AnalogVideoInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_AnalogVideoInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_AnalogVideoStandard(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_None: KS_AnalogVideoStandard = KS_AnalogVideoStandard(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_NTSC_M: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_NTSC_M_J: KS_AnalogVideoStandard = KS_AnalogVideoStandard(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_NTSC_433: KS_AnalogVideoStandard = KS_AnalogVideoStandard(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_B: KS_AnalogVideoStandard = KS_AnalogVideoStandard(16i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_D: KS_AnalogVideoStandard = KS_AnalogVideoStandard(32i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_G: KS_AnalogVideoStandard = KS_AnalogVideoStandard(64i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_H: KS_AnalogVideoStandard = KS_AnalogVideoStandard(128i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_I: KS_AnalogVideoStandard = KS_AnalogVideoStandard(256i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_M: KS_AnalogVideoStandard = KS_AnalogVideoStandard(512i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_N: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1024i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_60: KS_AnalogVideoStandard = KS_AnalogVideoStandard(2048i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_B: KS_AnalogVideoStandard = KS_AnalogVideoStandard(4096i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_D: KS_AnalogVideoStandard = KS_AnalogVideoStandard(8192i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_G: KS_AnalogVideoStandard = KS_AnalogVideoStandard(16384i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_H: KS_AnalogVideoStandard = KS_AnalogVideoStandard(32768i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_K: KS_AnalogVideoStandard = KS_AnalogVideoStandard(65536i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_K1: KS_AnalogVideoStandard = KS_AnalogVideoStandard(131072i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_L: KS_AnalogVideoStandard = KS_AnalogVideoStandard(262144i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_SECAM_L1: KS_AnalogVideoStandard = KS_AnalogVideoStandard(524288i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_AnalogVideo_PAL_N_COMBO: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1048576i32);
impl ::core::marker::Copy for KS_AnalogVideoStandard {}
impl ::core::clone::Clone for KS_AnalogVideoStandard {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_AnalogVideoStandard {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_AnalogVideoStandard {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_AnalogVideoStandard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AnalogVideoStandard").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KS_BITMAPINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_BITMAPINFOHEADER").field("biSize", &self.biSize).field("biWidth", &self.biWidth).field("biHeight", &self.biHeight).field("biPlanes", &self.biPlanes).field("biBitCount", &self.biBitCount).field("biCompression", &self.biCompression).field("biSizeImage", &self.biSizeImage).field("biXPelsPerMeter", &self.biXPelsPerMeter).field("biYPelsPerMeter", &self.biYPelsPerMeter).field("biClrUsed", &self.biClrUsed).field("biClrImportant", &self.biClrImportant).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_BITMAPINFOHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_BITMAPINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_BITMAPINFOHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_BITMAPINFOHEADER {}
impl ::core::default::Default for KS_BITMAPINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_COLCON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_COLCON").field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).field("_bitfield3", &self._bitfield3).field("_bitfield4", &self._bitfield4).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_COLCON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_COLCON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_COLCON>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_COLCON {}
impl ::core::default::Default for KS_COLCON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_COMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_COMPRESSION").field("RatioNumerator", &self.RatioNumerator).field("RatioDenominator", &self.RatioDenominator).field("RatioConstantMargin", &self.RatioConstantMargin).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_COMPRESSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_COMPRESSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_COMPRESSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_COMPRESSION {}
impl ::core::default::Default for KS_COMPRESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_COPY_MACROVISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_COPY_MACROVISION").field("MACROVISIONLevel", &self.MACROVISIONLevel).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_COPY_MACROVISION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_COPY_MACROVISION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_COPY_MACROVISION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_COPY_MACROVISION {}
impl ::core::default::Default for KS_COPY_MACROVISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_COPY_MACROVISION_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MACROVISION_DISABLED: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MACROVISION_LEVEL1: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MACROVISION_LEVEL2: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MACROVISION_LEVEL3: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(3i32);
impl ::core::marker::Copy for KS_COPY_MACROVISION_LEVEL {}
impl ::core::clone::Clone for KS_COPY_MACROVISION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_COPY_MACROVISION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_COPY_MACROVISION_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_COPY_MACROVISION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_COPY_MACROVISION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_CameraControlAsyncOperation(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAMERACONTROL_ASYNC_START: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAMERACONTROL_ASYNC_STOP: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CAMERACONTROL_ASYNC_RESET: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(3i32);
impl ::core::marker::Copy for KS_CameraControlAsyncOperation {}
impl ::core::clone::Clone for KS_CameraControlAsyncOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_CameraControlAsyncOperation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_CameraControlAsyncOperation {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_CameraControlAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_CameraControlAsyncOperation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_CompressionCaps(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanQuality: KS_CompressionCaps = KS_CompressionCaps(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanCrunch: KS_CompressionCaps = KS_CompressionCaps(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanKeyFrame: KS_CompressionCaps = KS_CompressionCaps(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanBFrame: KS_CompressionCaps = KS_CompressionCaps(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_CompressionCaps_CanWindow: KS_CompressionCaps = KS_CompressionCaps(16i32);
impl ::core::marker::Copy for KS_CompressionCaps {}
impl ::core::clone::Clone for KS_CompressionCaps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_CompressionCaps {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_CompressionCaps {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_CompressionCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_CompressionCaps").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for KS_DATAFORMAT_H264VIDEOINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DATAFORMAT_H264VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATAFORMAT_H264VIDEOINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DATAFORMAT_H264VIDEOINFO {}
impl ::core::default::Default for KS_DATAFORMAT_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KS_DATAFORMAT_IMAGEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DATAFORMAT_IMAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATAFORMAT_IMAGEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DATAFORMAT_IMAGEINFO {}
impl ::core::default::Default for KS_DATAFORMAT_IMAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATAFORMAT_MPEGVIDEOINFO2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATAFORMAT_MPEGVIDEOINFO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KS_DATAFORMAT_VBIINFOHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DATAFORMAT_VBIINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATAFORMAT_VBIINFOHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DATAFORMAT_VBIINFOHEADER {}
impl ::core::default::Default for KS_DATAFORMAT_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATAFORMAT_VIDEOINFOHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATAFORMAT_VIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATAFORMAT_VIDEOINFOHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATAFORMAT_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATAFORMAT_VIDEOINFOHEADER2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATAFORMAT_VIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATAFORMAT_VIDEOINFO_PALETTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATAFORMAT_VIDEOINFO_PALETTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_ANALOGVIDEO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_ANALOGVIDEO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_ANALOGVIDEO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_ANALOGVIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_ANALOGVIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_H264_VIDEO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_H264_VIDEO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_H264_VIDEO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_H264_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_H264_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_IMAGE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_IMAGE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_IMAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_MPEG1_VIDEO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_MPEG1_VIDEO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_MPEG1_VIDEO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_MPEG1_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_MPEG1_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_MPEG2_VIDEO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_MPEG2_VIDEO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_MPEG2_VIDEO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_MPEG2_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_MPEG2_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_VIDEO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_VIDEO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_VIDEO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_VIDEO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_VIDEO2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_VIDEO2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_VIDEO2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_VIDEO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_VIDEO_PALETTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_VIDEO_PALETTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_VIDEO_PALETTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_VIDEO_PALETTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO_PALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_DATARANGE_VIDEO_VBI {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_DATARANGE_VIDEO_VBI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DATARANGE_VIDEO_VBI>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_DATARANGE_VIDEO_VBI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO_VBI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_DVDCOPYSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_INITIALIZE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_INITIALIZE_TITLE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_AUTHENTICATION_NOT_REQUIRED: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_AUTHENTICATION_REQUIRED: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_DVDCOPYSTATE_DONE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(4i32);
impl ::core::marker::Copy for KS_DVDCOPYSTATE {}
impl ::core::clone::Clone for KS_DVDCOPYSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_DVDCOPYSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_DVDCOPYSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_DVDCOPYSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_DVDCOPYSTATE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KS_DVDCOPY_BUSKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_BUSKEY").field("BusKey", &self.BusKey).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_DVDCOPY_BUSKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_BUSKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DVDCOPY_BUSKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_BUSKEY {}
impl ::core::default::Default for KS_DVDCOPY_BUSKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_DVDCOPY_CHLGKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_CHLGKEY").field("ChlgKey", &self.ChlgKey).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_DVDCOPY_CHLGKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_CHLGKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DVDCOPY_CHLGKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_CHLGKEY {}
impl ::core::default::Default for KS_DVDCOPY_CHLGKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_DVDCOPY_DISCKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_DISCKEY").field("DiscKey", &self.DiscKey).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_DVDCOPY_DISCKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_DISCKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DVDCOPY_DISCKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_DISCKEY {}
impl ::core::default::Default for KS_DVDCOPY_DISCKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_DVDCOPY_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_REGION").field("Reserved", &self.Reserved).field("RegionData", &self.RegionData).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_DVDCOPY_REGION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_REGION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DVDCOPY_REGION>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_REGION {}
impl ::core::default::Default for KS_DVDCOPY_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_DVDCOPY_SET_COPY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_SET_COPY_STATE").field("DVDCopyState", &self.DVDCopyState).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_DVDCOPY_SET_COPY_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_SET_COPY_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DVDCOPY_SET_COPY_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_SET_COPY_STATE {}
impl ::core::default::Default for KS_DVDCOPY_SET_COPY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_DVDCOPY_TITLEKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_TITLEKEY").field("KeyFlags", &self.KeyFlags).field("ReservedNT", &self.ReservedNT).field("TitleKey", &self.TitleKey).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_DVDCOPY_TITLEKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_TITLEKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DVDCOPY_TITLEKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_TITLEKEY {}
impl ::core::default::Default for KS_DVDCOPY_TITLEKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_DVD_YCrCb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVD_YCrCb").field("Reserved", &self.Reserved).field("Y", &self.Y).field("Cr", &self.Cr).field("Cb", &self.Cb).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_DVD_YCrCb {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DVD_YCrCb {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DVD_YCrCb>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DVD_YCrCb {}
impl ::core::default::Default for KS_DVD_YCrCb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_DVD_YUV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVD_YUV").field("Reserved", &self.Reserved).field("Y", &self.Y).field("V", &self.V).field("U", &self.U).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_DVD_YUV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_DVD_YUV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_DVD_YUV>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_DVD_YUV {}
impl ::core::default::Default for KS_DVD_YUV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_FRAME_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_FRAME_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_FRAME_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_FRAME_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_FRAME_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_FRAME_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_FRAME_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_FRAME_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_FRAME_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_FRAME_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_FRAME_INFO_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_FRAME_INFO_1_0").field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_FRAME_INFO_1_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_FRAME_INFO_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_FRAME_INFO_1_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_FRAME_INFO_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct KS_FRAMING_ITEM {
    pub MemoryType: ::windows::core::GUID,
    pub BusType: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for KS_FRAMING_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_FRAMING_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_FRAMING_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_FRAMING_ITEM {}
impl ::core::default::Default for KS_FRAMING_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for KS_FRAMING_ITEM_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_FRAMING_ITEM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_FRAMING_ITEM_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_FRAMING_ITEM_0 {}
impl ::core::default::Default for KS_FRAMING_ITEM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_FRAMING_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_FRAMING_RANGE").field("MinFrameSize", &self.MinFrameSize).field("MaxFrameSize", &self.MaxFrameSize).field("Stepping", &self.Stepping).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_FRAMING_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_FRAMING_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_FRAMING_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_FRAMING_RANGE {}
impl ::core::default::Default for KS_FRAMING_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_FRAMING_RANGE_WEIGHTED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_FRAMING_RANGE_WEIGHTED").field("Range", &self.Range).field("InPlaceWeight", &self.InPlaceWeight).field("NotInPlaceWeight", &self.NotInPlaceWeight).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_FRAMING_RANGE_WEIGHTED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_FRAMING_RANGE_WEIGHTED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_FRAMING_RANGE_WEIGHTED>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_FRAMING_RANGE_WEIGHTED {}
impl ::core::default::Default for KS_FRAMING_RANGE_WEIGHTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_H264VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_H264VIDEOINFO")
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
unsafe impl ::windows::core::Abi for KS_H264VIDEOINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_H264VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_H264VIDEOINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_H264VIDEOINFO {}
impl ::core::default::Default for KS_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_LogicalMemoryType(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeDontCare: KS_LogicalMemoryType = KS_LogicalMemoryType(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeKernelPaged: KS_LogicalMemoryType = KS_LogicalMemoryType(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeKernelNonPaged: KS_LogicalMemoryType = KS_LogicalMemoryType(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeDeviceHostMapped: KS_LogicalMemoryType = KS_LogicalMemoryType(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeDeviceSpecific: KS_LogicalMemoryType = KS_LogicalMemoryType(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeUser: KS_LogicalMemoryType = KS_LogicalMemoryType(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MemoryTypeAnyHost: KS_LogicalMemoryType = KS_LogicalMemoryType(6i32);
impl ::core::marker::Copy for KS_LogicalMemoryType {}
impl ::core::clone::Clone for KS_LogicalMemoryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_LogicalMemoryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_LogicalMemoryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_LogicalMemoryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_LogicalMemoryType").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KS_MPEAUDIOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_MPEAUDIOINFO").field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_MPEAUDIOINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_MPEAUDIOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_MPEAUDIOINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_MPEAUDIOINFO {}
impl ::core::default::Default for KS_MPEAUDIOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_MPEG1VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_MPEG1VIDEOINFO").field("hdr", &self.hdr).field("dwStartTimeCode", &self.dwStartTimeCode).field("cbSequenceHeader", &self.cbSequenceHeader).field("bSequenceHeader", &self.bSequenceHeader).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_MPEG1VIDEOINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_MPEG1VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_MPEG1VIDEOINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_MPEG1VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_MPEG2Level(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Level_Low: KS_MPEG2Level = KS_MPEG2Level(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Level_Main: KS_MPEG2Level = KS_MPEG2Level(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Level_High1440: KS_MPEG2Level = KS_MPEG2Level(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Level_High: KS_MPEG2Level = KS_MPEG2Level(3i32);
impl ::core::marker::Copy for KS_MPEG2Level {}
impl ::core::clone::Clone for KS_MPEG2Level {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_MPEG2Level {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_MPEG2Level {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_MPEG2Level {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_MPEG2Level").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_MPEG2Profile(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_Simple: KS_MPEG2Profile = KS_MPEG2Profile(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_Main: KS_MPEG2Profile = KS_MPEG2Profile(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_SNRScalable: KS_MPEG2Profile = KS_MPEG2Profile(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_SpatiallyScalable: KS_MPEG2Profile = KS_MPEG2Profile(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_MPEG2Profile_High: KS_MPEG2Profile = KS_MPEG2Profile(4i32);
impl ::core::marker::Copy for KS_MPEG2Profile {}
impl ::core::clone::Clone for KS_MPEG2Profile {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_MPEG2Profile {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_MPEG2Profile {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_MPEG2Profile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_MPEG2Profile").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_MPEGVIDEOINFO2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_MPEGVIDEOINFO2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_MPEGVIDEOINFO2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_MPEGVIDEOINFO2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_PhysicalConnectorType(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_Tuner: KS_PhysicalConnectorType = KS_PhysicalConnectorType(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_Composite: KS_PhysicalConnectorType = KS_PhysicalConnectorType(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_SVideo: KS_PhysicalConnectorType = KS_PhysicalConnectorType(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_RGB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_YRYBY: KS_PhysicalConnectorType = KS_PhysicalConnectorType(5i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_SerialDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(6i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_ParallelDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(7i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_SCSI: KS_PhysicalConnectorType = KS_PhysicalConnectorType(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_AUX: KS_PhysicalConnectorType = KS_PhysicalConnectorType(9i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_1394: KS_PhysicalConnectorType = KS_PhysicalConnectorType(10i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_USB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(11i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_VideoDecoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(12i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_VideoEncoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(13i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Video_SCART: KS_PhysicalConnectorType = KS_PhysicalConnectorType(14i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_Tuner: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4096i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_Line: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4097i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_Mic: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4098i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_AESDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4099i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_SPDIFDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4100i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_SCSI: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4101i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_AUX: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4102i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_1394: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4103i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_USB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4104i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_PhysConn_Audio_AudioDecoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4105i32);
impl ::core::marker::Copy for KS_PhysicalConnectorType {}
impl ::core::clone::Clone for KS_PhysicalConnectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_PhysicalConnectorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_PhysicalConnectorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_PhysicalConnectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_PhysicalConnectorType").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KS_RGBQUAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_RGBQUAD").field("rgbBlue", &self.rgbBlue).field("rgbGreen", &self.rgbGreen).field("rgbRed", &self.rgbRed).field("rgbReserved", &self.rgbReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_RGBQUAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_RGBQUAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_RGBQUAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_RGBQUAD {}
impl ::core::default::Default for KS_RGBQUAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KS_SECURE_CAMERA_SCENARIO_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae53fc6e_8d89_4488_9d2e_4d008731c5fd);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_SEEKING_CAPABILITIES(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanSeekAbsolute: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanSeekForwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanSeekBackwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanGetCurrentPos: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanGetStopPos: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(16i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanGetDuration: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(32i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_CanPlayBackwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(64i32);
impl ::core::marker::Copy for KS_SEEKING_CAPABILITIES {}
impl ::core::clone::Clone for KS_SEEKING_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_SEEKING_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_SEEKING_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_SEEKING_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_SEEKING_CAPABILITIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_SEEKING_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_NoPositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_AbsolutePositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_RelativePositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_IncrementalPositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_PositioningBitsMask: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_SeekToKeyFrame: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_SEEKING_ReturnTime: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(8i32);
impl ::core::marker::Copy for KS_SEEKING_FLAGS {}
impl ::core::clone::Clone for KS_SEEKING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_SEEKING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_SEEKING_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_SEEKING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_SEEKING_FLAGS").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KS_TRUECOLORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_TRUECOLORINFO").field("dwBitMasks", &self.dwBitMasks).field("bmiColors", &self.bmiColors).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_TRUECOLORINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_TRUECOLORINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_TRUECOLORINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_TRUECOLORINFO {}
impl ::core::default::Default for KS_TRUECOLORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_TUNER_STRATEGY(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_STRATEGY_PLL: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_STRATEGY_SIGNAL_STRENGTH: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_STRATEGY_DRIVER_TUNES: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(4i32);
impl ::core::marker::Copy for KS_TUNER_STRATEGY {}
impl ::core::clone::Clone for KS_TUNER_STRATEGY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_TUNER_STRATEGY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_TUNER_STRATEGY {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_TUNER_STRATEGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_TUNER_STRATEGY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_TUNER_TUNING_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_TUNING_EXACT: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_TUNING_FINE: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_TUNER_TUNING_COARSE: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(3i32);
impl ::core::marker::Copy for KS_TUNER_TUNING_FLAGS {}
impl ::core::clone::Clone for KS_TUNER_TUNING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_TUNER_TUNING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_TUNER_TUNING_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_TUNER_TUNING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_TUNER_TUNING_FLAGS").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for KS_TVTUNER_CHANGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_TVTUNER_CHANGE_INFO").field("dwFlags", &self.dwFlags).field("dwCountryCode", &self.dwCountryCode).field("dwAnalogVideoStandard", &self.dwAnalogVideoStandard).field("dwChannel", &self.dwChannel).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_TVTUNER_CHANGE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_TVTUNER_CHANGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_TVTUNER_CHANGE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_TVTUNER_CHANGE_INFO {}
impl ::core::default::Default for KS_TVTUNER_CHANGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_VBIINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_VBIINFOHEADER")
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
unsafe impl ::windows::core::Abi for KS_VBIINFOHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_VBIINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_VBIINFOHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_VBIINFOHEADER {}
impl ::core::default::Default for KS_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for KS_VBI_FRAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_VBI_FRAME_INFO").field("ExtendedHeaderSize", &self.ExtendedHeaderSize).field("dwFrameFlags", &self.dwFrameFlags).field("PictureNumber", &self.PictureNumber).field("DropCount", &self.DropCount).field("dwSamplingFrequency", &self.dwSamplingFrequency).field("TvTunerChangeInfo", &self.TvTunerChangeInfo).field("VBIInfoHeader", &self.VBIInfoHeader).finish()
    }
}
unsafe impl ::windows::core::Abi for KS_VBI_FRAME_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KS_VBI_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_VBI_FRAME_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for KS_VBI_FRAME_INFO {}
impl ::core::default::Default for KS_VBI_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_VIDEODECODER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEODECODER_FLAGS_CAN_DISABLE_OUTPUT: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEODECODER_FLAGS_CAN_USE_VCR_LOCKING: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VIDEODECODER_FLAGS_CAN_INDICATE_LOCKED: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(4i32);
impl ::core::marker::Copy for KS_VIDEODECODER_FLAGS {}
impl ::core::clone::Clone for KS_VIDEODECODER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_VIDEODECODER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_VIDEODECODER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_VIDEODECODER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_VIDEODECODER_FLAGS").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_VIDEOINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_VIDEOINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_VIDEOINFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_VIDEOINFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_VIDEOINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_VIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_VIDEOINFOHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_VIDEOINFOHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_VIDEOINFOHEADER2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFOHEADER2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_VIDEOINFOHEADER2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KS_VIDEOINFOHEADER2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFOHEADER2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_VIDEOINFOHEADER2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFOHEADER2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFOHEADER2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub guid: ::windows::core::GUID,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_VIDEO_STREAM_CONFIG_CAPS")
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
unsafe impl ::windows::core::Abi for KS_VIDEO_STREAM_CONFIG_CAPS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KS_VIDEO_STREAM_CONFIG_CAPS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEO_STREAM_CONFIG_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_VideoControlFlags(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_FlipHorizontal: KS_VideoControlFlags = KS_VideoControlFlags(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_FlipVertical: KS_VideoControlFlags = KS_VideoControlFlags(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_Obsolete_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = KS_VideoControlFlags(16i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_Obsolete_VideoControlFlag_Trigger: KS_VideoControlFlags = KS_VideoControlFlags(32i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = KS_VideoControlFlags(4i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_Trigger: KS_VideoControlFlags = KS_VideoControlFlags(8i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_IndependentImagePin: KS_VideoControlFlags = KS_VideoControlFlags(64i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_StillCapturePreviewFrame: KS_VideoControlFlags = KS_VideoControlFlags(128i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_StartPhotoSequenceCapture: KS_VideoControlFlags = KS_VideoControlFlags(256i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_VideoControlFlag_StopPhotoSequenceCapture: KS_VideoControlFlags = KS_VideoControlFlags(512i32);
impl ::core::marker::Copy for KS_VideoControlFlags {}
impl ::core::clone::Clone for KS_VideoControlFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_VideoControlFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_VideoControlFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_VideoControlFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_VideoControlFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KS_VideoStreamingHints(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_FrameInterval: KS_VideoStreamingHints = KS_VideoStreamingHints(256i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_KeyFrameRate: KS_VideoStreamingHints = KS_VideoStreamingHints(512i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_PFrameRate: KS_VideoStreamingHints = KS_VideoStreamingHints(1024i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_CompQuality: KS_VideoStreamingHints = KS_VideoStreamingHints(2048i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const KS_StreamingHint_CompWindowSize: KS_VideoStreamingHints = KS_VideoStreamingHints(4096i32);
impl ::core::marker::Copy for KS_VideoStreamingHints {}
impl ::core::clone::Clone for KS_VideoStreamingHints {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KS_VideoStreamingHints {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KS_VideoStreamingHints {
    type Abi = Self;
}
impl ::core::fmt::Debug for KS_VideoStreamingHints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_VideoStreamingHints").field(&self.0).finish()
    }
}
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
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateAllocator<'a, P0>(connectionhandle: P0, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KsCreateAllocator(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    KsCreateAllocator(connectionhandle.into(), ::core::mem::transmute(allocatorframing), ::core::mem::transmute(allocatorhandle))
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateAllocator2<'a, P0>(connectionhandle: P0, allocatorframing: *const KSALLOCATOR_FRAMING) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KsCreateAllocator2(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    KsCreateAllocator2(connectionhandle.into(), ::core::mem::transmute(allocatorframing), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateClock<'a, P0>(connectionhandle: P0, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KsCreateClock(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    KsCreateClock(connectionhandle.into(), ::core::mem::transmute(clockcreate), ::core::mem::transmute(clockhandle))
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateClock2<'a, P0>(connectionhandle: P0, clockcreate: *const KSCLOCK_CREATE) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KsCreateClock2(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    KsCreateClock2(connectionhandle.into(), ::core::mem::transmute(clockcreate), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreatePin<'a, P0>(filterhandle: P0, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KsCreatePin(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    KsCreatePin(filterhandle.into(), ::core::mem::transmute(connect), desiredaccess, ::core::mem::transmute(connectionhandle))
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreatePin2<'a, P0>(filterhandle: P0, connect: *const KSPIN_CONNECT, desiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KsCreatePin2(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    KsCreatePin2(filterhandle.into(), ::core::mem::transmute(connect), desiredaccess, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateTopologyNode<'a, P0>(parenthandle: P0, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KsCreateTopologyNode(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    KsCreateTopologyNode(parenthandle.into(), ::core::mem::transmute(nodecreate), desiredaccess, ::core::mem::transmute(nodehandle))
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KsCreateTopologyNode2<'a, P0>(parenthandle: P0, nodecreate: *const KSNODE_CREATE, desiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KsCreateTopologyNode2(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    KsCreateTopologyNode2(parenthandle.into(), ::core::mem::transmute(nodecreate), desiredaccess, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOOPEDSTREAMING_POSITION_EVENT_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOOPEDSTREAMING_POSITION_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MEDIUM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEDIUM_INFO").field("MediaPresent", &self.MediaPresent).field("MediaType", &self.MediaType).field("RecordInhibit", &self.RecordInhibit).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MEDIUM_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MEDIUM_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEDIUM_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MEDIUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MEDIUM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub union MF_MDL_SHARED_PAYLOAD_KEY {
    pub combined: MF_MDL_SHARED_PAYLOAD_KEY_0,
    pub GMDLHandle: ::windows::core::GUID,
}
impl ::core::marker::Copy for MF_MDL_SHARED_PAYLOAD_KEY {}
impl ::core::clone::Clone for MF_MDL_SHARED_PAYLOAD_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MF_MDL_SHARED_PAYLOAD_KEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MF_MDL_SHARED_PAYLOAD_KEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MF_MDL_SHARED_PAYLOAD_KEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for MF_MDL_SHARED_PAYLOAD_KEY {}
impl ::core::default::Default for MF_MDL_SHARED_PAYLOAD_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_MDL_SHARED_PAYLOAD_KEY_0").field("pHandle", &self.pHandle).field("fHandle", &self.fHandle).field("uPayload", &self.uPayload).finish()
    }
}
unsafe impl ::windows::core::Abi for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MF_MDL_SHARED_PAYLOAD_KEY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MF_MDL_SHARED_PAYLOAD_KEY_0 {}
impl ::core::default::Default for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NABTSFEC_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NABTSFEC_BUFFER").field("dataSize", &self.dataSize).field("groupID", &self.groupID).field("Reserved", &self.Reserved).field("data", &self.data).finish()
    }
}
unsafe impl ::windows::core::Abi for NABTSFEC_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NABTSFEC_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NABTSFEC_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for NABTSFEC_BUFFER {}
impl ::core::default::Default for NABTSFEC_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for NABTS_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NABTS_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NABTS_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for NABTS_BUFFER {}
impl ::core::default::Default for NABTS_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NABTS_BUFFER_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NABTS_BUFFER_LINE").field("Confidence", &self.Confidence).field("Bytes", &self.Bytes).finish()
    }
}
unsafe impl ::windows::core::Abi for NABTS_BUFFER_LINE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NABTS_BUFFER_LINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NABTS_BUFFER_LINE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NABTS_BUFFER_LINE {}
impl ::core::default::Default for NABTS_BUFFER_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for OPTIMAL_WEIGHT_TOTALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPTIMAL_WEIGHT_TOTALS").field("MinTotalNominator", &self.MinTotalNominator).field("MaxTotalNominator", &self.MaxTotalNominator).field("TotalDenominator", &self.TotalDenominator).finish()
    }
}
unsafe impl ::windows::core::Abi for OPTIMAL_WEIGHT_TOTALS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OPTIMAL_WEIGHT_TOTALS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPTIMAL_WEIGHT_TOTALS>()) == 0 }
    }
}
impl ::core::cmp::Eq for OPTIMAL_WEIGHT_TOTALS {}
impl ::core::default::Default for OPTIMAL_WEIGHT_TOTALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PINNAME_DISPLAYPORT_OUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21fbb329_1a4a_48da_a076_2318a3c59b26);
pub const PINNAME_HDMI_OUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x387bfc03_e7ef_4901_86e0_35b7c32b00ef);
pub const PINNAME_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38a0cd98_d49b_4ce8_b48a_344667a17830);
pub const PINNAME_SPDIF_IN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15dc9025_22ad_41b3_8875_f4ceb0299e20);
pub const PINNAME_SPDIF_OUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a264481_e52c_4b82_8e7a_c8e2f91dc380);
pub const PINNAME_VIDEO_ANALOGVIDEOIN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4283_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4281_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4289_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CC_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aad8061_012d_11d2_b4b1_00a0d102cfbe);
pub const PINNAME_VIDEO_EDS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4287_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_NABTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4286_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_NABTS_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29703660_498a_11d2_b4b1_00a0d102cfbe);
pub const PINNAME_VIDEO_PREVIEW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4282_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_STILL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c428a_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_TELETEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4288_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_TIMECODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c428b_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VBI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4284_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VIDEOPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c4285_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VIDEOPORT_VBI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c428c_0353_11d1_905f_0000c0cc16ba);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PIPE_ALLOCATOR_PLACE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Pipe_Allocator_None: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Pipe_Allocator_FirstPin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Pipe_Allocator_LastPin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Pipe_Allocator_MiddlePin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(3i32);
impl ::core::marker::Copy for PIPE_ALLOCATOR_PLACE {}
impl ::core::clone::Clone for PIPE_ALLOCATOR_PLACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PIPE_ALLOCATOR_PLACE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PIPE_ALLOCATOR_PLACE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PIPE_ALLOCATOR_PLACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIPE_ALLOCATOR_PLACE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for PIPE_DIMENSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PIPE_DIMENSIONS").field("AllocatorPin", &self.AllocatorPin).field("MaxExpansionPin", &self.MaxExpansionPin).field("EndPin", &self.EndPin).finish()
    }
}
unsafe impl ::windows::core::Abi for PIPE_DIMENSIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PIPE_DIMENSIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PIPE_DIMENSIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PIPE_DIMENSIONS {}
impl ::core::default::Default for PIPE_DIMENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PIPE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_DontCare: PIPE_STATE = PIPE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_RangeNotFixed: PIPE_STATE = PIPE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_RangeFixed: PIPE_STATE = PIPE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_CompressionUnknown: PIPE_STATE = PIPE_STATE(3i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const PipeState_Finalized: PIPE_STATE = PIPE_STATE(4i32);
impl ::core::marker::Copy for PIPE_STATE {}
impl ::core::clone::Clone for PIPE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PIPE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PIPE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PIPE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIPE_STATE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for PIPE_TERMINATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PIPE_TERMINATION").field("Flags", &self.Flags).field("OutsideFactors", &self.OutsideFactors).field("Weigth", &self.Weigth).field("PhysicalRange", &self.PhysicalRange).field("OptimalRange", &self.OptimalRange).field("Compression", &self.Compression).finish()
    }
}
unsafe impl ::windows::core::Abi for PIPE_TERMINATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PIPE_TERMINATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PIPE_TERMINATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PIPE_TERMINATION {}
impl ::core::default::Default for PIPE_TERMINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PROPSETID_ALLOCATOR_CONTROL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53171960_148e_11d2_9979_0000c0cc16ba);
pub const PROPSETID_EXT_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5730a90_1a2c_11cf_8c23_00aa006b6814);
pub const PROPSETID_EXT_TRANSPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa03cd5f0_3045_11cf_8c44_00aa006b6814);
pub const PROPSETID_TIMECODE_READER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b496ce1_811b_11cf_8c77_00aa006b6814);
pub const PROPSETID_TUNER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0605_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_CAMERACONTROL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e13370_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_CAMERACONTROL_FLASH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x785e8f49_63a2_4144_ab70_ffb278fa26ce);
pub const PROPSETID_VIDCAP_CAMERACONTROL_IMAGE_PIN_CAPABILITY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d3d7bbf_5c6d_4138_bb00_584edd20f7c5);
pub const PROPSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d12d198_f86c_4fed_b023_5d87653da793);
pub const PROPSETID_VIDCAP_CAMERACONTROL_VIDEO_STABILIZATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43964bd3_7716_404e_8be1_d299b20e50fd);
pub const PROPSETID_VIDCAP_CROSSBAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0640_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_DROPPEDFRAMES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e13344_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_SELECTOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1abdaeca_68b6_4f83_9371_b413907c7b9f);
pub const PROPSETID_VIDCAP_TVAUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0650_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOCOMPRESSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e13343_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOCONTROL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0670_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEODECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e13350_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2e0610_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOPROCAMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e13360_30ac_11d0_a18c_00a0c9118956);
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
pub const RT_RCDATA: ::windows::core::PCWSTR = ::windows::core::PCWSTR(10i32 as _);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const RT_STRING: ::windows::core::PCWSTR = ::windows::core::PCWSTR(6i32 as _);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct SECURE_BUFFER_INFO {
    pub guidBufferIdentifier: ::windows::core::GUID,
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
impl ::core::fmt::Debug for SECURE_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURE_BUFFER_INFO").field("guidBufferIdentifier", &self.guidBufferIdentifier).field("cbBufferSize", &self.cbBufferSize).field("cbCaptured", &self.cbCaptured).field("ullReserved", &self.ullReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for SECURE_BUFFER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SECURE_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SECURE_BUFFER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SECURE_BUFFER_INFO {}
impl ::core::default::Default for SECURE_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub struct SOUNDDETECTOR_PATTERNHEADER {
    pub Size: u32,
    pub PatternType: ::windows::core::GUID,
}
impl ::core::marker::Copy for SOUNDDETECTOR_PATTERNHEADER {}
impl ::core::clone::Clone for SOUNDDETECTOR_PATTERNHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOUNDDETECTOR_PATTERNHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOUNDDETECTOR_PATTERNHEADER").field("Size", &self.Size).field("PatternType", &self.PatternType).finish()
    }
}
unsafe impl ::windows::core::Abi for SOUNDDETECTOR_PATTERNHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOUNDDETECTOR_PATTERNHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOUNDDETECTOR_PATTERNHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOUNDDETECTOR_PATTERNHEADER {}
impl ::core::default::Default for SOUNDDETECTOR_PATTERNHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TELEPHONY_CALLCONTROLOP(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLCONTROLOP_DISABLE: TELEPHONY_CALLCONTROLOP = TELEPHONY_CALLCONTROLOP(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLCONTROLOP_ENABLE: TELEPHONY_CALLCONTROLOP = TELEPHONY_CALLCONTROLOP(1i32);
impl ::core::marker::Copy for TELEPHONY_CALLCONTROLOP {}
impl ::core::clone::Clone for TELEPHONY_CALLCONTROLOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TELEPHONY_CALLCONTROLOP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TELEPHONY_CALLCONTROLOP {
    type Abi = Self;
}
impl ::core::fmt::Debug for TELEPHONY_CALLCONTROLOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TELEPHONY_CALLCONTROLOP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TELEPHONY_CALLSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLSTATE_DISABLED: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLSTATE_ENABLED: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLSTATE_HOLD: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(2i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLSTATE_PROVIDERTRANSITION: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(3i32);
impl ::core::marker::Copy for TELEPHONY_CALLSTATE {}
impl ::core::clone::Clone for TELEPHONY_CALLSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TELEPHONY_CALLSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TELEPHONY_CALLSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TELEPHONY_CALLSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TELEPHONY_CALLSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TELEPHONY_CALLTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLTYPE_CIRCUITSWITCHED: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_LTE: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_WLAN: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(2i32);
impl ::core::marker::Copy for TELEPHONY_CALLTYPE {}
impl ::core::clone::Clone for TELEPHONY_CALLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TELEPHONY_CALLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TELEPHONY_CALLTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TELEPHONY_CALLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TELEPHONY_CALLTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TELEPHONY_PROVIDERCHANGEOP(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_PROVIDERCHANGEOP_END: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_PROVIDERCHANGEOP_BEGIN: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const TELEPHONY_PROVIDERCHANGEOP_CANCEL: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(2i32);
impl ::core::marker::Copy for TELEPHONY_PROVIDERCHANGEOP {}
impl ::core::clone::Clone for TELEPHONY_PROVIDERCHANGEOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TELEPHONY_PROVIDERCHANGEOP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TELEPHONY_PROVIDERCHANGEOP {
    type Abi = Self;
}
impl ::core::fmt::Debug for TELEPHONY_PROVIDERCHANGEOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TELEPHONY_PROVIDERCHANGEOP").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for TRANSPORTAUDIOPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTAUDIOPARMS").field("EnableOutput", &self.EnableOutput).field("EnableRecord", &self.EnableRecord).field("EnableSelsync", &self.EnableSelsync).field("Input", &self.Input).field("MonitorSource", &self.MonitorSource).finish()
    }
}
unsafe impl ::windows::core::Abi for TRANSPORTAUDIOPARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRANSPORTAUDIOPARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSPORTAUDIOPARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRANSPORTAUDIOPARMS {}
impl ::core::default::Default for TRANSPORTAUDIOPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TRANSPORTBASICPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTBASICPARMS")
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
unsafe impl ::windows::core::Abi for TRANSPORTBASICPARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRANSPORTBASICPARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSPORTBASICPARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRANSPORTBASICPARMS {}
impl ::core::default::Default for TRANSPORTBASICPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TRANSPORTSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTSTATUS")
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
unsafe impl ::windows::core::Abi for TRANSPORTSTATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRANSPORTSTATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSPORTSTATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRANSPORTSTATUS {}
impl ::core::default::Default for TRANSPORTSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TRANSPORTVIDEOPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTVIDEOPARMS").field("OutputMode", &self.OutputMode).field("Input", &self.Input).finish()
    }
}
unsafe impl ::windows::core::Abi for TRANSPORTVIDEOPARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRANSPORTVIDEOPARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSPORTVIDEOPARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRANSPORTVIDEOPARMS {}
impl ::core::default::Default for TRANSPORTVIDEOPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TRANSPORT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_STATE").field("Mode", &self.Mode).field("State", &self.State).finish()
    }
}
unsafe impl ::windows::core::Abi for TRANSPORT_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRANSPORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSPORT_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRANSPORT_STATE {}
impl ::core::default::Default for TRANSPORT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TUNER_ANALOG_CAPS_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TUNER_ANALOG_CAPS_S").field("Mode", &self.Mode).field("StandardsSupported", &self.StandardsSupported).field("MinFrequency", &self.MinFrequency).field("MaxFrequency", &self.MaxFrequency).field("TuningGranularity", &self.TuningGranularity).field("SettlingTime", &self.SettlingTime).field("ScanSensingRange", &self.ScanSensingRange).field("FineTuneSensingRange", &self.FineTuneSensingRange).finish()
    }
}
unsafe impl ::windows::core::Abi for TUNER_ANALOG_CAPS_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TUNER_ANALOG_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TUNER_ANALOG_CAPS_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for TUNER_ANALOG_CAPS_S {}
impl ::core::default::Default for TUNER_ANALOG_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for VBICAP_PROPERTIES_PROTECTION_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICAP_PROPERTIES_PROTECTION_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICAP_PROPERTIES_PROTECTION_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICAP_PROPERTIES_PROTECTION_S {}
impl ::core::default::Default for VBICAP_PROPERTIES_PROTECTION_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_CC_SUBSTREAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_CC_SUBSTREAMS").field("SubstreamMask", &self.SubstreamMask).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_CC_SUBSTREAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_CC_SUBSTREAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_CC_SUBSTREAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_CC_SUBSTREAMS {}
impl ::core::default::Default for VBICODECFILTERING_CC_SUBSTREAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_NABTS_SUBSTREAMS").field("SubstreamMask", &self.SubstreamMask).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_NABTS_SUBSTREAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_NABTS_SUBSTREAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_NABTS_SUBSTREAMS {}
impl ::core::default::Default for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_SCANLINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_SCANLINES").field("DwordBitArray", &self.DwordBitArray).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_SCANLINES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_SCANLINES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_SCANLINES>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_SCANLINES {}
impl ::core::default::Default for VBICODECFILTERING_SCANLINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_CC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_CC").field("Common", &self.Common).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_STATISTICS_CC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_CC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_STATISTICS_CC>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_CC {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_CC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_CC_PIN").field("Common", &self.Common).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_STATISTICS_CC_PIN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_STATISTICS_CC_PIN>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_CC_PIN {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_COMMON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_COMMON")
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
unsafe impl ::windows::core::Abi for VBICODECFILTERING_STATISTICS_COMMON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_COMMON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_STATISTICS_COMMON>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_COMMON {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_COMMON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_COMMON_PIN").field("SRBsProcessed", &self.SRBsProcessed).field("SRBsIgnored", &self.SRBsIgnored).field("SRBsMissing", &self.SRBsMissing).field("InternalErrors", &self.InternalErrors).field("ExternalErrors", &self.ExternalErrors).field("Discontinuities", &self.Discontinuities).field("LineConfidenceAvg", &self.LineConfidenceAvg).field("BytesOutput", &self.BytesOutput).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_STATISTICS_COMMON_PIN>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_COMMON_PIN {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_NABTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_NABTS").field("Common", &self.Common).field("FECBundleBadLines", &self.FECBundleBadLines).field("FECQueueOverflows", &self.FECQueueOverflows).field("FECCorrectedLines", &self.FECCorrectedLines).field("FECUncorrectableLines", &self.FECUncorrectableLines).field("BundlesProcessed", &self.BundlesProcessed).field("BundlesSent2IP", &self.BundlesSent2IP).field("FilteredLines", &self.FilteredLines).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_STATISTICS_NABTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_NABTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_STATISTICS_NABTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_NABTS {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_NABTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_NABTS_PIN").field("Common", &self.Common).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_STATISTICS_NABTS_PIN>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_NABTS_PIN {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_TELETEXT").field("Common", &self.Common).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_STATISTICS_TELETEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_STATISTICS_TELETEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_TELETEXT {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_TELETEXT_PIN").field("Common", &self.Common).finish()
    }
}
unsafe impl ::windows::core::Abi for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VBICODECFILTERING_STATISTICS_TELETEXT_PIN>()) == 0 }
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VRAM_SURFACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VRAM_SURFACE_INFO").field("hSurface", &self.hSurface).field("VramPhysicalAddress", &self.VramPhysicalAddress).field("cbCaptured", &self.cbCaptured).field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("dwLinearSize", &self.dwLinearSize).field("lPitch", &self.lPitch).field("ullReserved", &self.ullReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for VRAM_SURFACE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VRAM_SURFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VRAM_SURFACE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for VRAM_SURFACE_INFO {}
impl ::core::default::Default for VRAM_SURFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for VRAM_SURFACE_INFO_PROPERTY_S {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VRAM_SURFACE_INFO_PROPERTY_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VRAM_SURFACE_INFO_PROPERTY_S>()) == 0 }
    }
}
impl ::core::cmp::Eq for VRAM_SURFACE_INFO_PROPERTY_S {}
impl ::core::default::Default for VRAM_SURFACE_INFO_PROPERTY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNF_KSCAMERA_STREAMSTATE_INFO").field("ProcessId", &self.ProcessId).field("SessionId", &self.SessionId).field("StreamState", &self.StreamState).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for WNF_KSCAMERA_STREAMSTATE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNF_KSCAMERA_STREAMSTATE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WNF_KSCAMERA_STREAMSTATE_INFO {}
impl ::core::default::Default for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WST_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WST_BUFFER").field("ScanlinesRequested", &self.ScanlinesRequested).field("WstLines", &self.WstLines).finish()
    }
}
unsafe impl ::windows::core::Abi for WST_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WST_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WST_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WST_BUFFER {}
impl ::core::default::Default for WST_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WST_BUFFER_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WST_BUFFER_LINE").field("Confidence", &self.Confidence).field("Bytes", &self.Bytes).finish()
    }
}
unsafe impl ::windows::core::Abi for WST_BUFFER_LINE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WST_BUFFER_LINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WST_BUFFER_LINE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WST_BUFFER_LINE {}
impl ::core::default::Default for WST_BUFFER_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub ProcessingMode: ::windows::core::GUID,
    pub SamplesPerProcessingPacket: u32,
    pub ProcessingPacketDurationInHns: u32,
}
impl ::core::marker::Copy for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {}
impl ::core::clone::Clone for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT").field("ProcessingMode", &self.ProcessingMode).field("SamplesPerProcessingPacket", &self.SamplesPerProcessingPacket).field("ProcessingPacketDurationInHns", &self.ProcessingPacketDurationInHns).finish()
    }
}
unsafe impl ::windows::core::Abi for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {}
impl ::core::default::Default for _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _TunerDecoderLockType(pub i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Tuner_LockType_None: _TunerDecoderLockType = _TunerDecoderLockType(0i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Tuner_LockType_Within_Scan_Sensing_Range: _TunerDecoderLockType = _TunerDecoderLockType(1i32);
#[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
pub const Tuner_LockType_Locked: _TunerDecoderLockType = _TunerDecoderLockType(2i32);
impl ::core::marker::Copy for _TunerDecoderLockType {}
impl ::core::clone::Clone for _TunerDecoderLockType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _TunerDecoderLockType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _TunerDecoderLockType {
    type Abi = Self;
}
impl ::core::fmt::Debug for _TunerDecoderLockType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_TunerDecoderLockType").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
