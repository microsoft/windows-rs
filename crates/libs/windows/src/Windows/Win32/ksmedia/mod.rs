pub const APO_CLASS_UUID: windows_core::GUID = windows_core::GUID::from_u128(0x5989fce8_9cd0_467d_8a6a_5419e31529d4);
pub const AUDIOENDPOINT_CLASS_UUID: windows_core::GUID = windows_core::GUID::from_u128(0xc166523c_fe0c_4a94_a586_f1a80cfbbf3e);
pub const AUDIOLOOPBACK_TAPPOINT_CAPS_POSTVOLUMEMUTE: i32 = 2;
pub const AUDIOLOOPBACK_TAPPOINT_CAPS_PREVOLUMEMUTE: i32 = 1;
pub const AUDIOLOOPBACK_TAPPOINT_POSTVOLUMEMUTE: AUDIOLOOPBACK_TAPPOINT_TYPE = 1;
pub const AUDIOLOOPBACK_TAPPOINT_PREVOLUMEMUTE: AUDIOLOOPBACK_TAPPOINT_TYPE = 0;
pub type AUDIOLOOPBACK_TAPPOINT_TYPE = i32;
pub const AUDIOMODULE_MAX_DATA_SIZE: i32 = 64000;
pub const AUDIOMODULE_MAX_NAME_CCH_SIZE: i32 = 128;
pub type AUDIOPOSTURE_ORIENTATION = i32;
pub const AUDIOPOSTURE_ORIENTATION_NOTROTATED: AUDIOPOSTURE_ORIENTATION = 0;
pub const AUDIOPOSTURE_ORIENTATION_ROTATED180DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 2;
pub const AUDIOPOSTURE_ORIENTATION_ROTATED270DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 3;
pub const AUDIOPOSTURE_ORIENTATION_ROTATED90DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    pub ResourceGroupAcquired: windows_core::BOOL,
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
pub const AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adbe_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_AUTOMATIC_GAIN_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc0_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BASS_BOOST: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc5_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BASS_MANAGEMENT: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adca_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BEAMFORMING: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc1_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_CONSTANT_TONE_REMOVAL: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc2_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_DEEP_NOISE_SUPPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64add0_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_DYNAMIC_RANGE_COMPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adce_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_ENVIRONMENTAL_EFFECTS: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adcb_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_EQUALIZER: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc3_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_FAR_FIELD_BEAMFORMING: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adcf_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_LOUDNESS_EQUALIZER: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc4_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_NOISE_SUPPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adbf_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_ROOM_CORRECTION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc9_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_COMPENSATION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adcd_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_FILL: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc8_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adcc_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_VIRTUAL_HEADPHONES: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc7_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_VIRTUAL_SURROUND: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc6_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_SIGNALPROCESSINGMODE_COMMUNICATIONS: windows_core::GUID = windows_core::GUID::from_u128(0x98951333_b9cd_48b1_a0a3_ff40682d73f7);
pub const AUDIO_SIGNALPROCESSINGMODE_DEFAULT: windows_core::GUID = windows_core::GUID::from_u128(0xc18e2f7e_933d_4965_b7d1_1eef228d2af3);
pub const AUDIO_SIGNALPROCESSINGMODE_FAR_FIELD_SPEECH: windows_core::GUID = windows_core::GUID::from_u128(0x28941cba_3be6_4a78_9a76_30fd91559b64);
pub const AUDIO_SIGNALPROCESSINGMODE_MEDIA: windows_core::GUID = windows_core::GUID::from_u128(0x4780004e_7133_41d8_8c74_660dadd2c0ee);
pub const AUDIO_SIGNALPROCESSINGMODE_MOVIE: windows_core::GUID = windows_core::GUID::from_u128(0xb26feb0d_ec94_477c_9494_d1ab8e753f6e);
pub const AUDIO_SIGNALPROCESSINGMODE_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x9cf2a70b_f377_403b_bd6b_360863e0355c);
pub const AUDIO_SIGNALPROCESSINGMODE_RAW: windows_core::GUID = windows_core::GUID::from_u128(0x9e90ea20_b493_4fd1_a1a8_7e1361a956cf);
pub const AUDIO_SIGNALPROCESSINGMODE_SPEECH: windows_core::GUID = windows_core::GUID::from_u128(0xfc1cfc9b_b9d6_4cfa_b5e0_4bb2166878b2);
pub const BLUETOOTHLE_MIDI_SERVICE_UUID: windows_core::GUID = windows_core::GUID::from_u128(0x03b80e5a_ede8_4b33_a751_6ce34ec4c700);
pub const BLUETOOTH_MIDI_DATAIO_CHARACTERISTIC: windows_core::GUID = windows_core::GUID::from_u128(0x7772e5db_3868_4112_a1a9_f2669d106bf3);
pub type CAPTURE_MEMORY_ALLOCATION_FLAGS = i32;
pub const CASCADE_FORM: KSDS3D_HRTF_FILTER_METHOD = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const CC_MAX_HW_DECODE_LINES: i32 = 12;
pub const CLSID_KsIBasicAudioInterfaceHandler: windows_core::GUID = windows_core::GUID::from_u128(0xb9f8ac3e_0f71_11d2_b72c_00c04fb6bd3d);
pub const CODECAPI_ALLSETTINGS: windows_core::GUID = windows_core::GUID::from_u128(0x6a577e92_83e1_4113_adc2_4fcec32f83a1);
pub const CODECAPI_AUDIO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0xb9d19a3e_f897_429c_bc46_8138b7272b2d);
pub const CODECAPI_CHANGELISTS: windows_core::GUID = windows_core::GUID::from_u128(0x62b12acf_f6b0_47d9_9456_96f22c4e0b9d);
pub const CODECAPI_CURRENTCHANGELIST: windows_core::GUID = windows_core::GUID::from_u128(0x1cb14e83_7d72_4657_83fd_47a2c5b9d13d);
pub const CODECAPI_SETALLDEFAULTS: windows_core::GUID = windows_core::GUID::from_u128(0x6c5e6a7c_acf8_4f55_a999_1a628109051b);
pub const CODECAPI_SUPPORTSEVENTS: windows_core::GUID = windows_core::GUID::from_u128(0x0581af97_7693_4dbd_9dca_3f9ebd6585a1);
pub const CODECAPI_VIDEO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x7112e8e1_3d03_47ef_8e60_03f1cf537301);
pub type CONSTRICTOR_OPTION = i32;
pub const CONSTRICTOR_OPTION_DISABLE: CONSTRICTOR_OPTION = 0;
pub const CONSTRICTOR_OPTION_MUTE: CONSTRICTOR_OPTION = 1;
pub const ConstantBitRate: VIDEOENCODER_BITRATE_MODE = 0;
pub const DDPF_FOURCC: i32 = 4;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DDVIDEOPORTCONNECT {
    pub dwSize: u32,
    pub dwPortWidth: u32,
    pub guidTypeID: windows_core::GUID,
    pub dwFlags: u32,
    pub dwReserved1: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_KsAudio_Controller_DeviceInterface_Path: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x13e004d6_b066_43bd_913b_a415cd13da87), pid: 3 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_KsAudio_PacketSize_Constraints: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x13e004d6_b066_43bd_913b_a415cd13da87), pid: 2 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_KsAudio_PacketSize_Constraints2: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x9404f781_7191_409b_8b0b_80bf6ec229ae), pid: 2 };
pub const DIRECT_FORM: KSDS3D_HRTF_FILTER_METHOD = 0;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct DS3DVECTOR {
    pub Anonymous: DS3DVECTOR_0,
    pub Anonymous2: DS3DVECTOR_1,
    pub Anonymous3: DS3DVECTOR_2,
}
#[cfg(feature = "minwindef")]
impl Default for DS3DVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_0 {
    pub x: super::FLOAT,
    pub dvX: super::FLOAT,
}
#[cfg(feature = "minwindef")]
impl Default for DS3DVECTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_1 {
    pub y: super::FLOAT,
    pub dvY: super::FLOAT,
}
#[cfg(feature = "minwindef")]
impl Default for DS3DVECTOR_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_2 {
    pub z: super::FLOAT,
    pub dvZ: super::FLOAT,
}
#[cfg(feature = "minwindef")]
impl Default for DS3DVECTOR_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS3D_HRTF_VERSION_1: KSDS3D_HRTF_FILTER_VERSION = 0;
pub type EDeviceControlUseType = i32;
pub const ENCAPIPARAM_BITRATE: windows_core::GUID = windows_core::GUID::from_u128(0x49cc4c43_ca83_4ad4_a9af_f3696af666df);
pub const ENCAPIPARAM_BITRATE_MODE: windows_core::GUID = windows_core::GUID::from_u128(0xee5fb25c_c713_40d1_9d58_c0d7241e250f);
pub const ENCAPIPARAM_PEAK_BITRATE: windows_core::GUID = windows_core::GUID::from_u128(0x703f16a9_3d48_44a1_b077_018dff915d19);
pub type EPcxConnectionType = i32;
pub type EPcxGenLocation = i32;
pub const EPcxGenLocation_enum_count: EPcxGenLocation = 4;
pub type EPcxGeoLocation = i32;
pub const EPcxGeoLocation_enum_count: EPcxGeoLocation = 16;
pub type EPxcPortConnection = i32;
pub const EVENTSETID_CROSSBAR: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0641_28e4_11d0_a18c_00a0c9118956);
pub const EVENTSETID_TUNER: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0606_28e4_11d0_a18c_00a0c9118956);
pub const EVENTSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: windows_core::GUID = windows_core::GUID::from_u128(0x2fdffc5d_c732_4ba6_b5df_6b4d7fc88b8b);
pub const EVENTSETID_VIDEODECODER: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0621_28e4_11d0_a18c_00a0c9118956);
pub const FLOAT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = 0;
pub const FULL_FILTER: KSDS3D_HRTF_FILTER_QUALITY = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    pub Size: u32,
    pub PrimaryChannelCount: u32,
    pub PrimaryChannelStartPosition: u32,
    pub PrimaryChannelMask: u32,
    pub InterleavedChannelCount: u32,
    pub InterleavedChannelStartPosition: u32,
    pub InterleavedChannelMask: u32,
}
pub const JACKDESC2_DYNAMIC_FORMAT_CHANGE_CAPABILITY: i32 = 2;
pub const JACKDESC2_PRESENCE_DETECT_CAPABILITY: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAC3_ALTERNATE_AUDIO {
    pub fStereo: windows_core::BOOL,
    pub DualMode: u32,
}
pub const KSAC3_ALTERNATE_AUDIO_1: i32 = 1;
pub const KSAC3_ALTERNATE_AUDIO_2: i32 = 2;
pub const KSAC3_ALTERNATE_AUDIO_BOTH: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAC3_BIT_STREAM_MODE {
    pub BitStreamMode: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAC3_DIALOGUE_LEVEL {
    pub DialogueLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAC3_DOWNMIX {
    pub fDownMix: windows_core::BOOL,
    pub fDolbySurround: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAC3_ERROR_CONCEALMENT {
    pub fRepeatPreviousBlock: windows_core::BOOL,
    pub fErrorInCurrentBlock: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAC3_ROOM_TYPE {
    pub fLargeRoom: windows_core::BOOL,
}
pub const KSAC3_SERVICE_COMMENTARY: i32 = 5;
pub const KSAC3_SERVICE_DIALOG_ONLY: i32 = 4;
pub const KSAC3_SERVICE_EMERGENCY_FLASH: i32 = 6;
pub const KSAC3_SERVICE_HEARING_IMPAIRED: i32 = 3;
pub const KSAC3_SERVICE_MAIN_AUDIO: i32 = 0;
pub const KSAC3_SERVICE_NO_DIALOG: i32 = 1;
pub const KSAC3_SERVICE_VISUALLY_IMPAIRED: i32 = 2;
pub const KSAC3_SERVICE_VOICE_OVER: i32 = 7;
pub const KSALGORITHMINSTANCE_SYSTEM_ACOUSTIC_ECHO_CANCEL: windows_core::GUID = windows_core::GUID::from_u128(0x1c22c56d_9879_4f5b_a389_27996ddc2810);
pub const KSALGORITHMINSTANCE_SYSTEM_AGC: windows_core::GUID = windows_core::GUID::from_u128(0x950e55b9_877c_4c67_be08_e47b5611130a);
pub const KSALGORITHMINSTANCE_SYSTEM_MICROPHONE_ARRAY_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0xb6f5a0a0_9e61_4f8c_91e3_76cf0f3c471f);
pub const KSALGORITHMINSTANCE_SYSTEM_NOISE_SUPPRESS: windows_core::GUID = windows_core::GUID::from_u128(0x5ab0882e_7274_4516_877d_4eee99ba4fd0);
pub const KSATTRIBUTEID_AUDIOLOOPBACK_TAPPOINT: windows_core::GUID = windows_core::GUID::from_u128(0x2795a0f7_1688_44fe_bc14_bf8273992141);
pub const KSATTRIBUTEID_AUDIOSIGNALPROCESSING_MODE: windows_core::GUID = windows_core::GUID::from_u128(0xe1f89eb5_5f46_419b_967b_ff6770b98401);
pub const KSATTRIBUTEID_VIDEOFORMAT_DX12: windows_core::GUID = windows_core::GUID::from_u128(0xfc9d87b5_0b02_438e_89b0_e241fce889ad);
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSATTRIBUTE_AUDIOLOOPBACK_TAPPOINT {
    pub AttributeHeader: super::KSATTRIBUTE,
    pub TapPoint: AUDIOLOOPBACK_TAPPOINT_TYPE,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    pub AttributeHeader: super::KSATTRIBUTE,
    pub SignalProcessingMode: windows_core::GUID,
}
pub const KSAUDDECOUTMODE_PCM_51: i32 = 2;
pub const KSAUDDECOUTMODE_SPDIFF: i32 = 4;
pub const KSAUDDECOUTMODE_STEREO_ANALOG: i32 = 1;
pub const KSAUDFNAME_3D_CENTER: windows_core::GUID = windows_core::GUID::from_u128(0x9f0670b4_991f_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_3D_DEPTH: windows_core::GUID = windows_core::GUID::from_u128(0x63ff5747_991f_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_3D_STEREO: windows_core::GUID = windows_core::GUID::from_u128(0x185fede2_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_ALTERNATE_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0x2bc31d6b_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_AUX: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfe_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_AUX_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfd_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_AUX_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfc_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_BASS: windows_core::GUID = windows_core::GUID::from_u128(0x185fede0_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfb_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf3_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedea_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fede9_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_IN: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf9_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf4_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedec_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedeb_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MASTER_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fede4_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MASTER_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fede3_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MICROPHONE_BOOST: windows_core::GUID = windows_core::GUID::from_u128(0x2bc31d6a_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MIC_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf5_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIC_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedee_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIC_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185feded_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf8_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf2_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fede8_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fede7_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDRANGE: windows_core::GUID = windows_core::GUID::from_u128(0xa2cbe478_ae84_49a1_8b72_4ad09b78ed34);
pub const KSAUDFNAME_MONO_MIX: windows_core::GUID = windows_core::GUID::from_u128(0x00dff078_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_MIX_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x2bc31d69_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_MIX_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x22b0eafe_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT: windows_core::GUID = windows_core::GUID::from_u128(0xf9b41dc3_96e2_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x1ad247ec_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x1ad247eb_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_PC_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0x185fedff_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PC_SPEAKER_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf1_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PC_SPEAKER_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf0_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PEAKMETER: windows_core::GUID = windows_core::GUID::from_u128(0x57e24340_fc5b_4612_a562_72b11a29dfae);
pub const KSAUDFNAME_RECORDING_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfa_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_RECORDING_SOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedef_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_STEREO_MIX: windows_core::GUID = windows_core::GUID::from_u128(0x00dff077_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_STEREO_MIX_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x22b0eafd_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_STEREO_MIX_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x1ad247ed_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_TREBLE: windows_core::GUID = windows_core::GUID::from_u128(0x185fede1_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x915daec4_a434_11d2_ac52_00c04f8efb68);
pub const KSAUDFNAME_VIDEO_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x9b46e709_992a_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_VIDEO_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x9b46e708_992a_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_VOLUME_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf7_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf6_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fede6_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_OUT_MIX: windows_core::GUID = windows_core::GUID::from_u128(0x185fee00_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fede5_9905_11d1_95a9_00c04fb925d3);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    pub MinBufferBytes: u32,
    pub MaxBufferBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIOENGINE_DESCRIPTOR {
    pub nHostPinId: u32,
    pub nOffloadPinId: u32,
    pub nLoopbackPinId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIOENGINE_DEVICECONTROLS {
    pub Volume: EDeviceControlUseType,
    pub Mute: EDeviceControlUseType,
    pub PeakMeter: EDeviceControlUseType,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIOENGINE_VOLUMELEVEL {
    pub TargetVolume: i32,
    pub CurveType: AUDIO_CURVE_TYPE,
    pub CurveDuration: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KSAUDIOMODULE_DESCRIPTOR {
    pub ClassId: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIOMODULE_NOTIFICATION_0_0 {
    pub DeviceId: windows_core::GUID,
    pub ClassId: windows_core::GUID,
    pub InstanceId: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSAUDIOMODULE_PROPERTY {
    pub Property: super::KSPROPERTY,
    pub ClassId: windows_core::GUID,
    pub InstanceId: u32,
}
#[cfg(feature = "ks")]
impl Default for KSAUDIOMODULE_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIO_CHANNEL_CONFIG {
    pub ActiveSpeakerPositions: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIO_COPY_PROTECTION {
    pub fCopyrighted: windows_core::BOOL,
    pub fOriginal: windows_core::BOOL,
}
pub const KSAUDIO_CPU_RESOURCES_HOST_CPU: i32 = 2147483647;
pub const KSAUDIO_CPU_RESOURCES_NOT_HOST_CPU: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIO_DYNAMIC_RANGE {
    pub QuietCompression: u32,
    pub LoudCompression: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIO_MICROPHONE_COORDINATES {
    pub usType: u16,
    pub wXCoord: i16,
    pub wYCoord: i16,
    pub wZCoord: i16,
    pub wVerticalAngle: i16,
    pub wHorizontalAngle: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIO_MIXLEVEL {
    pub Mute: windows_core::BOOL,
    pub Level: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIO_MIX_CAPS {
    pub Mute: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT {
    pub ProcessingMode: windows_core::GUID,
    pub SamplesPerProcessingPacket: u32,
    pub ProcessingPacketDurationInHns: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIO_POSITION {
    pub PlayOffset: super::DWORDLONG,
    pub WriteOffset: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSAUDIO_POSITIONEX {
    pub TimerFrequency: super::LARGE_INTEGER,
    pub TimeStamp1: super::LARGE_INTEGER,
    pub Position: KSAUDIO_POSITION,
    pub TimeStamp2: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for KSAUDIO_POSITIONEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSAUDIO_PRESENTATION_POSITION {
    pub u64PositionInBlocks: u64,
    pub u64QPCPosition: u64,
}
pub const KSAUDIO_QUALITY_ADVANCED: i32 = 3;
pub const KSAUDIO_QUALITY_BASIC: i32 = 2;
pub const KSAUDIO_QUALITY_PC: i32 = 1;
pub const KSAUDIO_QUALITY_WORST: i32 = 0;
pub const KSAUDIO_SPEAKER_1POINT1: i32 = 12;
pub const KSAUDIO_SPEAKER_2POINT1: i32 = 11;
pub const KSAUDIO_SPEAKER_3POINT0: i32 = 7;
pub const KSAUDIO_SPEAKER_3POINT1: i32 = 15;
pub const KSAUDIO_SPEAKER_5POINT0: i32 = 1543;
pub const KSAUDIO_SPEAKER_5POINT1: i32 = 63;
pub const KSAUDIO_SPEAKER_5POINT1_BACK: i32 = 63;
pub const KSAUDIO_SPEAKER_5POINT1_SURROUND: i32 = 1551;
pub const KSAUDIO_SPEAKER_7POINT0: i32 = 1591;
pub const KSAUDIO_SPEAKER_7POINT1: i32 = 255;
pub const KSAUDIO_SPEAKER_7POINT1_SURROUND: i32 = 1599;
pub const KSAUDIO_SPEAKER_7POINT1_WIDE: i32 = 255;
pub const KSAUDIO_SPEAKER_DIRECTOUT: i32 = 0;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_CENTER: i32 = 4;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_LEFT: i32 = 1;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_RIGHT: i32 = 2;
pub const KSAUDIO_SPEAKER_GROUND_REAR_LEFT: i32 = 16;
pub const KSAUDIO_SPEAKER_GROUND_REAR_RIGHT: i32 = 32;
pub const KSAUDIO_SPEAKER_MONO: i32 = 4;
pub const KSAUDIO_SPEAKER_QUAD: i32 = 51;
pub const KSAUDIO_SPEAKER_STEREO: i32 = 3;
pub const KSAUDIO_SPEAKER_SUPER_WOOFER: i32 = 8;
pub const KSAUDIO_SPEAKER_SURROUND: i32 = 263;
pub const KSAUDIO_SPEAKER_TOP_MIDDLE: i32 = 2048;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_HEADPHONE: i32 = -1;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MAX: i32 = 180;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MIN: i32 = 5;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_NARROW: i32 = 10;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_WIDE: i32 = 20;
pub const KSCAMERAPROFILE_BalancedVideoAndPhoto: windows_core::GUID = windows_core::GUID::from_u128(0x6b52b017_42c7_4a21_bfe3_23f009149887);
pub const KSCAMERAPROFILE_CompressedCamera: windows_core::GUID = windows_core::GUID::from_u128(0x0e34cdc1_27ad_437f_abde_02b629f37b44);
pub const KSCAMERAPROFILE_FLAGS_FACEDETECTION: i32 = 8;
pub const KSCAMERAPROFILE_FLAGS_PHOTOHDR: i32 = 4;
pub const KSCAMERAPROFILE_FLAGS_PREVIEW_RES_MUSTMATCH: i32 = 32;
pub const KSCAMERAPROFILE_FLAGS_VARIABLEPHOTOSEQUENCE: i32 = 16;
pub const KSCAMERAPROFILE_FLAGS_VIDEOHDR: i32 = 2;
pub const KSCAMERAPROFILE_FLAGS_VIDEOSTABLIZATION: i32 = 1;
pub const KSCAMERAPROFILE_FaceAuth_Mode: windows_core::GUID = windows_core::GUID::from_u128(0x81361b22_700b_4546_a2d4_c52e907bfc27);
pub const KSCAMERAPROFILE_HDRWithWCGPhoto: windows_core::GUID = windows_core::GUID::from_u128(0x9bf6f1ff_b555_4625_b326_a46def318fb7);
pub const KSCAMERAPROFILE_HDRWithWCGVideo: windows_core::GUID = windows_core::GUID::from_u128(0x4b27c336_4924_4989_b994_fdaf1dc7cd85);
pub const KSCAMERAPROFILE_HighFrameRate: windows_core::GUID = windows_core::GUID::from_u128(0x566e6113_8c35_48e7_b89f_d23fdc1219dc);
pub const KSCAMERAPROFILE_HighQualityPhoto: windows_core::GUID = windows_core::GUID::from_u128(0x32440725_961b_4ca3_b5b2_854e719d9e1b);
pub const KSCAMERAPROFILE_Legacy: windows_core::GUID = windows_core::GUID::from_u128(0xb4894d81_62b7_4eec_8740_80658c4a9d3e);
pub const KSCAMERAPROFILE_PhotoSequence: windows_core::GUID = windows_core::GUID::from_u128(0x02399d9d_4ee8_49ba_bc07_5ff156531413);
pub const KSCAMERAPROFILE_VariablePhotoSequence: windows_core::GUID = windows_core::GUID::from_u128(0x9ff2cb56_e75a_49b1_a928_9985d5946f87);
pub const KSCAMERAPROFILE_VideoConferencing: windows_core::GUID = windows_core::GUID::from_u128(0xc5444a88_e1bf_4597_b2dd_9e1ead864bb8);
pub const KSCAMERAPROFILE_VideoHDR8: windows_core::GUID = windows_core::GUID::from_u128(0xd4f3f4ec_bdff_4314_b1d4_008e281f74e7);
pub const KSCAMERAPROFILE_VideoRecording: windows_core::GUID = windows_core::GUID::from_u128(0xa0e517e8_8f8c_4f6f_9a57_46fc2f647ec0);
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_AUTO: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_FNF: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_HDR: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_ULTRALOWLIGHT: i32 = 8;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_BLUR: i32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    pub Resolution: super::SIZE,
    pub MaxFrameRate: KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0,
    pub MaskResolution: super::SIZE,
    pub SubType: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    pub Numerator: i32,
    pub Denominator: i32,
}
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_MASK: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_SHALLOWFOCUS: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    pub PitchAngle: i32,
    pub YawAngle: i32,
    pub Flag: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_EXTENDEDPROP_CAPS_ASYNCCONTROL: u64 = 9223372036854775808;
pub const KSCAMERA_EXTENDEDPROP_CAPS_CANCELLABLE: i64 = 4611686018427387904;
pub const KSCAMERA_EXTENDEDPROP_CAPS_MASK: u64 = 18374686479671623680;
pub const KSCAMERA_EXTENDEDPROP_CAPS_RESERVED: u64 = 18374686479671623680;
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_AUTOFACEFRAMING: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    pub Size: u32,
    pub Count: u32,
}
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_MANUAL: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_MASK: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    pub OriginX: i32,
    pub OriginY: i32,
    pub WindowSize: i32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Value: i32,
    pub Reserved: u64,
}
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_FULLSTEP: i32 = 16;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_HALFSTEP: i32 = 8;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_QUARTERSTEP: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_SIXTHSTEP: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_THIRDSTEP: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_STARE: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_ALTERNATIVE_FRAME_ILLUMINATION: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_BACKGROUND_SUBTRACTION: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_DISABLED: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_ADVANCED_MASK: i32 = 24;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_BLINK: i32 = 8;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_MASK: i32 = 7;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PHOTO: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PREVIEW: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_SMILE: i32 = 16;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_VIDEO: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    pub NormalizedFocalLengthX: u32,
    pub NormalizedFocalLengthY: u32,
    pub Flag: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_AUTO: i32 = 256;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_MASK: i32 = 384;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_ON: i32 = 128;
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO_ADJUSTABLEPOWER: i32 = 8;
pub const KSCAMERA_EXTENDEDPROP_FLASH_MODE_MASK: i32 = 15;
pub const KSCAMERA_EXTENDEDPROP_FLASH_MULTIFLASHSUPPORTED: i32 = 64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON_ADJUSTABLEPOWER: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_FLASH_REDEYEREDUCTION: i32 = 16;
pub const KSCAMERA_EXTENDEDPROP_FLASH_SINGLEFLASH: i32 = 32;
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_ON: i32 = 1;
pub type KSCAMERA_EXTENDEDPROP_FOCUSSTATE = i32;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FAILED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 4;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FOCUSED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 3;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_LOST: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 1;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_SEARCHING: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 2;
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_UNINITIALIZED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = 0;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUS: i32 = 256;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUSLOCK: i32 = 512;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_HYPERFOCAL: i32 = 33554432;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_INFINITY: i32 = 16777216;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_MASK: i32 = 117440512;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_NEAREST: i32 = 67108864;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DRIVERFALLBACK_OFF: i32 = 2048;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_MODE_ADVANCED_MASK: i32 = 7680;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_MODE_MASK: i32 = 263;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_FULLRANGE: i32 = 262144;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_HYPERFOCAL: i32 = 1048576;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_INFINITY: i32 = 524288;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_MACRO: i32 = 65536;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_MASK: i32 = 2031616;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_NORMAL: i32 = 131072;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_REGIONBASED: i32 = 4096;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_UNLOCK: i32 = 1024;
pub const KSCAMERA_EXTENDEDPROP_FRAMERATE_THROTTLE_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_FRAMERATE_THROTTLE_ON: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_HEADER {
    pub Version: u32,
    pub PinId: u32,
    pub Size: u32,
    pub Result: u32,
    pub Flags: u64,
    pub Capability: u64,
}
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALTERNATING_FRAME_ILLUMINATION: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALWAYS_ON: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_OFF: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_ISO_100: i32 = 8;
pub const KSCAMERA_EXTENDEDPROP_ISO_12800: i32 = 1024;
pub const KSCAMERA_EXTENDEDPROP_ISO_1600: i32 = 128;
pub const KSCAMERA_EXTENDEDPROP_ISO_200: i32 = 16;
pub const KSCAMERA_EXTENDEDPROP_ISO_25600: i32 = 2048;
pub const KSCAMERA_EXTENDEDPROP_ISO_3200: i32 = 256;
pub const KSCAMERA_EXTENDEDPROP_ISO_400: i32 = 32;
pub const KSCAMERA_EXTENDEDPROP_ISO_50: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_ISO_6400: i32 = 512;
pub const KSCAMERA_EXTENDEDPROP_ISO_80: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_ISO_800: i32 = 64;
pub const KSCAMERA_EXTENDEDPROP_ISO_AUTO: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_ISO_MANUAL: i64 = 36028797018963968;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_METADATAINFO {
    pub BufferAlignment: i32,
    pub MaxMetadataBufferSize: u32,
}
pub const KSCAMERA_EXTENDEDPROP_METADATA_ALIGNMENTREQUIRED: i32 = 256;
pub const KSCAMERA_EXTENDEDPROP_METADATA_MEMORYTYPE_MASK: i32 = 255;
pub const KSCAMERA_EXTENDEDPROP_METADATA_SYSTEMMEMORY: i32 = 1;
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
pub const KSCAMERA_EXTENDEDPROP_OIS_AUTO: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_OIS_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_OIS_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_DEFAULT: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_LATENCY: i32 = 8;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PERF_MASK: i32 = 28;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PHOTO: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_POWER: i32 = 16;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PRIMARYUSE_MASK: i32 = 3;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_QUALITY: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_VIDEO: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_ON: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    pub RequestedHistoryFrames: u32,
    pub MaxHistoryFrames: u32,
    pub SubMode: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_NORMAL: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_NONE: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_VARIABLE: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_16X: i32 = 8;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_2X: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_4X: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_8X: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_DISABLE: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_PROFILE {
    pub ProfileId: windows_core::GUID,
    pub Index: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_DYNAMIC: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_MASK: i32 = 3;
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_ON: i32 = 1;
pub type KSCAMERA_EXTENDEDPROP_ROITYPE = i32;
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_FACE: KSCAMERA_EXTENDEDPROP_ROITYPE = 1;
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_UNKNOWN: KSCAMERA_EXTENDEDPROP_ROITYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    pub ControlId: u32,
    pub MaxNumberOfROIs: u32,
    pub Capability: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    pub Size: u32,
    pub ConfigCapCount: u32,
    pub Reserved: u64,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_INFO {
    pub Region: super::RECT,
    pub Flags: u64,
    pub Weight: i32,
    pub RegionOfInterestType: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    pub ControlId: u32,
    pub ROICount: u32,
    pub Result: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    pub Size: u32,
    pub ControlCount: u32,
    pub Reserved: u64,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_AUTO: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BACKLIT: i32 = 1024;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BEACH: i32 = 32;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_CANDLELIGHT: i32 = 128;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_LANDSCAPE: i32 = 256;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MACRO: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MANUAL: i64 = 36028797018963968;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHT: i32 = 16;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHTPORTRAIT: i32 = 512;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_PORTRAIT: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SNOW: i32 = 8;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SPORT: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SUNSET: i32 = 64;
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_DISABLED: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_ENABLED: i32 = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSCAMERA_EXTENDEDPROP_VALUE {
    pub Value: KSCAMERA_EXTENDEDPROP_VALUE_0,
}
#[cfg(feature = "winnt")]
impl Default for KSCAMERA_EXTENDEDPROP_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union KSCAMERA_EXTENDEDPROP_VALUE_0 {
    pub dbl: f64,
    pub ull: u64,
    pub ul: u32,
    pub ratio: super::ULARGE_INTEGER,
    pub l: i32,
    pub ll: i64,
}
#[cfg(feature = "winnt")]
impl Default for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_VFR_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_VFR_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_AUTO: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_AUTO: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_LOCK: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_MANUAL: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_MASK: i32 = 7;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Step: i32,
    pub VideoProc: KSCAMERA_EXTENDEDPROP_VALUE,
    pub Reserved: u64,
}
#[cfg(feature = "winnt")]
impl Default for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_AUTO: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_AUTO: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_OFF: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_ON: i32 = 4;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_OFF: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON_ADJUSTABLEPOWER: i32 = 2;
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_DISABLED: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_ENABLED: i32 = 1;
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
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DEFAULT: i32 = 0;
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DIRECT: i32 = 1;
pub const KSCAMERA_EXTENDEDPROP_ZOOM_SMOOTH: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub MaskCoverageBoundingBox: super::RECT,
    pub MaskResolution: super::SIZE,
    pub ForegroundBoundingBox: super::RECT,
    pub MaskData: [u8; 1],
}
#[cfg(feature = "windef")]
impl Default for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURECOMPENSATION: i32 = 2;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURETIME: i32 = 1;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASH: i32 = 64;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASHPOWER: i32 = 128;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FOCUSSTATE: i32 = 8;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ISOSPEED: i32 = 4;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_LENSPOSITION: i32 = 16;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SCENEMODE: i32 = 512;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SENSORFRAMERATE: i32 = 1024;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_WHITEBALANCE: i32 = 32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ZOOMFACTOR: i32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_METADATA_DIGITALWINDOW {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Window: KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_METADATA_FRAMEILLUMINATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Flags: u32,
    pub Reserved: u32,
}
pub const KSCAMERA_METADATA_FRAMEILLUMINATION_FLAG_ON: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_METADATA_ITEMHEADER {
    pub MetadataId: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_METADATA_PHOTOCONFIRMATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub PhotoConfirmationIndex: u32,
    pub Reserved: u32,
}
pub type KSCAMERA_MetadataId = i32;
pub const KSCAMERA_PERFRAMESETTING_AUTO: i64 = 4294967296;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    pub Size: u32,
    pub ItemCount: u32,
    pub Flags: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    pub Size: u32,
    pub Reserved: u32,
    pub Id: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    pub Size: u32,
    pub Id: u32,
    pub ItemCount: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_HEADER {
    pub Size: u32,
    pub FrameCount: u32,
    pub Id: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
pub const KSCAMERA_PERFRAMESETTING_ITEM_ISO: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 4;
pub const KSCAMERA_PERFRAMESETTING_ITEM_PHOTOCONFIRMATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = 6;
pub type KSCAMERA_PERFRAMESETTING_ITEM_TYPE = i32;
pub const KSCAMERA_PERFRAMESETTING_MANUAL: i64 = 8589934592;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PROFILE_CONCURRENCYINFO {
    pub ReferenceGuid: windows_core::GUID,
    pub Reserved: u32,
    pub ProfileCount: u32,
    pub Profiles: PKSCAMERA_PROFILE_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PROFILE_INFO {
    pub ProfileId: windows_core::GUID,
    pub Index: u32,
    pub PinCount: u32,
    pub Pins: PKSCAMERA_PROFILE_PININFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PROFILE_MEDIAINFO_0 {
    pub X: u32,
    pub Y: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PROFILE_MEDIAINFO_1 {
    pub Numerator: u32,
    pub Denominator: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_PROFILE_PININFO {
    pub PinCategory: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PROFILE_PININFO_0_0 {
    pub PinIndex: u16,
    pub ProfileSensorType: u16,
}
pub const KSCATEGORY_ACOUSTIC_ECHO_CANCEL: windows_core::GUID = windows_core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const KSCATEGORY_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x6994ad04_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_CROSSBAR: windows_core::GUID = windows_core::GUID::from_u128(0xa799a801_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x19689bf6_c384_48fd_ad51_90e58c79f70b);
pub const KSCATEGORY_ESCALANTE_PLATFORM_DRIVER: windows_core::GUID = windows_core::GUID::from_u128(0x74f3aea8_9768_11d1_8e07_00a0c95ec22e);
pub const KSCATEGORY_MICROPHONE_ARRAY_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x830a44f2_a32d_476b_be97_42845673b35a);
pub const KSCATEGORY_MULTIPLEXER: windows_core::GUID = windows_core::GUID::from_u128(0x7a5de1d3_01a1_452c_b481_4fa2b96271e8);
pub const KSCATEGORY_NETWORK: windows_core::GUID = windows_core::GUID::from_u128(0x67c9cc3c_69c4_11d2_8759_00a0c9223196);
pub const KSCATEGORY_REALTIME: windows_core::GUID = windows_core::GUID::from_u128(0xeb115ffc_10c8_4964_831d_6dcb02e6f23f);
pub const KSCATEGORY_TEXT: windows_core::GUID = windows_core::GUID::from_u128(0x6994ad06_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_TOPOLOGY: windows_core::GUID = windows_core::GUID::from_u128(0xdda54a40_1e4c_11d1_a050_405705c10000);
pub const KSCATEGORY_TVAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xa799a802_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_TVTUNER: windows_core::GUID = windows_core::GUID::from_u128(0xa799a800_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_VBICODEC: windows_core::GUID = windows_core::GUID::from_u128(0x07dad660_22f1_11d1_a9f4_00c04fbbde8f);
pub const KSCATEGORY_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x6994ad05_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_VIRTUAL: windows_core::GUID = windows_core::GUID::from_u128(0x3503eac4_1f26_11d1_8ab0_00a0c9223196);
pub const KSCATEGORY_VPMUX: windows_core::GUID = windows_core::GUID::from_u128(0xa799a803_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_WDMAUD_USE_PIN_NAME: windows_core::GUID = windows_core::GUID::from_u128(0x47a4fa20_a251_11d1_a050_0000f8004788);
pub const KSCOMPONENTID_USBAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x8f1275f0_26e9_4264_ba4d_39fff01d94aa);
pub const KSCameraProfileSensorType_Custom: i32 = 128;
pub const KSCameraProfileSensorType_Depth: i32 = 4;
pub const KSCameraProfileSensorType_ImageSegmentation: i32 = 16;
pub const KSCameraProfileSensorType_Infrared: i32 = 2;
pub const KSCameraProfileSensorType_PoseTracking: i32 = 8;
pub const KSCameraProfileSensorType_RGB: i32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "ks", feature = "mmeapi"))]
#[derive(Clone, Copy)]
pub struct KSDATAFORMAT_DSOUND {
    pub DataFormat: super::KSDATARANGE,
    pub BufferDesc: KSDSOUND_BUFFERDESC,
}
#[cfg(all(feature = "ks", feature = "mmeapi"))]
impl Default for KSDATAFORMAT_DSOUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDATAFORMAT_SPECIFIER_AC3_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e4_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_ANALOGVIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x0482dde0_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_AC3_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b35_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b32_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b31_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b34_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b33_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DSOUND: windows_core::GUID = windows_core::GUID::from_u128(0x518590a2_a184_11d0_8522_00c04fd9baf3);
pub const KSDATAFORMAT_SPECIFIER_H264_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x2017be05_6629_4248_aaed_7e1a47bc9b9c);
pub const KSDATAFORMAT_SPECIFIER_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x692fa379_d3e8_4651_b5b4_0b94b013eeaf);
pub const KSDATAFORMAT_SPECIFIER_JPEG_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x692fa379_d3e8_4651_b5b4_0b94b013eeaf);
pub const KSDATAFORMAT_SPECIFIER_LPCM_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e6_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_MPEG1_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x05589f82_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SPECIFIER_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e5_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_VBI: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76e0_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SPECIFIER_VC_ID: windows_core::GUID = windows_core::GUID::from_u128(0xad98d184_aac3_11d0_a41c_00a0c9223196);
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO: windows_core::GUID = windows_core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO2: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76a0_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SPECIFIER_WAVEFORMATEX: windows_core::GUID = windows_core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SUBTYPE_AC3_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802c_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_ADPCM: windows_core::GUID = windows_core::GUID::from_u128(0x00000002_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_ALAW: windows_core::GUID = windows_core::GUID::from_u128(0x00000006_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_ANALOG: windows_core::GUID = windows_core::GUID::from_u128(0x6dba3190_67bd_11cf_a0f7_0020afd156e4);
pub const KSDATAFORMAT_SUBTYPE_CC: windows_core::GUID = windows_core::GUID::from_u128(0x33214cc1_011f_11d2_b4b1_00a0d102cfbe);
pub const KSDATAFORMAT_SUBTYPE_D16: windows_core::GUID = windows_core::GUID::from_u128(0x00000050_0004_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_DRM: windows_core::GUID = windows_core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_DSS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xa0af4f82_e163_11d0_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_DSS_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xa0af4f81_e163_11d0_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_DTS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8033_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_AAC: windows_core::GUID = windows_core::GUID::from_u128(0x00000006_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ATRAC: windows_core::GUID = windows_core::GUID::from_u128(0x00000008_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL: windows_core::GUID = windows_core::GUID::from_u128(0x00000092_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS: windows_core::GUID = windows_core::GUID::from_u128(0x0000000a_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS_ATMOS: windows_core::GUID = windows_core::GUID::from_u128(0x0000010a_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT20: windows_core::GUID = windows_core::GUID::from_u128(0x0000010c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21: windows_core::GUID = windows_core::GUID::from_u128(0x0000030c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21_PROFILE4: windows_core::GUID = windows_core::GUID::from_u128(0x0000070c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MLP: windows_core::GUID = windows_core::GUID::from_u128(0x0000000c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DST: windows_core::GUID = windows_core::GUID::from_u128(0x0000000d_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS: windows_core::GUID = windows_core::GUID::from_u128(0x00000008_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E1: windows_core::GUID = windows_core::GUID::from_u128(0x0000010b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E2: windows_core::GUID = windows_core::GUID::from_u128(0x0000030b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS_HD: windows_core::GUID = windows_core::GUID::from_u128(0x0000000b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG1: windows_core::GUID = windows_core::GUID::from_u128(0x00000003_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG2: windows_core::GUID = windows_core::GUID::from_u128(0x00000004_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG3: windows_core::GUID = windows_core::GUID::from_u128(0x00000005_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL1_BL: windows_core::GUID = windows_core::GUID::from_u128(0x000210bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL1_LC: windows_core::GUID = windows_core::GUID::from_u128(0x000110bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL2_BL: windows_core::GUID = windows_core::GUID::from_u128(0x000220bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL2_LC: windows_core::GUID = windows_core::GUID::from_u128(0x000120bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL3_BL: windows_core::GUID = windows_core::GUID::from_u128(0x000230bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL3_LC: windows_core::GUID = windows_core::GUID::from_u128(0x000130bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL4_BL: windows_core::GUID = windows_core::GUID::from_u128(0x000240bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL4_LC: windows_core::GUID = windows_core::GUID::from_u128(0x000140bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL5_BL: windows_core::GUID = windows_core::GUID::from_u128(0x000250bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEGH_LEVEL5_LC: windows_core::GUID = windows_core::GUID::from_u128(0x000150bf_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ONE_BIT_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x00000009_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_WMA_PRO: windows_core::GUID = windows_core::GUID::from_u128(0x00000164_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IMAGE_RGB32: windows_core::GUID = windows_core::GUID::from_u128(0x00000016_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_JPEG: windows_core::GUID = windows_core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const KSDATAFORMAT_SUBTYPE_L16: windows_core::GUID = windows_core::GUID::from_u128(0x00000051_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L16_CUSTOM: windows_core::GUID = windows_core::GUID::from_u128(0x00000051_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L16_IR: windows_core::GUID = windows_core::GUID::from_u128(0x00000051_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8: windows_core::GUID = windows_core::GUID::from_u128(0x00000032_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8_CUSTOM: windows_core::GUID = windows_core::GUID::from_u128(0x00000032_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8_IR: windows_core::GUID = windows_core::GUID::from_u128(0x00000032_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_LPCM_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8032_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_Line21_BytePair: windows_core::GUID = windows_core::GUID::from_u128(0x6e8d4a22_310c_11d0_b79a_00aa003767a7);
pub const KSDATAFORMAT_SUBTYPE_Line21_GOPPacket: windows_core::GUID = windows_core::GUID::from_u128(0x6e8d4a23_310c_11d0_b79a_00aa003767a7);
pub const KSDATAFORMAT_SUBTYPE_MIDI: windows_core::GUID = windows_core::GUID::from_u128(0x1d262760_e957_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SUBTYPE_MIDI_BUS: windows_core::GUID = windows_core::GUID::from_u128(0x2ca15fa0_6cfe_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SUBTYPE_MJPG_CUSTOM: windows_core::GUID = windows_core::GUID::from_u128(0x47504a4d_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MJPG_DEPTH: windows_core::GUID = windows_core::GUID::from_u128(0x47504a4d_0004_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MJPG_IR: windows_core::GUID = windows_core::GUID::from_u128(0x47504a4d_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MPEG: windows_core::GUID = windows_core::GUID::from_u128(0x00000050_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Packet: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb80_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Payload: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb81_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Video: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb86_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802b_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_MPEGLAYER3: windows_core::GUID = windows_core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MPEG_HEAAC: windows_core::GUID = windows_core::GUID::from_u128(0x00001610_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MULAW: windows_core::GUID = windows_core::GUID::from_u128(0x00000007_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_NABTS: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76e2_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SUBTYPE_NABTS_FEC: windows_core::GUID = windows_core::GUID::from_u128(0xe757bca1_39ac_11d1_a9f5_00c04fbbde8f);
pub const KSDATAFORMAT_SUBTYPE_OVERLAY: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7f_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_RAW8: windows_core::GUID = windows_core::GUID::from_u128(0xca20d9a0_3e3e_11d1_9bf9_00c04fbbdebf);
pub const KSDATAFORMAT_SUBTYPE_RIFF: windows_core::GUID = windows_core::GUID::from_u128(0x4995daee_9ee6_11d0_a40e_00a0c9223196);
pub const KSDATAFORMAT_SUBTYPE_RIFFMIDI: windows_core::GUID = windows_core::GUID::from_u128(0x4995daf0_9ee6_11d0_a40e_00a0c9223196);
pub const KSDATAFORMAT_SUBTYPE_RIFFWAVE: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb8b_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_SDDS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8034_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_AC3_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b25_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b22_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b21_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b24_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b23_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_SUBPICTURE: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802d_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_TELETEXT: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76e3_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SUBTYPE_UNIVERSALMIDIPACKET: windows_core::GUID = windows_core::GUID::from_u128(0xfbffd49e_ce26_464a_9dfc_fee42456c81c);
pub const KSDATAFORMAT_SUBTYPE_VPVBI: windows_core::GUID = windows_core::GUID::from_u128(0x5a9b6a41_1a22_11d1_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_VPVideo: windows_core::GUID = windows_core::GUID::from_u128(0x5a9b6a40_1a22_11d1_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO2: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO3: windows_core::GUID = windows_core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO_LOSSLESS: windows_core::GUID = windows_core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_ANALOGAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x0482dee1_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_TYPE_ANALOGVIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x0482dde1_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_TYPE_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_AUXLine21Data: windows_core::GUID = windows_core::GUID::from_u128(0x670aea80_3a82_11d0_b79b_00aa003767a7);
pub const KSDATAFORMAT_TYPE_DVD_ENCRYPTED_PACK: windows_core::GUID = windows_core::GUID::from_u128(0xed0b916a_044d_11d1_aa78_00c04fc31d60);
pub const KSDATAFORMAT_TYPE_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x72178c23_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const KSDATAFORMAT_TYPE_MIDI: windows_core::GUID = windows_core::GUID::from_u128(0x7364696d_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_MPEG2_PES: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8020_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MPEG2_PROGRAM: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8022_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MPEG2_TRANSPORT: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8023_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MUSIC: windows_core::GUID = windows_core::GUID::from_u128(0xe725d360_62cc_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_TYPE_NABTS: windows_core::GUID = windows_core::GUID::from_u128(0xe757bca0_39ac_11d1_a9f5_00c04fbbde8f);
pub const KSDATAFORMAT_TYPE_STANDARD_ELEMENTARY_STREAM: windows_core::GUID = windows_core::GUID::from_u128(0x36523b11_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_STANDARD_PACK_HEADER: windows_core::GUID = windows_core::GUID::from_u128(0x36523b13_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_STANDARD_PES_PACKET: windows_core::GUID = windows_core::GUID::from_u128(0x36523b12_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_TEXT: windows_core::GUID = windows_core::GUID::from_u128(0x73747874_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_VBI: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76e1_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_TYPE_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
#[repr(C, packed(1))]
#[cfg(all(feature = "ks", feature = "mmeapi"))]
#[derive(Clone, Copy)]
pub struct KSDATAFORMAT_WAVEFORMATEX {
    pub DataFormat: super::KSDATARANGE,
    pub WaveFormatEx: super::WAVEFORMATEX,
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
    pub DataFormat: super::KSDATARANGE,
    pub WaveFormatExt: super::WAVEFORMATEXTENSIBLE,
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
    pub DataRange: super::KSDATARANGE,
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
    pub DataRange: super::KSDATARANGE,
    pub Technology: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDEVICE_PROFILE_INFO_0_0 {
    pub Info: KSCAMERA_PROFILE_INFO,
    pub Reserved: u32,
    pub ConcurrencyCount: u32,
    pub Concurrency: PKSCAMERA_PROFILE_CONCURRENCYINFO,
}
pub const KSDEVICE_PROFILE_TYPE_CAMERA: i32 = 1;
pub const KSDEVICE_PROFILE_TYPE_UNKNOWN: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct KSDS3D_BUFFER_ALL {
    pub Position: DS3DVECTOR,
    pub Velocity: DS3DVECTOR,
    pub InsideConeAngle: u32,
    pub OutsideConeAngle: u32,
    pub ConeOrientation: DS3DVECTOR,
    pub ConeOutsideVolume: i32,
    pub MinDistance: super::FLOAT,
    pub MaxDistance: super::FLOAT,
    pub Mode: u32,
}
#[cfg(feature = "minwindef")]
impl Default for KSDS3D_BUFFER_ALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDS3D_BUFFER_CONE_ANGLES {
    pub InsideConeAngle: u32,
    pub OutsideConeAngle: u32,
}
pub const KSDS3D_COEFF_COUNT: KSDS3D_HRTF_COEFF_FORMAT = 2;
pub const KSDS3D_FILTER_METHOD_COUNT: KSDS3D_HRTF_FILTER_METHOD = 2;
pub const KSDS3D_FILTER_QUALITY_COUNT: KSDS3D_HRTF_FILTER_QUALITY = 2;
pub type KSDS3D_HRTF_COEFF_FORMAT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KSDS3D_HRTF_INIT_MSG {
    pub Size: u32,
    pub Quality: KSDS3D_HRTF_FILTER_QUALITY,
    pub SampleRate: super::FLOAT,
    pub MaxFilterSize: u32,
    pub FilterTransientMuteLength: u32,
    pub FilterOverlapBufferLength: u32,
    pub OutputOverlapBufferLength: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDS3D_HRTF_PARAMS_MSG {
    pub Size: u32,
    pub Enabled: u32,
    pub SwapChannels: windows_core::BOOL,
    pub ZeroAzimuth: windows_core::BOOL,
    pub CrossFadeOutput: windows_core::BOOL,
    pub FilterSize: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KSDS3D_ITD_PARAMS {
    pub Channel: i32,
    pub VolSmoothScale: super::FLOAT,
    pub TotalDryAttenuation: super::FLOAT,
    pub TotalWetAttenuation: super::FLOAT,
    pub SmoothFrequency: i32,
    pub Delay: i32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KSDS3D_ITD_PARAMS_MSG {
    pub Enabled: u32,
    pub LeftParams: KSDS3D_ITD_PARAMS,
    pub RightParams: KSDS3D_ITD_PARAMS,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct KSDS3D_LISTENER_ALL {
    pub Position: DS3DVECTOR,
    pub Velocity: DS3DVECTOR,
    pub OrientFront: DS3DVECTOR,
    pub OrientTop: DS3DVECTOR,
    pub DistanceFactor: super::FLOAT,
    pub RolloffFactor: super::FLOAT,
    pub DopplerFactor: super::FLOAT,
}
#[cfg(feature = "minwindef")]
impl Default for KSDS3D_LISTENER_ALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct KSDS3D_LISTENER_ORIENTATION {
    pub Front: DS3DVECTOR,
    pub Top: DS3DVECTOR,
}
#[cfg(feature = "minwindef")]
impl Default for KSDS3D_LISTENER_ORIENTATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDSOUND_3D_MODE_DISABLE: i32 = 2;
pub const KSDSOUND_3D_MODE_HEADRELATIVE: i32 = 1;
pub const KSDSOUND_3D_MODE_NORMAL: i32 = 0;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct KSDSOUND_BUFFERDESC {
    pub Flags: u32,
    pub Control: u32,
    pub WaveFormatEx: super::WAVEFORMATEX,
}
pub const KSDSOUND_BUFFER_CTRL_3D: i32 = 1;
pub const KSDSOUND_BUFFER_CTRL_FREQUENCY: i32 = 2;
pub const KSDSOUND_BUFFER_CTRL_HRTF_3D: i32 = 1073741824;
pub const KSDSOUND_BUFFER_CTRL_PAN: i32 = 4;
pub const KSDSOUND_BUFFER_CTRL_POSITIONNOTIFY: i32 = 16;
pub const KSDSOUND_BUFFER_CTRL_VOLUME: i32 = 8;
pub const KSDSOUND_BUFFER_LOCHARDWARE: i32 = 4;
pub const KSDSOUND_BUFFER_LOCSOFTWARE: i32 = 8;
pub const KSDSOUND_BUFFER_PRIMARY: i32 = 1;
pub const KSDSOUND_BUFFER_STATIC: i32 = 2;
pub const KSEVENTSETID_AudioControlChange: windows_core::GUID = windows_core::GUID::from_u128(0xe85e9698_fa2f_11d1_95bd_00c04fb925d3);
pub const KSEVENTSETID_CameraAsyncControl: windows_core::GUID = windows_core::GUID::from_u128(0x22a11754_9701_4088_b33f_6b9cbc52df5e);
pub const KSEVENTSETID_CameraEvent: windows_core::GUID = windows_core::GUID::from_u128(0x7899b2e0_6b43_4964_9d2a_a21f4061f576);
pub const KSEVENTSETID_DynamicFormatChange: windows_core::GUID = windows_core::GUID::from_u128(0x162ac456_83d7_4239_96df_c75ffa138bc6);
pub const KSEVENTSETID_EXTDEV_Command: windows_core::GUID = windows_core::GUID::from_u128(0x109c7988_b3cb_11d2_b48e_006097b3391b);
pub const KSEVENTSETID_ExtendedCameraControl: windows_core::GUID = windows_core::GUID::from_u128(0x571c92c9_13a2_47e3_a649_d2a778166384);
pub const KSEVENTSETID_LoopedStreaming: windows_core::GUID = windows_core::GUID::from_u128(0x4682b940_c6ef_11d0_96d8_00aa0051e51d);
pub const KSEVENTSETID_SoundDetector: windows_core::GUID = windows_core::GUID::from_u128(0x69785c9b_fc2d_49d6_ac32_4799f87de9f6);
pub const KSEVENTSETID_Telephony: windows_core::GUID = windows_core::GUID::from_u128(0xb77f12b4_ceb4_4484_8d5e_52c1e7d8762d);
pub const KSEVENTSETID_VIDCAPTOSTI: windows_core::GUID = windows_core::GUID::from_u128(0xdb47de20_f628_11d1_ba41_00a0c90d2b05);
pub const KSEVENTSETID_VIDCAP_TVAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0651_28e4_11d0_a18c_00a0c9118956);
pub const KSEVENTSETID_VPNotify: windows_core::GUID = windows_core::GUID::from_u128(0x20c5598e_d3c8_11d0_8dfc_00c04fd7c08b);
pub const KSEVENTSETID_VPVBINotify: windows_core::GUID = windows_core::GUID::from_u128(0xec529b01_1a1f_11d1_bad9_00609744111a);
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
    pub EventData: super::KSEVENTDATA,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const KSINTERFACESETID_Media: windows_core::GUID = windows_core::GUID::from_u128(0x3a13eb40_30a7_11d0_a5d6_28db04c10000);
pub type KSINTERFACE_MEDIA = i32;
pub const KSINTERFACE_MEDIA_MUSIC: KSINTERFACE_MEDIA = 0;
pub const KSINTERFACE_MEDIA_WAVE_BUFFERED: KSINTERFACE_MEDIA = 1;
pub const KSINTERFACE_MEDIA_WAVE_QUEUED: KSINTERFACE_MEDIA = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSJACK_DESCRIPTION {
    pub ChannelMapping: u32,
    pub Color: u32,
    pub ConnectionType: EPcxConnectionType,
    pub GeoLocation: EPcxGeoLocation,
    pub GenLocation: EPcxGenLocation,
    pub PortConnection: EPxcPortConnection,
    pub IsConnected: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSJACK_DESCRIPTION2 {
    pub DeviceStateInfo: u32,
    pub JackCapabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSJACK_DESCRIPTION3 {
    pub ConfigId: u32,
}
pub type KSJACK_SINK_CONNECTIONTYPE = i32;
pub const KSJACK_SINK_CONNECTIONTYPE_DISPLAYPORT: KSJACK_SINK_CONNECTIONTYPE = 1;
pub const KSJACK_SINK_CONNECTIONTYPE_HDMI: KSJACK_SINK_CONNECTIONTYPE = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KSJACK_SINK_INFORMATION {
    pub ConnType: KSJACK_SINK_CONNECTIONTYPE,
    pub ManufacturerId: u16,
    pub ProductId: u16,
    pub AudioLatency: u16,
    pub HDCPCapable: windows_core::BOOL,
    pub AICapable: windows_core::BOOL,
    pub SinkDescriptionLength: u8,
    pub SinkDescription: [u16; 32],
    pub PortId: super::LUID,
}
#[cfg(feature = "winnt")]
impl Default for KSJACK_SINK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSMEDIUMSETID_MidiBus: windows_core::GUID = windows_core::GUID::from_u128(0x05908040_3246_11d0_a5d6_28db04c10000);
pub const KSMEDIUMSETID_VPBus: windows_core::GUID = windows_core::GUID::from_u128(0xa18c15ec_ce43_11d0_abe7_00a0c9223196);
pub const KSMETHODSETID_Wavetable: windows_core::GUID = windows_core::GUID::from_u128(0xdcef31eb_d907_11d0_9583_00c04fb925d3);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMIDILOOPED_BUFFER {
    pub BufferAddress: *mut core::ffi::c_void,
    pub ActualBufferSize: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSMIDILOOPED_BUFFER_PROPERTY {
    pub Property: super::KSPROPERTY,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMIDILOOPED_EVENT {
    pub WriteEvent: super::HANDLE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMIDILOOPED_EVENT2 {
    pub WriteEvent: super::HANDLE,
    pub ReadEvent: super::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMIDILOOPED_REGISTERS {
    pub WritePosition: *mut core::ffi::c_void,
    pub ReadPosition: *mut core::ffi::c_void,
}
pub const KSMPEGVIDMODE_LTRBOX: i32 = 2;
pub const KSMPEGVIDMODE_PANSCAN: i32 = 1;
pub const KSMPEGVIDMODE_SCALE: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub Property: super::KSPROPERTY,
    pub MultipleItem: super::KSMULTIPLE_ITEM,
}
#[cfg(feature = "ks")]
impl Default for KSMULTIPLE_DATA_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMUSICFORMAT {
    pub TimeDeltaMs: u32,
    pub ByteCount: u32,
}
pub const KSMUSIC_TECHNOLOGY_FMSYNTH: windows_core::GUID = windows_core::GUID::from_u128(0x252c5c80_62e9_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_PORT: windows_core::GUID = windows_core::GUID::from_u128(0x86c92e60_62e8_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_SQSYNTH: windows_core::GUID = windows_core::GUID::from_u128(0x0ecf4380_62e9_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_SWSYNTH: windows_core::GUID = windows_core::GUID::from_u128(0x37407736_3620_11d1_85d3_0000f8754380);
pub const KSMUSIC_TECHNOLOGY_WAVETABLE: windows_core::GUID = windows_core::GUID::from_u128(0x394ec7c0_62e9_11cf_a5d6_28db04c10000);
pub const KSNODEPIN_DEMUX_IN: i32 = 0;
pub const KSNODEPIN_DEMUX_OUT: i32 = 1;
pub const KSNODEPIN_STANDARD_IN: i32 = 1;
pub const KSNODEPIN_STANDARD_OUT: i32 = 0;
pub const KSNODEPIN_SUM_MUX_IN: i32 = 1;
pub const KSNODEPIN_SUM_MUX_OUT: i32 = 0;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY {
    pub Property: super::KSPROPERTY,
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
pub const KSNODETYPE_1394_DA_STREAM: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_1394_DV_STREAM_SOUNDTRACK: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_3D_EFFECTS: windows_core::GUID = windows_core::GUID::from_u128(0x55515860_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ADC: windows_core::GUID = windows_core::GUID::from_u128(0x4d837fe0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_AGC: windows_core::GUID = windows_core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ANALOG_CONNECTOR: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_ANALOG_TAPE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_AUDIO_ENGINE: windows_core::GUID = windows_core::GUID::from_u128(0x35caf6e4_f3b3_4168_bb4b_55e77a461c7e);
pub const KSNODETYPE_AUDIO_KEYWORDDETECTOR: windows_core::GUID = windows_core::GUID::from_u128(0x3817e0b8_df58_4375_b669_c49634331f9d);
pub const KSNODETYPE_AUDIO_LOOPBACK: windows_core::GUID = windows_core::GUID::from_u128(0x8f42c0b2_91ce_4bcf_9ccd_0e599037ab35);
pub const KSNODETYPE_AUDIO_MODULE: windows_core::GUID = windows_core::GUID::from_u128(0x45aab42e_caeb_4052_8aa9_b38cb5109619);
pub const KSNODETYPE_BIDIRECTIONAL_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CABLE_TUNER_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ee_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CD_PLAYER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CHORUS: windows_core::GUID = windows_core::GUID::from_u128(0x20173f20_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_COMMUNICATION_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DAC: windows_core::GUID = windows_core::GUID::from_u128(0x507ae360_c554_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DAT_IO_DIGITAL_AUDIO_TAPE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DCC_IO_DIGITAL_COMPACT_CASSETTE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DELAY: windows_core::GUID = windows_core::GUID::from_u128(0x144981e0_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DEMUX: windows_core::GUID = windows_core::GUID::from_u128(0xc0eb67d4_e807_11d0_958a_00c04fb925d3);
pub const KSNODETYPE_DESKTOP_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DESKTOP_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DEV_SPECIFIC: windows_core::GUID = windows_core::GUID::from_u128(0x941c7ac0_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DIGITAL_AUDIO_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DISPLAYPORT_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xe47e4031_3ea6_418d_8f9b_b73843ccba97);
pub const KSNODETYPE_DOWN_LINE_PHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ee3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DRM_DESCRAMBLE: windows_core::GUID = windows_core::GUID::from_u128(0xffbb6e3f_ccfe_4d84_90d9_421418b03a8e);
pub const KSNODETYPE_DSS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ef_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DVD_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220eb_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DYN_RANGE_COMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x08c8a6a8_601f_4af8_8793_d905ff4ca97d);
pub const KSNODETYPE_ECHO_CANCELING_SPEAKERPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_ECHO_SUPPRESSING_SPEAKERPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EMBEDDED_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EQUALIZATION_NOISE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EQUALIZER: windows_core::GUID = windows_core::GUID::from_u128(0x9d41b4a0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_EXTERNAL_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_FM_RX: windows_core::GUID = windows_core::GUID::from_u128(0x834a733c_f485_41c0_a62b_513025014e40);
pub const KSNODETYPE_HANDSET: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HDMI_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xd1b9cc2a_f519_417f_91c9_55fa65481001);
pub const KSNODETYPE_HEADPHONES: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HEADSET: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HEAD_MOUNTED_DISPLAY_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_INPUT_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LEGACY_AUDIO_CONNECTOR: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LEVEL_CALIBRATION_NOISE_SOURCE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LINE_CONNECTOR: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LOUDNESS: windows_core::GUID = windows_core::GUID::from_u128(0x41887440_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_LOW_FREQUENCY_EFFECTS_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MICROPHONE_ARRAY: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MIDI_ELEMENT: windows_core::GUID = windows_core::GUID::from_u128(0x01c6fe66_6e48_4c65_ac9b_52db5d656c7e);
pub const KSNODETYPE_MIDI_JACK: windows_core::GUID = windows_core::GUID::from_u128(0x265e0c3f_fa39_4df3_ab04_be01b91e299a);
pub const KSNODETYPE_MINIDISK: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MULTITRACK_RECORDER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220f2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x02b223c0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_MUX: windows_core::GUID = windows_core::GUID::from_u128(0x2ceaf780_c556_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_NOISE_SUPPRESS: windows_core::GUID = windows_core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const KSNODETYPE_OMNI_DIRECTIONAL_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_OUTPUT_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PARAMETRIC_EQUALIZER: windows_core::GUID = windows_core::GUID::from_u128(0x19bb3a6a_ce2b_4442_87ec_6727c3cab477);
pub const KSNODETYPE_PEAKMETER: windows_core::GUID = windows_core::GUID::from_u128(0xa085651e_5f0d_4b36_a869_d195d6ab4b9e);
pub const KSNODETYPE_PERSONAL_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PHONE_LINE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ee1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PHONOGRAPH: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e8_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PROCESSING_MICROPHONE_ARRAY: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PROLOGIC_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0x831c2c80_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_PROLOGIC_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x8074c5b2_3c66_11d2_b45a_3078302c2030);
pub const KSNODETYPE_RADIO_RECEIVER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220f0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_RADIO_TRANSMITTER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220f1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_REVERB: windows_core::GUID = windows_core::GUID::from_u128(0xef0328e0_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ROOM_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SATELLITE_RECEIVER_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ed_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPDIF_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKERPHONE_NO_ECHO_REDUCTION: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKERS_STATIC_JACK: windows_core::GUID = windows_core::GUID::from_u128(0x28e04f87_4dbe_4f8d_8589_025d209dfb4a);
pub const KSNODETYPE_SRC: windows_core::GUID = windows_core::GUID::from_u128(0x9db7b9e0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_STEREO_WIDE: windows_core::GUID = windows_core::GUID::from_u128(0xa9e69800_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SUM: windows_core::GUID = windows_core::GUID::from_u128(0xda441a60_c556_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SUPERMIX: windows_core::GUID = windows_core::GUID::from_u128(0xe573adc0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SYNTHESIZER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220f3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TELEPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ee2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TELEPHONY_BIDI: windows_core::GUID = windows_core::GUID::from_u128(0x686d7cc0_d903_4258_b443_3a3d3580741c);
pub const KSNODETYPE_TELEPHONY_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ee0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TONE: windows_core::GUID = windows_core::GUID::from_u128(0x7607e580_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_TV_TUNER_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ec_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_UPDOWN_MIX: windows_core::GUID = windows_core::GUID::from_u128(0xb7edc5cf_7b63_4ee2_a100_29ee2cb6b2de);
pub const KSNODETYPE_VCR_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e9_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_CAMERA_TERMINAL: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_DISC_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ea_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_INPUT_MTT: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_INPUT_TERMINAL: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_OUTPUT_MTT: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e8_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_OUTPUT_TERMINAL: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_PROCESSING: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_SELECTOR: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_STREAMING: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x3a5acc00_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNOTIFICATIONID_AudioModule: windows_core::GUID = windows_core::GUID::from_u128(0x9c2220f0_d9a6_4d5c_a036_573857fd50d2);
pub const KSNOTIFICATIONID_SoundDetector: windows_core::GUID = windows_core::GUID::from_u128(0x6389d844_bb32_4c4c_a802_f4b4b77afead);
pub const KSPROPERTYSETID_ExtendedCameraControl: windows_core::GUID = windows_core::GUID::from_u128(0x1cb79112_c0d2_4213_9ca6_cd4fdb927972);
pub const KSPROPERTYSETID_NetworkCameraControl: windows_core::GUID = windows_core::GUID::from_u128(0x0e780f09_5745_4e3a_bc9f_f226ea43a6ec);
pub const KSPROPERTYSETID_PerFrameSettingControl: windows_core::GUID = windows_core::GUID::from_u128(0xf1f3e261_dee6_4537_bff5_ee206db54aac);
pub const KSPROPERTYSETID_WindowsCameraEffect: windows_core::GUID = windows_core::GUID::from_u128(0x1666d655_21a6_4982_9728_52c39e869f90);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    pub InterleavedCapSupported: u32,
}
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE: KSPROPERTY_ALLOCATOR_CONTROL = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    pub InterleavedCapPossible: u32,
}
pub const KSPROPERTY_ALLOCATOR_CONTROL_HONOR_COUNT: KSPROPERTY_ALLOCATOR_CONTROL = 0;
pub const KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE: KSPROPERTY_ALLOCATOR_CONTROL = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const KSPROPERTY_AUDIO_BUFFER_DURATION: i32 = 1;
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
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ABSOLUTE: i32 = 0;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ASYNCHRONOUS: i32 = 4;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_AUTO: i32 = 1;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_MANUAL: i32 = 2;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_RELATIVE: i32 = 16;
pub type KSPROPERTY_CAMERACONTROL_FLASH = i32;
pub const KSPROPERTY_CAMERACONTROL_FLASH_AUTO: i32 = 2;
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_AUTO: i32 = 1;
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_MANUAL: i32 = 2;
pub const KSPROPERTY_CAMERACONTROL_FLASH_OFF: i32 = 0;
pub const KSPROPERTY_CAMERACONTROL_FLASH_ON: i32 = 1;
pub const KSPROPERTY_CAMERACONTROL_FLASH_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_FLASH = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_FLASH_S {
    pub Flash: u32,
    pub Capabilities: u32,
}
pub const KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH: KSPROPERTY_VIDCAP_CAMERACONTROL = 18;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    pub Property: super::KSPROPERTY,
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
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_EXCLUSIVE_WITH_RECORD: i32 = 1;
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    pub Capabilities: u32,
    pub Reserved0: u32,
}
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_SEQUENCE_EXCLUSIVE_WITH_RECORD: i32 = 2;
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
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_NODE_S2 {
    pub NodeProperty: super::KSP_NODE,
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
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_EXPOSURE: i32 = 512;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_FOCUS: i32 = 256;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_WB: i32 = 1024;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONVERGEMODE: i32 = 1073741824;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_ASYNC: u32 = 2147483648;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_AUTO: i32 = 1;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_MANUAL: i32 = 2;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    pub FocusRect: super::RECT,
    pub AutoFocusLock: windows_core::BOOL,
    pub AutoExposureLock: windows_core::BOOL,
    pub AutoWhitebalanceLock: windows_core::BOOL,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub FocusRect: super::RECT,
}
#[cfg(all(feature = "ks", feature = "windef"))]
impl Default for KSPROPERTY_CAMERACONTROL_S_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_TILT: KSPROPERTY_VIDCAP_CAMERACONTROL = 1;
pub const KSPROPERTY_CAMERACONTROL_TILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = 11;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_AUTO: i32 = 4;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_AUTO: i32 = 1;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_MANUAL: i32 = 2;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_HIGH: i32 = 1;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_LOW: i32 = 3;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_MEDIUM: i32 = 2;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_OFF: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub type KSPROPERTY_COMPOSIT_ON = windows_core::BOOL;
pub type KSPROPERTY_COPYPROT = i32;
pub const KSPROPERTY_COPY_MACROVISION: KSPROPERTY_COPYPROT = 5;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_ACTIVE_S {
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub Direction: super::KSPIN_DATAFLOW,
    pub Index: u32,
    pub PinType: u32,
    pub RelatedPinIndex: u32,
    pub Medium: super::KSPIN_MEDIUM,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub NodeProperty: super::KSP_NODE,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub Property: super::KSPROPERTY,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_EXTXPORT_S_0_0 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    pub MetadataItems: u32,
    pub Size: u32,
    pub PTZStatus: windows_core::BOOL,
    pub Events: windows_core::BOOL,
    pub Analytics: windows_core::BOOL,
    pub Reserved: windows_core::BOOL,
}
pub type KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = i32;
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE_EVENTSINFO: KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = 0;
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTP: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub NodeProperty: super::KSP_NODE,
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
    pub Property: super::KSPROPERTY,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub NodeProperty: super::KSP_NODE,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub ModesSupported: u32,
    pub VideoMedium: super::KSPIN_MEDIUM,
    pub TVAudioMedium: super::KSPIN_MEDIUM,
    pub RadioAudioMedium: super::KSPIN_MEDIUM,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub IFMedium: super::KSPIN_MEDIUM,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub NetworkType: windows_core::GUID,
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
    pub Property: super::KSPROPERTY,
    pub fSupportsHardwareAssistedScanning: windows_core::BOOL,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub AutoDetect: windows_core::BOOL,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub Capabilities: u32,
    pub InputMedium: super::KSPIN_MEDIUM,
    pub OutputMedium: super::KSPIN_MEDIUM,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::SIZE,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::SIZE,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_AUTO: i32 = 1;
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_MANUAL: i32 = 2;
pub const KSPROPERTY_VIDEOPROCAMP_GAIN: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 9;
pub const KSPROPERTY_VIDEOPROCAMP_GAMMA: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 5;
pub const KSPROPERTY_VIDEOPROCAMP_HUE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = 2;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S {
    pub NodeProperty: super::KSP_NODE,
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
    pub NodeProperty: super::KSP_NODE,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
pub const KSPROPSETID_AC3: windows_core::GUID = windows_core::GUID::from_u128(0xbfabe720_6e1f_11d0_bcf2_444553540000);
pub const KSPROPSETID_Audio: windows_core::GUID = windows_core::GUID::from_u128(0x45ffaaa0_6e1b_11d0_bcf2_444553540000);
pub const KSPROPSETID_AudioBufferDuration: windows_core::GUID = windows_core::GUID::from_u128(0x4e73c07f_23cc_4955_a7ea_3da502496290);
pub const KSPROPSETID_AudioDecoderOut: windows_core::GUID = windows_core::GUID::from_u128(0x6ca6e020_43bd_11d0_bd6a_003505c103a9);
pub const KSPROPSETID_AudioEngine: windows_core::GUID = windows_core::GUID::from_u128(0x3a2f82dc_886f_4baa_9eb4_082b9025c536);
pub const KSPROPSETID_AudioLoopback: windows_core::GUID = windows_core::GUID::from_u128(0xb3648bc8_5b91_468a_b94d_f4641250917c);
pub const KSPROPSETID_AudioModule: windows_core::GUID = windows_core::GUID::from_u128(0xc034fdb0_ff75_47c8_aa3c_ee46716b50c6);
pub const KSPROPSETID_AudioPosture: windows_core::GUID = windows_core::GUID::from_u128(0xa3fb7b0d_474e_4f51_a379_51282dd4fa8f);
pub const KSPROPSETID_AudioResourceManagement: windows_core::GUID = windows_core::GUID::from_u128(0xd0b305e1_b2cc_484c_8f23_e5d28ad9cf88);
pub const KSPROPSETID_AudioSignalProcessing: windows_core::GUID = windows_core::GUID::from_u128(0x4f67b528_30c9_40de_b2fb_859ddd1f3470);
pub const KSPROPSETID_Bibliographic: windows_core::GUID = windows_core::GUID::from_u128(0x07ba150e_e2b1_11d0_ac17_00a0c9223196);
pub const KSPROPSETID_BtAudio: windows_core::GUID = windows_core::GUID::from_u128(0x7fa06c40_b8f6_4c7e_8556_e8c33a12e54d);
pub const KSPROPSETID_CopyProt: windows_core::GUID = windows_core::GUID::from_u128(0x0e8a0a40_6aef_11d0_9ed0_00a024ca19b3);
pub const KSPROPSETID_Cyclic: windows_core::GUID = windows_core::GUID::from_u128(0x3ffeaea0_2bee_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_DirectSound3DBuffer: windows_core::GUID = windows_core::GUID::from_u128(0x437b3411_d060_11d0_8583_00c04fd9baf3);
pub const KSPROPSETID_DirectSound3DListener: windows_core::GUID = windows_core::GUID::from_u128(0x437b3414_d060_11d0_8583_00c04fd9baf3);
pub const KSPROPSETID_DrmAudioStream: windows_core::GUID = windows_core::GUID::from_u128(0x2f2c8ddd_4198_4fac_ba29_61bb05b7de06);
pub const KSPROPSETID_DvdSubPic: windows_core::GUID = windows_core::GUID::from_u128(0xac390460_43af_11d0_bd6a_003505c103a9);
pub const KSPROPSETID_FMRXControl: windows_core::GUID = windows_core::GUID::from_u128(0x947bba3a_e8ee_4786_90c4_8428185f05be);
pub const KSPROPSETID_FMRXTopology: windows_core::GUID = windows_core::GUID::from_u128(0x0c46ce8f_dc2d_4204_9dc9_f58963366563);
pub const KSPROPSETID_Hrtf3d: windows_core::GUID = windows_core::GUID::from_u128(0xb66decb0_a083_11d0_851e_00c04fd9baf3);
pub const KSPROPSETID_InterleavedAudio: windows_core::GUID = windows_core::GUID::from_u128(0xe9ebe550_d619_4c0a_976b_7062322b3006);
pub const KSPROPSETID_Itd3d: windows_core::GUID = windows_core::GUID::from_u128(0x6429f090_9fd9_11d0_a75b_00a0c90365e3);
pub const KSPROPSETID_Jack: windows_core::GUID = windows_core::GUID::from_u128(0x4509f757_2d46_4637_8e62_ce7db944f57b);
pub const KSPROPSETID_MPEG4_MediaType_Attributes: windows_core::GUID = windows_core::GUID::from_u128(0xff6c4bfa_07a9_4c7b_a237_672f9d68065f);
pub const KSPROPSETID_MidiLoopedStreaming: windows_core::GUID = windows_core::GUID::from_u128(0x1f306ba6_fd9b_427a_bcb3_27cbcf0e0f19);
pub const KSPROPSETID_Mpeg2Vid: windows_core::GUID = windows_core::GUID::from_u128(0xc8e11b60_0cc9_11d0_bd69_003505c103a9);
pub const KSPROPSETID_OverlayUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x490ea5cf_7681_11d1_a21c_00a0c9223196);
pub const KSPROPSETID_RtAudio: windows_core::GUID = windows_core::GUID::from_u128(0xa855a48c_2f78_4729_9051_1968746b9eef);
pub const KSPROPSETID_SoundDetector: windows_core::GUID = windows_core::GUID::from_u128(0x113c425e_fd17_4057_b422_ed4074f1afdf);
pub const KSPROPSETID_SoundDetector2: windows_core::GUID = windows_core::GUID::from_u128(0xfe07e322_450c_4bd5_84ca_a948500ea6aa);
pub const KSPROPSETID_TSRateChange: windows_core::GUID = windows_core::GUID::from_u128(0xa503c5c0_1d1d_11d1_ad80_444553540000);
pub const KSPROPSETID_TelephonyControl: windows_core::GUID = windows_core::GUID::from_u128(0xb6df7eb1_d099_489f_a6a0_c0106f0887a7);
pub const KSPROPSETID_TelephonyTopology: windows_core::GUID = windows_core::GUID::from_u128(0xabf25c7e_0e64_4e32_b190_d0f6d7c53e97);
pub const KSPROPSETID_TopologyNode: windows_core::GUID = windows_core::GUID::from_u128(0x45ffaaa1_6e1b_11d0_bcf2_444553540000);
pub const KSPROPSETID_VBICAP_PROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0xf162c607_7b35_496f_ad7f_2dca3b46b718);
pub const KSPROPSETID_VBICodecFiltering: windows_core::GUID = windows_core::GUID::from_u128(0xcafeb0ca_8715_11d0_bd6a_0035c0edbabe);
pub const KSPROPSETID_VPConfig: windows_core::GUID = windows_core::GUID::from_u128(0xbc29a660_30e3_11d0_9e69_00c04fd7c15b);
pub const KSPROPSETID_VPVBIConfig: windows_core::GUID = windows_core::GUID::from_u128(0xec529b00_1a1f_11d1_bad9_00609744111a);
pub const KSPROPSETID_VramCapture: windows_core::GUID = windows_core::GUID::from_u128(0xe73face3_2880_4902_b799_88d0cd634e0f);
pub const KSPROPSETID_Wave: windows_core::GUID = windows_core::GUID::from_u128(0x924e54b0_630f_11cf_ada7_08003e30494a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRTAUDIO_BUFFER {
    pub BufferAddress: *mut core::ffi::c_void,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRTAUDIO_BUFFER32 {
    pub BufferAddress: u32,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY {
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRTAUDIO_GETREADPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub PerformanceCounterValue: u64,
    pub MoreData: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRTAUDIO_HWLATENCY {
    pub FifoSize: u32,
    pub ChipsetDelay: u32,
    pub CodecDelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRTAUDIO_HWREGISTER {
    pub Register: *mut core::ffi::c_void,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
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
    pub Property: super::KSPROPERTY,
    pub NotificationEvent: super::HANDLE,
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
    pub Property: super::KSPROPERTY,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRTAUDIO_PACKETVREGISTER {
    pub CompletedPacketCount: super::PULONG64,
    pub CompletedPacketQPC: super::PULONG64,
    pub CompletedPacketHash: super::PULONG64,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    pub Property: super::KSPROPERTY,
    pub BaseAddress: *mut core::ffi::c_void,
}
#[cfg(feature = "ks")]
impl Default for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRTAUDIO_SETWRITEPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub EosPacketLength: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSSOUNDDETECTORPROPERTY {
    pub Property: super::KSPROPERTY,
    pub EventId: windows_core::GUID,
}
#[cfg(feature = "ks")]
impl Default for KSSOUNDDETECTORPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSTELEPHONY_CALLCONTROL {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallControlOp: TELEPHONY_CALLCONTROLOP,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSTELEPHONY_CALLINFO {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallState: TELEPHONY_CALLSTATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSTELEPHONY_PROVIDERCHANGE {
    pub CallType: TELEPHONY_CALLTYPE,
    pub ProviderChangeOp: TELEPHONY_PROVIDERCHANGEOP,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSTOPOLOGY_ENDPOINTIDPAIR {
    pub RenderEndpoint: KSTOPOLOGY_ENDPOINTID,
    pub CaptureEndpoint: KSTOPOLOGY_ENDPOINTID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSVPMAXPIXELRATE {
    pub Size: KS_AMVPSIZE,
    pub MaxPixelsPerSecond: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSVPSIZE_PROP {
    pub Property: super::KSPROPERTY,
    pub Size: KS_AMVPSIZE,
}
#[cfg(feature = "ks")]
impl Default for KSVPSIZE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSVPSURFACEPARAMS {
    pub dwPitch: u32,
    pub dwXOrigin: u32,
    pub dwYOrigin: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct KSWAVETABLE_WAVE_DESC {
    pub Identifier: super::KSIDENTIFIER,
    pub Size: u32,
    pub Looped: windows_core::BOOL,
    pub LoopPoint: u32,
    pub InROM: windows_core::BOOL,
    pub Format: super::KSDATARANGE,
}
#[cfg(feature = "ks")]
impl Default for KSWAVETABLE_WAVE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSWAVE_BUFFER {
    pub Attributes: u32,
    pub BufferSize: u32,
    pub BufferAddress: *mut core::ffi::c_void,
}
pub const KSWAVE_BUFFER_ATTRIBUTEF_LOOPING: i32 = 1;
pub const KSWAVE_BUFFER_ATTRIBUTEF_STATIC: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSWAVE_COMPATCAPS {
    pub ulDeviceType: u32,
}
pub const KSWAVE_COMPATCAPS_INPUT: i32 = 0;
pub const KSWAVE_COMPATCAPS_OUTPUT: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSWAVE_VOLUME {
    pub LeftAttenuation: i32,
    pub RightAttenuation: i32,
}
pub const KS_47NABTS_SCALER: f64 = 4.714286312925246;
pub const KS_AMCONTROL_COLORINFO_PRESENT: i32 = 128;
pub const KS_AMCONTROL_PAD_TO_16x9: i32 = 4;
pub const KS_AMCONTROL_PAD_TO_4x3: i32 = 2;
pub const KS_AMCONTROL_USED: i32 = 1;
pub type KS_AMPixAspectRatio = i32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AMVPDATAINFO {
    pub dwSize: u32,
    pub dwMicrosecondsPerField: u32,
    pub amvpDimInfo: KS_AMVPDIMINFO,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub bEnableDoubleClock: windows_core::BOOL,
    pub bEnableVACT: windows_core::BOOL,
    pub bDataIsInterlaced: windows_core::BOOL,
    pub lHalfLinesOdd: i32,
    pub bFieldPolarityInverted: windows_core::BOOL,
    pub dwNumLinesInVREF: u32,
    pub lHalfLinesEven: i32,
    pub dwReserved1: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AMVPDIMINFO {
    pub dwFieldWidth: u32,
    pub dwFieldHeight: u32,
    pub dwVBIWidth: u32,
    pub dwVBIHeight: u32,
    pub rcValidRegion: super::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AM_ExactRateChange {
    pub OutputZeroTime: super::REFERENCE_TIME,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AM_SimpleRateChange {
    pub StartTime: super::REFERENCE_TIME,
    pub Rate: i32,
}
pub type KS_AM_Step = u32;
pub const KS_AM_UseNewCSSKey: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_ANALOGVIDEOINFO {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: super::REFERENCE_TIME,
}
pub type KS_AnalogVideoStandard = i32;
pub const KS_AnalogVideo_NTSC_433: KS_AnalogVideoStandard = 4;
pub const KS_AnalogVideo_NTSC_M: KS_AnalogVideoStandard = 1;
pub const KS_AnalogVideo_NTSC_M_J: KS_AnalogVideoStandard = 2;
pub const KS_AnalogVideo_NTSC_Mask: i32 = 7;
pub const KS_AnalogVideo_None: KS_AnalogVideoStandard = 0;
pub const KS_AnalogVideo_PAL_60: KS_AnalogVideoStandard = 2048;
pub const KS_AnalogVideo_PAL_B: KS_AnalogVideoStandard = 16;
pub const KS_AnalogVideo_PAL_D: KS_AnalogVideoStandard = 32;
pub const KS_AnalogVideo_PAL_G: KS_AnalogVideoStandard = 64;
pub const KS_AnalogVideo_PAL_H: KS_AnalogVideoStandard = 128;
pub const KS_AnalogVideo_PAL_I: KS_AnalogVideoStandard = 256;
pub const KS_AnalogVideo_PAL_M: KS_AnalogVideoStandard = 512;
pub const KS_AnalogVideo_PAL_Mask: i32 = 1052656;
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
pub const KS_AnalogVideo_SECAM_Mask: i32 = 1044480;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const KS_BI_BITFIELDS: i32 = 3;
pub const KS_BI_JPEG: i32 = 4;
pub const KS_BI_RGB: i32 = 0;
pub const KS_BI_RLE4: i32 = 2;
pub const KS_BI_RLE8: i32 = 1;
pub const KS_CAMERACONTROL_ASYNC_RESET: KS_CameraControlAsyncOperation = 3;
pub const KS_CAMERACONTROL_ASYNC_START: KS_CameraControlAsyncOperation = 1;
pub const KS_CAMERACONTROL_ASYNC_STOP: KS_CameraControlAsyncOperation = 2;
pub const KS_CAPTURE_ALLOC_INVALID: CAPTURE_MEMORY_ALLOCATION_FLAGS = 0;
pub const KS_CAPTURE_ALLOC_SECURE_BUFFER: CAPTURE_MEMORY_ALLOCATION_FLAGS = 16;
pub const KS_CAPTURE_ALLOC_SYSTEM: CAPTURE_MEMORY_ALLOCATION_FLAGS = 1;
pub const KS_CAPTURE_ALLOC_SYSTEM_AGP: CAPTURE_MEMORY_ALLOCATION_FLAGS = 4;
pub const KS_CAPTURE_ALLOC_VRAM: CAPTURE_MEMORY_ALLOCATION_FLAGS = 2;
pub const KS_CAPTURE_ALLOC_VRAM_MAPPED: CAPTURE_MEMORY_ALLOCATION_FLAGS = 8;
pub const KS_CC_SUBSTREAM_EVEN: i32 = 2;
pub const KS_CC_SUBSTREAM_FIELD1_MASK: i32 = 240;
pub const KS_CC_SUBSTREAM_FIELD2_MASK: i32 = 7936;
pub const KS_CC_SUBSTREAM_ODD: i32 = 1;
pub const KS_CC_SUBSTREAM_SERVICE_CC1: i32 = 16;
pub const KS_CC_SUBSTREAM_SERVICE_CC2: i32 = 32;
pub const KS_CC_SUBSTREAM_SERVICE_CC3: i32 = 256;
pub const KS_CC_SUBSTREAM_SERVICE_CC4: i32 = 512;
pub const KS_CC_SUBSTREAM_SERVICE_T1: i32 = 64;
pub const KS_CC_SUBSTREAM_SERVICE_T2: i32 = 128;
pub const KS_CC_SUBSTREAM_SERVICE_T3: i32 = 1024;
pub const KS_CC_SUBSTREAM_SERVICE_T4: i32 = 2048;
pub const KS_CC_SUBSTREAM_SERVICE_XDS: i32 = 4096;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_COLCON {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
}
impl KS_COLCON {
    pub fn emph1col(&self) -> u8 {
        (self._bitfield1 << 4) >> 4
    }
    pub fn set_emph1col(&mut self, value: u8) {
        self._bitfield1 = (self._bitfield1 & !15) | (value & 15);
    }
    pub fn emph2col(&self) -> u8 {
        self._bitfield1 >> 4
    }
    pub fn set_emph2col(&mut self, value: u8) {
        self._bitfield1 = (self._bitfield1 & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn backcol(&self) -> u8 {
        (self._bitfield2 << 4) >> 4
    }
    pub fn set_backcol(&mut self, value: u8) {
        self._bitfield2 = (self._bitfield2 & !15) | (value & 15);
    }
    pub fn patcol(&self) -> u8 {
        self._bitfield2 >> 4
    }
    pub fn set_patcol(&mut self, value: u8) {
        self._bitfield2 = (self._bitfield2 & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn emph1con(&self) -> u8 {
        (self._bitfield3 << 4) >> 4
    }
    pub fn set_emph1con(&mut self, value: u8) {
        self._bitfield3 = (self._bitfield3 & !15) | (value & 15);
    }
    pub fn emph2con(&self) -> u8 {
        self._bitfield3 >> 4
    }
    pub fn set_emph2con(&mut self, value: u8) {
        self._bitfield3 = (self._bitfield3 & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn backcon(&self) -> u8 {
        (self._bitfield4 << 4) >> 4
    }
    pub fn set_backcon(&mut self, value: u8) {
        self._bitfield4 = (self._bitfield4 & !15) | (value & 15);
    }
    pub fn patcon(&self) -> u8 {
        self._bitfield4 >> 4
    }
    pub fn set_patcon(&mut self, value: u8) {
        self._bitfield4 = (self._bitfield4 & !(15 << 4)) | ((value & 15) << 4);
    }
}
pub const KS_COPYPROTECT_RestrictDuplication: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub DataFormat: super::KSDATARANGE,
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
    pub DataFormat: super::KSDATARANGE,
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
    pub DataFormat: super::KSDATARANGE,
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
    pub DataFormat: super::KSDATARANGE,
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
    pub DataFormat: super::KSDATARANGE,
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
    pub DataFormat: super::KSDATARANGE,
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
    pub DataFormat: super::KSDATARANGE,
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
    pub DataRange: super::KSDATARANGE,
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
    pub DataRange: super::KSDATARANGE,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
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
    pub DataRange: super::KSDATARANGE,
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
    pub DataRange: super::KSDATARANGE,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
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
    pub DataRange: super::KSDATARANGE,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
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
    pub DataRange: super::KSDATARANGE,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
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
    pub DataRange: super::KSDATARANGE,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
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
    pub DataRange: super::KSDATARANGE,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
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
    pub DataRange: super::KSDATARANGE,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KS_DVDCOPY_DISCKEY {
    pub DiscKey: [u8; 2048],
}
impl Default for KS_DVDCOPY_DISCKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_DVDCOPY_SET_COPY_STATE {
    pub DVDCopyState: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const KS_DVD_CGMS_COPY_ONCE: i32 = 16;
pub const KS_DVD_CGMS_COPY_PERMITTED: i32 = 0;
pub const KS_DVD_CGMS_COPY_PROTECT_MASK: i32 = 24;
pub const KS_DVD_CGMS_NO_COPY: i32 = 24;
pub const KS_DVD_CGMS_RESERVED_MASK: i32 = 120;
pub const KS_DVD_COPYRIGHTED: i32 = 64;
pub const KS_DVD_COPYRIGHT_MASK: i32 = 64;
pub const KS_DVD_NOT_COPYRIGHTED: i32 = 0;
pub const KS_DVD_SECTOR_NOT_PROTECTED: i32 = 0;
pub const KS_DVD_SECTOR_PROTECTED: i32 = 32;
pub const KS_DVD_SECTOR_PROTECT_MASK: i32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_DVD_YCrCb {
    pub Reserved: u8,
    pub Y: u8,
    pub Cr: u8,
    pub Cb: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub hDirectDraw: super::HANDLE,
    pub hSurfaceHandle: super::HANDLE,
    pub DirectDrawRect: super::RECT,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_FRAME_INFO_1_0 {
    pub Reserved3: u32,
    pub Reserved4: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const KS_INTERLACE_1FieldPerSample: i32 = 2;
pub const KS_INTERLACE_DisplayModeBobOnly: i32 = 0;
pub const KS_INTERLACE_DisplayModeBobOrWeave: i32 = 128;
pub const KS_INTERLACE_DisplayModeMask: i32 = 192;
pub const KS_INTERLACE_DisplayModeWeaveOnly: i32 = 64;
pub const KS_INTERLACE_Field1First: i32 = 4;
pub const KS_INTERLACE_FieldPatBothIrregular: i32 = 48;
pub const KS_INTERLACE_FieldPatBothRegular: i32 = 32;
pub const KS_INTERLACE_FieldPatField1Only: i32 = 0;
pub const KS_INTERLACE_FieldPatField2Only: i32 = 16;
pub const KS_INTERLACE_FieldPatternMask: i32 = 48;
pub const KS_INTERLACE_IsInterlaced: i32 = 1;
pub const KS_INTERLACE_UNUSED: i32 = 8;
pub const KS_MACROVISION_DISABLED: KS_COPY_MACROVISION_LEVEL = 0;
pub const KS_MACROVISION_LEVEL1: KS_COPY_MACROVISION_LEVEL = 1;
pub const KS_MACROVISION_LEVEL2: KS_COPY_MACROVISION_LEVEL = 2;
pub const KS_MACROVISION_LEVEL3: KS_COPY_MACROVISION_LEVEL = 3;
pub const KS_MAX_SIZE_MPEG1_SEQUENCE_INFO: i32 = 140;
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const KS_MPEG2_27MhzTimebase: i32 = 256;
pub const KS_MPEG2_DSS_UserData: i32 = 64;
pub const KS_MPEG2_DVB_UserData: i32 = 128;
pub const KS_MPEG2_DVDLine21Field1: i32 = 2;
pub const KS_MPEG2_DVDLine21Field2: i32 = 4;
pub const KS_MPEG2_DoPanScan: i32 = 1;
pub const KS_MPEG2_FilmCameraMode: i32 = 16;
pub const KS_MPEG2_LetterboxAnalogOut: i32 = 32;
pub const KS_MPEG2_SourceIsLetterboxed: i32 = 8;
pub const KS_MPEG2_WidescreenAnalogOut: i32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_MPEGAUDIOINFO {
    pub dwFlags: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
}
pub const KS_MPEGAUDIOINFO_27MhzTimebase: i32 = 1;
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
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_ADVERTISER_BASE: i32 = 2224;
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_CONTENT_BASE: i32 = 2208;
pub const KS_NABTS_GROUPID_MICROSOFT_RESERVED_TEST_DATA_BASE: i32 = 2288;
pub const KS_NABTS_GROUPID_NETWORK_WIDE_ADVERTISER_BASE: i32 = 2160;
pub const KS_NABTS_GROUPID_NETWORK_WIDE_CONTENT_BASE: i32 = 2144;
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_ADVERTISER_BASE: i32 = 2064;
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_BASE: i32 = 2048;
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_ADVERTISER_BASE: i32 = 2096;
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_CONTENT_BASE: i32 = 2080;
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_ADVERTISER_BASE: i32 = 2128;
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_CONTENT_BASE: i32 = 2112;
pub const KS_NABTS_GROUPID_TELEVISION_STATION_ADVERTISER_BASE: i32 = 2192;
pub const KS_NABTS_GROUPID_TELEVISION_STATION_CONTENT_BASE: i32 = 2176;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
pub const KS_SECURE_CAMERA_SCENARIO_ID: windows_core::GUID = windows_core::GUID::from_u128(0xae53fc6e_8d89_4488_9d2e_4d008731c5fd);
#[cfg(target_arch = "x86")]
pub const KS_SIZE_EGA_PALETTE: u32 = 64;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KS_SIZE_EGA_PALETTE: u64 = 64;
#[cfg(target_arch = "x86")]
pub const KS_SIZE_MASKS: u32 = 12;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KS_SIZE_MASKS: u64 = 12;
#[cfg(target_arch = "x86")]
pub const KS_SIZE_PALETTE: u32 = 1024;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KS_SIZE_PALETTE: u64 = 1024;
pub const KS_SIZE_PREHEADER: i32 = 48;
pub const KS_StreamingHint_CompQuality: KS_VideoStreamingHints = 2048;
pub const KS_StreamingHint_CompWindowSize: KS_VideoStreamingHints = 4096;
pub const KS_StreamingHint_FrameInterval: KS_VideoStreamingHints = 256;
pub const KS_StreamingHint_KeyFrameRate: KS_VideoStreamingHints = 512;
pub const KS_StreamingHint_PFrameRate: KS_VideoStreamingHints = 1024;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const KS_TVAUDIO_MODE_LANG_A: i32 = 16;
pub const KS_TVAUDIO_MODE_LANG_B: i32 = 32;
pub const KS_TVAUDIO_MODE_LANG_C: i32 = 64;
pub const KS_TVAUDIO_MODE_MONO: i32 = 1;
pub const KS_TVAUDIO_MODE_STEREO: i32 = 2;
pub const KS_TVAUDIO_PRESET_LANG_A: i32 = 4096;
pub const KS_TVAUDIO_PRESET_LANG_B: i32 = 8192;
pub const KS_TVAUDIO_PRESET_LANG_C: i32 = 16384;
pub const KS_TVAUDIO_PRESET_STEREO: i32 = 512;
pub const KS_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 1;
pub const KS_TVTUNER_CHANGE_END_TUNE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_TVTUNER_CHANGE_INFO {
    pub dwFlags: u32,
    pub dwCountryCode: u32,
    pub dwAnalogVideoStandard: u32,
    pub dwChannel: u32,
}
pub const KS_VBICAP_PROTECTION_MV_DETECTED: i32 = 4;
pub const KS_VBICAP_PROTECTION_MV_HARDWARE: i32 = 2;
pub const KS_VBICAP_PROTECTION_MV_PRESENT: i32 = 1;
pub const KS_VBIDATARATE_CC: i32 = 503493;
pub const KS_VBIDATARATE_NABTS: i32 = 5727272;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const KS_VBISAMPLINGRATE_47X_NABTS: i32 = 27000000;
pub const KS_VBISAMPLINGRATE_4X_NABTS: i32 = 22909088;
pub const KS_VBISAMPLINGRATE_5X_NABTS: i32 = 28636360;
pub const KS_VBI_FLAG_FIELD1: i32 = 1;
pub const KS_VBI_FLAG_FIELD2: i32 = 2;
pub const KS_VBI_FLAG_MV_DETECTED: i32 = 1024;
pub const KS_VBI_FLAG_MV_HARDWARE: i32 = 512;
pub const KS_VBI_FLAG_MV_PRESENT: i32 = 256;
pub const KS_VBI_FLAG_TVTUNER_CHANGE: i32 = 16;
pub const KS_VBI_FLAG_VBIINFOHEADER_CHANGE: i32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::REFERENCE_TIME,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_VIDEOINFOHEADER {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::REFERENCE_TIME,
    pub bmiHeader: KS_BITMAPINFOHEADER,
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct KS_VIDEOINFOHEADER2 {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::REFERENCE_TIME,
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
pub const KS_VIDEOSTREAM_CAPTURE: i32 = 2;
pub const KS_VIDEOSTREAM_CC: i32 = 256;
pub const KS_VIDEOSTREAM_EDS: i32 = 512;
pub const KS_VIDEOSTREAM_IS_VPE: i32 = 32768;
pub const KS_VIDEOSTREAM_NABTS: i32 = 32;
pub const KS_VIDEOSTREAM_PREVIEW: i32 = 1;
pub const KS_VIDEOSTREAM_STILL: i32 = 4096;
pub const KS_VIDEOSTREAM_TELETEXT: i32 = 1024;
pub const KS_VIDEOSTREAM_VBI: i32 = 16;
pub const KS_VIDEO_ALLOC_VPE_AGP: i32 = 4;
pub const KS_VIDEO_ALLOC_VPE_DISPLAY: i32 = 2;
pub const KS_VIDEO_ALLOC_VPE_SYSTEM: i32 = 1;
pub const KS_VIDEO_FLAG_B_FRAME: i32 = 32;
pub const KS_VIDEO_FLAG_FIELD1: i32 = 1;
pub const KS_VIDEO_FLAG_FIELD1FIRST: i32 = 4;
pub const KS_VIDEO_FLAG_FIELD2: i32 = 2;
pub const KS_VIDEO_FLAG_FIELD_MASK: i32 = 3;
pub const KS_VIDEO_FLAG_FRAME: i32 = 0;
pub const KS_VIDEO_FLAG_IPB_MASK: i32 = 48;
pub const KS_VIDEO_FLAG_I_FRAME: i32 = 0;
pub const KS_VIDEO_FLAG_P_FRAME: i32 = 16;
pub const KS_VIDEO_FLAG_REPEAT_FIELD: i32 = 64;
pub const KS_VIDEO_FLAG_WEAVE: i32 = 8;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_VIDEO_STREAM_CONFIG_CAPS {
    pub guid: windows_core::GUID,
    pub VideoStandard: u32,
    pub InputSize: super::SIZE,
    pub MinCroppingSize: super::SIZE,
    pub MaxCroppingSize: super::SIZE,
    pub CropGranularityX: i32,
    pub CropGranularityY: i32,
    pub CropAlignX: i32,
    pub CropAlignY: i32,
    pub MinOutputSize: super::SIZE,
    pub MaxOutputSize: super::SIZE,
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
pub const KS_iBLUE: i32 = 2;
pub const KS_iEGA_COLORS: i32 = 16;
pub const KS_iGREEN: i32 = 1;
pub const KS_iMASK_COLORS: i32 = 3;
pub const KS_iMAXBITS: i32 = 8;
pub const KS_iPALETTE: i32 = 8;
pub const KS_iPALETTE_COLORS: i32 = 256;
pub const KS_iRED: i32 = 0;
pub const KS_iTRUECOLOR: i32 = 16;
pub const LIGHT_FILTER: KSDS3D_HRTF_FILTER_QUALITY = 1;
#[repr(C)]
#[cfg(all(feature = "ks", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LOOPEDSTREAMING_POSITION_EVENT_DATA {
    pub KsEventData: super::KSEVENTDATA,
    pub Position: super::DWORDLONG,
}
#[cfg(all(feature = "ks", feature = "winnt"))]
impl Default for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPDDPIXELFORMAT = *mut DDPIXELFORMAT;
pub type LPDDVIDEOPORTCONNECT = *mut DDVIDEOPORTCONNECT;
pub const MAX_NABTS_VBI_LINES_PER_FIELD: i32 = 11;
pub const MAX_RESOURCEGROUPID_LENGTH: i32 = 256;
pub const MAX_SINK_DESCRIPTION_NAME_LENGTH: i32 = 32;
pub const MAX_WST_VBI_LINES_PER_FIELD: i32 = 17;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEDIUM_INFO {
    pub MediaPresent: windows_core::BOOL,
    pub MediaType: u32,
    pub RecordInhibit: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NABTS_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 36],
}
impl Default for NABTS_BUFFER_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NABTS_BUFFER_PICTURENUMBER_SUPPORT: i32 = 1;
pub const NABTS_BYTES_PER_LINE: i32 = 36;
pub const NABTS_LINES_PER_BUNDLE: i32 = 16;
pub const NABTS_PAYLOAD_PER_LINE: i32 = 28;
pub type PAUDIORESOURCEMANAGEMENT_RESOURCEGROUP = *mut AUDIORESOURCEMANAGEMENT_RESOURCEGROUP;
pub type PCAPTURE_MEMORY_ALLOCATION_FLAGS = *mut CAPTURE_MEMORY_ALLOCATION_FLAGS;
pub type PCC_BYTE_PAIR = *mut CC_BYTE_PAIR;
pub type PCC_HW_FIELD = *mut CC_HW_FIELD;
pub type PDEVCAPS = *mut DEVCAPS;
#[cfg(feature = "minwindef")]
pub type PDS3DVECTOR = *mut DS3DVECTOR;
pub const PINNAME_DISPLAYPORT_OUT: windows_core::GUID = windows_core::GUID::from_u128(0x21fbb329_1a4a_48da_a076_2318a3c59b26);
pub const PINNAME_HDMI_OUT: windows_core::GUID = windows_core::GUID::from_u128(0x387bfc03_e7ef_4901_86e0_35b7c32b00ef);
pub const PINNAME_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x38a0cd98_d49b_4ce8_b48a_344667a17830);
pub const PINNAME_SPDIF_IN: windows_core::GUID = windows_core::GUID::from_u128(0x15dc9025_22ad_41b3_8875_f4ceb0299e20);
pub const PINNAME_SPDIF_OUT: windows_core::GUID = windows_core::GUID::from_u128(0x3a264481_e52c_4b82_8e7a_c8e2f91dc380);
pub const PINNAME_VIDEO_ANALOGVIDEOIN: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4283_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4281_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CC: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4289_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CC_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x1aad8061_012d_11d2_b4b1_00a0d102cfbe);
pub const PINNAME_VIDEO_EDS: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4287_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_NABTS: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4286_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_NABTS_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x29703660_498a_11d2_b4b1_00a0d102cfbe);
pub const PINNAME_VIDEO_PREVIEW: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4282_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_STILL: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c428a_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_TELETEXT: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4288_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_TIMECODE: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c428b_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VBI: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4284_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VIDEOPORT: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4285_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VIDEOPORT_VBI: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c428c_0353_11d1_905f_0000c0cc16ba);
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
#[cfg(feature = "winnt")]
pub type PKSCAMERA_EXTENDEDPROP_VALUE = *mut KSCAMERA_EXTENDEDPROP_VALUE;
#[cfg(feature = "winnt")]
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
#[cfg(feature = "minwindef")]
pub type PKSDS3D_BUFFER_ALL = *mut KSDS3D_BUFFER_ALL;
pub type PKSDS3D_BUFFER_CONE_ANGLES = *mut KSDS3D_BUFFER_CONE_ANGLES;
pub type PKSDS3D_HRTF_FILTER_FORMAT_MSG = *mut KSDS3D_HRTF_FILTER_FORMAT_MSG;
#[cfg(feature = "minwindef")]
pub type PKSDS3D_HRTF_INIT_MSG = *mut KSDS3D_HRTF_INIT_MSG;
pub type PKSDS3D_HRTF_PARAMS_MSG = *mut KSDS3D_HRTF_PARAMS_MSG;
#[cfg(feature = "minwindef")]
pub type PKSDS3D_ITD_PARAMS = *mut KSDS3D_ITD_PARAMS;
#[cfg(feature = "minwindef")]
pub type PKSDS3D_ITD_PARAMS_MSG = *mut KSDS3D_ITD_PARAMS_MSG;
#[cfg(feature = "minwindef")]
pub type PKSDS3D_LISTENER_ALL = *mut KSDS3D_LISTENER_ALL;
#[cfg(feature = "minwindef")]
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
    pub NodeProperty: super::KSP_NODE,
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
pub type PKSPROPERTY_COMPOSIT_ON = *mut windows_core::BOOL;
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
pub const PROPSETID_ALLOCATOR_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x53171960_148e_11d2_9979_0000c0cc16ba);
pub const PROPSETID_EXT_DEVICE: windows_core::GUID = windows_core::GUID::from_u128(0xb5730a90_1a2c_11cf_8c23_00aa006b6814);
pub const PROPSETID_EXT_TRANSPORT: windows_core::GUID = windows_core::GUID::from_u128(0xa03cd5f0_3045_11cf_8c44_00aa006b6814);
pub const PROPSETID_TIMECODE_READER: windows_core::GUID = windows_core::GUID::from_u128(0x9b496ce1_811b_11cf_8c77_00aa006b6814);
pub const PROPSETID_TUNER: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0605_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_CAMERACONTROL: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13370_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_CAMERACONTROL_FLASH: windows_core::GUID = windows_core::GUID::from_u128(0x785e8f49_63a2_4144_ab70_ffb278fa26ce);
pub const PROPSETID_VIDCAP_CAMERACONTROL_IMAGE_PIN_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x9d3d7bbf_5c6d_4138_bb00_584edd20f7c5);
pub const PROPSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: windows_core::GUID = windows_core::GUID::from_u128(0x9d12d198_f86c_4fed_b023_5d87653da793);
pub const PROPSETID_VIDCAP_CAMERACONTROL_VIDEO_STABILIZATION: windows_core::GUID = windows_core::GUID::from_u128(0x43964bd3_7716_404e_8be1_d299b20e50fd);
pub const PROPSETID_VIDCAP_CROSSBAR: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0640_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_DROPPEDFRAMES: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13344_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_SELECTOR: windows_core::GUID = windows_core::GUID::from_u128(0x1abdaeca_68b6_4f83_9371_b413907c7b9f);
pub const PROPSETID_VIDCAP_TVAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0650_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOCOMPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13343_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOCONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0670_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEODECODER: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13350_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0610_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOPROCAMP: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13360_30ac_11d0_a18c_00a0c9118956);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURE_BUFFER_INFO {
    pub guidBufferIdentifier: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOUNDDETECTOR_PATTERNHEADER {
    pub Size: u32,
    pub PatternType: windows_core::GUID,
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
    pub qw: super::DWORDLONG,
}
#[cfg(feature = "winnt")]
impl Default for TIMECODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPORTAUDIOPARMS {
    pub EnableOutput: i32,
    pub EnableRecord: i32,
    pub EnableSelsync: i32,
    pub Input: i32,
    pub MonitorSource: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub VolumeName: [super::TCHAR; 40],
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPORTVIDEOPARMS {
    pub OutputMode: i32,
    pub Input: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPORT_STATE {
    pub Mode: u32,
    pub State: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UMPDATAFORMAT {
    pub Position: i64,
    pub ByteCount: u32,
}
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy)]
pub struct VBICAP_PROPERTIES_PROTECTION_S {
    pub Property: super::KSPROPERTY,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VBICODECFILTERING_CC_SUBSTREAMS {
    pub SubstreamMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBICODECFILTERING_NABTS_SUBSTREAMS {
    pub SubstreamMask: [u32; 128],
}
impl Default for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBICODECFILTERING_SCANLINES {
    pub DwordBitArray: [u32; 32],
}
impl Default for VBICODECFILTERING_SCANLINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_CC {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_CC_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_NABTS_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
pub type VIDEOENCODER_BITRATE_MODE = i32;
#[repr(C)]
#[cfg(feature = "ks")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIDEOFORMAT_DX12 {
    pub Header: super::KSATTRIBUTE,
    pub resourceLayout: u32,
    pub resourceFlags: u32,
    pub customLayout: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub Property: super::KSPROPERTY,
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
    pub FormatExt: super::WAVEFORMATEXTENSIBLE,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WNF_KSCAMERA_STREAMSTATE_INFO {
    pub ProcessId: u32,
    pub SessionId: u32,
    pub StreamState: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WST_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 42],
}
impl Default for WST_BUFFER_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WST_BYTES_PER_LINE: i32 = 42;
pub const WST_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 4096;
pub const WST_TVTUNER_CHANGE_END_TUNE: i32 = 8192;
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
pub const eGeoLocReserved5: EPcxGeoLocation = 14;
pub const eGeoLocReserved6: EPcxGeoLocation = 15;
pub const eGeoLocRight: EPcxGeoLocation = 4;
pub const eGeoLocRiser: EPcxGeoLocation = 8;
pub const eGeoLocTop: EPcxGeoLocation = 5;
pub const ePortConnBothIntegratedAndJack: EPxcPortConnection = 2;
pub const ePortConnIntegratedDevice: EPxcPortConnection = 1;
pub const ePortConnJack: EPxcPortConnection = 0;
pub const ePortConnUnknown: EPxcPortConnection = 3;
