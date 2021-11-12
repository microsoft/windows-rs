#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn WMCreateBackupRestorer(pcallback: ::windows_sys::core::IUnknown, ppbackup: *mut IWMLicenseBackup) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateEditor(ppeditor: *mut IWMMetadataEditor) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateIndexer(ppindexer: *mut IWMIndexer) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateProfileManager(ppprofilemanager: *mut IWMProfileManager) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateReader(punkcert: ::windows_sys::core::IUnknown, dwrights: u32, ppreader: *mut IWMReader) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateSyncReader(punkcert: ::windows_sys::core::IUnknown, dwrights: u32, ppsyncreader: *mut IWMSyncReader) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateWriter(punkcert: ::windows_sys::core::IUnknown, ppwriter: *mut IWMWriter) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateWriterFileSink(ppsink: *mut IWMWriterFileSink) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateWriterNetworkSink(ppsink: *mut IWMWriterNetworkSink) -> ::windows_sys::core::HRESULT;
    pub fn WMCreateWriterPushSink(ppsink: *mut IWMWriterPushSink) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WMIsContentProtected(pwszfilename: super::super::Foundation::PWSTR, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
}
pub struct AM_WMT_EVENT_DATA(i32);
pub const CLSID_ClientNetManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3440550862, data2: 40002, data3: 4562, data4: [190, 237, 0, 96, 8, 47, 32, 84] };
pub const CLSID_WMBandwidthSharing_Exclusive: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2942329002, data2: 20887, data3: 4562, data4: [182, 175, 0, 192, 79, 217, 8, 233] };
pub const CLSID_WMBandwidthSharing_Partial: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2942329003, data2: 20887, data3: 4562, data4: [182, 175, 0, 192, 79, 217, 8, 233] };
pub const CLSID_WMMUTEX_Bitrate: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3605146113, data2: 13786, data3: 4561, data4: [144, 52, 0, 160, 201, 3, 73, 190] };
pub const CLSID_WMMUTEX_Language: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3605146112, data2: 13786, data3: 4561, data4: [144, 52, 0, 160, 201, 3, 73, 190] };
pub const CLSID_WMMUTEX_Presentation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3605146114, data2: 13786, data3: 4561, data4: [144, 52, 0, 160, 201, 3, 73, 190] };
pub const CLSID_WMMUTEX_Unknown: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3605146115, data2: 13786, data3: 4561, data4: [144, 52, 0, 160, 201, 3, 73, 190] };
pub struct DRM_COPY_OPL(i32);
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS(i32);
pub struct DRM_OPL_OUTPUT_IDS(i32);
pub const DRM_OPL_TYPES: u32 = 1u32;
pub struct DRM_OUTPUT_PROTECTION(i32);
pub struct DRM_PLAY_OPL(i32);
pub struct DRM_VAL16(i32);
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS(i32);
#[repr(transparent)]
pub struct IAMWMBufferPass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMWMBufferPassCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INSNetSourceCreator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INSSBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INSSBuffer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INSSBuffer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INSSBuffer4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMAddressAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMAddressAccess2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMAuthorizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMBackupRestoreProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMBandwidthSharing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMClientConnections(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMClientConnections2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMCodecAMVideoAccelerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMCodecInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMCodecInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMCodecInfo3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMCodecVideoAccelerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMCredentialCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMEditor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMMessageParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMReader3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMTranscryptionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMTranscryptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMTranscryptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMWriter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDRMWriter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDeviceRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMGetSecureChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMHeaderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMHeaderInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMHeaderInfo3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMIStreamProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMImageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMIndexer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMIndexer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMInputMediaProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMLanguageList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMLicenseBackup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMLicenseRestore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMLicenseRevocationAgent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMMediaProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMMetadataEditor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMMetadataEditor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMMutualExclusion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMMutualExclusion2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMOutputMediaProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMPacketSize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMPacketSize2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMPlayerHook(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMPlayerTimestampHook(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMProfile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMProfile3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMProfileManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMProfileManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMProfileManagerLanguage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMPropertyVault(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMProximityDetection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderAccelerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderAdvanced(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderAdvanced2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderAdvanced3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderAdvanced4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderAdvanced5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderAdvanced6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderAllocatorEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderCallbackAdvanced(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderNetworkConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderNetworkConfig2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderPlaylistBurn(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderStreamClock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderTimecode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMReaderTypeNegotiation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMRegisterCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMRegisteredDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMSBufferAllocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMSecureChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMStatusCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMStreamConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMStreamConfig2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMStreamConfig3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMStreamList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMStreamPrioritization(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMSyncReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMSyncReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMVideoMediaProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWatermarkInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterAdvanced(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterAdvanced2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterAdvanced3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterFileSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterFileSink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterFileSink3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterNetworkSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterPostView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterPostViewCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterPreprocess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterPushSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMWriterSink(pub *mut ::core::ffi::c_void);
pub struct NETSOURCE_URLCREDPOLICY_SETTINGS(i32);
pub struct WEBSTREAM_SAMPLE_TYPE(i32);
pub struct WMDRM_IMPORT_INIT_STRUCT(i32);
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1u32;
pub const WMFORMAT_MPEG2Video: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3765272803, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const WMFORMAT_Script: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1552224498,
    data2: 57022,
    data3: 19623,
    data4: [187, 165, 240, 122, 16, 79, 141, 255],
};
pub const WMFORMAT_VideoInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 89694080, data2: 50006, data3: 4558, data4: [191, 1, 0, 170, 0, 85, 89, 90] };
pub const WMFORMAT_WaveFormatEx: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 89694081, data2: 50006, data3: 4558, data4: [191, 1, 0, 170, 0, 85, 89, 90] };
pub const WMFORMAT_WebStream: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3659426579,
    data2: 33625,
    data3: 16464,
    data4: [179, 152, 56, 142, 150, 91, 240, 12],
};
pub const WMMEDIASUBTYPE_ACELPnet: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 304, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_Base: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 0, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_DRM: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 9, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_I420: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 808596553, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_IYUV: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1448433993, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_M4S2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 844313677, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MP3: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 85, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MP43: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 859066445, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MP4S: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1395937357, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3765272614, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const WMMEDIASUBTYPE_MSS1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 827544397, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MSS2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 844321613, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_P422: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 842150992, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_PCM: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_RGB1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3828804472, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB24: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3828804477, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB32: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3828804478, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB4: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3828804473, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB555: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3828804476, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB565: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3828804475, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB8: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3828804474, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_UYVY: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1498831189, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_VIDEOIMAGE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 491406834, data2: 58870, data3: 19268, data4: [131, 136, 240, 174, 92, 14, 12, 55] };
pub const WMMEDIASUBTYPE_WMAudioV2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMAudioV7: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMAudioV8: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMAudioV9: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 354, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMAudio_Lossless: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 355, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMSP1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 10, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMSP2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 11, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMV1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 827739479, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMV2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 844516695, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMV3: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 861293911, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMVA: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1096174935, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMVP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1347833175, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WVC1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 826496599, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WVP2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 844125783, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WebStream: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2002933716,
    data2: 50727,
    data3: 16843,
    data4: [143, 129, 122, 199, 255, 28, 64, 204],
};
pub const WMMEDIASUBTYPE_YUY2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 844715353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_YV12: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 842094169, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_YVU9: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 961893977, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_YVYU: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1431918169, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIATYPE_Audio: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1935963489, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIATYPE_FileTransfer: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3655628153,
    data2: 37646,
    data3: 17447,
    data4: [173, 252, 173, 128, 242, 144, 228, 112],
};
pub const WMMEDIATYPE_Image: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 883232728,
    data2: 35493,
    data3: 17286,
    data4: [129, 254, 160, 239, 224, 72, 142, 49],
};
pub const WMMEDIATYPE_Script: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1935895908, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIATYPE_Text: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2612666023, data2: 23218, data3: 18473, data4: [186, 87, 9, 64, 32, 155, 207, 62] };
pub const WMMEDIATYPE_Video: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1935960438, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMMPEG2VIDEOINFO(i32);
pub struct WMSCRIPTFORMAT(i32);
pub const WMSCRIPTTYPE_TwoStrings: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2196998768, data2: 49823, data3: 4561, data4: [151, 173, 0, 160, 201, 94, 168, 80] };
pub struct WMT_ATTR_DATATYPE(i32);
pub struct WMT_ATTR_IMAGETYPE(i32);
pub struct WMT_BUFFER_SEGMENT(i32);
pub struct WMT_CODEC_INFO_TYPE(i32);
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA(i32);
pub struct WMT_CREDENTIAL_FLAGS(i32);
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1696734298, data2: 64117, data3: 19257, data4: [181, 12, 6, 195, 54, 182, 163, 239] };
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 410831138,
    data2: 36604,
    data3: 17412,
    data4: [157, 175, 99, 244, 131, 13, 241, 188],
};
pub struct WMT_DRMLA_TRUST(i32);
pub struct WMT_FILESINK_DATA_UNIT(i32);
pub struct WMT_FILESINK_MODE(i32);
pub struct WMT_IMAGE_TYPE(i32);
pub struct WMT_INDEXER_TYPE(i32);
pub struct WMT_INDEX_TYPE(i32);
pub struct WMT_MUSICSPEECH_CLASS_MODE(i32);
pub struct WMT_NET_PROTOCOL(i32);
pub struct WMT_OFFSET_FORMAT(i32);
pub struct WMT_PAYLOAD_FRAGMENT(i32);
pub struct WMT_PLAY_MODE(i32);
pub struct WMT_PROXY_SETTINGS(i32);
pub struct WMT_RIGHTS(i32);
pub struct WMT_STATUS(i32);
pub struct WMT_STORAGE_FORMAT(i32);
pub struct WMT_STREAM_SELECTION(i32);
pub struct WMT_TIMECODE_EXTENSION_DATA(i32);
pub struct WMT_TIMECODE_FRAMERATE(i32);
pub struct WMT_TRANSPORT_TYPE(i32);
pub struct WMT_VERSION(i32);
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536i32;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834u32;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835u32;
pub struct WMT_VIDEOIMAGE_SAMPLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WMT_VIDEOIMAGE_SAMPLE2(i32);
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8u32;
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4u32;
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1u32;
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1u32;
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2u32;
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2u32;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4u32;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8u32;
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11u32;
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12u32;
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13u32;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14u32;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18u32;
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19u32;
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20u32;
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21u32;
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23u32;
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24u32;
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27u32;
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29u32;
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30u32;
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WMT_WATERMARK_ENTRY(i32);
pub struct WMT_WATERMARK_ENTRY_TYPE(i32);
pub struct WMT_WEBSTREAM_FORMAT(i32);
pub struct WMT_WEBSTREAM_SAMPLE_HEADER(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER2(i32);
pub struct WM_ADDRESS_ACCESSENTRY(i32);
pub struct WM_AETYPE(i32);
pub struct WM_CLIENT_PROPERTIES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WM_CLIENT_PROPERTIES_EX(i32);
pub const WM_CL_INTERLACED420: u32 = 0u32;
pub const WM_CL_PROGRESSIVE420: u32 = 1u32;
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32u32;
pub const WM_CT_INTERLACED: u32 = 128u32;
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16u32;
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64u32;
pub struct WM_DM_INTERLACED_TYPE(i32);
pub struct WM_DM_IT_FIRST_FRAME_COHERENCY(i32);
pub struct WM_LEAKY_BUCKET_PAIR(i32);
pub const WM_MAX_STREAMS: u32 = 63u32;
pub const WM_MAX_VIDEO_STREAMS: u32 = 63u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WM_MEDIA_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WM_PICTURE(i32);
pub struct WM_PLAYBACK_DRC_LEVEL(i32);
pub struct WM_PORT_NUMBER_RANGE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WM_READER_CLIENTINFO(i32);
pub struct WM_READER_STATISTICS(i32);
pub struct WM_SFEX_TYPE(i32);
pub struct WM_SF_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WM_STREAM_PRIORITY_RECORD(i32);
pub struct WM_STREAM_TYPE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WM_SYNCHRONISED_LYRICS(i32);
pub const WM_SampleExtensionGUID_ChromaLocation: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1281019040,
    data2: 37494,
    data3: 19244,
    data4: [158, 76, 160, 237, 239, 221, 33, 126],
};
pub const WM_SampleExtensionGUID_ColorSpaceInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4154120790, data2: 12523, data3: 20267, data4: [159, 122, 242, 75, 19, 154, 17, 87] };
pub const WM_SampleExtensionGUID_ContentType: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3583040544,
    data2: 1980,
    data3: 17260,
    data4: [156, 247, 243, 187, 251, 241, 164, 220],
};
pub const WM_SampleExtensionGUID_FileName: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3781553166,
    data2: 6637,
    data3: 17879,
    data4: [180, 167, 37, 203, 209, 226, 142, 155],
};
pub const WM_SampleExtensionGUID_OutputCleanPoint: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4146740335, data2: 28340, data3: 20156, data4: [177, 146, 9, 173, 151, 89, 232, 40] };
pub const WM_SampleExtensionGUID_PixelAspectRatio: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 455009620,
    data2: 63978,
    data3: 19400,
    data4: [130, 26, 55, 107, 116, 228, 196, 184],
};
pub const WM_SampleExtensionGUID_SampleDuration: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3334313040,
    data2: 34431,
    data3: 18695,
    data4: [131, 163, 199, 121, 33, 183, 51, 173],
};
pub const WM_SampleExtensionGUID_SampleProtectionSalt: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1409539822, data2: 47598, data3: 17295, data4: [170, 131, 56, 4, 153, 126, 86, 157] };
pub const WM_SampleExtensionGUID_Timecode: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 966104556,
    data2: 34407,
    data3: 20013,
    data4: [143, 219, 152, 129, 76, 231, 108, 30],
};
pub const WM_SampleExtensionGUID_UserDataInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1932244218, data2: 30910, data3: 17737, data4: [153, 189, 2, 219, 26, 85, 183, 168] };
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1u32;
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3u32;
pub const WM_SampleExtension_ContentType_Size: u32 = 1u32;
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2u32;
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2u32;
pub const WM_SampleExtension_Timecode_Size: u32 = 14u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WM_USER_TEXT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WM_USER_WEB_URL(i32);
pub struct WM_WRITER_STATISTICS(i32);
pub struct WM_WRITER_STATISTICS_EX(i32);
pub struct _AM_ASFWRITERCONFIG_PARAM(i32);
pub const g_dwWMContentAttributes: u32 = 5u32;
pub const g_dwWMNSCAttributes: u32 = 5u32;
pub const g_dwWMSpecialAttributes: u32 = 20u32;
pub const g_wszASFLeakyBucketPairs: &'static str = "ASFLeakyBucketPairs";
pub const g_wszAllowInterlacedOutput: &'static str = "AllowInterlacedOutput";
pub const g_wszAverageLevel: &'static str = "AverageLevel";
pub const g_wszBufferAverage: &'static str = "Buffer Average";
pub const g_wszComplexity: &'static str = "_COMPLEXITYEX";
pub const g_wszComplexityLive: &'static str = "_COMPLEXITYEXLIVE";
pub const g_wszComplexityMax: &'static str = "_COMPLEXITYEXMAX";
pub const g_wszComplexityOffline: &'static str = "_COMPLEXITYEXOFFLINE";
pub const g_wszDecoderComplexityRequested: &'static str = "_DECODERCOMPLEXITYPROFILE";
pub const g_wszDedicatedDeliveryThread: &'static str = "DedicatedDeliveryThread";
pub const g_wszDeinterlaceMode: &'static str = "DeinterlaceMode";
pub const g_wszDeliverOnReceive: &'static str = "DeliverOnReceive";
pub const g_wszDeviceConformanceTemplate: &'static str = "DeviceConformanceTemplate";
pub const g_wszDynamicRangeControl: &'static str = "DynamicRangeControl";
pub const g_wszEDL: &'static str = "_EDL";
pub const g_wszEarlyDataDelivery: &'static str = "EarlyDataDelivery";
pub const g_wszEnableDiscreteOutput: &'static str = "EnableDiscreteOutput";
pub const g_wszEnableFrameInterpolation: &'static str = "EnableFrameInterpolation";
pub const g_wszEnableWMAProSPDIFOutput: &'static str = "EnableWMAProSPDIFOutput";
pub const g_wszFailSeekOnError: &'static str = "FailSeekOnError";
pub const g_wszFixedFrameRate: &'static str = "FixedFrameRate";
pub const g_wszFold6To2Channels3: &'static str = "Fold6To2Channels3";
pub const g_wszFoldToChannelsTemplate: &'static str = "Fold%luTo%luChannels%lu";
pub const g_wszInitialPatternForInverseTelecine: &'static str = "InitialPatternForInverseTelecine";
pub const g_wszInterlacedCoding: &'static str = "InterlacedCoding";
pub const g_wszIsVBRSupported: &'static str = "_ISVBRSUPPORTED";
pub const g_wszJPEGCompressionQuality: &'static str = "JPEGCompressionQuality";
pub const g_wszJustInTimeDecode: &'static str = "JustInTimeDecode";
pub const g_wszMixedClassMode: &'static str = "MixedClassMode";
pub const g_wszMusicClassMode: &'static str = "MusicClassMode";
pub const g_wszMusicSpeechClassMode: &'static str = "MusicSpeechClassMode";
pub const g_wszNeedsPreviousSample: &'static str = "NeedsPreviousSample";
pub const g_wszNumPasses: &'static str = "_PASSESUSED";
pub const g_wszOriginalSourceFormatTag: &'static str = "_SOURCEFORMATTAG";
pub const g_wszOriginalWaveFormat: &'static str = "_ORIGINALWAVEFORMAT";
pub const g_wszPeakValue: &'static str = "PeakValue";
pub const g_wszPermitSeeksBeyondEndOfStream: &'static str = "PermitSeeksBeyondEndOfStream";
pub const g_wszReloadIndexOnSeek: &'static str = "ReloadIndexOnSeek";
pub const g_wszScrambledAudio: &'static str = "ScrambledAudio";
pub const g_wszSingleOutputBuffer: &'static str = "SingleOutputBuffer";
pub const g_wszSoftwareScaling: &'static str = "SoftwareScaling";
pub const g_wszSourceBufferTime: &'static str = "SourceBufferTime";
pub const g_wszSourceMaxBytesAtOnce: &'static str = "SourceMaxBytesAtOnce";
pub const g_wszSpeakerConfig: &'static str = "SpeakerConfig";
pub const g_wszSpeechCaps: &'static str = "SpeechFormatCap";
pub const g_wszSpeechClassMode: &'static str = "SpeechClassMode";
pub const g_wszStreamLanguage: &'static str = "StreamLanguage";
pub const g_wszStreamNumIndexObjects: &'static str = "StreamNumIndexObjects";
pub const g_wszUsePacketAtSeekPoint: &'static str = "UsePacketAtSeekPoint";
pub const g_wszVBRBitrateMax: &'static str = "_RMAX";
pub const g_wszVBRBufferWindowMax: &'static str = "_BMAX";
pub const g_wszVBREnabled: &'static str = "_VBRENABLED";
pub const g_wszVBRPeak: &'static str = "VBR Peak";
pub const g_wszVBRQuality: &'static str = "_VBRQUALITY";
pub const g_wszVideoSampleDurations: &'static str = "VideoSampleDurations";
pub const g_wszWMADID: &'static str = "WM/ADID";
pub const g_wszWMASFPacketCount: &'static str = "WM/ASFPacketCount";
pub const g_wszWMASFSecurityObjectsSize: &'static str = "WM/ASFSecurityObjectsSize";
pub const g_wszWMAlbumArtist: &'static str = "WM/AlbumArtist";
pub const g_wszWMAlbumArtistSort: &'static str = "WM/AlbumArtistSort";
pub const g_wszWMAlbumCoverURL: &'static str = "WM/AlbumCoverURL";
pub const g_wszWMAlbumTitle: &'static str = "WM/AlbumTitle";
pub const g_wszWMAlbumTitleSort: &'static str = "WM/AlbumTitleSort";
pub const g_wszWMAspectRatioX: &'static str = "AspectRatioX";
pub const g_wszWMAspectRatioY: &'static str = "AspectRatioY";
pub const g_wszWMAudioFileURL: &'static str = "WM/AudioFileURL";
pub const g_wszWMAudioSourceURL: &'static str = "WM/AudioSourceURL";
pub const g_wszWMAuthor: &'static str = "Author";
pub const g_wszWMAuthorSort: &'static str = "AuthorSort";
pub const g_wszWMAuthorURL: &'static str = "WM/AuthorURL";
pub const g_wszWMBannerImageData: &'static str = "BannerImageData";
pub const g_wszWMBannerImageType: &'static str = "BannerImageType";
pub const g_wszWMBannerImageURL: &'static str = "BannerImageURL";
pub const g_wszWMBeatsPerMinute: &'static str = "WM/BeatsPerMinute";
pub const g_wszWMBitrate: &'static str = "Bitrate";
pub const g_wszWMBroadcast: &'static str = "Broadcast";
pub const g_wszWMCategory: &'static str = "WM/Category";
pub const g_wszWMCodec: &'static str = "WM/Codec";
pub const g_wszWMComposer: &'static str = "WM/Composer";
pub const g_wszWMComposerSort: &'static str = "WM/ComposerSort";
pub const g_wszWMConductor: &'static str = "WM/Conductor";
pub const g_wszWMContainerFormat: &'static str = "WM/ContainerFormat";
pub const g_wszWMContentDistributor: &'static str = "WM/ContentDistributor";
pub const g_wszWMContentGroupDescription: &'static str = "WM/ContentGroupDescription";
pub const g_wszWMCopyright: &'static str = "Copyright";
pub const g_wszWMCopyrightURL: &'static str = "CopyrightURL";
pub const g_wszWMCurrentBitrate: &'static str = "CurrentBitrate";
pub const g_wszWMDRM: &'static str = "WM/DRM";
pub const g_wszWMDRM_ContentID: &'static str = "DRM_ContentID";
pub const g_wszWMDRM_Flags: &'static str = "DRM_Flags";
pub const g_wszWMDRM_HeaderSignPrivKey: &'static str = "DRM_HeaderSignPrivKey";
pub const g_wszWMDRM_IndividualizedVersion: &'static str = "DRM_IndividualizedVersion";
pub const g_wszWMDRM_KeyID: &'static str = "DRM_KeyID";
pub const g_wszWMDRM_KeySeed: &'static str = "DRM_KeySeed";
pub const g_wszWMDRM_LASignatureCert: &'static str = "DRM_LASignatureCert";
pub const g_wszWMDRM_LASignatureLicSrvCert: &'static str = "DRM_LASignatureLicSrvCert";
pub const g_wszWMDRM_LASignaturePrivKey: &'static str = "DRM_LASignaturePrivKey";
pub const g_wszWMDRM_LASignatureRootCert: &'static str = "DRM_LASignatureRootCert";
pub const g_wszWMDRM_Level: &'static str = "DRM_Level";
pub const g_wszWMDRM_LicenseAcqURL: &'static str = "DRM_LicenseAcqURL";
pub const g_wszWMDRM_SourceID: &'static str = "DRM_SourceID";
pub const g_wszWMDRM_V1LicenseAcqURL: &'static str = "DRM_V1LicenseAcqURL";
pub const g_wszWMDVDID: &'static str = "WM/DVDID";
pub const g_wszWMDescription: &'static str = "Description";
pub const g_wszWMDirector: &'static str = "WM/Director";
pub const g_wszWMDuration: &'static str = "Duration";
pub const g_wszWMEncodedBy: &'static str = "WM/EncodedBy";
pub const g_wszWMEncodingSettings: &'static str = "WM/EncodingSettings";
pub const g_wszWMEncodingTime: &'static str = "WM/EncodingTime";
pub const g_wszWMEpisodeNumber: &'static str = "WM/EpisodeNumber";
pub const g_wszWMFileSize: &'static str = "FileSize";
pub const g_wszWMGenre: &'static str = "WM/Genre";
pub const g_wszWMGenreID: &'static str = "WM/GenreID";
pub const g_wszWMHasArbitraryDataStream: &'static str = "HasArbitraryDataStream";
pub const g_wszWMHasAttachedImages: &'static str = "HasAttachedImages";
pub const g_wszWMHasAudio: &'static str = "HasAudio";
pub const g_wszWMHasFileTransferStream: &'static str = "HasFileTransferStream";
pub const g_wszWMHasImage: &'static str = "HasImage";
pub const g_wszWMHasScript: &'static str = "HasScript";
pub const g_wszWMHasVideo: &'static str = "HasVideo";
pub const g_wszWMISAN: &'static str = "WM/ISAN";
pub const g_wszWMISRC: &'static str = "WM/ISRC";
pub const g_wszWMInitialKey: &'static str = "WM/InitialKey";
pub const g_wszWMIsCompilation: &'static str = "WM/IsCompilation";
pub const g_wszWMIsVBR: &'static str = "IsVBR";
pub const g_wszWMLanguage: &'static str = "WM/Language";
pub const g_wszWMLyrics: &'static str = "WM/Lyrics";
pub const g_wszWMLyrics_Synchronised: &'static str = "WM/Lyrics_Synchronised";
pub const g_wszWMMCDI: &'static str = "WM/MCDI";
pub const g_wszWMMediaClassPrimaryID: &'static str = "WM/MediaClassPrimaryID";
pub const g_wszWMMediaClassSecondaryID: &'static str = "WM/MediaClassSecondaryID";
pub const g_wszWMMediaCredits: &'static str = "WM/MediaCredits";
pub const g_wszWMMediaIsDelay: &'static str = "WM/MediaIsDelay";
pub const g_wszWMMediaIsFinale: &'static str = "WM/MediaIsFinale";
pub const g_wszWMMediaIsLive: &'static str = "WM/MediaIsLive";
pub const g_wszWMMediaIsPremiere: &'static str = "WM/MediaIsPremiere";
pub const g_wszWMMediaIsRepeat: &'static str = "WM/MediaIsRepeat";
pub const g_wszWMMediaIsSAP: &'static str = "WM/MediaIsSAP";
pub const g_wszWMMediaIsStereo: &'static str = "WM/MediaIsStereo";
pub const g_wszWMMediaIsSubtitled: &'static str = "WM/MediaIsSubtitled";
pub const g_wszWMMediaIsTape: &'static str = "WM/MediaIsTape";
pub const g_wszWMMediaNetworkAffiliation: &'static str = "WM/MediaNetworkAffiliation";
pub const g_wszWMMediaOriginalBroadcastDateTime: &'static str = "WM/MediaOriginalBroadcastDateTime";
pub const g_wszWMMediaOriginalChannel: &'static str = "WM/MediaOriginalChannel";
pub const g_wszWMMediaStationCallSign: &'static str = "WM/MediaStationCallSign";
pub const g_wszWMMediaStationName: &'static str = "WM/MediaStationName";
pub const g_wszWMModifiedBy: &'static str = "WM/ModifiedBy";
pub const g_wszWMMood: &'static str = "WM/Mood";
pub const g_wszWMNSCAddress: &'static str = "NSC_Address";
pub const g_wszWMNSCDescription: &'static str = "NSC_Description";
pub const g_wszWMNSCEmail: &'static str = "NSC_Email";
pub const g_wszWMNSCName: &'static str = "NSC_Name";
pub const g_wszWMNSCPhone: &'static str = "NSC_Phone";
pub const g_wszWMNumberOfFrames: &'static str = "NumberOfFrames";
pub const g_wszWMOptimalBitrate: &'static str = "OptimalBitrate";
pub const g_wszWMOriginalAlbumTitle: &'static str = "WM/OriginalAlbumTitle";
pub const g_wszWMOriginalArtist: &'static str = "WM/OriginalArtist";
pub const g_wszWMOriginalFilename: &'static str = "WM/OriginalFilename";
pub const g_wszWMOriginalLyricist: &'static str = "WM/OriginalLyricist";
pub const g_wszWMOriginalReleaseTime: &'static str = "WM/OriginalReleaseTime";
pub const g_wszWMOriginalReleaseYear: &'static str = "WM/OriginalReleaseYear";
pub const g_wszWMParentalRating: &'static str = "WM/ParentalRating";
pub const g_wszWMParentalRatingReason: &'static str = "WM/ParentalRatingReason";
pub const g_wszWMPartOfSet: &'static str = "WM/PartOfSet";
pub const g_wszWMPeakBitrate: &'static str = "WM/PeakBitrate";
pub const g_wszWMPeriod: &'static str = "WM/Period";
pub const g_wszWMPicture: &'static str = "WM/Picture";
pub const g_wszWMPlaylistDelay: &'static str = "WM/PlaylistDelay";
pub const g_wszWMProducer: &'static str = "WM/Producer";
pub const g_wszWMPromotionURL: &'static str = "WM/PromotionURL";
pub const g_wszWMProtected: &'static str = "Is_Protected";
pub const g_wszWMProtectionType: &'static str = "WM/ProtectionType";
pub const g_wszWMProvider: &'static str = "WM/Provider";
pub const g_wszWMProviderCopyright: &'static str = "WM/ProviderCopyright";
pub const g_wszWMProviderRating: &'static str = "WM/ProviderRating";
pub const g_wszWMProviderStyle: &'static str = "WM/ProviderStyle";
pub const g_wszWMPublisher: &'static str = "WM/Publisher";
pub const g_wszWMRadioStationName: &'static str = "WM/RadioStationName";
pub const g_wszWMRadioStationOwner: &'static str = "WM/RadioStationOwner";
pub const g_wszWMRating: &'static str = "Rating";
pub const g_wszWMSeasonNumber: &'static str = "WM/SeasonNumber";
pub const g_wszWMSeekable: &'static str = "Seekable";
pub const g_wszWMSharedUserRating: &'static str = "WM/SharedUserRating";
pub const g_wszWMSignature_Name: &'static str = "Signature_Name";
pub const g_wszWMSkipBackward: &'static str = "Can_Skip_Backward";
pub const g_wszWMSkipForward: &'static str = "Can_Skip_Forward";
pub const g_wszWMStreamTypeInfo: &'static str = "WM/StreamTypeInfo";
pub const g_wszWMStridable: &'static str = "Stridable";
pub const g_wszWMSubTitle: &'static str = "WM/SubTitle";
pub const g_wszWMSubTitleDescription: &'static str = "WM/SubTitleDescription";
pub const g_wszWMSubscriptionContentID: &'static str = "WM/SubscriptionContentID";
pub const g_wszWMText: &'static str = "WM/Text";
pub const g_wszWMTitle: &'static str = "Title";
pub const g_wszWMTitleSort: &'static str = "TitleSort";
pub const g_wszWMToolName: &'static str = "WM/ToolName";
pub const g_wszWMToolVersion: &'static str = "WM/ToolVersion";
pub const g_wszWMTrack: &'static str = "WM/Track";
pub const g_wszWMTrackNumber: &'static str = "WM/TrackNumber";
pub const g_wszWMTrusted: &'static str = "Is_Trusted";
pub const g_wszWMUniqueFileIdentifier: &'static str = "WM/UniqueFileIdentifier";
pub const g_wszWMUse_Advanced_DRM: &'static str = "Use_Advanced_DRM";
pub const g_wszWMUse_DRM: &'static str = "Use_DRM";
pub const g_wszWMUserWebURL: &'static str = "WM/UserWebURL";
pub const g_wszWMVideoClosedCaptioning: &'static str = "WM/VideoClosedCaptioning";
pub const g_wszWMVideoFrameRate: &'static str = "WM/VideoFrameRate";
pub const g_wszWMVideoHeight: &'static str = "WM/VideoHeight";
pub const g_wszWMVideoWidth: &'static str = "WM/VideoWidth";
pub const g_wszWMWMADRCAverageReference: &'static str = "WM/WMADRCAverageReference";
pub const g_wszWMWMADRCAverageTarget: &'static str = "WM/WMADRCAverageTarget";
pub const g_wszWMWMADRCPeakReference: &'static str = "WM/WMADRCPeakReference";
pub const g_wszWMWMADRCPeakTarget: &'static str = "WM/WMADRCPeakTarget";
pub const g_wszWMWMCPDistributor: &'static str = "WM/WMCPDistributor";
pub const g_wszWMWMCPDistributorID: &'static str = "WM/WMCPDistributorID";
pub const g_wszWMWMCollectionGroupID: &'static str = "WM/WMCollectionGroupID";
pub const g_wszWMWMCollectionID: &'static str = "WM/WMCollectionID";
pub const g_wszWMWMContentID: &'static str = "WM/WMContentID";
pub const g_wszWMWMShadowFileSourceDRMType: &'static str = "WM/WMShadowFileSourceDRMType";
pub const g_wszWMWMShadowFileSourceFileType: &'static str = "WM/WMShadowFileSourceFileType";
pub const g_wszWMWriter: &'static str = "WM/Writer";
pub const g_wszWMYear: &'static str = "WM/Year";
pub const g_wszWatermarkCLSID: &'static str = "WatermarkCLSID";
pub const g_wszWatermarkConfig: &'static str = "WatermarkConfig";
