#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateAllocator(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateAllocator2(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateClock(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateClock2(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreatePin(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreatePin2(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateTopologyNode(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateTopologyNode2(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
}
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
pub struct ALLOCATOR_PROPERTIES_EX(i32);
pub struct APO_CLASS_UUID(i32);
pub struct AUDIOENDPOINT_CLASS_UUID(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AUDIOMODULE_MAX_DATA_SIZE: u32 = 64000u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const AUDIOMODULE_MAX_NAME_CCH_SIZE: u32 = 128u32;
pub struct AUDIOPOSTURE_ORIENTATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIORESOURCEMANAGEMENT_RESOURCEGROUP(i32);
pub struct AUDIO_CURVE_TYPE(i32);
pub struct AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION(i32);
pub struct AUDIO_EFFECT_TYPE_AUTOMATIC_GAIN_CONTROL(i32);
pub struct AUDIO_EFFECT_TYPE_BASS_BOOST(i32);
pub struct AUDIO_EFFECT_TYPE_BASS_MANAGEMENT(i32);
pub struct AUDIO_EFFECT_TYPE_BEAMFORMING(i32);
pub struct AUDIO_EFFECT_TYPE_CONSTANT_TONE_REMOVAL(i32);
pub struct AUDIO_EFFECT_TYPE_DEEP_NOISE_SUPPRESSION(i32);
pub struct AUDIO_EFFECT_TYPE_DYNAMIC_RANGE_COMPRESSION(i32);
pub struct AUDIO_EFFECT_TYPE_ENVIRONMENTAL_EFFECTS(i32);
pub struct AUDIO_EFFECT_TYPE_EQUALIZER(i32);
pub struct AUDIO_EFFECT_TYPE_FAR_FIELD_BEAMFORMING(i32);
pub struct AUDIO_EFFECT_TYPE_LOUDNESS_EQUALIZER(i32);
pub struct AUDIO_EFFECT_TYPE_NOISE_SUPPRESSION(i32);
pub struct AUDIO_EFFECT_TYPE_ROOM_CORRECTION(i32);
pub struct AUDIO_EFFECT_TYPE_SPEAKER_COMPENSATION(i32);
pub struct AUDIO_EFFECT_TYPE_SPEAKER_FILL(i32);
pub struct AUDIO_EFFECT_TYPE_SPEAKER_PROTECTION(i32);
pub struct AUDIO_EFFECT_TYPE_VIRTUAL_HEADPHONES(i32);
pub struct AUDIO_EFFECT_TYPE_VIRTUAL_SURROUND(i32);
pub struct AUDIO_SIGNALPROCESSINGMODE_COMMUNICATIONS(i32);
pub struct AUDIO_SIGNALPROCESSINGMODE_DEFAULT(i32);
pub struct AUDIO_SIGNALPROCESSINGMODE_FAR_FIELD_SPEECH(i32);
pub struct AUDIO_SIGNALPROCESSINGMODE_MEDIA(i32);
pub struct AUDIO_SIGNALPROCESSINGMODE_MOVIE(i32);
pub struct AUDIO_SIGNALPROCESSINGMODE_NOTIFICATION(i32);
pub struct AUDIO_SIGNALPROCESSINGMODE_RAW(i32);
pub struct AUDIO_SIGNALPROCESSINGMODE_SPEECH(i32);
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
pub struct BLUETOOTHLE_MIDI_SERVICE_UUID(i32);
pub struct BLUETOOTH_MIDI_DATAIO_CHARACTERISTIC(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const BUS_INTERFACE_REFERENCE_VERSION: u32 = 256u32;
pub struct CAPTURE_MEMORY_ALLOCATION_FLAGS(i32);
pub struct CC_BYTE_PAIR(i32);
pub struct CC_HW_FIELD(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const CC_MAX_HW_DECODE_LINES: u32 = 12u32;
pub struct CLSID_KsIBasicAudioInterfaceHandler(i32);
pub struct CLSID_Proxy(i32);
pub struct CODECAPI_ALLSETTINGS(i32);
pub struct CODECAPI_AUDIO_ENCODER(i32);
pub struct CODECAPI_CHANGELISTS(i32);
pub struct CODECAPI_CURRENTCHANGELIST(i32);
pub struct CODECAPI_SETALLDEFAULTS(i32);
pub struct CODECAPI_SUPPORTSEVENTS(i32);
pub struct CODECAPI_VIDEO_ENCODER(i32);
pub struct CONSTRICTOR_OPTION(i32);
pub struct DEVCAPS(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_KsAudio_Controller_DeviceInterface_Path: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 333448406, data2: 45158, data3: 17341, data4: [145, 59, 164, 21, 205, 19, 218, 135] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_KsAudio_PacketSize_Constraints: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 333448406, data2: 45158, data3: 17341, data4: [145, 59, 164, 21, 205, 19, 218, 135] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_KsAudio_PacketSize_Constraints2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2483353473,
        data2: 29073,
        data3: 16539,
        data4: [139, 11, 128, 191, 110, 194, 41, 174],
    },
    pid: 2u32,
};
pub struct DS3DVECTOR(i32);
pub struct ENCAPIPARAM_BITRATE(i32);
pub struct ENCAPIPARAM_BITRATE_MODE(i32);
pub struct ENCAPIPARAM_PEAK_BITRATE(i32);
pub struct EPcxConnectionType(i32);
pub struct EPcxGenLocation(i32);
pub struct EPcxGeoLocation(i32);
pub struct EPxcPortConnection(i32);
pub struct EVENTSETID_CROSSBAR(i32);
pub struct EVENTSETID_TUNER(i32);
pub struct EVENTSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST(i32);
pub struct EVENTSETID_VIDEODECODER(i32);
pub struct FRAMING_CACHE_OPS(i32);
pub struct FRAMING_PROP(i32);
pub struct GUID_NULL(i32);
pub struct IKsAggregateControl(pub *mut ::core::ffi::c_void);
pub struct IKsAllocator(i32);
pub struct IKsAllocatorEx(i32);
pub struct IKsControl(pub *mut ::core::ffi::c_void);
pub struct IKsFormatSupport(pub *mut ::core::ffi::c_void);
pub struct IKsJackContainerId(pub *mut ::core::ffi::c_void);
pub struct IKsJackDescription(pub *mut ::core::ffi::c_void);
pub struct IKsJackDescription2(pub *mut ::core::ffi::c_void);
pub struct IKsJackSinkInformation(pub *mut ::core::ffi::c_void);
pub struct IKsPin(i32);
pub struct IKsPropertySet(pub *mut ::core::ffi::c_void);
pub struct IKsTopology(pub *mut ::core::ffi::c_void);
pub struct INTERLEAVED_AUDIO_FORMAT_INFORMATION(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub struct KSAC3_ALTERNATE_AUDIO(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_ALTERNATE_AUDIO_1: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_ALTERNATE_AUDIO_2: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAC3_ALTERNATE_AUDIO_BOTH: u32 = 3u32;
pub struct KSAC3_BIT_STREAM_MODE(i32);
pub struct KSAC3_DIALOGUE_LEVEL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSAC3_DOWNMIX(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSAC3_ERROR_CONCEALMENT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSAC3_ROOM_TYPE(i32);
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
pub struct KSALGORITHMINSTANCE_SYSTEM_ACOUSTIC_ECHO_CANCEL(i32);
pub struct KSALGORITHMINSTANCE_SYSTEM_AGC(i32);
pub struct KSALGORITHMINSTANCE_SYSTEM_MICROPHONE_ARRAY_PROCESSOR(i32);
pub struct KSALGORITHMINSTANCE_SYSTEM_NOISE_SUPPRESS(i32);
pub struct KSALLOCATORMODE(i32);
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
pub struct KSALLOCATOR_FRAMING(i32);
pub struct KSALLOCATOR_FRAMING_EX(i32);
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
pub struct KSATTRIBUTE(i32);
pub struct KSATTRIBUTEID_AUDIOSIGNALPROCESSING_MODE(i32);
pub struct KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSATTRIBUTE_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDDECOUTMODE_PCM_51: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDDECOUTMODE_SPDIFF: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDDECOUTMODE_STEREO_ANALOG: u32 = 1u32;
pub struct KSAUDFNAME_3D_CENTER(i32);
pub struct KSAUDFNAME_3D_DEPTH(i32);
pub struct KSAUDFNAME_3D_STEREO(i32);
pub struct KSAUDFNAME_ALTERNATE_MICROPHONE(i32);
pub struct KSAUDFNAME_AUX(i32);
pub struct KSAUDFNAME_AUX_MUTE(i32);
pub struct KSAUDFNAME_AUX_VOLUME(i32);
pub struct KSAUDFNAME_BASS(i32);
pub struct KSAUDFNAME_CD_AUDIO(i32);
pub struct KSAUDFNAME_CD_IN_VOLUME(i32);
pub struct KSAUDFNAME_CD_MUTE(i32);
pub struct KSAUDFNAME_CD_VOLUME(i32);
pub struct KSAUDFNAME_LINE_IN(i32);
pub struct KSAUDFNAME_LINE_IN_VOLUME(i32);
pub struct KSAUDFNAME_LINE_MUTE(i32);
pub struct KSAUDFNAME_LINE_VOLUME(i32);
pub struct KSAUDFNAME_MASTER_MUTE(i32);
pub struct KSAUDFNAME_MASTER_VOLUME(i32);
pub struct KSAUDFNAME_MICROPHONE_BOOST(i32);
pub struct KSAUDFNAME_MIC_IN_VOLUME(i32);
pub struct KSAUDFNAME_MIC_MUTE(i32);
pub struct KSAUDFNAME_MIC_VOLUME(i32);
pub struct KSAUDFNAME_MIDI(i32);
pub struct KSAUDFNAME_MIDI_IN_VOLUME(i32);
pub struct KSAUDFNAME_MIDI_MUTE(i32);
pub struct KSAUDFNAME_MIDI_VOLUME(i32);
pub struct KSAUDFNAME_MIDRANGE(i32);
pub struct KSAUDFNAME_MONO_MIX(i32);
pub struct KSAUDFNAME_MONO_MIX_MUTE(i32);
pub struct KSAUDFNAME_MONO_MIX_VOLUME(i32);
pub struct KSAUDFNAME_MONO_OUT(i32);
pub struct KSAUDFNAME_MONO_OUT_MUTE(i32);
pub struct KSAUDFNAME_MONO_OUT_VOLUME(i32);
pub struct KSAUDFNAME_PC_SPEAKER(i32);
pub struct KSAUDFNAME_PC_SPEAKER_MUTE(i32);
pub struct KSAUDFNAME_PC_SPEAKER_VOLUME(i32);
pub struct KSAUDFNAME_PEAKMETER(i32);
pub struct KSAUDFNAME_RECORDING_CONTROL(i32);
pub struct KSAUDFNAME_RECORDING_SOURCE(i32);
pub struct KSAUDFNAME_STEREO_MIX(i32);
pub struct KSAUDFNAME_STEREO_MIX_MUTE(i32);
pub struct KSAUDFNAME_STEREO_MIX_VOLUME(i32);
pub struct KSAUDFNAME_TREBLE(i32);
pub struct KSAUDFNAME_VIDEO(i32);
pub struct KSAUDFNAME_VIDEO_MUTE(i32);
pub struct KSAUDFNAME_VIDEO_VOLUME(i32);
pub struct KSAUDFNAME_VOLUME_CONTROL(i32);
pub struct KSAUDFNAME_WAVE_IN_VOLUME(i32);
pub struct KSAUDFNAME_WAVE_MUTE(i32);
pub struct KSAUDFNAME_WAVE_OUT_MIX(i32);
pub struct KSAUDFNAME_WAVE_VOLUME(i32);
pub struct KSAUDIOENGINE_BUFFER_SIZE_RANGE(i32);
pub struct KSAUDIOENGINE_DESCRIPTOR(i32);
pub struct KSAUDIOENGINE_VOLUMELEVEL(i32);
pub struct KSAUDIOMODULE_DESCRIPTOR(i32);
pub struct KSAUDIOMODULE_NOTIFICATION(i32);
pub struct KSAUDIOMODULE_PROPERTY(i32);
pub struct KSAUDIO_CHANNEL_CONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSAUDIO_COPY_PROTECTION(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_CPU_RESOURCES_HOST_CPU: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSAUDIO_CPU_RESOURCES_NOT_HOST_CPU: u32 = 0u32;
pub struct KSAUDIO_DYNAMIC_RANGE(i32);
pub struct KSAUDIO_MICROPHONE_COORDINATES(i32);
pub struct KSAUDIO_MIC_ARRAY_GEOMETRY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSAUDIO_MIXCAP_TABLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSAUDIO_MIXLEVEL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSAUDIO_MIX_CAPS(i32);
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS(i32);
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS2(i32);
pub struct KSAUDIO_POSITION(i32);
pub struct KSAUDIO_POSITIONEX(i32);
pub struct KSAUDIO_PRESENTATION_POSITION(i32);
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
pub struct KSCAMERAPROFILE_BalancedVideoAndPhoto(i32);
pub struct KSCAMERAPROFILE_CompressedCamera(i32);
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
pub struct KSCAMERAPROFILE_FaceAuth_Mode(i32);
pub struct KSCAMERAPROFILE_HDRWithWCGPhoto(i32);
pub struct KSCAMERAPROFILE_HDRWithWCGVideo(i32);
pub struct KSCAMERAPROFILE_HighFrameRate(i32);
pub struct KSCAMERAPROFILE_HighQualityPhoto(i32);
pub struct KSCAMERAPROFILE_Legacy(i32);
pub struct KSCAMERAPROFILE_PhotoSequence(i32);
pub struct KSCAMERAPROFILE_VariablePhotoSequence(i32);
pub struct KSCAMERAPROFILE_VideoConferencing(i32);
pub struct KSCAMERAPROFILE_VideoHDR8(i32);
pub struct KSCAMERAPROFILE_VideoRecording(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_MASK: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_OFF: u64 = 0u64;
pub struct KSCAMERA_EXTENDEDPROP_CAMERAOFFSET(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS(i32);
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_MANUAL: u64 = 0u64;
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING(i32);
pub struct KSCAMERA_EXTENDEDPROP_EVCOMPENSATION(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_FIELDOFVIEW(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_FOCUSSTATE(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_HEADER(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_METADATAINFO(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_ALIGNMENTREQUIRED: u64 = 256u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_MEMORYTYPE_MASK: u64 = 255u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_METADATA_SYSTEMMEMORY: u64 = 1u64;
pub struct KSCAMERA_EXTENDEDPROP_MetadataAlignment(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_PHOTOMODE(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_PROFILE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_DYNAMIC: u64 = 2u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_OFF: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_ON: u64 = 1u64;
pub struct KSCAMERA_EXTENDEDPROP_ROITYPE(i32);
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS(i32);
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_ROI_FOCUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_ROI_INFO(i32);
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL(i32);
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_VALUE(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING(i32);
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
pub struct KSCAMERA_EXTENDEDPROP_WBPRESET(i32);
pub struct KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DEFAULT: u64 = 0u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DIRECT: u64 = 1u64;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_EXTENDEDPROP_ZOOM_SMOOTH: u64 = 2u64;
pub struct KSCAMERA_MAXVIDEOFPS_FORPHOTORES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK(i32);
pub struct KSCAMERA_METADATA_CAPTURESTATS(i32);
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
pub struct KSCAMERA_METADATA_DIGITALWINDOW(i32);
pub struct KSCAMERA_METADATA_FRAMEILLUMINATION(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_METADATA_FRAMEILLUMINATION_FLAG_ON: u32 = 1u32;
pub struct KSCAMERA_METADATA_ITEMHEADER(i32);
pub struct KSCAMERA_METADATA_PHOTOCONFIRMATION(i32);
pub struct KSCAMERA_MetadataId(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_PERFRAMESETTING_AUTO: u64 = 4294967296u64;
pub struct KSCAMERA_PERFRAMESETTING_CAP_HEADER(i32);
pub struct KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER(i32);
pub struct KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM(i32);
pub struct KSCAMERA_PERFRAMESETTING_FRAME_HEADER(i32);
pub struct KSCAMERA_PERFRAMESETTING_HEADER(i32);
pub struct KSCAMERA_PERFRAMESETTING_ITEM_HEADER(i32);
pub struct KSCAMERA_PERFRAMESETTING_ITEM_TYPE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSCAMERA_PERFRAMESETTING_MANUAL: u64 = 8589934592u64;
pub struct KSCAMERA_PROFILE_CONCURRENCYINFO(i32);
pub struct KSCAMERA_PROFILE_INFO(i32);
pub struct KSCAMERA_PROFILE_MEDIAINFO(i32);
pub struct KSCAMERA_PROFILE_PININFO(i32);
pub struct KSCATEGORY_ACOUSTIC_ECHO_CANCEL(i32);
pub struct KSCATEGORY_AUDIO(i32);
pub struct KSCATEGORY_BRIDGE(i32);
pub struct KSCATEGORY_CAPTURE(i32);
pub struct KSCATEGORY_CLOCK(i32);
pub struct KSCATEGORY_COMMUNICATIONSTRANSFORM(i32);
pub struct KSCATEGORY_CROSSBAR(i32);
pub struct KSCATEGORY_DATACOMPRESSOR(i32);
pub struct KSCATEGORY_DATADECOMPRESSOR(i32);
pub struct KSCATEGORY_DATATRANSFORM(i32);
pub struct KSCATEGORY_ENCODER(i32);
pub struct KSCATEGORY_ESCALANTE_PLATFORM_DRIVER(i32);
pub struct KSCATEGORY_FILESYSTEM(i32);
pub struct KSCATEGORY_INTERFACETRANSFORM(i32);
pub struct KSCATEGORY_MEDIUMTRANSFORM(i32);
pub struct KSCATEGORY_MICROPHONE_ARRAY_PROCESSOR(i32);
pub struct KSCATEGORY_MIXER(i32);
pub struct KSCATEGORY_MULTIPLEXER(i32);
pub struct KSCATEGORY_NETWORK(i32);
pub struct KSCATEGORY_NETWORK_CAMERA(i32);
pub struct KSCATEGORY_PROXY(i32);
pub struct KSCATEGORY_QUALITY(i32);
pub struct KSCATEGORY_REALTIME(i32);
pub struct KSCATEGORY_RENDER(i32);
pub struct KSCATEGORY_SENSOR_CAMERA(i32);
pub struct KSCATEGORY_SENSOR_GROUP(i32);
pub struct KSCATEGORY_SPLITTER(i32);
pub struct KSCATEGORY_TEXT(i32);
pub struct KSCATEGORY_TOPOLOGY(i32);
pub struct KSCATEGORY_TVAUDIO(i32);
pub struct KSCATEGORY_TVTUNER(i32);
pub struct KSCATEGORY_VBICODEC(i32);
pub struct KSCATEGORY_VIDEO(i32);
pub struct KSCATEGORY_VIDEO_CAMERA(i32);
pub struct KSCATEGORY_VIRTUAL(i32);
pub struct KSCATEGORY_VPMUX(i32);
pub struct KSCATEGORY_WDMAUD_USE_PIN_NAME(i32);
pub struct KSCLOCK_CREATE(i32);
pub struct KSCOMPONENTID(i32);
pub struct KSCOMPONENTID_USBAUDIO(i32);
pub struct KSCORRELATED_TIME(i32);
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
pub struct KSDATAFORMAT(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDATAFORMAT_BIT_ATTRIBUTES: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDATAFORMAT_BIT_TEMPORAL_COMPRESSION: u32 = 0u32;
pub struct KSDATAFORMAT_SPECIFIER_AC3_AUDIO(i32);
pub struct KSDATAFORMAT_SPECIFIER_ANALOGVIDEO(i32);
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_AC3_AUDIO(i32);
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_AUDIO(i32);
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_VIDEO(i32);
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_AUDIO(i32);
pub struct KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_VIDEO(i32);
pub struct KSDATAFORMAT_SPECIFIER_DSOUND(i32);
pub struct KSDATAFORMAT_SPECIFIER_FILEHANDLE(i32);
pub struct KSDATAFORMAT_SPECIFIER_FILENAME(i32);
pub struct KSDATAFORMAT_SPECIFIER_H264_VIDEO(i32);
pub struct KSDATAFORMAT_SPECIFIER_IMAGE(i32);
pub struct KSDATAFORMAT_SPECIFIER_JPEG_IMAGE(i32);
pub struct KSDATAFORMAT_SPECIFIER_LPCM_AUDIO(i32);
pub struct KSDATAFORMAT_SPECIFIER_MPEG1_VIDEO(i32);
pub struct KSDATAFORMAT_SPECIFIER_MPEG2_AUDIO(i32);
pub struct KSDATAFORMAT_SPECIFIER_MPEG2_VIDEO(i32);
pub struct KSDATAFORMAT_SPECIFIER_NONE(i32);
pub struct KSDATAFORMAT_SPECIFIER_VBI(i32);
pub struct KSDATAFORMAT_SPECIFIER_VC_ID(i32);
pub struct KSDATAFORMAT_SPECIFIER_VIDEOINFO(i32);
pub struct KSDATAFORMAT_SPECIFIER_VIDEOINFO2(i32);
pub struct KSDATAFORMAT_SPECIFIER_WAVEFORMATEX(i32);
pub struct KSDATAFORMAT_SUBTYPE_AC3_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_ANALOG(i32);
pub struct KSDATAFORMAT_SUBTYPE_CC(i32);
pub struct KSDATAFORMAT_SUBTYPE_D16(i32);
pub struct KSDATAFORMAT_SUBTYPE_DSS_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_DSS_VIDEO(i32);
pub struct KSDATAFORMAT_SUBTYPE_DTS_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_AAC(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_ATRAC(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS_ATMOS(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT20(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MLP(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DST(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DTS(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E1(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E2(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_DTS_HD(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEG1(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEG2(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_MPEG3(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_ONE_BIT_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_IEC61937_WMA_PRO(i32);
pub struct KSDATAFORMAT_SUBTYPE_IMAGE_RGB32(i32);
pub struct KSDATAFORMAT_SUBTYPE_JPEG(i32);
pub struct KSDATAFORMAT_SUBTYPE_L16(i32);
pub struct KSDATAFORMAT_SUBTYPE_L16_CUSTOM(i32);
pub struct KSDATAFORMAT_SUBTYPE_L16_IR(i32);
pub struct KSDATAFORMAT_SUBTYPE_L8(i32);
pub struct KSDATAFORMAT_SUBTYPE_L8_CUSTOM(i32);
pub struct KSDATAFORMAT_SUBTYPE_L8_IR(i32);
pub struct KSDATAFORMAT_SUBTYPE_LPCM_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_Line21_BytePair(i32);
pub struct KSDATAFORMAT_SUBTYPE_Line21_GOPPacket(i32);
pub struct KSDATAFORMAT_SUBTYPE_MIDI(i32);
pub struct KSDATAFORMAT_SUBTYPE_MIDI_BUS(i32);
pub struct KSDATAFORMAT_SUBTYPE_MJPG_CUSTOM(i32);
pub struct KSDATAFORMAT_SUBTYPE_MJPG_DEPTH(i32);
pub struct KSDATAFORMAT_SUBTYPE_MJPG_IR(i32);
pub struct KSDATAFORMAT_SUBTYPE_MPEG1Packet(i32);
pub struct KSDATAFORMAT_SUBTYPE_MPEG1Payload(i32);
pub struct KSDATAFORMAT_SUBTYPE_MPEG1Video(i32);
pub struct KSDATAFORMAT_SUBTYPE_MPEG2_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_MPEG2_VIDEO(i32);
pub struct KSDATAFORMAT_SUBTYPE_MPEGLAYER3(i32);
pub struct KSDATAFORMAT_SUBTYPE_MPEG_HEAAC(i32);
pub struct KSDATAFORMAT_SUBTYPE_NABTS(i32);
pub struct KSDATAFORMAT_SUBTYPE_NABTS_FEC(i32);
pub struct KSDATAFORMAT_SUBTYPE_NONE(i32);
pub struct KSDATAFORMAT_SUBTYPE_OVERLAY(i32);
pub struct KSDATAFORMAT_SUBTYPE_PCM(i32);
pub struct KSDATAFORMAT_SUBTYPE_RAW8(i32);
pub struct KSDATAFORMAT_SUBTYPE_RIFF(i32);
pub struct KSDATAFORMAT_SUBTYPE_RIFFMIDI(i32);
pub struct KSDATAFORMAT_SUBTYPE_RIFFWAVE(i32);
pub struct KSDATAFORMAT_SUBTYPE_SDDS_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_AC3_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_VIDEO(i32);
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_AUDIO(i32);
pub struct KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_VIDEO(i32);
pub struct KSDATAFORMAT_SUBTYPE_SUBPICTURE(i32);
pub struct KSDATAFORMAT_SUBTYPE_TELETEXT(i32);
pub struct KSDATAFORMAT_SUBTYPE_VPVBI(i32);
pub struct KSDATAFORMAT_SUBTYPE_VPVideo(i32);
pub struct KSDATAFORMAT_SUBTYPE_WAVEFORMATEX(i32);
pub struct KSDATAFORMAT_SUBTYPE_WMAUDIO2(i32);
pub struct KSDATAFORMAT_SUBTYPE_WMAUDIO3(i32);
pub struct KSDATAFORMAT_SUBTYPE_WMAUDIO_LOSSLESS(i32);
pub struct KSDATAFORMAT_TYPE_ANALOGAUDIO(i32);
pub struct KSDATAFORMAT_TYPE_ANALOGVIDEO(i32);
pub struct KSDATAFORMAT_TYPE_AUDIO(i32);
pub struct KSDATAFORMAT_TYPE_AUXLine21Data(i32);
pub struct KSDATAFORMAT_TYPE_DVD_ENCRYPTED_PACK(i32);
pub struct KSDATAFORMAT_TYPE_IMAGE(i32);
pub struct KSDATAFORMAT_TYPE_MIDI(i32);
pub struct KSDATAFORMAT_TYPE_MPEG2_PES(i32);
pub struct KSDATAFORMAT_TYPE_MPEG2_PROGRAM(i32);
pub struct KSDATAFORMAT_TYPE_MPEG2_TRANSPORT(i32);
pub struct KSDATAFORMAT_TYPE_MUSIC(i32);
pub struct KSDATAFORMAT_TYPE_NABTS(i32);
pub struct KSDATAFORMAT_TYPE_STANDARD_ELEMENTARY_STREAM(i32);
pub struct KSDATAFORMAT_TYPE_STANDARD_PACK_HEADER(i32);
pub struct KSDATAFORMAT_TYPE_STANDARD_PES_PACKET(i32);
pub struct KSDATAFORMAT_TYPE_STREAM(i32);
pub struct KSDATAFORMAT_TYPE_TEXT(i32);
pub struct KSDATAFORMAT_TYPE_VBI(i32);
pub struct KSDATAFORMAT_TYPE_VIDEO(i32);
pub struct KSDATARANGE_AUDIO(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDATARANGE_BIT_ATTRIBUTES: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDATARANGE_BIT_REQUIRED_ATTRIBUTES: u32 = 2u32;
pub struct KSDATARANGE_MUSIC(i32);
pub struct KSDEGRADESETID_Standard(i32);
pub struct KSDEGRADE_STANDARD(i32);
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
pub struct KSDEVICE_PROFILE_INFO(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_PROFILE_TYPE_CAMERA: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDEVICE_PROFILE_TYPE_UNKNOWN: u32 = 0u32;
pub struct KSDEVICE_THERMAL_STATE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSDISPATCH_FASTIO: u32 = 2147483648u32;
pub struct KSDISPLAYCHANGE(i32);
pub struct KSDS3D_BUFFER_ALL(i32);
pub struct KSDS3D_BUFFER_CONE_ANGLES(i32);
pub struct KSDS3D_HRTF_COEFF_FORMAT(i32);
pub struct KSDS3D_HRTF_FILTER_FORMAT_MSG(i32);
pub struct KSDS3D_HRTF_FILTER_METHOD(i32);
pub struct KSDS3D_HRTF_FILTER_QUALITY(i32);
pub struct KSDS3D_HRTF_FILTER_VERSION(i32);
pub struct KSDS3D_HRTF_INIT_MSG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSDS3D_HRTF_PARAMS_MSG(i32);
pub struct KSDS3D_ITD_PARAMS(i32);
pub struct KSDS3D_ITD_PARAMS_MSG(i32);
pub struct KSDS3D_LISTENER_ALL(i32);
pub struct KSDS3D_LISTENER_ORIENTATION(i32);
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
pub struct KSERROR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENTDATA(i32);
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
pub struct KSEVENTSETID_AudioControlChange(i32);
pub struct KSEVENTSETID_CameraAsyncControl(i32);
pub struct KSEVENTSETID_CameraEvent(i32);
pub struct KSEVENTSETID_Clock(i32);
pub struct KSEVENTSETID_Connection(i32);
pub struct KSEVENTSETID_Device(i32);
pub struct KSEVENTSETID_DynamicFormatChange(i32);
pub struct KSEVENTSETID_EXTDEV_Command(i32);
pub struct KSEVENTSETID_ExtendedCameraControl(i32);
pub struct KSEVENTSETID_LoopedStreaming(i32);
pub struct KSEVENTSETID_PinCapsChange(i32);
pub struct KSEVENTSETID_SoundDetector(i32);
pub struct KSEVENTSETID_StreamAllocator(i32);
pub struct KSEVENTSETID_Telephony(i32);
pub struct KSEVENTSETID_VIDCAPTOSTI(i32);
pub struct KSEVENTSETID_VIDCAP_TVAUDIO(i32);
pub struct KSEVENTSETID_VPNotify(i32);
pub struct KSEVENTSETID_VPVBINotify(i32);
pub struct KSEVENTSETID_VolumeLimit(i32);
pub struct KSEVENT_AUDIO_CONTROL_CHANGE(i32);
pub struct KSEVENT_CAMERACONTROL(i32);
pub struct KSEVENT_CAMERAEVENT(i32);
pub struct KSEVENT_CLOCK_POSITION(i32);
pub struct KSEVENT_CONNECTION(i32);
pub struct KSEVENT_CROSSBAR(i32);
pub struct KSEVENT_DEVCMD(i32);
pub struct KSEVENT_DEVICE(i32);
pub struct KSEVENT_DYNAMICFORMATCHANGE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_ENTRY_BUFFERED: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_ENTRY_DELETED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSEVENT_ENTRY_ONESHOT: u32 = 2u32;
pub struct KSEVENT_LOOPEDSTREAMING(i32);
pub struct KSEVENT_PINCAPS_CHANGENOTIFICATIONS(i32);
pub struct KSEVENT_SOUNDDETECTOR(i32);
pub struct KSEVENT_STREAMALLOCATOR(i32);
pub struct KSEVENT_TELEPHONY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENT_TIME_INTERVAL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENT_TIME_MARK(i32);
pub struct KSEVENT_TUNER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSEVENT_TUNER_INITIATE_SCAN_S(i32);
pub struct KSEVENT_TVAUDIO(i32);
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
pub struct KSEVENT_VIDCAPTOSTI(i32);
pub struct KSEVENT_VIDEODECODER(i32);
pub struct KSEVENT_VOLUMELIMIT(i32);
pub struct KSEVENT_VPNOTIFY(i32);
pub struct KSEVENT_VPVBINOTIFY(i32);
pub struct KSE_NODE(i32);
pub struct KSE_PIN(i32);
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
pub struct KSFRAMETIME(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSFRAMETIME_VARIABLESIZE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct KSGOP_USERDATA(i32);
pub struct KSIDENTIFIER(i32);
pub struct KSINTERFACESETID_FileIo(i32);
pub struct KSINTERFACESETID_Media(i32);
pub struct KSINTERFACESETID_Standard(i32);
pub struct KSINTERFACE_FILEIO(i32);
pub struct KSINTERFACE_MEDIA(i32);
pub struct KSINTERFACE_STANDARD(i32);
pub struct KSINTERVAL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSJACK_DESCRIPTION(i32);
pub struct KSJACK_DESCRIPTION2(i32);
pub struct KSJACK_SINK_CONNECTIONTYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSJACK_SINK_INFORMATION(i32);
pub struct KSMEDIUMSETID_MidiBus(i32);
pub struct KSMEDIUMSETID_Standard(i32);
pub struct KSMEDIUMSETID_VPBus(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMEDIUM_STANDARD_DEVIO: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMEDIUM_TYPE_ANYINSTANCE: u32 = 0u32;
pub struct KSMEMORY_TYPE_DEVICE_UNKNOWN(i32);
pub struct KSMEMORY_TYPE_KERNEL_NONPAGED(i32);
pub struct KSMEMORY_TYPE_KERNEL_PAGED(i32);
pub struct KSMEMORY_TYPE_SYSTEM(i32);
pub struct KSMEMORY_TYPE_USER(i32);
pub struct KSMETHODSETID_StreamAllocator(i32);
pub struct KSMETHODSETID_StreamIo(i32);
pub struct KSMETHODSETID_Wavetable(i32);
pub struct KSMETHOD_STREAMALLOCATOR(i32);
pub struct KSMETHOD_STREAMIO(i32);
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
pub struct KSMETHOD_WAVETABLE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMETHOD_WAVE_QUEUED_BREAKLOOP: u32 = 1u32;
pub struct KSMFT_CATEGORY_AUDIO_DECODER(i32);
pub struct KSMFT_CATEGORY_AUDIO_EFFECT(i32);
pub struct KSMFT_CATEGORY_AUDIO_ENCODER(i32);
pub struct KSMFT_CATEGORY_DEMULTIPLEXER(i32);
pub struct KSMFT_CATEGORY_MULTIPLEXER(i32);
pub struct KSMFT_CATEGORY_OTHER(i32);
pub struct KSMFT_CATEGORY_VIDEO_DECODER(i32);
pub struct KSMFT_CATEGORY_VIDEO_EFFECT(i32);
pub struct KSMFT_CATEGORY_VIDEO_ENCODER(i32);
pub struct KSMFT_CATEGORY_VIDEO_PROCESSOR(i32);
pub struct KSMICARRAY_MICARRAYTYPE(i32);
pub struct KSMICARRAY_MICTYPE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMPEGVIDMODE_LTRBOX: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMPEGVIDMODE_PANSCAN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSMPEGVIDMODE_SCALE: u32 = 4u32;
pub struct KSMPEGVID_RECT(i32);
pub struct KSMULTIPLE_DATA_PROP(i32);
pub struct KSMULTIPLE_ITEM(i32);
pub struct KSMUSICFORMAT(i32);
pub struct KSMUSIC_TECHNOLOGY_FMSYNTH(i32);
pub struct KSMUSIC_TECHNOLOGY_PORT(i32);
pub struct KSMUSIC_TECHNOLOGY_SQSYNTH(i32);
pub struct KSMUSIC_TECHNOLOGY_SWSYNTH(i32);
pub struct KSMUSIC_TECHNOLOGY_WAVETABLE(i32);
pub struct KSM_NODE(i32);
pub struct KSNAME_Allocator(i32);
pub struct KSNAME_Clock(i32);
pub struct KSNAME_Filter(i32);
pub struct KSNAME_Pin(i32);
pub struct KSNAME_TopologyNode(i32);
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
pub struct KSNODEPROPERTY(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER(i32);
#[cfg(any(target_arch = "x86",))]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER(i32);
pub struct KSNODEPROPERTY_AUDIO_CHANNEL(i32);
pub struct KSNODEPROPERTY_AUDIO_DEV_SPECIFIC(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY(i32);
#[cfg(any(target_arch = "x86",))]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY(i32);
pub struct KSNODETYPE_1394_DA_STREAM(i32);
pub struct KSNODETYPE_1394_DV_STREAM_SOUNDTRACK(i32);
pub struct KSNODETYPE_3D_EFFECTS(i32);
pub struct KSNODETYPE_ADC(i32);
pub struct KSNODETYPE_AGC(i32);
pub struct KSNODETYPE_ANALOG_CONNECTOR(i32);
pub struct KSNODETYPE_ANALOG_TAPE(i32);
pub struct KSNODETYPE_AUDIO_ENGINE(i32);
pub struct KSNODETYPE_AUDIO_KEYWORDDETECTOR(i32);
pub struct KSNODETYPE_AUDIO_LOOPBACK(i32);
pub struct KSNODETYPE_AUDIO_MODULE(i32);
pub struct KSNODETYPE_BIDIRECTIONAL_UNDEFINED(i32);
pub struct KSNODETYPE_CABLE_TUNER_AUDIO(i32);
pub struct KSNODETYPE_CD_PLAYER(i32);
pub struct KSNODETYPE_CHORUS(i32);
pub struct KSNODETYPE_COMMUNICATION_SPEAKER(i32);
pub struct KSNODETYPE_DAC(i32);
pub struct KSNODETYPE_DAT_IO_DIGITAL_AUDIO_TAPE(i32);
pub struct KSNODETYPE_DCC_IO_DIGITAL_COMPACT_CASSETTE(i32);
pub struct KSNODETYPE_DELAY(i32);
pub struct KSNODETYPE_DEMUX(i32);
pub struct KSNODETYPE_DESKTOP_MICROPHONE(i32);
pub struct KSNODETYPE_DESKTOP_SPEAKER(i32);
pub struct KSNODETYPE_DEV_SPECIFIC(i32);
pub struct KSNODETYPE_DIGITAL_AUDIO_INTERFACE(i32);
pub struct KSNODETYPE_DISPLAYPORT_INTERFACE(i32);
pub struct KSNODETYPE_DOWN_LINE_PHONE(i32);
pub struct KSNODETYPE_DRM_DESCRAMBLE(i32);
pub struct KSNODETYPE_DSS_AUDIO(i32);
pub struct KSNODETYPE_DVD_AUDIO(i32);
pub struct KSNODETYPE_DYN_RANGE_COMPRESSOR(i32);
pub struct KSNODETYPE_ECHO_CANCELING_SPEAKERPHONE(i32);
pub struct KSNODETYPE_ECHO_SUPPRESSING_SPEAKERPHONE(i32);
pub struct KSNODETYPE_EMBEDDED_UNDEFINED(i32);
pub struct KSNODETYPE_EQUALIZATION_NOISE(i32);
pub struct KSNODETYPE_EQUALIZER(i32);
pub struct KSNODETYPE_EXTERNAL_UNDEFINED(i32);
pub struct KSNODETYPE_FM_RX(i32);
pub struct KSNODETYPE_HANDSET(i32);
pub struct KSNODETYPE_HDMI_INTERFACE(i32);
pub struct KSNODETYPE_HEADPHONES(i32);
pub struct KSNODETYPE_HEADSET(i32);
pub struct KSNODETYPE_HEAD_MOUNTED_DISPLAY_AUDIO(i32);
pub struct KSNODETYPE_INPUT_UNDEFINED(i32);
pub struct KSNODETYPE_LEGACY_AUDIO_CONNECTOR(i32);
pub struct KSNODETYPE_LEVEL_CALIBRATION_NOISE_SOURCE(i32);
pub struct KSNODETYPE_LINE_CONNECTOR(i32);
pub struct KSNODETYPE_LOUDNESS(i32);
pub struct KSNODETYPE_LOW_FREQUENCY_EFFECTS_SPEAKER(i32);
pub struct KSNODETYPE_MICROPHONE(i32);
pub struct KSNODETYPE_MICROPHONE_ARRAY(i32);
pub struct KSNODETYPE_MIDI_ELEMENT(i32);
pub struct KSNODETYPE_MIDI_JACK(i32);
pub struct KSNODETYPE_MINIDISK(i32);
pub struct KSNODETYPE_MULTITRACK_RECORDER(i32);
pub struct KSNODETYPE_MUTE(i32);
pub struct KSNODETYPE_MUX(i32);
pub struct KSNODETYPE_NOISE_SUPPRESS(i32);
pub struct KSNODETYPE_OMNI_DIRECTIONAL_MICROPHONE(i32);
pub struct KSNODETYPE_OUTPUT_UNDEFINED(i32);
pub struct KSNODETYPE_PARAMETRIC_EQUALIZER(i32);
pub struct KSNODETYPE_PEAKMETER(i32);
pub struct KSNODETYPE_PERSONAL_MICROPHONE(i32);
pub struct KSNODETYPE_PHONE_LINE(i32);
pub struct KSNODETYPE_PHONOGRAPH(i32);
pub struct KSNODETYPE_PROCESSING_MICROPHONE_ARRAY(i32);
pub struct KSNODETYPE_PROLOGIC_DECODER(i32);
pub struct KSNODETYPE_PROLOGIC_ENCODER(i32);
pub struct KSNODETYPE_RADIO_RECEIVER(i32);
pub struct KSNODETYPE_RADIO_TRANSMITTER(i32);
pub struct KSNODETYPE_REVERB(i32);
pub struct KSNODETYPE_ROOM_SPEAKER(i32);
pub struct KSNODETYPE_SATELLITE_RECEIVER_AUDIO(i32);
pub struct KSNODETYPE_SPDIF_INTERFACE(i32);
pub struct KSNODETYPE_SPEAKER(i32);
pub struct KSNODETYPE_SPEAKERPHONE_NO_ECHO_REDUCTION(i32);
pub struct KSNODETYPE_SPEAKERS_STATIC_JACK(i32);
pub struct KSNODETYPE_SRC(i32);
pub struct KSNODETYPE_STEREO_WIDE(i32);
pub struct KSNODETYPE_SUM(i32);
pub struct KSNODETYPE_SUPERMIX(i32);
pub struct KSNODETYPE_SYNTHESIZER(i32);
pub struct KSNODETYPE_TELEPHONE(i32);
pub struct KSNODETYPE_TELEPHONY_BIDI(i32);
pub struct KSNODETYPE_TELEPHONY_UNDEFINED(i32);
pub struct KSNODETYPE_TONE(i32);
pub struct KSNODETYPE_TV_TUNER_AUDIO(i32);
pub struct KSNODETYPE_UPDOWN_MIX(i32);
pub struct KSNODETYPE_VCR_AUDIO(i32);
pub struct KSNODETYPE_VIDEO_CAMERA_TERMINAL(i32);
pub struct KSNODETYPE_VIDEO_DISC_AUDIO(i32);
pub struct KSNODETYPE_VIDEO_INPUT_MTT(i32);
pub struct KSNODETYPE_VIDEO_INPUT_TERMINAL(i32);
pub struct KSNODETYPE_VIDEO_OUTPUT_MTT(i32);
pub struct KSNODETYPE_VIDEO_OUTPUT_TERMINAL(i32);
pub struct KSNODETYPE_VIDEO_PROCESSING(i32);
pub struct KSNODETYPE_VIDEO_SELECTOR(i32);
pub struct KSNODETYPE_VIDEO_STREAMING(i32);
pub struct KSNODETYPE_VOLUME(i32);
pub struct KSNODE_CREATE(i32);
pub struct KSNOTIFICATIONID_AudioModule(i32);
pub struct KSNOTIFICATIONID_SoundDetector(i32);
pub struct KSPIN_CINSTANCES(i32);
pub struct KSPIN_COMMUNICATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPIN_CONNECT(i32);
pub struct KSPIN_DATAFLOW(i32);
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
pub struct KSPIN_MDL_CACHING_EVENT(i32);
pub struct KSPIN_MDL_CACHING_NOTIFICATION(i32);
pub struct KSPIN_MDL_CACHING_NOTIFICATION32(i32);
pub struct KSPIN_PHYSICALCONNECTION(i32);
pub struct KSPPROPERTY_ALLOCATOR_MDLCACHING(i32);
pub struct KSPRIORITY(i32);
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
pub struct KSPROPERTYSETID_ExtendedCameraControl(i32);
pub struct KSPROPERTYSETID_NetworkCameraControl(i32);
pub struct KSPROPERTYSETID_PerFrameSettingControl(i32);
pub struct KSPROPERTY_AC3(i32);
pub struct KSPROPERTY_ALLOCATOR_CONTROL(i32);
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S(i32);
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S(i32);
pub struct KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S(i32);
pub struct KSPROPERTY_AUDDECOUT(i32);
pub struct KSPROPERTY_AUDIO(i32);
pub struct KSPROPERTY_AUDIOENGINE(i32);
pub struct KSPROPERTY_AUDIOMODULE(i32);
pub struct KSPROPERTY_AUDIOPOSTURE(i32);
pub struct KSPROPERTY_AUDIORESOURCEMANAGEMENT(i32);
pub struct KSPROPERTY_AUDIOSIGNALPROCESSING(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_AUDIO_BUFFER_DURATION: u32 = 1u32;
pub struct KSPROPERTY_BIBLIOGRAPHIC(i32);
pub struct KSPROPERTY_BOUNDS_LONG(i32);
pub struct KSPROPERTY_BOUNDS_LONGLONG(i32);
pub struct KSPROPERTY_BTAUDIO(i32);
pub struct KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(i32);
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
pub struct KSPROPERTY_CAMERACONTROL_FLASH(i32);
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
pub struct KSPROPERTY_CAMERACONTROL_FLASH_S(i32);
pub struct KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S(i32);
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_EXCLUSIVE_WITH_RECORD: i32 = 1i32;
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_SEQUENCE_EXCLUSIVE_WITH_RECORD: i32 = 2i32;
pub struct KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S(i32);
pub struct KSPROPERTY_CAMERACONTROL_NODE_S(i32);
pub struct KSPROPERTY_CAMERACONTROL_NODE_S2(i32);
pub struct KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(i32);
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S(i32);
pub struct KSPROPERTY_CAMERACONTROL_S(i32);
pub struct KSPROPERTY_CAMERACONTROL_S2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_CAMERACONTROL_S_EX(i32);
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
pub struct KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S(i32);
pub struct KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE(i32);
pub struct KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(i32);
pub struct KSPROPERTY_CLOCK(i32);
pub struct KSPROPERTY_CONNECTION(i32);
pub struct KSPROPERTY_COPYPROT(i32);
pub struct KSPROPERTY_CROSSBAR_ACTIVE_S(i32);
pub struct KSPROPERTY_CROSSBAR_CAPS_S(i32);
pub struct KSPROPERTY_CROSSBAR_PININFO_S(i32);
pub struct KSPROPERTY_CROSSBAR_ROUTE_S(i32);
pub struct KSPROPERTY_CYCLIC(i32);
pub struct KSPROPERTY_DESCRIPTION(i32);
pub struct KSPROPERTY_DIRECTSOUND3DBUFFER(i32);
pub struct KSPROPERTY_DIRECTSOUND3DLISTENER(i32);
pub struct KSPROPERTY_DRMAUDIOSTREAM(i32);
pub struct KSPROPERTY_DROPPEDFRAMES_CURRENT_S(i32);
pub struct KSPROPERTY_DVDSUBPIC(i32);
pub struct KSPROPERTY_EXTDEVICE(i32);
pub struct KSPROPERTY_EXTDEVICE_S(i32);
pub struct KSPROPERTY_EXTENSION_UNIT(i32);
pub struct KSPROPERTY_EXTXPORT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_NODE_S(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_EXTXPORT_S(i32);
pub struct KSPROPERTY_FMRX_CONTROL(i32);
pub struct KSPROPERTY_FMRX_TOPOLOGY(i32);
pub struct KSPROPERTY_GENERAL(i32);
pub struct KSPROPERTY_HRTF3D(i32);
pub struct KSPROPERTY_INTERLEAVEDAUDIO(i32);
pub struct KSPROPERTY_ITD3D(i32);
pub struct KSPROPERTY_JACK(i32);
pub struct KSPROPERTY_MEDIAAVAILABLE(i32);
pub struct KSPROPERTY_MEDIASEEKING(i32);
pub struct KSPROPERTY_MEMBERSHEADER(i32);
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
pub struct KSPROPERTY_MPEG2VID(i32);
pub struct KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES(i32);
pub struct KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO(i32);
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE(i32);
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER(i32);
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(i32);
pub struct KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(i32);
pub struct KSPROPERTY_OVERLAYUPDATE(i32);
pub struct KSPROPERTY_PIN(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_PIN_FLAGS_ATTRIBUTE_RANGE_AWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_PIN_FLAGS_MASK: u32 = 1u32;
pub struct KSPROPERTY_POSITIONS(i32);
pub struct KSPROPERTY_QUALITY(i32);
pub struct KSPROPERTY_RTAUDIO(i32);
pub struct KSPROPERTY_SELECTOR_NODE_S(i32);
pub struct KSPROPERTY_SELECTOR_S(i32);
pub struct KSPROPERTY_SERIAL(i32);
pub struct KSPROPERTY_SERIALHDR(i32);
pub struct KSPROPERTY_SOUNDDETECTOR(i32);
pub struct KSPROPERTY_SPHLI(i32);
pub struct KSPROPERTY_SPPAL(i32);
pub struct KSPROPERTY_STEPPING_LONG(i32);
pub struct KSPROPERTY_STEPPING_LONGLONG(i32);
pub struct KSPROPERTY_STREAM(i32);
pub struct KSPROPERTY_STREAMINTERFACE(i32);
pub struct KSPROPERTY_TELEPHONY_CONTROL(i32);
pub struct KSPROPERTY_TELEPHONY_TOPOLOGY(i32);
pub struct KSPROPERTY_TIMECODE(i32);
pub struct KSPROPERTY_TIMECODE_NODE_S(i32);
pub struct KSPROPERTY_TIMECODE_S(i32);
pub struct KSPROPERTY_TOPOLOGY(i32);
pub struct KSPROPERTY_TOPOLOGYNODE(i32);
pub struct KSPROPERTY_TUNER(i32);
pub struct KSPROPERTY_TUNER_CAPS_S(i32);
pub struct KSPROPERTY_TUNER_FREQUENCY_S(i32);
pub struct KSPROPERTY_TUNER_IF_MEDIUM_S(i32);
pub struct KSPROPERTY_TUNER_INPUT_S(i32);
pub struct KSPROPERTY_TUNER_MODES(i32);
pub struct KSPROPERTY_TUNER_MODE_CAPS_S(i32);
pub struct KSPROPERTY_TUNER_MODE_S(i32);
pub struct KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_TUNER_SCAN_CAPS_S(i32);
pub struct KSPROPERTY_TUNER_SCAN_STATUS_S(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_TUNER_STANDARD_MODE_S(i32);
pub struct KSPROPERTY_TUNER_STANDARD_S(i32);
pub struct KSPROPERTY_TUNER_STATUS_S(i32);
pub struct KSPROPERTY_TVAUDIO_CAPS_S(i32);
pub struct KSPROPERTY_TVAUDIO_S(i32);
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
pub struct KSPROPERTY_VBICAP(i32);
pub struct KSPROPERTY_VBICODECFILTERING(i32);
pub struct KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S(i32);
pub struct KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S(i32);
pub struct KSPROPERTY_VBICODECFILTERING_SCANLINES_S(i32);
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S(i32);
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S(i32);
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S(i32);
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S(i32);
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S(i32);
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S(i32);
pub struct KSPROPERTY_VIDCAP_CAMERACONTROL(i32);
pub struct KSPROPERTY_VIDCAP_CROSSBAR(i32);
pub struct KSPROPERTY_VIDCAP_DROPPEDFRAMES(i32);
pub struct KSPROPERTY_VIDCAP_SELECTOR(i32);
pub struct KSPROPERTY_VIDCAP_TVAUDIO(i32);
pub struct KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(i32);
pub struct KSPROPERTY_VIDCAP_VIDEOCONTROL(i32);
pub struct KSPROPERTY_VIDCAP_VIDEODECODER(i32);
pub struct KSPROPERTY_VIDCAP_VIDEOENCODER(i32);
pub struct KSPROPERTY_VIDCAP_VIDEOPROCAMP(i32);
pub struct KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S(i32);
pub struct KSPROPERTY_VIDEOCOMPRESSION_S(i32);
pub struct KSPROPERTY_VIDEOCOMPRESSION_S1(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S(i32);
pub struct KSPROPERTY_VIDEOCONTROL_CAPS_S(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S(i32);
pub struct KSPROPERTY_VIDEOCONTROL_MODE_S(i32);
pub struct KSPROPERTY_VIDEODECODER_CAPS_S(i32);
pub struct KSPROPERTY_VIDEODECODER_S(i32);
pub struct KSPROPERTY_VIDEODECODER_STATUS2_S(i32);
pub struct KSPROPERTY_VIDEODECODER_STATUS_S(i32);
pub struct KSPROPERTY_VIDEOENCODER_S(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_AUTO: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_MANUAL: i32 = 2i32;
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S(i32);
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S2(i32);
pub struct KSPROPERTY_VIDEOPROCAMP_S(i32);
pub struct KSPROPERTY_VIDEOPROCAMP_S2(i32);
pub struct KSPROPERTY_VIDMEM_TRANSPORT(i32);
pub struct KSPROPERTY_VPCONFIG(i32);
pub struct KSPROPERTY_WAVE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSPROPERTY_WAVE_QUEUED_POSITION: u32 = 1u32;
pub struct KSPROPSETID_AC3(i32);
pub struct KSPROPSETID_Audio(i32);
pub struct KSPROPSETID_AudioBufferDuration(i32);
pub struct KSPROPSETID_AudioDecoderOut(i32);
pub struct KSPROPSETID_AudioEngine(i32);
pub struct KSPROPSETID_AudioModule(i32);
pub struct KSPROPSETID_AudioPosture(i32);
pub struct KSPROPSETID_AudioResourceManagement(i32);
pub struct KSPROPSETID_AudioSignalProcessing(i32);
pub struct KSPROPSETID_Bibliographic(i32);
pub struct KSPROPSETID_BtAudio(i32);
pub struct KSPROPSETID_Clock(i32);
pub struct KSPROPSETID_Connection(i32);
pub struct KSPROPSETID_CopyProt(i32);
pub struct KSPROPSETID_Cyclic(i32);
pub struct KSPROPSETID_DirectSound3DBuffer(i32);
pub struct KSPROPSETID_DirectSound3DListener(i32);
pub struct KSPROPSETID_DrmAudioStream(i32);
pub struct KSPROPSETID_DvdSubPic(i32);
pub struct KSPROPSETID_FMRXControl(i32);
pub struct KSPROPSETID_FMRXTopology(i32);
pub struct KSPROPSETID_General(i32);
pub struct KSPROPSETID_Hrtf3d(i32);
pub struct KSPROPSETID_InterleavedAudio(i32);
pub struct KSPROPSETID_Itd3d(i32);
pub struct KSPROPSETID_Jack(i32);
pub struct KSPROPSETID_MPEG4_MediaType_Attributes(i32);
pub struct KSPROPSETID_MediaSeeking(i32);
pub struct KSPROPSETID_MemoryTransport(i32);
pub struct KSPROPSETID_Mpeg2Vid(i32);
pub struct KSPROPSETID_OverlayUpdate(i32);
pub struct KSPROPSETID_Pin(i32);
pub struct KSPROPSETID_PinMDLCacheClearProp(i32);
pub struct KSPROPSETID_Quality(i32);
pub struct KSPROPSETID_RtAudio(i32);
pub struct KSPROPSETID_SoundDetector(i32);
pub struct KSPROPSETID_SoundDetector2(i32);
pub struct KSPROPSETID_Stream(i32);
pub struct KSPROPSETID_StreamAllocator(i32);
pub struct KSPROPSETID_StreamInterface(i32);
pub struct KSPROPSETID_TSRateChange(i32);
pub struct KSPROPSETID_TelephonyControl(i32);
pub struct KSPROPSETID_TelephonyTopology(i32);
pub struct KSPROPSETID_Topology(i32);
pub struct KSPROPSETID_TopologyNode(i32);
pub struct KSPROPSETID_VBICAP_PROPERTIES(i32);
pub struct KSPROPSETID_VBICodecFiltering(i32);
pub struct KSPROPSETID_VPConfig(i32);
pub struct KSPROPSETID_VPVBIConfig(i32);
pub struct KSPROPSETID_VramCapture(i32);
pub struct KSPROPSETID_Wave(i32);
pub struct KSPROPTYPESETID_General(i32);
pub struct KSP_NODE(i32);
pub struct KSP_PIN(i32);
pub struct KSP_TIMEFORMAT(i32);
pub struct KSQUALITY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSQUALITY_MANAGER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSQUERYBUFFER(i32);
pub struct KSRATE(i32);
pub struct KSRATE_CAPABILITY(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSRATE_NOPRESENTATIONDURATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSRATE_NOPRESENTATIONSTART: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct KSRELATIVEEVENT(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSRELATIVEEVENT_FLAG_HANDLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSRELATIVEEVENT_FLAG_POINTER: u32 = 2u32;
pub struct KSRESET(i32);
pub struct KSRESOLUTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSRTAUDIO_BUFFER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSRTAUDIO_BUFFER32(i32);
pub struct KSRTAUDIO_BUFFER_PROPERTY(i32);
pub struct KSRTAUDIO_BUFFER_PROPERTY32(i32);
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION(i32);
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSRTAUDIO_GETREADPACKET_INFO(i32);
pub struct KSRTAUDIO_HWLATENCY(i32);
pub struct KSRTAUDIO_HWREGISTER(i32);
pub struct KSRTAUDIO_HWREGISTER32(i32);
pub struct KSRTAUDIO_HWREGISTER_PROPERTY(i32);
pub struct KSRTAUDIO_HWREGISTER_PROPERTY32(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY(i32);
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32(i32);
pub struct KSRTAUDIO_PACKETVREGISTER(i32);
pub struct KSRTAUDIO_PACKETVREGISTER_PROPERTY(i32);
pub struct KSRTAUDIO_SETWRITEPACKET_INFO(i32);
pub struct KSSOUNDDETECTORPROPERTY(i32);
pub struct KSSTATE(i32);
pub struct KSSTREAMALLOCATOR_STATUS(i32);
pub struct KSSTREAMALLOCATOR_STATUS_EX(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_FAILUREEXCEPTION: u32 = 8192u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct KSSTREAM_HEADER(i32);
#[cfg(any(target_arch = "x86",))]
pub struct KSSTREAM_HEADER(i32);
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
pub struct KSSTREAM_METADATA_INFO(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_NONPAGED_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_PAGED_DATA: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_READ: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_SYNCHRONOUS: u32 = 4096u32;
pub struct KSSTREAM_UVC_METADATA(i32);
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_UVC_SECURE_ATTRIBUTE_SIZE: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSSTREAM_WRITE: u32 = 1u32;
pub struct KSTELEPHONY_CALLCONTROL(i32);
pub struct KSTELEPHONY_CALLINFO(i32);
pub struct KSTELEPHONY_PROVIDERCHANGE(i32);
pub struct KSTIME(i32);
pub struct KSTIME_FORMAT_BYTE(i32);
pub struct KSTIME_FORMAT_FIELD(i32);
pub struct KSTIME_FORMAT_FRAME(i32);
pub struct KSTIME_FORMAT_MEDIA_TIME(i32);
pub struct KSTIME_FORMAT_SAMPLE(i32);
pub struct KSTOPOLOGY(i32);
pub struct KSTOPOLOGY_CONNECTION(i32);
pub struct KSTOPOLOGY_ENDPOINTID(i32);
pub struct KSTOPOLOGY_ENDPOINTIDPAIR(i32);
pub struct KSVPMAXPIXELRATE(i32);
pub struct KSVPSIZE_PROP(i32);
pub struct KSVPSURFACEPARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KSWAVETABLE_WAVE_DESC(i32);
pub struct KSWAVE_BUFFER(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSWAVE_BUFFER_ATTRIBUTEF_LOOPING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSWAVE_BUFFER_ATTRIBUTEF_STATIC: u32 = 2u32;
pub struct KSWAVE_COMPATCAPS(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSWAVE_COMPATCAPS_INPUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KSWAVE_COMPATCAPS_OUTPUT: u32 = 1u32;
pub struct KSWAVE_INPUT_CAPABILITIES(i32);
pub struct KSWAVE_OUTPUT_CAPABILITIES(i32);
pub struct KSWAVE_VOLUME(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AMCONTROL_COLORINFO_PRESENT: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AMCONTROL_PAD_TO_16x9: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AMCONTROL_PAD_TO_4x3: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AMCONTROL_USED: u32 = 1u32;
pub struct KS_AMPixAspectRatio(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_AMVPDATAINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_AMVPDIMINFO(i32);
pub struct KS_AMVPSIZE(i32);
pub struct KS_AMVP_MODE(i32);
pub struct KS_AMVP_SELECTFORMATBY(i32);
pub struct KS_AM_ExactRateChange(i32);
pub struct KS_AM_PROPERTY_TS_RATE_CHANGE(i32);
pub struct KS_AM_SimpleRateChange(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AM_UseNewCSSKey: i32 = 1i32;
#[cfg(feature = "Win32_Foundation")]
pub struct KS_AnalogVideoInfo(i32);
pub struct KS_AnalogVideoStandard(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AnalogVideo_NTSC_Mask: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AnalogVideo_PAL_Mask: u32 = 1052656u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_AnalogVideo_SECAM_Mask: u32 = 1044480u32;
pub struct KS_BITMAPINFOHEADER(i32);
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
pub struct KS_COLCON(i32);
pub struct KS_COMPRESSION(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_COPYPROTECT_RestrictDuplication: u32 = 1u32;
pub struct KS_COPY_MACROVISION(i32);
pub struct KS_COPY_MACROVISION_LEVEL(i32);
pub struct KS_CameraControlAsyncOperation(i32);
pub struct KS_CompressionCaps(i32);
pub struct KS_DATAFORMAT_H264VIDEOINFO(i32);
pub struct KS_DATAFORMAT_IMAGEINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATAFORMAT_MPEGVIDEOINFO2(i32);
pub struct KS_DATAFORMAT_VBIINFOHEADER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATAFORMAT_VIDEOINFO_PALETTE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_ANALOGVIDEO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_H264_VIDEO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_IMAGE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_MPEG1_VIDEO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_MPEG2_VIDEO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_VIDEO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_VIDEO2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_VIDEO_PALETTE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_DATARANGE_VIDEO_VBI(i32);
pub struct KS_DVDCOPYSTATE(i32);
pub struct KS_DVDCOPY_BUSKEY(i32);
pub struct KS_DVDCOPY_CHLGKEY(i32);
pub struct KS_DVDCOPY_DISCKEY(i32);
pub struct KS_DVDCOPY_REGION(i32);
pub struct KS_DVDCOPY_SET_COPY_STATE(i32);
pub struct KS_DVDCOPY_TITLEKEY(i32);
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
pub struct KS_DVD_YCrCb(i32);
pub struct KS_DVD_YUV(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_FRAME_INFO(i32);
pub struct KS_FRAMING_ITEM(i32);
pub struct KS_FRAMING_RANGE(i32);
pub struct KS_FRAMING_RANGE_WEIGHTED(i32);
pub struct KS_H264VIDEOINFO(i32);
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
pub struct KS_LogicalMemoryType(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const KS_MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140u32;
pub struct KS_MPEAUDIOINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_MPEG1VIDEOINFO(i32);
pub struct KS_MPEG2Level(i32);
pub struct KS_MPEG2Profile(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub struct KS_MPEGVIDEOINFO2(i32);
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
pub struct KS_PhysicalConnectorType(i32);
pub struct KS_RGBQUAD(i32);
pub struct KS_SECURE_CAMERA_SCENARIO_ID(i32);
pub struct KS_SEEKING_CAPABILITIES(i32);
pub struct KS_SEEKING_FLAGS(i32);
pub struct KS_TRUECOLORINFO(i32);
pub struct KS_TUNER_STRATEGY(i32);
pub struct KS_TUNER_TUNING_FLAGS(i32);
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
pub struct KS_TVTUNER_CHANGE_INFO(i32);
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
pub struct KS_VBIINFOHEADER(i32);
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
pub struct KS_VBI_FRAME_INFO(i32);
pub struct KS_VIDEODECODER_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_VIDEOINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_VIDEOINFOHEADER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct KS_VIDEOINFOHEADER2(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub struct KS_VIDEO_STREAM_CONFIG_CAPS(i32);
pub struct KS_VideoControlFlags(i32);
pub struct KS_VideoStreamingHints(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub struct LOOPEDSTREAMING_POSITION_EVENT_DATA(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MAX_NABTS_VBI_LINES_PER_FIELD: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MAX_RESOURCEGROUPID_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MAX_SINK_DESCRIPTION_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MAX_WST_VBI_LINES_PER_FIELD: u32 = 17u32;
#[cfg(feature = "Win32_Foundation")]
pub struct MEDIUM_INFO(i32);
pub struct MF_MDL_SHARED_PAYLOAD_KEY(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MIN_DEV_VER_FOR_FLAGS: u32 = 272u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const MIN_DEV_VER_FOR_QI: u32 = 256u32;
pub struct NABTSFEC_BUFFER(i32);
pub struct NABTS_BUFFER(i32);
pub struct NABTS_BUFFER_LINE(i32);
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
pub struct OPTIMAL_WEIGHT_TOTALS(i32);
pub struct PINNAME_DISPLAYPORT_OUT(i32);
pub struct PINNAME_HDMI_OUT(i32);
pub struct PINNAME_IMAGE(i32);
pub struct PINNAME_SPDIF_IN(i32);
pub struct PINNAME_SPDIF_OUT(i32);
pub struct PINNAME_VIDEO_ANALOGVIDEOIN(i32);
pub struct PINNAME_VIDEO_CAPTURE(i32);
pub struct PINNAME_VIDEO_CC(i32);
pub struct PINNAME_VIDEO_CC_CAPTURE(i32);
pub struct PINNAME_VIDEO_EDS(i32);
pub struct PINNAME_VIDEO_NABTS(i32);
pub struct PINNAME_VIDEO_NABTS_CAPTURE(i32);
pub struct PINNAME_VIDEO_PREVIEW(i32);
pub struct PINNAME_VIDEO_STILL(i32);
pub struct PINNAME_VIDEO_TELETEXT(i32);
pub struct PINNAME_VIDEO_TIMECODE(i32);
pub struct PINNAME_VIDEO_VBI(i32);
pub struct PINNAME_VIDEO_VIDEOPORT(i32);
pub struct PINNAME_VIDEO_VIDEOPORT_VBI(i32);
pub struct PIPE_ALLOCATOR_PLACE(i32);
pub struct PIPE_DIMENSIONS(i32);
pub struct PIPE_STATE(i32);
pub struct PIPE_TERMINATION(i32);
pub struct PROPSETID_ALLOCATOR_CONTROL(i32);
pub struct PROPSETID_EXT_DEVICE(i32);
pub struct PROPSETID_EXT_TRANSPORT(i32);
pub struct PROPSETID_TIMECODE_READER(i32);
pub struct PROPSETID_TUNER(i32);
pub struct PROPSETID_VIDCAP_CAMERACONTROL(i32);
pub struct PROPSETID_VIDCAP_CAMERACONTROL_FLASH(i32);
pub struct PROPSETID_VIDCAP_CAMERACONTROL_IMAGE_PIN_CAPABILITY(i32);
pub struct PROPSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST(i32);
pub struct PROPSETID_VIDCAP_CAMERACONTROL_VIDEO_STABILIZATION(i32);
pub struct PROPSETID_VIDCAP_CROSSBAR(i32);
pub struct PROPSETID_VIDCAP_DROPPEDFRAMES(i32);
pub struct PROPSETID_VIDCAP_SELECTOR(i32);
pub struct PROPSETID_VIDCAP_TVAUDIO(i32);
pub struct PROPSETID_VIDCAP_VIDEOCOMPRESSION(i32);
pub struct PROPSETID_VIDCAP_VIDEOCONTROL(i32);
pub struct PROPSETID_VIDCAP_VIDEODECODER(i32);
pub struct PROPSETID_VIDCAP_VIDEOENCODER(i32);
pub struct PROPSETID_VIDCAP_VIDEOPROCAMP(i32);
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
pub struct SECURE_BUFFER_INFO(i32);
pub struct SOUNDDETECTOR_PATTERNHEADER(i32);
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
pub struct TELEPHONY_CALLCONTROLOP(i32);
pub struct TELEPHONY_CALLSTATE(i32);
pub struct TELEPHONY_CALLTYPE(i32);
pub struct TELEPHONY_PROVIDERCHANGEOP(i32);
pub struct TRANSPORTAUDIOPARMS(i32);
pub struct TRANSPORTBASICPARMS(i32);
pub struct TRANSPORTSTATUS(i32);
pub struct TRANSPORTVIDEOPARMS(i32);
pub struct TRANSPORT_STATE(i32);
pub struct TUNER_ANALOG_CAPS_S(i32);
pub struct VBICAP_PROPERTIES_PROTECTION_S(i32);
pub struct VBICODECFILTERING_CC_SUBSTREAMS(i32);
pub struct VBICODECFILTERING_NABTS_SUBSTREAMS(i32);
pub struct VBICODECFILTERING_SCANLINES(i32);
pub struct VBICODECFILTERING_STATISTICS_CC(i32);
pub struct VBICODECFILTERING_STATISTICS_CC_PIN(i32);
pub struct VBICODECFILTERING_STATISTICS_COMMON(i32);
pub struct VBICODECFILTERING_STATISTICS_COMMON_PIN(i32);
pub struct VBICODECFILTERING_STATISTICS_NABTS(i32);
pub struct VBICODECFILTERING_STATISTICS_NABTS_PIN(i32);
pub struct VBICODECFILTERING_STATISTICS_TELETEXT(i32);
pub struct VBICODECFILTERING_STATISTICS_TELETEXT_PIN(i32);
pub struct VRAM_SURFACE_INFO(i32);
pub struct VRAM_SURFACE_INFO_PROPERTY_S(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const WAVE_FORMAT_EXTENSIBLE: u32 = 65534u32;
pub struct WNF_KSCAMERA_STREAMSTATE_INFO(i32);
pub struct WST_BUFFER(i32);
pub struct WST_BUFFER_LINE(i32);
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const WST_BYTES_PER_LINE: u32 = 42u32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const WST_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Media_KernelStreaming`*"]
pub const WST_TVTUNER_CHANGE_END_TUNE: i32 = 8192i32;
pub struct _KSAUDIO_PACKETSIZE_SIGNALPROCESSINGMODE_CONSTRAINT(i32);
pub struct _TunerDecoderLockType(i32);
