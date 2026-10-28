#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFCreateTransformActivate(ppactivate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type DeviceStreamState = i32;
pub const DeviceStreamState_Disabled: DeviceStreamState = 3;
pub const DeviceStreamState_Pause: DeviceStreamState = 1;
pub const DeviceStreamState_Run: DeviceStreamState = 2;
pub const DeviceStreamState_Stop: DeviceStreamState = 0;
pub const MEDeviceStreamCreated: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0252a1cf_3540_43b4_9164_d72eb405fa40);
pub type MF3DVideoOutputType = i32;
pub const MF3DVideoOutputType_BaseView: MF3DVideoOutputType = 0;
pub const MF3DVideoOutputType_Stereo: MF3DVideoOutputType = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFAudioDecoderDegradationInfo {
    pub eDegradationReason: MFT_AUDIO_DECODER_DEGRADATION_REASON,
    pub eType: MFT_AUDIO_DECODER_DEGRADATION_TYPE,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CATEGORY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xc57a84c0_1a80_40a3_97b5_9272a403c8ae), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLSID: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xc57a84c0_1a80_40a3_97b5_9272a403c8ae), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_EXATTRIBUTE_SUPPORTED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x456fe843_3c87_40c0_949d_1409c97dab2c), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MULTICHANNEL_CHANNEL_MASK: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x58bdaf8c_3224_4692_86d0_44d65c5bf82b), pid: 1 };
pub const MFT_AUDIO_DECODER_AUDIO_ENDPOINT_FORMFACTOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8d574310_909a_433a_ace7_eee74719f901);
pub const MFT_AUDIO_DECODER_AUDIO_ENDPOINT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc7ccdd6e_5398_4695_8be7_51b3e95111bd);
pub const MFT_AUDIO_DECODER_AUDIO_ENDPOINT_IS_DIGITAL_STEREO_ONLY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x26e5a90d_4ad1_4f8c_b8af_adf14d2178f1);
pub const MFT_AUDIO_DECODER_DEGRADATION_INFO_ATTRIBUTE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c3386ad_ec20_430d_b2a5_505c7178d9c4);
pub type MFT_AUDIO_DECODER_DEGRADATION_REASON = i32;
pub const MFT_AUDIO_DECODER_DEGRADATION_REASON_LICENSING_REQUIREMENT: MFT_AUDIO_DECODER_DEGRADATION_REASON = 1;
pub const MFT_AUDIO_DECODER_DEGRADATION_REASON_NONE: MFT_AUDIO_DECODER_DEGRADATION_REASON = 0;
pub type MFT_AUDIO_DECODER_DEGRADATION_TYPE = i32;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX2CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 1;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX6CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 2;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX8CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 3;
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_NONE: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 0;
pub const MFT_AUDIO_DECODER_SPATIAL_METADATA_CLIENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05987df4_1270_4999_925f_8e939a7c0af7);
pub const MFT_CODEC_MERIT_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x88a7cb15_7b07_4a34_9128_e64c6703c4d3);
pub const MFT_CONNECTED_STREAM_ATTRIBUTE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x71eeb820_a59f_4de2_bcec_38db1dd611a4);
pub const MFT_CONNECTED_TO_HW_STREAM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x34e6e728_06d6_4491_a553_4795650db912);
pub const MFT_DECODER_AUTOMATIC_SOFTWARE_FALLBACK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41f34f53_1bf6_49ed_b95d_02d2a1d7115a);
pub const MFT_DECODER_EXPOSE_OUTPUT_TYPES_IN_NATIVE_ORDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef80833f_f8fa_44d9_80d8_41ed6232670c);
pub const MFT_DECODER_FINAL_VIDEO_RESOLUTION_HINT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc2f8496_15c4_407a_b6f0_1b66ab5fbf53);
pub const MFT_DECODER_OPERATING_POINT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1230334_55d4_4d97_82a7_26d3e6456725);
pub const MFT_DECODER_QUALITY_MANAGEMENT_CUSTOM_CONTROL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa24e30d7_de25_4558_bbfb_71070a2d332e);
pub const MFT_DECODER_QUALITY_MANAGEMENT_RECOVERY_WITHOUT_ARTIFACTS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd8980deb_0a48_425f_8623_611db41d3810);
pub const MFT_DRAIN_NO_TAILS: _MFT_DRAIN_TYPE = 1;
pub const MFT_DRAIN_PRODUCE_TAILS: _MFT_DRAIN_TYPE = 0;
pub const MFT_ENCODER_SUPPORTS_CONFIG_EVENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86a355ae_3a77_4ec4_9f31_01149a4e92de);
pub const MFT_END_STREAMING_AWARE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x70fbc845_b07e_4089_b064_399dc6110f29);
pub const MFT_ENUM_HARDWARE_URL_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2fb866ac_b078_4942_ab6c_003d05cda674);
pub const MFT_ENUM_HARDWARE_VENDOR_ID_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3aecb0cc_035b_4bcc_8185_2b8d551ef3af);
pub const MFT_ENUM_TRANSCODE_ONLY_ATTRIBUTE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x111ea8cd_b62a_4bdb_89f6_67ffcdc2458b);
pub const MFT_FIELDOFUSE_UNLOCK_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8ec2e9fd_9148_410d_831e_702439461a8e);
pub const MFT_FRIENDLY_NAME_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x314ffbae_5b41_4c95_9c19_4e7d586face3);
pub const MFT_HW_TIMESTAMP_WITH_QPC_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8d030fb8_cc43_4258_a22e_9210bef89be4);
pub const MFT_INPUT_DATA_BUFFER_PLACEHOLDER: _MFT_INPUT_DATA_BUFFER_FLAGS = -1;
pub const MFT_INPUT_STATUS_ACCEPT_DATA: _MFT_INPUT_STATUS_FLAGS = 1;
pub const MFT_INPUT_STREAM_DOES_NOT_ADDREF: _MFT_INPUT_STREAM_INFO_FLAGS = 256;
pub const MFT_INPUT_STREAM_FIXED_SAMPLE_SIZE: _MFT_INPUT_STREAM_INFO_FLAGS = 4;
pub const MFT_INPUT_STREAM_HOLDS_BUFFERS: _MFT_INPUT_STREAM_INFO_FLAGS = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MFT_INPUT_STREAM_INFO {
    pub hnsMaxLatency: super::LONGLONG,
    pub dwFlags: super::DWORD,
    pub cbSize: super::DWORD,
    pub cbMaxLookahead: super::DWORD,
    pub cbAlignment: super::DWORD,
}
pub const MFT_INPUT_STREAM_OPTIONAL: _MFT_INPUT_STREAM_INFO_FLAGS = 1024;
pub const MFT_INPUT_STREAM_PROCESSES_IN_PLACE: _MFT_INPUT_STREAM_INFO_FLAGS = 2048;
pub const MFT_INPUT_STREAM_REMOVABLE: _MFT_INPUT_STREAM_INFO_FLAGS = 512;
pub const MFT_INPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER: _MFT_INPUT_STREAM_INFO_FLAGS = 2;
pub const MFT_INPUT_STREAM_WHOLE_SAMPLES: _MFT_INPUT_STREAM_INFO_FLAGS = 1;
pub const MFT_INPUT_TYPES_Attributes: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4276c9b1_759d_4bf3_9cd0_0d723d138f96);
pub const MFT_MESSAGE_COMMAND_DRAIN: MFT_MESSAGE_TYPE = 1;
pub const MFT_MESSAGE_COMMAND_FLUSH: MFT_MESSAGE_TYPE = 0;
pub const MFT_MESSAGE_COMMAND_FLUSH_OUTPUT_STREAM: MFT_MESSAGE_TYPE = 268435464;
pub const MFT_MESSAGE_COMMAND_MARKER: MFT_MESSAGE_TYPE = 536870912;
pub const MFT_MESSAGE_COMMAND_SET_OUTPUT_STREAM_STATE: MFT_MESSAGE_TYPE = 268435463;
pub const MFT_MESSAGE_COMMAND_TICK: MFT_MESSAGE_TYPE = 4;
pub const MFT_MESSAGE_DROP_SAMPLES: MFT_MESSAGE_TYPE = 3;
pub const MFT_MESSAGE_NOTIFY_BEGIN_STREAMING: MFT_MESSAGE_TYPE = 268435456;
pub const MFT_MESSAGE_NOTIFY_END_OF_STREAM: MFT_MESSAGE_TYPE = 268435458;
pub const MFT_MESSAGE_NOTIFY_END_STREAMING: MFT_MESSAGE_TYPE = 268435457;
pub const MFT_MESSAGE_NOTIFY_EVENT: MFT_MESSAGE_TYPE = 268435462;
pub const MFT_MESSAGE_NOTIFY_REACQUIRE_RESOURCES: MFT_MESSAGE_TYPE = 268435461;
pub const MFT_MESSAGE_NOTIFY_RELEASE_RESOURCES: MFT_MESSAGE_TYPE = 268435460;
pub const MFT_MESSAGE_NOTIFY_START_OF_STREAM: MFT_MESSAGE_TYPE = 268435459;
pub const MFT_MESSAGE_SET_D3D_MANAGER: MFT_MESSAGE_TYPE = 2;
pub type MFT_MESSAGE_TYPE = i32;
#[cfg(feature = "shared")]
pub const MFT_OUTPUT_BOUND_LOWER_UNBOUNDED: super::LONGLONG = -9223372036854775808;
pub const MFT_OUTPUT_BOUND_UPPER_UNBOUNDED: i64 = 9223372036854775807;
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MFT_OUTPUT_DATA_BUFFER {
    pub dwStreamID: super::DWORD,
    pub pSample: *mut core::ffi::c_void,
    pub dwStatus: super::DWORD,
    pub pEvents: *mut core::ffi::c_void,
}
pub const MFT_OUTPUT_DATA_BUFFER_FORMAT_CHANGE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 256;
pub const MFT_OUTPUT_DATA_BUFFER_INCOMPLETE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 16777216;
pub const MFT_OUTPUT_DATA_BUFFER_NO_SAMPLE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 768;
pub const MFT_OUTPUT_DATA_BUFFER_STREAM_END: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 512;
pub const MFT_OUTPUT_STATUS_SAMPLE_READY: _MFT_OUTPUT_STATUS_FLAGS = 1;
pub const MFT_OUTPUT_STREAM_CAN_PROVIDE_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 512;
pub const MFT_OUTPUT_STREAM_DISCARDABLE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 8;
pub const MFT_OUTPUT_STREAM_FIXED_SAMPLE_SIZE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MFT_OUTPUT_STREAM_INFO {
    pub dwFlags: super::DWORD,
    pub cbSize: super::DWORD,
    pub cbAlignment: super::DWORD,
}
pub const MFT_OUTPUT_STREAM_LAZY_READ: _MFT_OUTPUT_STREAM_INFO_FLAGS = 1024;
pub const MFT_OUTPUT_STREAM_OPTIONAL: _MFT_OUTPUT_STREAM_INFO_FLAGS = 16;
pub const MFT_OUTPUT_STREAM_PROVIDES_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 256;
pub const MFT_OUTPUT_STREAM_REMOVABLE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 2048;
pub const MFT_OUTPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER: _MFT_OUTPUT_STREAM_INFO_FLAGS = 2;
pub const MFT_OUTPUT_STREAM_WHOLE_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 1;
pub const MFT_OUTPUT_TYPES_Attributes: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8eae8cf3_a44f_4306_ba5c_bf5dda242818);
pub const MFT_POLICY_SET_AWARE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5a633b19_cc39_4fa8_8ca5_59981b7a0018);
pub const MFT_PREFERRED_ENCODER_PROFILE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x53004909_1ef5_46d7_a18e_5a75f8b5905f);
pub const MFT_PREFERRED_OUTPUTTYPE_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7e700499_396a_49ee_b1b4_f628021e8c9d);
pub const MFT_PROCESS_LOCAL_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x543186e4_4649_4e65_b588_4aa352aff379);
pub const MFT_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _MFT_PROCESS_OUTPUT_FLAGS = 1;
pub const MFT_PROCESS_OUTPUT_REGENERATE_LAST_OUTPUT: _MFT_PROCESS_OUTPUT_FLAGS = 2;
pub const MFT_PROCESS_OUTPUT_STATUS_NEW_STREAMS: _MFT_PROCESS_OUTPUT_STATUS = 256;
pub const MFT_REMUX_MARK_I_PICTURE_AS_CLEAN_POINT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x364e8f85_3f2e_436c_b2a2_4440a012a9e8);
pub const MFT_SET_TYPE_TEST_ONLY: _MFT_SET_TYPE_FLAGS = 1;
pub const MFT_STREAMS_UNLIMITED: u32 = 4294967295;
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MFT_STREAM_STATE_PARAM {
    pub StreamId: super::DWORD,
    pub State: super::MF_STREAM_STATE,
}
pub const MFT_SUPPORT_3DVIDEO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x093f81b1_4f2e_4631_8168_7934032a01d3);
pub const MFT_TRANSFORM_CLSID_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6821c42b_65a4_4e82_99bc_9a88205ecd0c);
pub const MFT_USING_HARDWARE_DRM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x34faa77d_d79e_4957_b8ce_362b2684996c);
pub const MF_DMFT_FRAME_BUFFER_INFO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x396ce1c9_67a9_454c_8797_95a45799d804);
pub const MF_ENABLE_3DVIDEO_OUTPUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbdad7bca_0e5f_4b10_ab16_26de381b6293);
pub const MF_SA_AUDIO_ENDPOINT_AWARE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc0381701_805c_42b2_ac8d_e2b4bf21f4f8);
pub const MF_SA_BUFFERS_PER_SAMPLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x873c5171_1e3d_4e25_988d_b433ce041983);
pub const MF_SA_D3D11_ALLOCATE_DISPLAYABLE_RESOURCES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeeface6d_2ea9_4adf_bbdf_7bbc482a1b6d);
pub const MF_SA_D3D11_ALLOW_DYNAMIC_YUV_TEXTURE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xce06d49f_0613_4b9d_86a6_d8c4f9c10075);
pub const MF_SA_D3D11_AWARE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x206b4fc8_fcf9_4c51_afe3_9764369e33a0);
pub const MF_SA_D3D11_BINDFLAGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeacf97ad_065c_4408_bee3_fdcbfd128be2);
pub const MF_SA_D3D11_HW_PROTECTED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3a8ba9d9_92ca_4307_a391_6999dbf3b6ce);
pub const MF_SA_D3D11_SHARED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7b8f32c3_6d96_4b89_9203_dd38b61414f3);
pub const MF_SA_D3D11_SHARED_WITHOUT_MUTEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39dbd44d_2e44_4931_a4c8_352d3dc42115);
pub const MF_SA_D3D11_USAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe85fe442_2ca3_486e_a9c7_109dda609880);
pub const MF_SA_D3D_AWARE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeaa35c29_775e_488e_9b61_b3283e49583b);
pub const MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x851745d5_c3d6_476d_9527_498ef2d10d18);
pub const MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT_PROGRESSIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0f5523a5_1cb2_47c5_a550_2eeb84b4d14a);
pub const MF_SA_REQUIRED_SAMPLE_COUNT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x18802c61_324b_4952_abd0_176ff5c696ff);
pub const MF_SA_REQUIRED_SAMPLE_COUNT_PROGRESSIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb172d58e_fa77_4e48_8d2a_1df2d850eac2);
pub const MF_TRANSFORM_ASYNC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf81a699a_649a_497d_8c73_29f8fed6ad7a);
pub const MF_TRANSFORM_ASYNC_UNLOCK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe5666d6b_3422_4eb6_a421_da7db1f8e207);
pub const MF_TRANSFORM_CATEGORY_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xceabba49_506d_4757_a6ff_66c184987e4e);
pub const MF_TRANSFORM_FLAGS_Attribute: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9359bb7e_6275_46c4_a025_1c01e45f1a86);
pub type PDeviceStreamState = *mut DeviceStreamState;
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub type PMFT_OUTPUT_DATA_BUFFER = *mut MFT_OUTPUT_DATA_BUFFER;
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub type PMFT_STREAM_STATE_PARAM = *mut MFT_STREAM_STATE_PARAM;
#[cfg(feature = "shared")]
pub type PSTREAM_MEDIUM = *mut STREAM_MEDIUM;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct STREAM_MEDIUM {
    pub gidMedium: windows_sys::core::GUID,
    pub unMediumInstance: super::UINT32,
}
pub type _MFT_DRAIN_TYPE = i32;
pub type _MFT_INPUT_DATA_BUFFER_FLAGS = i32;
pub type _MFT_INPUT_STATUS_FLAGS = i32;
pub type _MFT_INPUT_STREAM_INFO_FLAGS = i32;
pub type _MFT_OUTPUT_DATA_BUFFER_FLAGS = i32;
pub type _MFT_OUTPUT_STATUS_FLAGS = i32;
pub type _MFT_OUTPUT_STREAM_INFO_FLAGS = i32;
pub type _MFT_PROCESS_OUTPUT_FLAGS = i32;
pub type _MFT_PROCESS_OUTPUT_STATUS = i32;
pub type _MFT_SET_TYPE_FLAGS = i32;
