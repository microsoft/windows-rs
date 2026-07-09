#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct APO_CLASS_UUID(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIOENDPOINT_CLASS_UUID(pub u8);
pub const AUDIOLOOPBACK_TAPPOINT_CAPS_POSTVOLUMEMUTE: u32 = 2;
pub const AUDIOLOOPBACK_TAPPOINT_CAPS_PREVOLUMEMUTE: u32 = 1;
pub const AUDIOLOOPBACK_TAPPOINT_POSTVOLUMEMUTE: AUDIOLOOPBACK_TAPPOINT_TYPE = 1;
pub const AUDIOLOOPBACK_TAPPOINT_PREVOLUMEMUTE: AUDIOLOOPBACK_TAPPOINT_TYPE = 0;
pub type AUDIOLOOPBACK_TAPPOINT_TYPE = i32;
pub const AUDIOMODULE_MAX_DATA_SIZE: u32 = 64000;
pub const AUDIOMODULE_MAX_NAME_CCH_SIZE: u32 = 128;
pub type AUDIOPOSTURE_ORIENTATION = i32;
pub const AUDIOPOSTURE_ORIENTATION_NOTROTATED: AUDIOPOSTURE_ORIENTATION = 0;
pub const AUDIOPOSTURE_ORIENTATION_ROTATED180DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 2;
pub const AUDIOPOSTURE_ORIENTATION_ROTATED270DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 3;
pub const AUDIOPOSTURE_ORIENTATION_ROTATED90DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    pub ResourceGroupAcquired: windows_sys::core::BOOL,
    pub ResourceGroupName: [u16; 256],
}
impl Default for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AUDIO_CURVE_TYPE = i32;
pub const AUDIO_CURVE_TYPE_NONE: AUDIO_CURVE_TYPE = 0;
pub const AUDIO_CURVE_TYPE_WINDOWS_FADE: AUDIO_CURVE_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_AUTOMATIC_GAIN_CONTROL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_BASS_BOOST(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_BASS_MANAGEMENT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_BEAMFORMING(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_CONSTANT_TONE_REMOVAL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_DEEP_NOISE_SUPPRESSION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_DYNAMIC_RANGE_COMPRESSION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_ENVIRONMENTAL_EFFECTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_EQUALIZER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_FAR_FIELD_BEAMFORMING(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_LOUDNESS_EQUALIZER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_NOISE_SUPPRESSION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_ROOM_CORRECTION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_SPEAKER_COMPENSATION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_SPEAKER_FILL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_SPEAKER_PROTECTION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_VIRTUAL_HEADPHONES(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT_TYPE_VIRTUAL_SURROUND(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_SIGNALPROCESSINGMODE_COMMUNICATIONS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_SIGNALPROCESSINGMODE_DEFAULT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_SIGNALPROCESSINGMODE_FAR_FIELD_SPEECH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_SIGNALPROCESSINGMODE_MEDIA(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_SIGNALPROCESSINGMODE_MOVIE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_SIGNALPROCESSINGMODE_NOTIFICATION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_SIGNALPROCESSINGMODE_RAW(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_SIGNALPROCESSINGMODE_SPEECH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BLUETOOTHLE_MIDI_SERVICE_UUID(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BLUETOOTH_MIDI_DATAIO_CHARACTERISTIC(pub u8);
pub type CAPTURE_MEMORY_ALLOCATION_FLAGS = i32;
pub const CASCADE_FORM: KSDS3D_HRTF_FILTER_METHOD = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CC_BYTE_PAIR {
    pub Decoded: [u8; 2],
    pub Reserved: u16,
}
impl Default for CC_BYTE_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CC_HW_FIELD {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub fieldFlags: u32,
    pub PictureNumber: i64,
    pub Lines: [CC_BYTE_PAIR; 12],
}
impl Default for CC_HW_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CC_MAX_HW_DECODE_LINES: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLSID_KsIBasicAudioInterfaceHandler(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CODECAPI_ALLSETTINGS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CODECAPI_AUDIO_ENCODER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CODECAPI_CHANGELISTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CODECAPI_CURRENTCHANGELIST(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CODECAPI_SETALLDEFAULTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CODECAPI_SUPPORTSEVENTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CODECAPI_VIDEO_ENCODER(pub u8);
pub type CONSTRICTOR_OPTION = i32;
pub const CONSTRICTOR_OPTION_DISABLE: CONSTRICTOR_OPTION = 0;
pub const CONSTRICTOR_OPTION_MUTE: CONSTRICTOR_OPTION = 1;
pub const ConstantBitRate: VIDEOENCODER_BITRATE_MODE = 0;
pub const DDPF_FOURCC: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDPIXELFORMAT {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFourCC: u32,
    pub Anonymous: DDPIXELFORMAT_0,
    pub Anonymous2: DDPIXELFORMAT_1,
    pub Anonymous3: DDPIXELFORMAT_2,
    pub Anonymous4: DDPIXELFORMAT_3,
    pub Anonymous5: DDPIXELFORMAT_4,
}
impl Default for DDPIXELFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDPIXELFORMAT_0 {
    pub dwRGBBitCount: u32,
    pub dwYUVBitCount: u32,
    pub dwZBufferBitDepth: u32,
    pub dwAlphaBitDepth: u32,
}
impl Default for DDPIXELFORMAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDPIXELFORMAT_1 {
    pub dwRBitMask: u32,
    pub dwYBitMask: u32,
}
impl Default for DDPIXELFORMAT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDPIXELFORMAT_2 {
    pub dwGBitMask: u32,
    pub dwUBitMask: u32,
}
impl Default for DDPIXELFORMAT_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDPIXELFORMAT_3 {
    pub dwBBitMask: u32,
    pub dwVBitMask: u32,
}
impl Default for DDPIXELFORMAT_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDPIXELFORMAT_4 {
    pub dwRGBAlphaBitMask: u32,
    pub dwYUVAlphaBitMask: u32,
    pub dwRGBZBitMask: u32,
    pub dwYUVZBitMask: u32,
}
impl Default for DDPIXELFORMAT_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DDVIDEOPORTCONNECT {
    pub dwSize: u32,
    pub dwPortWidth: u32,
    pub guidTypeID: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub dwReserved1: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const DIRECT_FORM: KSDS3D_HRTF_FILTER_METHOD = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS3DVECTOR {
    pub Anonymous: DS3DVECTOR_0,
    pub Anonymous2: DS3DVECTOR_1,
    pub Anonymous3: DS3DVECTOR_2,
}
impl Default for DS3DVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_0 {
    pub x: f32,
    pub dvX: f32,
}
impl Default for DS3DVECTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_1 {
    pub y: f32,
    pub dvY: f32,
}
impl Default for DS3DVECTOR_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_2 {
    pub z: f32,
    pub dvZ: f32,
}
impl Default for DS3DVECTOR_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS3D_HRTF_VERSION_1: KSDS3D_HRTF_FILTER_VERSION = 0;
pub type EDeviceControlUseType = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ENCAPIPARAM_BITRATE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ENCAPIPARAM_BITRATE_MODE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ENCAPIPARAM_PEAK_BITRATE(pub u8);
pub type EPcxConnectionType = i32;
pub type EPcxGenLocation = i32;
pub const EPcxGenLocation_enum_count: EPcxGenLocation = 4;
pub type EPcxGeoLocation = i32;
pub const EPcxGeoLocation_enum_count: EPcxGeoLocation = 16;
pub type EPxcPortConnection = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EVENTSETID_CROSSBAR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EVENTSETID_TUNER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EVENTSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EVENTSETID_VIDEODECODER(pub u8);
pub const FLOAT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = 0;
pub const FULL_FILTER: KSDS3D_HRTF_FILTER_QUALITY = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    pub Size: u32,
    pub PrimaryChannelCount: u32,
    pub PrimaryChannelStartPosition: u32,
    pub PrimaryChannelMask: u32,
    pub InterleavedChannelCount: u32,
    pub InterleavedChannelStartPosition: u32,
    pub InterleavedChannelMask: u32,
}
pub const JACKDESC2_DYNAMIC_FORMAT_CHANGE_CAPABILITY: u32 = 2;
pub const JACKDESC2_PRESENCE_DETECT_CAPABILITY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAC3_ALTERNATE_AUDIO {
    pub fStereo: windows_sys::core::BOOL,
    pub DualMode: u32,
}
pub const KSAC3_ALTERNATE_AUDIO_1: u32 = 1;
pub const KSAC3_ALTERNATE_AUDIO_2: u32 = 2;
pub const KSAC3_ALTERNATE_AUDIO_BOTH: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAC3_BIT_STREAM_MODE {
    pub BitStreamMode: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAC3_DIALOGUE_LEVEL {
    pub DialogueLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAC3_DOWNMIX {
    pub fDownMix: windows_sys::core::BOOL,
    pub fDolbySurround: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAC3_ERROR_CONCEALMENT {
    pub fRepeatPreviousBlock: windows_sys::core::BOOL,
    pub fErrorInCurrentBlock: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAC3_ROOM_TYPE {
    pub fLargeRoom: windows_sys::core::BOOL,
}
pub const KSAC3_SERVICE_COMMENTARY: u32 = 5;
pub const KSAC3_SERVICE_DIALOG_ONLY: u32 = 4;
pub const KSAC3_SERVICE_EMERGENCY_FLASH: u32 = 6;
pub const KSAC3_SERVICE_HEARING_IMPAIRED: u32 = 3;
pub const KSAC3_SERVICE_MAIN_AUDIO: u32 = 0;
pub const KSAC3_SERVICE_NO_DIALOG: u32 = 1;
pub const KSAC3_SERVICE_VISUALLY_IMPAIRED: u32 = 2;
pub const KSAC3_SERVICE_VOICE_OVER: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSALGORITHMINSTANCE_SYSTEM_ACOUSTIC_ECHO_CANCEL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSALGORITHMINSTANCE_SYSTEM_AGC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSALGORITHMINSTANCE_SYSTEM_MICROPHONE_ARRAY_PROCESSOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSALGORITHMINSTANCE_SYSTEM_NOISE_SUPPRESS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSATTRIBUTEID_AUDIOLOOPBACK_TAPPOINT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSATTRIBUTEID_AUDIOSIGNALPROCESSING_MODE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSATTRIBUTEID_VIDEOFORMAT_DX12(pub u8);
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Default)]
pub struct KSATTRIBUTE_AUDIOLOOPBACK_TAPPOINT {
    pub AttributeHeader: super::ks::KSATTRIBUTE,
    pub TapPoint: AUDIOLOOPBACK_TAPPOINT_TYPE,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Default)]
pub struct KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    pub AttributeHeader: super::ks::KSATTRIBUTE,
    pub SignalProcessingMode: windows_sys::core::GUID,
}
pub const KSAUDDECOUTMODE_PCM_51: u32 = 2;
pub const KSAUDDECOUTMODE_SPDIFF: u32 = 4;
pub const KSAUDDECOUTMODE_STEREO_ANALOG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_3D_CENTER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_3D_DEPTH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_3D_STEREO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_ALTERNATE_MICROPHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_AUX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_AUX_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_AUX_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_BASS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_CD_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_CD_IN_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_CD_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_CD_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_LINE_IN(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_LINE_IN_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_LINE_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_LINE_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MASTER_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MASTER_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MICROPHONE_BOOST(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MIC_IN_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MIC_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MIC_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MIDI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MIDI_IN_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MIDI_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MIDI_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MIDRANGE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MONO_MIX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MONO_MIX_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MONO_MIX_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MONO_OUT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MONO_OUT_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_MONO_OUT_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_PC_SPEAKER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_PC_SPEAKER_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_PC_SPEAKER_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_PEAKMETER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_RECORDING_CONTROL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_RECORDING_SOURCE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_STEREO_MIX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_STEREO_MIX_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_STEREO_MIX_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_TREBLE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_VIDEO_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_VIDEO_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_VOLUME_CONTROL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_WAVE_IN_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_WAVE_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_WAVE_OUT_MIX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDFNAME_WAVE_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    pub MinBufferBytes: u32,
    pub MaxBufferBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIOENGINE_DESCRIPTOR {
    pub nHostPinId: u32,
    pub nOffloadPinId: u32,
    pub nLoopbackPinId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIOENGINE_DEVICECONTROLS {
    pub Volume: EDeviceControlUseType,
    pub Mute: EDeviceControlUseType,
    pub PeakMeter: EDeviceControlUseType,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIOENGINE_VOLUMELEVEL {
    pub TargetVolume: i32,
    pub CurveType: AUDIO_CURVE_TYPE,
    pub CurveDuration: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIOMODULE_DESCRIPTOR {
    pub ClassId: windows_sys::core::GUID,
    pub InstanceId: u32,
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub Name: [u16; 128],
}
impl Default for KSAUDIOMODULE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIOMODULE_NOTIFICATION {
    pub Anonymous: KSAUDIOMODULE_NOTIFICATION_0,
}
impl Default for KSAUDIOMODULE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSAUDIOMODULE_NOTIFICATION_0 {
    pub ProviderId: KSAUDIOMODULE_NOTIFICATION_0_0,
    pub Alignment: i64,
}
impl Default for KSAUDIOMODULE_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIOMODULE_NOTIFICATION_0_0 {
    pub DeviceId: windows_sys::core::GUID,
    pub ClassId: windows_sys::core::GUID,
    pub InstanceId: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSAUDIOMODULE_PROPERTY {
    pub Property: super::ks::KSPROPERTY,
    pub ClassId: windows_sys::core::GUID,
    pub InstanceId: u32,
}
#[cfg(feature = "ks")]
impl Default for KSAUDIOMODULE_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_CHANNEL_CONFIG {
    pub ActiveSpeakerPositions: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_COPY_PROTECTION {
    pub fCopyrighted: windows_sys::core::BOOL,
    pub fOriginal: windows_sys::core::BOOL,
}
pub const KSAUDIO_CPU_RESOURCES_HOST_CPU: u32 = 2147483647;
pub const KSAUDIO_CPU_RESOURCES_NOT_HOST_CPU: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_DYNAMIC_RANGE {
    pub QuietCompression: u32,
    pub LoudCompression: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_MICROPHONE_COORDINATES {
    pub usType: u16,
    pub wXCoord: i16,
    pub wYCoord: i16,
    pub wZCoord: i16,
    pub wVerticalAngle: i16,
    pub wHorizontalAngle: i16,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIO_MIXCAP_TABLE {
    pub InputChannels: u32,
    pub OutputChannels: u32,
    pub Capabilities: [KSAUDIO_MIX_CAPS; 1],
}
impl Default for KSAUDIO_MIXCAP_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_MIXLEVEL {
    pub Mute: windows_sys::core::BOOL,
    pub Level: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIO_MIX_CAPS {
    pub Mute: windows_sys::core::BOOL,
    pub Minimum: i32,
    pub Maximum: i32,
    pub Anonymous: KSAUDIO_MIX_CAPS_0,
}
impl Default for KSAUDIO_MIX_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSAUDIO_MIX_CAPS_0 {
    pub Reset: i32,
    pub Resolution: i32,
}
impl Default for KSAUDIO_MIX_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS {
    pub MinPacketPeriodInHns: u32,
    pub PacketSizeFileAlignment: u32,
    pub Reserved: u32,
    pub NumProcessingModeConstraints: u32,
    pub ProcessingModeConstraints: [KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT; 1],
}
impl Default for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    pub MinPacketPeriodInHns: u32,
    pub PacketSizeFileAlignment: u32,
    pub MaxPacketSizeInBytes: u32,
    pub NumProcessingModeConstraints: u32,
    pub ProcessingModeConstraints: [KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT; 1],
}
impl Default for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT {
    pub ProcessingMode: windows_sys::core::GUID,
    pub SamplesPerProcessingPacket: u32,
    pub ProcessingPacketDurationInHns: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_POSITION {
    pub PlayOffset: super::winnt::DWORDLONG,
    pub WriteOffset: super::winnt::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_POSITIONEX {
    pub TimerFrequency: i64,
    pub TimeStamp1: i64,
    pub Position: KSAUDIO_POSITION,
    pub TimeStamp2: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSAUDIO_PRESENTATION_POSITION {
    pub u64PositionInBlocks: u64,
    pub u64QPCPosition: u64,
}
pub const KSAUDIO_QUALITY_ADVANCED: u32 = 3;
pub const KSAUDIO_QUALITY_BASIC: u32 = 2;
pub const KSAUDIO_QUALITY_PC: u32 = 1;
pub const KSAUDIO_QUALITY_WORST: u32 = 0;
pub const KSAUDIO_SPEAKER_1POINT1: u32 = 12;
pub const KSAUDIO_SPEAKER_2POINT1: u32 = 11;
pub const KSAUDIO_SPEAKER_3POINT0: u32 = 7;
pub const KSAUDIO_SPEAKER_3POINT1: u32 = 15;
pub const KSAUDIO_SPEAKER_5POINT0: u32 = 1543;
pub const KSAUDIO_SPEAKER_5POINT1: u32 = 63;
pub const KSAUDIO_SPEAKER_5POINT1_BACK: u32 = 63;
pub const KSAUDIO_SPEAKER_5POINT1_SURROUND: u32 = 1551;
pub const KSAUDIO_SPEAKER_7POINT0: u32 = 1591;
pub const KSAUDIO_SPEAKER_7POINT1: u32 = 255;
pub const KSAUDIO_SPEAKER_7POINT1_SURROUND: u32 = 1599;
pub const KSAUDIO_SPEAKER_7POINT1_WIDE: u32 = 255;
pub const KSAUDIO_SPEAKER_DIRECTOUT: u32 = 0;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_CENTER: u32 = 4;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_LEFT: u32 = 1;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_RIGHT: u32 = 2;
pub const KSAUDIO_SPEAKER_GROUND_REAR_LEFT: u32 = 16;
pub const KSAUDIO_SPEAKER_GROUND_REAR_RIGHT: u32 = 32;
pub const KSAUDIO_SPEAKER_MONO: u32 = 4;
pub const KSAUDIO_SPEAKER_QUAD: u32 = 51;
pub const KSAUDIO_SPEAKER_STEREO: u32 = 3;
pub const KSAUDIO_SPEAKER_SUPER_WOOFER: u32 = 8;
pub const KSAUDIO_SPEAKER_SURROUND: u32 = 263;
pub const KSAUDIO_SPEAKER_TOP_MIDDLE: u32 = 2048;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_HEADPHONE: i32 = -1;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MAX: u32 = 180;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MIN: u32 = 5;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_NARROW: u32 = 10;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_WIDE: u32 = 20;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_BalancedVideoAndPhoto(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_CompressedCamera(pub u8);
pub const KSCAMERAPROFILE_FLAGS_FACEDETECTION: u32 = 8;
pub const KSCAMERAPROFILE_FLAGS_PHOTOHDR: u32 = 4;
pub const KSCAMERAPROFILE_FLAGS_PREVIEW_RES_MUSTMATCH: u32 = 32;
pub const KSCAMERAPROFILE_FLAGS_VARIABLEPHOTOSEQUENCE: u32 = 16;
pub const KSCAMERAPROFILE_FLAGS_VIDEOHDR: u32 = 2;
pub const KSCAMERAPROFILE_FLAGS_VIDEOSTABLIZATION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_FaceAuth_Mode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_HDRWithWCGPhoto(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_HDRWithWCGVideo(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_HighFrameRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_HighQualityPhoto(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_Legacy(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_PhotoSequence(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_VariablePhotoSequence(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_VideoConferencing(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_VideoHDR8(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERAPROFILE_VideoRecording(pub u8);
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_AUTO: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_FNF: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_HDR: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_ULTRALOWLIGHT: u32 = 8;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_BLUR: u32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    pub Resolution: super::windef::SIZE,
    pub MaxFrameRate: KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0,
    pub MaskResolution: super::windef::SIZE,
    pub SubType: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    pub Numerator: i32,
    pub Denominator: i32,
}
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_MASK: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_SHALLOWFOCUS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    pub PitchAngle: i32,
    pub YawAngle: i32,
    pub Flag: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_EXTENDEDPROP_CAPS_ASYNCCONTROL: u64 = 9223372036854775808;
pub const KSCAMERA_EXTENDEDPROP_CAPS_CANCELLABLE: u64 = 4611686018427387904;
pub const KSCAMERA_EXTENDEDPROP_CAPS_MASK: u64 = 18374686479671623680;
pub const KSCAMERA_EXTENDEDPROP_CAPS_RESERVED: u64 = 18374686479671623680;
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_AUTOFACEFRAMING: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    pub Size: u32,
    pub Count: u32,
}
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_MANUAL: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_MASK: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    pub OriginX: i32,
    pub OriginY: i32,
    pub WindowSize: i32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Value: i32,
    pub Reserved: u64,
}
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_FULLSTEP: u32 = 16;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_HALFSTEP: u32 = 8;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_QUARTERSTEP: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_SIXTHSTEP: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_THIRDSTEP: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_STARE: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_ALTERNATIVE_FRAME_ILLUMINATION: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_BACKGROUND_SUBTRACTION: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_DISABLED: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_ADVANCED_MASK: u32 = 24;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_BLINK: u32 = 8;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_MASK: u32 = 7;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PHOTO: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PREVIEW: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_SMILE: u32 = 16;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_VIDEO: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    pub NormalizedFocalLengthX: u32,
    pub NormalizedFocalLengthY: u32,
    pub Flag: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_EXTENDEDPROP_FIELDOFVIEW2_CONFIGCAPS {
    pub DefaultDiagonalFieldOfViewInDegrees: u16,
    pub DiscreteFoVStopsCount: u16,
    pub DiscreteFoVStops: [u16; 360],
    pub Reserved: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW2_CONFIGCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_FILTERSCOPE: u32 = 4294967295;
pub const KSCAMERA_EXTENDEDPROP_FLAG_CANCELOPERATION: u64 = 9223372036854775808;
pub const KSCAMERA_EXTENDEDPROP_FLAG_MASK: u64 = 18374686479671623680;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_AUTO: u32 = 256;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_MASK: u32 = 384;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_ON: u32 = 128;
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO_ADJUSTABLEPOWER: u32 = 8;
pub const KSCAMERA_EXTENDEDPROP_FLASH_MODE_MASK: u32 = 15;
pub const KSCAMERA_EXTENDEDPROP_FLASH_MULTIFLASHSUPPORTED: u32 = 64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON_ADJUSTABLEPOWER: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_FLASH_REDEYEREDUCTION: u32 = 16;
pub const KSCAMERA_EXTENDEDPROP_FLASH_SINGLEFLASH: u32 = 32;
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_ON: u32 = 1;
pub type KSCAMERA_EXTENDEDPROP_FOCUSSTATE = i32;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FAILED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 4;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FOCUSED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 3;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_LOST: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 1;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_SEARCHING: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 2;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_UNINITIALIZED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 0;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUS: u32 = 256;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUSLOCK: u32 = 512;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_HYPERFOCAL: u32 = 33554432;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_INFINITY: u32 = 16777216;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_MASK: u32 = 117440512;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_NEAREST: u32 = 67108864;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DRIVERFALLBACK_OFF: u32 = 2048;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_MODE_ADVANCED_MASK: u32 = 7680;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_MODE_MASK: u32 = 263;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_FULLRANGE: u32 = 262144;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_HYPERFOCAL: u32 = 1048576;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_INFINITY: u32 = 524288;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_MACRO: u32 = 65536;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_MASK: u32 = 2031616;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_NORMAL: u32 = 131072;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_REGIONBASED: u32 = 4096;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_UNLOCK: u32 = 1024;
pub const KSCAMERA_EXTENDEDPROP_FRAMERATE_THROTTLE_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FRAMERATE_THROTTLE_ON: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_HEADER {
    pub Version: u32,
    pub PinId: u32,
    pub Size: u32,
    pub Result: u32,
    pub Flags: u64,
    pub Capability: u64,
}
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALTERNATING_FRAME_ILLUMINATION: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALWAYS_ON: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_OFF: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_ISO_100: u32 = 8;
pub const KSCAMERA_EXTENDEDPROP_ISO_12800: u32 = 1024;
pub const KSCAMERA_EXTENDEDPROP_ISO_1600: u32 = 128;
pub const KSCAMERA_EXTENDEDPROP_ISO_200: u32 = 16;
pub const KSCAMERA_EXTENDEDPROP_ISO_25600: u32 = 2048;
pub const KSCAMERA_EXTENDEDPROP_ISO_3200: u32 = 256;
pub const KSCAMERA_EXTENDEDPROP_ISO_400: u32 = 32;
pub const KSCAMERA_EXTENDEDPROP_ISO_50: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_ISO_6400: u32 = 512;
pub const KSCAMERA_EXTENDEDPROP_ISO_80: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_ISO_800: u32 = 64;
pub const KSCAMERA_EXTENDEDPROP_ISO_AUTO: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_ISO_MANUAL: u64 = 36028797018963968;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_METADATAINFO {
    pub BufferAlignment: i32,
    pub MaxMetadataBufferSize: u32,
}
pub const KSCAMERA_EXTENDEDPROP_METADATA_ALIGNMENTREQUIRED: u32 = 256;
pub const KSCAMERA_EXTENDEDPROP_METADATA_MEMORYTYPE_MASK: u32 = 255;
pub const KSCAMERA_EXTENDEDPROP_METADATA_SYSTEMMEMORY: u32 = 1;
pub type KSCAMERA_EXTENDEDPROP_MetadataAlignment = i32;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_1024: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 10;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_128: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 7;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_16: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 4;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_2048: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 11;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_256: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 8;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_32: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 5;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_4096: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 12;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_512: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 9;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_64: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 6;
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_8192: KSCAMERA_EXTENDEDPROP_MetadataAlignment = 13;
pub const KSCAMERA_EXTENDEDPROP_OIS_AUTO: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_OIS_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_OIS_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_DEFAULT: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_LATENCY: u32 = 8;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PERF_MASK: u32 = 28;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PHOTO: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_POWER: u32 = 16;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PRIMARYUSE_MASK: u32 = 3;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_QUALITY: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_VIDEO: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_ON: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    pub RequestedHistoryFrames: u32,
    pub MaxHistoryFrames: u32,
    pub SubMode: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_NORMAL: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_NONE: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_VARIABLE: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_16X: u32 = 8;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_2X: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_4X: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_8X: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_DISABLE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_PROFILE {
    pub ProfileId: windows_sys::core::GUID,
    pub Index: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_DYNAMIC: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_MASK: u32 = 3;
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_ON: u32 = 1;
pub type KSCAMERA_EXTENDEDPROP_ROITYPE = i32;
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_FACE: KSCAMERA_EXTENDEDPROP_ROITYPE = 1;
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_UNKNOWN: KSCAMERA_EXTENDEDPROP_ROITYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    pub ControlId: u32,
    pub MaxNumberOfROIs: u32,
    pub Capability: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    pub Size: u32,
    pub ConfigCapCount: u32,
    pub Reserved: u64,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_INFO {
    pub Region: super::windef::RECT,
    pub Flags: u64,
    pub Weight: i32,
    pub RegionOfInterestType: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    pub ControlId: u32,
    pub ROICount: u32,
    pub Result: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    pub Size: u32,
    pub ControlCount: u32,
    pub Reserved: u64,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_AUTO: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BACKLIT: u32 = 1024;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BEACH: u32 = 32;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_CANDLELIGHT: u32 = 128;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_LANDSCAPE: u32 = 256;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MACRO: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MANUAL: u64 = 36028797018963968;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHT: u32 = 16;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHTPORTRAIT: u32 = 512;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_PORTRAIT: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SNOW: u32 = 8;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SPORT: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SUNSET: u32 = 64;
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_DISABLED: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_ENABLED: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_EXTENDEDPROP_VALUE {
    pub Value: KSCAMERA_EXTENDEDPROP_VALUE_0,
}
impl Default for KSCAMERA_EXTENDEDPROP_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSCAMERA_EXTENDEDPROP_VALUE_0 {
    pub dbl: f64,
    pub ull: u64,
    pub ul: u32,
    pub ratio: u64,
    pub l: i32,
    pub ll: i64,
}
impl Default for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_VFR_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_VFR_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_AUTO: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_AUTO: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_LOCK: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_MANUAL: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_MASK: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Step: i32,
    pub VideoProc: KSCAMERA_EXTENDEDPROP_VALUE,
    pub Reserved: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_AUTO: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_AUTO: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_OFF: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_ON: u32 = 4;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_OFF: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON_ADJUSTABLEPOWER: u32 = 2;
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_DISABLED: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_ENABLED: u32 = 1;
pub type KSCAMERA_EXTENDEDPROP_WBPRESET = i32;
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CANDLELIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = 6;
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CLOUDY: KSCAMERA_EXTENDEDPROP_WBPRESET = 1;
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_DAYLIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = 2;
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLASH: KSCAMERA_EXTENDEDPROP_WBPRESET = 3;
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLUORESCENT: KSCAMERA_EXTENDEDPROP_WBPRESET = 4;
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_TUNGSTEN: KSCAMERA_EXTENDEDPROP_WBPRESET = 5;
pub type KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = i32;
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_PRESET: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = 2;
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_TEMPERATURE: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = 1;
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DEFAULT: u32 = 0;
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DIRECT: u32 = 1;
pub const KSCAMERA_EXTENDEDPROP_ZOOM_SMOOTH: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    pub PhotoResWidth: u32,
    pub PhotoResHeight: u32,
    pub PreviewFPSNum: u32,
    pub PreviewFPSDenom: u32,
    pub CaptureFPSNum: u32,
    pub CaptureFPSDenom: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub MaskCoverageBoundingBox: super::windef::RECT,
    pub MaskResolution: super::windef::SIZE,
    pub ForegroundBoundingBox: super::windef::RECT,
    pub MaskData: [u8; 1],
}
#[cfg(feature = "windef")]
impl Default for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURECOMPENSATION: u32 = 2;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURETIME: u32 = 1;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASH: u32 = 64;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASHPOWER: u32 = 128;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FOCUSSTATE: u32 = 8;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ISOSPEED: u32 = 4;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_LENSPOSITION: u32 = 16;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SCENEMODE: u32 = 512;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SENSORFRAMERATE: u32 = 1024;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_WHITEBALANCE: u32 = 32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ZOOMFACTOR: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_METADATA_DIGITALWINDOW {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Window: KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_METADATA_FRAMEILLUMINATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Flags: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_METADATA_FRAMEILLUMINATION_FLAG_ON: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_METADATA_ITEMHEADER {
    pub MetadataId: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_METADATA_PHOTOCONFIRMATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub PhotoConfirmationIndex: u32,
    pub Reserved: u32,
}
pub type KSCAMERA_MetadataId = i32;
pub const KSCAMERA_PERFRAMESETTING_AUTO: u64 = 4294967296;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    pub Size: u32,
    pub ItemCount: u32,
    pub Flags: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    pub Size: u32,
    pub Reserved: u32,
    pub Id: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    pub Size: u32,
    pub Id: u32,
    pub ItemCount: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PERFRAMESETTING_HEADER {
    pub Size: u32,
    pub FrameCount: u32,
    pub Id: windows_sys::core::GUID,
    pub Flags: u64,
    pub LoopCount: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_PERFRAMESETTING_ITEM_CUSTOM: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 7;
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_COMPENSATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 3;
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_TIME: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 1;
pub const KSCAMERA_PERFRAMESETTING_ITEM_FLASH: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 2;
pub const KSCAMERA_PERFRAMESETTING_ITEM_FOCUS: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
pub const KSCAMERA_PERFRAMESETTING_ITEM_ISO: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 4;
pub const KSCAMERA_PERFRAMESETTING_ITEM_PHOTOCONFIRMATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 6;
pub type KSCAMERA_PERFRAMESETTING_ITEM_TYPE = i32;
pub const KSCAMERA_PERFRAMESETTING_MANUAL: u64 = 8589934592;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_PROFILE_CONCURRENCYINFO {
    pub ReferenceGuid: windows_sys::core::GUID,
    pub Reserved: u32,
    pub ProfileCount: u32,
    pub Profiles: PKSCAMERA_PROFILE_INFO,
}
impl Default for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_PROFILE_INFO {
    pub ProfileId: windows_sys::core::GUID,
    pub Index: u32,
    pub PinCount: u32,
    pub Pins: PKSCAMERA_PROFILE_PININFO,
}
impl Default for KSCAMERA_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PROFILE_MEDIAINFO {
    pub Resolution: KSCAMERA_PROFILE_MEDIAINFO_0,
    pub MaxFrameRate: KSCAMERA_PROFILE_MEDIAINFO_1,
    pub Flags: u64,
    pub Data0: u32,
    pub Data1: u32,
    pub Data2: u32,
    pub Data3: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PROFILE_MEDIAINFO_0 {
    pub X: u32,
    pub Y: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PROFILE_MEDIAINFO_1 {
    pub Numerator: u32,
    pub Denominator: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_PROFILE_PININFO {
    pub PinCategory: windows_sys::core::GUID,
    pub Anonymous: KSCAMERA_PROFILE_PININFO_0,
    pub MediaInfoCount: u32,
    pub MediaInfos: PKSCAMERA_PROFILE_MEDIAINFO,
}
impl Default for KSCAMERA_PROFILE_PININFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSCAMERA_PROFILE_PININFO_0 {
    pub Anonymous: KSCAMERA_PROFILE_PININFO_0_0,
    pub Reserved: u32,
}
impl Default for KSCAMERA_PROFILE_PININFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCAMERA_PROFILE_PININFO_0_0 {
    pub PinIndex: u16,
    pub ProfileSensorType: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_ACOUSTIC_ECHO_CANCEL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_CROSSBAR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_ENCODER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_ESCALANTE_PLATFORM_DRIVER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_MICROPHONE_ARRAY_PROCESSOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_MULTIPLEXER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_NETWORK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_REALTIME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_TEXT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_TOPOLOGY(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_TVAUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_TVTUNER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_VBICODEC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_VIRTUAL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_VPMUX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCATEGORY_WDMAUD_USE_PIN_NAME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSCOMPONENTID_USBAUDIO(pub u8);
pub const KSCameraProfileSensorType_Custom: u32 = 128;
pub const KSCameraProfileSensorType_Depth: u32 = 4;
pub const KSCameraProfileSensorType_ImageSegmentation: u32 = 16;
pub const KSCameraProfileSensorType_Infrared: u32 = 2;
pub const KSCameraProfileSensorType_PoseTracking: u32 = 8;
pub const KSCameraProfileSensorType_RGB: u32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "ks", feature = "mmeapi"))]
#[derive(Clone, Copy)]
pub struct KSDATAFORMAT_DSOUND {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub BufferDesc: KSDSOUND_BUFFERDESC,
}
#[cfg(all(feature = "ks", feature = "mmeapi"))]
impl Default for KSDATAFORMAT_DSOUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_AC3_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_ANALOGVIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_AC3_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_DSOUND(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_H264_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_IMAGE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_JPEG_IMAGE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_LPCM_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_MPEG1_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_MPEG2_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_MPEG2_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_VBI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_VC_ID(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_VIDEOINFO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_VIDEOINFO2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SPECIFIER_WAVEFORMATEX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_AC3_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_ADPCM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_ALAW(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_ANALOG(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_CC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_D16(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_DRM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_DSS_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_DSS_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_DTS_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_AAC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_ATRAC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS_ATMOS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT20(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21_PROFILE4(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MLP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DST(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E1(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DTS_HD(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEG1(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEG2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEG3(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL1_BL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL1_LC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL2_BL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL2_LC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL3_BL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL3_LC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL4_BL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL4_LC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL5_BL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL5_LC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_ONE_BIT_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_WMA_PRO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_IMAGE_RGB32(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_JPEG(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_L16(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_L16_CUSTOM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_L16_IR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_L8(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_L8_CUSTOM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_L8_IR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_LPCM_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_Line21_BytePair(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_Line21_GOPPacket(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MIDI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MIDI_BUS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MJPG_CUSTOM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MJPG_DEPTH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MJPG_IR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MPEG(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MPEG1Packet(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MPEG1Payload(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MPEG1Video(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MPEG2_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MPEG2_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MPEGLAYER3(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MPEG_HEAAC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_MULAW(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_NABTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_NABTS_FEC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_OVERLAY(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_RAW8(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_RIFF(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_RIFFMIDI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_RIFFWAVE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_SDDS_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_AC3_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_VIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_SUBPICTURE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_TELETEXT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_UNIVERSALMIDIPACKET(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_VPVBI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_VPVideo(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_WMAUDIO2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_WMAUDIO3(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_SUBTYPE_WMAUDIO_LOSSLESS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_ANALOGAUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_ANALOGVIDEO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_AUXLine21Data(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_DVD_ENCRYPTED_PACK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_IMAGE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_MIDI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_MPEG2_PES(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_MPEG2_PROGRAM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_MPEG2_TRANSPORT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_MUSIC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_NABTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_STANDARD_ELEMENTARY_STREAM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_STANDARD_PACK_HEADER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_STANDARD_PES_PACKET(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_TEXT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_VBI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDATAFORMAT_TYPE_VIDEO(pub u8);
#[repr(C, packed(1))]
#[cfg(all(feature = "ks", feature = "mmeapi"))]
#[derive(Clone, Copy)]
pub struct KSDATAFORMAT_WAVEFORMATEX {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub WaveFormatEx: super::mmeapi::WAVEFORMATEX,
}
#[cfg(all(feature = "ks", feature = "mmeapi"))]
impl Default for KSDATAFORMAT_WAVEFORMATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "ks", feature = "mmeapi", feature = "mmreg"))]
#[derive(Clone, Copy)]
pub struct KSDATAFORMAT_WAVEFORMATEXTENSIBLE {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub WaveFormatExt: super::mmreg::WAVEFORMATEXTENSIBLE,
}
#[cfg(all(feature = "ks", feature = "mmeapi", feature = "mmreg"))]
impl Default for KSDATAFORMAT_WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSDATARANGE_AUDIO {
    pub DataRange: super::ks::KSDATARANGE,
    pub MaximumChannels: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
}
#[cfg(feature = "ks")]
impl Default for KSDATARANGE_AUDIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSDATARANGE_MUSIC {
    pub DataRange: super::ks::KSDATARANGE,
    pub Technology: windows_sys::core::GUID,
    pub Channels: u32,
    pub Notes: u32,
    pub ChannelMask: u32,
}
#[cfg(feature = "ks")]
impl Default for KSDATARANGE_MUSIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDEVICE_PROFILE_INFO {
    pub Type: u32,
    pub Size: u32,
    pub Anonymous: KSDEVICE_PROFILE_INFO_0,
}
impl Default for KSDEVICE_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSDEVICE_PROFILE_INFO_0 {
    pub Camera: KSDEVICE_PROFILE_INFO_0_0,
}
impl Default for KSDEVICE_PROFILE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDEVICE_PROFILE_INFO_0_0 {
    pub Info: KSCAMERA_PROFILE_INFO,
    pub Reserved: u32,
    pub ConcurrencyCount: u32,
    pub Concurrency: PKSCAMERA_PROFILE_CONCURRENCYINFO,
}
impl Default for KSDEVICE_PROFILE_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDEVICE_PROFILE_TYPE_CAMERA: u32 = 1;
pub const KSDEVICE_PROFILE_TYPE_UNKNOWN: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDISPLAYCHANGE {
    pub PelsWidth: u32,
    pub PelsHeight: u32,
    pub BitsPerPel: u32,
    pub DeviceID: [u16; 1],
}
impl Default for KSDISPLAYCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for KSDS3D_BUFFER_ALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDS3D_BUFFER_CONE_ANGLES {
    pub InsideConeAngle: u32,
    pub OutsideConeAngle: u32,
}
pub const KSDS3D_COEFF_COUNT: KSDS3D_HRTF_COEFF_FORMAT = 2;
pub const KSDS3D_FILTER_METHOD_COUNT: KSDS3D_HRTF_FILTER_METHOD = 2;
pub const KSDS3D_FILTER_QUALITY_COUNT: KSDS3D_HRTF_FILTER_QUALITY = 2;
pub type KSDS3D_HRTF_COEFF_FORMAT = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDS3D_HRTF_FILTER_FORMAT_MSG {
    pub FilterMethod: KSDS3D_HRTF_FILTER_METHOD,
    pub CoeffFormat: KSDS3D_HRTF_COEFF_FORMAT,
    pub Version: KSDS3D_HRTF_FILTER_VERSION,
    pub Reserved: u32,
}
pub type KSDS3D_HRTF_FILTER_METHOD = i32;
pub type KSDS3D_HRTF_FILTER_QUALITY = i32;
pub type KSDS3D_HRTF_FILTER_VERSION = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDS3D_HRTF_PARAMS_MSG {
    pub Size: u32,
    pub Enabled: u32,
    pub SwapChannels: windows_sys::core::BOOL,
    pub ZeroAzimuth: windows_sys::core::BOOL,
    pub CrossFadeOutput: windows_sys::core::BOOL,
    pub FilterSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDS3D_ITD_PARAMS {
    pub Channel: i32,
    pub VolSmoothScale: f32,
    pub TotalDryAttenuation: f32,
    pub TotalWetAttenuation: f32,
    pub SmoothFrequency: i32,
    pub Delay: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSDS3D_ITD_PARAMS_MSG {
    pub Enabled: u32,
    pub LeftParams: KSDS3D_ITD_PARAMS,
    pub RightParams: KSDS3D_ITD_PARAMS,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDS3D_LISTENER_ALL {
    pub Position: DS3DVECTOR,
    pub Velocity: DS3DVECTOR,
    pub OrientFront: DS3DVECTOR,
    pub OrientTop: DS3DVECTOR,
    pub DistanceFactor: f32,
    pub RolloffFactor: f32,
    pub DopplerFactor: f32,
}
impl Default for KSDS3D_LISTENER_ALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDS3D_LISTENER_ORIENTATION {
    pub Front: DS3DVECTOR,
    pub Top: DS3DVECTOR,
}
impl Default for KSDS3D_LISTENER_ORIENTATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDSOUND_3D_MODE_DISABLE: u32 = 2;
pub const KSDSOUND_3D_MODE_HEADRELATIVE: u32 = 1;
pub const KSDSOUND_3D_MODE_NORMAL: u32 = 0;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct KSDSOUND_BUFFERDESC {
    pub Flags: u32,
    pub Control: u32,
    pub WaveFormatEx: super::mmeapi::WAVEFORMATEX,
}
pub const KSDSOUND_BUFFER_CTRL_3D: u32 = 1;
pub const KSDSOUND_BUFFER_CTRL_FREQUENCY: u32 = 2;
pub const KSDSOUND_BUFFER_CTRL_HRTF_3D: u32 = 1073741824;
pub const KSDSOUND_BUFFER_CTRL_PAN: u32 = 4;
pub const KSDSOUND_BUFFER_CTRL_POSITIONNOTIFY: u32 = 16;
pub const KSDSOUND_BUFFER_CTRL_VOLUME: u32 = 8;
pub const KSDSOUND_BUFFER_LOCHARDWARE: u32 = 4;
pub const KSDSOUND_BUFFER_LOCSOFTWARE: u32 = 8;
pub const KSDSOUND_BUFFER_PRIMARY: u32 = 1;
pub const KSDSOUND_BUFFER_STATIC: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_AudioControlChange(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_CameraAsyncControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_CameraEvent(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_DynamicFormatChange(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_EXTDEV_Command(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_ExtendedCameraControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_LoopedStreaming(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_SoundDetector(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_Telephony(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_VIDCAPTOSTI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_VIDCAP_TVAUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_VPNotify(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSEVENTSETID_VPVBINotify(pub u8);
pub type KSEVENT_AUDIO_CONTROL_CHANGE = i32;
pub type KSEVENT_CAMERACONTROL = i32;
pub const KSEVENT_CAMERACONTROL_FOCUS: KSEVENT_CAMERACONTROL = 0;
pub const KSEVENT_CAMERACONTROL_ZOOM: KSEVENT_CAMERACONTROL = 1;
pub type KSEVENT_CAMERAEVENT = i32;
pub const KSEVENT_CONTROL_CHANGE: KSEVENT_AUDIO_CONTROL_CHANGE = 0;
pub type KSEVENT_CROSSBAR = i32;
pub const KSEVENT_CROSSBAR_CHANGED: KSEVENT_CROSSBAR = 0;
pub type KSEVENT_DEVCMD = i32;
pub type KSEVENT_DYNAMICFORMATCHANGE = i32;
pub const KSEVENT_DYNAMIC_FORMAT_CHANGE: KSEVENT_DYNAMICFORMATCHANGE = 0;
pub const KSEVENT_EXTDEV_COMMAND_BUSRESET: KSEVENT_DEVCMD = 2;
pub const KSEVENT_EXTDEV_COMMAND_CONTROL_INTERIM_READY: KSEVENT_DEVCMD = 1;
pub const KSEVENT_EXTDEV_COMMAND_NOTIFY_INTERIM_READY: KSEVENT_DEVCMD = 0;
pub const KSEVENT_EXTDEV_NOTIFY_MEDIUM_CHANGE: KSEVENT_DEVCMD = 7;
pub const KSEVENT_EXTDEV_NOTIFY_REMOVAL: KSEVENT_DEVCMD = 6;
pub const KSEVENT_EXTDEV_OPERATION_MODE_UPDATE: KSEVENT_DEVCMD = 4;
pub const KSEVENT_EXTDEV_TIMECODE_UPDATE: KSEVENT_DEVCMD = 3;
pub const KSEVENT_EXTDEV_TRANSPORT_STATE_UPDATE: KSEVENT_DEVCMD = 5;
pub type KSEVENT_LOOPEDSTREAMING = i32;
pub const KSEVENT_LOOPEDSTREAMING_POSITION: KSEVENT_LOOPEDSTREAMING = 0;
pub const KSEVENT_PHOTO_SAMPLE_SCANNED: KSEVENT_CAMERAEVENT = 0;
pub type KSEVENT_SOUNDDETECTOR = i32;
pub const KSEVENT_SOUNDDETECTOR_MATCHDETECTED: KSEVENT_SOUNDDETECTOR = 1;
pub type KSEVENT_TELEPHONY = i32;
pub const KSEVENT_TELEPHONY_ENDPOINTPAIRS_CHANGED: KSEVENT_TELEPHONY = 0;
pub type KSEVENT_TUNER = i32;
pub const KSEVENT_TUNER_CHANGED: KSEVENT_TUNER = 0;
pub const KSEVENT_TUNER_INITIATE_SCAN: KSEVENT_TUNER = 1;
#[repr(C)]
#[cfg(all(feature = "ks", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KSEVENT_TUNER_INITIATE_SCAN_S {
    pub EventData: super::ks::KSEVENTDATA,
    pub StartFrequency: u32,
    pub EndFrequency: u32,
}
#[cfg(all(feature = "ks", feature = "winnt"))]
impl Default for KSEVENT_TUNER_INITIATE_SCAN_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KSEVENT_TVAUDIO = i32;
pub const KSEVENT_TVAUDIO_CHANGED: KSEVENT_TVAUDIO = 0;
pub type KSEVENT_VIDCAPTOSTI = i32;
pub const KSEVENT_VIDCAPTOSTI_EXT_TRIGGER: KSEVENT_VIDCAPTOSTI = 0;
pub const KSEVENT_VIDCAP_AUTO_UPDATE: KSEVENT_VIDCAPTOSTI = 1;
pub const KSEVENT_VIDCAP_SEARCH: KSEVENT_VIDCAPTOSTI = 2;
pub type KSEVENT_VIDEODECODER = i32;
pub const KSEVENT_VIDEODECODER_CHANGED: KSEVENT_VIDEODECODER = 0;
pub type KSEVENT_VPNOTIFY = i32;
pub const KSEVENT_VPNOTIFY_FORMATCHANGE: KSEVENT_VPNOTIFY = 0;
pub type KSEVENT_VPVBINOTIFY = i32;
pub const KSEVENT_VPVBINOTIFY_FORMATCHANGE: KSEVENT_VPVBINOTIFY = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSGOP_USERDATA {
    pub sc: u32,
    pub reserved1: u32,
    pub cFields: u8,
    pub l21Data: [i8; 3],
}
impl Default for KSGOP_USERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSINTERFACESETID_Media(pub u8);
pub type KSINTERFACE_MEDIA = i32;
pub const KSINTERFACE_MEDIA_MUSIC: KSINTERFACE_MEDIA = 0;
pub const KSINTERFACE_MEDIA_WAVE_BUFFERED: KSINTERFACE_MEDIA = 1;
pub const KSINTERFACE_MEDIA_WAVE_QUEUED: KSINTERFACE_MEDIA = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSJACK_DESCRIPTION {
    pub ChannelMapping: u32,
    pub Color: u32,
    pub ConnectionType: EPcxConnectionType,
    pub GeoLocation: EPcxGeoLocation,
    pub GenLocation: EPcxGenLocation,
    pub PortConnection: EPxcPortConnection,
    pub IsConnected: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSJACK_DESCRIPTION2 {
    pub DeviceStateInfo: u32,
    pub JackCapabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSJACK_DESCRIPTION3 {
    pub ConfigId: u32,
}
pub type KSJACK_SINK_CONNECTIONTYPE = i32;
pub const KSJACK_SINK_CONNECTIONTYPE_DISPLAYPORT: KSJACK_SINK_CONNECTIONTYPE = 1;
pub const KSJACK_SINK_CONNECTIONTYPE_HDMI: KSJACK_SINK_CONNECTIONTYPE = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSJACK_SINK_INFORMATION {
    pub ConnType: KSJACK_SINK_CONNECTIONTYPE,
    pub ManufacturerId: u16,
    pub ProductId: u16,
    pub AudioLatency: u16,
    pub HDCPCapable: windows_sys::core::BOOL,
    pub AICapable: windows_sys::core::BOOL,
    pub SinkDescriptionLength: u8,
    pub SinkDescription: [u16; 32],
    pub PortId: super::winnt::LUID,
}
#[cfg(feature = "winnt")]
impl Default for KSJACK_SINK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMEDIUMSETID_MidiBus(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMEDIUMSETID_VPBus(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMETHODSETID_Wavetable(pub u8);
pub type KSMETHOD_WAVETABLE = i32;
pub const KSMETHOD_WAVETABLE_WAVE_ALLOC: KSMETHOD_WAVETABLE = 0;
pub const KSMETHOD_WAVETABLE_WAVE_FIND: KSMETHOD_WAVETABLE = 2;
pub const KSMETHOD_WAVETABLE_WAVE_FREE: KSMETHOD_WAVETABLE = 1;
pub const KSMETHOD_WAVETABLE_WAVE_WRITE: KSMETHOD_WAVETABLE = 3;
pub type KSMICARRAY_MICARRAYTYPE = i32;
pub const KSMICARRAY_MICARRAYTYPE_3D: KSMICARRAY_MICARRAYTYPE = 2;
pub const KSMICARRAY_MICARRAYTYPE_LINEAR: KSMICARRAY_MICARRAYTYPE = 0;
pub const KSMICARRAY_MICARRAYTYPE_PLANAR: KSMICARRAY_MICARRAYTYPE = 1;
pub type KSMICARRAY_MICTYPE = i32;
pub const KSMICARRAY_MICTYPE_8SHAPED: KSMICARRAY_MICTYPE = 5;
pub const KSMICARRAY_MICTYPE_CARDIOID: KSMICARRAY_MICTYPE = 2;
pub const KSMICARRAY_MICTYPE_HYPERCARDIOID: KSMICARRAY_MICTYPE = 4;
pub const KSMICARRAY_MICTYPE_OMNIDIRECTIONAL: KSMICARRAY_MICTYPE = 0;
pub const KSMICARRAY_MICTYPE_SUBCARDIOID: KSMICARRAY_MICTYPE = 1;
pub const KSMICARRAY_MICTYPE_SUPERCARDIOID: KSMICARRAY_MICTYPE = 3;
pub const KSMICARRAY_MICTYPE_VENDORDEFINED: KSMICARRAY_MICTYPE = 15;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSMIDILOOPED_BUFFER {
    pub BufferAddress: *mut core::ffi::c_void,
    pub ActualBufferSize: u32,
}
impl Default for KSMIDILOOPED_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSMIDILOOPED_BUFFER_PROPERTY {
    pub Property: super::ks::KSPROPERTY,
    pub RequestedBufferSize: u32,
}
#[cfg(feature = "ks")]
impl Default for KSMIDILOOPED_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSMIDILOOPED_EVENT {
    pub WriteEvent: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for KSMIDILOOPED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSMIDILOOPED_EVENT2 {
    pub WriteEvent: super::winnt::HANDLE,
    pub ReadEvent: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for KSMIDILOOPED_EVENT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSMIDILOOPED_REGISTERS {
    pub WritePosition: *mut core::ffi::c_void,
    pub ReadPosition: *mut core::ffi::c_void,
}
impl Default for KSMIDILOOPED_REGISTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSMPEGVIDMODE_LTRBOX: u32 = 2;
pub const KSMPEGVIDMODE_PANSCAN: u32 = 1;
pub const KSMPEGVIDMODE_SCALE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMPEGVID_RECT {
    pub StartX: u32,
    pub StartY: u32,
    pub EndX: u32,
    pub EndY: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSMULTIPLE_DATA_PROP {
    pub Property: super::ks::KSPROPERTY,
    pub MultipleItem: super::ks::KSMULTIPLE_ITEM,
}
#[cfg(feature = "ks")]
impl Default for KSMULTIPLE_DATA_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMUSICFORMAT {
    pub TimeDeltaMs: u32,
    pub ByteCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMUSIC_TECHNOLOGY_FMSYNTH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMUSIC_TECHNOLOGY_PORT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMUSIC_TECHNOLOGY_SQSYNTH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMUSIC_TECHNOLOGY_SWSYNTH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSMUSIC_TECHNOLOGY_WAVETABLE(pub u8);
pub const KSNODEPIN_DEMUX_IN: u32 = 0;
pub const KSNODEPIN_DEMUX_OUT: u32 = 1;
pub const KSNODEPIN_STANDARD_IN: u32 = 1;
pub const KSNODEPIN_STANDARD_OUT: u32 = 0;
pub const KSNODEPIN_SUM_MUX_IN: u32 = 1;
pub const KSNODEPIN_SUM_MUX_OUT: u32 = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY {
    pub Property: super::ks::KSPROPERTY,
    pub NodeId: u32,
    pub Reserved: u32,
}
#[cfg(feature = "ks")]
impl Default for KSNODEPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER {
    pub NodeProperty: KSNODEPROPERTY,
    pub ListenerId: *mut core::ffi::c_void,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "ks")]
impl Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER {
    pub NodeProperty: KSNODEPROPERTY,
    pub ListenerId: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "ks")]
impl Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_CHANNEL {
    pub NodeProperty: KSNODEPROPERTY,
    pub Channel: i32,
    pub Reserved: u32,
}
#[cfg(feature = "ks")]
impl Default for KSNODEPROPERTY_AUDIO_CHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    pub NodeProperty: KSNODEPROPERTY,
    pub DevSpecificId: u32,
    pub DeviceInfo: u32,
    pub Length: u32,
}
#[cfg(feature = "ks")]
impl Default for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY {
    pub NodeProperty: KSNODEPROPERTY,
    pub AppContext: *mut core::ffi::c_void,
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "ks")]
impl Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY {
    pub NodeProperty: KSNODEPROPERTY,
    pub AppContext: *mut core::ffi::c_void,
    pub Length: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "ks")]
impl Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_1394_DA_STREAM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_1394_DV_STREAM_SOUNDTRACK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_3D_EFFECTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_ADC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_AGC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_ANALOG_CONNECTOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_ANALOG_TAPE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_AUDIO_ENGINE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_AUDIO_KEYWORDDETECTOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_AUDIO_LOOPBACK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_AUDIO_MODULE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_BIDIRECTIONAL_UNDEFINED(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_CABLE_TUNER_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_CD_PLAYER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_CHORUS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_COMMUNICATION_SPEAKER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DAC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DAT_IO_DIGITAL_AUDIO_TAPE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DCC_IO_DIGITAL_COMPACT_CASSETTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DELAY(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DEMUX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DESKTOP_MICROPHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DESKTOP_SPEAKER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DEV_SPECIFIC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DIGITAL_AUDIO_INTERFACE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DISPLAYPORT_INTERFACE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DOWN_LINE_PHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DRM_DESCRAMBLE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DSS_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DVD_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_DYN_RANGE_COMPRESSOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_ECHO_CANCELING_SPEAKERPHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_ECHO_SUPPRESSING_SPEAKERPHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_EMBEDDED_UNDEFINED(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_EQUALIZATION_NOISE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_EQUALIZER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_EXTERNAL_UNDEFINED(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_FM_RX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_HANDSET(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_HDMI_INTERFACE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_HEADPHONES(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_HEADSET(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_HEAD_MOUNTED_DISPLAY_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_INPUT_UNDEFINED(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_LEGACY_AUDIO_CONNECTOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_LEVEL_CALIBRATION_NOISE_SOURCE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_LINE_CONNECTOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_LOUDNESS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_LOW_FREQUENCY_EFFECTS_SPEAKER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_MICROPHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_MICROPHONE_ARRAY(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_MIDI_ELEMENT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_MIDI_JACK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_MINIDISK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_MULTITRACK_RECORDER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_MUTE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_MUX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_NOISE_SUPPRESS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_OMNI_DIRECTIONAL_MICROPHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_OUTPUT_UNDEFINED(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_PARAMETRIC_EQUALIZER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_PEAKMETER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_PERSONAL_MICROPHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_PHONE_LINE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_PHONOGRAPH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_PROCESSING_MICROPHONE_ARRAY(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_PROLOGIC_DECODER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_PROLOGIC_ENCODER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_RADIO_RECEIVER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_RADIO_TRANSMITTER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_REVERB(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_ROOM_SPEAKER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SATELLITE_RECEIVER_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SPDIF_INTERFACE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SPEAKER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SPEAKERPHONE_NO_ECHO_REDUCTION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SPEAKERS_STATIC_JACK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SRC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_STEREO_WIDE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SUPERMIX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_SYNTHESIZER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_TELEPHONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_TELEPHONY_BIDI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_TELEPHONY_UNDEFINED(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_TONE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_TV_TUNER_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_UPDOWN_MIX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VCR_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_CAMERA_TERMINAL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_DISC_AUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_INPUT_MTT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_INPUT_TERMINAL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_OUTPUT_MTT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_OUTPUT_TERMINAL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_PROCESSING(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_SELECTOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VIDEO_STREAMING(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNODETYPE_VOLUME(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNOTIFICATIONID_AudioModule(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSNOTIFICATIONID_SoundDetector(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTYSETID_ExtendedCameraControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTYSETID_NetworkCameraControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTYSETID_PerFrameSettingControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTYSETID_WindowsCameraEffect(pub u8);
pub type KSPROPERTY_AC3 = i32;
pub const KSPROPERTY_AC3_ALTERNATE_AUDIO: KSPROPERTY_AC3 = 2;
pub const KSPROPERTY_AC3_BIT_STREAM_MODE: KSPROPERTY_AC3 = 4;
pub const KSPROPERTY_AC3_DIALOGUE_LEVEL: KSPROPERTY_AC3 = 5;
pub const KSPROPERTY_AC3_DOWNMIX: KSPROPERTY_AC3 = 3;
pub const KSPROPERTY_AC3_ERROR_CONCEALMENT: KSPROPERTY_AC3 = 1;
pub const KSPROPERTY_AC3_LANGUAGE_CODE: KSPROPERTY_AC3 = 6;
pub const KSPROPERTY_AC3_ROOM_TYPE: KSPROPERTY_AC3 = 7;
pub type KSPROPERTY_ALLOCATOR_CONTROL = i32;
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS: KSPROPERTY_ALLOCATOR_CONTROL = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    pub InterleavedCapSupported: u32,
}
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE: KSPROPERTY_ALLOCATOR_CONTROL = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    pub InterleavedCapPossible: u32,
}
pub const KSPROPERTY_ALLOCATOR_CONTROL_HONOR_COUNT: KSPROPERTY_ALLOCATOR_CONTROL = 0;
pub const KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE: KSPROPERTY_ALLOCATOR_CONTROL = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    pub CX: u32,
    pub CY: u32,
}
pub const KSPROPERTY_ATN_READER: KSPROPERTY_TIMECODE = 1;
pub type KSPROPERTY_AUDDECOUT = i32;
pub const KSPROPERTY_AUDDECOUT_CUR_MODE: KSPROPERTY_AUDDECOUT = 1;
pub const KSPROPERTY_AUDDECOUT_MODES: KSPROPERTY_AUDDECOUT = 0;
pub type KSPROPERTY_AUDIO = i32;
pub type KSPROPERTY_AUDIOENGINE = i32;
pub const KSPROPERTY_AUDIOENGINE_BUFFER_SIZE_RANGE: KSPROPERTY_AUDIOENGINE = 7;
pub const KSPROPERTY_AUDIOENGINE_DESCRIPTOR: KSPROPERTY_AUDIOENGINE = 6;
pub const KSPROPERTY_AUDIOENGINE_DEVICECONTROLS: KSPROPERTY_AUDIOENGINE = 10;
pub const KSPROPERTY_AUDIOENGINE_DEVICEFORMAT: KSPROPERTY_AUDIOENGINE = 4;
pub const KSPROPERTY_AUDIOENGINE_GFXENABLE: KSPROPERTY_AUDIOENGINE = 1;
pub const KSPROPERTY_AUDIOENGINE_LFXENABLE: KSPROPERTY_AUDIOENGINE = 0;
pub const KSPROPERTY_AUDIOENGINE_LOOPBACK_PROTECTION: KSPROPERTY_AUDIOENGINE = 8;
pub const KSPROPERTY_AUDIOENGINE_MIXFORMAT: KSPROPERTY_AUDIOENGINE = 2;
pub const KSPROPERTY_AUDIOENGINE_SUPPORTEDDEVICEFORMATS: KSPROPERTY_AUDIOENGINE = 5;
pub const KSPROPERTY_AUDIOENGINE_VOLUMELEVEL: KSPROPERTY_AUDIOENGINE = 9;
pub type KSPROPERTY_AUDIOLOOPBACK = i32;
pub const KSPROPERTY_AUDIOLOOPBACK_TAPPOINT_CAPS: KSPROPERTY_AUDIOLOOPBACK = 0;
pub type KSPROPERTY_AUDIOMODULE = i32;
pub const KSPROPERTY_AUDIOMODULE_COMMAND: KSPROPERTY_AUDIOMODULE = 2;
pub const KSPROPERTY_AUDIOMODULE_DESCRIPTORS: KSPROPERTY_AUDIOMODULE = 1;
pub const KSPROPERTY_AUDIOMODULE_NOTIFICATION_DEVICE_ID: KSPROPERTY_AUDIOMODULE = 3;
pub type KSPROPERTY_AUDIOPOSTURE = i32;
pub const KSPROPERTY_AUDIOPOSTURE_ORIENTATION: KSPROPERTY_AUDIOPOSTURE = 1;
pub type KSPROPERTY_AUDIORESOURCEMANAGEMENT = i32;
pub const KSPROPERTY_AUDIORESOURCEMANAGEMENT_RESOURCEGROUP: KSPROPERTY_AUDIORESOURCEMANAGEMENT = 0;
pub type KSPROPERTY_AUDIOSIGNALPROCESSING = i32;
pub const KSPROPERTY_AUDIOSIGNALPROCESSING_MODES: KSPROPERTY_AUDIOSIGNALPROCESSING = 0;
pub const KSPROPERTY_AUDIO_3D_INTERFACE: KSPROPERTY_AUDIO = 36;
pub const KSPROPERTY_AUDIO_AGC: KSPROPERTY_AUDIO = 21;
pub const KSPROPERTY_AUDIO_ALGORITHM_INSTANCE: KSPROPERTY_AUDIO = 38;
pub const KSPROPERTY_AUDIO_BASS: KSPROPERTY_AUDIO = 14;
pub const KSPROPERTY_AUDIO_BASS_BOOST: KSPROPERTY_AUDIO = 17;
pub const KSPROPERTY_AUDIO_BUFFER_DURATION: u32 = 1;
pub const KSPROPERTY_AUDIO_CHANNEL_CONFIG: KSPROPERTY_AUDIO = 3;
pub const KSPROPERTY_AUDIO_CHORUS_LEVEL: KSPROPERTY_AUDIO = 27;
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_DEPTH: KSPROPERTY_AUDIO = 47;
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_RATE: KSPROPERTY_AUDIO = 46;
pub const KSPROPERTY_AUDIO_COPY_PROTECTION: KSPROPERTY_AUDIO = 2;
pub const KSPROPERTY_AUDIO_CPU_RESOURCES: KSPROPERTY_AUDIO = 33;
pub const KSPROPERTY_AUDIO_DELAY: KSPROPERTY_AUDIO = 22;
pub const KSPROPERTY_AUDIO_DEMUX_DEST: KSPROPERTY_AUDIO = 29;
pub const KSPROPERTY_AUDIO_DEV_SPECIFIC: KSPROPERTY_AUDIO = 28;
pub const KSPROPERTY_AUDIO_DYNAMIC_RANGE: KSPROPERTY_AUDIO = 6;
pub const KSPROPERTY_AUDIO_DYNAMIC_SAMPLING_RATE: KSPROPERTY_AUDIO = 9;
pub const KSPROPERTY_AUDIO_EQ_BANDS: KSPROPERTY_AUDIO = 20;
pub const KSPROPERTY_AUDIO_EQ_LEVEL: KSPROPERTY_AUDIO = 18;
pub const KSPROPERTY_AUDIO_FILTER_STATE: KSPROPERTY_AUDIO = 39;
pub const KSPROPERTY_AUDIO_LATENCY: KSPROPERTY_AUDIO = 1;
pub const KSPROPERTY_AUDIO_LINEAR_BUFFER_POSITION: KSPROPERTY_AUDIO = 54;
pub const KSPROPERTY_AUDIO_LOUDNESS: KSPROPERTY_AUDIO = 23;
pub const KSPROPERTY_AUDIO_MANUFACTURE_GUID: KSPROPERTY_AUDIO = 31;
pub const KSPROPERTY_AUDIO_MIC_ARRAY_GEOMETRY: KSPROPERTY_AUDIO = 51;
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY: KSPROPERTY_AUDIO = 58;
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY2: KSPROPERTY_AUDIO = 60;
pub const KSPROPERTY_AUDIO_MIC_SNR: KSPROPERTY_AUDIO = 59;
pub const KSPROPERTY_AUDIO_MID: KSPROPERTY_AUDIO = 15;
pub const KSPROPERTY_AUDIO_MIX_LEVEL_CAPS: KSPROPERTY_AUDIO = 11;
pub const KSPROPERTY_AUDIO_MIX_LEVEL_TABLE: KSPROPERTY_AUDIO = 10;
pub const KSPROPERTY_AUDIO_MUTE: KSPROPERTY_AUDIO = 13;
pub const KSPROPERTY_AUDIO_MUX_SOURCE: KSPROPERTY_AUDIO = 12;
pub const KSPROPERTY_AUDIO_NUM_EQ_BANDS: KSPROPERTY_AUDIO = 19;
pub const KSPROPERTY_AUDIO_PEAKMETER: KSPROPERTY_AUDIO = 37;
pub const KSPROPERTY_AUDIO_PEAKMETER2: KSPROPERTY_AUDIO = 55;
pub const KSPROPERTY_AUDIO_PEQ_BAND_CENTER_FREQ: KSPROPERTY_AUDIO = 43;
pub const KSPROPERTY_AUDIO_PEQ_BAND_LEVEL: KSPROPERTY_AUDIO = 45;
pub const KSPROPERTY_AUDIO_PEQ_BAND_Q_FACTOR: KSPROPERTY_AUDIO = 44;
pub const KSPROPERTY_AUDIO_PEQ_MAX_BANDS: KSPROPERTY_AUDIO = 41;
pub const KSPROPERTY_AUDIO_PEQ_NUM_BANDS: KSPROPERTY_AUDIO = 42;
pub const KSPROPERTY_AUDIO_POSITION: KSPROPERTY_AUDIO = 5;
pub const KSPROPERTY_AUDIO_POSITIONEX: KSPROPERTY_AUDIO = 50;
pub const KSPROPERTY_AUDIO_PREFERRED_STATUS: KSPROPERTY_AUDIO = 40;
pub const KSPROPERTY_AUDIO_PRESENTATION_POSITION: KSPROPERTY_AUDIO = 52;
pub const KSPROPERTY_AUDIO_PRODUCT_GUID: KSPROPERTY_AUDIO = 32;
pub const KSPROPERTY_AUDIO_QUALITY: KSPROPERTY_AUDIO = 7;
pub const KSPROPERTY_AUDIO_REVERB_DELAY_FEEDBACK: KSPROPERTY_AUDIO = 49;
pub const KSPROPERTY_AUDIO_REVERB_LEVEL: KSPROPERTY_AUDIO = 26;
pub const KSPROPERTY_AUDIO_REVERB_TIME: KSPROPERTY_AUDIO = 48;
pub const KSPROPERTY_AUDIO_SAMPLING_RATE: KSPROPERTY_AUDIO = 8;
pub const KSPROPERTY_AUDIO_STEREO_ENHANCE: KSPROPERTY_AUDIO = 30;
pub const KSPROPERTY_AUDIO_STEREO_SPEAKER_GEOMETRY: KSPROPERTY_AUDIO = 34;
pub const KSPROPERTY_AUDIO_SURROUND_ENCODE: KSPROPERTY_AUDIO = 35;
pub const KSPROPERTY_AUDIO_TREBLE: KSPROPERTY_AUDIO = 16;
pub const KSPROPERTY_AUDIO_VOLUMELEVEL: KSPROPERTY_AUDIO = 4;
pub const KSPROPERTY_AUDIO_VOLUMELIMIT_ENGAGED: KSPROPERTY_AUDIO = 57;
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_LASTBUFFER_POSITION: KSPROPERTY_AUDIO = 56;
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_POSITION: KSPROPERTY_AUDIO = 53;
pub const KSPROPERTY_AUDIO_WIDENESS: KSPROPERTY_AUDIO = 25;
pub const KSPROPERTY_AUDIO_WIDE_MODE: KSPROPERTY_AUDIO = 24;
pub type KSPROPERTY_BIBLIOGRAPHIC = i32;
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYGEOGRAPHIC: KSPROPERTY_BIBLIOGRAPHIC = 825570848;
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = 808465952;
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYRELATED: KSPROPERTY_BIBLIOGRAPHIC = 808728352;
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808727584;
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTOPICALTERM: KSPROPERTY_BIBLIOGRAPHIC = 808793632;
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808662816;
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDFORMAVAILABLE: KSPROPERTY_BIBLIOGRAPHIC = 808662304;
pub const KSPROPERTY_BIBLIOGRAPHIC_AWARDS: KSPROPERTY_BIBLIOGRAPHIC = 909653280;
pub const KSPROPERTY_BIBLIOGRAPHIC_BIBLIOGRAPHYNOTE: KSPROPERTY_BIBLIOGRAPHIC = 875574560;
pub const KSPROPERTY_BIBLIOGRAPHIC_CATALOGINGSOURCE: KSPROPERTY_BIBLIOGRAPHIC = 808726560;
pub const KSPROPERTY_BIBLIOGRAPHIC_CITATION: KSPROPERTY_BIBLIOGRAPHIC = 808531232;
pub const KSPROPERTY_BIBLIOGRAPHIC_CONTENTSNOTE: KSPROPERTY_BIBLIOGRAPHIC = 892351776;
pub const KSPROPERTY_BIBLIOGRAPHIC_CREATIONCREDIT: KSPROPERTY_BIBLIOGRAPHIC = 942683424;
pub const KSPROPERTY_BIBLIOGRAPHIC_GENERALNOTE: KSPROPERTY_BIBLIOGRAPHIC = 808465696;
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMCURRICULUM: KSPROPERTY_BIBLIOGRAPHIC = 943011360;
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMGENRE: KSPROPERTY_BIBLIOGRAPHIC = 892679712;
pub const KSPROPERTY_BIBLIOGRAPHIC_ISBN: KSPROPERTY_BIBLIOGRAPHIC = 808595488;
pub const KSPROPERTY_BIBLIOGRAPHIC_ISSN: KSPROPERTY_BIBLIOGRAPHIC = 842149920;
pub const KSPROPERTY_BIBLIOGRAPHIC_LCCN: KSPROPERTY_BIBLIOGRAPHIC = 808529952;
pub const KSPROPERTY_BIBLIOGRAPHIC_LEADER: KSPROPERTY_BIBLIOGRAPHIC = 1380207648;
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINCORPORATEBODY: KSPROPERTY_BIBLIOGRAPHIC = 808530208;
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINMEETINGNAME: KSPROPERTY_BIBLIOGRAPHIC = 825307424;
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = 808464672;
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808661280;
pub const KSPROPERTY_BIBLIOGRAPHIC_PARTICIPANT: KSPROPERTY_BIBLIOGRAPHIC = 825308448;
pub const KSPROPERTY_BIBLIOGRAPHIC_PHYSICALDESCRIPTION: KSPROPERTY_BIBLIOGRAPHIC = 808465184;
pub const KSPROPERTY_BIBLIOGRAPHIC_PUBLICATION: KSPROPERTY_BIBLIOGRAPHIC = 808858144;
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = 809055264;
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = 808466464;
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808663072;
pub const KSPROPERTY_BIBLIOGRAPHIC_SUMMARY: KSPROPERTY_BIBLIOGRAPHIC = 808596768;
pub const KSPROPERTY_BIBLIOGRAPHIC_SYSTEMDETAILS: KSPROPERTY_BIBLIOGRAPHIC = 942880032;
pub const KSPROPERTY_BIBLIOGRAPHIC_TARGETAUDIENCE: KSPROPERTY_BIBLIOGRAPHIC = 825373984;
pub const KSPROPERTY_BIBLIOGRAPHIC_TITLESTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = 892613152;
pub const KSPROPERTY_BIBLIOGRAPHIC_UNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 808727072;
pub const KSPROPERTY_BIBLIOGRAPHIC_VARYINGFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = 909390368;
pub type KSPROPERTY_BTAUDIO = i32;
pub const KSPROPERTY_CAMERACONTROL_AUTO_EXPOSURE_PRIORITY: KSPROPERTY_VIDCAP_CAMERACONTROL = 19;
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE: KSPROPERTY_VIDCAP_CAMERACONTROL = 4;
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 14;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ADVANCEDPHOTO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 33;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_BACKGROUNDSEGMENTATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 41;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_CAMERAANGLEOFFSET: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 17;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 43;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 42;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 47;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END2: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 47;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EVCOMPENSATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 16;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EXPOSUREMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 12;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EYEGAZECORRECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 40;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEAUTH_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 35;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEDETECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 29;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FIELDOFVIEW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 15;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FIELDOFVIEW2: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 46;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FIELDOFVIEW2_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 45;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FLASHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 9;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 13;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSPRIORITY: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 19;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSSTATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 20;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FRAMERATE_THROTTLE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 44;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_HISTOGRAM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 31;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_IRTORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 38;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 14;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO_ADVANCED: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 26;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MAXVIDFPS_PHOTORES: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 5;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MCC: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 25;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_METADATA: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 18;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OIS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 32;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OPTIMIZATIONHINT: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 10;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOCONFIRMATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 23;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 1;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMAXFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 2;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 0;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTHUMBNAIL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 6;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTRIGGERTIME: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 3;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PROFILE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 34;
pub type KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = i32;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_RELATIVEPANELOPTIMIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 39;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 21;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_ISPCONTROL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 22;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SCENEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 7;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SECURE_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 36;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_TORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 8;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VFR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 28;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOHDR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 30;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOSTABILIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 27;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOTEMPORALDENOISING: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 37;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WARMSTART: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 4;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WHITEBALANCEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 11;
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ZOOM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = 24;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ABSOLUTE: u32 = 0;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ASYNCHRONOUS: u32 = 4;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_AUTO: u32 = 1;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_MANUAL: u32 = 2;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_RELATIVE: u32 = 16;
pub type KSPROPERTY_CAMERACONTROL_FLASH = i32;
pub const KSPROPERTY_CAMERACONTROL_FLASH_AUTO: u32 = 2;
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_AUTO: u32 = 1;
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_MANUAL: u32 = 2;
pub const KSPROPERTY_CAMERACONTROL_FLASH_OFF: u32 = 0;
pub const KSPROPERTY_CAMERACONTROL_FLASH_ON: u32 = 1;
pub const KSPROPERTY_CAMERACONTROL_FLASH_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_FLASH = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_CAMERACONTROL_FLASH_S {
    pub Flash: u32,
    pub Capabilities: u32,
}
pub const KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH: KSPROPERTY_VIDCAP_CAMERACONTROL = 18;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    pub Property: super::ks::KSPROPERTY,
    pub lOcularFocalLength: i32,
    pub lObjectiveFocalLengthMin: i32,
    pub lObjectiveFocalLengthMax: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_FOCUS: KSPROPERTY_VIDCAP_CAMERACONTROL = 6;
pub const KSPROPERTY_CAMERACONTROL_FOCUS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 16;
pub type KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY = i32;
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_EXCLUSIVE_WITH_RECORD: u32 = 1;
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    pub Capabilities: u32,
    pub Reserved0: u32,
}
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_SEQUENCE_EXCLUSIVE_WITH_RECORD: u32 = 2;
pub const KSPROPERTY_CAMERACONTROL_IRIS: KSPROPERTY_VIDCAP_CAMERACONTROL = 5;
pub const KSPROPERTY_CAMERACONTROL_IRIS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 15;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    pub NodeProperty: KSNODEPROPERTY,
    pub lOcularFocalLength: i32,
    pub lObjectiveFocalLengthMin: i32,
    pub lObjectiveFocalLengthMax: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "ks")]
pub type KSPROPERTY_CAMERACONTROL_NODE_S = PKSPROPERTY_CAMERACONTROL_NODE_S;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_NODE_S2 {
    pub NodeProperty: super::ks::KSP_NODE,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_PAN: KSPROPERTY_VIDCAP_CAMERACONTROL = 0;
pub const KSPROPERTY_CAMERACONTROL_PANTILT: KSPROPERTY_VIDCAP_CAMERACONTROL = 9;
pub const KSPROPERTY_CAMERACONTROL_PANTILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 17;
pub const KSPROPERTY_CAMERACONTROL_PAN_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 10;
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CAPABILITY: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = 0;
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CLEAR: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = 2;
pub type KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = i32;
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_SET: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = 1;
pub const KSPROPERTY_CAMERACONTROL_PRIVACY: KSPROPERTY_VIDCAP_CAMERACONTROL = 8;
pub type KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST = i32;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_EXPOSURE: u32 = 512;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_FOCUS: u32 = 256;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_WB: u32 = 1024;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONVERGEMODE: u32 = 1073741824;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_ASYNC: u32 = 2147483648;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_AUTO: u32 = 1;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_MANUAL: u32 = 2;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    pub FocusRect: super::windef::RECT,
    pub AutoFocusLock: windows_sys::core::BOOL,
    pub AutoExposureLock: windows_sys::core::BOOL,
    pub AutoWhitebalanceLock: windows_sys::core::BOOL,
    pub Anonymous: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0,
}
#[cfg(feature = "windef")]
impl Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    pub Capabilities: u32,
    pub Configuration: u32,
}
#[cfg(feature = "windef")]
impl Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_ROLL: KSPROPERTY_VIDCAP_CAMERACONTROL = 2;
pub const KSPROPERTY_CAMERACONTROL_ROLL_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 12;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_S {
    pub Property: super::ks::KSPROPERTY,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CAMERACONTROL_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_S2 {
    pub Property: super::ks::KSPROPERTY,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CAMERACONTROL_S2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_SCANMODE: KSPROPERTY_VIDCAP_CAMERACONTROL = 7;
#[repr(C)]
#[cfg(all(feature = "ks", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_S_EX {
    pub Property: super::ks::KSPROPERTY,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub FocusRect: super::windef::RECT,
}
#[cfg(all(feature = "ks", feature = "windef"))]
impl Default for KSPROPERTY_CAMERACONTROL_S_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_TILT: KSPROPERTY_VIDCAP_CAMERACONTROL = 1;
pub const KSPROPERTY_CAMERACONTROL_TILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 11;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_AUTO: u32 = 4;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_AUTO: u32 = 1;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_MANUAL: u32 = 2;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_HIGH: u32 = 1;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_LOW: u32 = 3;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_MEDIUM: u32 = 2;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_OFF: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    pub VideoStabilizationMode: u32,
    pub Capabilities: u32,
}
pub type KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE = i32;
pub const KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE = 0;
pub const KSPROPERTY_CAMERACONTROL_ZOOM: KSPROPERTY_VIDCAP_CAMERACONTROL = 3;
pub const KSPROPERTY_CAMERACONTROL_ZOOM_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 13;
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_CLEAR: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = 0;
pub type KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = i32;
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_SET: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = 1;
pub type KSPROPERTY_COMPOSIT_ON = windows_sys::core::BOOL;
pub type KSPROPERTY_COPYPROT = i32;
pub const KSPROPERTY_COPY_MACROVISION: KSPROPERTY_COPYPROT = 5;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_ACTIVE_S {
    pub Property: super::ks::KSPROPERTY,
    pub IndexInputPin: u32,
    pub Active: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CROSSBAR_ACTIVE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CROSSBAR_CAN_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = 2;
pub const KSPROPERTY_CROSSBAR_CAPS: KSPROPERTY_VIDCAP_CROSSBAR = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_CAPS_S {
    pub Property: super::ks::KSPROPERTY,
    pub NumberOfInputs: u32,
    pub NumberOfOutputs: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CROSSBAR_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CROSSBAR_INPUT_ACTIVE: KSPROPERTY_VIDCAP_CROSSBAR = 4;
pub const KSPROPERTY_CROSSBAR_PININFO: KSPROPERTY_VIDCAP_CROSSBAR = 1;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_PININFO_S {
    pub Property: super::ks::KSPROPERTY,
    pub Direction: super::ks::KSPIN_DATAFLOW,
    pub Index: u32,
    pub PinType: u32,
    pub RelatedPinIndex: u32,
    pub Medium: super::ks::KSPIN_MEDIUM,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CROSSBAR_PININFO_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CROSSBAR_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = 3;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_ROUTE_S {
    pub Property: super::ks::KSPROPERTY,
    pub IndexInputPin: u32,
    pub IndexOutputPin: u32,
    pub CanRoute: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_CROSSBAR_ROUTE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CURRENT_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = 3;
pub type KSPROPERTY_CYCLIC = i32;
pub const KSPROPERTY_CYCLIC_POSITION: KSPROPERTY_CYCLIC = 0;
pub type KSPROPERTY_DIRECTSOUND3DBUFFER = i32;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_ALL: KSPROPERTY_DIRECTSOUND3DBUFFER = 0;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEANGLES: KSPROPERTY_DIRECTSOUND3DBUFFER = 3;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEORIENTATION: KSPROPERTY_DIRECTSOUND3DBUFFER = 4;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEOUTSIDEVOLUME: KSPROPERTY_DIRECTSOUND3DBUFFER = 5;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MAXDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = 7;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MINDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = 6;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MODE: KSPROPERTY_DIRECTSOUND3DBUFFER = 8;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_POSITION: KSPROPERTY_DIRECTSOUND3DBUFFER = 1;
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_VELOCITY: KSPROPERTY_DIRECTSOUND3DBUFFER = 2;
pub type KSPROPERTY_DIRECTSOUND3DLISTENER = i32;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALL: KSPROPERTY_DIRECTSOUND3DLISTENER = 0;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALLOCATION: KSPROPERTY_DIRECTSOUND3DLISTENER = 8;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_BATCH: KSPROPERTY_DIRECTSOUND3DLISTENER = 7;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DISTANCEFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = 4;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DOPPLERFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = 6;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ORIENTATION: KSPROPERTY_DIRECTSOUND3DLISTENER = 3;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_POSITION: KSPROPERTY_DIRECTSOUND3DLISTENER = 1;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ROLLOFFFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = 5;
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_VELOCITY: KSPROPERTY_DIRECTSOUND3DLISTENER = 2;
pub const KSPROPERTY_DISPLAY_ADAPTER_GUID: KSPROPERTY_VIDMEM_TRANSPORT = 1;
pub type KSPROPERTY_DRMAUDIOSTREAM = i32;
pub const KSPROPERTY_DRMAUDIOSTREAM_CONTENTID: KSPROPERTY_DRMAUDIOSTREAM = 0;
pub const KSPROPERTY_DROPPEDFRAMES_CURRENT: KSPROPERTY_VIDCAP_DROPPEDFRAMES = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    pub Property: super::ks::KSPROPERTY,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub AverageFrameSize: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_DVDCOPY_CHLG_KEY: KSPROPERTY_COPYPROT = 1;
pub const KSPROPERTY_DVDCOPY_DEC_KEY2: KSPROPERTY_COPYPROT = 3;
pub const KSPROPERTY_DVDCOPY_DISC_KEY: KSPROPERTY_COPYPROT = 128;
pub const KSPROPERTY_DVDCOPY_DVD_KEY1: KSPROPERTY_COPYPROT = 2;
pub const KSPROPERTY_DVDCOPY_REGION: KSPROPERTY_COPYPROT = 6;
pub const KSPROPERTY_DVDCOPY_SET_COPY_STATE: KSPROPERTY_COPYPROT = 7;
pub const KSPROPERTY_DVDCOPY_TITLE_KEY: KSPROPERTY_COPYPROT = 4;
pub type KSPROPERTY_DVDSUBPIC = i32;
pub const KSPROPERTY_DVDSUBPIC_COMPOSIT_ON: KSPROPERTY_DVDSUBPIC = 2;
pub const KSPROPERTY_DVDSUBPIC_HLI: KSPROPERTY_DVDSUBPIC = 1;
pub const KSPROPERTY_DVDSUBPIC_PALETTE: KSPROPERTY_DVDSUBPIC = 0;
pub type KSPROPERTY_EXTDEVICE = i32;
pub const KSPROPERTY_EXTDEVICE_CAPABILITIES: KSPROPERTY_EXTDEVICE = 4;
pub const KSPROPERTY_EXTDEVICE_ID: KSPROPERTY_EXTDEVICE = 0;
pub const KSPROPERTY_EXTDEVICE_PORT: KSPROPERTY_EXTDEVICE = 3;
pub const KSPROPERTY_EXTDEVICE_POWER_STATE: KSPROPERTY_EXTDEVICE = 2;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_EXTDEVICE_S {
    pub Property: super::ks::KSPROPERTY,
    pub u: KSPROPERTY_EXTDEVICE_S_0,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_EXTDEVICE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub union KSPROPERTY_EXTDEVICE_S_0 {
    pub Capabilities: DEVCAPS,
    pub DevPort: u32,
    pub PowerState: u32,
    pub pawchString: [u16; 260],
    pub NodeUniqueID: [u32; 2],
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_EXTDEVICE_S_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_EXTDEVICE_VERSION: KSPROPERTY_EXTDEVICE = 1;
pub type KSPROPERTY_EXTENSION_UNIT = i32;
pub const KSPROPERTY_EXTENSION_UNIT_CONTROL: KSPROPERTY_EXTENSION_UNIT = 1;
pub const KSPROPERTY_EXTENSION_UNIT_INFO: KSPROPERTY_EXTENSION_UNIT = 0;
pub const KSPROPERTY_EXTENSION_UNIT_PASS_THROUGH: KSPROPERTY_EXTENSION_UNIT = 65535;
pub type KSPROPERTY_EXTXPORT = i32;
pub const KSPROPERTY_EXTXPORT_ATN_SEARCH: KSPROPERTY_EXTXPORT = 8;
pub const KSPROPERTY_EXTXPORT_CAPABILITIES: KSPROPERTY_EXTXPORT = 0;
pub const KSPROPERTY_EXTXPORT_INPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = 1;
pub const KSPROPERTY_EXTXPORT_LOAD_MEDIUM: KSPROPERTY_EXTXPORT = 3;
pub const KSPROPERTY_EXTXPORT_MEDIUM_INFO: KSPROPERTY_EXTXPORT = 4;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_EXTXPORT_NODE_S {
    pub NodeProperty: super::ks::KSP_NODE,
    pub u: KSPROPERTY_EXTXPORT_NODE_S_0,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_EXTXPORT_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub union KSPROPERTY_EXTXPORT_NODE_S_0 {
    pub Capabilities: u32,
    pub SignalMode: u32,
    pub LoadMedium: u32,
    pub MediumInfo: MEDIUM_INFO,
    pub XPrtState: TRANSPORT_STATE,
    pub Timecode: KSPROPERTY_EXTXPORT_NODE_S_0_0,
    pub dwTimecode: u32,
    pub dwAbsTrackNumber: u32,
    pub RawAVC: KSPROPERTY_EXTXPORT_NODE_S_0_1,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_EXTXPORT_NODE_S_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    pub PayloadSize: u32,
    pub Payload: [u8; 512],
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_EXTXPORT_OUTPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = 2;
pub const KSPROPERTY_EXTXPORT_RTC_SEARCH: KSPROPERTY_EXTXPORT = 9;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_EXTXPORT_S {
    pub Property: super::ks::KSPROPERTY,
    pub u: KSPROPERTY_EXTXPORT_S_0,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_EXTXPORT_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub union KSPROPERTY_EXTXPORT_S_0 {
    pub Capabilities: u32,
    pub SignalMode: u32,
    pub LoadMedium: u32,
    pub MediumInfo: MEDIUM_INFO,
    pub XPrtState: TRANSPORT_STATE,
    pub Timecode: KSPROPERTY_EXTXPORT_S_0_0,
    pub dwTimecode: u32,
    pub dwAbsTrackNumber: u32,
    pub RawAVC: KSPROPERTY_EXTXPORT_S_0_1,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_EXTXPORT_S_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_EXTXPORT_S_0_0 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_EXTXPORT_S_0_1 {
    pub PayloadSize: u32,
    pub Payload: [u8; 512],
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_EXTXPORT_S_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_EXTXPORT_STATE: KSPROPERTY_EXTXPORT = 5;
pub const KSPROPERTY_EXTXPORT_STATE_NOTIFY: KSPROPERTY_EXTXPORT = 6;
pub const KSPROPERTY_EXTXPORT_TIMECODE_SEARCH: KSPROPERTY_EXTXPORT = 7;
pub const KSPROPERTY_FMRX_ANTENNAENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = 2;
pub type KSPROPERTY_FMRX_CONTROL = i32;
pub const KSPROPERTY_FMRX_ENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = 0;
pub const KSPROPERTY_FMRX_STATE: KSPROPERTY_FMRX_CONTROL = 0;
pub type KSPROPERTY_FMRX_TOPOLOGY = i32;
pub const KSPROPERTY_FMRX_VOLUME: KSPROPERTY_FMRX_TOPOLOGY = 1;
pub type KSPROPERTY_HRTF3D = i32;
pub const KSPROPERTY_HRTF3D_FILTER_FORMAT: KSPROPERTY_HRTF3D = 2;
pub const KSPROPERTY_HRTF3D_INITIALIZE: KSPROPERTY_HRTF3D = 1;
pub const KSPROPERTY_HRTF3D_PARAMS: KSPROPERTY_HRTF3D = 0;
pub type KSPROPERTY_INTERLEAVEDAUDIO = i32;
pub const KSPROPERTY_INTERLEAVEDAUDIO_FORMATINFORMATION: KSPROPERTY_INTERLEAVEDAUDIO = 1;
pub type KSPROPERTY_ITD3D = i32;
pub const KSPROPERTY_ITD3D_PARAMS: KSPROPERTY_ITD3D = 0;
pub type KSPROPERTY_JACK = i32;
pub const KSPROPERTY_JACK_CONTAINERID: KSPROPERTY_JACK = 4;
pub const KSPROPERTY_JACK_DESCRIPTION: KSPROPERTY_JACK = 1;
pub const KSPROPERTY_JACK_DESCRIPTION2: KSPROPERTY_JACK = 2;
pub const KSPROPERTY_JACK_DESCRIPTION3: KSPROPERTY_JACK = 5;
pub const KSPROPERTY_JACK_SINK_INFO: KSPROPERTY_JACK = 3;
pub const KSPROPERTY_MAP_CAPTURE_HANDLE_TO_VRAM_ADDRESS: KSPROPERTY_VIDMEM_TRANSPORT = 4;
pub type KSPROPERTY_MIDILOOPEDSTREAMING = i32;
pub const KSPROPERTY_MIDILOOPEDSTREAMING_BUFFER: KSPROPERTY_MIDILOOPEDSTREAMING = 0;
pub const KSPROPERTY_MIDILOOPEDSTREAMING_NOTIFICATION_EVENT: KSPROPERTY_MIDILOOPEDSTREAMING = 2;
pub const KSPROPERTY_MIDILOOPEDSTREAMING_REGISTERS: KSPROPERTY_MIDILOOPEDSTREAMING = 1;
pub type KSPROPERTY_MPEG2VID = i32;
pub const KSPROPERTY_MPEG2VID_16_9_PANSCAN: KSPROPERTY_MPEG2VID = 4;
pub const KSPROPERTY_MPEG2VID_16_9_RECT: KSPROPERTY_MPEG2VID = 3;
pub const KSPROPERTY_MPEG2VID_4_3_RECT: KSPROPERTY_MPEG2VID = 2;
pub const KSPROPERTY_MPEG2VID_CUR_MODE: KSPROPERTY_MPEG2VID = 1;
pub const KSPROPERTY_MPEG2VID_MODES: KSPROPERTY_MPEG2VID = 0;
pub type KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES = i32;
pub const KSPROPERTY_MPEG4_MEDIATYPE_SD_BOX: KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES = 1;
pub const KSPROPERTY_NETWORKCAMERACONTROL_EVENTTOPICS_XML: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub EventFilter: [u16; 1],
}
impl Default for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    pub MetadataItems: u32,
    pub Size: u32,
    pub PTZStatus: windows_sys::core::BOOL,
    pub Events: windows_sys::core::BOOL,
    pub Analytics: windows_sys::core::BOOL,
    pub Reserved: windows_sys::core::BOOL,
}
pub type KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = i32;
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE_EVENTSINFO: KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = 0;
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTP: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    pub Size: u32,
    pub Type: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE,
}
pub type KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = i32;
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_DISABLE: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = 0;
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_HOSTNTP: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = 1;
pub type KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = i32;
pub const KSPROPERTY_NETWORKCAMERACONTROL_URI: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 1;
pub const KSPROPERTY_ONESHOT_DISCONNECT: KSPROPERTY_BTAUDIO = 1;
pub const KSPROPERTY_ONESHOT_RECONNECT: KSPROPERTY_BTAUDIO = 0;
pub type KSPROPERTY_OVERLAYUPDATE = i32;
pub const KSPROPERTY_OVERLAYUPDATE_CLIPLIST: KSPROPERTY_OVERLAYUPDATE = 1;
pub const KSPROPERTY_OVERLAYUPDATE_COLORKEY: KSPROPERTY_OVERLAYUPDATE = 4;
pub const KSPROPERTY_OVERLAYUPDATE_COLORREF: KSPROPERTY_OVERLAYUPDATE = 268435456;
pub const KSPROPERTY_OVERLAYUPDATE_DISPLAYCHANGE: KSPROPERTY_OVERLAYUPDATE = 16;
pub const KSPROPERTY_OVERLAYUPDATE_INTERESTS: KSPROPERTY_OVERLAYUPDATE = 0;
pub const KSPROPERTY_OVERLAYUPDATE_PALETTE: KSPROPERTY_OVERLAYUPDATE = 2;
pub const KSPROPERTY_OVERLAYUPDATE_VIDEOPOSITION: KSPROPERTY_OVERLAYUPDATE = 8;
pub const KSPROPERTY_PREFERRED_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = 2;
pub const KSPROPERTY_RAW_AVC_CMD: KSPROPERTY_EXTXPORT = 10;
pub type KSPROPERTY_RTAUDIO = i32;
pub const KSPROPERTY_RTAUDIO_BUFFER: KSPROPERTY_RTAUDIO = 1;
pub const KSPROPERTY_RTAUDIO_BUFFER_WITH_NOTIFICATION: KSPROPERTY_RTAUDIO = 5;
pub const KSPROPERTY_RTAUDIO_CLOCKREGISTER: KSPROPERTY_RTAUDIO = 4;
pub const KSPROPERTY_RTAUDIO_GETPOSITIONFUNCTION: KSPROPERTY_RTAUDIO = 0;
pub const KSPROPERTY_RTAUDIO_GETREADPACKET: KSPROPERTY_RTAUDIO = 11;
pub const KSPROPERTY_RTAUDIO_HWLATENCY: KSPROPERTY_RTAUDIO = 2;
pub const KSPROPERTY_RTAUDIO_PACKETCOUNT: KSPROPERTY_RTAUDIO = 9;
pub const KSPROPERTY_RTAUDIO_PACKETVREGISTER: KSPROPERTY_RTAUDIO = 13;
pub const KSPROPERTY_RTAUDIO_POSITIONREGISTER: KSPROPERTY_RTAUDIO = 3;
pub const KSPROPERTY_RTAUDIO_PRESENTATION_POSITION: KSPROPERTY_RTAUDIO = 10;
pub const KSPROPERTY_RTAUDIO_QUERY_NOTIFICATION_SUPPORT: KSPROPERTY_RTAUDIO = 8;
pub const KSPROPERTY_RTAUDIO_REGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = 6;
pub const KSPROPERTY_RTAUDIO_SETWRITEPACKET: KSPROPERTY_RTAUDIO = 12;
pub const KSPROPERTY_RTAUDIO_UNREGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = 7;
pub const KSPROPERTY_RTC_READER: KSPROPERTY_TIMECODE = 2;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_SELECTOR_NODE_S {
    pub NodeProperty: super::ks::KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_SELECTOR_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_SELECTOR_NUM_SOURCES: KSPROPERTY_VIDCAP_SELECTOR = 1;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_SELECTOR_S {
    pub Property: super::ks::KSPROPERTY,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_SELECTOR_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_SELECTOR_SOURCE_NODE_ID: KSPROPERTY_VIDCAP_SELECTOR = 0;
pub type KSPROPERTY_SOUNDDETECTOR = i32;
pub const KSPROPERTY_SOUNDDETECTOR_ARMED: KSPROPERTY_SOUNDDETECTOR = 3;
pub const KSPROPERTY_SOUNDDETECTOR_MATCHRESULT: KSPROPERTY_SOUNDDETECTOR = 4;
pub const KSPROPERTY_SOUNDDETECTOR_PATTERNS: KSPROPERTY_SOUNDDETECTOR = 2;
pub const KSPROPERTY_SOUNDDETECTOR_RESET: KSPROPERTY_SOUNDDETECTOR = 5;
pub const KSPROPERTY_SOUNDDETECTOR_STREAMINGSUPPORT: KSPROPERTY_SOUNDDETECTOR = 6;
pub const KSPROPERTY_SOUNDDETECTOR_SUPPORTEDPATTERNS: KSPROPERTY_SOUNDDETECTOR = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_SPPAL {
    pub sppal: [KS_DVD_YUV; 16],
}
impl Default for KSPROPERTY_SPPAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TELEPHONY_CALLCONTROL: KSPROPERTY_TELEPHONY_CONTROL = 2;
pub const KSPROPERTY_TELEPHONY_CALLHOLD: KSPROPERTY_TELEPHONY_CONTROL = 4;
pub const KSPROPERTY_TELEPHONY_CALLINFO: KSPROPERTY_TELEPHONY_CONTROL = 1;
pub type KSPROPERTY_TELEPHONY_CONTROL = i32;
pub const KSPROPERTY_TELEPHONY_ENDPOINTIDPAIR: KSPROPERTY_TELEPHONY_TOPOLOGY = 0;
pub const KSPROPERTY_TELEPHONY_MUTE_TX: KSPROPERTY_TELEPHONY_CONTROL = 5;
pub const KSPROPERTY_TELEPHONY_PROVIDERCHANGE: KSPROPERTY_TELEPHONY_CONTROL = 3;
pub const KSPROPERTY_TELEPHONY_PROVIDERID: KSPROPERTY_TELEPHONY_CONTROL = 0;
pub type KSPROPERTY_TELEPHONY_TOPOLOGY = i32;
pub const KSPROPERTY_TELEPHONY_VOLUME: KSPROPERTY_TELEPHONY_TOPOLOGY = 1;
pub type KSPROPERTY_TIMECODE = i32;
#[repr(C)]
#[cfg(all(feature = "ks", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TIMECODE_NODE_S {
    pub NodeProperty: super::ks::KSP_NODE,
    pub TimecodeSamp: TIMECODE_SAMPLE,
}
#[cfg(all(feature = "ks", feature = "winnt"))]
impl Default for KSPROPERTY_TIMECODE_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TIMECODE_READER: KSPROPERTY_TIMECODE = 0;
#[repr(C)]
#[cfg(all(feature = "ks", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TIMECODE_S {
    pub Property: super::ks::KSPROPERTY,
    pub TimecodeSamp: TIMECODE_SAMPLE,
}
#[cfg(all(feature = "ks", feature = "winnt"))]
impl Default for KSPROPERTY_TIMECODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KSPROPERTY_TOPOLOGYNODE = i32;
pub const KSPROPERTY_TOPOLOGYNODE_ENABLE: KSPROPERTY_TOPOLOGYNODE = 1;
pub const KSPROPERTY_TOPOLOGYNODE_RESET: KSPROPERTY_TOPOLOGYNODE = 2;
pub type KSPROPERTY_TUNER = i32;
pub const KSPROPERTY_TUNER_CAPS: KSPROPERTY_TUNER = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_CAPS_S {
    pub Property: super::ks::KSPROPERTY,
    pub ModesSupported: u32,
    pub VideoMedium: super::ks::KSPIN_MEDIUM,
    pub TVAudioMedium: super::ks::KSPIN_MEDIUM,
    pub RadioAudioMedium: super::ks::KSPIN_MEDIUM,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_FREQUENCY: KSPROPERTY_TUNER = 4;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_FREQUENCY_S {
    pub Property: super::ks::KSPROPERTY,
    pub Frequency: u32,
    pub LastFrequency: u32,
    pub TuningFlags: u32,
    pub VideoSubChannel: u32,
    pub AudioSubChannel: u32,
    pub Channel: u32,
    pub Country: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_FREQUENCY_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_IF_MEDIUM: KSPROPERTY_TUNER = 7;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_IF_MEDIUM_S {
    pub Property: super::ks::KSPROPERTY,
    pub IFMedium: super::ks::KSPIN_MEDIUM,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_IF_MEDIUM_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_INPUT: KSPROPERTY_TUNER = 5;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_INPUT_S {
    pub Property: super::ks::KSPROPERTY,
    pub InputIndex: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_INPUT_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_MODE: KSPROPERTY_TUNER = 2;
pub type KSPROPERTY_TUNER_MODES = i32;
pub const KSPROPERTY_TUNER_MODE_AM_RADIO: KSPROPERTY_TUNER_MODES = 4;
pub const KSPROPERTY_TUNER_MODE_ATSC: KSPROPERTY_TUNER_MODES = 16;
pub const KSPROPERTY_TUNER_MODE_CAPS: KSPROPERTY_TUNER = 1;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_MODE_CAPS_S {
    pub Property: super::ks::KSPROPERTY,
    pub Mode: u32,
    pub StandardsSupported: u32,
    pub MinFrequency: u32,
    pub MaxFrequency: u32,
    pub TuningGranularity: u32,
    pub NumberOfInputs: u32,
    pub SettlingTime: u32,
    pub Strategy: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_MODE_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_MODE_DSS: KSPROPERTY_TUNER_MODES = 8;
pub const KSPROPERTY_TUNER_MODE_FM_RADIO: KSPROPERTY_TUNER_MODES = 2;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_MODE_S {
    pub Property: super::ks::KSPROPERTY,
    pub Mode: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_MODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_MODE_TV: KSPROPERTY_TUNER_MODES = 1;
pub const KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS: KSPROPERTY_TUNER = 11;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    pub Property: super::ks::KSPROPERTY,
    pub NetworkType: windows_sys::core::GUID,
    pub BufferSize: u32,
    pub NetworkTunerCapabilities: *mut core::ffi::c_void,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_SCAN_CAPS: KSPROPERTY_TUNER = 8;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_SCAN_CAPS_S {
    pub Property: super::ks::KSPROPERTY,
    pub fSupportsHardwareAssistedScanning: windows_sys::core::BOOL,
    pub SupportedBroadcastStandards: u32,
    pub GUIDBucket: *mut core::ffi::c_void,
    pub lengthofBucket: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_SCAN_STATUS: KSPROPERTY_TUNER = 9;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_SCAN_STATUS_S {
    pub Property: super::ks::KSPROPERTY,
    pub LockStatus: TunerLockType,
    pub CurrentFrequency: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_SCAN_STATUS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_STANDARD: KSPROPERTY_TUNER = 3;
pub const KSPROPERTY_TUNER_STANDARD_MODE: KSPROPERTY_TUNER = 10;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_STANDARD_MODE_S {
    pub Property: super::ks::KSPROPERTY,
    pub AutoDetect: windows_sys::core::BOOL,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_STANDARD_MODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_STANDARD_S {
    pub Property: super::ks::KSPROPERTY,
    pub Standard: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_STANDARD_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_STATUS: KSPROPERTY_TUNER = 6;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_STATUS_S {
    pub Property: super::ks::KSPROPERTY,
    pub CurrentFrequency: u32,
    pub PLLOffset: u32,
    pub SignalStrength: u32,
    pub Busy: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TUNER_STATUS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TVAUDIO_CAPS: KSPROPERTY_VIDCAP_TVAUDIO = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TVAUDIO_CAPS_S {
    pub Property: super::ks::KSPROPERTY,
    pub Capabilities: u32,
    pub InputMedium: super::ks::KSPIN_MEDIUM,
    pub OutputMedium: super::ks::KSPIN_MEDIUM,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TVAUDIO_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TVAUDIO_CURRENTLY_AVAILABLE_MODES: KSPROPERTY_VIDCAP_TVAUDIO = 2;
pub const KSPROPERTY_TVAUDIO_MODE: KSPROPERTY_VIDCAP_TVAUDIO = 1;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TVAUDIO_S {
    pub Property: super::ks::KSPROPERTY,
    pub Mode: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_TVAUDIO_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KSPROPERTY_VBICAP = i32;
pub const KSPROPERTY_VBICAP_PROPERTIES_PROTECTION: KSPROPERTY_VBICAP = 1;
pub type KSPROPERTY_VBICODECFILTERING = i32;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    pub Property: super::ks::KSPROPERTY,
    pub Substreams: VBICODECFILTERING_CC_SUBSTREAMS,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    pub Property: super::ks::KSPROPERTY,
    pub Substreams: VBICODECFILTERING_NABTS_SUBSTREAMS,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = 2;
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = 1;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    pub Property: super::ks::KSPROPERTY,
    pub Scanlines: VBICODECFILTERING_SCANLINES,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VBICODECFILTERING_STATISTICS: KSPROPERTY_VBICODECFILTERING = 5;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    pub Property: super::ks::KSPROPERTY,
    pub Statistics: VBICODECFILTERING_STATISTICS_CC_PIN,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    pub Property: super::ks::KSPROPERTY,
    pub Statistics: VBICODECFILTERING_STATISTICS_CC,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    pub Property: super::ks::KSPROPERTY,
    pub Statistics: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    pub Property: super::ks::KSPROPERTY,
    pub Statistics: VBICODECFILTERING_STATISTICS_COMMON,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    pub Property: super::ks::KSPROPERTY,
    pub Statistics: VBICODECFILTERING_STATISTICS_NABTS_PIN,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    pub Property: super::ks::KSPROPERTY,
    pub Statistics: VBICODECFILTERING_STATISTICS_NABTS,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = 4;
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = 3;
pub type KSPROPERTY_VIDCAP_CAMERACONTROL = i32;
pub type KSPROPERTY_VIDCAP_CROSSBAR = i32;
pub type KSPROPERTY_VIDCAP_DROPPEDFRAMES = i32;
pub type KSPROPERTY_VIDCAP_SELECTOR = i32;
pub type KSPROPERTY_VIDCAP_TVAUDIO = i32;
pub type KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = i32;
pub type KSPROPERTY_VIDCAP_VIDEOCONTROL = i32;
pub type KSPROPERTY_VIDCAP_VIDEODECODER = i32;
pub type KSPROPERTY_VIDCAP_VIDEOENCODER = i32;
pub type KSPROPERTY_VIDCAP_VIDEOPROCAMP = i32;
pub const KSPROPERTY_VIDEOCOMPRESSION_GETINFO: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    pub Property: super::ks::KSPROPERTY,
    pub StreamIndex: u32,
    pub DefaultKeyFrameRate: i32,
    pub DefaultPFrameRate: i32,
    pub DefaultQuality: i32,
    pub NumberOfQualitySettings: i32,
    pub Capabilities: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCOMPRESSION_KEYFRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 1;
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_FRAME_SIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 5;
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 4;
pub const KSPROPERTY_VIDEOCOMPRESSION_PFRAMES_PER_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 2;
pub const KSPROPERTY_VIDEOCOMPRESSION_QUALITY: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 3;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCOMPRESSION_S {
    pub Property: super::ks::KSPROPERTY,
    pub StreamIndex: u32,
    pub Value: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOCOMPRESSION_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCOMPRESSION_S1 {
    pub Property: super::ks::KSPROPERTY,
    pub StreamIndex: u32,
    pub Value: i32,
    pub Flags: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCOMPRESSION_WINDOWSIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = 6;
pub const KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCONTROL = 1;
#[repr(C)]
#[cfg(all(feature = "ks", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    pub Property: super::ks::KSPROPERTY,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::windef::SIZE,
    pub CurrentActualFrameRate: i64,
    pub CurrentMaxAvailableFrameRate: i64,
}
#[cfg(all(feature = "ks", feature = "windef"))]
impl Default for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCONTROL_CAPS: KSPROPERTY_VIDCAP_VIDEOCONTROL = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCONTROL_CAPS_S {
    pub Property: super::ks::KSPROPERTY,
    pub StreamIndex: u32,
    pub VideoControlCaps: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCONTROL_FRAME_RATES: KSPROPERTY_VIDCAP_VIDEOCONTROL = 2;
#[repr(C)]
#[cfg(all(feature = "ks", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    pub Property: super::ks::KSPROPERTY,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::windef::SIZE,
}
#[cfg(all(feature = "ks", feature = "windef"))]
impl Default for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCONTROL_MODE: KSPROPERTY_VIDCAP_VIDEOCONTROL = 3;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCONTROL_MODE_S {
    pub Property: super::ks::KSPROPERTY,
    pub StreamIndex: u32,
    pub Mode: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOCONTROL_MODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEODECODER_CAPS: KSPROPERTY_VIDCAP_VIDEODECODER = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEODECODER_CAPS_S {
    pub Property: super::ks::KSPROPERTY,
    pub StandardsSupported: u32,
    pub Capabilities: u32,
    pub SettlingTime: u32,
    pub HSyncPerVSync: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEODECODER_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEODECODER_OUTPUT_ENABLE: KSPROPERTY_VIDCAP_VIDEODECODER = 3;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEODECODER_S {
    pub Property: super::ks::KSPROPERTY,
    pub Value: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEODECODER_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEODECODER_STANDARD: KSPROPERTY_VIDCAP_VIDEODECODER = 1;
pub const KSPROPERTY_VIDEODECODER_STATUS: KSPROPERTY_VIDCAP_VIDEODECODER = 2;
pub const KSPROPERTY_VIDEODECODER_STATUS2: KSPROPERTY_VIDCAP_VIDEODECODER = 5;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEODECODER_STATUS2_S {
    pub Property: super::ks::KSPROPERTY,
    pub NumberOfLines: u32,
    pub SignalLocked: u32,
    pub ChromaLock: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEODECODER_STATUS2_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEODECODER_STATUS_S {
    pub Property: super::ks::KSPROPERTY,
    pub NumberOfLines: u32,
    pub SignalLocked: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEODECODER_STATUS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEODECODER_VCR_TIMING: KSPROPERTY_VIDCAP_VIDEODECODER = 4;
pub const KSPROPERTY_VIDEOENCODER_CAPS: KSPROPERTY_VIDCAP_VIDEOENCODER = 0;
pub const KSPROPERTY_VIDEOENCODER_CC_ENABLE: KSPROPERTY_VIDCAP_VIDEOENCODER = 3;
pub const KSPROPERTY_VIDEOENCODER_COPYPROTECTION: KSPROPERTY_VIDCAP_VIDEOENCODER = 2;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOENCODER_S {
    pub Property: super::ks::KSPROPERTY,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOENCODER_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOENCODER_STANDARD: KSPROPERTY_VIDCAP_VIDEOENCODER = 1;
pub const KSPROPERTY_VIDEOPROCAMP_BACKLIGHT_COMPENSATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 8;
pub const KSPROPERTY_VIDEOPROCAMP_BRIGHTNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 0;
pub const KSPROPERTY_VIDEOPROCAMP_COLORENABLE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 6;
pub const KSPROPERTY_VIDEOPROCAMP_CONTRAST: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 1;
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 10;
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER_LIMIT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 11;
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_AUTO: u32 = 1;
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_MANUAL: u32 = 2;
pub const KSPROPERTY_VIDEOPROCAMP_GAIN: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 9;
pub const KSPROPERTY_VIDEOPROCAMP_GAMMA: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 5;
pub const KSPROPERTY_VIDEOPROCAMP_HUE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 2;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S {
    pub NodeProperty: super::ks::KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    pub NodeProperty: super::ks::KSP_NODE,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOPROCAMP_POWERLINE_FREQUENCY: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 13;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_S {
    pub Property: super::ks::KSPROPERTY,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOPROCAMP_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_S2 {
    pub Property: super::ks::KSPROPERTY,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
#[cfg(feature = "ks")]
impl Default for KSPROPERTY_VIDEOPROCAMP_S2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOPROCAMP_SATURATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 3;
pub const KSPROPERTY_VIDEOPROCAMP_SHARPNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 4;
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 7;
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE_COMPONENT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 12;
pub type KSPROPERTY_VIDMEM_TRANSPORT = i32;
pub type KSPROPERTY_VPCONFIG = i32;
pub const KSPROPERTY_VPCONFIG_DDRAWHANDLE: KSPROPERTY_VPCONFIG = 12;
pub const KSPROPERTY_VPCONFIG_DDRAWSURFACEHANDLE: KSPROPERTY_VPCONFIG = 14;
pub const KSPROPERTY_VPCONFIG_DECIMATIONCAPABILITY: KSPROPERTY_VPCONFIG = 10;
pub const KSPROPERTY_VPCONFIG_GETCONNECTINFO: KSPROPERTY_VPCONFIG = 1;
pub const KSPROPERTY_VPCONFIG_GETVIDEOFORMAT: KSPROPERTY_VPCONFIG = 7;
pub const KSPROPERTY_VPCONFIG_INFORMVPINPUT: KSPROPERTY_VPCONFIG = 5;
pub const KSPROPERTY_VPCONFIG_INVERTPOLARITY: KSPROPERTY_VPCONFIG = 9;
pub const KSPROPERTY_VPCONFIG_MAXPIXELRATE: KSPROPERTY_VPCONFIG = 4;
pub const KSPROPERTY_VPCONFIG_NUMCONNECTINFO: KSPROPERTY_VPCONFIG = 0;
pub const KSPROPERTY_VPCONFIG_NUMVIDEOFORMAT: KSPROPERTY_VPCONFIG = 6;
pub const KSPROPERTY_VPCONFIG_SCALEFACTOR: KSPROPERTY_VPCONFIG = 11;
pub const KSPROPERTY_VPCONFIG_SETCONNECTINFO: KSPROPERTY_VPCONFIG = 2;
pub const KSPROPERTY_VPCONFIG_SETVIDEOFORMAT: KSPROPERTY_VPCONFIG = 8;
pub const KSPROPERTY_VPCONFIG_SURFACEPARAMS: KSPROPERTY_VPCONFIG = 15;
pub const KSPROPERTY_VPCONFIG_VIDEOPORTID: KSPROPERTY_VPCONFIG = 13;
pub const KSPROPERTY_VPCONFIG_VPDATAINFO: KSPROPERTY_VPCONFIG = 3;
pub type KSPROPERTY_WAVE = i32;
pub const KSPROPERTY_WAVE_BUFFER: KSPROPERTY_WAVE = 3;
pub const KSPROPERTY_WAVE_COMPATIBLE_CAPABILITIES: KSPROPERTY_WAVE = 0;
pub const KSPROPERTY_WAVE_FREQUENCY: KSPROPERTY_WAVE = 4;
pub const KSPROPERTY_WAVE_INPUT_CAPABILITIES: KSPROPERTY_WAVE = 1;
pub const KSPROPERTY_WAVE_OUTPUT_CAPABILITIES: KSPROPERTY_WAVE = 2;
pub const KSPROPERTY_WAVE_PAN: KSPROPERTY_WAVE = 6;
pub const KSPROPERTY_WAVE_VOLUME: KSPROPERTY_WAVE = 5;
pub const KSPROPERYT_NETWORKCAMERACONTROL_NTPINFO_TYPE_CUSTOM: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AC3(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_Audio(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AudioBufferDuration(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AudioDecoderOut(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AudioEngine(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AudioLoopback(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AudioModule(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AudioPosture(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AudioResourceManagement(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_AudioSignalProcessing(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_Bibliographic(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_BtAudio(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_CopyProt(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_Cyclic(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_DirectSound3DBuffer(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_DirectSound3DListener(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_DrmAudioStream(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_DvdSubPic(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_FMRXControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_FMRXTopology(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_Hrtf3d(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_InterleavedAudio(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_Itd3d(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_Jack(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_MPEG4_MediaType_Attributes(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_MidiLoopedStreaming(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_Mpeg2Vid(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_OverlayUpdate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_RtAudio(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_SoundDetector(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_SoundDetector2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_TSRateChange(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_TelephonyControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_TelephonyTopology(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_TopologyNode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_VBICAP_PROPERTIES(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_VBICodecFiltering(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_VPConfig(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_VPVBIConfig(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_VramCapture(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSPROPSETID_Wave(pub u8);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER {
    pub BufferAddress: *mut core::ffi::c_void,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: windows_sys::core::BOOL,
}
impl Default for KSRTAUDIO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSRTAUDIO_BUFFER32 {
    pub BufferAddress: u32,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY {
    pub Property: super::ks::KSPROPERTY,
    pub BaseAddress: *mut core::ffi::c_void,
    pub RequestedBufferSize: u32,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY32 {
    pub Property: super::ks::KSPROPERTY,
    pub BaseAddress: u32,
    pub RequestedBufferSize: u32,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_BUFFER_PROPERTY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    pub Property: super::ks::KSPROPERTY,
    pub BaseAddress: *mut core::ffi::c_void,
    pub RequestedBufferSize: u32,
    pub NotificationCount: u32,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    pub Property: super::ks::KSPROPERTY,
    pub BaseAddress: u32,
    pub RequestedBufferSize: u32,
    pub NotificationCount: u32,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSRTAUDIO_GETREADPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub PerformanceCounterValue: u64,
    pub MoreData: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSRTAUDIO_HWLATENCY {
    pub FifoSize: u32,
    pub ChipsetDelay: u32,
    pub CodecDelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_HWREGISTER {
    pub Register: *mut core::ffi::c_void,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
impl Default for KSRTAUDIO_HWREGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSRTAUDIO_HWREGISTER32 {
    pub Register: u32,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_HWREGISTER_PROPERTY {
    pub Property: super::ks::KSPROPERTY,
    pub BaseAddress: *mut core::ffi::c_void,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_HWREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_HWREGISTER_PROPERTY32 {
    pub Property: super::ks::KSPROPERTY,
    pub BaseAddress: u32,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    pub Property: super::ks::KSPROPERTY,
    pub NotificationEvent: super::winnt::HANDLE,
}
#[cfg(all(feature = "ks", feature = "winnt"))]
impl Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    pub Property: super::ks::KSPROPERTY,
    pub NotificationEvent: u32,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_PACKETVREGISTER {
    pub CompletedPacketCount: super::basetsd::PULONG64,
    pub CompletedPacketQPC: super::basetsd::PULONG64,
    pub CompletedPacketHash: super::basetsd::PULONG64,
}
#[cfg(feature = "basetsd")]
impl Default for KSRTAUDIO_PACKETVREGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    pub Property: super::ks::KSPROPERTY,
    pub BaseAddress: *mut core::ffi::c_void,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSRTAUDIO_SETWRITEPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub EosPacketLength: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSSOUNDDETECTORPROPERTY {
    pub Property: super::ks::KSPROPERTY,
    pub EventId: windows_sys::core::GUID,
}
#[cfg(feature = "ks")]
impl Default for KSSOUNDDETECTORPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSTELEPHONY_CALLCONTROL {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallControlOp: TELEPHONY_CALLCONTROLOP,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSTELEPHONY_CALLINFO {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallState: TELEPHONY_CALLSTATE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSTELEPHONY_PROVIDERCHANGE {
    pub CallType: TELEPHONY_CALLTYPE,
    pub ProviderChangeOp: TELEPHONY_PROVIDERCHANGEOP,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSTOPOLOGY_ENDPOINTID {
    pub TopologyName: [u16; 260],
    pub PinId: u32,
}
impl Default for KSTOPOLOGY_ENDPOINTID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSTOPOLOGY_ENDPOINTIDPAIR {
    pub RenderEndpoint: KSTOPOLOGY_ENDPOINTID,
    pub CaptureEndpoint: KSTOPOLOGY_ENDPOINTID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSVPMAXPIXELRATE {
    pub Size: KS_AMVPSIZE,
    pub MaxPixelsPerSecond: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSVPSIZE_PROP {
    pub Property: super::ks::KSPROPERTY,
    pub Size: KS_AMVPSIZE,
}
#[cfg(feature = "ks")]
impl Default for KSVPSIZE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSVPSURFACEPARAMS {
    pub dwPitch: u32,
    pub dwXOrigin: u32,
    pub dwYOrigin: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSWAVETABLE_WAVE_DESC {
    pub Identifier: super::ks::KSIDENTIFIER,
    pub Size: u32,
    pub Looped: windows_sys::core::BOOL,
    pub LoopPoint: u32,
    pub InROM: windows_sys::core::BOOL,
    pub Format: super::ks::KSDATAFORMAT,
}
#[cfg(feature = "ks")]
impl Default for KSWAVETABLE_WAVE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSWAVE_BUFFER {
    pub Attributes: u32,
    pub BufferSize: u32,
    pub BufferAddress: *mut core::ffi::c_void,
}
impl Default for KSWAVE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSWAVE_BUFFER_ATTRIBUTEF_LOOPING: u32 = 1;
pub const KSWAVE_BUFFER_ATTRIBUTEF_STATIC: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSWAVE_COMPATCAPS {
    pub ulDeviceType: u32,
}
pub const KSWAVE_COMPATCAPS_INPUT: u32 = 0;
pub const KSWAVE_COMPATCAPS_OUTPUT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSWAVE_INPUT_CAPABILITIES {
    pub MaximumChannelsPerConnection: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
    pub TotalConnections: u32,
    pub ActiveConnections: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KSWAVE_VOLUME {
    pub LeftAttenuation: i32,
    pub RightAttenuation: i32,
}
pub const KS_AMCONTROL_COLORINFO_PRESENT: u32 = 128;
pub const KS_AMCONTROL_PAD_TO_16x9: u32 = 4;
pub const KS_AMCONTROL_PAD_TO_4x3: u32 = 2;
pub const KS_AMCONTROL_USED: u32 = 1;
pub type KS_AMPixAspectRatio = i32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KS_AMVPDATAINFO {
    pub dwSize: u32,
    pub dwMicrosecondsPerField: u32,
    pub amvpDimInfo: KS_AMVPDIMINFO,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub bEnableDoubleClock: windows_sys::core::BOOL,
    pub bEnableVACT: windows_sys::core::BOOL,
    pub bDataIsInterlaced: windows_sys::core::BOOL,
    pub lHalfLinesOdd: i32,
    pub bFieldPolarityInverted: windows_sys::core::BOOL,
    pub dwNumLinesInVREF: u32,
    pub lHalfLinesEven: i32,
    pub dwReserved1: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KS_AMVPDIMINFO {
    pub dwFieldWidth: u32,
    pub dwFieldHeight: u32,
    pub dwVBIWidth: u32,
    pub dwVBIHeight: u32,
    pub rcValidRegion: super::windef::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_AMVPSIZE {
    pub dwWidth: u32,
    pub dwHeight: u32,
}
pub const KS_AMVP_BEST_BANDWIDTH: KS_AMVP_SELECTFORMATBY = 1;
pub const KS_AMVP_DO_NOT_CARE: KS_AMVP_SELECTFORMATBY = 0;
pub const KS_AMVP_INPUT_SAME_AS_OUTPUT: KS_AMVP_SELECTFORMATBY = 2;
pub type KS_AMVP_MODE = i32;
pub const KS_AMVP_MODE_BOBINTERLEAVED: KS_AMVP_MODE = 1;
pub const KS_AMVP_MODE_BOBNONINTERLEAVED: KS_AMVP_MODE = 2;
pub const KS_AMVP_MODE_SKIPEVEN: KS_AMVP_MODE = 3;
pub const KS_AMVP_MODE_SKIPODD: KS_AMVP_MODE = 4;
pub const KS_AMVP_MODE_WEAVE: KS_AMVP_MODE = 0;
pub type KS_AMVP_SELECTFORMATBY = i32;
#[repr(C)]
#[cfg(feature = "mediaobj")]
#[derive(Clone, Copy, Default)]
pub struct KS_AM_ExactRateChange {
    pub OutputZeroTime: super::mediaobj::REFERENCE_TIME,
    pub Rate: i32,
}
pub type KS_AM_MaxFullDataRate = i32;
pub type KS_AM_PROPERTY_TS_RATE_CHANGE = i32;
pub const KS_AM_RATE_ExactRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = 2;
pub const KS_AM_RATE_MaxFullDataRate: KS_AM_PROPERTY_TS_RATE_CHANGE = 3;
pub const KS_AM_RATE_SimpleRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = 1;
pub const KS_AM_RATE_Step: KS_AM_PROPERTY_TS_RATE_CHANGE = 4;
#[repr(C)]
#[cfg(feature = "mediaobj")]
#[derive(Clone, Copy, Default)]
pub struct KS_AM_SimpleRateChange {
    pub StartTime: super::mediaobj::REFERENCE_TIME,
    pub Rate: i32,
}
pub type KS_AM_Step = u32;
pub const KS_AM_UseNewCSSKey: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct KS_ANALOGVIDEOINFO {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: super::mediaobj::REFERENCE_TIME,
}
pub type KS_AnalogVideoStandard = i32;
pub const KS_AnalogVideo_NTSC_433: KS_AnalogVideoStandard = 4;
pub const KS_AnalogVideo_NTSC_M: KS_AnalogVideoStandard = 1;
pub const KS_AnalogVideo_NTSC_M_J: KS_AnalogVideoStandard = 2;
pub const KS_AnalogVideo_NTSC_Mask: u32 = 7;
pub const KS_AnalogVideo_None: KS_AnalogVideoStandard = 0;
pub const KS_AnalogVideo_PAL_60: KS_AnalogVideoStandard = 2048;
pub const KS_AnalogVideo_PAL_B: KS_AnalogVideoStandard = 16;
pub const KS_AnalogVideo_PAL_D: KS_AnalogVideoStandard = 32;
pub const KS_AnalogVideo_PAL_G: KS_AnalogVideoStandard = 64;
pub const KS_AnalogVideo_PAL_H: KS_AnalogVideoStandard = 128;
pub const KS_AnalogVideo_PAL_I: KS_AnalogVideoStandard = 256;
pub const KS_AnalogVideo_PAL_M: KS_AnalogVideoStandard = 512;
pub const KS_AnalogVideo_PAL_Mask: u32 = 1052656;
pub const KS_AnalogVideo_PAL_N: KS_AnalogVideoStandard = 1024;
pub const KS_AnalogVideo_PAL_N_COMBO: KS_AnalogVideoStandard = 1048576;
pub const KS_AnalogVideo_SECAM_B: KS_AnalogVideoStandard = 4096;
pub const KS_AnalogVideo_SECAM_D: KS_AnalogVideoStandard = 8192;
pub const KS_AnalogVideo_SECAM_G: KS_AnalogVideoStandard = 16384;
pub const KS_AnalogVideo_SECAM_H: KS_AnalogVideoStandard = 32768;
pub const KS_AnalogVideo_SECAM_K: KS_AnalogVideoStandard = 65536;
pub const KS_AnalogVideo_SECAM_K1: KS_AnalogVideoStandard = 131072;
pub const KS_AnalogVideo_SECAM_L: KS_AnalogVideoStandard = 262144;
pub const KS_AnalogVideo_SECAM_L1: KS_AnalogVideoStandard = 524288;
pub const KS_AnalogVideo_SECAM_Mask: u32 = 1044480;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const KS_BI_BITFIELDS: u32 = 3;
pub const KS_BI_JPEG: u32 = 4;
pub const KS_BI_RGB: u32 = 0;
pub const KS_BI_RLE4: u32 = 2;
pub const KS_BI_RLE8: u32 = 1;
pub const KS_CAMERACONTROL_ASYNC_RESET: KS_CameraControlAsyncOperation = 3;
pub const KS_CAMERACONTROL_ASYNC_START: KS_CameraControlAsyncOperation = 1;
pub const KS_CAMERACONTROL_ASYNC_STOP: KS_CameraControlAsyncOperation = 2;
pub const KS_CAPTURE_ALLOC_INVALID: CAPTURE_MEMORY_ALLOCATION_FLAGS = 0;
pub const KS_CAPTURE_ALLOC_SECURE_BUFFER: CAPTURE_MEMORY_ALLOCATION_FLAGS = 16;
pub const KS_CAPTURE_ALLOC_SYSTEM: CAPTURE_MEMORY_ALLOCATION_FLAGS = 1;
pub const KS_CAPTURE_ALLOC_SYSTEM_AGP: CAPTURE_MEMORY_ALLOCATION_FLAGS = 4;
pub const KS_CAPTURE_ALLOC_VRAM: CAPTURE_MEMORY_ALLOCATION_FLAGS = 2;
pub const KS_CAPTURE_ALLOC_VRAM_MAPPED: CAPTURE_MEMORY_ALLOCATION_FLAGS = 8;
pub const KS_CC_SUBSTREAM_EVEN: u32 = 2;
pub const KS_CC_SUBSTREAM_FIELD1_MASK: u32 = 240;
pub const KS_CC_SUBSTREAM_FIELD2_MASK: u32 = 7936;
pub const KS_CC_SUBSTREAM_ODD: u32 = 1;
pub const KS_CC_SUBSTREAM_SERVICE_CC1: u32 = 16;
pub const KS_CC_SUBSTREAM_SERVICE_CC2: u32 = 32;
pub const KS_CC_SUBSTREAM_SERVICE_CC3: u32 = 256;
pub const KS_CC_SUBSTREAM_SERVICE_CC4: u32 = 512;
pub const KS_CC_SUBSTREAM_SERVICE_T1: u32 = 64;
pub const KS_CC_SUBSTREAM_SERVICE_T2: u32 = 128;
pub const KS_CC_SUBSTREAM_SERVICE_T3: u32 = 1024;
pub const KS_CC_SUBSTREAM_SERVICE_T4: u32 = 2048;
pub const KS_CC_SUBSTREAM_SERVICE_XDS: u32 = 4096;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_COLCON {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
}
pub const KS_COPYPROTECT_RestrictDuplication: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_COPY_MACROVISION {
    pub MACROVISIONLevel: u32,
}
pub type KS_COPY_MACROVISION_LEVEL = i32;
pub type KS_CameraControlAsyncOperation = i32;
pub type KS_CompressionCaps = i32;
pub const KS_CompressionCaps_CanBFrame: KS_CompressionCaps = 8;
pub const KS_CompressionCaps_CanCrunch: KS_CompressionCaps = 2;
pub const KS_CompressionCaps_CanKeyFrame: KS_CompressionCaps = 4;
pub const KS_CompressionCaps_CanQuality: KS_CompressionCaps = 1;
pub const KS_CompressionCaps_CanWindow: KS_CompressionCaps = 16;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_H264VIDEOINFO {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub H264VideoInfoHeader: KS_H264VIDEOINFO,
}
#[cfg(feature = "ks")]
impl Default for KS_DATAFORMAT_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_IMAGEINFO {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub ImageInfoHeader: KS_BITMAPINFOHEADER,
}
#[cfg(feature = "ks")]
impl Default for KS_DATAFORMAT_IMAGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_MPEGVIDEOINFO2 {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub MpegVideoInfoHeader2: KS_MPEGVIDEOINFO2,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_VBIINFOHEADER {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
#[cfg(feature = "ks")]
impl Default for KS_DATAFORMAT_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATAFORMAT_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER2 {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub VideoInfoHeader2: KS_VIDEOINFOHEADER2,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_VIDEOINFO_PALETTE {
    pub DataFormat: super::ks::KSDATAFORMAT,
    pub VideoInfo: KS_VIDEOINFO,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_ANALOGVIDEO {
    pub DataRange: super::ks::KSDATARANGE,
    pub AnalogVideoInfo: KS_ANALOGVIDEOINFO,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATARANGE_ANALOGVIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_H264_VIDEO {
    pub DataRange: super::ks::KSDATARANGE,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_H264VIDEOINFO,
}
#[cfg(all(feature = "ks", feature = "windef"))]
impl Default for KS_DATARANGE_H264_VIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_IMAGE {
    pub DataRange: super::ks::KSDATARANGE,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub ImageInfoHeader: KS_BITMAPINFOHEADER,
}
#[cfg(all(feature = "ks", feature = "windef"))]
impl Default for KS_DATARANGE_IMAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_MPEG1_VIDEO {
    pub DataRange: super::ks::KSDATARANGE,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_MPEG1VIDEOINFO,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATARANGE_MPEG1_VIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_MPEG2_VIDEO {
    pub DataRange: super::ks::KSDATARANGE,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_MPEGVIDEOINFO2,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATARANGE_MPEG2_VIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_VIDEO {
    pub DataRange: super::ks::KSDATARANGE,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATARANGE_VIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_VIDEO2 {
    pub DataRange: super::ks::KSDATARANGE,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER2,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATARANGE_VIDEO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_VIDEO_PALETTE {
    pub DataRange: super::ks::KSDATARANGE,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfo: KS_VIDEOINFO,
}
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
impl Default for KS_DATARANGE_VIDEO_PALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ks", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_VIDEO_VBI {
    pub DataRange: super::ks::KSDATARANGE,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
#[cfg(all(feature = "ks", feature = "windef"))]
impl Default for KS_DATARANGE_VIDEO_VBI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KS_DVDCOPYSTATE = i32;
pub const KS_DVDCOPYSTATE_AUTHENTICATION_NOT_REQUIRED: KS_DVDCOPYSTATE = 2;
pub const KS_DVDCOPYSTATE_AUTHENTICATION_REQUIRED: KS_DVDCOPYSTATE = 3;
pub const KS_DVDCOPYSTATE_DONE: KS_DVDCOPYSTATE = 4;
pub const KS_DVDCOPYSTATE_INITIALIZE: KS_DVDCOPYSTATE = 0;
pub const KS_DVDCOPYSTATE_INITIALIZE_TITLE: KS_DVDCOPYSTATE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DVDCOPY_BUSKEY {
    pub BusKey: [u8; 5],
    pub Reserved: [u8; 1],
}
impl Default for KS_DVDCOPY_BUSKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DVDCOPY_CHLGKEY {
    pub ChlgKey: [u8; 10],
    pub Reserved: [u8; 2],
}
impl Default for KS_DVDCOPY_CHLGKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DVDCOPY_DISCKEY {
    pub DiscKey: [u8; 2048],
}
impl Default for KS_DVDCOPY_DISCKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DVDCOPY_REGION {
    pub Reserved: u8,
    pub RegionData: u8,
    pub Reserved2: [u8; 2],
}
impl Default for KS_DVDCOPY_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_DVDCOPY_SET_COPY_STATE {
    pub DVDCopyState: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DVDCOPY_TITLEKEY {
    pub KeyFlags: u32,
    pub ReservedNT: [u32; 2],
    pub TitleKey: [u8; 6],
    pub Reserved: [u8; 2],
}
impl Default for KS_DVDCOPY_TITLEKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_DVD_CGMS_COPY_ONCE: u32 = 16;
pub const KS_DVD_CGMS_COPY_PERMITTED: u32 = 0;
pub const KS_DVD_CGMS_COPY_PROTECT_MASK: u32 = 24;
pub const KS_DVD_CGMS_NO_COPY: u32 = 24;
pub const KS_DVD_CGMS_RESERVED_MASK: u32 = 120;
pub const KS_DVD_COPYRIGHTED: u32 = 64;
pub const KS_DVD_COPYRIGHT_MASK: u32 = 64;
pub const KS_DVD_NOT_COPYRIGHTED: u32 = 0;
pub const KS_DVD_SECTOR_NOT_PROTECTED: u32 = 0;
pub const KS_DVD_SECTOR_PROTECTED: u32 = 32;
pub const KS_DVD_SECTOR_PROTECT_MASK: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_DVD_YCrCb {
    pub Reserved: u8,
    pub Y: u8,
    pub Cr: u8,
    pub Cb: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_DVD_YUV {
    pub Reserved: u8,
    pub Y: u8,
    pub V: u8,
    pub U: u8,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KS_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub hDirectDraw: super::winnt::HANDLE,
    pub hSurfaceHandle: super::winnt::HANDLE,
    pub DirectDrawRect: super::windef::RECT,
    pub Anonymous: KS_FRAME_INFO_0,
    pub Reserved2: u32,
    pub Anonymous2: KS_FRAME_INFO_1,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for KS_FRAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KS_FRAME_INFO_0 {
    pub lSurfacePitch: i32,
    pub Reserved1: u32,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for KS_FRAME_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KS_FRAME_INFO_1 {
    pub Anonymous: KS_FRAME_INFO_1_0,
    pub FrameCompletionNumber: u64,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for KS_FRAME_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KS_FRAME_INFO_1_0 {
    pub Reserved3: u32,
    pub Reserved4: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const KS_INTERLACE_1FieldPerSample: u32 = 2;
pub const KS_INTERLACE_DisplayModeBobOnly: u32 = 0;
pub const KS_INTERLACE_DisplayModeBobOrWeave: u32 = 128;
pub const KS_INTERLACE_DisplayModeMask: u32 = 192;
pub const KS_INTERLACE_DisplayModeWeaveOnly: u32 = 64;
pub const KS_INTERLACE_Field1First: u32 = 4;
pub const KS_INTERLACE_FieldPatBothIrregular: u32 = 48;
pub const KS_INTERLACE_FieldPatBothRegular: u32 = 32;
pub const KS_INTERLACE_FieldPatField1Only: u32 = 0;
pub const KS_INTERLACE_FieldPatField2Only: u32 = 16;
pub const KS_INTERLACE_FieldPatternMask: u32 = 48;
pub const KS_INTERLACE_IsInterlaced: u32 = 1;
pub const KS_INTERLACE_UNUSED: u32 = 8;
pub const KS_MACROVISION_DISABLED: KS_COPY_MACROVISION_LEVEL = 0;
pub const KS_MACROVISION_LEVEL1: KS_COPY_MACROVISION_LEVEL = 1;
pub const KS_MACROVISION_LEVEL2: KS_COPY_MACROVISION_LEVEL = 2;
pub const KS_MACROVISION_LEVEL3: KS_COPY_MACROVISION_LEVEL = 3;
pub const KS_MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140;
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_MPEG1VIDEOINFO {
    pub hdr: KS_VIDEOINFOHEADER,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub bSequenceHeader: [u8; 1],
}
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for KS_MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KS_MPEG2Level = i32;
pub const KS_MPEG2Level_High: KS_MPEG2Level = 3;
pub const KS_MPEG2Level_High1440: KS_MPEG2Level = 2;
pub const KS_MPEG2Level_Low: KS_MPEG2Level = 0;
pub const KS_MPEG2Level_Main: KS_MPEG2Level = 1;
pub type KS_MPEG2Profile = i32;
pub const KS_MPEG2Profile_High: KS_MPEG2Profile = 4;
pub const KS_MPEG2Profile_Main: KS_MPEG2Profile = 1;
pub const KS_MPEG2Profile_SNRScalable: KS_MPEG2Profile = 2;
pub const KS_MPEG2Profile_Simple: KS_MPEG2Profile = 0;
pub const KS_MPEG2Profile_SpatiallyScalable: KS_MPEG2Profile = 3;
pub const KS_MPEG2_27MhzTimebase: u32 = 256;
pub const KS_MPEG2_DSS_UserData: u32 = 64;
pub const KS_MPEG2_DVB_UserData: u32 = 128;
pub const KS_MPEG2_DVDLine21Field1: u32 = 2;
pub const KS_MPEG2_DVDLine21Field2: u32 = 4;
pub const KS_MPEG2_DoPanScan: u32 = 1;
pub const KS_MPEG2_FilmCameraMode: u32 = 16;
pub const KS_MPEG2_LetterboxAnalogOut: u32 = 32;
pub const KS_MPEG2_SourceIsLetterboxed: u32 = 8;
pub const KS_MPEG2_WidescreenAnalogOut: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_MPEGAUDIOINFO {
    pub dwFlags: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
}
pub const KS_MPEGAUDIOINFO_27MhzTimebase: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_MPEGVIDEOINFO2 {
    pub hdr: KS_VIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub bSequenceHeader: [u32; 1],
}
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for KS_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_ADVERTISER_BASE: u32 = 2224;
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_CONTENT_BASE: u32 = 2208;
pub const KS_NABTS_GROUPID_MICROSOFT_RESERVED_TEST_DATA_BASE: u32 = 2288;
pub const KS_NABTS_GROUPID_NETWORK_WIDE_ADVERTISER_BASE: u32 = 2160;
pub const KS_NABTS_GROUPID_NETWORK_WIDE_CONTENT_BASE: u32 = 2144;
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_ADVERTISER_BASE: u32 = 2064;
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_BASE: u32 = 2048;
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_ADVERTISER_BASE: u32 = 2096;
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_CONTENT_BASE: u32 = 2080;
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_ADVERTISER_BASE: u32 = 2128;
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_CONTENT_BASE: u32 = 2112;
pub const KS_NABTS_GROUPID_TELEVISION_STATION_ADVERTISER_BASE: u32 = 2192;
pub const KS_NABTS_GROUPID_TELEVISION_STATION_CONTENT_BASE: u32 = 2176;
pub const KS_Obsolete_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = 16;
pub const KS_Obsolete_VideoControlFlag_Trigger: KS_VideoControlFlags = 32;
pub const KS_PhysConn_Audio_1394: KS_PhysicalConnectorType = 4103;
pub const KS_PhysConn_Audio_AESDigital: KS_PhysicalConnectorType = 4099;
pub const KS_PhysConn_Audio_AUX: KS_PhysicalConnectorType = 4102;
pub const KS_PhysConn_Audio_AudioDecoder: KS_PhysicalConnectorType = 4105;
pub const KS_PhysConn_Audio_Line: KS_PhysicalConnectorType = 4097;
pub const KS_PhysConn_Audio_Mic: KS_PhysicalConnectorType = 4098;
pub const KS_PhysConn_Audio_SCSI: KS_PhysicalConnectorType = 4101;
pub const KS_PhysConn_Audio_SPDIFDigital: KS_PhysicalConnectorType = 4100;
pub const KS_PhysConn_Audio_Tuner: KS_PhysicalConnectorType = 4096;
pub const KS_PhysConn_Audio_USB: KS_PhysicalConnectorType = 4104;
pub const KS_PhysConn_Video_1394: KS_PhysicalConnectorType = 10;
pub const KS_PhysConn_Video_AUX: KS_PhysicalConnectorType = 9;
pub const KS_PhysConn_Video_Composite: KS_PhysicalConnectorType = 2;
pub const KS_PhysConn_Video_ParallelDigital: KS_PhysicalConnectorType = 7;
pub const KS_PhysConn_Video_RGB: KS_PhysicalConnectorType = 4;
pub const KS_PhysConn_Video_SCART: KS_PhysicalConnectorType = 14;
pub const KS_PhysConn_Video_SCSI: KS_PhysicalConnectorType = 8;
pub const KS_PhysConn_Video_SVideo: KS_PhysicalConnectorType = 3;
pub const KS_PhysConn_Video_SerialDigital: KS_PhysicalConnectorType = 6;
pub const KS_PhysConn_Video_Tuner: KS_PhysicalConnectorType = 1;
pub const KS_PhysConn_Video_USB: KS_PhysicalConnectorType = 11;
pub const KS_PhysConn_Video_VideoDecoder: KS_PhysicalConnectorType = 12;
pub const KS_PhysConn_Video_VideoEncoder: KS_PhysicalConnectorType = 13;
pub const KS_PhysConn_Video_YRYBY: KS_PhysicalConnectorType = 5;
pub type KS_PhysicalConnectorType = i32;
pub const KS_PixAspectRatio_NTSC16x9: KS_AMPixAspectRatio = 1;
pub const KS_PixAspectRatio_NTSC4x3: KS_AMPixAspectRatio = 0;
pub const KS_PixAspectRatio_PAL16x9: KS_AMPixAspectRatio = 3;
pub const KS_PixAspectRatio_PAL4x3: KS_AMPixAspectRatio = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_SECURE_CAMERA_SCENARIO_ID(pub u8);
pub const KS_SIZE_PREHEADER: u32 = 48;
pub const KS_StreamingHint_CompQuality: KS_VideoStreamingHints = 2048;
pub const KS_StreamingHint_CompWindowSize: KS_VideoStreamingHints = 4096;
pub const KS_StreamingHint_FrameInterval: KS_VideoStreamingHints = 256;
pub const KS_StreamingHint_KeyFrameRate: KS_VideoStreamingHints = 512;
pub const KS_StreamingHint_PFrameRate: KS_VideoStreamingHints = 1024;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_TRUECOLORINFO {
    pub dwBitMasks: [u32; 3],
    pub bmiColors: [KS_RGBQUAD; 256],
}
impl Default for KS_TRUECOLORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KS_TUNER_STRATEGY = i32;
pub const KS_TUNER_STRATEGY_DRIVER_TUNES: KS_TUNER_STRATEGY = 4;
pub const KS_TUNER_STRATEGY_PLL: KS_TUNER_STRATEGY = 1;
pub const KS_TUNER_STRATEGY_SIGNAL_STRENGTH: KS_TUNER_STRATEGY = 2;
pub const KS_TUNER_TUNING_COARSE: KS_TUNER_TUNING_FLAGS = 3;
pub const KS_TUNER_TUNING_EXACT: KS_TUNER_TUNING_FLAGS = 1;
pub const KS_TUNER_TUNING_FINE: KS_TUNER_TUNING_FLAGS = 2;
pub type KS_TUNER_TUNING_FLAGS = i32;
pub const KS_TVAUDIO_MODE_LANG_A: u32 = 16;
pub const KS_TVAUDIO_MODE_LANG_B: u32 = 32;
pub const KS_TVAUDIO_MODE_LANG_C: u32 = 64;
pub const KS_TVAUDIO_MODE_MONO: u32 = 1;
pub const KS_TVAUDIO_MODE_STEREO: u32 = 2;
pub const KS_TVAUDIO_PRESET_LANG_A: u32 = 4096;
pub const KS_TVAUDIO_PRESET_LANG_B: u32 = 8192;
pub const KS_TVAUDIO_PRESET_LANG_C: u32 = 16384;
pub const KS_TVAUDIO_PRESET_STEREO: u32 = 512;
pub const KS_TVTUNER_CHANGE_BEGIN_TUNE: u32 = 1;
pub const KS_TVTUNER_CHANGE_END_TUNE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_TVTUNER_CHANGE_INFO {
    pub dwFlags: u32,
    pub dwCountryCode: u32,
    pub dwAnalogVideoStandard: u32,
    pub dwChannel: u32,
}
pub const KS_VBICAP_PROTECTION_MV_DETECTED: u32 = 4;
pub const KS_VBICAP_PROTECTION_MV_HARDWARE: u32 = 2;
pub const KS_VBICAP_PROTECTION_MV_PRESENT: u32 = 1;
pub const KS_VBIDATARATE_CC: u32 = 503493;
pub const KS_VBIDATARATE_NABTS: u32 = 5727272;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const KS_VBISAMPLINGRATE_47X_NABTS: u32 = 27000000;
pub const KS_VBISAMPLINGRATE_4X_NABTS: u32 = 22909088;
pub const KS_VBISAMPLINGRATE_5X_NABTS: u32 = 28636360;
pub const KS_VBI_FLAG_FIELD1: u32 = 1;
pub const KS_VBI_FLAG_FIELD2: u32 = 2;
pub const KS_VBI_FLAG_MV_DETECTED: u32 = 1024;
pub const KS_VBI_FLAG_MV_HARDWARE: u32 = 512;
pub const KS_VBI_FLAG_MV_PRESENT: u32 = 256;
pub const KS_VBI_FLAG_TVTUNER_CHANGE: u32 = 16;
pub const KS_VBI_FLAG_VBIINFOHEADER_CHANGE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KS_VBI_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub dwSamplingFrequency: u32,
    pub TvTunerChangeInfo: KS_TVTUNER_CHANGE_INFO,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
pub type KS_VIDEODECODER_FLAGS = i32;
pub const KS_VIDEODECODER_FLAGS_CAN_DISABLE_OUTPUT: KS_VIDEODECODER_FLAGS = 1;
pub const KS_VIDEODECODER_FLAGS_CAN_INDICATE_LOCKED: KS_VIDEODECODER_FLAGS = 4;
pub const KS_VIDEODECODER_FLAGS_CAN_USE_VCR_LOCKING: KS_VIDEODECODER_FLAGS = 2;
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_VIDEOINFO {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::mediaobj::REFERENCE_TIME,
    pub bmiHeader: KS_BITMAPINFOHEADER,
    pub Anonymous: KS_VIDEOINFO_0,
}
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for KS_VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub union KS_VIDEOINFO_0 {
    pub bmiColors: [KS_RGBQUAD; 256],
    pub dwBitMasks: [u32; 3],
    pub TrueColorInfo: KS_TRUECOLORINFO,
}
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for KS_VIDEOINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct KS_VIDEOINFOHEADER {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::mediaobj::REFERENCE_TIME,
    pub bmiHeader: KS_BITMAPINFOHEADER,
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_VIDEOINFOHEADER2 {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::mediaobj::REFERENCE_TIME,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub Anonymous: KS_VIDEOINFOHEADER2_0,
    pub dwReserved2: u32,
    pub bmiHeader: KS_BITMAPINFOHEADER,
}
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for KS_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub union KS_VIDEOINFOHEADER2_0 {
    pub dwControlFlags: u32,
    pub dwReserved1: u32,
}
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for KS_VIDEOINFOHEADER2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_VIDEOSTREAM_CAPTURE: u32 = 2;
pub const KS_VIDEOSTREAM_CC: u32 = 256;
pub const KS_VIDEOSTREAM_EDS: u32 = 512;
pub const KS_VIDEOSTREAM_IS_VPE: u32 = 32768;
pub const KS_VIDEOSTREAM_NABTS: u32 = 32;
pub const KS_VIDEOSTREAM_PREVIEW: u32 = 1;
pub const KS_VIDEOSTREAM_STILL: u32 = 4096;
pub const KS_VIDEOSTREAM_TELETEXT: u32 = 1024;
pub const KS_VIDEOSTREAM_VBI: u32 = 16;
pub const KS_VIDEO_ALLOC_VPE_AGP: u32 = 4;
pub const KS_VIDEO_ALLOC_VPE_DISPLAY: u32 = 2;
pub const KS_VIDEO_ALLOC_VPE_SYSTEM: u32 = 1;
pub const KS_VIDEO_FLAG_B_FRAME: u32 = 32;
pub const KS_VIDEO_FLAG_FIELD1: u32 = 1;
pub const KS_VIDEO_FLAG_FIELD1FIRST: u32 = 4;
pub const KS_VIDEO_FLAG_FIELD2: u32 = 2;
pub const KS_VIDEO_FLAG_FIELD_MASK: u32 = 3;
pub const KS_VIDEO_FLAG_FRAME: u32 = 0;
pub const KS_VIDEO_FLAG_IPB_MASK: u32 = 48;
pub const KS_VIDEO_FLAG_I_FRAME: u32 = 0;
pub const KS_VIDEO_FLAG_P_FRAME: u32 = 16;
pub const KS_VIDEO_FLAG_REPEAT_FIELD: u32 = 64;
pub const KS_VIDEO_FLAG_WEAVE: u32 = 8;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct KS_VIDEO_STREAM_CONFIG_CAPS {
    pub guid: windows_sys::core::GUID,
    pub VideoStandard: u32,
    pub InputSize: super::windef::SIZE,
    pub MinCroppingSize: super::windef::SIZE,
    pub MaxCroppingSize: super::windef::SIZE,
    pub CropGranularityX: i32,
    pub CropGranularityY: i32,
    pub CropAlignX: i32,
    pub CropAlignY: i32,
    pub MinOutputSize: super::windef::SIZE,
    pub MaxOutputSize: super::windef::SIZE,
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
pub const KS_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = 4;
pub const KS_VideoControlFlag_FlipHorizontal: KS_VideoControlFlags = 1;
pub const KS_VideoControlFlag_FlipVertical: KS_VideoControlFlags = 2;
pub const KS_VideoControlFlag_IndependentImagePin: KS_VideoControlFlags = 64;
pub const KS_VideoControlFlag_StartPhotoSequenceCapture: KS_VideoControlFlags = 256;
pub const KS_VideoControlFlag_StillCapturePreviewFrame: KS_VideoControlFlags = 128;
pub const KS_VideoControlFlag_StopPhotoSequenceCapture: KS_VideoControlFlags = 512;
pub const KS_VideoControlFlag_Trigger: KS_VideoControlFlags = 8;
pub type KS_VideoControlFlags = i32;
pub type KS_VideoStreamingHints = i32;
pub const KS_iBLUE: u32 = 2;
pub const KS_iEGA_COLORS: u32 = 16;
pub const KS_iGREEN: u32 = 1;
pub const KS_iMASK_COLORS: u32 = 3;
pub const KS_iMAXBITS: u32 = 8;
pub const KS_iPALETTE: u32 = 8;
pub const KS_iPALETTE_COLORS: u32 = 256;
pub const KS_iRED: u32 = 0;
pub const KS_iTRUECOLOR: u32 = 16;
pub const LIGHT_FILTER: KSDS3D_HRTF_FILTER_QUALITY = 1;
#[repr(C)]
#[cfg(all(feature = "ks", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LOOPEDSTREAMING_POSITION_EVENT_DATA {
    pub KsEventData: super::ks::KSEVENTDATA,
    pub Position: super::winnt::DWORDLONG,
}
#[cfg(all(feature = "ks", feature = "winnt"))]
impl Default for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPDDPIXELFORMAT = *mut DDPIXELFORMAT;
pub type LPDDVIDEOPORTCONNECT = *mut DDVIDEOPORTCONNECT;
pub const MAX_NABTS_VBI_LINES_PER_FIELD: u32 = 11;
pub const MAX_RESOURCEGROUPID_LENGTH: u32 = 256;
pub const MAX_SINK_DESCRIPTION_NAME_LENGTH: u32 = 32;
pub const MAX_WST_VBI_LINES_PER_FIELD: u32 = 17;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MEDIUM_INFO {
    pub MediaPresent: windows_sys::core::BOOL,
    pub MediaType: u32,
    pub RecordInhibit: windows_sys::core::BOOL,
}
pub const MetadataId_BackgroundSegmentationMask: KSCAMERA_MetadataId = 8;
pub const MetadataId_CameraExtrinsics: KSCAMERA_MetadataId = 4;
pub const MetadataId_CameraIntrinsics: KSCAMERA_MetadataId = 5;
pub const MetadataId_CaptureStats: KSCAMERA_MetadataId = 3;
pub const MetadataId_Custom_Start: KSCAMERA_MetadataId = -2147483648;
pub const MetadataId_DigitalWindow: KSCAMERA_MetadataId = 7;
pub const MetadataId_FrameIllumination: KSCAMERA_MetadataId = 6;
pub const MetadataId_PhotoConfirmation: KSCAMERA_MetadataId = 1;
pub const MetadataId_Standard_End: KSCAMERA_MetadataId = 8;
pub const MetadataId_Standard_Start: KSCAMERA_MetadataId = 1;
pub const MetadataId_UsbVideoHeader: KSCAMERA_MetadataId = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NABTSFEC_BUFFER {
    pub dataSize: u32,
    pub groupID: u16,
    pub Reserved: u16,
    pub data: [u8; 448],
}
impl Default for NABTSFEC_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NABTS_BUFFER {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub PictureNumber: i64,
    pub NabtsLines: [NABTS_BUFFER_LINE; 11],
}
impl Default for NABTS_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NABTS_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 36],
}
impl Default for NABTS_BUFFER_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NABTS_BUFFER_PICTURENUMBER_SUPPORT: u32 = 1;
pub const NABTS_BYTES_PER_LINE: u32 = 36;
pub const NABTS_LINES_PER_BUNDLE: u32 = 16;
pub const NABTS_PAYLOAD_PER_LINE: u32 = 28;
pub type PAUDIORESOURCEMANAGEMENT_RESOURCEGROUP = *mut AUDIORESOURCEMANAGEMENT_RESOURCEGROUP;
pub type PCAPTURE_MEMORY_ALLOCATION_FLAGS = *mut CAPTURE_MEMORY_ALLOCATION_FLAGS;
pub type PCC_BYTE_PAIR = *mut CC_BYTE_PAIR;
pub type PCC_HW_FIELD = *mut CC_HW_FIELD;
pub type PDEVCAPS = *mut DEVCAPS;
pub type PDS3DVECTOR = *mut DS3DVECTOR;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_DISPLAYPORT_OUT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_HDMI_OUT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_IMAGE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_SPDIF_IN(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_SPDIF_OUT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_ANALOGVIDEOIN(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_CAPTURE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_CC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_CC_CAPTURE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_EDS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_NABTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_NABTS_CAPTURE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_PREVIEW(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_STILL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_TELETEXT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_TIMECODE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_VBI(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_VIDEOPORT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PINNAME_VIDEO_VIDEOPORT_VBI(pub u8);
pub type PKSAC3_ALTERNATE_AUDIO = *mut KSAC3_ALTERNATE_AUDIO;
pub type PKSAC3_BIT_STREAM_MODE = *mut KSAC3_BIT_STREAM_MODE;
pub type PKSAC3_DIALOGUE_LEVEL = *mut KSAC3_DIALOGUE_LEVEL;
pub type PKSAC3_DOWNMIX = *mut KSAC3_DOWNMIX;
pub type PKSAC3_ERROR_CONCEALMENT = *mut KSAC3_ERROR_CONCEALMENT;
pub type PKSAC3_ROOM_TYPE = *mut KSAC3_ROOM_TYPE;
#[cfg(feature = "ks")]
pub type PKSATTRIBUTE_AUDIOLOOPBACK_TAPPOINT = *mut KSATTRIBUTE_AUDIOLOOPBACK_TAPPOINT;
#[cfg(feature = "ks")]
pub type PKSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE = *mut KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE;
pub type PKSAUDIOENGINE_BUFFER_SIZE_RANGE = *mut KSAUDIOENGINE_BUFFER_SIZE_RANGE;
pub type PKSAUDIOENGINE_DESCRIPTOR = *mut KSAUDIOENGINE_DESCRIPTOR;
pub type PKSAUDIOENGINE_DEVICECONTROLS = *mut KSAUDIOENGINE_DEVICECONTROLS;
pub type PKSAUDIOENGINE_VOLUMELEVEL = *mut KSAUDIOENGINE_VOLUMELEVEL;
pub type PKSAUDIOMODULE_DESCRIPTOR = *mut KSAUDIOMODULE_DESCRIPTOR;
pub type PKSAUDIOMODULE_NOTIFICATION = *mut KSAUDIOMODULE_NOTIFICATION;
#[cfg(feature = "ks")]
pub type PKSAUDIOMODULE_PROPERTY = *mut KSAUDIOMODULE_PROPERTY;
pub type PKSAUDIO_CHANNEL_CONFIG = *mut KSAUDIO_CHANNEL_CONFIG;
pub type PKSAUDIO_COPY_PROTECTION = *mut KSAUDIO_COPY_PROTECTION;
pub type PKSAUDIO_DYNAMIC_RANGE = *mut KSAUDIO_DYNAMIC_RANGE;
pub type PKSAUDIO_MICROPHONE_COORDINATES = *mut KSAUDIO_MICROPHONE_COORDINATES;
pub type PKSAUDIO_MIC_ARRAY_GEOMETRY = *mut KSAUDIO_MIC_ARRAY_GEOMETRY;
pub type PKSAUDIO_MIXCAP_TABLE = *mut KSAUDIO_MIXCAP_TABLE;
pub type PKSAUDIO_MIXLEVEL = *mut KSAUDIO_MIXLEVEL;
pub type PKSAUDIO_MIX_CAPS = *mut KSAUDIO_MIX_CAPS;
#[cfg(feature = "winnt")]
pub type PKSAUDIO_POSITION = *mut KSAUDIO_POSITION;
#[cfg(feature = "winnt")]
pub type PKSAUDIO_POSITIONEX = *mut KSAUDIO_POSITIONEX;
pub type PKSAUDIO_PRESENTATION_POSITION = *mut KSAUDIO_PRESENTATION_POSITION;
#[cfg(feature = "windef")]
pub type PKSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS = *mut KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS;
pub type PKSCAMERA_EXTENDEDPROP_CAMERAOFFSET = *mut KSCAMERA_EXTENDEDPROP_CAMERAOFFSET;
pub type PKSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS = *mut KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS;
pub type PKSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER = *mut KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER;
pub type PKSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING = *mut KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING;
pub type PKSCAMERA_EXTENDEDPROP_EVCOMPENSATION = *mut KSCAMERA_EXTENDEDPROP_EVCOMPENSATION;
pub type PKSCAMERA_EXTENDEDPROP_FIELDOFVIEW = *mut KSCAMERA_EXTENDEDPROP_FIELDOFVIEW;
pub type PKSCAMERA_EXTENDEDPROP_FIELDOFVIEW2_CONFIGCAPS = *mut KSCAMERA_EXTENDEDPROP_FIELDOFVIEW2_CONFIGCAPS;
pub type PKSCAMERA_EXTENDEDPROP_HEADER = *mut KSCAMERA_EXTENDEDPROP_HEADER;
pub type PKSCAMERA_EXTENDEDPROP_METADATAINFO = *mut KSCAMERA_EXTENDEDPROP_METADATAINFO;
pub type PKSCAMERA_EXTENDEDPROP_PHOTOMODE = *mut KSCAMERA_EXTENDEDPROP_PHOTOMODE;
pub type PKSCAMERA_EXTENDEDPROP_PROFILE = *mut KSCAMERA_EXTENDEDPROP_PROFILE;
pub type PKSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS = *mut KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS;
pub type PKSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER = *mut KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER;
#[cfg(feature = "windef")]
pub type PKSCAMERA_EXTENDEDPROP_ROI_EXPOSURE = *mut KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE;
#[cfg(feature = "windef")]
pub type PKSCAMERA_EXTENDEDPROP_ROI_FOCUS = *mut KSCAMERA_EXTENDEDPROP_ROI_FOCUS;
#[cfg(feature = "windef")]
pub type PKSCAMERA_EXTENDEDPROP_ROI_INFO = *mut KSCAMERA_EXTENDEDPROP_ROI_INFO;
pub type PKSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL = *mut KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL;
pub type PKSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER = *mut KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER;
#[cfg(feature = "windef")]
pub type PKSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE = *mut KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE;
pub type PKSCAMERA_EXTENDEDPROP_VALUE = *mut KSCAMERA_EXTENDEDPROP_VALUE;
pub type PKSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING = *mut KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING;
pub type PKSCAMERA_MAXVIDEOFPS_FORPHOTORES = *mut KSCAMERA_MAXVIDEOFPS_FORPHOTORES;
#[cfg(feature = "windef")]
pub type PKSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK = *mut KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK;
pub type PKSCAMERA_METADATA_CAPTURESTATS = *mut KSCAMERA_METADATA_CAPTURESTATS;
pub type PKSCAMERA_METADATA_DIGITALWINDOW = *mut KSCAMERA_METADATA_DIGITALWINDOW;
pub type PKSCAMERA_METADATA_FRAMEILLUMINATION = *mut KSCAMERA_METADATA_FRAMEILLUMINATION;
pub type PKSCAMERA_METADATA_ITEMHEADER = *mut KSCAMERA_METADATA_ITEMHEADER;
pub type PKSCAMERA_METADATA_PHOTOCONFIRMATION = *mut KSCAMERA_METADATA_PHOTOCONFIRMATION;
pub type PKSCAMERA_PERFRAMESETTING_CAP_HEADER = *mut KSCAMERA_PERFRAMESETTING_CAP_HEADER;
pub type PKSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER = *mut KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER;
pub type PKSCAMERA_PERFRAMESETTING_CUSTOM_ITEM = *mut KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM;
pub type PKSCAMERA_PERFRAMESETTING_FRAME_HEADER = *mut KSCAMERA_PERFRAMESETTING_FRAME_HEADER;
pub type PKSCAMERA_PERFRAMESETTING_HEADER = *mut KSCAMERA_PERFRAMESETTING_HEADER;
pub type PKSCAMERA_PERFRAMESETTING_ITEM_HEADER = *mut KSCAMERA_PERFRAMESETTING_ITEM_HEADER;
pub type PKSCAMERA_PROFILE_CONCURRENCYINFO = *mut KSCAMERA_PROFILE_CONCURRENCYINFO;
pub type PKSCAMERA_PROFILE_INFO = *mut KSCAMERA_PROFILE_INFO;
pub type PKSCAMERA_PROFILE_MEDIAINFO = *mut KSCAMERA_PROFILE_MEDIAINFO;
pub type PKSCAMERA_PROFILE_PININFO = *mut KSCAMERA_PROFILE_PININFO;
#[cfg(all(feature = "ks", feature = "mmeapi"))]
pub type PKSDATAFORMAT_DSOUND = *mut KSDATAFORMAT_DSOUND;
#[cfg(all(feature = "ks", feature = "mmeapi"))]
pub type PKSDATAFORMAT_WAVEFORMATEX = *mut KSDATAFORMAT_WAVEFORMATEX;
#[cfg(all(feature = "ks", feature = "mmeapi", feature = "mmreg"))]
pub type PKSDATAFORMAT_WAVEFORMATEXTENSIBLE = *mut KSDATAFORMAT_WAVEFORMATEXTENSIBLE;
#[cfg(feature = "ks")]
pub type PKSDATARANGE_AUDIO = *mut KSDATARANGE_AUDIO;
#[cfg(feature = "ks")]
pub type PKSDATARANGE_MUSIC = *mut KSDATARANGE_MUSIC;
pub type PKSDEVICE_PROFILE_INFO = *mut KSDEVICE_PROFILE_INFO;
pub type PKSDISPLAYCHANGE = *mut KSDISPLAYCHANGE;
pub type PKSDS3D_BUFFER_ALL = *mut KSDS3D_BUFFER_ALL;
pub type PKSDS3D_BUFFER_CONE_ANGLES = *mut KSDS3D_BUFFER_CONE_ANGLES;
pub type PKSDS3D_HRTF_FILTER_FORMAT_MSG = *mut KSDS3D_HRTF_FILTER_FORMAT_MSG;
pub type PKSDS3D_HRTF_INIT_MSG = *mut KSDS3D_HRTF_INIT_MSG;
pub type PKSDS3D_HRTF_PARAMS_MSG = *mut KSDS3D_HRTF_PARAMS_MSG;
pub type PKSDS3D_ITD_PARAMS = *mut KSDS3D_ITD_PARAMS;
pub type PKSDS3D_ITD_PARAMS_MSG = *mut KSDS3D_ITD_PARAMS_MSG;
pub type PKSDS3D_LISTENER_ALL = *mut KSDS3D_LISTENER_ALL;
pub type PKSDS3D_LISTENER_ORIENTATION = *mut KSDS3D_LISTENER_ORIENTATION;
#[cfg(feature = "mmeapi")]
pub type PKSDSOUND_BUFFERDESC = *mut KSDSOUND_BUFFERDESC;
#[cfg(all(feature = "ks", feature = "winnt"))]
pub type PKSEVENT_TUNER_INITIATE_SCAN_S = *mut KSEVENT_TUNER_INITIATE_SCAN_S;
pub type PKSGOP_USERDATA = *mut KSGOP_USERDATA;
pub type PKSJACK_DESCRIPTION = *mut KSJACK_DESCRIPTION;
pub type PKSJACK_DESCRIPTION2 = *mut KSJACK_DESCRIPTION2;
pub type PKSJACK_DESCRIPTION3 = *mut KSJACK_DESCRIPTION3;
#[cfg(feature = "winnt")]
pub type PKSJACK_SINK_INFORMATION = *mut KSJACK_SINK_INFORMATION;
pub type PKSMIDILOOPED_BUFFER = *mut KSMIDILOOPED_BUFFER;
#[cfg(feature = "ks")]
pub type PKSMIDILOOPED_BUFFER_PROPERTY = *mut KSMIDILOOPED_BUFFER_PROPERTY;
#[cfg(feature = "winnt")]
pub type PKSMIDILOOPED_EVENT = *mut KSMIDILOOPED_EVENT;
#[cfg(feature = "winnt")]
pub type PKSMIDILOOPED_EVENT2 = *mut KSMIDILOOPED_EVENT2;
pub type PKSMIDILOOPED_REGISTERS = *mut KSMIDILOOPED_REGISTERS;
pub type PKSMPEGVID_RECT = *mut KSMPEGVID_RECT;
#[cfg(feature = "ks")]
pub type PKSMULTIPLE_DATA_PROP = *mut KSMULTIPLE_DATA_PROP;
pub type PKSMUSICFORMAT = *mut KSMUSICFORMAT;
#[cfg(feature = "ks")]
pub type PKSNODEPROPERTY = *mut KSNODEPROPERTY;
#[cfg(feature = "ks")]
pub type PKSNODEPROPERTY_AUDIO_3D_LISTENER = *mut KSNODEPROPERTY_AUDIO_3D_LISTENER;
#[cfg(feature = "ks")]
pub type PKSNODEPROPERTY_AUDIO_CHANNEL = *mut KSNODEPROPERTY_AUDIO_CHANNEL;
#[cfg(feature = "ks")]
pub type PKSNODEPROPERTY_AUDIO_DEV_SPECIFIC = *mut KSNODEPROPERTY_AUDIO_DEV_SPECIFIC;
#[cfg(feature = "ks")]
pub type PKSNODEPROPERTY_AUDIO_PROPERTY = *mut KSNODEPROPERTY_AUDIO_PROPERTY;
pub type PKSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S = *mut KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S;
pub type PKSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S = *mut KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S;
pub type PKSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S = *mut KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S;
pub type PKSPROPERTY_CAMERACONTROL_FLASH_S = *mut KSPROPERTY_CAMERACONTROL_FLASH_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S = *mut KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S;
pub type PKSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S = *mut KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S = *mut KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct PKSPROPERTY_CAMERACONTROL_NODE_S {
    pub NodeProperty: super::ks::KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
#[cfg(feature = "ks")]
impl Default for PKSPROPERTY_CAMERACONTROL_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CAMERACONTROL_NODE_S2 = *mut KSPROPERTY_CAMERACONTROL_NODE_S2;
#[cfg(feature = "windef")]
pub type PKSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S = *mut KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CAMERACONTROL_S = *mut KSPROPERTY_CAMERACONTROL_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CAMERACONTROL_S2 = *mut KSPROPERTY_CAMERACONTROL_S2;
#[cfg(all(feature = "ks", feature = "windef"))]
pub type PKSPROPERTY_CAMERACONTROL_S_EX = *mut KSPROPERTY_CAMERACONTROL_S_EX;
pub type PKSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S = *mut KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S;
pub type PKSPROPERTY_COMPOSIT_ON = *mut windows_sys::core::BOOL;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CROSSBAR_ACTIVE_S = *mut KSPROPERTY_CROSSBAR_ACTIVE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CROSSBAR_CAPS_S = *mut KSPROPERTY_CROSSBAR_CAPS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CROSSBAR_PININFO_S = *mut KSPROPERTY_CROSSBAR_PININFO_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_CROSSBAR_ROUTE_S = *mut KSPROPERTY_CROSSBAR_ROUTE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_DROPPEDFRAMES_CURRENT_S = *mut KSPROPERTY_DROPPEDFRAMES_CURRENT_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_EXTDEVICE_S = *mut KSPROPERTY_EXTDEVICE_S;
pub type PKSPROPERTY_EXTENSION_UNIT = *mut KSPROPERTY_EXTENSION_UNIT;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_EXTXPORT_NODE_S = *mut KSPROPERTY_EXTXPORT_NODE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_EXTXPORT_S = *mut KSPROPERTY_EXTXPORT_S;
pub type PKSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO = *mut KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO;
pub type PKSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO = *mut KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO;
pub type PKSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER = *mut KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_SELECTOR_NODE_S = *mut KSPROPERTY_SELECTOR_NODE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_SELECTOR_S = *mut KSPROPERTY_SELECTOR_S;
pub type PKSPROPERTY_SPHLI = *mut KSPROPERTY_SPHLI;
pub type PKSPROPERTY_SPPAL = *mut KSPROPERTY_SPPAL;
#[cfg(all(feature = "ks", feature = "winnt"))]
pub type PKSPROPERTY_TIMECODE_NODE_S = *mut KSPROPERTY_TIMECODE_NODE_S;
#[cfg(all(feature = "ks", feature = "winnt"))]
pub type PKSPROPERTY_TIMECODE_S = *mut KSPROPERTY_TIMECODE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_CAPS_S = *mut KSPROPERTY_TUNER_CAPS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_FREQUENCY_S = *mut KSPROPERTY_TUNER_FREQUENCY_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_IF_MEDIUM_S = *mut KSPROPERTY_TUNER_IF_MEDIUM_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_INPUT_S = *mut KSPROPERTY_TUNER_INPUT_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_MODE_CAPS_S = *mut KSPROPERTY_TUNER_MODE_CAPS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_MODE_S = *mut KSPROPERTY_TUNER_MODE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S = *mut KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_SCAN_CAPS_S = *mut KSPROPERTY_TUNER_SCAN_CAPS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_SCAN_STATUS_S = *mut KSPROPERTY_TUNER_SCAN_STATUS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_STANDARD_MODE_S = *mut KSPROPERTY_TUNER_STANDARD_MODE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_STANDARD_S = *mut KSPROPERTY_TUNER_STANDARD_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TUNER_STATUS_S = *mut KSPROPERTY_TUNER_STATUS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TVAUDIO_CAPS_S = *mut KSPROPERTY_TVAUDIO_CAPS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_TVAUDIO_S = *mut KSPROPERTY_TVAUDIO_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S = *mut KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S = *mut KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_SCANLINES_S = *mut KSPROPERTY_VBICODECFILTERING_SCANLINES_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S = *mut KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S = *mut KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S = *mut KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S = *mut KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S = *mut KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S = *mut KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S;
pub type PKSPROPERTY_VIDCAP_SELECTOR = *mut KSPROPERTY_VIDCAP_SELECTOR;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOCOMPRESSION_GETINFO_S = *mut KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOCOMPRESSION_S = *mut KSPROPERTY_VIDEOCOMPRESSION_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOCOMPRESSION_S1 = *mut KSPROPERTY_VIDEOCOMPRESSION_S1;
#[cfg(all(feature = "ks", feature = "windef"))]
pub type PKSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S = *mut KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOCONTROL_CAPS_S = *mut KSPROPERTY_VIDEOCONTROL_CAPS_S;
#[cfg(all(feature = "ks", feature = "windef"))]
pub type PKSPROPERTY_VIDEOCONTROL_FRAME_RATES_S = *mut KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOCONTROL_MODE_S = *mut KSPROPERTY_VIDEOCONTROL_MODE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEODECODER_CAPS_S = *mut KSPROPERTY_VIDEODECODER_CAPS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEODECODER_S = *mut KSPROPERTY_VIDEODECODER_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEODECODER_STATUS2_S = *mut KSPROPERTY_VIDEODECODER_STATUS2_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEODECODER_STATUS_S = *mut KSPROPERTY_VIDEODECODER_STATUS_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOENCODER_S = *mut KSPROPERTY_VIDEOENCODER_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOPROCAMP_NODE_S = *mut KSPROPERTY_VIDEOPROCAMP_NODE_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOPROCAMP_NODE_S2 = *mut KSPROPERTY_VIDEOPROCAMP_NODE_S2;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOPROCAMP_S = *mut KSPROPERTY_VIDEOPROCAMP_S;
#[cfg(feature = "ks")]
pub type PKSPROPERTY_VIDEOPROCAMP_S2 = *mut KSPROPERTY_VIDEOPROCAMP_S2;
pub type PKSRTAUDIO_BUFFER = *mut KSRTAUDIO_BUFFER;
pub type PKSRTAUDIO_BUFFER32 = *mut KSRTAUDIO_BUFFER32;
#[cfg(feature = "ks")]
pub type PKSRTAUDIO_BUFFER_PROPERTY = *mut KSRTAUDIO_BUFFER_PROPERTY;
#[cfg(feature = "ks")]
pub type PKSRTAUDIO_BUFFER_PROPERTY32 = *mut KSRTAUDIO_BUFFER_PROPERTY32;
#[cfg(feature = "ks")]
pub type PKSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION = *mut KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION;
#[cfg(feature = "ks")]
pub type PKSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 = *mut KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32;
pub type PKSRTAUDIO_GETREADPACKET_INFO = *mut KSRTAUDIO_GETREADPACKET_INFO;
pub type PKSRTAUDIO_HWLATENCY = *mut KSRTAUDIO_HWLATENCY;
pub type PKSRTAUDIO_HWREGISTER = *mut KSRTAUDIO_HWREGISTER;
pub type PKSRTAUDIO_HWREGISTER32 = *mut KSRTAUDIO_HWREGISTER32;
#[cfg(feature = "ks")]
pub type PKSRTAUDIO_HWREGISTER_PROPERTY = *mut KSRTAUDIO_HWREGISTER_PROPERTY;
#[cfg(feature = "ks")]
pub type PKSRTAUDIO_HWREGISTER_PROPERTY32 = *mut KSRTAUDIO_HWREGISTER_PROPERTY32;
#[cfg(all(feature = "ks", feature = "winnt"))]
pub type PKSRTAUDIO_NOTIFICATION_EVENT_PROPERTY = *mut KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY;
#[cfg(feature = "ks")]
pub type PKSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 = *mut KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32;
#[cfg(feature = "basetsd")]
pub type PKSRTAUDIO_PACKETVREGISTER = *mut KSRTAUDIO_PACKETVREGISTER;
#[cfg(feature = "ks")]
pub type PKSRTAUDIO_PACKETVREGISTER_PROPERTY = *mut KSRTAUDIO_PACKETVREGISTER_PROPERTY;
pub type PKSRTAUDIO_SETWRITEPACKET_INFO = *mut KSRTAUDIO_SETWRITEPACKET_INFO;
#[cfg(feature = "ks")]
pub type PKSSOUNDDETECTORPROPERTY = *mut KSSOUNDDETECTORPROPERTY;
pub type PKSTELEPHONY_CALLCONTROL = *mut KSTELEPHONY_CALLCONTROL;
pub type PKSTELEPHONY_CALLINFO = *mut KSTELEPHONY_CALLINFO;
pub type PKSTELEPHONY_PROVIDERCHANGE = *mut KSTELEPHONY_PROVIDERCHANGE;
pub type PKSTOPOLOGY_ENDPOINTID = *mut KSTOPOLOGY_ENDPOINTID;
pub type PKSTOPOLOGY_ENDPOINTIDPAIR = *mut KSTOPOLOGY_ENDPOINTIDPAIR;
pub type PKSVPMAXPIXELRATE = *mut KSVPMAXPIXELRATE;
#[cfg(feature = "ks")]
pub type PKSVPSIZE_PROP = *mut KSVPSIZE_PROP;
pub type PKSVPSURFACEPARAMS = *mut KSVPSURFACEPARAMS;
#[cfg(feature = "ks")]
pub type PKSWAVETABLE_WAVE_DESC = *mut KSWAVETABLE_WAVE_DESC;
pub type PKSWAVE_BUFFER = *mut KSWAVE_BUFFER;
pub type PKSWAVE_COMPATCAPS = *mut KSWAVE_COMPATCAPS;
pub type PKSWAVE_INPUT_CAPABILITIES = *mut KSWAVE_INPUT_CAPABILITIES;
pub type PKSWAVE_OUTPUT_CAPABILITIES = *mut KSWAVE_OUTPUT_CAPABILITIES;
pub type PKSWAVE_VOLUME = *mut KSWAVE_VOLUME;
#[cfg(feature = "windef")]
pub type PKS_AMVPDATAINFO = *mut KS_AMVPDATAINFO;
#[cfg(feature = "windef")]
pub type PKS_AMVPDIMINFO = *mut KS_AMVPDIMINFO;
pub type PKS_AMVPSIZE = *mut KS_AMVPSIZE;
#[cfg(feature = "mediaobj")]
pub type PKS_AM_ExactRateChange = *mut KS_AM_ExactRateChange;
#[cfg(feature = "mediaobj")]
pub type PKS_AM_SimpleRateChange = *mut KS_AM_SimpleRateChange;
#[cfg(all(feature = "mediaobj", feature = "windef"))]
pub type PKS_ANALOGVIDEOINFO = *mut KS_ANALOGVIDEOINFO;
pub type PKS_BITMAPINFOHEADER = *mut KS_BITMAPINFOHEADER;
pub type PKS_COLCON = *mut KS_COLCON;
pub type PKS_COPY_MACROVISION = *mut KS_COPY_MACROVISION;
pub type PKS_COPY_MACROVISION_LEVEL = *mut KS_COPY_MACROVISION_LEVEL;
#[cfg(feature = "ks")]
pub type PKS_DATAFORMAT_H264VIDEOINFO = *mut KS_DATAFORMAT_H264VIDEOINFO;
#[cfg(feature = "ks")]
pub type PKS_DATAFORMAT_IMAGEINFO = *mut KS_DATAFORMAT_IMAGEINFO;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATAFORMAT_MPEGVIDEOINFO2 = *mut KS_DATAFORMAT_MPEGVIDEOINFO2;
#[cfg(feature = "ks")]
pub type PKS_DATAFORMAT_VBIINFOHEADER = *mut KS_DATAFORMAT_VBIINFOHEADER;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATAFORMAT_VIDEOINFOHEADER = *mut KS_DATAFORMAT_VIDEOINFOHEADER;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATAFORMAT_VIDEOINFOHEADER2 = *mut KS_DATAFORMAT_VIDEOINFOHEADER2;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATAFORMAT_VIDEOINFO_PALETTE = *mut KS_DATAFORMAT_VIDEOINFO_PALETTE;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATARANGE_ANALOGVIDEO = *mut KS_DATARANGE_ANALOGVIDEO;
#[cfg(all(feature = "ks", feature = "windef"))]
pub type PKS_DATARANGE_H264_VIDEO = *mut KS_DATARANGE_H264_VIDEO;
#[cfg(all(feature = "ks", feature = "windef"))]
pub type PKS_DATARANGE_IMAGE = *mut KS_DATARANGE_IMAGE;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATARANGE_MPEG1_VIDEO = *mut KS_DATARANGE_MPEG1_VIDEO;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATARANGE_MPEG2_VIDEO = *mut KS_DATARANGE_MPEG2_VIDEO;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATARANGE_VIDEO = *mut KS_DATARANGE_VIDEO;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATARANGE_VIDEO2 = *mut KS_DATARANGE_VIDEO2;
#[cfg(all(feature = "ks", feature = "mediaobj", feature = "windef"))]
pub type PKS_DATARANGE_VIDEO_PALETTE = *mut KS_DATARANGE_VIDEO_PALETTE;
#[cfg(all(feature = "ks", feature = "windef"))]
pub type PKS_DATARANGE_VIDEO_VBI = *mut KS_DATARANGE_VIDEO_VBI;
pub type PKS_DVDCOPY_BUSKEY = *mut KS_DVDCOPY_BUSKEY;
pub type PKS_DVDCOPY_CHLGKEY = *mut KS_DVDCOPY_CHLGKEY;
pub type PKS_DVDCOPY_DISCKEY = *mut KS_DVDCOPY_DISCKEY;
pub type PKS_DVDCOPY_REGION = *mut KS_DVDCOPY_REGION;
pub type PKS_DVDCOPY_SET_COPY_STATE = *mut KS_DVDCOPY_SET_COPY_STATE;
pub type PKS_DVDCOPY_TITLEKEY = *mut KS_DVDCOPY_TITLEKEY;
pub type PKS_DVD_YCrCb = *mut KS_DVD_YCrCb;
pub type PKS_DVD_YUV = *mut KS_DVD_YUV;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PKS_FRAME_INFO = *mut KS_FRAME_INFO;
pub type PKS_H264VIDEOINFO = *mut KS_H264VIDEOINFO;
#[cfg(all(feature = "mediaobj", feature = "windef"))]
pub type PKS_MPEG1VIDEOINFO = *mut KS_MPEG1VIDEOINFO;
pub type PKS_MPEGAUDIOINFO = *mut KS_MPEGAUDIOINFO;
#[cfg(all(feature = "mediaobj", feature = "windef"))]
pub type PKS_MPEGVIDEOINFO2 = *mut KS_MPEGVIDEOINFO2;
pub type PKS_RGBQUAD = *mut KS_RGBQUAD;
pub type PKS_TRUECOLORINFO = *mut KS_TRUECOLORINFO;
pub type PKS_TVTUNER_CHANGE_INFO = *mut KS_TVTUNER_CHANGE_INFO;
pub type PKS_VBIINFOHEADER = *mut KS_VBIINFOHEADER;
pub type PKS_VBI_FRAME_INFO = *mut KS_VBI_FRAME_INFO;
#[cfg(all(feature = "mediaobj", feature = "windef"))]
pub type PKS_VIDEOINFO = *mut KS_VIDEOINFO;
#[cfg(all(feature = "mediaobj", feature = "windef"))]
pub type PKS_VIDEOINFOHEADER = *mut KS_VIDEOINFOHEADER;
#[cfg(all(feature = "mediaobj", feature = "windef"))]
pub type PKS_VIDEOINFOHEADER2 = *mut KS_VIDEOINFOHEADER2;
#[cfg(feature = "windef")]
pub type PKS_VIDEO_STREAM_CONFIG_CAPS = *mut KS_VIDEO_STREAM_CONFIG_CAPS;
#[cfg(all(feature = "ks", feature = "winnt"))]
pub type PLOOPEDSTREAMING_POSITION_EVENT_DATA = *mut LOOPEDSTREAMING_POSITION_EVENT_DATA;
pub type PMEDIUM_INFO = *mut MEDIUM_INFO;
pub type PNABTSFEC_BUFFER = *mut NABTSFEC_BUFFER;
pub type PNABTS_BUFFER = *mut NABTS_BUFFER;
pub type PNABTS_BUFFER_LINE = *mut NABTS_BUFFER_LINE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_ALLOCATOR_CONTROL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_EXT_DEVICE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_EXT_TRANSPORT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_TIMECODE_READER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_TUNER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_CAMERACONTROL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_CAMERACONTROL_FLASH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_CAMERACONTROL_IMAGE_PIN_CAPABILITY(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_CAMERACONTROL_VIDEO_STABILIZATION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_CROSSBAR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_DROPPEDFRAMES(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_SELECTOR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_TVAUDIO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_VIDEOCOMPRESSION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_VIDEOCONTROL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_VIDEODECODER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_VIDEOENCODER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPSETID_VIDCAP_VIDEOPROCAMP(pub u8);
pub type PSECURE_BUFFER_INFO = *mut SECURE_BUFFER_INFO;
#[cfg(feature = "winnt")]
pub type PTIMECODE = *mut TIMECODE;
#[cfg(feature = "winnt")]
pub type PTIMECODE_SAMPLE = *mut TIMECODE_SAMPLE;
pub type PTRANSPORTAUDIOPARMS = *mut TRANSPORTAUDIOPARMS;
#[cfg(feature = "winnt")]
pub type PTRANSPORTBASICPARMS = *mut TRANSPORTBASICPARMS;
pub type PTRANSPORTSTATUS = *mut TRANSPORTSTATUS;
pub type PTRANSPORTVIDEOPARMS = *mut TRANSPORTVIDEOPARMS;
pub type PTRANSPORT_STATE = *mut TRANSPORT_STATE;
pub type PTUNER_ANALOG_CAPS_S = *mut TUNER_ANALOG_CAPS_S;
pub type PUMPDATAFORMAT = *mut UMPDATAFORMAT;
#[cfg(feature = "ks")]
pub type PVBICAP_PROPERTIES_PROTECTION_S = *mut VBICAP_PROPERTIES_PROTECTION_S;
pub type PVBICODECFILTERING_CC_SUBSTREAMS = *mut VBICODECFILTERING_CC_SUBSTREAMS;
pub type PVBICODECFILTERING_NABTS_SUBSTREAMS = *mut VBICODECFILTERING_NABTS_SUBSTREAMS;
pub type PVBICODECFILTERING_SCANLINES = *mut VBICODECFILTERING_SCANLINES;
pub type PVBICODECFILTERING_STATISTICS_CC = *mut VBICODECFILTERING_STATISTICS_CC;
pub type PVBICODECFILTERING_STATISTICS_CC_PIN = *mut VBICODECFILTERING_STATISTICS_CC_PIN;
pub type PVBICODECFILTERING_STATISTICS_COMMON = *mut VBICODECFILTERING_STATISTICS_COMMON;
pub type PVBICODECFILTERING_STATISTICS_COMMON_PIN = *mut VBICODECFILTERING_STATISTICS_COMMON_PIN;
pub type PVBICODECFILTERING_STATISTICS_NABTS = *mut VBICODECFILTERING_STATISTICS_NABTS;
pub type PVBICODECFILTERING_STATISTICS_NABTS_PIN = *mut VBICODECFILTERING_STATISTICS_NABTS_PIN;
pub type PVBICODECFILTERING_STATISTICS_TELETEXT = *mut VBICODECFILTERING_STATISTICS_TELETEXT;
pub type PVBICODECFILTERING_STATISTICS_TELETEXT_PIN = *mut VBICODECFILTERING_STATISTICS_TELETEXT_PIN;
#[cfg(feature = "ks")]
pub type PVIDEOFORMAT_DX12 = *mut VIDEOFORMAT_DX12;
pub type PVRAM_SURFACE_INFO = *mut VRAM_SURFACE_INFO;
#[cfg(feature = "ks")]
pub type PVRAM_SURFACE_INFO_PROPERTY_S = *mut VRAM_SURFACE_INFO_PROPERTY_S;
#[cfg(all(feature = "mmeapi", feature = "mmreg"))]
pub type PWAVEFORMATEXTENSIBLE_IEC61937 = *mut WAVEFORMATEXTENSIBLE_IEC61937;
pub type PWNF_KSCAMERA_STREAMSTATE_INFO = *mut WNF_KSCAMERA_STREAMSTATE_INFO;
pub type PWST_BUFFER = *mut WST_BUFFER;
pub type PWST_BUFFER_LINE = *mut WST_BUFFER_LINE;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURE_BUFFER_INFO {
    pub guidBufferIdentifier: windows_sys::core::GUID,
    pub cbBufferSize: u32,
    pub cbCaptured: u32,
    pub ullReserved: [u64; 16],
}
impl Default for SECURE_BUFFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHORT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SOUNDDETECTOR_PATTERNHEADER {
    pub Size: u32,
    pub PatternType: windows_sys::core::GUID,
}
pub type TELEPHONY_CALLCONTROLOP = i32;
pub const TELEPHONY_CALLCONTROLOP_DISABLE: TELEPHONY_CALLCONTROLOP = 0;
pub const TELEPHONY_CALLCONTROLOP_ENABLE: TELEPHONY_CALLCONTROLOP = 1;
pub type TELEPHONY_CALLSTATE = i32;
pub const TELEPHONY_CALLSTATE_DISABLED: TELEPHONY_CALLSTATE = 0;
pub const TELEPHONY_CALLSTATE_ENABLED: TELEPHONY_CALLSTATE = 1;
pub const TELEPHONY_CALLSTATE_HOLD: TELEPHONY_CALLSTATE = 2;
pub const TELEPHONY_CALLSTATE_PROVIDERTRANSITION: TELEPHONY_CALLSTATE = 3;
pub type TELEPHONY_CALLTYPE = i32;
pub const TELEPHONY_CALLTYPE_CIRCUITSWITCHED: TELEPHONY_CALLTYPE = 0;
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_LTE: TELEPHONY_CALLTYPE = 1;
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_WLAN: TELEPHONY_CALLTYPE = 2;
pub type TELEPHONY_PROVIDERCHANGEOP = i32;
pub const TELEPHONY_PROVIDERCHANGEOP_BEGIN: TELEPHONY_PROVIDERCHANGEOP = 1;
pub const TELEPHONY_PROVIDERCHANGEOP_CANCEL: TELEPHONY_PROVIDERCHANGEOP = 2;
pub const TELEPHONY_PROVIDERCHANGEOP_END: TELEPHONY_PROVIDERCHANGEOP = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union TIMECODE {
    pub Anonymous: TIMECODE_0,
    pub qw: super::winnt::DWORDLONG,
}
#[cfg(feature = "winnt")]
impl Default for TIMECODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct TIMECODE_0 {
    pub wFrameRate: u16,
    pub wFrameFract: u16,
    pub dwFrames: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct TIMECODE_SAMPLE {
    pub qwTick: i64,
    pub timecode: TIMECODE,
    pub dwUser: u32,
    pub dwFlags: u32,
}
#[cfg(feature = "winnt")]
impl Default for TIMECODE_SAMPLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSPORTAUDIOPARMS {
    pub EnableOutput: i32,
    pub EnableRecord: i32,
    pub EnableSelsync: i32,
    pub Input: i32,
    pub MonitorSource: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
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
    pub VolumeName: [super::winnt::TCHAR; 40],
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
#[cfg(feature = "winnt")]
impl Default for TRANSPORTBASICPARMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSPORTVIDEOPARMS {
    pub OutputMode: i32,
    pub Input: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSPORT_STATE {
    pub Mode: u32,
    pub State: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub type TunerLockType = i32;
pub const Tuner_LockType_Locked: TunerLockType = 2;
pub const Tuner_LockType_None: TunerLockType = 0;
pub const Tuner_LockType_Within_Scan_Sensing_Range: TunerLockType = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UMPDATAFORMAT {
    pub Position: i64,
    pub ByteCount: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct VBICAP_PROPERTIES_PROTECTION_S {
    pub Property: super::ks::KSPROPERTY,
    pub StreamIndex: u32,
    pub Status: u32,
}
#[cfg(feature = "ks")]
impl Default for VBICAP_PROPERTIES_PROTECTION_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VBICODECFILTERING_CC_SUBSTREAMS {
    pub SubstreamMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VBICODECFILTERING_NABTS_SUBSTREAMS {
    pub SubstreamMask: [u32; 128],
}
impl Default for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VBICODECFILTERING_SCANLINES {
    pub DwordBitArray: [u32; 32],
}
impl Default for VBICODECFILTERING_SCANLINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VBICODECFILTERING_STATISTICS_CC {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VBICODECFILTERING_STATISTICS_CC_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VBICODECFILTERING_STATISTICS_NABTS_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
pub type VIDEOENCODER_BITRATE_MODE = i32;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Default)]
pub struct VIDEOFORMAT_DX12 {
    pub Header: super::ks::KSATTRIBUTE,
    pub resourceLayout: u32,
    pub resourceFlags: u32,
    pub customLayout: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for VRAM_SURFACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct VRAM_SURFACE_INFO_PROPERTY_S {
    pub Property: super::ks::KSPROPERTY,
    pub pVramSurfaceInfo: PVRAM_SURFACE_INFO,
}
#[cfg(feature = "ks")]
impl Default for VRAM_SURFACE_INFO_PROPERTY_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VariableBitRateAverage: VIDEOENCODER_BITRATE_MODE = 1;
pub const VariableBitRatePeak: VIDEOENCODER_BITRATE_MODE = 2;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmeapi", feature = "mmreg"))]
#[derive(Clone, Copy)]
pub struct WAVEFORMATEXTENSIBLE_IEC61937 {
    pub FormatExt: super::mmreg::WAVEFORMATEXTENSIBLE,
    pub dwEncodedSamplesPerSec: u32,
    pub dwEncodedChannelCount: u32,
    pub dwAverageBytesPerSec: u32,
}
#[cfg(all(feature = "mmeapi", feature = "mmreg"))]
impl Default for WAVEFORMATEXTENSIBLE_IEC61937 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WNF_KSCAMERA_STREAMSTATE_INFO {
    pub ProcessId: u32,
    pub SessionId: u32,
    pub StreamState: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WST_BUFFER {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub WstLines: [WST_BUFFER_LINE; 17],
}
impl Default for WST_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WST_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 42],
}
impl Default for WST_BUFFER_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WST_BYTES_PER_LINE: u32 = 42;
pub const WST_TVTUNER_CHANGE_BEGIN_TUNE: u32 = 4096;
pub const WST_TVTUNER_CHANGE_END_TUNE: u32 = 8192;
pub const eConnType3Point5mm: EPcxConnectionType = 1;
pub const eConnTypeAtapiInternal: EPcxConnectionType = 3;
pub const eConnTypeCombination: EPcxConnectionType = 11;
pub const eConnTypeMultichannelAnalogDIN: EPcxConnectionType = 8;
pub const eConnTypeOptical: EPcxConnectionType = 5;
pub const eConnTypeOtherAnalog: EPcxConnectionType = 7;
pub const eConnTypeOtherDigital: EPcxConnectionType = 6;
pub const eConnTypeQuarter: EPcxConnectionType = 2;
pub const eConnTypeRCA: EPcxConnectionType = 4;
pub const eConnTypeRJ11Modem: EPcxConnectionType = 10;
pub const eConnTypeUnknown: EPcxConnectionType = 0;
pub const eConnTypeXlrProfessional: EPcxConnectionType = 9;
pub const eDeviceControlUseMissing: EDeviceControlUseType = 0;
pub const eDeviceControlUsePrimary: EDeviceControlUseType = 1;
pub const eDeviceControlUseSecondary: EDeviceControlUseType = 2;
pub const eGenLocInternal: EPcxGenLocation = 1;
pub const eGenLocOther: EPcxGenLocation = 3;
pub const eGenLocPrimaryBox: EPcxGenLocation = 0;
pub const eGenLocSeparate: EPcxGenLocation = 2;
pub const eGeoLocATAPI: EPcxGeoLocation = 13;
pub const eGeoLocBottom: EPcxGeoLocation = 6;
pub const eGeoLocDrivebay: EPcxGeoLocation = 10;
pub const eGeoLocFront: EPcxGeoLocation = 2;
pub const eGeoLocHDMI: EPcxGeoLocation = 11;
pub const eGeoLocInsideMobileLid: EPcxGeoLocation = 9;
pub const eGeoLocLeft: EPcxGeoLocation = 3;
pub const eGeoLocNotApplicable: EPcxGeoLocation = 14;
pub const eGeoLocOutsideMobileLid: EPcxGeoLocation = 12;
pub const eGeoLocRear: EPcxGeoLocation = 1;
pub const eGeoLocRearPanel: EPcxGeoLocation = 7;
pub const eGeoLocReserved5: u32 = 14;
pub const eGeoLocReserved6: EPcxGeoLocation = 15;
pub const eGeoLocRight: EPcxGeoLocation = 4;
pub const eGeoLocRiser: EPcxGeoLocation = 8;
pub const eGeoLocTop: EPcxGeoLocation = 5;
pub const ePortConnBothIntegratedAndJack: EPxcPortConnection = 2;
pub const ePortConnIntegratedDevice: EPxcPortConnection = 1;
pub const ePortConnJack: EPxcPortConnection = 0;
pub const ePortConnUnknown: EPxcPortConnection = 3;
