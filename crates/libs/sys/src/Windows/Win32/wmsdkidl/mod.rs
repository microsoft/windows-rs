windows_link::link!("wmvcore.dll" "system" fn WMCreateBackupRestorer(pcallback : *mut core::ffi::c_void, ppbackup : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateEditor(ppeditor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateIndexer(ppindexer : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateProfileManager(ppprofilemanager : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("wmvcore.dll" "system" fn WMCreateReader(punkcert : *mut core::ffi::c_void, dwrights : super::DWORD, ppreader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("wmvcore.dll" "system" fn WMCreateSyncReader(punkcert : *mut core::ffi::c_void, dwrights : super::DWORD, ppsyncreader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateWriter(punkcert : *mut core::ffi::c_void, ppwriter : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateWriterFileSink(ppsink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateWriterNetworkSink(ppsink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wmvcore.dll" "system" fn WMCreateWriterPushSink(ppsink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("wmvcore.dll" "system" fn WMIsContentProtected(pwszfilename : *const super::WCHAR, pfisprotected : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
pub const CLSID_WMBandwidthSharing_Exclusive: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaf6060aa_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMBandwidthSharing_Partial: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaf6060ab_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMMUTEX_Bitrate: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd6e22a01_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Language: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd6e22a00_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Presentation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd6e22a02_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Unknown: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd6e22a03_35da_11d1_9034_00a0c90349be);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: super::WORD,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: super::WORD,
    pub wUncompressedDigitalVideo: super::WORD,
    pub wAnalogVideo: super::WORD,
    pub wCompressedDigitalAudio: super::WORD,
    pub wUncompressedDigitalAudio: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: super::WORD,
    pub rgIds: *mut windows_sys::core::GUID,
}
pub const DRM_OPL_TYPES: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: windows_sys::core::GUID,
    pub bConfigData: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DRM_VAL16 {
    pub val: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type DRM_VIDEO_OUTPUT_PROTECTION = DRM_OUTPUT_PROTECTION;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: super::WORD,
    pub rgVop: *mut DRM_VIDEO_OUTPUT_PROTECTION,
}
pub const IID_IWMAddressAccess: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbb3c6389_1633_4e92_af14_9f3173ba39d0);
pub const IID_IWMAddressAccess2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x65a83fc2_3e98_4d4d_81b5_2a742886b33d);
pub const IID_IWMBackupRestoreProps: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3c8e0da6_996f_4ff3_a1af_4838f9377e2e);
pub const IID_IWMBandwidthSharing: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xad694af1_f8d9_42f8_bc47_70311b0c4f9e);
pub const IID_IWMClientConnections: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x73c66010_a299_41df_b1f0_ccf03b09c1c6);
pub const IID_IWMClientConnections2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4091571e_4701_4593_bb3d_d5f5f0c74246);
pub const IID_IWMCodecInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa970f41e_34de_4a98_b3ba_e4b3ca7528f0);
pub const IID_IWMCodecInfo2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa65e273_b686_4056_91ec_dd768d4df710);
pub const IID_IWMCodecInfo3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7e51f487_4d93_4f98_8ab4_27d0565adc51);
pub const IID_IWMCredentialCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x342e0eb7_e651_450c_975b_2ace2c90c48e);
pub const IID_IWMDRMEditor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xff130ebc_a6c3_42a6_b401_c3382c3e08b3);
pub const IID_IWMDRMMessageParser: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa73a0072_25a0_4c99_b4a5_ede8101a6c39);
pub const IID_IWMDRMReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd2827540_3ee7_432c_b14c_dc17f085d3b3);
pub const IID_IWMDRMReader2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbefe7a75_9f1d_4075_b9d9_a3c37bda49a0);
pub const IID_IWMDRMReader3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe08672de_f1e7_4ff4_a0a3_fc4b08e4caf8);
pub const IID_IWMDRMTranscryptionManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb1a887b2_a4f0_407a_b02e_efbd23bbecdf);
pub const IID_IWMDRMTranscryptor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69059850_6e6f_4bb2_806f_71863ddfc471);
pub const IID_IWMDRMTranscryptor2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe0da439f_d331_496a_bece_18e5bac5dd23);
pub const IID_IWMDRMWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd6ea5dd0_12a0_43f4_90ab_a3fd451e6a07);
pub const IID_IWMDRMWriter2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x38ee7a94_40e2_4e10_aa3f_33fd3210ed5b);
pub const IID_IWMDRMWriter3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa7184082_a4aa_4dde_ac9c_e75dbd1117ce);
pub const IID_IWMDeviceRegistration: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf6211f03_8d21_4e94_93e6_8510805f2d99);
pub const IID_IWMHeaderInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bda_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMHeaderInfo2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x15cf9781_454e_482e_b393_85fae487a810);
pub const IID_IWMHeaderInfo3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x15cc68e3_27cc_4ecd_b222_3f5d02d80bd5);
pub const IID_IWMIStreamProps: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6816dad3_2b4b_4c8e_8149_874c3483a753);
pub const IID_IWMImageInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9f0aa3b6_7267_4d89_88f2_ba915aa5c4c6);
pub const IID_IWMIndexer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d7cdc71_9888_11d3_8edc_00c04f6109cf);
pub const IID_IWMIndexer2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb70f1e42_6255_4df0_a6b9_02b212d9e2bb);
pub const IID_IWMInputMediaProps: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bd5_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMLanguageList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdf683f00_2d49_4d8e_92b7_fb19f6a0dc57);
pub const IID_IWMLicenseBackup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05e5ac9f_3fb6_4508_bb43_a4067ba1ebe8);
pub const IID_IWMLicenseRestore: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc70b6334_a22e_4efb_a245_15e65a004a13);
pub const IID_IWMLicenseRevocationAgent: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6967f2c9_4e26_4b57_8894_799880f7ac7b);
pub const IID_IWMMediaProps: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bce_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMMetadataEditor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bd9_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMMetadataEditor2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x203cffe3_2e18_4fdf_b59d_6e71530534cf);
pub const IID_IWMMutualExclusion: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bde_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMMutualExclusion2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0302b57d_89d1_4ba2_85c9_166f2c53eb91);
pub const IID_IWMOutputMediaProps: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bd7_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMPacketSize: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcdfb97ab_188f_40b3_b643_5b7903975c59);
pub const IID_IWMPacketSize2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8bfc2b9e_b646_4233_a877_1c6a079669dc);
pub const IID_IWMPlayerHook: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe5b7ca9a_0f1c_4f66_9002_74ec50d8b304);
pub const IID_IWMProfile: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bdb_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMProfile2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x07e72d33_d94e_4be7_8843_60ae5ff7e5f5);
pub const IID_IWMProfile3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00ef96cc_a461_4546_8bcd_c9a28f0e06f5);
pub const IID_IWMProfileManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd16679f2_6ca0_472d_8d31_2f5d55aee155);
pub const IID_IWMProfileManager2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7a924e51_73c1_494d_8019_23d37ed9b89a);
pub const IID_IWMProfileManagerLanguage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba4dcc78_7ee0_4ab8_b27a_dbce8bc51454);
pub const IID_IWMPropertyVault: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x72995a79_5090_42a4_9c8c_d9d0b6d34be5);
pub const IID_IWMProximityDetection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a9fd8ee_b651_4bf0_b849_7d4ece79a2b1);
pub const IID_IWMReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bd6_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderAccelerator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbddc4d08_944d_4d52_a612_46c3fda07dd4);
pub const IID_IWMReaderAdvanced: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bea_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderAdvanced2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xae14a945_b90c_4d0d_9127_80d665f7d73e);
pub const IID_IWMReaderAdvanced3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5dc0674b_f04b_4a4e_9f2a_b1afde2c8100);
pub const IID_IWMReaderAdvanced4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x945a76a2_12ae_4d48_bd3c_cd1d90399b85);
pub const IID_IWMReaderAdvanced5: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x24c44db0_55d1_49ae_a5cc_f13815e36363);
pub const IID_IWMReaderAdvanced6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x18a2e7f8_428f_4acd_8a00_e64639bc93de);
pub const IID_IWMReaderAllocatorEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9f762fa7_a22e_428d_93c9_ac82f3aafe5a);
pub const IID_IWMReaderCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bd8_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderCallbackAdvanced: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406beb_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderNetworkConfig: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bec_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderNetworkConfig2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd979a853_042b_4050_8387_c939db22013f);
pub const IID_IWMReaderPlaylistBurn: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf28c0300_9baa_4477_a846_1744d9cbf533);
pub const IID_IWMReaderStreamClock: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bed_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderTimecode: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf369e2f0_e081_4fe6_8450_b810b2f410d1);
pub const IID_IWMReaderTypeNegotiation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfdbe5592_81a1_41ea_93bd_735cad1adc05);
pub const IID_IWMRegisterCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf4b1f99_4de2_4e49_a363_252740d99bc1);
pub const IID_IWMRegisteredDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4503bec_5508_4148_97ac_bfa75760a70d);
pub const IID_IWMStatusCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d7cdc70_9888_11d3_8edc_00c04f6109cf);
pub const IID_IWMStreamConfig: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bdc_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMStreamConfig2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7688d8cb_fc0d_43bd_9459_5a8dec200cfa);
pub const IID_IWMStreamConfig3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcb164104_3aa9_45a7_9ac9_4daee131d6e1);
pub const IID_IWMStreamList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bdd_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMStreamPrioritization: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8c1c6090_f9a8_4748_8ec3_dd1108ba1e77);
pub const IID_IWMSyncReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9397f121_7705_4dc9_b049_98b698188414);
pub const IID_IWMSyncReader2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfaed3d21_1b6b_4af7_8cb6_3e189bbc187b);
pub const IID_IWMVideoMediaProps: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bcf_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWatermarkInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f497062_f2e2_4624_8ea7_9dd40d81fc8d);
pub const IID_IWMWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406bd4_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWriterAdvanced: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406be3_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWriterAdvanced2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x962dc1ec_c046_4db8_9cc7_26ceae500817);
pub const IID_IWMWriterAdvanced3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2cd6492d_7c37_4e76_9d3b_59261183a22e);
pub const IID_IWMWriterFileSink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406be5_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWriterFileSink2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x14282ba7_4aef_4205_8ce5_c229035a05bc);
pub const IID_IWMWriterFileSink3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3fea4feb_2945_47a7_a1dd_c53a8fc4c45c);
pub const IID_IWMWriterNetworkSink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406be7_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWriterPostView: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81e20ce4_75ef_491a_8004_fc53c45bdc3e);
pub const IID_IWMWriterPostViewCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd9d6549d_a193_4f24_b308_03123d9b7f8d);
pub const IID_IWMWriterPreprocess: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfc54a285_38c4_45b5_aa23_85b9f7cb424b);
pub const IID_IWMWriterPushSink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc10e6a5_072c_467d_bf57_6330a9dde12a);
pub const IID_IWMWriterSink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96406be4_2b2b_11d3_b36b_00c04f6108ff);
pub type LPCWSTR_WMSDK_TYPE_SAFE = *const u16;
pub const WAVE_FORMAT_DRM: i32 = 9;
pub const WEBSTREAM_SAMPLE_TYPE_FILE: i32 = 1;
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: super::DWORD,
    pub cbEncryptedSessionKeyMessage: super::DWORD,
    pub pbEncryptedSessionKeyMessage: *mut super::BYTE,
    pub cbEncryptedKeyMessage: super::DWORD,
    pub pbEncryptedKeyMessage: *mut super::BYTE,
}
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: i32 = 1;
pub const WMFORMAT_MPEG2Video: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const WMFORMAT_Script: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5c8510f2_debe_4ca7_bba5_f07a104f8dff);
pub const WMFORMAT_VideoInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WaveFormatEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WebStream: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xda1e6b13_8359_4050_b398_388e965bf00c);
pub const WMMEDIASUBTYPE_ACELPnet: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000130_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_Base: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_DRM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_I420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_IYUV: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_M4S2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP43: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP4S: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const WMMEDIASUBTYPE_MSS1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MSS2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_P422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x32323450_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_PCM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_RGB1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436eb78_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB24: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436eb7d_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB32: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436eb7e_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436eb79_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB555: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436eb7c_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB565: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436eb7b_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436eb7a_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_UYVY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_VIDEOIMAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1d4a45f2_e5f6_4b44_8388_f0ae5c0e0c37);
pub const WMMEDIASUBTYPE_WMAudioV2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV7: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV9: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudio_Lossless: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0000000a_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0000000b_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVC1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVP2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WebStream: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x776257d4_c627_41cb_8f81_7ac7ff1c40cc);
pub const WMMEDIASUBTYPE_YUY2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YV12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVU9: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVYU: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Audio: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_FileTransfer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd9e47579_930e_4427_adfc_ad80f290e470);
pub const WMMEDIATYPE_Image: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x34a50fd8_8aa5_4386_81fe_a0efe0488e31);
pub const WMMEDIATYPE_Script: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x73636d64_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Text: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9bba1ea7_5ab2_4829_ba57_0940209bcf3e);
pub const WMMEDIATYPE_Video: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: super::DWORD,
    pub cbSequenceHeader: super::DWORD,
    pub dwProfile: super::DWORD,
    pub dwLevel: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwSequenceHeader: [super::DWORD; 1],
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
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
pub const WMSCRIPTTYPE_TwoStrings: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x82f38a70_c29f_11d1_97ad_00a0c95ea850);
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
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
#[derive(Clone, Copy, Default)]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: *mut core::ffi::c_void,
    pub cbOffset: super::DWORD,
    pub cbLength: super::DWORD,
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
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: super::BYTE,
    pub ucColorTransferChar: super::BYTE,
    pub ucColorMatrixCoef: super::BYTE,
}
pub const WMT_CONNECTING: WMT_STATUS = 8;
pub const WMT_CONTENT_ENABLER: WMT_STATUS = 51;
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = 4;
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = 2;
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = 16;
pub type WMT_CREDENTIAL_FLAGS = i32;
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = 8;
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = 1;
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x65221c5a_fa75_4b39_b50c_06c336b6a3ef);
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x187cc922_8efc_4404_9daf_63f4830df1bc);
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
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
#[derive(Clone, Copy, Default)]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: super::DWORD,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: super::DWORD,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
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
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
#[derive(Clone, Copy, Default)]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: super::DWORD,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: super::WORD,
    pub dwTimecode: super::DWORD,
    pub dwUserbits: super::DWORD,
    pub dwAmFlags: super::DWORD,
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
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: i32 = 491406834;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: i32 = 491406835;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: super::DWORD,
    pub cbStruct: super::ULONG,
    pub dwControlFlags: super::DWORD,
    pub dwInputFlagsCur: super::DWORD,
    pub lCurMotionXtoX: super::LONG,
    pub lCurMotionYtoX: super::LONG,
    pub lCurMotionXoffset: super::LONG,
    pub lCurMotionXtoY: super::LONG,
    pub lCurMotionYtoY: super::LONG,
    pub lCurMotionYoffset: super::LONG,
    pub lCurBlendCoef1: super::LONG,
    pub lCurBlendCoef2: super::LONG,
    pub dwInputFlagsPrev: super::DWORD,
    pub lPrevMotionXtoX: super::LONG,
    pub lPrevMotionYtoX: super::LONG,
    pub lPrevMotionXoffset: super::LONG,
    pub lPrevMotionXtoY: super::LONG,
    pub lPrevMotionYtoY: super::LONG,
    pub lPrevMotionYoffset: super::LONG,
    pub lPrevBlendCoef1: super::LONG,
    pub lPrevBlendCoef2: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: super::DWORD,
    pub dwStructSize: super::DWORD,
    pub dwControlFlags: super::DWORD,
    pub dwViewportWidth: super::DWORD,
    pub dwViewportHeight: super::DWORD,
    pub dwCurrImageWidth: super::DWORD,
    pub dwCurrImageHeight: super::DWORD,
    pub fCurrRegionX0: super::FLOAT,
    pub fCurrRegionY0: super::FLOAT,
    pub fCurrRegionWidth: super::FLOAT,
    pub fCurrRegionHeight: super::FLOAT,
    pub fCurrBlendCoef: super::FLOAT,
    pub dwPrevImageWidth: super::DWORD,
    pub dwPrevImageHeight: super::DWORD,
    pub fPrevRegionX0: super::FLOAT,
    pub fPrevRegionY0: super::FLOAT,
    pub fPrevRegionWidth: super::FLOAT,
    pub fPrevRegionHeight: super::FLOAT,
    pub fPrevBlendCoef: super::FLOAT,
    pub dwEffectType: super::DWORD,
    pub dwNumEffectParas: super::DWORD,
    pub fEffectPara0: super::FLOAT,
    pub fEffectPara1: super::FLOAT,
    pub fEffectPara2: super::FLOAT,
    pub fEffectPara3: super::FLOAT,
    pub fEffectPara4: super::FLOAT,
    pub bKeepPrevImage: windows_sys::core::BOOL,
}
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: i32 = 8;
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: i32 = 4;
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: i32 = 1;
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: i32 = 1;
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: i32 = 2;
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: i32 = 2;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: i32 = 4;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: i32 = 8;
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: i32 = 11;
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: i32 = 12;
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: i32 = 13;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: i32 = 14;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: i32 = 15;
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: i32 = 16;
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: i32 = 17;
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: i32 = 18;
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: i32 = 19;
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: i32 = 20;
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: i32 = 21;
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: i32 = 23;
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: i32 = 24;
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: i32 = 27;
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: i32 = 29;
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: i32 = 30;
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: i32 = 31;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: super::CLSID,
    pub cbDisplayName: super::UINT,
    pub pwszDisplayName: windows_sys::core::PWSTR,
}
pub type WMT_WATERMARK_ENTRY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: super::WORD,
    pub cbSampleHeaderFixedData: super::WORD,
    pub wVersion: super::WORD,
    pub wReserved: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: super::WORD,
    pub wPart: super::WORD,
    pub cTotalParts: super::WORD,
    pub wSampleType: super::WORD,
    pub wszURL: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = 1;
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: super::DWORD,
    pub dwBitErrorRate: super::DWORD,
    pub AvgTimePerFrame: super::LONGLONG,
    pub bmiHeader: super::BITMAPINFOHEADER,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: super::DWORD,
    pub dwBitErrorRate: super::DWORD,
    pub AvgTimePerFrame: super::LONGLONG,
    pub dwInterlaceFlags: super::DWORD,
    pub dwCopyProtectFlags: super::DWORD,
    pub dwPictAspectRatioX: super::DWORD,
    pub dwPictAspectRatioY: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub bmiHeader: super::BITMAPINFOHEADER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: super::DWORD,
    pub dwMask: super::DWORD,
}
pub type WM_AETYPE = i32;
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = 101;
pub const WM_AETYPE_INCLUDE: WM_AETYPE = 105;
#[cfg(feature = "shared")]
pub const WM_BACKUP_OVERWRITE: super::DWORD = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: super::DWORD,
    pub dwPort: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: super::DWORD,
    pub pwszIPAddress: windows_sys::core::PCWSTR,
    pub pwszPort: windows_sys::core::PCWSTR,
    pub pwszDNSName: windows_sys::core::PCWSTR,
}
pub const WM_CL_INTERLACED420: i32 = 0;
pub const WM_CL_PROGRESSIVE420: i32 = 1;
pub const WM_CT_BOTTOM_FIELD_FIRST: i32 = 32;
pub const WM_CT_INTERLACED: i32 = 128;
pub const WM_CT_REPEAT_FIRST_FIELD: i32 = 16;
pub const WM_CT_TOP_FIELD_FIRST: i32 = 64;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: super::DWORD,
    pub msBufferWindow: super::DWORD,
}
pub const WM_MAX_STREAMS: i32 = 63;
pub const WM_MAX_VIDEO_STREAMS: i32 = 63;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_MEDIA_TYPE {
    pub majortype: windows_sys::core::GUID,
    pub subtype: windows_sys::core::GUID,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub lSampleSize: super::ULONG,
    pub formattype: windows_sys::core::GUID,
    pub pUnk: *mut core::ffi::c_void,
    pub cbFormat: super::ULONG,
    pub pbFormat: *mut super::BYTE,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_PICTURE {
    pub pwszMIMEType: windows_sys::core::PWSTR,
    pub bPictureType: super::BYTE,
    pub pwszDescription: windows_sys::core::PWSTR,
    pub dwDataLen: super::DWORD,
    pub pbData: *mut super::BYTE,
}
pub const WM_PLAYBACK_DRC_HIGH: i32 = 0;
pub const WM_PLAYBACK_DRC_LOW: i32 = 2;
pub const WM_PLAYBACK_DRC_MEDIUM: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: super::WORD,
    pub wPortEnd: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: super::DWORD,
    pub wszLang: *mut super::WCHAR,
    pub wszBrowserUserAgent: *mut super::WCHAR,
    pub wszBrowserWebPage: *mut super::WCHAR,
    pub qwReserved: super::QWORD,
    pub pReserved: *mut super::LPARAM,
    pub wszHostExe: *mut super::WCHAR,
    pub qwHostVersion: super::QWORD,
    pub wszPlayerUserAgent: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_READER_STATISTICS {
    pub cbSize: super::DWORD,
    pub dwBandwidth: super::DWORD,
    pub cPacketsReceived: super::DWORD,
    pub cPacketsRecovered: super::DWORD,
    pub cPacketsLost: super::DWORD,
    pub wQuality: super::WORD,
}
#[cfg(feature = "shared")]
pub const WM_RESTORE_INDIVIDUALIZE: super::DWORD = 2;
pub const WM_SFEX_DATALOSS: i32 = 4;
pub const WM_SFEX_NOTASYNCPOINT: i32 = 2;
pub const WM_SF_CLEANPOINT: i32 = 1;
pub const WM_SF_DATALOSS: i32 = 4;
pub const WM_SF_DISCONTINUITY: i32 = 2;
#[cfg(feature = "shared")]
pub const WM_START_CURRENTPOSITION: super::QWORD = 18446744073709551615;
#[repr(C, packed(2))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: super::WORD,
    pub fMandatory: windows_sys::core::BOOL,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: windows_sys::core::GUID,
    pub cbFormat: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: super::BYTE,
    pub bContentType: super::BYTE,
    pub pwszContentDescriptor: windows_sys::core::PWSTR,
    pub dwLyricsLen: super::DWORD,
    pub pbLyrics: *mut super::BYTE,
}
pub const WM_SampleExtensionGUID_ChromaLocation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4c5acca0_9276_4b2c_9e4c_a0edefdd217e);
pub const WM_SampleExtensionGUID_ColorSpaceInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf79ada56_30eb_4f2b_9f7a_f24b139a1157);
pub const WM_SampleExtensionGUID_ContentType: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd590dc20_07bc_436c_9cf7_f3bbfbf1a4dc);
pub const WM_SampleExtensionGUID_FileName: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe165ec0e_19ed_45d7_b4a7_25cbd1e28e9b);
pub const WM_SampleExtensionGUID_OutputCleanPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf72a3c6f_6eb4_4ebc_b192_09ad9759e828);
pub const WM_SampleExtensionGUID_PixelAspectRatio: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b1ee554_f9ea_4bc8_821a_376b74e4c4b8);
pub const WM_SampleExtensionGUID_SampleDuration: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc6bd9450_867f_4907_83a3_c77921b733ad);
pub const WM_SampleExtensionGUID_SampleProtectionSalt: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
pub const WM_SampleExtensionGUID_Timecode: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x399595ec_8667_4e2d_8fdb_98814ce76c1e);
pub const WM_SampleExtensionGUID_UserDataInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x732bb4fa_78be_4549_99bd_02db1a55b7a8);
pub const WM_SampleExtension_ChromaLocation_Size: i32 = 1;
pub const WM_SampleExtension_ColorSpaceInfo_Size: i32 = 3;
pub const WM_SampleExtension_ContentType_Size: i32 = 1;
pub const WM_SampleExtension_PixelAspectRatio_Size: i32 = 2;
pub const WM_SampleExtension_SampleDuration_Size: i32 = 2;
pub const WM_SampleExtension_Timecode_Size: i32 = 14;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WM_USER_TEXT {
    pub pwszDescription: windows_sys::core::PWSTR,
    pub pwszText: windows_sys::core::PWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: windows_sys::core::PWSTR,
    pub pwszURL: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: super::QWORD,
    pub qwByteCount: super::QWORD,
    pub qwDroppedSampleCount: super::QWORD,
    pub qwDroppedByteCount: super::QWORD,
    pub dwCurrentBitrate: super::DWORD,
    pub dwAverageBitrate: super::DWORD,
    pub dwExpectedBitrate: super::DWORD,
    pub dwCurrentSampleRate: super::DWORD,
    pub dwAverageSampleRate: super::DWORD,
    pub dwExpectedSampleRate: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: super::DWORD,
    pub dwCurrentSampleDropRateInQueue: super::DWORD,
    pub dwCurrentSampleDropRateInCodec: super::DWORD,
    pub dwCurrentSampleDropRateInMultiplexer: super::DWORD,
    pub dwTotalSampleDropsInQueue: super::DWORD,
    pub dwTotalSampleDropsInCodec: super::DWORD,
    pub dwTotalSampleDropsInMultiplexer: super::DWORD,
}
#[cfg(feature = "shared")]
pub const g_dwWMContentAttributes: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const g_dwWMNSCAttributes: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const g_dwWMSpecialAttributes: super::DWORD = 20;
pub const g_wszASFLeakyBucketPairs: windows_sys::core::PCWSTR = windows_sys::core::w!("ASFLeakyBucketPairs");
pub const g_wszAllowInterlacedOutput: windows_sys::core::PCWSTR = windows_sys::core::w!("AllowInterlacedOutput");
pub const g_wszAverageLevel: windows_sys::core::PCWSTR = windows_sys::core::w!("AverageLevel");
pub const g_wszBufferAverage: windows_sys::core::PCWSTR = windows_sys::core::w!("Buffer Average");
pub const g_wszComplexity: windows_sys::core::PCWSTR = windows_sys::core::w!("_COMPLEXITYEX");
pub const g_wszComplexityLive: windows_sys::core::PCWSTR = windows_sys::core::w!("_COMPLEXITYEXLIVE");
pub const g_wszComplexityMax: windows_sys::core::PCWSTR = windows_sys::core::w!("_COMPLEXITYEXMAX");
pub const g_wszComplexityOffline: windows_sys::core::PCWSTR = windows_sys::core::w!("_COMPLEXITYEXOFFLINE");
pub const g_wszDecoderComplexityRequested: windows_sys::core::PCWSTR = windows_sys::core::w!("_DECODERCOMPLEXITYPROFILE");
pub const g_wszDedicatedDeliveryThread: windows_sys::core::PCWSTR = windows_sys::core::w!("DedicatedDeliveryThread");
pub const g_wszDeinterlaceMode: windows_sys::core::PCWSTR = windows_sys::core::w!("DeinterlaceMode");
pub const g_wszDeliverOnReceive: windows_sys::core::PCWSTR = windows_sys::core::w!("DeliverOnReceive");
pub const g_wszDeviceConformanceTemplate: windows_sys::core::PCWSTR = windows_sys::core::w!("DeviceConformanceTemplate");
pub const g_wszDynamicRangeControl: windows_sys::core::PCWSTR = windows_sys::core::w!("DynamicRangeControl");
pub const g_wszEDL: windows_sys::core::PCWSTR = windows_sys::core::w!("_EDL");
pub const g_wszEarlyDataDelivery: windows_sys::core::PCWSTR = windows_sys::core::w!("EarlyDataDelivery");
pub const g_wszEnableDiscreteOutput: windows_sys::core::PCWSTR = windows_sys::core::w!("EnableDiscreteOutput");
pub const g_wszEnableFrameInterpolation: windows_sys::core::PCWSTR = windows_sys::core::w!("EnableFrameInterpolation");
pub const g_wszEnableWMAProSPDIFOutput: windows_sys::core::PCWSTR = windows_sys::core::w!("EnableWMAProSPDIFOutput");
pub const g_wszFailSeekOnError: windows_sys::core::PCWSTR = windows_sys::core::w!("FailSeekOnError");
pub const g_wszFixedFrameRate: windows_sys::core::PCWSTR = windows_sys::core::w!("FixedFrameRate");
pub const g_wszFold6To2Channels3: windows_sys::core::PCWSTR = windows_sys::core::w!("Fold6To2Channels3");
pub const g_wszFoldToChannelsTemplate: windows_sys::core::PCWSTR = windows_sys::core::w!("Fold%luTo%luChannels%lu");
pub const g_wszInitialPatternForInverseTelecine: windows_sys::core::PCWSTR = windows_sys::core::w!("InitialPatternForInverseTelecine");
pub const g_wszInterlacedCoding: windows_sys::core::PCWSTR = windows_sys::core::w!("InterlacedCoding");
pub const g_wszIsVBRSupported: windows_sys::core::PCWSTR = windows_sys::core::w!("_ISVBRSUPPORTED");
pub const g_wszJPEGCompressionQuality: windows_sys::core::PCWSTR = windows_sys::core::w!("JPEGCompressionQuality");
pub const g_wszJustInTimeDecode: windows_sys::core::PCWSTR = windows_sys::core::w!("JustInTimeDecode");
pub const g_wszMixedClassMode: windows_sys::core::PCWSTR = windows_sys::core::w!("MixedClassMode");
pub const g_wszMusicClassMode: windows_sys::core::PCWSTR = windows_sys::core::w!("MusicClassMode");
pub const g_wszMusicSpeechClassMode: windows_sys::core::PCWSTR = windows_sys::core::w!("MusicSpeechClassMode");
pub const g_wszNeedsPreviousSample: windows_sys::core::PCWSTR = windows_sys::core::w!("NeedsPreviousSample");
pub const g_wszNumPasses: windows_sys::core::PCWSTR = windows_sys::core::w!("_PASSESUSED");
pub const g_wszOriginalSourceFormatTag: windows_sys::core::PCWSTR = windows_sys::core::w!("_SOURCEFORMATTAG");
pub const g_wszOriginalWaveFormat: windows_sys::core::PCWSTR = windows_sys::core::w!("_ORIGINALWAVEFORMAT");
pub const g_wszPeakValue: windows_sys::core::PCWSTR = windows_sys::core::w!("PeakValue");
pub const g_wszPermitSeeksBeyondEndOfStream: windows_sys::core::PCWSTR = windows_sys::core::w!("PermitSeeksBeyondEndOfStream");
pub const g_wszReloadIndexOnSeek: windows_sys::core::PCWSTR = windows_sys::core::w!("ReloadIndexOnSeek");
pub const g_wszScrambledAudio: windows_sys::core::PCWSTR = windows_sys::core::w!("ScrambledAudio");
pub const g_wszSingleOutputBuffer: windows_sys::core::PCWSTR = windows_sys::core::w!("SingleOutputBuffer");
pub const g_wszSoftwareScaling: windows_sys::core::PCWSTR = windows_sys::core::w!("SoftwareScaling");
pub const g_wszSourceBufferTime: windows_sys::core::PCWSTR = windows_sys::core::w!("SourceBufferTime");
pub const g_wszSourceMaxBytesAtOnce: windows_sys::core::PCWSTR = windows_sys::core::w!("SourceMaxBytesAtOnce");
pub const g_wszSpeakerConfig: windows_sys::core::PCWSTR = windows_sys::core::w!("SpeakerConfig");
pub const g_wszSpeechCaps: windows_sys::core::PCWSTR = windows_sys::core::w!("SpeechFormatCap");
pub const g_wszSpeechClassMode: windows_sys::core::PCWSTR = windows_sys::core::w!("SpeechClassMode");
pub const g_wszStreamLanguage: windows_sys::core::PCWSTR = windows_sys::core::w!("StreamLanguage");
pub const g_wszStreamNumIndexObjects: windows_sys::core::PCWSTR = windows_sys::core::w!("StreamNumIndexObjects");
pub const g_wszUsePacketAtSeekPoint: windows_sys::core::PCWSTR = windows_sys::core::w!("UsePacketAtSeekPoint");
pub const g_wszVBRBitrateMax: windows_sys::core::PCWSTR = windows_sys::core::w!("_RMAX");
pub const g_wszVBRBufferWindowMax: windows_sys::core::PCWSTR = windows_sys::core::w!("_BMAX");
pub const g_wszVBREnabled: windows_sys::core::PCWSTR = windows_sys::core::w!("_VBRENABLED");
pub const g_wszVBRPeak: windows_sys::core::PCWSTR = windows_sys::core::w!("VBR Peak");
pub const g_wszVBRQuality: windows_sys::core::PCWSTR = windows_sys::core::w!("_VBRQUALITY");
pub const g_wszVideoSampleDurations: windows_sys::core::PCWSTR = windows_sys::core::w!("VideoSampleDurations");
pub const g_wszWMADID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ADID");
pub const g_wszWMASFPacketCount: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ASFPacketCount");
pub const g_wszWMASFSecurityObjectsSize: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ASFSecurityObjectsSize");
pub const g_wszWMAlbumArtist: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/AlbumArtist");
pub const g_wszWMAlbumArtistSort: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/AlbumArtistSort");
pub const g_wszWMAlbumCoverURL: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/AlbumCoverURL");
pub const g_wszWMAlbumTitle: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/AlbumTitle");
pub const g_wszWMAlbumTitleSort: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/AlbumTitleSort");
pub const g_wszWMAspectRatioX: windows_sys::core::PCWSTR = windows_sys::core::w!("AspectRatioX");
pub const g_wszWMAspectRatioY: windows_sys::core::PCWSTR = windows_sys::core::w!("AspectRatioY");
pub const g_wszWMAudioFileURL: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/AudioFileURL");
pub const g_wszWMAudioSourceURL: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/AudioSourceURL");
pub const g_wszWMAuthor: windows_sys::core::PCWSTR = windows_sys::core::w!("Author");
pub const g_wszWMAuthorSort: windows_sys::core::PCWSTR = windows_sys::core::w!("AuthorSort");
pub const g_wszWMAuthorURL: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/AuthorURL");
pub const g_wszWMBannerImageData: windows_sys::core::PCWSTR = windows_sys::core::w!("BannerImageData");
pub const g_wszWMBannerImageType: windows_sys::core::PCWSTR = windows_sys::core::w!("BannerImageType");
pub const g_wszWMBannerImageURL: windows_sys::core::PCWSTR = windows_sys::core::w!("BannerImageURL");
pub const g_wszWMBeatsPerMinute: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/BeatsPerMinute");
pub const g_wszWMBitrate: windows_sys::core::PCWSTR = windows_sys::core::w!("Bitrate");
pub const g_wszWMBroadcast: windows_sys::core::PCWSTR = windows_sys::core::w!("Broadcast");
pub const g_wszWMCategory: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Category");
pub const g_wszWMCodec: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Codec");
pub const g_wszWMComposer: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Composer");
pub const g_wszWMComposerSort: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ComposerSort");
pub const g_wszWMConductor: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Conductor");
pub const g_wszWMContainerFormat: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ContainerFormat");
pub const g_wszWMContentDistributor: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ContentDistributor");
pub const g_wszWMContentGroupDescription: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ContentGroupDescription");
pub const g_wszWMCopyright: windows_sys::core::PCWSTR = windows_sys::core::w!("Copyright");
pub const g_wszWMCopyrightURL: windows_sys::core::PCWSTR = windows_sys::core::w!("CopyrightURL");
pub const g_wszWMCurrentBitrate: windows_sys::core::PCWSTR = windows_sys::core::w!("CurrentBitrate");
pub const g_wszWMDRM: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/DRM");
pub const g_wszWMDRM_ContentID: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_ContentID");
pub const g_wszWMDRM_Flags: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_Flags");
pub const g_wszWMDRM_HeaderSignPrivKey: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_HeaderSignPrivKey");
pub const g_wszWMDRM_IndividualizedVersion: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_IndividualizedVersion");
pub const g_wszWMDRM_KeyID: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_KeyID");
pub const g_wszWMDRM_KeySeed: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_KeySeed");
pub const g_wszWMDRM_LASignatureCert: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_LASignatureCert");
pub const g_wszWMDRM_LASignatureLicSrvCert: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_LASignatureLicSrvCert");
pub const g_wszWMDRM_LASignaturePrivKey: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_LASignaturePrivKey");
pub const g_wszWMDRM_LASignatureRootCert: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_LASignatureRootCert");
pub const g_wszWMDRM_Level: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_Level");
pub const g_wszWMDRM_LicenseAcqURL: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_LicenseAcqURL");
pub const g_wszWMDRM_SourceID: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_SourceID");
pub const g_wszWMDRM_V1LicenseAcqURL: windows_sys::core::PCWSTR = windows_sys::core::w!("DRM_V1LicenseAcqURL");
pub const g_wszWMDVDID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/DVDID");
pub const g_wszWMDescription: windows_sys::core::PCWSTR = windows_sys::core::w!("Description");
pub const g_wszWMDirector: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Director");
pub const g_wszWMDuration: windows_sys::core::PCWSTR = windows_sys::core::w!("Duration");
pub const g_wszWMEncodedBy: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/EncodedBy");
pub const g_wszWMEncodingSettings: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/EncodingSettings");
pub const g_wszWMEncodingTime: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/EncodingTime");
pub const g_wszWMEpisodeNumber: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/EpisodeNumber");
pub const g_wszWMFileSize: windows_sys::core::PCWSTR = windows_sys::core::w!("FileSize");
pub const g_wszWMGenre: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Genre");
pub const g_wszWMGenreID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/GenreID");
pub const g_wszWMHasArbitraryDataStream: windows_sys::core::PCWSTR = windows_sys::core::w!("HasArbitraryDataStream");
pub const g_wszWMHasAttachedImages: windows_sys::core::PCWSTR = windows_sys::core::w!("HasAttachedImages");
pub const g_wszWMHasAudio: windows_sys::core::PCWSTR = windows_sys::core::w!("HasAudio");
pub const g_wszWMHasFileTransferStream: windows_sys::core::PCWSTR = windows_sys::core::w!("HasFileTransferStream");
pub const g_wszWMHasImage: windows_sys::core::PCWSTR = windows_sys::core::w!("HasImage");
pub const g_wszWMHasScript: windows_sys::core::PCWSTR = windows_sys::core::w!("HasScript");
pub const g_wszWMHasVideo: windows_sys::core::PCWSTR = windows_sys::core::w!("HasVideo");
pub const g_wszWMISAN: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ISAN");
pub const g_wszWMISRC: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ISRC");
pub const g_wszWMInitialKey: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/InitialKey");
pub const g_wszWMIsCompilation: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/IsCompilation");
pub const g_wszWMIsVBR: windows_sys::core::PCWSTR = windows_sys::core::w!("IsVBR");
pub const g_wszWMLanguage: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Language");
pub const g_wszWMLyrics: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Lyrics");
pub const g_wszWMLyrics_Synchronised: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Lyrics_Synchronised");
pub const g_wszWMMCDI: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MCDI");
pub const g_wszWMMediaClassPrimaryID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaClassPrimaryID");
pub const g_wszWMMediaClassSecondaryID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaClassSecondaryID");
pub const g_wszWMMediaCredits: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaCredits");
pub const g_wszWMMediaIsDelay: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsDelay");
pub const g_wszWMMediaIsFinale: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsFinale");
pub const g_wszWMMediaIsLive: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsLive");
pub const g_wszWMMediaIsPremiere: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsPremiere");
pub const g_wszWMMediaIsRepeat: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsRepeat");
pub const g_wszWMMediaIsSAP: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsSAP");
pub const g_wszWMMediaIsStereo: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsStereo");
pub const g_wszWMMediaIsSubtitled: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsSubtitled");
pub const g_wszWMMediaIsTape: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaIsTape");
pub const g_wszWMMediaNetworkAffiliation: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaNetworkAffiliation");
pub const g_wszWMMediaOriginalBroadcastDateTime: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaOriginalBroadcastDateTime");
pub const g_wszWMMediaOriginalChannel: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaOriginalChannel");
pub const g_wszWMMediaStationCallSign: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaStationCallSign");
pub const g_wszWMMediaStationName: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/MediaStationName");
pub const g_wszWMModifiedBy: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ModifiedBy");
pub const g_wszWMMood: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Mood");
pub const g_wszWMNSCAddress: windows_sys::core::PCWSTR = windows_sys::core::w!("NSC_Address");
pub const g_wszWMNSCDescription: windows_sys::core::PCWSTR = windows_sys::core::w!("NSC_Description");
pub const g_wszWMNSCEmail: windows_sys::core::PCWSTR = windows_sys::core::w!("NSC_Email");
pub const g_wszWMNSCName: windows_sys::core::PCWSTR = windows_sys::core::w!("NSC_Name");
pub const g_wszWMNSCPhone: windows_sys::core::PCWSTR = windows_sys::core::w!("NSC_Phone");
pub const g_wszWMNumberOfFrames: windows_sys::core::PCWSTR = windows_sys::core::w!("NumberOfFrames");
pub const g_wszWMOptimalBitrate: windows_sys::core::PCWSTR = windows_sys::core::w!("OptimalBitrate");
pub const g_wszWMOriginalAlbumTitle: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/OriginalAlbumTitle");
pub const g_wszWMOriginalArtist: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/OriginalArtist");
pub const g_wszWMOriginalFilename: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/OriginalFilename");
pub const g_wszWMOriginalLyricist: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/OriginalLyricist");
pub const g_wszWMOriginalReleaseTime: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/OriginalReleaseTime");
pub const g_wszWMOriginalReleaseYear: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/OriginalReleaseYear");
pub const g_wszWMParentalRating: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ParentalRating");
pub const g_wszWMParentalRatingReason: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ParentalRatingReason");
pub const g_wszWMPartOfSet: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/PartOfSet");
pub const g_wszWMPeakBitrate: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/PeakBitrate");
pub const g_wszWMPeriod: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Period");
pub const g_wszWMPicture: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Picture");
pub const g_wszWMPlaylistDelay: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/PlaylistDelay");
pub const g_wszWMProducer: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Producer");
pub const g_wszWMPromotionURL: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/PromotionURL");
pub const g_wszWMProtected: windows_sys::core::PCWSTR = windows_sys::core::w!("Is_Protected");
pub const g_wszWMProtectionType: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ProtectionType");
pub const g_wszWMProvider: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Provider");
pub const g_wszWMProviderCopyright: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ProviderCopyright");
pub const g_wszWMProviderRating: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ProviderRating");
pub const g_wszWMProviderStyle: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ProviderStyle");
pub const g_wszWMPublisher: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Publisher");
pub const g_wszWMRadioStationName: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/RadioStationName");
pub const g_wszWMRadioStationOwner: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/RadioStationOwner");
pub const g_wszWMRating: windows_sys::core::PCWSTR = windows_sys::core::w!("Rating");
pub const g_wszWMSeasonNumber: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/SeasonNumber");
pub const g_wszWMSeekable: windows_sys::core::PCWSTR = windows_sys::core::w!("Seekable");
pub const g_wszWMSharedUserRating: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/SharedUserRating");
pub const g_wszWMSignature_Name: windows_sys::core::PCWSTR = windows_sys::core::w!("Signature_Name");
pub const g_wszWMSkipBackward: windows_sys::core::PCWSTR = windows_sys::core::w!("Can_Skip_Backward");
pub const g_wszWMSkipForward: windows_sys::core::PCWSTR = windows_sys::core::w!("Can_Skip_Forward");
pub const g_wszWMStreamTypeInfo: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/StreamTypeInfo");
pub const g_wszWMStridable: windows_sys::core::PCWSTR = windows_sys::core::w!("Stridable");
pub const g_wszWMSubTitle: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/SubTitle");
pub const g_wszWMSubTitleDescription: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/SubTitleDescription");
pub const g_wszWMSubscriptionContentID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/SubscriptionContentID");
pub const g_wszWMText: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Text");
pub const g_wszWMTitle: windows_sys::core::PCWSTR = windows_sys::core::w!("Title");
pub const g_wszWMTitleSort: windows_sys::core::PCWSTR = windows_sys::core::w!("TitleSort");
pub const g_wszWMToolName: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ToolName");
pub const g_wszWMToolVersion: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/ToolVersion");
pub const g_wszWMTrack: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Track");
pub const g_wszWMTrackNumber: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/TrackNumber");
pub const g_wszWMTrusted: windows_sys::core::PCWSTR = windows_sys::core::w!("Is_Trusted");
pub const g_wszWMUniqueFileIdentifier: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/UniqueFileIdentifier");
pub const g_wszWMUse_Advanced_DRM: windows_sys::core::PCWSTR = windows_sys::core::w!("Use_Advanced_DRM");
pub const g_wszWMUse_DRM: windows_sys::core::PCWSTR = windows_sys::core::w!("Use_DRM");
pub const g_wszWMUserWebURL: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/UserWebURL");
pub const g_wszWMVideoClosedCaptioning: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/VideoClosedCaptioning");
pub const g_wszWMVideoFrameRate: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/VideoFrameRate");
pub const g_wszWMVideoHeight: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/VideoHeight");
pub const g_wszWMVideoWidth: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/VideoWidth");
pub const g_wszWMWMADRCAverageReference: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMADRCAverageReference");
pub const g_wszWMWMADRCAverageTarget: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMADRCAverageTarget");
pub const g_wszWMWMADRCPeakReference: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMADRCPeakReference");
pub const g_wszWMWMADRCPeakTarget: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMADRCPeakTarget");
pub const g_wszWMWMCPDistributor: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMCPDistributor");
pub const g_wszWMWMCPDistributorID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMCPDistributorID");
pub const g_wszWMWMCollectionGroupID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMCollectionGroupID");
pub const g_wszWMWMCollectionID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMCollectionID");
pub const g_wszWMWMContentID: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMContentID");
pub const g_wszWMWMShadowFileSourceDRMType: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMShadowFileSourceDRMType");
pub const g_wszWMWMShadowFileSourceFileType: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/WMShadowFileSourceFileType");
pub const g_wszWMWriter: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Writer");
pub const g_wszWMYear: windows_sys::core::PCWSTR = windows_sys::core::w!("WM/Year");
pub const g_wszWatermarkCLSID: windows_sys::core::PCWSTR = windows_sys::core::w!("WatermarkCLSID");
pub const g_wszWatermarkConfig: windows_sys::core::PCWSTR = windows_sys::core::w!("WatermarkConfig");
