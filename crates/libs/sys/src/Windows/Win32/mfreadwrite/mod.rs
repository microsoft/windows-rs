#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromMediaSink(pmediasink : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsinkwriter : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromURL(pwszoutputurl : windows_sys::core::PCWSTR, pbytestream : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsinkwriter : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromByteStream(pbytestream : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromMediaSource(pmediasource : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromURL(pwszurl : windows_sys::core::PCWSTR, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const CLSID_MFReadWriteClassFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x48e2ed0f_98c2_4a37_bed5_166312ddd83f);
pub const CLSID_MFSinkWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa3bbfb17_8273_4e52_9e0e_9739dc887990);
pub const CLSID_MFSourceReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1777133c_0881_411b_a577_ad545f0714c4);
pub const MF_MEDIASINK_AUTOFINALIZE_SUPPORTED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x48c131be_135a_41cb_8290_03652509c999);
pub const MF_MEDIASINK_ENABLE_AUTOFINALIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x34014265_cb7e_4cde_ac7c_effd3b3c2530);
pub const MF_READWRITE_D3D_OPTIONAL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x216479d9_3071_42ca_bb6c_4c22102e1d18);
pub const MF_READWRITE_DISABLE_CONVERTERS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x98d5b065_1374_4847_8d5d_31520fee7156);
pub const MF_READWRITE_ENABLE_AUTOFINALIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdd7ca129_8cd1_4dc5_9dde_ce168675de61);
pub const MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa634a91c_822b_41b9_a494_4de4643612b0);
pub const MF_READWRITE_MMCSS_CLASS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39384300_d0eb_40b1_87a0_3318871b5a53);
pub const MF_READWRITE_MMCSS_CLASS_AUDIO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x430847da_0890_4b0e_938c_054332c547e1);
pub const MF_READWRITE_MMCSS_PRIORITY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43ad19ce_f33f_4ba9_a580_e4cd12f2d144);
pub const MF_READWRITE_MMCSS_PRIORITY_AUDIO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x273db885_2de2_4db2_a6a7_fdb66fb40b61);
pub const MF_READWRITE_USE_ONLY_HARDWARE_TRANSFORMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf9074427_bf8b_4f69_bbaf_524969056fb6);
pub const MF_SINK_WRITER_ALL_STREAMS: i32 = -2;
pub const MF_SINK_WRITER_ASYNC_CALLBACK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x48cb183e_7b0b_46f4_822e_5e1d2dda4354);
pub const MF_SINK_WRITER_D3D_MANAGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec822da2_e1e9_4b29_a0d8_563c719f5269);
pub const MF_SINK_WRITER_DISABLE_THROTTLING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x08b845d8_2b74_4afe_9d53_be16d2d5ae4f);
pub const MF_SINK_WRITER_ENCODER_CONFIG: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xad91cd04_a7cc_4ac7_99b6_a57b9a4a7c70);
pub const MF_SINK_WRITER_INVALID_STREAM_INDEX: i32 = -1;
pub const MF_SINK_WRITER_MEDIASINK: i32 = -1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MF_SINK_WRITER_STATISTICS {
    pub cb: super::DWORD,
    pub llLastTimestampReceived: super::LONGLONG,
    pub llLastTimestampEncoded: super::LONGLONG,
    pub llLastTimestampProcessed: super::LONGLONG,
    pub llLastStreamTickReceived: super::LONGLONG,
    pub llLastSinkSampleRequest: super::LONGLONG,
    pub qwNumSamplesReceived: super::QWORD,
    pub qwNumSamplesEncoded: super::QWORD,
    pub qwNumSamplesProcessed: super::QWORD,
    pub qwNumStreamTicksReceived: super::QWORD,
    pub dwByteCountQueued: super::DWORD,
    pub qwByteCountProcessed: super::QWORD,
    pub dwNumOutstandingSinkSampleRequests: super::DWORD,
    pub dwAverageSampleRateReceived: super::DWORD,
    pub dwAverageSampleRateEncoded: super::DWORD,
    pub dwAverageSampleRateProcessed: super::DWORD,
}
pub const MF_SOURCE_READERF_ALLEFFECTSREMOVED: MF_SOURCE_READER_FLAG = 512;
pub const MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 32;
pub const MF_SOURCE_READERF_ENDOFSTREAM: MF_SOURCE_READER_FLAG = 2;
pub const MF_SOURCE_READERF_ERROR: MF_SOURCE_READER_FLAG = 1;
pub const MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 16;
pub const MF_SOURCE_READERF_NEWSTREAM: MF_SOURCE_READER_FLAG = 4;
pub const MF_SOURCE_READERF_STREAMTICK: MF_SOURCE_READER_FLAG = 256;
pub const MF_SOURCE_READER_ALL_STREAMS: i32 = -2;
pub const MF_SOURCE_READER_ANY_STREAM: i32 = -2;
pub const MF_SOURCE_READER_ASYNC_CALLBACK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1e3dbeac_bb43_4c35_b507_cd644464c965);
pub const MF_SOURCE_READER_CONTROLF_DRAIN: MF_SOURCE_READER_CONTROL_FLAG = 1;
pub type MF_SOURCE_READER_CONTROL_FLAG = i32;
pub const MF_SOURCE_READER_CURRENT_TYPE_INDEX: i32 = -1;
pub const MF_SOURCE_READER_D3D11_BIND_FLAGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33f3197b_f73a_4e14_8d85_0e4c4368788d);
pub const MF_SOURCE_READER_D3D_MANAGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec822da2_e1e9_4b29_a0d8_563c719f5269);
pub const MF_SOURCE_READER_DISABLE_CAMERA_PLUGINS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9d3365dd_058f_4cfb_9f97_b314cc99c8ad);
pub const MF_SOURCE_READER_DISABLE_DXVA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa456cfd_3943_4a1e_a77d_1838c0ea2e35);
pub const MF_SOURCE_READER_DISCONNECT_MEDIASOURCE_ON_SHUTDOWN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56b67165_219e_456d_a22e_2d3004c7fe56);
pub const MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0f81da2c_b537_4672_a8b2_a681b17307a3);
pub const MF_SOURCE_READER_ENABLE_TRANSCODE_ONLY_TRANSFORMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdfd4f008_b5fd_4e78_ae44_62a1e67bbe27);
pub const MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb394f3d_ccf1_42ee_bbb3_f9b845d5681d);
pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: i32 = -3;
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: i32 = -4;
pub type MF_SOURCE_READER_FLAG = i32;
pub const MF_SOURCE_READER_INVALID_STREAM_INDEX: i32 = -1;
pub const MF_SOURCE_READER_MEDIASOURCE: i32 = -1;
pub const MF_SOURCE_READER_MEDIASOURCE_CHARACTERISTICS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d23f5c8_c5d7_4a9b_9971_5d11f8bca880);
pub const MF_SOURCE_READER_MEDIASOURCE_CONFIG: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9085abeb_0354_48f9_abb5_200df838c68e);
pub const MF_SOURCE_READER_PASSTHROUGH_MODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x043ff126_fe2c_4708_a09b_da2ab435ced9);
