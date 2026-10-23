#[inline]
pub unsafe fn WMCreateBackupRestorer(pcallback: &Option<windows_core::IUnknown>, ppbackup: *mut Option<IWMLicenseBackup>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateBackupRestorer(pcallback : *mut core::ffi::c_void, ppbackup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateBackupRestorer(core::mem::transmute_copy(pcallback), ppbackup as _) }
}
#[inline]
pub unsafe fn WMCreateEditor(ppeditor: *mut Option<IWMMetadataEditor>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateEditor(ppeditor : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateEditor(ppeditor as _) }
}
#[inline]
pub unsafe fn WMCreateIndexer(ppindexer: *mut Option<IWMIndexer>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateIndexer(ppindexer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateIndexer(ppindexer as _) }
}
#[inline]
pub unsafe fn WMCreateProfileManager(ppprofilemanager: *mut Option<IWMProfileManager>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateProfileManager(ppprofilemanager : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateProfileManager(ppprofilemanager as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WMCreateReader(punkcert: &Option<windows_core::IUnknown>, dwrights: super::DWORD, ppreader: *mut Option<IWMReader>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateReader(punkcert : *mut core::ffi::c_void, dwrights : super::DWORD, ppreader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateReader(core::mem::transmute_copy(punkcert), dwrights, ppreader as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WMCreateSyncReader(punkcert: &Option<windows_core::IUnknown>, dwrights: super::DWORD, ppsyncreader: *mut Option<IWMSyncReader>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateSyncReader(punkcert : *mut core::ffi::c_void, dwrights : super::DWORD, ppsyncreader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateSyncReader(core::mem::transmute_copy(punkcert), dwrights, ppsyncreader as _) }
}
#[inline]
pub unsafe fn WMCreateWriter(punkcert: &Option<windows_core::IUnknown>, ppwriter: *mut Option<IWMWriter>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateWriter(punkcert : *mut core::ffi::c_void, ppwriter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateWriter(core::mem::transmute_copy(punkcert), ppwriter as _) }
}
#[inline]
pub unsafe fn WMCreateWriterFileSink(ppsink: *mut Option<IWMWriterFileSink>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateWriterFileSink(ppsink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateWriterFileSink(ppsink as _) }
}
#[inline]
pub unsafe fn WMCreateWriterNetworkSink(ppsink: *mut Option<IWMWriterNetworkSink>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateWriterNetworkSink(ppsink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateWriterNetworkSink(ppsink as _) }
}
#[inline]
pub unsafe fn WMCreateWriterPushSink(ppsink: *mut Option<IWMWriterPushSink>) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMCreateWriterPushSink(ppsink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WMCreateWriterPushSink(ppsink as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WMIsContentProtected(pwszfilename: *const super::WCHAR, pfisprotected: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("wmvcore.dll" "system" fn WMIsContentProtected(pwszfilename : *const super::WCHAR, pfisprotected : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { WMIsContentProtected(pwszfilename as _, pfisprotected as _) }
}
pub const CLSID_WMBandwidthSharing_Exclusive: windows_core::GUID = windows_core::GUID::from_u128(0xaf6060aa_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMBandwidthSharing_Partial: windows_core::GUID = windows_core::GUID::from_u128(0xaf6060ab_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMMUTEX_Bitrate: windows_core::GUID = windows_core::GUID::from_u128(0xd6e22a01_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Language: windows_core::GUID = windows_core::GUID::from_u128(0xd6e22a00_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Presentation: windows_core::GUID = windows_core::GUID::from_u128(0xd6e22a02_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Unknown: windows_core::GUID = windows_core::GUID::from_u128(0xd6e22a03_35da_11d1_9034_00a0c90349be);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: super::WORD,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: super::WORD,
    pub wUncompressedDigitalVideo: super::WORD,
    pub wAnalogVideo: super::WORD,
    pub wCompressedDigitalAudio: super::WORD,
    pub wUncompressedDigitalAudio: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: super::WORD,
    pub rgIds: *mut windows_core::GUID,
}
pub const DRM_OPL_TYPES: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: windows_core::GUID,
    pub bConfigData: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: super::WORD,
    pub rgVop: *mut DRM_VIDEO_OUTPUT_PROTECTION,
}
pub const IID_IWMAddressAccess: windows_core::GUID = windows_core::GUID::from_u128(0xbb3c6389_1633_4e92_af14_9f3173ba39d0);
pub const IID_IWMAddressAccess2: windows_core::GUID = windows_core::GUID::from_u128(0x65a83fc2_3e98_4d4d_81b5_2a742886b33d);
pub const IID_IWMBackupRestoreProps: windows_core::GUID = windows_core::GUID::from_u128(0x3c8e0da6_996f_4ff3_a1af_4838f9377e2e);
pub const IID_IWMBandwidthSharing: windows_core::GUID = windows_core::GUID::from_u128(0xad694af1_f8d9_42f8_bc47_70311b0c4f9e);
pub const IID_IWMClientConnections: windows_core::GUID = windows_core::GUID::from_u128(0x73c66010_a299_41df_b1f0_ccf03b09c1c6);
pub const IID_IWMClientConnections2: windows_core::GUID = windows_core::GUID::from_u128(0x4091571e_4701_4593_bb3d_d5f5f0c74246);
pub const IID_IWMCodecInfo: windows_core::GUID = windows_core::GUID::from_u128(0xa970f41e_34de_4a98_b3ba_e4b3ca7528f0);
pub const IID_IWMCodecInfo2: windows_core::GUID = windows_core::GUID::from_u128(0xaa65e273_b686_4056_91ec_dd768d4df710);
pub const IID_IWMCodecInfo3: windows_core::GUID = windows_core::GUID::from_u128(0x7e51f487_4d93_4f98_8ab4_27d0565adc51);
pub const IID_IWMCredentialCallback: windows_core::GUID = windows_core::GUID::from_u128(0x342e0eb7_e651_450c_975b_2ace2c90c48e);
pub const IID_IWMDRMEditor: windows_core::GUID = windows_core::GUID::from_u128(0xff130ebc_a6c3_42a6_b401_c3382c3e08b3);
pub const IID_IWMDRMMessageParser: windows_core::GUID = windows_core::GUID::from_u128(0xa73a0072_25a0_4c99_b4a5_ede8101a6c39);
pub const IID_IWMDRMReader: windows_core::GUID = windows_core::GUID::from_u128(0xd2827540_3ee7_432c_b14c_dc17f085d3b3);
pub const IID_IWMDRMReader2: windows_core::GUID = windows_core::GUID::from_u128(0xbefe7a75_9f1d_4075_b9d9_a3c37bda49a0);
pub const IID_IWMDRMReader3: windows_core::GUID = windows_core::GUID::from_u128(0xe08672de_f1e7_4ff4_a0a3_fc4b08e4caf8);
pub const IID_IWMDRMTranscryptionManager: windows_core::GUID = windows_core::GUID::from_u128(0xb1a887b2_a4f0_407a_b02e_efbd23bbecdf);
pub const IID_IWMDRMTranscryptor: windows_core::GUID = windows_core::GUID::from_u128(0x69059850_6e6f_4bb2_806f_71863ddfc471);
pub const IID_IWMDRMTranscryptor2: windows_core::GUID = windows_core::GUID::from_u128(0xe0da439f_d331_496a_bece_18e5bac5dd23);
pub const IID_IWMDRMWriter: windows_core::GUID = windows_core::GUID::from_u128(0xd6ea5dd0_12a0_43f4_90ab_a3fd451e6a07);
pub const IID_IWMDRMWriter2: windows_core::GUID = windows_core::GUID::from_u128(0x38ee7a94_40e2_4e10_aa3f_33fd3210ed5b);
pub const IID_IWMDRMWriter3: windows_core::GUID = windows_core::GUID::from_u128(0xa7184082_a4aa_4dde_ac9c_e75dbd1117ce);
pub const IID_IWMDeviceRegistration: windows_core::GUID = windows_core::GUID::from_u128(0xf6211f03_8d21_4e94_93e6_8510805f2d99);
pub const IID_IWMHeaderInfo: windows_core::GUID = windows_core::GUID::from_u128(0x96406bda_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMHeaderInfo2: windows_core::GUID = windows_core::GUID::from_u128(0x15cf9781_454e_482e_b393_85fae487a810);
pub const IID_IWMHeaderInfo3: windows_core::GUID = windows_core::GUID::from_u128(0x15cc68e3_27cc_4ecd_b222_3f5d02d80bd5);
pub const IID_IWMIStreamProps: windows_core::GUID = windows_core::GUID::from_u128(0x6816dad3_2b4b_4c8e_8149_874c3483a753);
pub const IID_IWMImageInfo: windows_core::GUID = windows_core::GUID::from_u128(0x9f0aa3b6_7267_4d89_88f2_ba915aa5c4c6);
pub const IID_IWMIndexer: windows_core::GUID = windows_core::GUID::from_u128(0x6d7cdc71_9888_11d3_8edc_00c04f6109cf);
pub const IID_IWMIndexer2: windows_core::GUID = windows_core::GUID::from_u128(0xb70f1e42_6255_4df0_a6b9_02b212d9e2bb);
pub const IID_IWMInputMediaProps: windows_core::GUID = windows_core::GUID::from_u128(0x96406bd5_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMLanguageList: windows_core::GUID = windows_core::GUID::from_u128(0xdf683f00_2d49_4d8e_92b7_fb19f6a0dc57);
pub const IID_IWMLicenseBackup: windows_core::GUID = windows_core::GUID::from_u128(0x05e5ac9f_3fb6_4508_bb43_a4067ba1ebe8);
pub const IID_IWMLicenseRestore: windows_core::GUID = windows_core::GUID::from_u128(0xc70b6334_a22e_4efb_a245_15e65a004a13);
pub const IID_IWMLicenseRevocationAgent: windows_core::GUID = windows_core::GUID::from_u128(0x6967f2c9_4e26_4b57_8894_799880f7ac7b);
pub const IID_IWMMediaProps: windows_core::GUID = windows_core::GUID::from_u128(0x96406bce_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMMetadataEditor: windows_core::GUID = windows_core::GUID::from_u128(0x96406bd9_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMMetadataEditor2: windows_core::GUID = windows_core::GUID::from_u128(0x203cffe3_2e18_4fdf_b59d_6e71530534cf);
pub const IID_IWMMutualExclusion: windows_core::GUID = windows_core::GUID::from_u128(0x96406bde_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMMutualExclusion2: windows_core::GUID = windows_core::GUID::from_u128(0x0302b57d_89d1_4ba2_85c9_166f2c53eb91);
pub const IID_IWMOutputMediaProps: windows_core::GUID = windows_core::GUID::from_u128(0x96406bd7_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMPacketSize: windows_core::GUID = windows_core::GUID::from_u128(0xcdfb97ab_188f_40b3_b643_5b7903975c59);
pub const IID_IWMPacketSize2: windows_core::GUID = windows_core::GUID::from_u128(0x8bfc2b9e_b646_4233_a877_1c6a079669dc);
pub const IID_IWMPlayerHook: windows_core::GUID = windows_core::GUID::from_u128(0xe5b7ca9a_0f1c_4f66_9002_74ec50d8b304);
pub const IID_IWMProfile: windows_core::GUID = windows_core::GUID::from_u128(0x96406bdb_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMProfile2: windows_core::GUID = windows_core::GUID::from_u128(0x07e72d33_d94e_4be7_8843_60ae5ff7e5f5);
pub const IID_IWMProfile3: windows_core::GUID = windows_core::GUID::from_u128(0x00ef96cc_a461_4546_8bcd_c9a28f0e06f5);
pub const IID_IWMProfileManager: windows_core::GUID = windows_core::GUID::from_u128(0xd16679f2_6ca0_472d_8d31_2f5d55aee155);
pub const IID_IWMProfileManager2: windows_core::GUID = windows_core::GUID::from_u128(0x7a924e51_73c1_494d_8019_23d37ed9b89a);
pub const IID_IWMProfileManagerLanguage: windows_core::GUID = windows_core::GUID::from_u128(0xba4dcc78_7ee0_4ab8_b27a_dbce8bc51454);
pub const IID_IWMPropertyVault: windows_core::GUID = windows_core::GUID::from_u128(0x72995a79_5090_42a4_9c8c_d9d0b6d34be5);
pub const IID_IWMProximityDetection: windows_core::GUID = windows_core::GUID::from_u128(0x6a9fd8ee_b651_4bf0_b849_7d4ece79a2b1);
pub const IID_IWMReader: windows_core::GUID = windows_core::GUID::from_u128(0x96406bd6_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderAccelerator: windows_core::GUID = windows_core::GUID::from_u128(0xbddc4d08_944d_4d52_a612_46c3fda07dd4);
pub const IID_IWMReaderAdvanced: windows_core::GUID = windows_core::GUID::from_u128(0x96406bea_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderAdvanced2: windows_core::GUID = windows_core::GUID::from_u128(0xae14a945_b90c_4d0d_9127_80d665f7d73e);
pub const IID_IWMReaderAdvanced3: windows_core::GUID = windows_core::GUID::from_u128(0x5dc0674b_f04b_4a4e_9f2a_b1afde2c8100);
pub const IID_IWMReaderAdvanced4: windows_core::GUID = windows_core::GUID::from_u128(0x945a76a2_12ae_4d48_bd3c_cd1d90399b85);
pub const IID_IWMReaderAdvanced5: windows_core::GUID = windows_core::GUID::from_u128(0x24c44db0_55d1_49ae_a5cc_f13815e36363);
pub const IID_IWMReaderAdvanced6: windows_core::GUID = windows_core::GUID::from_u128(0x18a2e7f8_428f_4acd_8a00_e64639bc93de);
pub const IID_IWMReaderAllocatorEx: windows_core::GUID = windows_core::GUID::from_u128(0x9f762fa7_a22e_428d_93c9_ac82f3aafe5a);
pub const IID_IWMReaderCallback: windows_core::GUID = windows_core::GUID::from_u128(0x96406bd8_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderCallbackAdvanced: windows_core::GUID = windows_core::GUID::from_u128(0x96406beb_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderNetworkConfig: windows_core::GUID = windows_core::GUID::from_u128(0x96406bec_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderNetworkConfig2: windows_core::GUID = windows_core::GUID::from_u128(0xd979a853_042b_4050_8387_c939db22013f);
pub const IID_IWMReaderPlaylistBurn: windows_core::GUID = windows_core::GUID::from_u128(0xf28c0300_9baa_4477_a846_1744d9cbf533);
pub const IID_IWMReaderStreamClock: windows_core::GUID = windows_core::GUID::from_u128(0x96406bed_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMReaderTimecode: windows_core::GUID = windows_core::GUID::from_u128(0xf369e2f0_e081_4fe6_8450_b810b2f410d1);
pub const IID_IWMReaderTypeNegotiation: windows_core::GUID = windows_core::GUID::from_u128(0xfdbe5592_81a1_41ea_93bd_735cad1adc05);
pub const IID_IWMRegisterCallback: windows_core::GUID = windows_core::GUID::from_u128(0xcf4b1f99_4de2_4e49_a363_252740d99bc1);
pub const IID_IWMRegisteredDevice: windows_core::GUID = windows_core::GUID::from_u128(0xa4503bec_5508_4148_97ac_bfa75760a70d);
pub const IID_IWMStatusCallback: windows_core::GUID = windows_core::GUID::from_u128(0x6d7cdc70_9888_11d3_8edc_00c04f6109cf);
pub const IID_IWMStreamConfig: windows_core::GUID = windows_core::GUID::from_u128(0x96406bdc_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMStreamConfig2: windows_core::GUID = windows_core::GUID::from_u128(0x7688d8cb_fc0d_43bd_9459_5a8dec200cfa);
pub const IID_IWMStreamConfig3: windows_core::GUID = windows_core::GUID::from_u128(0xcb164104_3aa9_45a7_9ac9_4daee131d6e1);
pub const IID_IWMStreamList: windows_core::GUID = windows_core::GUID::from_u128(0x96406bdd_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMStreamPrioritization: windows_core::GUID = windows_core::GUID::from_u128(0x8c1c6090_f9a8_4748_8ec3_dd1108ba1e77);
pub const IID_IWMSyncReader: windows_core::GUID = windows_core::GUID::from_u128(0x9397f121_7705_4dc9_b049_98b698188414);
pub const IID_IWMSyncReader2: windows_core::GUID = windows_core::GUID::from_u128(0xfaed3d21_1b6b_4af7_8cb6_3e189bbc187b);
pub const IID_IWMVideoMediaProps: windows_core::GUID = windows_core::GUID::from_u128(0x96406bcf_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWatermarkInfo: windows_core::GUID = windows_core::GUID::from_u128(0x6f497062_f2e2_4624_8ea7_9dd40d81fc8d);
pub const IID_IWMWriter: windows_core::GUID = windows_core::GUID::from_u128(0x96406bd4_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWriterAdvanced: windows_core::GUID = windows_core::GUID::from_u128(0x96406be3_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWriterAdvanced2: windows_core::GUID = windows_core::GUID::from_u128(0x962dc1ec_c046_4db8_9cc7_26ceae500817);
pub const IID_IWMWriterAdvanced3: windows_core::GUID = windows_core::GUID::from_u128(0x2cd6492d_7c37_4e76_9d3b_59261183a22e);
pub const IID_IWMWriterFileSink: windows_core::GUID = windows_core::GUID::from_u128(0x96406be5_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWriterFileSink2: windows_core::GUID = windows_core::GUID::from_u128(0x14282ba7_4aef_4205_8ce5_c229035a05bc);
pub const IID_IWMWriterFileSink3: windows_core::GUID = windows_core::GUID::from_u128(0x3fea4feb_2945_47a7_a1dd_c53a8fc4c45c);
pub const IID_IWMWriterNetworkSink: windows_core::GUID = windows_core::GUID::from_u128(0x96406be7_2b2b_11d3_b36b_00c04f6108ff);
pub const IID_IWMWriterPostView: windows_core::GUID = windows_core::GUID::from_u128(0x81e20ce4_75ef_491a_8004_fc53c45bdc3e);
pub const IID_IWMWriterPostViewCallback: windows_core::GUID = windows_core::GUID::from_u128(0xd9d6549d_a193_4f24_b308_03123d9b7f8d);
pub const IID_IWMWriterPreprocess: windows_core::GUID = windows_core::GUID::from_u128(0xfc54a285_38c4_45b5_aa23_85b9f7cb424b);
pub const IID_IWMWriterPushSink: windows_core::GUID = windows_core::GUID::from_u128(0xdc10e6a5_072c_467d_bf57_6330a9dde12a);
pub const IID_IWMWriterSink: windows_core::GUID = windows_core::GUID::from_u128(0x96406be4_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::define_interface!(IWMAddressAccess, IWMAddressAccess_Vtbl, 0xbb3c6389_1633_4e92_af14_9f3173ba39d0);
windows_core::imp::interface_hierarchy!(IWMAddressAccess, windows_core::IUnknown);
impl IWMAddressAccess {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccessEntryCount)(windows_core::Interface::as_raw(self), aetype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: super::DWORD) -> windows_core::Result<WM_ADDRESS_ACCESSENTRY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccessEntry)(windows_core::Interface::as_raw(self), aetype, dwentrynum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddAccessEntry)(windows_core::Interface::as_raw(self), aetype, paddraccessentry) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAccessEntry)(windows_core::Interface::as_raw(self), aetype, dwentrynum) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAccessEntryCount: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAccessEntryCount: usize,
    #[cfg(feature = "shared")]
    pub GetAccessEntry: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, super::DWORD, *mut WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAccessEntry: usize,
    #[cfg(feature = "shared")]
    pub AddAccessEntry: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, *const WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddAccessEntry: usize,
    #[cfg(feature = "shared")]
    pub RemoveAccessEntry: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveAccessEntry: usize,
}
#[cfg(feature = "shared")]
pub trait IWMAddressAccess_Impl: windows_core::IUnknownImpl {
    fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> windows_core::Result<super::DWORD>;
    fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: super::DWORD) -> windows_core::Result<WM_ADDRESS_ACCESSENTRY>;
    fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> windows_core::Result<()>;
    fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMAddressAccess_Vtbl {
    pub const fn new<Identity: IWMAddressAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAccessEntryCount<Identity: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMAddressAccess_Impl::GetAccessEntryCount(this, core::mem::transmute_copy(&aetype)) {
                    Ok(ok__) => {
                        pcentries.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAccessEntry<Identity: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: super::DWORD, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMAddressAccess_Impl::GetAccessEntry(this, core::mem::transmute_copy(&aetype), core::mem::transmute_copy(&dwentrynum)) {
                    Ok(ok__) => {
                        paddraccessentry.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddAccessEntry<Identity: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMAddressAccess_Impl::AddAccessEntry(this, core::mem::transmute_copy(&aetype), core::mem::transmute_copy(&paddraccessentry)).into()
            }
        }
        unsafe extern "system" fn RemoveAccessEntry<Identity: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMAddressAccess_Impl::RemoveAccessEntry(this, core::mem::transmute_copy(&aetype), core::mem::transmute_copy(&dwentrynum)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAccessEntryCount: GetAccessEntryCount::<Identity, OFFSET>,
            GetAccessEntry: GetAccessEntry::<Identity, OFFSET>,
            AddAccessEntry: AddAccessEntry::<Identity, OFFSET>,
            RemoveAccessEntry: RemoveAccessEntry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMAddressAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMAddressAccess {}
windows_core::imp::define_interface!(IWMAddressAccess2, IWMAddressAccess2_Vtbl, 0x65a83fc2_3e98_4d4d_81b5_2a742886b33d);
impl core::ops::Deref for IWMAddressAccess2 {
    type Target = IWMAddressAccess;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMAddressAccess2, windows_core::IUnknown, IWMAddressAccess);
impl IWMAddressAccess2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: super::DWORD, pbstraddress: *mut windows_core::BSTR, pbstrmask: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAccessEntryEx)(windows_core::Interface::as_raw(self), aetype, dwentrynum, core::mem::transmute(pbstraddress), core::mem::transmute(pbstrmask)) }
    }
    pub unsafe fn AddAccessEntryEx(&self, aetype: WM_AETYPE, bstraddress: &windows_core::BSTR, bstrmask: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddAccessEntryEx)(windows_core::Interface::as_raw(self), aetype, core::mem::transmute_copy(bstraddress), core::mem::transmute_copy(bstrmask)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess2_Vtbl {
    pub base__: IWMAddressAccess_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAccessEntryEx: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, super::DWORD, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAccessEntryEx: usize,
    pub AddAccessEntryEx: unsafe extern "system" fn(*mut core::ffi::c_void, WM_AETYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMAddressAccess2_Impl: IWMAddressAccess_Impl {
    fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: super::DWORD, pbstraddress: *mut windows_core::BSTR, pbstrmask: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddAccessEntryEx(&self, aetype: WM_AETYPE, bstraddress: &windows_core::BSTR, bstrmask: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMAddressAccess2_Vtbl {
    pub const fn new<Identity: IWMAddressAccess2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAccessEntryEx<Identity: IWMAddressAccess2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: super::DWORD, pbstraddress: *mut *mut core::ffi::c_void, pbstrmask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMAddressAccess2_Impl::GetAccessEntryEx(this, core::mem::transmute_copy(&aetype), core::mem::transmute_copy(&dwentrynum), core::mem::transmute_copy(&pbstraddress), core::mem::transmute_copy(&pbstrmask)).into()
            }
        }
        unsafe extern "system" fn AddAccessEntryEx<Identity: IWMAddressAccess2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, bstraddress: *mut core::ffi::c_void, bstrmask: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMAddressAccess2_Impl::AddAccessEntryEx(this, core::mem::transmute_copy(&aetype), core::mem::transmute(&bstraddress), core::mem::transmute(&bstrmask)).into()
            }
        }
        Self {
            base__: IWMAddressAccess_Vtbl::new::<Identity, OFFSET>(),
            GetAccessEntryEx: GetAccessEntryEx::<Identity, OFFSET>,
            AddAccessEntryEx: AddAccessEntryEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMAddressAccess2 as windows_core::Interface>::IID || iid == &<IWMAddressAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMAddressAccess2 {}
windows_core::imp::define_interface!(IWMBackupRestoreProps, IWMBackupRestoreProps_Vtbl, 0x3c8e0da6_996f_4ff3_a1af_4838f9377e2e);
windows_core::imp::interface_hierarchy!(IWMBackupRestoreProps, windows_core::IUnknown);
impl IWMBackupRestoreProps {
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropCount(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetPropByIndex(&self, windex: super::WORD, pwszname: *mut super::WCHAR, pcchnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropByIndex)(windows_core::Interface::as_raw(self), windex, pwszname as _, pcchnamelen as _, ptype as _, pvalue as _, pcblength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropByName<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPropByName)(windows_core::Interface::as_raw(self), pszname.param().abi(), ptype as _, pvalue as _, pcblength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPropA<P0>(&self, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPropA)(windows_core::Interface::as_raw(self), pszname.param().abi(), r#type, pvalue, cblength) }
    }
    pub unsafe fn RemovePropA<P0>(&self, pcwszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemovePropA)(windows_core::Interface::as_raw(self), pcwszname.param().abi()) }
    }
    pub unsafe fn RemoveAllProps(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllProps)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBackupRestoreProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPropCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetPropByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WCHAR, *mut super::WORD, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetPropByIndex: usize,
    #[cfg(feature = "shared")]
    pub GetPropByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropByName: usize,
    #[cfg(feature = "shared")]
    pub SetPropA: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPropA: usize,
    pub RemovePropA: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveAllProps: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMBackupRestoreProps_Impl: windows_core::IUnknownImpl {
    fn GetPropCount(&self) -> windows_core::Result<super::WORD>;
    fn GetPropByIndex(&self, windex: super::WORD, pwszname: *mut super::WCHAR, pcchnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
    fn GetPropByName(&self, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
    fn SetPropA(&self, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::Result<()>;
    fn RemovePropA(&self, pcwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveAllProps(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMBackupRestoreProps_Vtbl {
    pub const fn new<Identity: IWMBackupRestoreProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropCount<Identity: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcprops: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMBackupRestoreProps_Impl::GetPropCount(this) {
                    Ok(ok__) => {
                        pcprops.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropByIndex<Identity: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::WORD, pwszname: *mut super::WCHAR, pcchnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMBackupRestoreProps_Impl::GetPropByIndex(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchnamelen), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn GetPropByName<Identity: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMBackupRestoreProps_Impl::GetPropByName(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn SetPropA<Identity: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMBackupRestoreProps_Impl::SetPropA(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
            }
        }
        unsafe extern "system" fn RemovePropA<Identity: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMBackupRestoreProps_Impl::RemovePropA(this, core::mem::transmute(&pcwszname)).into()
            }
        }
        unsafe extern "system" fn RemoveAllProps<Identity: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMBackupRestoreProps_Impl::RemoveAllProps(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropCount: GetPropCount::<Identity, OFFSET>,
            GetPropByIndex: GetPropByIndex::<Identity, OFFSET>,
            GetPropByName: GetPropByName::<Identity, OFFSET>,
            SetPropA: SetPropA::<Identity, OFFSET>,
            RemovePropA: RemovePropA::<Identity, OFFSET>,
            RemoveAllProps: RemoveAllProps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMBackupRestoreProps as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMBackupRestoreProps {}
windows_core::imp::define_interface!(IWMBandwidthSharing, IWMBandwidthSharing_Vtbl, 0xad694af1_f8d9_42f8_bc47_70311b0c4f9e);
impl core::ops::Deref for IWMBandwidthSharing {
    type Target = IWMStreamList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMBandwidthSharing, windows_core::IUnknown, IWMStreamList);
impl IWMBandwidthSharing {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, guidtype: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), guidtype) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBandwidth(&self, pdwbitrate: *mut super::DWORD, pmsbufferwindow: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBandwidth)(windows_core::Interface::as_raw(self), pdwbitrate as _, pmsbufferwindow as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetBandwidth(&self, dwbitrate: super::DWORD, msbufferwindow: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBandwidth)(windows_core::Interface::as_raw(self), dwbitrate, msbufferwindow) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBandwidthSharing_Vtbl {
    pub base__: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBandwidth: usize,
    #[cfg(feature = "shared")]
    pub SetBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetBandwidth: usize,
}
#[cfg(feature = "shared")]
pub trait IWMBandwidthSharing_Impl: IWMStreamList_Impl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetType(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetBandwidth(&self, pdwbitrate: *mut super::DWORD, pmsbufferwindow: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetBandwidth(&self, dwbitrate: super::DWORD, msbufferwindow: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMBandwidthSharing_Vtbl {
    pub const fn new<Identity: IWMBandwidthSharing_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMBandwidthSharing_Impl::GetType(this) {
                    Ok(ok__) => {
                        pguidtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMBandwidthSharing_Impl::SetType(this, core::mem::transmute_copy(&guidtype)).into()
            }
        }
        unsafe extern "system" fn GetBandwidth<Identity: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbitrate: *mut super::DWORD, pmsbufferwindow: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMBandwidthSharing_Impl::GetBandwidth(this, core::mem::transmute_copy(&pdwbitrate), core::mem::transmute_copy(&pmsbufferwindow)).into()
            }
        }
        unsafe extern "system" fn SetBandwidth<Identity: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbitrate: super::DWORD, msbufferwindow: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMBandwidthSharing_Impl::SetBandwidth(this, core::mem::transmute_copy(&dwbitrate), core::mem::transmute_copy(&msbufferwindow)).into()
            }
        }
        Self {
            base__: IWMStreamList_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            GetBandwidth: GetBandwidth::<Identity, OFFSET>,
            SetBandwidth: SetBandwidth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMBandwidthSharing as windows_core::Interface>::IID || iid == &<IWMStreamList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMBandwidthSharing {}
windows_core::imp::define_interface!(IWMClientConnections, IWMClientConnections_Vtbl, 0x73c66010_a299_41df_b1f0_ccf03b09c1c6);
windows_core::imp::interface_hierarchy!(IWMClientConnections, windows_core::IUnknown);
impl IWMClientConnections {
    #[cfg(feature = "shared")]
    pub unsafe fn GetClientCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClientCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetClientProperties(&self, dwclientnum: super::DWORD) -> windows_core::Result<WM_CLIENT_PROPERTIES> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClientProperties)(windows_core::Interface::as_raw(self), dwclientnum, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetClientCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetClientCount: usize,
    #[cfg(feature = "shared")]
    pub GetClientProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut WM_CLIENT_PROPERTIES) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetClientProperties: usize,
}
#[cfg(feature = "shared")]
pub trait IWMClientConnections_Impl: windows_core::IUnknownImpl {
    fn GetClientCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetClientProperties(&self, dwclientnum: super::DWORD) -> windows_core::Result<WM_CLIENT_PROPERTIES>;
}
#[cfg(feature = "shared")]
impl IWMClientConnections_Vtbl {
    pub const fn new<Identity: IWMClientConnections_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientCount<Identity: IWMClientConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcclients: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMClientConnections_Impl::GetClientCount(this) {
                    Ok(ok__) => {
                        pcclients.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClientProperties<Identity: IWMClientConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclientnum: super::DWORD, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMClientConnections_Impl::GetClientProperties(this, core::mem::transmute_copy(&dwclientnum)) {
                    Ok(ok__) => {
                        pclientproperties.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientCount: GetClientCount::<Identity, OFFSET>,
            GetClientProperties: GetClientProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMClientConnections as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMClientConnections {}
windows_core::imp::define_interface!(IWMClientConnections2, IWMClientConnections2_Vtbl, 0x4091571e_4701_4593_bb3d_d5f5f0c74246);
impl core::ops::Deref for IWMClientConnections2 {
    type Target = IWMClientConnections;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMClientConnections2, windows_core::IUnknown, IWMClientConnections);
impl IWMClientConnections2 {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetClientInfo(&self, dwclientnum: super::DWORD, pwsznetworkaddress: *mut super::WCHAR, pcchnetworkaddress: *mut super::DWORD, pwszport: *mut super::WCHAR, pcchport: *mut super::DWORD, pwszdnsname: *mut super::WCHAR, pcchdnsname: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientInfo)(windows_core::Interface::as_raw(self), dwclientnum, pwsznetworkaddress as _, pcchnetworkaddress as _, pwszport as _, pcchport as _, pwszdnsname as _, pcchdnsname as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections2_Vtbl {
    pub base__: IWMClientConnections_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetClientInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::WCHAR, *mut super::DWORD, *mut super::WCHAR, *mut super::DWORD, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetClientInfo: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMClientConnections2_Impl: IWMClientConnections_Impl {
    fn GetClientInfo(&self, dwclientnum: super::DWORD, pwsznetworkaddress: *mut super::WCHAR, pcchnetworkaddress: *mut super::DWORD, pwszport: *mut super::WCHAR, pcchport: *mut super::DWORD, pwszdnsname: *mut super::WCHAR, pcchdnsname: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMClientConnections2_Vtbl {
    pub const fn new<Identity: IWMClientConnections2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientInfo<Identity: IWMClientConnections2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclientnum: super::DWORD, pwsznetworkaddress: *mut super::WCHAR, pcchnetworkaddress: *mut super::DWORD, pwszport: *mut super::WCHAR, pcchport: *mut super::DWORD, pwszdnsname: *mut super::WCHAR, pcchdnsname: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMClientConnections2_Impl::GetClientInfo(this, core::mem::transmute_copy(&dwclientnum), core::mem::transmute_copy(&pwsznetworkaddress), core::mem::transmute_copy(&pcchnetworkaddress), core::mem::transmute_copy(&pwszport), core::mem::transmute_copy(&pcchport), core::mem::transmute_copy(&pwszdnsname), core::mem::transmute_copy(&pcchdnsname)).into()
            }
        }
        Self { base__: IWMClientConnections_Vtbl::new::<Identity, OFFSET>(), GetClientInfo: GetClientInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMClientConnections2 as windows_core::Interface>::IID || iid == &<IWMClientConnections as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMClientConnections2 {}
windows_core::imp::define_interface!(IWMCodecInfo, IWMCodecInfo_Vtbl, 0xa970f41e_34de_4a98_b3ba_e4b3ca7528f0);
windows_core::imp::interface_hierarchy!(IWMCodecInfo, windows_core::IUnknown);
impl IWMCodecInfo {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodecInfoCount)(windows_core::Interface::as_raw(self), guidtype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodecFormatCount)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCodecFormat(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD) -> windows_core::Result<IWMStreamConfig> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodecFormat)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCodecInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCodecInfoCount: usize,
    #[cfg(feature = "shared")]
    pub GetCodecFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCodecFormatCount: usize,
    #[cfg(feature = "shared")]
    pub GetCodecFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCodecFormat: usize,
}
#[cfg(feature = "shared")]
pub trait IWMCodecInfo_Impl: windows_core::IUnknownImpl {
    fn GetCodecInfoCount(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<super::DWORD>;
    fn GetCodecFormatCount(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn GetCodecFormat(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD) -> windows_core::Result<IWMStreamConfig>;
}
#[cfg(feature = "shared")]
impl IWMCodecInfo_Vtbl {
    pub const fn new<Identity: IWMCodecInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCodecInfoCount<Identity: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, pccodecs: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMCodecInfo_Impl::GetCodecInfoCount(this, core::mem::transmute_copy(&guidtype)) {
                    Ok(ok__) => {
                        pccodecs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodecFormatCount<Identity: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pcformat: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMCodecInfo_Impl::GetCodecFormatCount(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex)) {
                    Ok(ok__) => {
                        pcformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodecFormat<Identity: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD, ppistreamconfig: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMCodecInfo_Impl::GetCodecFormat(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute_copy(&dwformatindex)) {
                    Ok(ok__) => {
                        ppistreamconfig.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Identity, OFFSET>,
            GetCodecFormatCount: GetCodecFormatCount::<Identity, OFFSET>,
            GetCodecFormat: GetCodecFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMCodecInfo {}
windows_core::imp::define_interface!(IWMCodecInfo2, IWMCodecInfo2_Vtbl, 0xaa65e273_b686_4056_91ec_dd768d4df710);
impl core::ops::Deref for IWMCodecInfo2 {
    type Target = IWMCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMCodecInfo2, windows_core::IUnknown, IWMCodecInfo);
impl IWMCodecInfo2 {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetCodecName(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, wszname: *mut super::WCHAR, pcchname: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodecName)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, wszname as _, pcchname as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD, ppistreamconfig: *mut Option<IWMStreamConfig>, wszdesc: *mut super::WCHAR, pcchdesc: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodecFormatDesc)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, core::mem::transmute(ppistreamconfig), wszdesc as _, pcchdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo2_Vtbl {
    pub base__: IWMCodecInfo_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetCodecName: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetCodecName: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetCodecFormatDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetCodecFormatDesc: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMCodecInfo2_Impl: IWMCodecInfo_Impl {
    fn GetCodecName(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, wszname: *mut super::WCHAR, pcchname: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetCodecFormatDesc(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD, ppistreamconfig: windows_core::OutRef<IWMStreamConfig>, wszdesc: *mut super::WCHAR, pcchdesc: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMCodecInfo2_Vtbl {
    pub const fn new<Identity: IWMCodecInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCodecName<Identity: IWMCodecInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, wszname: *mut super::WCHAR, pcchname: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecInfo2_Impl::GetCodecName(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute_copy(&wszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        unsafe extern "system" fn GetCodecFormatDesc<Identity: IWMCodecInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD, ppistreamconfig: *mut *mut core::ffi::c_void, wszdesc: *mut super::WCHAR, pcchdesc: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecInfo2_Impl::GetCodecFormatDesc(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute_copy(&dwformatindex), core::mem::transmute_copy(&ppistreamconfig), core::mem::transmute_copy(&wszdesc), core::mem::transmute_copy(&pcchdesc)).into()
            }
        }
        Self {
            base__: IWMCodecInfo_Vtbl::new::<Identity, OFFSET>(),
            GetCodecName: GetCodecName::<Identity, OFFSET>,
            GetCodecFormatDesc: GetCodecFormatDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecInfo2 as windows_core::Interface>::IID || iid == &<IWMCodecInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMCodecInfo2 {}
windows_core::imp::define_interface!(IWMCodecInfo3, IWMCodecInfo3_Vtbl, 0x7e51f487_4d93_4f98_8ab4_27d0565adc51);
impl core::ops::Deref for IWMCodecInfo3 {
    type Target = IWMCodecInfo2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMCodecInfo3, windows_core::IUnknown, IWMCodecInfo, IWMCodecInfo2);
impl IWMCodecInfo3 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCodecFormatProp<P3>(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD, pszname: P3, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCodecFormatProp)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, pszname.param().abi(), ptype as _, pvalue as _, pdwsize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCodecProp<P2>(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: P2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCodecProp)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.param().abi(), ptype as _, pvalue as _, pdwsize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCodecEnumerationSetting<P2>(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: P2, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, dwsize: super::DWORD) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCodecEnumerationSetting)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.param().abi(), r#type, pvalue, dwsize) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCodecEnumerationSetting<P2>(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: P2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCodecEnumerationSetting)(windows_core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.param().abi(), ptype as _, pvalue as _, pdwsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo3_Vtbl {
    pub base__: IWMCodecInfo2_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCodecFormatProp: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, super::DWORD, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCodecFormatProp: usize,
    #[cfg(feature = "shared")]
    pub GetCodecProp: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCodecProp: usize,
    #[cfg(feature = "shared")]
    pub SetCodecEnumerationSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetCodecEnumerationSetting: usize,
    #[cfg(feature = "shared")]
    pub GetCodecEnumerationSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCodecEnumerationSetting: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMCodecInfo3_Impl: IWMCodecInfo2_Impl {
    fn GetCodecFormatProp(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetCodecProp(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetCodecEnumerationSetting(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, dwsize: super::DWORD) -> windows_core::Result<()>;
    fn GetCodecEnumerationSetting(&self, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMCodecInfo3_Vtbl {
    pub const fn new<Identity: IWMCodecInfo3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCodecFormatProp<Identity: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, dwformatindex: super::DWORD, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecInfo3_Impl::GetCodecFormatProp(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute_copy(&dwformatindex), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        unsafe extern "system" fn GetCodecProp<Identity: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecInfo3_Impl::GetCodecProp(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        unsafe extern "system" fn SetCodecEnumerationSetting<Identity: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, dwsize: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecInfo3_Impl::SetCodecEnumerationSetting(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&dwsize)).into()
            }
        }
        unsafe extern "system" fn GetCodecEnumerationSetting<Identity: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: super::DWORD, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecInfo3_Impl::GetCodecEnumerationSetting(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        Self {
            base__: IWMCodecInfo2_Vtbl::new::<Identity, OFFSET>(),
            GetCodecFormatProp: GetCodecFormatProp::<Identity, OFFSET>,
            GetCodecProp: GetCodecProp::<Identity, OFFSET>,
            SetCodecEnumerationSetting: SetCodecEnumerationSetting::<Identity, OFFSET>,
            GetCodecEnumerationSetting: GetCodecEnumerationSetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecInfo3 as windows_core::Interface>::IID || iid == &<IWMCodecInfo as windows_core::Interface>::IID || iid == &<IWMCodecInfo2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMCodecInfo3 {}
windows_core::imp::define_interface!(IWMCredentialCallback, IWMCredentialCallback_Vtbl, 0x342e0eb7_e651_450c_975b_2ace2c90c48e);
windows_core::imp::interface_hierarchy!(IWMCredentialCallback, windows_core::IUnknown);
impl IWMCredentialCallback {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn AcquireCredentials(&self, pwszrealm: *const super::WCHAR, pwszsite: *const super::WCHAR, pwszuser: *mut super::WCHAR, cchuser: super::DWORD, pwszpassword: *mut super::WCHAR, cchpassword: super::DWORD, hrstatus: windows_core::HRESULT, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireCredentials)(windows_core::Interface::as_raw(self), pwszrealm, pwszsite, pwszuser as _, cchuser, pwszpassword as _, cchpassword, hrstatus, pdwflags as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCredentialCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub AcquireCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *const super::WCHAR, *mut super::WCHAR, super::DWORD, *mut super::WCHAR, super::DWORD, windows_core::HRESULT, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    AcquireCredentials: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMCredentialCallback_Impl: windows_core::IUnknownImpl {
    fn AcquireCredentials(&self, pwszrealm: *const super::WCHAR, pwszsite: *const super::WCHAR, pwszuser: *mut super::WCHAR, cchuser: super::DWORD, pwszpassword: *mut super::WCHAR, cchpassword: super::DWORD, hrstatus: windows_core::HRESULT, pdwflags: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMCredentialCallback_Vtbl {
    pub const fn new<Identity: IWMCredentialCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcquireCredentials<Identity: IWMCredentialCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrealm: *const super::WCHAR, pwszsite: *const super::WCHAR, pwszuser: *mut super::WCHAR, cchuser: super::DWORD, pwszpassword: *mut super::WCHAR, cchpassword: super::DWORD, hrstatus: windows_core::HRESULT, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCredentialCallback_Impl::AcquireCredentials(this, core::mem::transmute_copy(&pwszrealm), core::mem::transmute_copy(&pwszsite), core::mem::transmute_copy(&pwszuser), core::mem::transmute_copy(&cchuser), core::mem::transmute_copy(&pwszpassword), core::mem::transmute_copy(&cchpassword), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AcquireCredentials: AcquireCredentials::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCredentialCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMCredentialCallback {}
windows_core::imp::define_interface!(IWMDRMEditor, IWMDRMEditor_Vtbl, 0xff130ebc_a6c3_42a6_b401_c3382c3e08b3);
windows_core::imp::interface_hierarchy!(IWMDRMEditor, windows_core::IUnknown);
impl IWMDRMEditor {
    #[cfg(feature = "shared")]
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDRMProperty)(windows_core::Interface::as_raw(self), pwstrname.param().abi(), pdwtype as _, pvalue as _, pcblength as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMEditor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetDRMProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDRMProperty: usize,
}
#[cfg(feature = "shared")]
pub trait IWMDRMEditor_Impl: windows_core::IUnknownImpl {
    fn GetDRMProperty(&self, pwstrname: &windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMDRMEditor_Vtbl {
    pub const fn new<Identity: IWMDRMEditor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDRMProperty<Identity: IWMDRMEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrname: windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMEditor_Impl::GetDRMProperty(this, core::mem::transmute(&pwstrname), core::mem::transmute_copy(&pdwtype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDRMProperty: GetDRMProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMEditor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMDRMEditor {}
windows_core::imp::define_interface!(IWMDRMMessageParser, IWMDRMMessageParser_Vtbl, 0xa73a0072_25a0_4c99_b4a5_ede8101a6c39);
windows_core::imp::interface_hierarchy!(IWMDRMMessageParser, windows_core::IUnknown);
impl IWMDRMMessageParser {
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: *const super::BYTE, cbregistrationreqmsg: super::DWORD, ppdevicecert: *mut Option<super::INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ParseRegistrationReqMsg)(windows_core::Interface::as_raw(self), pbregistrationreqmsg, cbregistrationreqmsg, core::mem::transmute(ppdevicecert), pdeviceserialnumber as _) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: *const super::BYTE, cblicenserequestmsg: super::DWORD, ppdevicecert: *mut Option<super::INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ParseLicenseRequestMsg)(windows_core::Interface::as_raw(self), pblicenserequestmsg, cblicenserequestmsg, core::mem::transmute(ppdevicecert), pdeviceserialnumber as _, core::mem::transmute(pbstraction)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMMessageParser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub ParseRegistrationReqMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, *mut *mut core::ffi::c_void, *mut DRM_VAL16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    ParseRegistrationReqMsg: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub ParseLicenseRequestMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, *mut *mut core::ffi::c_void, *mut DRM_VAL16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    ParseLicenseRequestMsg: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMDRMMessageParser_Impl: windows_core::IUnknownImpl {
    fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: *const super::BYTE, cbregistrationreqmsg: super::DWORD, ppdevicecert: windows_core::OutRef<super::INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> windows_core::Result<()>;
    fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: *const super::BYTE, cblicenserequestmsg: super::DWORD, ppdevicecert: windows_core::OutRef<super::INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMDRMMessageParser_Vtbl {
    pub const fn new<Identity: IWMDRMMessageParser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseRegistrationReqMsg<Identity: IWMDRMMessageParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregistrationreqmsg: *const super::BYTE, cbregistrationreqmsg: super::DWORD, ppdevicecert: *mut *mut core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMMessageParser_Impl::ParseRegistrationReqMsg(this, core::mem::transmute_copy(&pbregistrationreqmsg), core::mem::transmute_copy(&cbregistrationreqmsg), core::mem::transmute_copy(&ppdevicecert), core::mem::transmute_copy(&pdeviceserialnumber)).into()
            }
        }
        unsafe extern "system" fn ParseLicenseRequestMsg<Identity: IWMDRMMessageParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblicenserequestmsg: *const super::BYTE, cblicenserequestmsg: super::DWORD, ppdevicecert: *mut *mut core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMMessageParser_Impl::ParseLicenseRequestMsg(this, core::mem::transmute_copy(&pblicenserequestmsg), core::mem::transmute_copy(&cblicenserequestmsg), core::mem::transmute_copy(&ppdevicecert), core::mem::transmute_copy(&pdeviceserialnumber), core::mem::transmute_copy(&pbstraction)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseRegistrationReqMsg: ParseRegistrationReqMsg::<Identity, OFFSET>,
            ParseLicenseRequestMsg: ParseLicenseRequestMsg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMMessageParser as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMDRMMessageParser {}
windows_core::imp::define_interface!(IWMDRMReader, IWMDRMReader_Vtbl, 0xd2827540_3ee7_432c_b14c_dc17f085d3b3);
windows_core::imp::interface_hierarchy!(IWMDRMReader, windows_core::IUnknown);
impl IWMDRMReader {
    #[cfg(feature = "shared")]
    pub unsafe fn AcquireLicense(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireLicense)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelLicenseAcquisition)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Individualize(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Individualize)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn CancelIndividualization(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelIndividualization)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MonitorLicenseAcquisition)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelMonitorLicenseAcquisition)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDRMProperty<P0>(&self, pwstrname: P0, dwtype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDRMProperty)(windows_core::Interface::as_raw(self), pwstrname.param().abi(), dwtype, pvalue, cblength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDRMProperty)(windows_core::Interface::as_raw(self), pwstrname.param().abi(), pdwtype as _, pvalue as _, pcblength as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub AcquireLicense: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AcquireLicense: usize,
    pub CancelLicenseAcquisition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Individualize: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Individualize: usize,
    pub CancelIndividualization: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MonitorLicenseAcquisition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelMonitorLicenseAcquisition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetDRMProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDRMProperty: usize,
    #[cfg(feature = "shared")]
    pub GetDRMProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDRMProperty: usize,
}
#[cfg(feature = "shared")]
pub trait IWMDRMReader_Impl: windows_core::IUnknownImpl {
    fn AcquireLicense(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn CancelLicenseAcquisition(&self) -> windows_core::Result<()>;
    fn Individualize(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn CancelIndividualization(&self) -> windows_core::Result<()>;
    fn MonitorLicenseAcquisition(&self) -> windows_core::Result<()>;
    fn CancelMonitorLicenseAcquisition(&self) -> windows_core::Result<()>;
    fn SetDRMProperty(&self, pwstrname: &windows_core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::Result<()>;
    fn GetDRMProperty(&self, pwstrname: &windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMDRMReader_Vtbl {
    pub const fn new<Identity: IWMDRMReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcquireLicense<Identity: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader_Impl::AcquireLicense(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn CancelLicenseAcquisition<Identity: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader_Impl::CancelLicenseAcquisition(this).into()
            }
        }
        unsafe extern "system" fn Individualize<Identity: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader_Impl::Individualize(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn CancelIndividualization<Identity: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader_Impl::CancelIndividualization(this).into()
            }
        }
        unsafe extern "system" fn MonitorLicenseAcquisition<Identity: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader_Impl::MonitorLicenseAcquisition(this).into()
            }
        }
        unsafe extern "system" fn CancelMonitorLicenseAcquisition<Identity: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader_Impl::CancelMonitorLicenseAcquisition(this).into()
            }
        }
        unsafe extern "system" fn SetDRMProperty<Identity: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrname: windows_core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader_Impl::SetDRMProperty(this, core::mem::transmute(&pwstrname), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
            }
        }
        unsafe extern "system" fn GetDRMProperty<Identity: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrname: windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader_Impl::GetDRMProperty(this, core::mem::transmute(&pwstrname), core::mem::transmute_copy(&pdwtype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireLicense: AcquireLicense::<Identity, OFFSET>,
            CancelLicenseAcquisition: CancelLicenseAcquisition::<Identity, OFFSET>,
            Individualize: Individualize::<Identity, OFFSET>,
            CancelIndividualization: CancelIndividualization::<Identity, OFFSET>,
            MonitorLicenseAcquisition: MonitorLicenseAcquisition::<Identity, OFFSET>,
            CancelMonitorLicenseAcquisition: CancelMonitorLicenseAcquisition::<Identity, OFFSET>,
            SetDRMProperty: SetDRMProperty::<Identity, OFFSET>,
            GetDRMProperty: GetDRMProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMDRMReader {}
windows_core::imp::define_interface!(IWMDRMReader2, IWMDRMReader2_Vtbl, 0xbefe7a75_9f1d_4075_b9d9_a3c37bda49a0);
impl core::ops::Deref for IWMDRMReader2 {
    type Target = IWMDRMReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMReader2, windows_core::IUnknown, IWMDRMReader);
impl IWMDRMReader2 {
    pub unsafe fn SetEvaluateOutputLevelLicenses(&self, fevaluate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEvaluateOutputLevelLicenses)(windows_core::Interface::as_raw(self), fevaluate.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut super::DWORD, pdwminappcompliancelevel: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPlayOutputLevels)(windows_core::Interface::as_raw(self), pplayopl as _, pcblength as _, pdwminappcompliancelevel as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut super::DWORD, pdwminappcompliancelevel: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCopyOutputLevels)(windows_core::Interface::as_raw(self), pcopyopl as _, pcblength as _, pdwminappcompliancelevel as _) }
    }
    pub unsafe fn TryNextLicense(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TryNextLicense)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader2_Vtbl {
    pub base__: IWMDRMReader_Vtbl,
    pub SetEvaluateOutputLevelLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetPlayOutputLevels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DRM_PLAY_OPL, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPlayOutputLevels: usize,
    #[cfg(feature = "shared")]
    pub GetCopyOutputLevels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DRM_COPY_OPL, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCopyOutputLevels: usize,
    pub TryNextLicense: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMDRMReader2_Impl: IWMDRMReader_Impl {
    fn SetEvaluateOutputLevelLicenses(&self, fevaluate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut super::DWORD, pdwminappcompliancelevel: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut super::DWORD, pdwminappcompliancelevel: *mut super::DWORD) -> windows_core::Result<()>;
    fn TryNextLicense(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMDRMReader2_Vtbl {
    pub const fn new<Identity: IWMDRMReader2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEvaluateOutputLevelLicenses<Identity: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fevaluate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader2_Impl::SetEvaluateOutputLevelLicenses(this, core::mem::transmute_copy(&fevaluate)).into()
            }
        }
        unsafe extern "system" fn GetPlayOutputLevels<Identity: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut super::DWORD, pdwminappcompliancelevel: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader2_Impl::GetPlayOutputLevels(this, core::mem::transmute_copy(&pplayopl), core::mem::transmute_copy(&pcblength), core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
            }
        }
        unsafe extern "system" fn GetCopyOutputLevels<Identity: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut super::DWORD, pdwminappcompliancelevel: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader2_Impl::GetCopyOutputLevels(this, core::mem::transmute_copy(&pcopyopl), core::mem::transmute_copy(&pcblength), core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
            }
        }
        unsafe extern "system" fn TryNextLicense<Identity: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader2_Impl::TryNextLicense(this).into()
            }
        }
        Self {
            base__: IWMDRMReader_Vtbl::new::<Identity, OFFSET>(),
            SetEvaluateOutputLevelLicenses: SetEvaluateOutputLevelLicenses::<Identity, OFFSET>,
            GetPlayOutputLevels: GetPlayOutputLevels::<Identity, OFFSET>,
            GetCopyOutputLevels: GetCopyOutputLevels::<Identity, OFFSET>,
            TryNextLicense: TryNextLicense::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMReader2 as windows_core::Interface>::IID || iid == &<IWMDRMReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMDRMReader2 {}
windows_core::imp::define_interface!(IWMDRMReader3, IWMDRMReader3_Vtbl, 0xe08672de_f1e7_4ff4_a0a3_fc4b08e4caf8);
impl core::ops::Deref for IWMDRMReader3 {
    type Target = IWMDRMReader2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMReader3, windows_core::IUnknown, IWMDRMReader, IWMDRMReader2);
impl IWMDRMReader3 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetInclusionList(&self, ppguids: *mut *mut windows_core::GUID, pcguids: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInclusionList)(windows_core::Interface::as_raw(self), ppguids as _, pcguids as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader3_Vtbl {
    pub base__: IWMDRMReader2_Vtbl,
    #[cfg(feature = "shared")]
    pub GetInclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::GUID, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInclusionList: usize,
}
#[cfg(feature = "shared")]
pub trait IWMDRMReader3_Impl: IWMDRMReader2_Impl {
    fn GetInclusionList(&self, ppguids: *mut *mut windows_core::GUID, pcguids: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMDRMReader3_Vtbl {
    pub const fn new<Identity: IWMDRMReader3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInclusionList<Identity: IWMDRMReader3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppguids: *mut *mut windows_core::GUID, pcguids: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMReader3_Impl::GetInclusionList(this, core::mem::transmute_copy(&ppguids), core::mem::transmute_copy(&pcguids)).into()
            }
        }
        Self { base__: IWMDRMReader2_Vtbl::new::<Identity, OFFSET>(), GetInclusionList: GetInclusionList::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMReader3 as windows_core::Interface>::IID || iid == &<IWMDRMReader as windows_core::Interface>::IID || iid == &<IWMDRMReader2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMDRMReader3 {}
windows_core::imp::define_interface!(IWMDRMTranscryptionManager, IWMDRMTranscryptionManager_Vtbl, 0xb1a887b2_a4f0_407a_b02e_efbd23bbecdf);
windows_core::imp::interface_hierarchy!(IWMDRMTranscryptionManager, windows_core::IUnknown);
impl IWMDRMTranscryptionManager {
    pub unsafe fn CreateTranscryptor(&self) -> windows_core::Result<IWMDRMTranscryptor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTranscryptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTranscryptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDRMTranscryptionManager_Impl: windows_core::IUnknownImpl {
    fn CreateTranscryptor(&self) -> windows_core::Result<IWMDRMTranscryptor>;
}
impl IWMDRMTranscryptionManager_Vtbl {
    pub const fn new<Identity: IWMDRMTranscryptionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTranscryptor<Identity: IWMDRMTranscryptionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptranscryptor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDRMTranscryptionManager_Impl::CreateTranscryptor(this) {
                    Ok(ok__) => {
                        pptranscryptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTranscryptor: CreateTranscryptor::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMTranscryptionManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMDRMTranscryptionManager {}
windows_core::imp::define_interface!(IWMDRMTranscryptor, IWMDRMTranscryptor_Vtbl, 0x69059850_6e6f_4bb2_806f_71863ddfc471);
windows_core::imp::interface_hierarchy!(IWMDRMTranscryptor, windows_core::IUnknown);
impl IWMDRMTranscryptor {
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn Initialize<P4>(&self, bstrfilename: &windows_core::BSTR, pblicenserequestmsg: *const super::BYTE, cblicenserequestmsg: super::DWORD, pplicenseresponsemsg: *mut Option<super::INSSBuffer>, pcallback: P4, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IWMStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfilename), pblicenserequestmsg, cblicenserequestmsg, core::mem::transmute(pplicenseresponsemsg), pcallback.param().abi(), pvcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Seek(&self, hnstime: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), hnstime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Read(&self, pbdata: *const super::BYTE, pcbdata: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pbdata, pcbdata) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::BYTE, super::DWORD, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    Initialize: usize,
    #[cfg(feature = "shared")]
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Seek: usize,
    #[cfg(feature = "shared")]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Read: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMDRMTranscryptor_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, bstrfilename: &windows_core::BSTR, pblicenserequestmsg: *const super::BYTE, cblicenserequestmsg: super::DWORD, pplicenseresponsemsg: windows_core::OutRef<super::INSSBuffer>, pcallback: windows_core::Ref<IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Seek(&self, hnstime: super::QWORD) -> windows_core::Result<()>;
    fn Read(&self, pbdata: *const super::BYTE, pcbdata: *const super::DWORD) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMDRMTranscryptor_Vtbl {
    pub const fn new<Identity: IWMDRMTranscryptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, pblicenserequestmsg: *const super::BYTE, cblicenserequestmsg: super::DWORD, pplicenseresponsemsg: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMTranscryptor_Impl::Initialize(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&pblicenserequestmsg), core::mem::transmute_copy(&cblicenserequestmsg), core::mem::transmute_copy(&pplicenseresponsemsg), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnstime: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMTranscryptor_Impl::Seek(this, core::mem::transmute_copy(&hnstime)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *const super::BYTE, pcbdata: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMTranscryptor_Impl::Read(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&pcbdata)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMTranscryptor_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMDRMTranscryptor {}
windows_core::imp::define_interface!(IWMDRMTranscryptor2, IWMDRMTranscryptor2_Vtbl, 0xe0da439f_d331_496a_bece_18e5bac5dd23);
impl core::ops::Deref for IWMDRMTranscryptor2 {
    type Target = IWMDRMTranscryptor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMTranscryptor2, windows_core::IUnknown, IWMDRMTranscryptor);
impl IWMDRMTranscryptor2 {
    #[cfg(feature = "shared")]
    pub unsafe fn SeekEx(&self, cnsstarttime: super::QWORD, cnsduration: super::QWORD, flrate: f32, fincludefileheader: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SeekEx)(windows_core::Interface::as_raw(self), cnsstarttime, cnsduration, flrate, fincludefileheader.into()) }
    }
    pub unsafe fn ZeroAdjustTimestamps(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ZeroAdjustTimestamps)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSeekStartTime(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSeekStartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDuration(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor2_Vtbl {
    pub base__: IWMDRMTranscryptor_Vtbl,
    #[cfg(feature = "shared")]
    pub SeekEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD, super::QWORD, f32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SeekEx: usize,
    pub ZeroAdjustTimestamps: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetSeekStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSeekStartTime: usize,
    #[cfg(feature = "shared")]
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDuration: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMDRMTranscryptor2_Impl: IWMDRMTranscryptor_Impl {
    fn SeekEx(&self, cnsstarttime: super::QWORD, cnsduration: super::QWORD, flrate: f32, fincludefileheader: windows_core::BOOL) -> windows_core::Result<()>;
    fn ZeroAdjustTimestamps(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSeekStartTime(&self) -> windows_core::Result<super::QWORD>;
    fn GetDuration(&self) -> windows_core::Result<super::QWORD>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMDRMTranscryptor2_Vtbl {
    pub const fn new<Identity: IWMDRMTranscryptor2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SeekEx<Identity: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstarttime: super::QWORD, cnsduration: super::QWORD, flrate: f32, fincludefileheader: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMTranscryptor2_Impl::SeekEx(this, core::mem::transmute_copy(&cnsstarttime), core::mem::transmute_copy(&cnsduration), core::mem::transmute_copy(&flrate), core::mem::transmute_copy(&fincludefileheader)).into()
            }
        }
        unsafe extern "system" fn ZeroAdjustTimestamps<Identity: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMTranscryptor2_Impl::ZeroAdjustTimestamps(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn GetSeekStartTime<Identity: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnstime: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDRMTranscryptor2_Impl::GetSeekStartTime(this) {
                    Ok(ok__) => {
                        pcnstime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDuration<Identity: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsduration: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDRMTranscryptor2_Impl::GetDuration(this) {
                    Ok(ok__) => {
                        pcnsduration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMDRMTranscryptor_Vtbl::new::<Identity, OFFSET>(),
            SeekEx: SeekEx::<Identity, OFFSET>,
            ZeroAdjustTimestamps: ZeroAdjustTimestamps::<Identity, OFFSET>,
            GetSeekStartTime: GetSeekStartTime::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor2 as windows_core::Interface>::IID || iid == &<IWMDRMTranscryptor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMDRMTranscryptor2 {}
windows_core::imp::define_interface!(IWMDRMWriter, IWMDRMWriter_Vtbl, 0xd6ea5dd0_12a0_43f4_90ab_a3fd451e6a07);
windows_core::imp::interface_hierarchy!(IWMDRMWriter, windows_core::IUnknown);
impl IWMDRMWriter {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: *mut super::WCHAR, pcwchlength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GenerateKeySeed)(windows_core::Interface::as_raw(self), pwszkeyseed as _, pcwchlength as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: *mut super::WCHAR, pcwchlength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GenerateKeyID)(windows_core::Interface::as_raw(self), pwszkeyid as _, pcwchlength as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: *mut super::WCHAR, pcwchprivkeylength: *mut super::DWORD, pwszpubkey: *mut super::WCHAR, pcwchpubkeylength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GenerateSigningKeyPair)(windows_core::Interface::as_raw(self), pwszprivkey as _, pcwchprivkeylength as _, pwszpubkey as _, pcwchpubkeylength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDRMAttribute<P1>(&self, wstreamnum: super::WORD, pszname: P1, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDRMAttribute)(windows_core::Interface::as_raw(self), wstreamnum, pszname.param().abi(), r#type, pvalue, cblength) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GenerateKeySeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GenerateKeySeed: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GenerateKeyID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GenerateKeyID: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GenerateSigningKeyPair: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GenerateSigningKeyPair: usize,
    #[cfg(feature = "shared")]
    pub SetDRMAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDRMAttribute: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMDRMWriter_Impl: windows_core::IUnknownImpl {
    fn GenerateKeySeed(&self, pwszkeyseed: *mut super::WCHAR, pcwchlength: *mut super::DWORD) -> windows_core::Result<()>;
    fn GenerateKeyID(&self, pwszkeyid: *mut super::WCHAR, pcwchlength: *mut super::DWORD) -> windows_core::Result<()>;
    fn GenerateSigningKeyPair(&self, pwszprivkey: *mut super::WCHAR, pcwchprivkeylength: *mut super::DWORD, pwszpubkey: *mut super::WCHAR, pcwchpubkeylength: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetDRMAttribute(&self, wstreamnum: super::WORD, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMDRMWriter_Vtbl {
    pub const fn new<Identity: IWMDRMWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GenerateKeySeed<Identity: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszkeyseed: *mut super::WCHAR, pcwchlength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMWriter_Impl::GenerateKeySeed(this, core::mem::transmute_copy(&pwszkeyseed), core::mem::transmute_copy(&pcwchlength)).into()
            }
        }
        unsafe extern "system" fn GenerateKeyID<Identity: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszkeyid: *mut super::WCHAR, pcwchlength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMWriter_Impl::GenerateKeyID(this, core::mem::transmute_copy(&pwszkeyid), core::mem::transmute_copy(&pcwchlength)).into()
            }
        }
        unsafe extern "system" fn GenerateSigningKeyPair<Identity: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprivkey: *mut super::WCHAR, pcwchprivkeylength: *mut super::DWORD, pwszpubkey: *mut super::WCHAR, pcwchpubkeylength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMWriter_Impl::GenerateSigningKeyPair(this, core::mem::transmute_copy(&pwszprivkey), core::mem::transmute_copy(&pcwchprivkeylength), core::mem::transmute_copy(&pwszpubkey), core::mem::transmute_copy(&pcwchpubkeylength)).into()
            }
        }
        unsafe extern "system" fn SetDRMAttribute<Identity: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMWriter_Impl::SetDRMAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GenerateKeySeed: GenerateKeySeed::<Identity, OFFSET>,
            GenerateKeyID: GenerateKeyID::<Identity, OFFSET>,
            GenerateSigningKeyPair: GenerateSigningKeyPair::<Identity, OFFSET>,
            SetDRMAttribute: SetDRMAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMWriter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMDRMWriter {}
windows_core::imp::define_interface!(IWMDRMWriter2, IWMDRMWriter2_Vtbl, 0x38ee7a94_40e2_4e10_aa3f_33fd3210ed5b);
impl core::ops::Deref for IWMDRMWriter2 {
    type Target = IWMDRMWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMWriter2, windows_core::IUnknown, IWMDRMWriter);
impl IWMDRMWriter2 {
    #[cfg(feature = "shared")]
    pub unsafe fn SetWMDRMNetEncryption(&self, fsamplesencrypted: bool, pbkeyid: *const super::BYTE, cbkeyid: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWMDRMNetEncryption)(windows_core::Interface::as_raw(self), fsamplesencrypted.into(), pbkeyid, cbkeyid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter2_Vtbl {
    pub base__: IWMDRMWriter_Vtbl,
    #[cfg(feature = "shared")]
    pub SetWMDRMNetEncryption: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetWMDRMNetEncryption: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMDRMWriter2_Impl: IWMDRMWriter_Impl {
    fn SetWMDRMNetEncryption(&self, fsamplesencrypted: windows_core::BOOL, pbkeyid: *const super::BYTE, cbkeyid: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMDRMWriter2_Vtbl {
    pub const fn new<Identity: IWMDRMWriter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetWMDRMNetEncryption<Identity: IWMDRMWriter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsamplesencrypted: windows_core::BOOL, pbkeyid: *const super::BYTE, cbkeyid: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMWriter2_Impl::SetWMDRMNetEncryption(this, core::mem::transmute_copy(&fsamplesencrypted), core::mem::transmute_copy(&pbkeyid), core::mem::transmute_copy(&cbkeyid)).into()
            }
        }
        Self { base__: IWMDRMWriter_Vtbl::new::<Identity, OFFSET>(), SetWMDRMNetEncryption: SetWMDRMNetEncryption::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMWriter2 as windows_core::Interface>::IID || iid == &<IWMDRMWriter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMDRMWriter2 {}
windows_core::imp::define_interface!(IWMDRMWriter3, IWMDRMWriter3_Vtbl, 0xa7184082_a4aa_4dde_ac9c_e75dbd1117ce);
impl core::ops::Deref for IWMDRMWriter3 {
    type Target = IWMDRMWriter2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDRMWriter3, windows_core::IUnknown, IWMDRMWriter, IWMDRMWriter2);
impl IWMDRMWriter3 {
    #[cfg(feature = "shared")]
    pub unsafe fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProtectStreamSamples)(windows_core::Interface::as_raw(self), pimportinitstruct) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter3_Vtbl {
    pub base__: IWMDRMWriter2_Vtbl,
    #[cfg(feature = "shared")]
    pub SetProtectStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *const WMDRM_IMPORT_INIT_STRUCT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetProtectStreamSamples: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMDRMWriter3_Impl: IWMDRMWriter2_Impl {
    fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMDRMWriter3_Vtbl {
    pub const fn new<Identity: IWMDRMWriter3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: IWMDRMWriter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDRMWriter3_Impl::SetProtectStreamSamples(this, core::mem::transmute_copy(&pimportinitstruct)).into()
            }
        }
        Self { base__: IWMDRMWriter2_Vtbl::new::<Identity, OFFSET>(), SetProtectStreamSamples: SetProtectStreamSamples::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMWriter3 as windows_core::Interface>::IID || iid == &<IWMDRMWriter as windows_core::Interface>::IID || iid == &<IWMDRMWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMDRMWriter3 {}
windows_core::imp::define_interface!(IWMDeviceRegistration, IWMDeviceRegistration_Vtbl, 0xf6211f03_8d21_4e94_93e6_8510805f2d99);
windows_core::imp::interface_hierarchy!(IWMDeviceRegistration, windows_core::IUnknown);
impl IWMDeviceRegistration {
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterDevice(&self, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: DRM_VAL16) -> windows_core::Result<IWMRegisteredDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterDevice)(windows_core::Interface::as_raw(self), dwregistertype, pbcertificate, cbcertificate, serialnumber, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UnregisterDevice(&self, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: DRM_VAL16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterDevice)(windows_core::Interface::as_raw(self), dwregistertype, pbcertificate, cbcertificate, serialnumber) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRegistrationStats(&self, dwregistertype: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegistrationStats)(windows_core::Interface::as_raw(self), dwregistertype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFirstRegisteredDevice(&self, dwregistertype: super::DWORD) -> windows_core::Result<IWMRegisteredDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstRegisteredDevice)(windows_core::Interface::as_raw(self), dwregistertype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextRegisteredDevice(&self) -> windows_core::Result<IWMRegisteredDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextRegisteredDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRegisteredDeviceByID(&self, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: DRM_VAL16) -> windows_core::Result<IWMRegisteredDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegisteredDeviceByID)(windows_core::Interface::as_raw(self), dwregistertype, pbcertificate, cbcertificate, serialnumber, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub RegisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::BYTE, super::DWORD, DRM_VAL16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterDevice: usize,
    #[cfg(feature = "shared")]
    pub UnregisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::BYTE, super::DWORD, DRM_VAL16) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UnregisterDevice: usize,
    #[cfg(feature = "shared")]
    pub GetRegistrationStats: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRegistrationStats: usize,
    #[cfg(feature = "shared")]
    pub GetFirstRegisteredDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFirstRegisteredDevice: usize,
    pub GetNextRegisteredDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetRegisteredDeviceByID: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::BYTE, super::DWORD, DRM_VAL16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRegisteredDeviceByID: usize,
}
#[cfg(feature = "shared")]
pub trait IWMDeviceRegistration_Impl: windows_core::IUnknownImpl {
    fn RegisterDevice(&self, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: &DRM_VAL16) -> windows_core::Result<IWMRegisteredDevice>;
    fn UnregisterDevice(&self, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: &DRM_VAL16) -> windows_core::Result<()>;
    fn GetRegistrationStats(&self, dwregistertype: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn GetFirstRegisteredDevice(&self, dwregistertype: super::DWORD) -> windows_core::Result<IWMRegisteredDevice>;
    fn GetNextRegisteredDevice(&self) -> windows_core::Result<IWMRegisteredDevice>;
    fn GetRegisteredDeviceByID(&self, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: &DRM_VAL16) -> windows_core::Result<IWMRegisteredDevice>;
}
#[cfg(feature = "shared")]
impl IWMDeviceRegistration_Vtbl {
    pub const fn new<Identity: IWMDeviceRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterDevice<Identity: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: DRM_VAL16, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceRegistration_Impl::RegisterDevice(this, core::mem::transmute_copy(&dwregistertype), core::mem::transmute_copy(&pbcertificate), core::mem::transmute_copy(&cbcertificate), core::mem::transmute(&serialnumber)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterDevice<Identity: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: DRM_VAL16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDeviceRegistration_Impl::UnregisterDevice(this, core::mem::transmute_copy(&dwregistertype), core::mem::transmute_copy(&pbcertificate), core::mem::transmute_copy(&cbcertificate), core::mem::transmute(&serialnumber)).into()
            }
        }
        unsafe extern "system" fn GetRegistrationStats<Identity: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: super::DWORD, pcregistereddevices: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceRegistration_Impl::GetRegistrationStats(this, core::mem::transmute_copy(&dwregistertype)) {
                    Ok(ok__) => {
                        pcregistereddevices.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFirstRegisteredDevice<Identity: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: super::DWORD, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceRegistration_Impl::GetFirstRegisteredDevice(this, core::mem::transmute_copy(&dwregistertype)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextRegisteredDevice<Identity: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceRegistration_Impl::GetNextRegisteredDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRegisteredDeviceByID<Identity: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: super::DWORD, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, serialnumber: DRM_VAL16, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceRegistration_Impl::GetRegisteredDeviceByID(this, core::mem::transmute_copy(&dwregistertype), core::mem::transmute_copy(&pbcertificate), core::mem::transmute_copy(&cbcertificate), core::mem::transmute(&serialnumber)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDevice: RegisterDevice::<Identity, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, OFFSET>,
            GetRegistrationStats: GetRegistrationStats::<Identity, OFFSET>,
            GetFirstRegisteredDevice: GetFirstRegisteredDevice::<Identity, OFFSET>,
            GetNextRegisteredDevice: GetNextRegisteredDevice::<Identity, OFFSET>,
            GetRegisteredDeviceByID: GetRegisteredDeviceByID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDeviceRegistration as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMDeviceRegistration {}
windows_core::imp::define_interface!(IWMHeaderInfo, IWMHeaderInfo_Vtbl, 0x96406bda_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMHeaderInfo, windows_core::IUnknown);
impl IWMHeaderInfo {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttributeCount(&self, wstreamnum: super::WORD) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeCount)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetAttributeByIndex(&self, windex: super::WORD, pwstreamnum: *mut super::WORD, pwszname: *mut super::WCHAR, pcchnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeByIndex)(windows_core::Interface::as_raw(self), windex, pwstreamnum as _, pwszname as _, pcchnamelen as _, ptype as _, pvalue as _, pcblength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttributeByName<P1>(&self, pwstreamnum: *mut super::WORD, pszname: P1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeByName)(windows_core::Interface::as_raw(self), pwstreamnum as _, pszname.param().abi(), ptype as _, pvalue as _, pcblength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAttribute<P1>(&self, wstreamnum: super::WORD, pszname: P1, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAttribute)(windows_core::Interface::as_raw(self), wstreamnum, pszname.param().abi(), r#type, pvalue, cblength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMarkerCount(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMarkerCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetMarker(&self, windex: super::WORD, pwszmarkername: *mut super::WCHAR, pcchmarkernamelen: *mut super::WORD, pcnsmarkertime: *mut super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMarker)(windows_core::Interface::as_raw(self), windex, pwszmarkername as _, pcchmarkernamelen as _, pcnsmarkertime as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddMarker<P0>(&self, pwszmarkername: P0, cnsmarkertime: super::QWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMarker)(windows_core::Interface::as_raw(self), pwszmarkername.param().abi(), cnsmarkertime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveMarker(&self, windex: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveMarker)(windows_core::Interface::as_raw(self), windex) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetScriptCount(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScriptCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetScript(&self, windex: super::WORD, pwsztype: *mut super::WCHAR, pcchtypelen: *mut super::WORD, pwszcommand: *mut super::WCHAR, pcchcommandlen: *mut super::WORD, pcnsscripttime: *mut super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScript)(windows_core::Interface::as_raw(self), windex, pwsztype as _, pcchtypelen as _, pwszcommand as _, pcchcommandlen as _, pcnsscripttime as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddScript<P0, P1>(&self, pwsztype: P0, pwszcommand: P1, cnsscripttime: super::QWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddScript)(windows_core::Interface::as_raw(self), pwsztype.param().abi(), pwszcommand.param().abi(), cnsscripttime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveScript(&self, windex: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveScript)(windows_core::Interface::as_raw(self), windex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAttributeCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttributeCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetAttributeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WORD, *mut super::WCHAR, *mut super::WORD, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetAttributeByIndex: usize,
    #[cfg(feature = "shared")]
    pub GetAttributeByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttributeByName: usize,
    #[cfg(feature = "shared")]
    pub SetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAttribute: usize,
    #[cfg(feature = "shared")]
    pub GetMarkerCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMarkerCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WCHAR, *mut super::WORD, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetMarker: usize,
    #[cfg(feature = "shared")]
    pub AddMarker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddMarker: usize,
    #[cfg(feature = "shared")]
    pub RemoveMarker: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveMarker: usize,
    #[cfg(feature = "shared")]
    pub GetScriptCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetScriptCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetScript: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WCHAR, *mut super::WORD, *mut super::WCHAR, *mut super::WORD, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetScript: usize,
    #[cfg(feature = "shared")]
    pub AddScript: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddScript: usize,
    #[cfg(feature = "shared")]
    pub RemoveScript: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveScript: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMHeaderInfo_Impl: windows_core::IUnknownImpl {
    fn GetAttributeCount(&self, wstreamnum: super::WORD) -> windows_core::Result<super::WORD>;
    fn GetAttributeByIndex(&self, windex: super::WORD, pwstreamnum: *mut super::WORD, pwszname: *mut super::WCHAR, pcchnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
    fn GetAttributeByName(&self, pwstreamnum: *mut super::WORD, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
    fn SetAttribute(&self, wstreamnum: super::WORD, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::Result<()>;
    fn GetMarkerCount(&self) -> windows_core::Result<super::WORD>;
    fn GetMarker(&self, windex: super::WORD, pwszmarkername: *mut super::WCHAR, pcchmarkernamelen: *mut super::WORD, pcnsmarkertime: *mut super::QWORD) -> windows_core::Result<()>;
    fn AddMarker(&self, pwszmarkername: &windows_core::PCWSTR, cnsmarkertime: super::QWORD) -> windows_core::Result<()>;
    fn RemoveMarker(&self, windex: super::WORD) -> windows_core::Result<()>;
    fn GetScriptCount(&self) -> windows_core::Result<super::WORD>;
    fn GetScript(&self, windex: super::WORD, pwsztype: *mut super::WCHAR, pcchtypelen: *mut super::WORD, pwszcommand: *mut super::WCHAR, pcchcommandlen: *mut super::WORD, pcnsscripttime: *mut super::QWORD) -> windows_core::Result<()>;
    fn AddScript(&self, pwsztype: &windows_core::PCWSTR, pwszcommand: &windows_core::PCWSTR, cnsscripttime: super::QWORD) -> windows_core::Result<()>;
    fn RemoveScript(&self, windex: super::WORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMHeaderInfo_Vtbl {
    pub const fn new<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAttributeCount<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pcattributes: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMHeaderInfo_Impl::GetAttributeCount(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pcattributes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::WORD, pwstreamnum: *mut super::WORD, pwszname: *mut super::WCHAR, pcchnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::GetAttributeByIndex(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwstreamnum), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchnamelen), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn GetAttributeByName<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstreamnum: *mut super::WORD, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::GetAttributeByName(this, core::mem::transmute_copy(&pwstreamnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn SetAttribute<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::SetAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
            }
        }
        unsafe extern "system" fn GetMarkerCount<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmarkers: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMHeaderInfo_Impl::GetMarkerCount(this) {
                    Ok(ok__) => {
                        pcmarkers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMarker<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::WORD, pwszmarkername: *mut super::WCHAR, pcchmarkernamelen: *mut super::WORD, pcnsmarkertime: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::GetMarker(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwszmarkername), core::mem::transmute_copy(&pcchmarkernamelen), core::mem::transmute_copy(&pcnsmarkertime)).into()
            }
        }
        unsafe extern "system" fn AddMarker<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszmarkername: windows_core::PCWSTR, cnsmarkertime: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::AddMarker(this, core::mem::transmute(&pwszmarkername), core::mem::transmute_copy(&cnsmarkertime)).into()
            }
        }
        unsafe extern "system" fn RemoveMarker<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::RemoveMarker(this, core::mem::transmute_copy(&windex)).into()
            }
        }
        unsafe extern "system" fn GetScriptCount<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcscripts: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMHeaderInfo_Impl::GetScriptCount(this) {
                    Ok(ok__) => {
                        pcscripts.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScript<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::WORD, pwsztype: *mut super::WCHAR, pcchtypelen: *mut super::WORD, pwszcommand: *mut super::WCHAR, pcchcommandlen: *mut super::WORD, pcnsscripttime: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::GetScript(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwsztype), core::mem::transmute_copy(&pcchtypelen), core::mem::transmute_copy(&pwszcommand), core::mem::transmute_copy(&pcchcommandlen), core::mem::transmute_copy(&pcnsscripttime)).into()
            }
        }
        unsafe extern "system" fn AddScript<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztype: windows_core::PCWSTR, pwszcommand: windows_core::PCWSTR, cnsscripttime: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::AddScript(this, core::mem::transmute(&pwsztype), core::mem::transmute(&pwszcommand), core::mem::transmute_copy(&cnsscripttime)).into()
            }
        }
        unsafe extern "system" fn RemoveScript<Identity: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo_Impl::RemoveScript(this, core::mem::transmute_copy(&windex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttributeCount: GetAttributeCount::<Identity, OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Identity, OFFSET>,
            GetAttributeByName: GetAttributeByName::<Identity, OFFSET>,
            SetAttribute: SetAttribute::<Identity, OFFSET>,
            GetMarkerCount: GetMarkerCount::<Identity, OFFSET>,
            GetMarker: GetMarker::<Identity, OFFSET>,
            AddMarker: AddMarker::<Identity, OFFSET>,
            RemoveMarker: RemoveMarker::<Identity, OFFSET>,
            GetScriptCount: GetScriptCount::<Identity, OFFSET>,
            GetScript: GetScript::<Identity, OFFSET>,
            AddScript: AddScript::<Identity, OFFSET>,
            RemoveScript: RemoveScript::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMHeaderInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMHeaderInfo {}
windows_core::imp::define_interface!(IWMHeaderInfo2, IWMHeaderInfo2_Vtbl, 0x15cf9781_454e_482e_b393_85fae487a810);
impl core::ops::Deref for IWMHeaderInfo2 {
    type Target = IWMHeaderInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMHeaderInfo2, windows_core::IUnknown, IWMHeaderInfo);
impl IWMHeaderInfo2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCodecInfoCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodecInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetCodecInfo(&self, windex: super::DWORD, pcchname: *mut super::WORD, pwszname: *mut super::WCHAR, pcchdescription: *mut super::WORD, pwszdescription: *mut super::WCHAR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut super::WORD, pbcodecinfo: *mut super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodecInfo)(windows_core::Interface::as_raw(self), windex, pcchname as _, pwszname as _, pcchdescription as _, pwszdescription as _, pcodectype as _, pcbcodecinfo as _, pbcodecinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo2_Vtbl {
    pub base__: IWMHeaderInfo_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCodecInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCodecInfoCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetCodecInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::WORD, *mut super::WCHAR, *mut super::WORD, *mut super::WCHAR, *mut WMT_CODEC_INFO_TYPE, *mut super::WORD, *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetCodecInfo: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMHeaderInfo2_Impl: IWMHeaderInfo_Impl {
    fn GetCodecInfoCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetCodecInfo(&self, windex: super::DWORD, pcchname: *mut super::WORD, pwszname: *mut super::WCHAR, pcchdescription: *mut super::WORD, pwszdescription: *mut super::WCHAR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut super::WORD, pbcodecinfo: *mut super::BYTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMHeaderInfo2_Vtbl {
    pub const fn new<Identity: IWMHeaderInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCodecInfoCount<Identity: IWMHeaderInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccodecinfos: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMHeaderInfo2_Impl::GetCodecInfoCount(this) {
                    Ok(ok__) => {
                        pccodecinfos.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodecInfo<Identity: IWMHeaderInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::DWORD, pcchname: *mut super::WORD, pwszname: *mut super::WCHAR, pcchdescription: *mut super::WORD, pwszdescription: *mut super::WCHAR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut super::WORD, pbcodecinfo: *mut super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo2_Impl::GetCodecInfo(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchdescription), core::mem::transmute_copy(&pwszdescription), core::mem::transmute_copy(&pcodectype), core::mem::transmute_copy(&pcbcodecinfo), core::mem::transmute_copy(&pbcodecinfo)).into()
            }
        }
        Self {
            base__: IWMHeaderInfo_Vtbl::new::<Identity, OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Identity, OFFSET>,
            GetCodecInfo: GetCodecInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMHeaderInfo2 as windows_core::Interface>::IID || iid == &<IWMHeaderInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMHeaderInfo2 {}
windows_core::imp::define_interface!(IWMHeaderInfo3, IWMHeaderInfo3_Vtbl, 0x15cc68e3_27cc_4ecd_b222_3f5d02d80bd5);
impl core::ops::Deref for IWMHeaderInfo3 {
    type Target = IWMHeaderInfo2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMHeaderInfo3, windows_core::IUnknown, IWMHeaderInfo, IWMHeaderInfo2);
impl IWMHeaderInfo3 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttributeCountEx(&self, wstreamnum: super::WORD) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeCountEx)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttributeIndices<P1>(&self, wstreamnum: super::WORD, pwszname: P1, pwlangindex: *const super::WORD, pwindices: *mut super::WORD, pwcount: *mut super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeIndices)(windows_core::Interface::as_raw(self), wstreamnum, pwszname.param().abi(), pwlangindex, pwindices as _, pwcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttributeByIndexEx(&self, wstreamnum: super::WORD, windex: super::WORD, pwszname: windows_core::PWSTR, pwnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut super::WORD, pvalue: *mut super::BYTE, pdwdatalength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeByIndexEx)(windows_core::Interface::as_raw(self), wstreamnum, windex, pwszname, pwnamelen as _, ptype as _, pwlangindex as _, pvalue as _, pdwdatalength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ModifyAttribute(&self, wstreamnum: super::WORD, windex: super::WORD, r#type: WMT_ATTR_DATATYPE, wlangindex: super::WORD, pvalue: *const super::BYTE, dwlength: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ModifyAttribute)(windows_core::Interface::as_raw(self), wstreamnum, windex, r#type, wlangindex, pvalue, dwlength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddAttribute<P1>(&self, wstreamnum: super::WORD, pszname: P1, pwindex: *mut super::WORD, r#type: WMT_ATTR_DATATYPE, wlangindex: super::WORD, pvalue: *const super::BYTE, dwlength: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAttribute)(windows_core::Interface::as_raw(self), wstreamnum, pszname.param().abi(), pwindex as _, r#type, wlangindex, pvalue, dwlength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteAttribute(&self, wstreamnum: super::WORD, windex: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttribute)(windows_core::Interface::as_raw(self), wstreamnum, windex) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddCodecInfo<P0, P1>(&self, pwszname: P0, pwszdescription: P1, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: super::WORD, pbcodecinfo: *const super::BYTE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddCodecInfo)(windows_core::Interface::as_raw(self), pwszname.param().abi(), pwszdescription.param().abi(), codectype, cbcodecinfo, pbcodecinfo) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo3_Vtbl {
    pub base__: IWMHeaderInfo2_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAttributeCountEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttributeCountEx: usize,
    #[cfg(feature = "shared")]
    pub GetAttributeIndices: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::PCWSTR, *const super::WORD, *mut super::WORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttributeIndices: usize,
    #[cfg(feature = "shared")]
    pub GetAttributeByIndexEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD, windows_core::PWSTR, *mut super::WORD, *mut WMT_ATTR_DATATYPE, *mut super::WORD, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttributeByIndexEx: usize,
    #[cfg(feature = "shared")]
    pub ModifyAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD, WMT_ATTR_DATATYPE, super::WORD, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ModifyAttribute: usize,
    #[cfg(feature = "shared")]
    pub AddAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::PCWSTR, *mut super::WORD, WMT_ATTR_DATATYPE, super::WORD, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddAttribute: usize,
    #[cfg(feature = "shared")]
    pub DeleteAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteAttribute: usize,
    #[cfg(feature = "shared")]
    pub AddCodecInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, WMT_CODEC_INFO_TYPE, super::WORD, *const super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddCodecInfo: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMHeaderInfo3_Impl: IWMHeaderInfo2_Impl {
    fn GetAttributeCountEx(&self, wstreamnum: super::WORD) -> windows_core::Result<super::WORD>;
    fn GetAttributeIndices(&self, wstreamnum: super::WORD, pwszname: &windows_core::PCWSTR, pwlangindex: *const super::WORD, pwindices: *mut super::WORD, pwcount: *mut super::WORD) -> windows_core::Result<()>;
    fn GetAttributeByIndexEx(&self, wstreamnum: super::WORD, windex: super::WORD, pwszname: windows_core::PWSTR, pwnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut super::WORD, pvalue: *mut super::BYTE, pdwdatalength: *mut super::DWORD) -> windows_core::Result<()>;
    fn ModifyAttribute(&self, wstreamnum: super::WORD, windex: super::WORD, r#type: WMT_ATTR_DATATYPE, wlangindex: super::WORD, pvalue: *const super::BYTE, dwlength: super::DWORD) -> windows_core::Result<()>;
    fn AddAttribute(&self, wstreamnum: super::WORD, pszname: &windows_core::PCWSTR, pwindex: *mut super::WORD, r#type: WMT_ATTR_DATATYPE, wlangindex: super::WORD, pvalue: *const super::BYTE, dwlength: super::DWORD) -> windows_core::Result<()>;
    fn DeleteAttribute(&self, wstreamnum: super::WORD, windex: super::WORD) -> windows_core::Result<()>;
    fn AddCodecInfo(&self, pwszname: &windows_core::PCWSTR, pwszdescription: &windows_core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: super::WORD, pbcodecinfo: *const super::BYTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMHeaderInfo3_Vtbl {
    pub const fn new<Identity: IWMHeaderInfo3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAttributeCountEx<Identity: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pcattributes: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMHeaderInfo3_Impl::GetAttributeCountEx(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pcattributes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeIndices<Identity: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pwszname: windows_core::PCWSTR, pwlangindex: *const super::WORD, pwindices: *mut super::WORD, pwcount: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo3_Impl::GetAttributeIndices(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute(&pwszname), core::mem::transmute_copy(&pwlangindex), core::mem::transmute_copy(&pwindices), core::mem::transmute_copy(&pwcount)).into()
            }
        }
        unsafe extern "system" fn GetAttributeByIndexEx<Identity: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, windex: super::WORD, pwszname: windows_core::PWSTR, pwnamelen: *mut super::WORD, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut super::WORD, pvalue: *mut super::BYTE, pdwdatalength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo3_Impl::GetAttributeByIndexEx(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pwnamelen), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pwlangindex), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwdatalength)).into()
            }
        }
        unsafe extern "system" fn ModifyAttribute<Identity: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, windex: super::WORD, r#type: WMT_ATTR_DATATYPE, wlangindex: super::WORD, pvalue: *const super::BYTE, dwlength: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo3_Impl::ModifyAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&windex), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&wlangindex), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&dwlength)).into()
            }
        }
        unsafe extern "system" fn AddAttribute<Identity: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pszname: windows_core::PCWSTR, pwindex: *mut super::WORD, r#type: WMT_ATTR_DATATYPE, wlangindex: super::WORD, pvalue: *const super::BYTE, dwlength: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo3_Impl::AddAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&pwindex), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&wlangindex), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&dwlength)).into()
            }
        }
        unsafe extern "system" fn DeleteAttribute<Identity: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, windex: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo3_Impl::DeleteAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&windex)).into()
            }
        }
        unsafe extern "system" fn AddCodecInfo<Identity: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwszdescription: windows_core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: super::WORD, pbcodecinfo: *const super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMHeaderInfo3_Impl::AddCodecInfo(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwszdescription), core::mem::transmute_copy(&codectype), core::mem::transmute_copy(&cbcodecinfo), core::mem::transmute_copy(&pbcodecinfo)).into()
            }
        }
        Self {
            base__: IWMHeaderInfo2_Vtbl::new::<Identity, OFFSET>(),
            GetAttributeCountEx: GetAttributeCountEx::<Identity, OFFSET>,
            GetAttributeIndices: GetAttributeIndices::<Identity, OFFSET>,
            GetAttributeByIndexEx: GetAttributeByIndexEx::<Identity, OFFSET>,
            ModifyAttribute: ModifyAttribute::<Identity, OFFSET>,
            AddAttribute: AddAttribute::<Identity, OFFSET>,
            DeleteAttribute: DeleteAttribute::<Identity, OFFSET>,
            AddCodecInfo: AddCodecInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMHeaderInfo3 as windows_core::Interface>::IID || iid == &<IWMHeaderInfo as windows_core::Interface>::IID || iid == &<IWMHeaderInfo2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMHeaderInfo3 {}
windows_core::imp::define_interface!(IWMIStreamProps, IWMIStreamProps_Vtbl, 0x6816dad3_2b4b_4c8e_8149_874c3483a753);
windows_core::imp::interface_hierarchy!(IWMIStreamProps, windows_core::IUnknown);
impl IWMIStreamProps {
    #[cfg(feature = "shared")]
    pub unsafe fn GetProperty<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pszname.param().abi(), ptype as _, pvalue as _, pdwsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIStreamProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProperty: usize,
}
#[cfg(feature = "shared")]
pub trait IWMIStreamProps_Impl: windows_core::IUnknownImpl {
    fn GetProperty(&self, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMIStreamProps_Vtbl {
    pub const fn new<Identity: IWMIStreamProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: IWMIStreamProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMIStreamProps_Impl::GetProperty(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProperty: GetProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIStreamProps as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMIStreamProps {}
windows_core::imp::define_interface!(IWMImageInfo, IWMImageInfo_Vtbl, 0x9f0aa3b6_7267_4d89_88f2_ba915aa5c4c6);
windows_core::imp::interface_hierarchy!(IWMImageInfo, windows_core::IUnknown);
impl IWMImageInfo {
    #[cfg(feature = "shared")]
    pub unsafe fn GetImageCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetImage(&self, windex: super::DWORD, pcchmimetype: *mut super::WORD, pwszmimetype: *mut super::WCHAR, pcchdescription: *mut super::WORD, pwszdescription: *mut super::WCHAR, pimagetype: *mut super::WORD, pcbimagedata: *mut super::DWORD, pbimagedata: *mut super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetImage)(windows_core::Interface::as_raw(self), windex, pcchmimetype as _, pwszmimetype as _, pcchdescription as _, pwszdescription as _, pimagetype as _, pcbimagedata as _, pbimagedata as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMImageInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetImageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetImageCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::WORD, *mut super::WCHAR, *mut super::WORD, *mut super::WCHAR, *mut super::WORD, *mut super::DWORD, *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetImage: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMImageInfo_Impl: windows_core::IUnknownImpl {
    fn GetImageCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetImage(&self, windex: super::DWORD, pcchmimetype: *mut super::WORD, pwszmimetype: *mut super::WCHAR, pcchdescription: *mut super::WORD, pwszdescription: *mut super::WCHAR, pimagetype: *mut super::WORD, pcbimagedata: *mut super::DWORD, pbimagedata: *mut super::BYTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMImageInfo_Vtbl {
    pub const fn new<Identity: IWMImageInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImageCount<Identity: IWMImageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcimages: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMImageInfo_Impl::GetImageCount(this) {
                    Ok(ok__) => {
                        pcimages.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImage<Identity: IWMImageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::DWORD, pcchmimetype: *mut super::WORD, pwszmimetype: *mut super::WCHAR, pcchdescription: *mut super::WORD, pwszdescription: *mut super::WCHAR, pimagetype: *mut super::WORD, pcbimagedata: *mut super::DWORD, pbimagedata: *mut super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMImageInfo_Impl::GetImage(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pcchmimetype), core::mem::transmute_copy(&pwszmimetype), core::mem::transmute_copy(&pcchdescription), core::mem::transmute_copy(&pwszdescription), core::mem::transmute_copy(&pimagetype), core::mem::transmute_copy(&pcbimagedata), core::mem::transmute_copy(&pbimagedata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetImageCount: GetImageCount::<Identity, OFFSET>,
            GetImage: GetImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMImageInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMImageInfo {}
windows_core::imp::define_interface!(IWMIndexer, IWMIndexer_Vtbl, 0x6d7cdc71_9888_11d3_8edc_00c04f6109cf);
windows_core::imp::interface_hierarchy!(IWMIndexer, windows_core::IUnknown);
impl IWMIndexer {
    #[cfg(feature = "winnt")]
    pub unsafe fn StartIndexing<P1>(&self, pwszurl: *const super::WCHAR, pcallback: P1, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartIndexing)(windows_core::Interface::as_raw(self), pwszurl, pcallback.param().abi(), pvcontext) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub StartIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    StartIndexing: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IWMIndexer_Impl: windows_core::IUnknownImpl {
    fn StartIndexing(&self, pwszurl: *const super::WCHAR, pcallback: windows_core::Ref<IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IWMIndexer_Vtbl {
    pub const fn new<Identity: IWMIndexer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartIndexing<Identity: IWMIndexer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: *const super::WCHAR, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMIndexer_Impl::StartIndexing(this, core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IWMIndexer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMIndexer_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartIndexing: StartIndexing::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIndexer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IWMIndexer {}
windows_core::imp::define_interface!(IWMIndexer2, IWMIndexer2_Vtbl, 0xb70f1e42_6255_4df0_a6b9_02b212d9e2bb);
impl core::ops::Deref for IWMIndexer2 {
    type Target = IWMIndexer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMIndexer2, windows_core::IUnknown, IWMIndexer);
impl IWMIndexer2 {
    #[cfg(feature = "shared")]
    pub unsafe fn Configure(&self, wstreamnum: super::WORD, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const core::ffi::c_void, pvindextype: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), wstreamnum, nindexertype, pvinterval, pvindextype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer2_Vtbl {
    pub base__: IWMIndexer_Vtbl,
    #[cfg(feature = "shared")]
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, WMT_INDEXER_TYPE, *const core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Configure: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMIndexer2_Impl: IWMIndexer_Impl {
    fn Configure(&self, wstreamnum: super::WORD, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const core::ffi::c_void, pvindextype: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMIndexer2_Vtbl {
    pub const fn new<Identity: IWMIndexer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Configure<Identity: IWMIndexer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const core::ffi::c_void, pvindextype: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMIndexer2_Impl::Configure(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&nindexertype), core::mem::transmute_copy(&pvinterval), core::mem::transmute_copy(&pvindextype)).into()
            }
        }
        Self { base__: IWMIndexer_Vtbl::new::<Identity, OFFSET>(), Configure: Configure::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIndexer2 as windows_core::Interface>::IID || iid == &<IWMIndexer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMIndexer2 {}
windows_core::imp::define_interface!(IWMInputMediaProps, IWMInputMediaProps_Vtbl, 0x96406bd5_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMInputMediaProps {
    type Target = IWMMediaProps;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMInputMediaProps, windows_core::IUnknown, IWMMediaProps);
impl IWMInputMediaProps {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetConnectionName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConnectionName)(windows_core::Interface::as_raw(self), pwszname as _, pcchname as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetGroupName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGroupName)(windows_core::Interface::as_raw(self), pwszname as _, pcchname as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMInputMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetConnectionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetConnectionName: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetGroupName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetGroupName: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMInputMediaProps_Impl: IWMMediaProps_Impl {
    fn GetConnectionName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::Result<()>;
    fn GetGroupName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMInputMediaProps_Vtbl {
    pub const fn new<Identity: IWMInputMediaProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectionName<Identity: IWMInputMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMInputMediaProps_Impl::GetConnectionName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        unsafe extern "system" fn GetGroupName<Identity: IWMInputMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMInputMediaProps_Impl::GetGroupName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, OFFSET>(),
            GetConnectionName: GetConnectionName::<Identity, OFFSET>,
            GetGroupName: GetGroupName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMInputMediaProps as windows_core::Interface>::IID || iid == &<IWMMediaProps as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMInputMediaProps {}
windows_core::imp::define_interface!(IWMLanguageList, IWMLanguageList_Vtbl, 0xdf683f00_2d49_4d8e_92b7_fb19f6a0dc57);
windows_core::imp::interface_hierarchy!(IWMLanguageList, windows_core::IUnknown);
impl IWMLanguageList {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLanguageCount(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetLanguageDetails(&self, windex: super::WORD, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLanguageDetails)(windows_core::Interface::as_raw(self), windex, pwszlanguagestring as _, pcchlanguagestringlength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddLanguageByRFC1766String<P0>(&self, pwszlanguagestring: P0) -> windows_core::Result<super::WORD>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddLanguageByRFC1766String)(windows_core::Interface::as_raw(self), pwszlanguagestring.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLanguageList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLanguageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLanguageCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetLanguageDetails: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetLanguageDetails: usize,
    #[cfg(feature = "shared")]
    pub AddLanguageByRFC1766String: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddLanguageByRFC1766String: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMLanguageList_Impl: windows_core::IUnknownImpl {
    fn GetLanguageCount(&self) -> windows_core::Result<super::WORD>;
    fn GetLanguageDetails(&self, windex: super::WORD, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::Result<()>;
    fn AddLanguageByRFC1766String(&self, pwszlanguagestring: &windows_core::PCWSTR) -> windows_core::Result<super::WORD>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMLanguageList_Vtbl {
    pub const fn new<Identity: IWMLanguageList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLanguageCount<Identity: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcount: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMLanguageList_Impl::GetLanguageCount(this) {
                    Ok(ok__) => {
                        pwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLanguageDetails<Identity: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: super::WORD, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMLanguageList_Impl::GetLanguageDetails(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwszlanguagestring), core::mem::transmute_copy(&pcchlanguagestringlength)).into()
            }
        }
        unsafe extern "system" fn AddLanguageByRFC1766String<Identity: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlanguagestring: windows_core::PCWSTR, pwindex: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMLanguageList_Impl::AddLanguageByRFC1766String(this, core::mem::transmute(&pwszlanguagestring)) {
                    Ok(ok__) => {
                        pwindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Identity, OFFSET>,
            GetLanguageDetails: GetLanguageDetails::<Identity, OFFSET>,
            AddLanguageByRFC1766String: AddLanguageByRFC1766String::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMLanguageList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMLanguageList {}
windows_core::imp::define_interface!(IWMLicenseBackup, IWMLicenseBackup_Vtbl, 0x05e5ac9f_3fb6_4508_bb43_a4067ba1ebe8);
windows_core::imp::interface_hierarchy!(IWMLicenseBackup, windows_core::IUnknown);
impl IWMLicenseBackup {
    #[cfg(feature = "shared")]
    pub unsafe fn BackupLicenses<P1>(&self, dwflags: super::DWORD, pcallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).BackupLicenses)(windows_core::Interface::as_raw(self), dwflags, pcallback.param().abi()) }
    }
    pub unsafe fn CancelLicenseBackup(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelLicenseBackup)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseBackup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub BackupLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    BackupLicenses: usize,
    pub CancelLicenseBackup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMLicenseBackup_Impl: windows_core::IUnknownImpl {
    fn BackupLicenses(&self, dwflags: super::DWORD, pcallback: windows_core::Ref<IWMStatusCallback>) -> windows_core::Result<()>;
    fn CancelLicenseBackup(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMLicenseBackup_Vtbl {
    pub const fn new<Identity: IWMLicenseBackup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BackupLicenses<Identity: IWMLicenseBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMLicenseBackup_Impl::BackupLicenses(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn CancelLicenseBackup<Identity: IWMLicenseBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMLicenseBackup_Impl::CancelLicenseBackup(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BackupLicenses: BackupLicenses::<Identity, OFFSET>,
            CancelLicenseBackup: CancelLicenseBackup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMLicenseBackup as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMLicenseBackup {}
windows_core::imp::define_interface!(IWMLicenseRestore, IWMLicenseRestore_Vtbl, 0xc70b6334_a22e_4efb_a245_15e65a004a13);
windows_core::imp::interface_hierarchy!(IWMLicenseRestore, windows_core::IUnknown);
impl IWMLicenseRestore {
    #[cfg(feature = "shared")]
    pub unsafe fn RestoreLicenses<P1>(&self, dwflags: super::DWORD, pcallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RestoreLicenses)(windows_core::Interface::as_raw(self), dwflags, pcallback.param().abi()) }
    }
    pub unsafe fn CancelLicenseRestore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelLicenseRestore)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRestore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub RestoreLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RestoreLicenses: usize,
    pub CancelLicenseRestore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMLicenseRestore_Impl: windows_core::IUnknownImpl {
    fn RestoreLicenses(&self, dwflags: super::DWORD, pcallback: windows_core::Ref<IWMStatusCallback>) -> windows_core::Result<()>;
    fn CancelLicenseRestore(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMLicenseRestore_Vtbl {
    pub const fn new<Identity: IWMLicenseRestore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RestoreLicenses<Identity: IWMLicenseRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMLicenseRestore_Impl::RestoreLicenses(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn CancelLicenseRestore<Identity: IWMLicenseRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMLicenseRestore_Impl::CancelLicenseRestore(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RestoreLicenses: RestoreLicenses::<Identity, OFFSET>,
            CancelLicenseRestore: CancelLicenseRestore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMLicenseRestore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMLicenseRestore {}
windows_core::imp::define_interface!(IWMLicenseRevocationAgent, IWMLicenseRevocationAgent_Vtbl, 0x6967f2c9_4e26_4b57_8894_799880f7ac7b);
windows_core::imp::interface_hierarchy!(IWMLicenseRevocationAgent, windows_core::IUnknown);
impl IWMLicenseRevocationAgent {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLRBChallenge(&self, pmachineid: *const super::BYTE, dwmachineidlength: super::DWORD, pchallenge: *const super::BYTE, dwchallengelength: super::DWORD, pchallengeoutput: *mut super::BYTE, pdwchallengeoutputlength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLRBChallenge)(windows_core::Interface::as_raw(self), pmachineid, dwmachineidlength, pchallenge, dwchallengelength, pchallengeoutput as _, pdwchallengeoutputlength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ProcessLRB(&self, psignedlrb: *const super::BYTE, dwsignedlrblength: super::DWORD, psignedack: *mut super::BYTE, pdwsignedacklength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessLRB)(windows_core::Interface::as_raw(self), psignedlrb, dwsignedlrblength, psignedack as _, pdwsignedacklength as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRevocationAgent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLRBChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, *const super::BYTE, super::DWORD, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLRBChallenge: usize,
    #[cfg(feature = "shared")]
    pub ProcessLRB: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ProcessLRB: usize,
}
#[cfg(feature = "shared")]
pub trait IWMLicenseRevocationAgent_Impl: windows_core::IUnknownImpl {
    fn GetLRBChallenge(&self, pmachineid: *const super::BYTE, dwmachineidlength: super::DWORD, pchallenge: *const super::BYTE, dwchallengelength: super::DWORD, pchallengeoutput: *mut super::BYTE, pdwchallengeoutputlength: *mut super::DWORD) -> windows_core::Result<()>;
    fn ProcessLRB(&self, psignedlrb: *const super::BYTE, dwsignedlrblength: super::DWORD, psignedack: *mut super::BYTE, pdwsignedacklength: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMLicenseRevocationAgent_Vtbl {
    pub const fn new<Identity: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLRBChallenge<Identity: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmachineid: *const super::BYTE, dwmachineidlength: super::DWORD, pchallenge: *const super::BYTE, dwchallengelength: super::DWORD, pchallengeoutput: *mut super::BYTE, pdwchallengeoutputlength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMLicenseRevocationAgent_Impl::GetLRBChallenge(this, core::mem::transmute_copy(&pmachineid), core::mem::transmute_copy(&dwmachineidlength), core::mem::transmute_copy(&pchallenge), core::mem::transmute_copy(&dwchallengelength), core::mem::transmute_copy(&pchallengeoutput), core::mem::transmute_copy(&pdwchallengeoutputlength)).into()
            }
        }
        unsafe extern "system" fn ProcessLRB<Identity: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignedlrb: *const super::BYTE, dwsignedlrblength: super::DWORD, psignedack: *mut super::BYTE, pdwsignedacklength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMLicenseRevocationAgent_Impl::ProcessLRB(this, core::mem::transmute_copy(&psignedlrb), core::mem::transmute_copy(&dwsignedlrblength), core::mem::transmute_copy(&psignedack), core::mem::transmute_copy(&pdwsignedacklength)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLRBChallenge: GetLRBChallenge::<Identity, OFFSET>,
            ProcessLRB: ProcessLRB::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMLicenseRevocationAgent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMLicenseRevocationAgent {}
windows_core::imp::define_interface!(IWMMediaProps, IWMMediaProps_Vtbl, 0x96406bce_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMMediaProps, windows_core::IUnknown);
impl IWMMediaProps {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMediaType)(windows_core::Interface::as_raw(self), ptype, pcbtype as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMediaType)(windows_core::Interface::as_raw(self), ptype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMediaProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WM_MEDIA_TYPE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMediaType: usize,
    #[cfg(feature = "shared")]
    pub SetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *const WM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMediaType: usize,
}
#[cfg(feature = "shared")]
pub trait IWMMediaProps_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMMediaProps_Vtbl {
    pub const fn new<Identity: IWMMediaProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMMediaProps_Impl::GetType(this) {
                    Ok(ok__) => {
                        pguidtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMediaType<Identity: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMediaProps_Impl::GetMediaType(this, core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pcbtype)).into()
            }
        }
        unsafe extern "system" fn SetMediaType<Identity: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMediaProps_Impl::SetMediaType(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetMediaType: GetMediaType::<Identity, OFFSET>,
            SetMediaType: SetMediaType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMediaProps as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMMediaProps {}
windows_core::imp::define_interface!(IWMMetadataEditor, IWMMetadataEditor_Vtbl, 0x96406bd9_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMMetadataEditor, windows_core::IUnknown);
impl IWMMetadataEditor {
    #[cfg(feature = "winnt")]
    pub unsafe fn Open(&self, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszfilename) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Flush(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Open: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IWMMetadataEditor_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pwszfilename: *const super::WCHAR) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IWMMetadataEditor_Vtbl {
    pub const fn new<Identity: IWMMetadataEditor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMetadataEditor_Impl::Open(this, core::mem::transmute_copy(&pwszfilename)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMetadataEditor_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMetadataEditor_Impl::Flush(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMetadataEditor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IWMMetadataEditor {}
windows_core::imp::define_interface!(IWMMetadataEditor2, IWMMetadataEditor2_Vtbl, 0x203cffe3_2e18_4fdf_b59d_6e71530534cf);
impl core::ops::Deref for IWMMetadataEditor2 {
    type Target = IWMMetadataEditor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMMetadataEditor2, windows_core::IUnknown, IWMMetadataEditor);
impl IWMMetadataEditor2 {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn OpenEx(&self, pwszfilename: *const super::WCHAR, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenEx)(windows_core::Interface::as_raw(self), pwszfilename, dwdesiredaccess, dwsharemode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor2_Vtbl {
    pub base__: IWMMetadataEditor_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub OpenEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    OpenEx: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMMetadataEditor2_Impl: IWMMetadataEditor_Impl {
    fn OpenEx(&self, pwszfilename: *const super::WCHAR, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMMetadataEditor2_Vtbl {
    pub const fn new<Identity: IWMMetadataEditor2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenEx<Identity: IWMMetadataEditor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: *const super::WCHAR, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMetadataEditor2_Impl::OpenEx(this, core::mem::transmute_copy(&pwszfilename), core::mem::transmute_copy(&dwdesiredaccess), core::mem::transmute_copy(&dwsharemode)).into()
            }
        }
        Self { base__: IWMMetadataEditor_Vtbl::new::<Identity, OFFSET>(), OpenEx: OpenEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMetadataEditor2 as windows_core::Interface>::IID || iid == &<IWMMetadataEditor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMMetadataEditor2 {}
windows_core::imp::define_interface!(IWMMutualExclusion, IWMMutualExclusion_Vtbl, 0x96406bde_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMMutualExclusion {
    type Target = IWMStreamList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMMutualExclusion, windows_core::IUnknown, IWMStreamList);
impl IWMMutualExclusion {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, guidtype: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), guidtype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion_Vtbl {
    pub base__: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMMutualExclusion_Impl: IWMStreamList_Impl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetType(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMMutualExclusion_Vtbl {
    pub const fn new<Identity: IWMMutualExclusion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IWMMutualExclusion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMMutualExclusion_Impl::GetType(this) {
                    Ok(ok__) => {
                        pguidtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IWMMutualExclusion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion_Impl::SetType(this, core::mem::transmute_copy(&guidtype)).into()
            }
        }
        Self { base__: IWMStreamList_Vtbl::new::<Identity, OFFSET>(), GetType: GetType::<Identity, OFFSET>, SetType: SetType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMutualExclusion as windows_core::Interface>::IID || iid == &<IWMStreamList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMMutualExclusion {}
windows_core::imp::define_interface!(IWMMutualExclusion2, IWMMutualExclusion2_Vtbl, 0x0302b57d_89d1_4ba2_85c9_166f2c53eb91);
impl core::ops::Deref for IWMMutualExclusion2 {
    type Target = IWMMutualExclusion;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMMutualExclusion2, windows_core::IUnknown, IWMStreamList, IWMMutualExclusion);
impl IWMMutualExclusion2 {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), pwszname as _, pcchname as _) }
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), pwszname.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRecordCount(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecordCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddRecord(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRecord)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveRecord(&self, wrecordnumber: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveRecord)(windows_core::Interface::as_raw(self), wrecordnumber) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetRecordName(&self, wrecordnumber: super::WORD, pwszrecordname: *mut super::WCHAR, pcchrecordname: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRecordName)(windows_core::Interface::as_raw(self), wrecordnumber, pwszrecordname as _, pcchrecordname as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRecordName<P1>(&self, wrecordnumber: super::WORD, pwszrecordname: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecordName)(windows_core::Interface::as_raw(self), wrecordnumber, pwszrecordname.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamsForRecord(&self, wrecordnumber: super::WORD, pwstreamnumarray: *mut super::WORD, pcstreams: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamsForRecord)(windows_core::Interface::as_raw(self), wrecordnumber, pwstreamnumarray as _, pcstreams as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddStreamForRecord(&self, wrecordnumber: super::WORD, wstreamnumber: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddStreamForRecord)(windows_core::Interface::as_raw(self), wrecordnumber, wstreamnumber) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveStreamForRecord(&self, wrecordnumber: super::WORD, wstreamnumber: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveStreamForRecord)(windows_core::Interface::as_raw(self), wrecordnumber, wstreamnumber) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion2_Vtbl {
    pub base__: IWMMutualExclusion_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetName: usize,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetRecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRecordCount: usize,
    pub AddRecord: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RemoveRecord: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveRecord: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetRecordName: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetRecordName: usize,
    #[cfg(feature = "shared")]
    pub SetRecordName: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRecordName: usize,
    #[cfg(feature = "shared")]
    pub GetStreamsForRecord: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamsForRecord: usize,
    #[cfg(feature = "shared")]
    pub AddStreamForRecord: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddStreamForRecord: usize,
    #[cfg(feature = "shared")]
    pub RemoveStreamForRecord: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveStreamForRecord: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMMutualExclusion2_Impl: IWMMutualExclusion_Impl {
    fn GetName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::Result<()>;
    fn SetName(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRecordCount(&self) -> windows_core::Result<super::WORD>;
    fn AddRecord(&self) -> windows_core::Result<()>;
    fn RemoveRecord(&self, wrecordnumber: super::WORD) -> windows_core::Result<()>;
    fn GetRecordName(&self, wrecordnumber: super::WORD, pwszrecordname: *mut super::WCHAR, pcchrecordname: *mut super::WORD) -> windows_core::Result<()>;
    fn SetRecordName(&self, wrecordnumber: super::WORD, pwszrecordname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStreamsForRecord(&self, wrecordnumber: super::WORD, pwstreamnumarray: *mut super::WORD, pcstreams: *mut super::WORD) -> windows_core::Result<()>;
    fn AddStreamForRecord(&self, wrecordnumber: super::WORD, wstreamnumber: super::WORD) -> windows_core::Result<()>;
    fn RemoveStreamForRecord(&self, wrecordnumber: super::WORD, wstreamnumber: super::WORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMMutualExclusion2_Vtbl {
    pub const fn new<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        unsafe extern "system" fn SetName<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::SetName(this, core::mem::transmute(&pwszname)).into()
            }
        }
        unsafe extern "system" fn GetRecordCount<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwrecordcount: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMMutualExclusion2_Impl::GetRecordCount(this) {
                    Ok(ok__) => {
                        pwrecordcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddRecord<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::AddRecord(this).into()
            }
        }
        unsafe extern "system" fn RemoveRecord<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::RemoveRecord(this, core::mem::transmute_copy(&wrecordnumber)).into()
            }
        }
        unsafe extern "system" fn GetRecordName<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: super::WORD, pwszrecordname: *mut super::WCHAR, pcchrecordname: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::GetRecordName(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute_copy(&pwszrecordname), core::mem::transmute_copy(&pcchrecordname)).into()
            }
        }
        unsafe extern "system" fn SetRecordName<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: super::WORD, pwszrecordname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::SetRecordName(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute(&pwszrecordname)).into()
            }
        }
        unsafe extern "system" fn GetStreamsForRecord<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: super::WORD, pwstreamnumarray: *mut super::WORD, pcstreams: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::GetStreamsForRecord(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute_copy(&pwstreamnumarray), core::mem::transmute_copy(&pcstreams)).into()
            }
        }
        unsafe extern "system" fn AddStreamForRecord<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: super::WORD, wstreamnumber: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::AddStreamForRecord(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute_copy(&wstreamnumber)).into()
            }
        }
        unsafe extern "system" fn RemoveStreamForRecord<Identity: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: super::WORD, wstreamnumber: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMMutualExclusion2_Impl::RemoveStreamForRecord(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute_copy(&wstreamnumber)).into()
            }
        }
        Self {
            base__: IWMMutualExclusion_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, OFFSET>,
            AddRecord: AddRecord::<Identity, OFFSET>,
            RemoveRecord: RemoveRecord::<Identity, OFFSET>,
            GetRecordName: GetRecordName::<Identity, OFFSET>,
            SetRecordName: SetRecordName::<Identity, OFFSET>,
            GetStreamsForRecord: GetStreamsForRecord::<Identity, OFFSET>,
            AddStreamForRecord: AddStreamForRecord::<Identity, OFFSET>,
            RemoveStreamForRecord: RemoveStreamForRecord::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMutualExclusion2 as windows_core::Interface>::IID || iid == &<IWMStreamList as windows_core::Interface>::IID || iid == &<IWMMutualExclusion as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMMutualExclusion2 {}
windows_core::imp::define_interface!(IWMOutputMediaProps, IWMOutputMediaProps_Vtbl, 0x96406bd7_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMOutputMediaProps {
    type Target = IWMMediaProps;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMOutputMediaProps, windows_core::IUnknown, IWMMediaProps);
impl IWMOutputMediaProps {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetStreamGroupName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamGroupName)(windows_core::Interface::as_raw(self), pwszname as _, pcchname as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetConnectionName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConnectionName)(windows_core::Interface::as_raw(self), pwszname as _, pcchname as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMOutputMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetStreamGroupName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetStreamGroupName: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetConnectionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetConnectionName: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMOutputMediaProps_Impl: IWMMediaProps_Impl {
    fn GetStreamGroupName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::Result<()>;
    fn GetConnectionName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMOutputMediaProps_Vtbl {
    pub const fn new<Identity: IWMOutputMediaProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamGroupName<Identity: IWMOutputMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMOutputMediaProps_Impl::GetStreamGroupName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        unsafe extern "system" fn GetConnectionName<Identity: IWMOutputMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: *mut super::WCHAR, pcchname: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMOutputMediaProps_Impl::GetConnectionName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, OFFSET>(),
            GetStreamGroupName: GetStreamGroupName::<Identity, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMOutputMediaProps as windows_core::Interface>::IID || iid == &<IWMMediaProps as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMOutputMediaProps {}
windows_core::imp::define_interface!(IWMPacketSize, IWMPacketSize_Vtbl, 0xcdfb97ab_188f_40b3_b643_5b7903975c59);
windows_core::imp::interface_hierarchy!(IWMPacketSize, windows_core::IUnknown);
impl IWMPacketSize {
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxPacketSize(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxPacketSize)(windows_core::Interface::as_raw(self), dwmaxpacketsize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetMaxPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxPacketSize: usize,
    #[cfg(feature = "shared")]
    pub SetMaxPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMaxPacketSize: usize,
}
#[cfg(feature = "shared")]
pub trait IWMPacketSize_Impl: windows_core::IUnknownImpl {
    fn GetMaxPacketSize(&self) -> windows_core::Result<super::DWORD>;
    fn SetMaxPacketSize(&self, dwmaxpacketsize: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMPacketSize_Vtbl {
    pub const fn new<Identity: IWMPacketSize_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaxPacketSize<Identity: IWMPacketSize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxpacketsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPacketSize_Impl::GetMaxPacketSize(this) {
                    Ok(ok__) => {
                        pdwmaxpacketsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxPacketSize<Identity: IWMPacketSize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxpacketsize: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPacketSize_Impl::SetMaxPacketSize(this, core::mem::transmute_copy(&dwmaxpacketsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxPacketSize: GetMaxPacketSize::<Identity, OFFSET>,
            SetMaxPacketSize: SetMaxPacketSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPacketSize as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMPacketSize {}
windows_core::imp::define_interface!(IWMPacketSize2, IWMPacketSize2_Vtbl, 0x8bfc2b9e_b646_4233_a877_1c6a079669dc);
impl core::ops::Deref for IWMPacketSize2 {
    type Target = IWMPacketSize;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPacketSize2, windows_core::IUnknown, IWMPacketSize);
impl IWMPacketSize2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetMinPacketSize(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMinPacketSize(&self, dwminpacketsize: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinPacketSize)(windows_core::Interface::as_raw(self), dwminpacketsize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize2_Vtbl {
    pub base__: IWMPacketSize_Vtbl,
    #[cfg(feature = "shared")]
    pub GetMinPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMinPacketSize: usize,
    #[cfg(feature = "shared")]
    pub SetMinPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMinPacketSize: usize,
}
#[cfg(feature = "shared")]
pub trait IWMPacketSize2_Impl: IWMPacketSize_Impl {
    fn GetMinPacketSize(&self) -> windows_core::Result<super::DWORD>;
    fn SetMinPacketSize(&self, dwminpacketsize: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMPacketSize2_Vtbl {
    pub const fn new<Identity: IWMPacketSize2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMinPacketSize<Identity: IWMPacketSize2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminpacketsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPacketSize2_Impl::GetMinPacketSize(this) {
                    Ok(ok__) => {
                        pdwminpacketsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinPacketSize<Identity: IWMPacketSize2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminpacketsize: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPacketSize2_Impl::SetMinPacketSize(this, core::mem::transmute_copy(&dwminpacketsize)).into()
            }
        }
        Self {
            base__: IWMPacketSize_Vtbl::new::<Identity, OFFSET>(),
            GetMinPacketSize: GetMinPacketSize::<Identity, OFFSET>,
            SetMinPacketSize: SetMinPacketSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPacketSize2 as windows_core::Interface>::IID || iid == &<IWMPacketSize as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMPacketSize2 {}
windows_core::imp::define_interface!(IWMPlayerHook, IWMPlayerHook_Vtbl, 0xe5b7ca9a_0f1c_4f66_9002_74ec50d8b304);
windows_core::imp::interface_hierarchy!(IWMPlayerHook, windows_core::IUnknown);
impl IWMPlayerHook {
    pub unsafe fn PreDecode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreDecode)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreDecode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMPlayerHook_Impl: windows_core::IUnknownImpl {
    fn PreDecode(&self) -> windows_core::Result<()>;
}
impl IWMPlayerHook_Vtbl {
    pub const fn new<Identity: IWMPlayerHook_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreDecode<Identity: IWMPlayerHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPlayerHook_Impl::PreDecode(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PreDecode: PreDecode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPlayerHook as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMPlayerHook {}
windows_core::imp::define_interface!(IWMProfile, IWMProfile_Vtbl, 0x96406bdb_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMProfile, windows_core::IUnknown);
impl IWMProfile {
    pub unsafe fn GetVersion(&self) -> windows_core::Result<WMT_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), pwszname as _, pcchname as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetName(&self, pwszname: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), pwszname) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetDescription(&self, pwszdescription: *mut super::WCHAR, pcchdescription: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), pwszdescription as _, pcchdescription as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetDescription(&self, pwszdescription: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), pwszdescription) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStream(&self, dwstreamindex: super::DWORD) -> windows_core::Result<IWMStreamConfig> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: super::WORD) -> windows_core::Result<IWMStreamConfig> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamByNumber)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveStream<P0>(&self, pconfig: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMStreamConfig>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), pconfig.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveStreamByNumber)(windows_core::Interface::as_raw(self), wstreamnum) }
    }
    pub unsafe fn AddStream<P0>(&self, pconfig: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMStreamConfig>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), pconfig.param().abi()) }
    }
    pub unsafe fn ReconfigStream<P0>(&self, pconfig: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMStreamConfig>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReconfigStream)(windows_core::Interface::as_raw(self), pconfig.param().abi()) }
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const windows_core::GUID) -> windows_core::Result<IWMStreamConfig> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNewStream)(windows_core::Interface::as_raw(self), guidstreamtype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMutualExclusionCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMutualExclusionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: super::DWORD) -> windows_core::Result<IWMMutualExclusion> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMutualExclusion)(windows_core::Interface::as_raw(self), dwmeindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveMutualExclusion<P0>(&self, pme: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMMutualExclusion>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveMutualExclusion)(windows_core::Interface::as_raw(self), pme.param().abi()) }
    }
    pub unsafe fn AddMutualExclusion<P0>(&self, pme: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMMutualExclusion>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMutualExclusion)(windows_core::Interface::as_raw(self), pme.param().abi()) }
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> windows_core::Result<IWMMutualExclusion> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNewMutualExclusion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_VERSION) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetName: usize,
    #[cfg(feature = "winnt")]
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetName: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetDescription: usize,
    #[cfg(feature = "winnt")]
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetDescription: usize,
    #[cfg(feature = "shared")]
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamCount: usize,
    #[cfg(feature = "shared")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStream: usize,
    #[cfg(feature = "shared")]
    pub GetStreamByNumber: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamByNumber: usize,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RemoveStreamByNumber: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveStreamByNumber: usize,
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReconfigStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNewStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetMutualExclusionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMutualExclusionCount: usize,
    #[cfg(feature = "shared")]
    pub GetMutualExclusion: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMutualExclusion: usize,
    pub RemoveMutualExclusion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddMutualExclusion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNewMutualExclusion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMProfile_Impl: windows_core::IUnknownImpl {
    fn GetVersion(&self) -> windows_core::Result<WMT_VERSION>;
    fn GetName(&self, pwszname: *mut super::WCHAR, pcchname: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetName(&self, pwszname: *const super::WCHAR) -> windows_core::Result<()>;
    fn GetDescription(&self, pwszdescription: *mut super::WCHAR, pcchdescription: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetDescription(&self, pwszdescription: *const super::WCHAR) -> windows_core::Result<()>;
    fn GetStreamCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetStream(&self, dwstreamindex: super::DWORD) -> windows_core::Result<IWMStreamConfig>;
    fn GetStreamByNumber(&self, wstreamnum: super::WORD) -> windows_core::Result<IWMStreamConfig>;
    fn RemoveStream(&self, pconfig: windows_core::Ref<IWMStreamConfig>) -> windows_core::Result<()>;
    fn RemoveStreamByNumber(&self, wstreamnum: super::WORD) -> windows_core::Result<()>;
    fn AddStream(&self, pconfig: windows_core::Ref<IWMStreamConfig>) -> windows_core::Result<()>;
    fn ReconfigStream(&self, pconfig: windows_core::Ref<IWMStreamConfig>) -> windows_core::Result<()>;
    fn CreateNewStream(&self, guidstreamtype: *const windows_core::GUID) -> windows_core::Result<IWMStreamConfig>;
    fn GetMutualExclusionCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetMutualExclusion(&self, dwmeindex: super::DWORD) -> windows_core::Result<IWMMutualExclusion>;
    fn RemoveMutualExclusion(&self, pme: windows_core::Ref<IWMMutualExclusion>) -> windows_core::Result<()>;
    fn AddMutualExclusion(&self, pme: windows_core::Ref<IWMMutualExclusion>) -> windows_core::Result<()>;
    fn CreateNewMutualExclusion(&self) -> windows_core::Result<IWMMutualExclusion>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMProfile_Vtbl {
    pub const fn new<Identity: IWMProfile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVersion<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        pdwversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: *mut super::WCHAR, pcchname: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        unsafe extern "system" fn SetName<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::SetName(this, core::mem::transmute_copy(&pwszname)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdescription: *mut super::WCHAR, pcchdescription: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::GetDescription(this, core::mem::transmute_copy(&pwszdescription), core::mem::transmute_copy(&pcchdescription)).into()
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdescription: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::SetDescription(this, core::mem::transmute_copy(&pwszdescription)).into()
            }
        }
        unsafe extern "system" fn GetStreamCount<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcstreams: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile_Impl::GetStreamCount(this) {
                    Ok(ok__) => {
                        pcstreams.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, ppconfig: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile_Impl::GetStream(this, core::mem::transmute_copy(&dwstreamindex)) {
                    Ok(ok__) => {
                        ppconfig.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamByNumber<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, ppconfig: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile_Impl::GetStreamByNumber(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        ppconfig.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfig: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::RemoveStream(this, core::mem::transmute_copy(&pconfig)).into()
            }
        }
        unsafe extern "system" fn RemoveStreamByNumber<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::RemoveStreamByNumber(this, core::mem::transmute_copy(&wstreamnum)).into()
            }
        }
        unsafe extern "system" fn AddStream<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfig: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::AddStream(this, core::mem::transmute_copy(&pconfig)).into()
            }
        }
        unsafe extern "system" fn ReconfigStream<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfig: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::ReconfigStream(this, core::mem::transmute_copy(&pconfig)).into()
            }
        }
        unsafe extern "system" fn CreateNewStream<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidstreamtype: *const windows_core::GUID, ppconfig: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile_Impl::CreateNewStream(this, core::mem::transmute_copy(&guidstreamtype)) {
                    Ok(ok__) => {
                        ppconfig.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMutualExclusionCount<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcme: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile_Impl::GetMutualExclusionCount(this) {
                    Ok(ok__) => {
                        pcme.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMutualExclusion<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmeindex: super::DWORD, ppme: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile_Impl::GetMutualExclusion(this, core::mem::transmute_copy(&dwmeindex)) {
                    Ok(ok__) => {
                        ppme.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveMutualExclusion<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pme: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::RemoveMutualExclusion(this, core::mem::transmute_copy(&pme)).into()
            }
        }
        unsafe extern "system" fn AddMutualExclusion<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pme: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile_Impl::AddMutualExclusion(this, core::mem::transmute_copy(&pme)).into()
            }
        }
        unsafe extern "system" fn CreateNewMutualExclusion<Identity: IWMProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppme: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile_Impl::CreateNewMutualExclusion(this) {
                    Ok(ok__) => {
                        ppme.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            GetStreamByNumber: GetStreamByNumber::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
            RemoveStreamByNumber: RemoveStreamByNumber::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            ReconfigStream: ReconfigStream::<Identity, OFFSET>,
            CreateNewStream: CreateNewStream::<Identity, OFFSET>,
            GetMutualExclusionCount: GetMutualExclusionCount::<Identity, OFFSET>,
            GetMutualExclusion: GetMutualExclusion::<Identity, OFFSET>,
            RemoveMutualExclusion: RemoveMutualExclusion::<Identity, OFFSET>,
            AddMutualExclusion: AddMutualExclusion::<Identity, OFFSET>,
            CreateNewMutualExclusion: CreateNewMutualExclusion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfile as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMProfile {}
windows_core::imp::define_interface!(IWMProfile2, IWMProfile2_Vtbl, 0x07e72d33_d94e_4be7_8843_60ae5ff7e5f5);
impl core::ops::Deref for IWMProfile2 {
    type Target = IWMProfile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfile2, windows_core::IUnknown, IWMProfile);
impl IWMProfile2 {
    pub unsafe fn GetProfileID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProfileID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile2_Vtbl {
    pub base__: IWMProfile_Vtbl,
    pub GetProfileID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMProfile2_Impl: IWMProfile_Impl {
    fn GetProfileID(&self) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMProfile2_Vtbl {
    pub const fn new<Identity: IWMProfile2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProfileID<Identity: IWMProfile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile2_Impl::GetProfileID(this) {
                    Ok(ok__) => {
                        pguidid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWMProfile_Vtbl::new::<Identity, OFFSET>(), GetProfileID: GetProfileID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfile2 as windows_core::Interface>::IID || iid == &<IWMProfile as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMProfile2 {}
windows_core::imp::define_interface!(IWMProfile3, IWMProfile3_Vtbl, 0x00ef96cc_a461_4546_8bcd_c9a28f0e06f5);
impl core::ops::Deref for IWMProfile3 {
    type Target = IWMProfile2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfile3, windows_core::IUnknown, IWMProfile, IWMProfile2);
impl IWMProfile3 {
    pub unsafe fn GetStorageFormat(&self) -> windows_core::Result<WMT_STORAGE_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorageFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStorageFormat)(windows_core::Interface::as_raw(self), nstorageformat) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBandwidthSharingCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBandwidthSharingCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBandwidthSharing(&self, dwbsindex: super::DWORD) -> windows_core::Result<IWMBandwidthSharing> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBandwidthSharing)(windows_core::Interface::as_raw(self), dwbsindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveBandwidthSharing<P0>(&self, pbs: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMBandwidthSharing>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveBandwidthSharing)(windows_core::Interface::as_raw(self), pbs.param().abi()) }
    }
    pub unsafe fn AddBandwidthSharing<P0>(&self, pbs: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMBandwidthSharing>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddBandwidthSharing)(windows_core::Interface::as_raw(self), pbs.param().abi()) }
    }
    pub unsafe fn CreateNewBandwidthSharing(&self) -> windows_core::Result<IWMBandwidthSharing> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNewBandwidthSharing)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStreamPrioritization(&self) -> windows_core::Result<IWMStreamPrioritization> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamPrioritization)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetStreamPrioritization<P0>(&self, psp: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMStreamPrioritization>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStreamPrioritization)(windows_core::Interface::as_raw(self), psp.param().abi()) }
    }
    pub unsafe fn RemoveStreamPrioritization(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveStreamPrioritization)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateNewStreamPrioritization(&self) -> windows_core::Result<IWMStreamPrioritization> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNewStreamPrioritization)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetExpectedPacketCount(&self, msduration: super::QWORD) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExpectedPacketCount)(windows_core::Interface::as_raw(self), msduration, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile3_Vtbl {
    pub base__: IWMProfile2_Vtbl,
    pub GetStorageFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_STORAGE_FORMAT) -> windows_core::HRESULT,
    pub SetStorageFormat: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_STORAGE_FORMAT) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetBandwidthSharingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBandwidthSharingCount: usize,
    #[cfg(feature = "shared")]
    pub GetBandwidthSharing: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBandwidthSharing: usize,
    pub RemoveBandwidthSharing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddBandwidthSharing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNewBandwidthSharing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStreamPrioritization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStreamPrioritization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveStreamPrioritization: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNewStreamPrioritization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetExpectedPacketCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetExpectedPacketCount: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMProfile3_Impl: IWMProfile2_Impl {
    fn GetStorageFormat(&self) -> windows_core::Result<WMT_STORAGE_FORMAT>;
    fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> windows_core::Result<()>;
    fn GetBandwidthSharingCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetBandwidthSharing(&self, dwbsindex: super::DWORD) -> windows_core::Result<IWMBandwidthSharing>;
    fn RemoveBandwidthSharing(&self, pbs: windows_core::Ref<IWMBandwidthSharing>) -> windows_core::Result<()>;
    fn AddBandwidthSharing(&self, pbs: windows_core::Ref<IWMBandwidthSharing>) -> windows_core::Result<()>;
    fn CreateNewBandwidthSharing(&self) -> windows_core::Result<IWMBandwidthSharing>;
    fn GetStreamPrioritization(&self) -> windows_core::Result<IWMStreamPrioritization>;
    fn SetStreamPrioritization(&self, psp: windows_core::Ref<IWMStreamPrioritization>) -> windows_core::Result<()>;
    fn RemoveStreamPrioritization(&self) -> windows_core::Result<()>;
    fn CreateNewStreamPrioritization(&self) -> windows_core::Result<IWMStreamPrioritization>;
    fn GetExpectedPacketCount(&self, msduration: super::QWORD) -> windows_core::Result<super::QWORD>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMProfile3_Vtbl {
    pub const fn new<Identity: IWMProfile3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStorageFormat<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile3_Impl::GetStorageFormat(this) {
                    Ok(ok__) => {
                        pnstorageformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStorageFormat<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile3_Impl::SetStorageFormat(this, core::mem::transmute_copy(&nstorageformat)).into()
            }
        }
        unsafe extern "system" fn GetBandwidthSharingCount<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbs: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile3_Impl::GetBandwidthSharingCount(this) {
                    Ok(ok__) => {
                        pcbs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBandwidthSharing<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbsindex: super::DWORD, ppbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile3_Impl::GetBandwidthSharing(this, core::mem::transmute_copy(&dwbsindex)) {
                    Ok(ok__) => {
                        ppbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveBandwidthSharing<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile3_Impl::RemoveBandwidthSharing(this, core::mem::transmute_copy(&pbs)).into()
            }
        }
        unsafe extern "system" fn AddBandwidthSharing<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile3_Impl::AddBandwidthSharing(this, core::mem::transmute_copy(&pbs)).into()
            }
        }
        unsafe extern "system" fn CreateNewBandwidthSharing<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile3_Impl::CreateNewBandwidthSharing(this) {
                    Ok(ok__) => {
                        ppbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamPrioritization<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile3_Impl::GetStreamPrioritization(this) {
                    Ok(ok__) => {
                        ppsp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamPrioritization<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psp: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile3_Impl::SetStreamPrioritization(this, core::mem::transmute_copy(&psp)).into()
            }
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfile3_Impl::RemoveStreamPrioritization(this).into()
            }
        }
        unsafe extern "system" fn CreateNewStreamPrioritization<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile3_Impl::CreateNewStreamPrioritization(this) {
                    Ok(ok__) => {
                        ppsp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExpectedPacketCount<Identity: IWMProfile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msduration: super::QWORD, pcpackets: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfile3_Impl::GetExpectedPacketCount(this, core::mem::transmute_copy(&msduration)) {
                    Ok(ok__) => {
                        pcpackets.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMProfile2_Vtbl::new::<Identity, OFFSET>(),
            GetStorageFormat: GetStorageFormat::<Identity, OFFSET>,
            SetStorageFormat: SetStorageFormat::<Identity, OFFSET>,
            GetBandwidthSharingCount: GetBandwidthSharingCount::<Identity, OFFSET>,
            GetBandwidthSharing: GetBandwidthSharing::<Identity, OFFSET>,
            RemoveBandwidthSharing: RemoveBandwidthSharing::<Identity, OFFSET>,
            AddBandwidthSharing: AddBandwidthSharing::<Identity, OFFSET>,
            CreateNewBandwidthSharing: CreateNewBandwidthSharing::<Identity, OFFSET>,
            GetStreamPrioritization: GetStreamPrioritization::<Identity, OFFSET>,
            SetStreamPrioritization: SetStreamPrioritization::<Identity, OFFSET>,
            RemoveStreamPrioritization: RemoveStreamPrioritization::<Identity, OFFSET>,
            CreateNewStreamPrioritization: CreateNewStreamPrioritization::<Identity, OFFSET>,
            GetExpectedPacketCount: GetExpectedPacketCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfile3 as windows_core::Interface>::IID || iid == &<IWMProfile as windows_core::Interface>::IID || iid == &<IWMProfile2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMProfile3 {}
windows_core::imp::define_interface!(IWMProfileManager, IWMProfileManager_Vtbl, 0xd16679f2_6ca0_472d_8d31_2f5d55aee155);
windows_core::imp::interface_hierarchy!(IWMProfileManager, windows_core::IUnknown);
impl IWMProfileManager {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> windows_core::Result<IWMProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEmptyProfile)(windows_core::Interface::as_raw(self), dwversion, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const windows_core::GUID) -> windows_core::Result<IWMProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadProfileByID)(windows_core::Interface::as_raw(self), guidprofile, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn LoadProfileByData(&self, pwszprofile: *const super::WCHAR) -> windows_core::Result<IWMProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadProfileByData)(windows_core::Interface::as_raw(self), pwszprofile, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SaveProfile<P0>(&self, piwmprofile: P0, pwszprofile: *const super::WCHAR, pdwlength: *mut super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveProfile)(windows_core::Interface::as_raw(self), piwmprofile.param().abi(), pwszprofile, pdwlength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSystemProfileCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemProfileCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: super::DWORD) -> windows_core::Result<IWMProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadSystemProfile)(windows_core::Interface::as_raw(self), dwprofileindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateEmptyProfile: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_VERSION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadProfileByID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub LoadProfileByData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    LoadProfileByData: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SaveProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SaveProfile: usize,
    #[cfg(feature = "shared")]
    pub GetSystemProfileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSystemProfileCount: usize,
    #[cfg(feature = "shared")]
    pub LoadSystemProfile: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LoadSystemProfile: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMProfileManager_Impl: windows_core::IUnknownImpl {
    fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> windows_core::Result<IWMProfile>;
    fn LoadProfileByID(&self, guidprofile: *const windows_core::GUID) -> windows_core::Result<IWMProfile>;
    fn LoadProfileByData(&self, pwszprofile: *const super::WCHAR) -> windows_core::Result<IWMProfile>;
    fn SaveProfile(&self, piwmprofile: windows_core::Ref<IWMProfile>, pwszprofile: *const super::WCHAR, pdwlength: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetSystemProfileCount(&self) -> windows_core::Result<super::DWORD>;
    fn LoadSystemProfile(&self, dwprofileindex: super::DWORD) -> windows_core::Result<IWMProfile>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMProfileManager_Vtbl {
    pub const fn new<Identity: IWMProfileManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEmptyProfile<Identity: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfileManager_Impl::CreateEmptyProfile(this, core::mem::transmute_copy(&dwversion)) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadProfileByID<Identity: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprofile: *const windows_core::GUID, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfileManager_Impl::LoadProfileByID(this, core::mem::transmute_copy(&guidprofile)) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadProfileByData<Identity: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprofile: *const super::WCHAR, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfileManager_Impl::LoadProfileByData(this, core::mem::transmute_copy(&pwszprofile)) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SaveProfile<Identity: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmprofile: *mut core::ffi::c_void, pwszprofile: *const super::WCHAR, pdwlength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfileManager_Impl::SaveProfile(this, core::mem::transmute_copy(&piwmprofile), core::mem::transmute_copy(&pwszprofile), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        unsafe extern "system" fn GetSystemProfileCount<Identity: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcprofiles: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfileManager_Impl::GetSystemProfileCount(this) {
                    Ok(ok__) => {
                        pcprofiles.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadSystemProfile<Identity: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofileindex: super::DWORD, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMProfileManager_Impl::LoadSystemProfile(this, core::mem::transmute_copy(&dwprofileindex)) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEmptyProfile: CreateEmptyProfile::<Identity, OFFSET>,
            LoadProfileByID: LoadProfileByID::<Identity, OFFSET>,
            LoadProfileByData: LoadProfileByData::<Identity, OFFSET>,
            SaveProfile: SaveProfile::<Identity, OFFSET>,
            GetSystemProfileCount: GetSystemProfileCount::<Identity, OFFSET>,
            LoadSystemProfile: LoadSystemProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfileManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMProfileManager {}
windows_core::imp::define_interface!(IWMProfileManager2, IWMProfileManager2_Vtbl, 0x7a924e51_73c1_494d_8019_23d37ed9b89a);
impl core::ops::Deref for IWMProfileManager2 {
    type Target = IWMProfileManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMProfileManager2, windows_core::IUnknown, IWMProfileManager);
impl IWMProfileManager2 {
    pub unsafe fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSystemProfileVersion)(windows_core::Interface::as_raw(self), pdwversion as _) }
    }
    pub unsafe fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSystemProfileVersion)(windows_core::Interface::as_raw(self), dwversion) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager2_Vtbl {
    pub base__: IWMProfileManager_Vtbl,
    pub GetSystemProfileVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_VERSION) -> windows_core::HRESULT,
    pub SetSystemProfileVersion: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_VERSION) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMProfileManager2_Impl: IWMProfileManager_Impl {
    fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> windows_core::Result<()>;
    fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMProfileManager2_Vtbl {
    pub const fn new<Identity: IWMProfileManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSystemProfileVersion<Identity: IWMProfileManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfileManager2_Impl::GetSystemProfileVersion(this, core::mem::transmute_copy(&pdwversion)).into()
            }
        }
        unsafe extern "system" fn SetSystemProfileVersion<Identity: IWMProfileManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwversion: WMT_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfileManager2_Impl::SetSystemProfileVersion(this, core::mem::transmute_copy(&dwversion)).into()
            }
        }
        Self {
            base__: IWMProfileManager_Vtbl::new::<Identity, OFFSET>(),
            GetSystemProfileVersion: GetSystemProfileVersion::<Identity, OFFSET>,
            SetSystemProfileVersion: SetSystemProfileVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfileManager2 as windows_core::Interface>::IID || iid == &<IWMProfileManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMProfileManager2 {}
windows_core::imp::define_interface!(IWMProfileManagerLanguage, IWMProfileManagerLanguage_Vtbl, 0xba4dcc78_7ee0_4ab8_b27a_dbce8bc51454);
windows_core::imp::interface_hierarchy!(IWMProfileManagerLanguage, windows_core::IUnknown);
impl IWMProfileManagerLanguage {
    #[cfg(feature = "shared")]
    pub unsafe fn GetUserLanguageID(&self, wlangid: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUserLanguageID)(windows_core::Interface::as_raw(self), wlangid as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetUserLanguageID(&self, wlangid: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUserLanguageID)(windows_core::Interface::as_raw(self), wlangid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManagerLanguage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetUserLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetUserLanguageID: usize,
    #[cfg(feature = "shared")]
    pub SetUserLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetUserLanguageID: usize,
}
#[cfg(feature = "shared")]
pub trait IWMProfileManagerLanguage_Impl: windows_core::IUnknownImpl {
    fn GetUserLanguageID(&self, wlangid: *mut super::WORD) -> windows_core::Result<()>;
    fn SetUserLanguageID(&self, wlangid: super::WORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMProfileManagerLanguage_Vtbl {
    pub const fn new<Identity: IWMProfileManagerLanguage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUserLanguageID<Identity: IWMProfileManagerLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wlangid: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfileManagerLanguage_Impl::GetUserLanguageID(this, core::mem::transmute_copy(&wlangid)).into()
            }
        }
        unsafe extern "system" fn SetUserLanguageID<Identity: IWMProfileManagerLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wlangid: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProfileManagerLanguage_Impl::SetUserLanguageID(this, core::mem::transmute_copy(&wlangid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserLanguageID: GetUserLanguageID::<Identity, OFFSET>,
            SetUserLanguageID: SetUserLanguageID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfileManagerLanguage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMProfileManagerLanguage {}
windows_core::imp::define_interface!(IWMPropertyVault, IWMPropertyVault_Vtbl, 0x72995a79_5090_42a4_9c8c_d9d0b6d34be5);
windows_core::imp::interface_hierarchy!(IWMPropertyVault, windows_core::IUnknown);
impl IWMPropertyVault {
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropertyCount(&self, pdwcount: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self), pdwcount) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropertyByName<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyByName)(windows_core::Interface::as_raw(self), pszname.param().abi(), ptype as _, pvalue as _, pdwsize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetProperty<P0>(&self, pszname: P0, ptype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, dwsize: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), pszname.param().abi(), ptype, pvalue, dwsize) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropertyByIndex(&self, dwindex: super::DWORD, pszname: windows_core::PWSTR, pdwnamelen: *mut super::DWORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyByIndex)(windows_core::Interface::as_raw(self), dwindex, pszname, pdwnamelen as _, ptype as _, pvalue as _, pdwsize as _) }
    }
    pub unsafe fn CopyPropertiesFrom<P0>(&self, piwmpropertyvault: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyPropertiesFrom)(windows_core::Interface::as_raw(self), piwmpropertyvault.param().abi()) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPropertyVault_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropertyCount: usize,
    #[cfg(feature = "shared")]
    pub GetPropertyByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropertyByName: usize,
    #[cfg(feature = "shared")]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetProperty: usize,
    #[cfg(feature = "shared")]
    pub GetPropertyByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PWSTR, *mut super::DWORD, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropertyByIndex: usize,
    pub CopyPropertiesFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMPropertyVault_Impl: windows_core::IUnknownImpl {
    fn GetPropertyCount(&self, pdwcount: *const super::DWORD) -> windows_core::Result<()>;
    fn GetPropertyByName(&self, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetProperty(&self, pszname: &windows_core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, dwsize: super::DWORD) -> windows_core::Result<()>;
    fn GetPropertyByIndex(&self, dwindex: super::DWORD, pszname: windows_core::PWSTR, pdwnamelen: *mut super::DWORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::Result<()>;
    fn CopyPropertiesFrom(&self, piwmpropertyvault: windows_core::Ref<IWMPropertyVault>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMPropertyVault_Vtbl {
    pub const fn new<Identity: IWMPropertyVault_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyCount<Identity: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPropertyVault_Impl::GetPropertyCount(this, core::mem::transmute_copy(&pdwcount)).into()
            }
        }
        unsafe extern "system" fn GetPropertyByName<Identity: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPropertyVault_Impl::GetPropertyByName(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, dwsize: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPropertyVault_Impl::SetProperty(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&dwsize)).into()
            }
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, pszname: windows_core::PWSTR, pdwnamelen: *mut super::DWORD, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPropertyVault_Impl::GetPropertyByIndex(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&pdwnamelen), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        unsafe extern "system" fn CopyPropertiesFrom<Identity: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpropertyvault: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPropertyVault_Impl::CopyPropertiesFrom(this, core::mem::transmute_copy(&piwmpropertyvault)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPropertyVault_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, OFFSET>,
            GetPropertyByName: GetPropertyByName::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, OFFSET>,
            CopyPropertiesFrom: CopyPropertiesFrom::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPropertyVault as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMPropertyVault {}
windows_core::imp::define_interface!(IWMProximityDetection, IWMProximityDetection_Vtbl, 0x6a9fd8ee_b651_4bf0_b849_7d4ece79a2b1);
windows_core::imp::interface_hierarchy!(IWMProximityDetection, windows_core::IUnknown);
impl IWMProximityDetection {
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn StartDetection<P6>(&self, pbregistrationmsg: *const super::BYTE, cbregistrationmsg: super::DWORD, pblocaladdress: *const super::BYTE, cblocaladdress: super::DWORD, dwextraportsallowed: super::DWORD, ppregistrationresponsemsg: *mut Option<super::INSSBuffer>, pcallback: P6, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P6: windows_core::Param<IWMStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartDetection)(windows_core::Interface::as_raw(self), pbregistrationmsg, cbregistrationmsg, pblocaladdress, cblocaladdress, dwextraportsallowed, core::mem::transmute(ppregistrationresponsemsg), pcallback.param().abi(), pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProximityDetection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub StartDetection: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, *const super::BYTE, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    StartDetection: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMProximityDetection_Impl: windows_core::IUnknownImpl {
    fn StartDetection(&self, pbregistrationmsg: *const super::BYTE, cbregistrationmsg: super::DWORD, pblocaladdress: *const super::BYTE, cblocaladdress: super::DWORD, dwextraportsallowed: super::DWORD, ppregistrationresponsemsg: windows_core::OutRef<super::INSSBuffer>, pcallback: windows_core::Ref<IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMProximityDetection_Vtbl {
    pub const fn new<Identity: IWMProximityDetection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartDetection<Identity: IWMProximityDetection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregistrationmsg: *const super::BYTE, cbregistrationmsg: super::DWORD, pblocaladdress: *const super::BYTE, cblocaladdress: super::DWORD, dwextraportsallowed: super::DWORD, ppregistrationresponsemsg: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMProximityDetection_Impl::StartDetection(this, core::mem::transmute_copy(&pbregistrationmsg), core::mem::transmute_copy(&cbregistrationmsg), core::mem::transmute_copy(&pblocaladdress), core::mem::transmute_copy(&cblocaladdress), core::mem::transmute_copy(&dwextraportsallowed), core::mem::transmute_copy(&ppregistrationresponsemsg), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StartDetection: StartDetection::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProximityDetection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMProximityDetection {}
windows_core::imp::define_interface!(IWMReader, IWMReader_Vtbl, 0x96406bd6_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMReader, windows_core::IUnknown);
impl IWMReader {
    #[cfg(feature = "winnt")]
    pub unsafe fn Open<P1>(&self, pwszurl: *const super::WCHAR, pcallback: P1, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMReaderCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszurl, pcallback.param().abi(), pvcontext) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputProps(&self, dwoutputnum: super::DWORD) -> windows_core::Result<IWMOutputMediaProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOutputProps<P1>(&self, dwoutputnum: super::DWORD, poutput: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMOutputMediaProps>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, poutput.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnumber: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputFormatCount)(windows_core::Interface::as_raw(self), dwoutputnumber, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputFormat(&self, dwoutputnumber: super::DWORD, dwformatnumber: super::DWORD) -> windows_core::Result<IWMOutputMediaProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputFormat)(windows_core::Interface::as_raw(self), dwoutputnumber, dwformatnumber, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Start(&self, cnsstart: super::QWORD, cnsduration: super::QWORD, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), cnsstart, cnsduration, frate, pvcontext) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Open: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputCount: usize,
    #[cfg(feature = "shared")]
    pub GetOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputProps: usize,
    #[cfg(feature = "shared")]
    pub SetOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOutputProps: usize,
    #[cfg(feature = "shared")]
    pub GetOutputFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputFormatCount: usize,
    #[cfg(feature = "shared")]
    pub GetOutputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputFormat: usize,
    #[cfg(feature = "shared")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD, super::QWORD, f32, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMReader_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pwszurl: *const super::WCHAR, pcallback: windows_core::Ref<IWMReaderCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetOutputCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetOutputProps(&self, dwoutputnum: super::DWORD) -> windows_core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&self, dwoutputnum: super::DWORD, poutput: windows_core::Ref<IWMOutputMediaProps>) -> windows_core::Result<()>;
    fn GetOutputFormatCount(&self, dwoutputnumber: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn GetOutputFormat(&self, dwoutputnumber: super::DWORD, dwformatnumber: super::DWORD) -> windows_core::Result<IWMOutputMediaProps>;
    fn Start(&self, cnsstart: super::QWORD, cnsduration: super::QWORD, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMReader_Vtbl {
    pub const fn new<Identity: IWMReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: *const super::WCHAR, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReader_Impl::Open(this, core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReader_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetOutputCount<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoutputs: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReader_Impl::GetOutputCount(this) {
                    Ok(ok__) => {
                        pcoutputs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputProps<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReader_Impl::GetOutputProps(this, core::mem::transmute_copy(&dwoutputnum)) {
                    Ok(ok__) => {
                        ppoutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutputProps<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReader_Impl::SetOutputProps(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnumber: super::DWORD, pcformats: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReader_Impl::GetOutputFormatCount(this, core::mem::transmute_copy(&dwoutputnumber)) {
                    Ok(ok__) => {
                        pcformats.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnumber: super::DWORD, dwformatnumber: super::DWORD, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReader_Impl::GetOutputFormat(this, core::mem::transmute_copy(&dwoutputnumber), core::mem::transmute_copy(&dwformatnumber)) {
                    Ok(ok__) => {
                        ppprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Start<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstart: super::QWORD, cnsduration: super::QWORD, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReader_Impl::Start(this, core::mem::transmute_copy(&cnsstart), core::mem::transmute_copy(&cnsduration), core::mem::transmute_copy(&frate), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReader_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReader_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IWMReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReader_Impl::Resume(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            GetOutputProps: GetOutputProps::<Identity, OFFSET>,
            SetOutputProps: SetOutputProps::<Identity, OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Identity, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReader {}
windows_core::imp::define_interface!(IWMReaderAccelerator, IWMReaderAccelerator_Vtbl, 0xbddc4d08_944d_4d52_a612_46c3fda07dd4);
windows_core::imp::interface_hierarchy!(IWMReaderAccelerator, windows_core::IUnknown);
impl IWMReaderAccelerator {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetCodecInterface(&self, dwoutputnum: super::DWORD, riid: *const super::IID, ppvcodecinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodecInterface)(windows_core::Interface::as_raw(self), dwoutputnum, riid, ppvcodecinterface as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Notify(&self, dwoutputnum: super::DWORD, psubtype: *const WM_MEDIA_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), dwoutputnum, psubtype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAccelerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetCodecInterface: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetCodecInterface: usize,
    #[cfg(feature = "shared")]
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const WM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Notify: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IWMReaderAccelerator_Impl: windows_core::IUnknownImpl {
    fn GetCodecInterface(&self, dwoutputnum: super::DWORD, riid: *const super::IID, ppvcodecinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Notify(&self, dwoutputnum: super::DWORD, psubtype: *const WM_MEDIA_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IWMReaderAccelerator_Vtbl {
    pub const fn new<Identity: IWMReaderAccelerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCodecInterface<Identity: IWMReaderAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, riid: *const super::IID, ppvcodecinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAccelerator_Impl::GetCodecInterface(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvcodecinterface)).into()
            }
        }
        unsafe extern "system" fn Notify<Identity: IWMReaderAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, psubtype: *const WM_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAccelerator_Impl::Notify(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&psubtype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodecInterface: GetCodecInterface::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAccelerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IWMReaderAccelerator {}
windows_core::imp::define_interface!(IWMReaderAdvanced, IWMReaderAdvanced_Vtbl, 0x96406bea_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced, windows_core::IUnknown);
impl IWMReaderAdvanced {
    pub unsafe fn SetUserProvidedClock(&self, fuserclock: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUserProvidedClock)(windows_core::Interface::as_raw(self), fuserclock.into()) }
    }
    pub unsafe fn GetUserProvidedClock(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserProvidedClock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeliverTime(&self, cnstime: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeliverTime)(windows_core::Interface::as_raw(self), cnstime) }
    }
    pub unsafe fn SetManualStreamSelection(&self, fselection: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetManualStreamSelection)(windows_core::Interface::as_raw(self), fselection.into()) }
    }
    pub unsafe fn GetManualStreamSelection(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetManualStreamSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: super::WORD, pwstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamsSelected)(windows_core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: super::WORD) -> windows_core::Result<WMT_STREAM_SELECTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelected)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReceiveSelectionCallbacks(&self, fgetcallbacks: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReceiveSelectionCallbacks)(windows_core::Interface::as_raw(self), fgetcallbacks.into()) }
    }
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReceiveSelectionCallbacks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetReceiveStreamSamples(&self, wstreamnum: super::WORD, freceivestreamsamples: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReceiveStreamSamples)(windows_core::Interface::as_raw(self), wstreamnum, freceivestreamsamples.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: super::WORD) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReceiveStreamSamples)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllocateForOutput(&self, dwoutputnum: super::DWORD, fallocate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, fallocate.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: super::DWORD) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllocateForStream(&self, wstreamnum: super::WORD, fallocate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllocateForStream)(windows_core::Interface::as_raw(self), wstreamnum, fallocate.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: super::WORD) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllocateForStream)(windows_core::Interface::as_raw(self), dwsreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), pstatistics as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientInfo)(windows_core::Interface::as_raw(self), pclientinfo) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxOutputSampleSize)(windows_core::Interface::as_raw(self), dwoutput, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: super::WORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxStreamSampleSize)(windows_core::Interface::as_raw(self), wstream, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NotifyLateDelivery)(windows_core::Interface::as_raw(self), cnslateness) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUserProvidedClock: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetUserProvidedClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub DeliverTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeliverTime: usize,
    pub SetManualStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetManualStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetStreamsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *const super::WORD, *const WMT_STREAM_SELECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetStreamsSelected: usize,
    #[cfg(feature = "shared")]
    pub GetStreamSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut WMT_STREAM_SELECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamSelected: usize,
    pub SetReceiveSelectionCallbacks: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetReceiveSelectionCallbacks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetReceiveStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetReceiveStreamSamples: usize,
    #[cfg(feature = "shared")]
    pub GetReceiveStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetReceiveStreamSamples: usize,
    #[cfg(feature = "shared")]
    pub SetAllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllocateForOutput: usize,
    #[cfg(feature = "shared")]
    pub GetAllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllocateForOutput: usize,
    #[cfg(feature = "shared")]
    pub SetAllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllocateForStream: usize,
    #[cfg(feature = "shared")]
    pub GetAllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllocateForStream: usize,
    #[cfg(feature = "shared")]
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WM_READER_STATISTICS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatistics: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub SetClientInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const WM_READER_CLIENTINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt")))]
    SetClientInfo: usize,
    #[cfg(feature = "shared")]
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxOutputSampleSize: usize,
    #[cfg(feature = "shared")]
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxStreamSampleSize: usize,
    #[cfg(feature = "shared")]
    pub NotifyLateDelivery: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    NotifyLateDelivery: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub trait IWMReaderAdvanced_Impl: windows_core::IUnknownImpl {
    fn SetUserProvidedClock(&self, fuserclock: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetUserProvidedClock(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DeliverTime(&self, cnstime: super::QWORD) -> windows_core::Result<()>;
    fn SetManualStreamSelection(&self, fselection: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetManualStreamSelection(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetStreamsSelected(&self, cstreamcount: super::WORD, pwstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION) -> windows_core::Result<()>;
    fn GetStreamSelected(&self, wstreamnum: super::WORD) -> windows_core::Result<WMT_STREAM_SELECTION>;
    fn SetReceiveSelectionCallbacks(&self, fgetcallbacks: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetReceiveSelectionCallbacks(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetReceiveStreamSamples(&self, wstreamnum: super::WORD, freceivestreamsamples: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetReceiveStreamSamples(&self, wstreamnum: super::WORD) -> windows_core::Result<windows_core::BOOL>;
    fn SetAllocateForOutput(&self, dwoutputnum: super::DWORD, fallocate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAllocateForOutput(&self, dwoutputnum: super::DWORD) -> windows_core::Result<windows_core::BOOL>;
    fn SetAllocateForStream(&self, wstreamnum: super::WORD, fallocate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAllocateForStream(&self, dwsreamnum: super::WORD) -> windows_core::Result<windows_core::BOOL>;
    fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> windows_core::Result<()>;
    fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> windows_core::Result<()>;
    fn GetMaxOutputSampleSize(&self, dwoutput: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn GetMaxStreamSampleSize(&self, wstream: super::WORD) -> windows_core::Result<super::DWORD>;
    fn NotifyLateDelivery(&self, cnslateness: super::QWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl IWMReaderAdvanced_Vtbl {
    pub const fn new<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetUserProvidedClock<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuserclock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::SetUserProvidedClock(this, core::mem::transmute_copy(&fuserclock)).into()
            }
        }
        unsafe extern "system" fn GetUserProvidedClock<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuserclock: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetUserProvidedClock(this) {
                    Ok(ok__) => {
                        pfuserclock.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeliverTime<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnstime: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::DeliverTime(this, core::mem::transmute_copy(&cnstime)).into()
            }
        }
        unsafe extern "system" fn SetManualStreamSelection<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fselection: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::SetManualStreamSelection(this, core::mem::transmute_copy(&fselection)).into()
            }
        }
        unsafe extern "system" fn GetManualStreamSelection<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfselection: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetManualStreamSelection(this) {
                    Ok(ok__) => {
                        pfselection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cstreamcount: super::WORD, pwstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::SetStreamsSelected(this, core::mem::transmute_copy(&cstreamcount), core::mem::transmute_copy(&pwstreamnumbers), core::mem::transmute_copy(&pselections)).into()
            }
        }
        unsafe extern "system" fn GetStreamSelected<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pselection: *mut WMT_STREAM_SELECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetStreamSelected(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pselection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReceiveSelectionCallbacks<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fgetcallbacks: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::SetReceiveSelectionCallbacks(this, core::mem::transmute_copy(&fgetcallbacks)).into()
            }
        }
        unsafe extern "system" fn GetReceiveSelectionCallbacks<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfgetcallbacks: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetReceiveSelectionCallbacks(this) {
                    Ok(ok__) => {
                        pfgetcallbacks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReceiveStreamSamples<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, freceivestreamsamples: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::SetReceiveStreamSamples(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&freceivestreamsamples)).into()
            }
        }
        unsafe extern "system" fn GetReceiveStreamSamples<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pfreceivestreamsamples: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetReceiveStreamSamples(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pfreceivestreamsamples.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, fallocate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::SetAllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&fallocate)).into()
            }
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pfallocate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetAllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum)) {
                    Ok(ok__) => {
                        pfallocate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, fallocate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::SetAllocateForStream(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&fallocate)).into()
            }
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsreamnum: super::WORD, pfallocate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetAllocateForStream(this, core::mem::transmute_copy(&dwsreamnum)) {
                    Ok(ok__) => {
                        pfallocate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::GetStatistics(this, core::mem::transmute_copy(&pstatistics)).into()
            }
        }
        unsafe extern "system" fn SetClientInfo<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::SetClientInfo(this, core::mem::transmute_copy(&pclientinfo)).into()
            }
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutput: super::DWORD, pcbmax: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetMaxOutputSampleSize(this, core::mem::transmute_copy(&dwoutput)) {
                    Ok(ok__) => {
                        pcbmax.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstream: super::WORD, pcbmax: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced_Impl::GetMaxStreamSampleSize(this, core::mem::transmute_copy(&wstream)) {
                    Ok(ok__) => {
                        pcbmax.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyLateDelivery<Identity: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnslateness: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced_Impl::NotifyLateDelivery(this, core::mem::transmute_copy(&cnslateness)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUserProvidedClock: SetUserProvidedClock::<Identity, OFFSET>,
            GetUserProvidedClock: GetUserProvidedClock::<Identity, OFFSET>,
            DeliverTime: DeliverTime::<Identity, OFFSET>,
            SetManualStreamSelection: SetManualStreamSelection::<Identity, OFFSET>,
            GetManualStreamSelection: GetManualStreamSelection::<Identity, OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Identity, OFFSET>,
            GetStreamSelected: GetStreamSelected::<Identity, OFFSET>,
            SetReceiveSelectionCallbacks: SetReceiveSelectionCallbacks::<Identity, OFFSET>,
            GetReceiveSelectionCallbacks: GetReceiveSelectionCallbacks::<Identity, OFFSET>,
            SetReceiveStreamSamples: SetReceiveStreamSamples::<Identity, OFFSET>,
            GetReceiveStreamSamples: GetReceiveStreamSamples::<Identity, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            SetClientInfo: SetClientInfo::<Identity, OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Identity, OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Identity, OFFSET>,
            NotifyLateDelivery: NotifyLateDelivery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReaderAdvanced {}
windows_core::imp::define_interface!(IWMReaderAdvanced2, IWMReaderAdvanced2_Vtbl, 0xae14a945_b90c_4d0d_9127_80d665f7d73e);
impl core::ops::Deref for IWMReaderAdvanced2 {
    type Target = IWMReaderAdvanced;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced2, windows_core::IUnknown, IWMReaderAdvanced);
impl IWMReaderAdvanced2 {
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPlayMode)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn GetPlayMode(&self) -> windows_core::Result<WMT_PLAY_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlayMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut super::DWORD, pcnsbuffering: *mut super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBufferProgress)(windows_core::Interface::as_raw(self), pdwpercent as _, pcnsbuffering as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut super::DWORD, pqwbytesdownloaded: *mut super::QWORD, pcnsdownload: *mut super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDownloadProgress)(windows_core::Interface::as_raw(self), pdwpercent as _, pqwbytesdownloaded as _, pcnsdownload as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSaveAsProgress(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSaveAsProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SaveFileAs(&self, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveFileAs)(windows_core::Interface::as_raw(self), pwszfilename) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: *mut super::WCHAR, pcchprotocol: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProtocolName)(windows_core::Interface::as_raw(self), pwszprotocol as _, pcchprotocol as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: super::WORD, cnsduration: super::QWORD, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartAtMarker)(windows_core::Interface::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputSetting<P1>(&self, dwoutputnum: super::DWORD, pszname: P1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetOutputSetting)(windows_core::Interface::as_raw(self), dwoutputnum, pszname.param().abi(), ptype as _, pvalue as _, pcblength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOutputSetting<P1>(&self, dwoutputnum: super::DWORD, pszname: P1, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputSetting)(windows_core::Interface::as_raw(self), dwoutputnum, pszname.param().abi(), r#type, pvalue, cblength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Preroll(&self, cnsstart: super::QWORD, cnsduration: super::QWORD, frate: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Preroll)(windows_core::Interface::as_raw(self), cnsstart, cnsduration, frate) }
    }
    pub unsafe fn SetLogClientID(&self, flogclientid: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogClientID)(windows_core::Interface::as_raw(self), flogclientid.into()) }
    }
    pub unsafe fn GetLogClientID(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLogClientID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StopBuffering(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopBuffering)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
        P1: windows_core::Param<IWMReaderCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), pcallback.param().abi(), pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced2_Vtbl {
    pub base__: IWMReaderAdvanced_Vtbl,
    pub SetPlayMode: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_PLAY_MODE) -> windows_core::HRESULT,
    pub GetPlayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_PLAY_MODE) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetBufferProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferProgress: usize,
    #[cfg(feature = "shared")]
    pub GetDownloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::QWORD, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDownloadProgress: usize,
    #[cfg(feature = "shared")]
    pub GetSaveAsProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSaveAsProgress: usize,
    #[cfg(feature = "winnt")]
    pub SaveFileAs: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SaveFileAs: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetProtocolName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetProtocolName: usize,
    #[cfg(feature = "shared")]
    pub StartAtMarker: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::QWORD, f32, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    StartAtMarker: usize,
    #[cfg(feature = "shared")]
    pub GetOutputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputSetting: usize,
    #[cfg(feature = "shared")]
    pub SetOutputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOutputSetting: usize,
    #[cfg(feature = "shared")]
    pub Preroll: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD, super::QWORD, f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Preroll: usize,
    pub SetLogClientID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLogClientID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub StopBuffering: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub OpenStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    OpenStream: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
pub trait IWMReaderAdvanced2_Impl: IWMReaderAdvanced_Impl {
    fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> windows_core::Result<()>;
    fn GetPlayMode(&self) -> windows_core::Result<WMT_PLAY_MODE>;
    fn GetBufferProgress(&self, pdwpercent: *mut super::DWORD, pcnsbuffering: *mut super::QWORD) -> windows_core::Result<()>;
    fn GetDownloadProgress(&self, pdwpercent: *mut super::DWORD, pqwbytesdownloaded: *mut super::QWORD, pcnsdownload: *mut super::QWORD) -> windows_core::Result<()>;
    fn GetSaveAsProgress(&self) -> windows_core::Result<super::DWORD>;
    fn SaveFileAs(&self, pwszfilename: *const super::WCHAR) -> windows_core::Result<()>;
    fn GetProtocolName(&self, pwszprotocol: *mut super::WCHAR, pcchprotocol: *mut super::DWORD) -> windows_core::Result<()>;
    fn StartAtMarker(&self, wmarkerindex: super::WORD, cnsduration: super::QWORD, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetOutputSetting(&self, dwoutputnum: super::DWORD, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
    fn SetOutputSetting(&self, dwoutputnum: super::DWORD, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::Result<()>;
    fn Preroll(&self, cnsstart: super::QWORD, cnsduration: super::QWORD, frate: f32) -> windows_core::Result<()>;
    fn SetLogClientID(&self, flogclientid: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLogClientID(&self) -> windows_core::Result<windows_core::BOOL>;
    fn StopBuffering(&self) -> windows_core::Result<()>;
    fn OpenStream(&self, pstream: windows_core::Ref<super::IStream>, pcallback: windows_core::Ref<IWMReaderCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl IWMReaderAdvanced2_Vtbl {
    pub const fn new<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPlayMode<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: WMT_PLAY_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::SetPlayMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn GetPlayMode<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced2_Impl::GetPlayMode(this) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBufferProgress<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpercent: *mut super::DWORD, pcnsbuffering: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::GetBufferProgress(this, core::mem::transmute_copy(&pdwpercent), core::mem::transmute_copy(&pcnsbuffering)).into()
            }
        }
        unsafe extern "system" fn GetDownloadProgress<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpercent: *mut super::DWORD, pqwbytesdownloaded: *mut super::QWORD, pcnsdownload: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::GetDownloadProgress(this, core::mem::transmute_copy(&pdwpercent), core::mem::transmute_copy(&pqwbytesdownloaded), core::mem::transmute_copy(&pcnsdownload)).into()
            }
        }
        unsafe extern "system" fn GetSaveAsProgress<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpercent: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced2_Impl::GetSaveAsProgress(this) {
                    Ok(ok__) => {
                        pdwpercent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SaveFileAs<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::SaveFileAs(this, core::mem::transmute_copy(&pwszfilename)).into()
            }
        }
        unsafe extern "system" fn GetProtocolName<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: *mut super::WCHAR, pcchprotocol: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::GetProtocolName(this, core::mem::transmute_copy(&pwszprotocol), core::mem::transmute_copy(&pcchprotocol)).into()
            }
        }
        unsafe extern "system" fn StartAtMarker<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmarkerindex: super::WORD, cnsduration: super::QWORD, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::StartAtMarker(this, core::mem::transmute_copy(&wmarkerindex), core::mem::transmute_copy(&cnsduration), core::mem::transmute_copy(&frate), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn GetOutputSetting<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::GetOutputSetting(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn SetOutputSetting<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::SetOutputSetting(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
            }
        }
        unsafe extern "system" fn Preroll<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstart: super::QWORD, cnsduration: super::QWORD, frate: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::Preroll(this, core::mem::transmute_copy(&cnsstart), core::mem::transmute_copy(&cnsduration), core::mem::transmute_copy(&frate)).into()
            }
        }
        unsafe extern "system" fn SetLogClientID<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flogclientid: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::SetLogClientID(this, core::mem::transmute_copy(&flogclientid)).into()
            }
        }
        unsafe extern "system" fn GetLogClientID<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflogclientid: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced2_Impl::GetLogClientID(this) {
                    Ok(ok__) => {
                        pflogclientid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StopBuffering<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::StopBuffering(this).into()
            }
        }
        unsafe extern "system" fn OpenStream<Identity: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced2_Impl::OpenStream(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self {
            base__: IWMReaderAdvanced_Vtbl::new::<Identity, OFFSET>(),
            SetPlayMode: SetPlayMode::<Identity, OFFSET>,
            GetPlayMode: GetPlayMode::<Identity, OFFSET>,
            GetBufferProgress: GetBufferProgress::<Identity, OFFSET>,
            GetDownloadProgress: GetDownloadProgress::<Identity, OFFSET>,
            GetSaveAsProgress: GetSaveAsProgress::<Identity, OFFSET>,
            SaveFileAs: SaveFileAs::<Identity, OFFSET>,
            GetProtocolName: GetProtocolName::<Identity, OFFSET>,
            StartAtMarker: StartAtMarker::<Identity, OFFSET>,
            GetOutputSetting: GetOutputSetting::<Identity, OFFSET>,
            SetOutputSetting: SetOutputSetting::<Identity, OFFSET>,
            Preroll: Preroll::<Identity, OFFSET>,
            SetLogClientID: SetLogClientID::<Identity, OFFSET>,
            GetLogClientID: GetLogClientID::<Identity, OFFSET>,
            StopBuffering: StopBuffering::<Identity, OFFSET>,
            OpenStream: OpenStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReaderAdvanced2 {}
windows_core::imp::define_interface!(IWMReaderAdvanced3, IWMReaderAdvanced3_Vtbl, 0x5dc0674b_f04b_4a4e_9f2a_b1afde2c8100);
impl core::ops::Deref for IWMReaderAdvanced3 {
    type Target = IWMReaderAdvanced2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced3, windows_core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2);
impl IWMReaderAdvanced3 {
    pub unsafe fn StopNetStreaming(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopNetStreaming)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn StartAtPosition(&self, wstreamnum: super::WORD, pvoffsetstart: *const core::ffi::c_void, pvduration: *const core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartAtPosition)(windows_core::Interface::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced3_Vtbl {
    pub base__: IWMReaderAdvanced2_Vtbl,
    pub StopNetStreaming: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub StartAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *const core::ffi::c_void, *const core::ffi::c_void, WMT_OFFSET_FORMAT, f32, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    StartAtPosition: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
pub trait IWMReaderAdvanced3_Impl: IWMReaderAdvanced2_Impl {
    fn StopNetStreaming(&self) -> windows_core::Result<()>;
    fn StartAtPosition(&self, wstreamnum: super::WORD, pvoffsetstart: *const core::ffi::c_void, pvduration: *const core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl IWMReaderAdvanced3_Vtbl {
    pub const fn new<Identity: IWMReaderAdvanced3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StopNetStreaming<Identity: IWMReaderAdvanced3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced3_Impl::StopNetStreaming(this).into()
            }
        }
        unsafe extern "system" fn StartAtPosition<Identity: IWMReaderAdvanced3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pvoffsetstart: *const core::ffi::c_void, pvduration: *const core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced3_Impl::StartAtPosition(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pvoffsetstart), core::mem::transmute_copy(&pvduration), core::mem::transmute_copy(&dwoffsetformat), core::mem::transmute_copy(&frate), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self {
            base__: IWMReaderAdvanced2_Vtbl::new::<Identity, OFFSET>(),
            StopNetStreaming: StopNetStreaming::<Identity, OFFSET>,
            StartAtPosition: StartAtPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced3 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReaderAdvanced3 {}
windows_core::imp::define_interface!(IWMReaderAdvanced4, IWMReaderAdvanced4_Vtbl, 0x945a76a2_12ae_4d48_bd3c_cd1d90399b85);
impl core::ops::Deref for IWMReaderAdvanced4 {
    type Target = IWMReaderAdvanced3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced4, windows_core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3);
impl IWMReaderAdvanced4 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: super::DWORD) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageCount)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetLanguage(&self, dwoutputnum: super::DWORD, wlanguage: super::WORD, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), dwoutputnum, wlanguage, pwszlanguagestring as _, pcchlanguagestringlength as _) }
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxSpeedFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsUsingFastCache(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUsingFastCache)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddLogParam<P0, P1, P2>(&self, wsznamespace: P0, wszname: P1, wszvalue: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddLogParam)(windows_core::Interface::as_raw(self), wsznamespace.param().abi(), wszname.param().abi(), wszvalue.param().abi()) }
    }
    pub unsafe fn SendLogParams(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendLogParams)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CanSaveFileAs(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanSaveFileAs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CancelSaveFileAs(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelSaveFileAs)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetURL(&self, pwszurl: *mut super::WCHAR, pcchurl: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), pwszurl as _, pcchurl as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced4_Vtbl {
    pub base__: IWMReaderAdvanced3_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLanguageCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLanguageCount: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::WORD, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetLanguage: usize,
    pub GetMaxSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsUsingFastCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub AddLogParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SendLogParams: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanSaveFileAs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CancelSaveFileAs: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetURL: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
pub trait IWMReaderAdvanced4_Impl: IWMReaderAdvanced3_Impl {
    fn GetLanguageCount(&self, dwoutputnum: super::DWORD) -> windows_core::Result<super::WORD>;
    fn GetLanguage(&self, dwoutputnum: super::DWORD, wlanguage: super::WORD, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::Result<()>;
    fn GetMaxSpeedFactor(&self) -> windows_core::Result<f64>;
    fn IsUsingFastCache(&self) -> windows_core::Result<windows_core::BOOL>;
    fn AddLogParam(&self, wsznamespace: &windows_core::PCWSTR, wszname: &windows_core::PCWSTR, wszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SendLogParams(&self) -> windows_core::Result<()>;
    fn CanSaveFileAs(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CancelSaveFileAs(&self) -> windows_core::Result<()>;
    fn GetURL(&self, pwszurl: *mut super::WCHAR, pcchurl: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl IWMReaderAdvanced4_Vtbl {
    pub const fn new<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLanguageCount<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pwlanguagecount: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced4_Impl::GetLanguageCount(this, core::mem::transmute_copy(&dwoutputnum)) {
                    Ok(ok__) => {
                        pwlanguagecount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, wlanguage: super::WORD, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced4_Impl::GetLanguage(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&wlanguage), core::mem::transmute_copy(&pwszlanguagestring), core::mem::transmute_copy(&pcchlanguagestringlength)).into()
            }
        }
        unsafe extern "system" fn GetMaxSpeedFactor<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdblfactor: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced4_Impl::GetMaxSpeedFactor(this) {
                    Ok(ok__) => {
                        pdblfactor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsUsingFastCache<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfusingfastcache: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced4_Impl::IsUsingFastCache(this) {
                    Ok(ok__) => {
                        pfusingfastcache.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddLogParam<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznamespace: windows_core::PCWSTR, wszname: windows_core::PCWSTR, wszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced4_Impl::AddLogParam(this, core::mem::transmute(&wsznamespace), core::mem::transmute(&wszname), core::mem::transmute(&wszvalue)).into()
            }
        }
        unsafe extern "system" fn SendLogParams<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced4_Impl::SendLogParams(this).into()
            }
        }
        unsafe extern "system" fn CanSaveFileAs<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcansave: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderAdvanced4_Impl::CanSaveFileAs(this) {
                    Ok(ok__) => {
                        pfcansave.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelSaveFileAs<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced4_Impl::CancelSaveFileAs(this).into()
            }
        }
        unsafe extern "system" fn GetURL<Identity: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: *mut super::WCHAR, pcchurl: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced4_Impl::GetURL(this, core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pcchurl)).into()
            }
        }
        Self {
            base__: IWMReaderAdvanced3_Vtbl::new::<Identity, OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            GetMaxSpeedFactor: GetMaxSpeedFactor::<Identity, OFFSET>,
            IsUsingFastCache: IsUsingFastCache::<Identity, OFFSET>,
            AddLogParam: AddLogParam::<Identity, OFFSET>,
            SendLogParams: SendLogParams::<Identity, OFFSET>,
            CanSaveFileAs: CanSaveFileAs::<Identity, OFFSET>,
            CancelSaveFileAs: CancelSaveFileAs::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced4 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReaderAdvanced4 {}
windows_core::imp::define_interface!(IWMReaderAdvanced5, IWMReaderAdvanced5_Vtbl, 0x24c44db0_55d1_49ae_a5cc_f13815e36363);
impl core::ops::Deref for IWMReaderAdvanced5 {
    type Target = IWMReaderAdvanced4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced5, windows_core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3, IWMReaderAdvanced4);
impl IWMReaderAdvanced5 {
    #[cfg(feature = "shared")]
    pub unsafe fn SetPlayerHook<P1>(&self, dwoutputnum: super::DWORD, phook: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMPlayerHook>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPlayerHook)(windows_core::Interface::as_raw(self), dwoutputnum, phook.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced5_Vtbl {
    pub base__: IWMReaderAdvanced4_Vtbl,
    #[cfg(feature = "shared")]
    pub SetPlayerHook: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPlayerHook: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
pub trait IWMReaderAdvanced5_Impl: IWMReaderAdvanced4_Impl {
    fn SetPlayerHook(&self, dwoutputnum: super::DWORD, phook: windows_core::Ref<IWMPlayerHook>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl IWMReaderAdvanced5_Vtbl {
    pub const fn new<Identity: IWMReaderAdvanced5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPlayerHook<Identity: IWMReaderAdvanced5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, phook: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced5_Impl::SetPlayerHook(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&phook)).into()
            }
        }
        Self { base__: IWMReaderAdvanced4_Vtbl::new::<Identity, OFFSET>(), SetPlayerHook: SetPlayerHook::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced5 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced3 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReaderAdvanced5 {}
windows_core::imp::define_interface!(IWMReaderAdvanced6, IWMReaderAdvanced6_Vtbl, 0x18a2e7f8_428f_4acd_8a00_e64639bc93de);
impl core::ops::Deref for IWMReaderAdvanced6 {
    type Target = IWMReaderAdvanced5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderAdvanced6, windows_core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3, IWMReaderAdvanced4, IWMReaderAdvanced5);
impl IWMReaderAdvanced6 {
    #[cfg(feature = "shared")]
    pub unsafe fn SetProtectStreamSamples(&self, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, dwcertificatetype: super::DWORD, dwflags: super::DWORD, pbinitializationvector: *mut super::BYTE, pcbinitializationvector: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProtectStreamSamples)(windows_core::Interface::as_raw(self), pbcertificate, cbcertificate, dwcertificatetype, dwflags, pbinitializationvector as _, pcbinitializationvector as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced6_Vtbl {
    pub base__: IWMReaderAdvanced5_Vtbl,
    #[cfg(feature = "shared")]
    pub SetProtectStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, super::DWORD, super::DWORD, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetProtectStreamSamples: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
pub trait IWMReaderAdvanced6_Impl: IWMReaderAdvanced5_Impl {
    fn SetProtectStreamSamples(&self, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, dwcertificatetype: super::DWORD, dwflags: super::DWORD, pbinitializationvector: *mut super::BYTE, pcbinitializationvector: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl IWMReaderAdvanced6_Vtbl {
    pub const fn new<Identity: IWMReaderAdvanced6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: IWMReaderAdvanced6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcertificate: *const super::BYTE, cbcertificate: super::DWORD, dwcertificatetype: super::DWORD, dwflags: super::DWORD, pbinitializationvector: *mut super::BYTE, pcbinitializationvector: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAdvanced6_Impl::SetProtectStreamSamples(this, core::mem::transmute_copy(&pbcertificate), core::mem::transmute_copy(&cbcertificate), core::mem::transmute_copy(&dwcertificatetype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbinitializationvector), core::mem::transmute_copy(&pcbinitializationvector)).into()
            }
        }
        Self { base__: IWMReaderAdvanced5_Vtbl::new::<Identity, OFFSET>(), SetProtectStreamSamples: SetProtectStreamSamples::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced6 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced3 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced4 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReaderAdvanced6 {}
windows_core::imp::define_interface!(IWMReaderAllocatorEx, IWMReaderAllocatorEx_Vtbl, 0x9f762fa7_a22e_428d_93c9_ac82f3aafe5a);
windows_core::imp::interface_hierarchy!(IWMReaderAllocatorEx, windows_core::IUnknown);
impl IWMReaderAllocatorEx {
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn AllocateForStreamEx(&self, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: *mut Option<super::INSSBuffer>, dwflags: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AllocateForStreamEx)(windows_core::Interface::as_raw(self), wstreamnum, cbbuffer, core::mem::transmute(ppbuffer), dwflags, cnssampletime, cnssampleduration, pvcontext) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn AllocateForOutputEx(&self, dwoutputnum: super::DWORD, cbbuffer: super::DWORD, ppbuffer: *mut Option<super::INSSBuffer>, dwflags: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AllocateForOutputEx)(windows_core::Interface::as_raw(self), dwoutputnum, cbbuffer, core::mem::transmute(ppbuffer), dwflags, cnssampletime, cnssampleduration, pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAllocatorEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub AllocateForStreamEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::DWORD, *mut *mut core::ffi::c_void, super::DWORD, super::QWORD, super::QWORD, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    AllocateForStreamEx: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub AllocateForOutputEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void, super::DWORD, super::QWORD, super::QWORD, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    AllocateForOutputEx: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMReaderAllocatorEx_Impl: windows_core::IUnknownImpl {
    fn AllocateForStreamEx(&self, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: windows_core::OutRef<super::INSSBuffer>, dwflags: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateForOutputEx(&self, dwoutputnum: super::DWORD, cbbuffer: super::DWORD, ppbuffer: windows_core::OutRef<super::INSSBuffer>, dwflags: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMReaderAllocatorEx_Vtbl {
    pub const fn new<Identity: IWMReaderAllocatorEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllocateForStreamEx<Identity: IWMReaderAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: *mut *mut core::ffi::c_void, dwflags: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAllocatorEx_Impl::AllocateForStreamEx(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn AllocateForOutputEx<Identity: IWMReaderAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, cbbuffer: super::DWORD, ppbuffer: *mut *mut core::ffi::c_void, dwflags: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderAllocatorEx_Impl::AllocateForOutputEx(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocateForStreamEx: AllocateForStreamEx::<Identity, OFFSET>,
            AllocateForOutputEx: AllocateForOutputEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAllocatorEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMReaderAllocatorEx {}
windows_core::imp::define_interface!(IWMReaderCallback, IWMReaderCallback_Vtbl, 0x96406bd8_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMReaderCallback {
    type Target = IWMStatusCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderCallback, windows_core::IUnknown, IWMStatusCallback);
impl IWMReaderCallback {
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn OnSample<P4>(&self, dwoutputnum: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: P4, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::INSSBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSample)(windows_core::Interface::as_raw(self), dwoutputnum, cnssampletime, cnssampleduration, dwflags, psample.param().abi(), pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallback_Vtbl {
    pub base__: IWMStatusCallback_Vtbl,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub OnSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::QWORD, super::QWORD, super::DWORD, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    OnSample: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMReaderCallback_Impl: IWMStatusCallback_Impl {
    fn OnSample(&self, dwoutputnum: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: windows_core::Ref<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMReaderCallback_Vtbl {
    pub const fn new<Identity: IWMReaderCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSample<Identity: IWMReaderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderCallback_Impl::OnSample(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&psample), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self { base__: IWMStatusCallback_Vtbl::new::<Identity, OFFSET>(), OnSample: OnSample::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderCallback as windows_core::Interface>::IID || iid == &<IWMStatusCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMReaderCallback {}
windows_core::imp::define_interface!(IWMReaderCallbackAdvanced, IWMReaderCallbackAdvanced_Vtbl, 0x96406beb_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMReaderCallbackAdvanced, windows_core::IUnknown);
impl IWMReaderCallbackAdvanced {
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn OnStreamSample<P4>(&self, wstreamnum: super::WORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: P4, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::INSSBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStreamSample)(windows_core::Interface::as_raw(self), wstreamnum, cnssampletime, cnssampleduration, dwflags, psample.param().abi(), pvcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnTime(&self, cnscurrenttime: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTime)(windows_core::Interface::as_raw(self), cnscurrenttime, pvcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnStreamSelection(&self, wstreamcount: super::WORD, pstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStreamSelection)(windows_core::Interface::as_raw(self), wstreamcount, pstreamnumbers, pselections, pvcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnOutputPropsChanged(&self, dwoutputnum: super::DWORD, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnOutputPropsChanged)(windows_core::Interface::as_raw(self), dwoutputnum, pmediatype, pvcontext) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn AllocateForStream(&self, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: *mut Option<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AllocateForStream)(windows_core::Interface::as_raw(self), wstreamnum, cbbuffer, core::mem::transmute(ppbuffer), pvcontext) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn AllocateForOutput(&self, dwoutputnum: super::DWORD, cbbuffer: super::DWORD, ppbuffer: *mut Option<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, cbbuffer, core::mem::transmute(ppbuffer), pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallbackAdvanced_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub OnStreamSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::QWORD, super::QWORD, super::DWORD, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    OnStreamSample: usize,
    #[cfg(feature = "shared")]
    pub OnTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnTime: usize,
    #[cfg(feature = "shared")]
    pub OnStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *const super::WORD, *const WMT_STREAM_SELECTION, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnStreamSelection: usize,
    #[cfg(feature = "shared")]
    pub OnOutputPropsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const WM_MEDIA_TYPE, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnOutputPropsChanged: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub AllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::DWORD, *mut *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    AllocateForStream: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub AllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    AllocateForOutput: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMReaderCallbackAdvanced_Impl: windows_core::IUnknownImpl {
    fn OnStreamSample(&self, wstreamnum: super::WORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: windows_core::Ref<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn OnTime(&self, cnscurrenttime: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn OnStreamSelection(&self, wstreamcount: super::WORD, pstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn OnOutputPropsChanged(&self, dwoutputnum: super::DWORD, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateForStream(&self, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: windows_core::OutRef<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateForOutput(&self, dwoutputnum: super::DWORD, cbbuffer: super::DWORD, ppbuffer: windows_core::OutRef<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMReaderCallbackAdvanced_Vtbl {
    pub const fn new<Identity: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStreamSample<Identity: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderCallbackAdvanced_Impl::OnStreamSample(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&psample), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn OnTime<Identity: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnscurrenttime: super::QWORD, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderCallbackAdvanced_Impl::OnTime(this, core::mem::transmute_copy(&cnscurrenttime), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn OnStreamSelection<Identity: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamcount: super::WORD, pstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderCallbackAdvanced_Impl::OnStreamSelection(this, core::mem::transmute_copy(&wstreamcount), core::mem::transmute_copy(&pstreamnumbers), core::mem::transmute_copy(&pselections), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn OnOutputPropsChanged<Identity: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderCallbackAdvanced_Impl::OnOutputPropsChanged(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&pmediatype), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn AllocateForStream<Identity: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: *mut *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderCallbackAdvanced_Impl::AllocateForStream(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn AllocateForOutput<Identity: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, cbbuffer: super::DWORD, ppbuffer: *mut *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderCallbackAdvanced_Impl::AllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStreamSample: OnStreamSample::<Identity, OFFSET>,
            OnTime: OnTime::<Identity, OFFSET>,
            OnStreamSelection: OnStreamSelection::<Identity, OFFSET>,
            OnOutputPropsChanged: OnOutputPropsChanged::<Identity, OFFSET>,
            AllocateForStream: AllocateForStream::<Identity, OFFSET>,
            AllocateForOutput: AllocateForOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderCallbackAdvanced as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMReaderCallbackAdvanced {}
windows_core::imp::define_interface!(IWMReaderNetworkConfig, IWMReaderNetworkConfig_Vtbl, 0x96406bec_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMReaderNetworkConfig, windows_core::IUnknown);
impl IWMReaderNetworkConfig {
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferingTime(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferingTime)(windows_core::Interface::as_raw(self), cnsbufferingtime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUDPPortRanges)(windows_core::Interface::as_raw(self), prangearray as _, pcranges as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetUDPPortRanges(&self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUDPPortRanges)(windows_core::Interface::as_raw(self), prangearray, cranges) }
    }
    pub unsafe fn GetProxySettings<P0>(&self, pwszprotocol: P0) -> windows_core::Result<WMT_PROXY_SETTINGS>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProxySettings)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProxySettings<P0>(&self, pwszprotocol: P0, proxysetting: WMT_PROXY_SETTINGS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProxySettings)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), proxysetting) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetProxyHostName<P0>(&self, pwszprotocol: P0, pwszhostname: *mut super::WCHAR, pcchhostname: *mut super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProxyHostName)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), pwszhostname as _, pcchhostname as _) }
    }
    pub unsafe fn SetProxyHostName<P0, P1>(&self, pwszprotocol: P0, pwszhostname: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProxyHostName)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), pwszhostname.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetProxyPort<P0>(&self, pwszprotocol: P0) -> windows_core::Result<super::DWORD>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProxyPort)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetProxyPort<P0>(&self, pwszprotocol: P0, dwport: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProxyPort)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), dwport) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetProxyExceptionList<P0>(&self, pwszprotocol: P0, pwszexceptionlist: *mut super::WCHAR, pcchexceptionlist: *mut super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProxyExceptionList)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), pwszexceptionlist as _, pcchexceptionlist as _) }
    }
    pub unsafe fn SetProxyExceptionList<P0, P1>(&self, pwszprotocol: P0, pwszexceptionlist: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProxyExceptionList)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), pwszexceptionlist.param().abi()) }
    }
    pub unsafe fn GetProxyBypassForLocal<P0>(&self, pwszprotocol: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProxyBypassForLocal)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProxyBypassForLocal<P0>(&self, pwszprotocol: P0, fbypassforlocal: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProxyBypassForLocal)(windows_core::Interface::as_raw(self), pwszprotocol.param().abi(), fbypassforlocal.into()) }
    }
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetForceRerunAutoProxyDetection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetForceRerunAutoProxyDetection(&self, fforcererundetection: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetForceRerunAutoProxyDetection)(windows_core::Interface::as_raw(self), fforcererundetection.into()) }
    }
    pub unsafe fn GetEnableMulticast(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableMulticast)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableMulticast(&self, fenablemulticast: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableMulticast)(windows_core::Interface::as_raw(self), fenablemulticast.into()) }
    }
    pub unsafe fn GetEnableHTTP(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableHTTP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableHTTP(&self, fenablehttp: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableHTTP)(windows_core::Interface::as_raw(self), fenablehttp.into()) }
    }
    pub unsafe fn GetEnableUDP(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableUDP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableUDP(&self, fenableudp: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableUDP)(windows_core::Interface::as_raw(self), fenableudp.into()) }
    }
    pub unsafe fn GetEnableTCP(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableTCP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableTCP(&self, fenabletcp: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableTCP)(windows_core::Interface::as_raw(self), fenabletcp.into()) }
    }
    pub unsafe fn ResetProtocolRollover(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetProtocolRollover)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetConnectionBandwidth(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionBandwidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConnectionBandwidth)(windows_core::Interface::as_raw(self), dwconnectionbandwidth) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetNumProtocolsSupported(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumProtocolsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: super::DWORD, pwszprotocolname: *mut super::WCHAR, pcchprotocolname: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedProtocolName)(windows_core::Interface::as_raw(self), dwprotocolnum, pwszprotocolname as _, pcchprotocolname as _) }
    }
    pub unsafe fn AddLoggingUrl<P0>(&self, pwszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddLoggingUrl)(windows_core::Interface::as_raw(self), pwszurl.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLoggingUrl(&self, dwindex: super::DWORD, pwszurl: windows_core::PWSTR, pcchurl: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLoggingUrl)(windows_core::Interface::as_raw(self), dwindex, pwszurl, pcchurl as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLoggingUrlCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLoggingUrlCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetLoggingUrlList)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetBufferingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferingTime: usize,
    #[cfg(feature = "shared")]
    pub SetBufferingTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetBufferingTime: usize,
    #[cfg(feature = "shared")]
    pub GetUDPPortRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WM_PORT_NUMBER_RANGE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetUDPPortRanges: usize,
    #[cfg(feature = "shared")]
    pub SetUDPPortRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *const WM_PORT_NUMBER_RANGE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetUDPPortRanges: usize,
    pub GetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMT_PROXY_SETTINGS) -> windows_core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WMT_PROXY_SETTINGS) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetProxyHostName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetProxyHostName: usize,
    pub SetProxyHostName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetProxyPort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProxyPort: usize,
    #[cfg(feature = "shared")]
    pub SetProxyPort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetProxyPort: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetProxyExceptionList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetProxyExceptionList: usize,
    pub SetProxyExceptionList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetProxyBypassForLocal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetProxyBypassForLocal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetForceRerunAutoProxyDetection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetForceRerunAutoProxyDetection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetEnableMulticast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnableMulticast: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetEnableHTTP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnableHTTP: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetEnableUDP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnableUDP: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetEnableTCP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnableTCP: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ResetProtocolRollover: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetConnectionBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetConnectionBandwidth: usize,
    #[cfg(feature = "shared")]
    pub SetConnectionBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetConnectionBandwidth: usize,
    #[cfg(feature = "shared")]
    pub GetNumProtocolsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNumProtocolsSupported: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetSupportedProtocolName: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetSupportedProtocolName: usize,
    pub AddLoggingUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetLoggingUrl: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PWSTR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLoggingUrl: usize,
    #[cfg(feature = "shared")]
    pub GetLoggingUrlCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLoggingUrlCount: usize,
    pub ResetLoggingUrlList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMReaderNetworkConfig_Impl: windows_core::IUnknownImpl {
    fn GetBufferingTime(&self) -> windows_core::Result<super::QWORD>;
    fn SetBufferingTime(&self, cnsbufferingtime: super::QWORD) -> windows_core::Result<()>;
    fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetUDPPortRanges(&self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: super::DWORD) -> windows_core::Result<()>;
    fn GetProxySettings(&self, pwszprotocol: &windows_core::PCWSTR) -> windows_core::Result<WMT_PROXY_SETTINGS>;
    fn SetProxySettings(&self, pwszprotocol: &windows_core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> windows_core::Result<()>;
    fn GetProxyHostName(&self, pwszprotocol: &windows_core::PCWSTR, pwszhostname: *mut super::WCHAR, pcchhostname: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetProxyHostName(&self, pwszprotocol: &windows_core::PCWSTR, pwszhostname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetProxyPort(&self, pwszprotocol: &windows_core::PCWSTR) -> windows_core::Result<super::DWORD>;
    fn SetProxyPort(&self, pwszprotocol: &windows_core::PCWSTR, dwport: super::DWORD) -> windows_core::Result<()>;
    fn GetProxyExceptionList(&self, pwszprotocol: &windows_core::PCWSTR, pwszexceptionlist: *mut super::WCHAR, pcchexceptionlist: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetProxyExceptionList(&self, pwszprotocol: &windows_core::PCWSTR, pwszexceptionlist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetProxyBypassForLocal(&self, pwszprotocol: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn SetProxyBypassForLocal(&self, pwszprotocol: &windows_core::PCWSTR, fbypassforlocal: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetForceRerunAutoProxyDetection(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetForceRerunAutoProxyDetection(&self, fforcererundetection: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetEnableMulticast(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnableMulticast(&self, fenablemulticast: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetEnableHTTP(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnableHTTP(&self, fenablehttp: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetEnableUDP(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnableUDP(&self, fenableudp: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetEnableTCP(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnableTCP(&self, fenabletcp: windows_core::BOOL) -> windows_core::Result<()>;
    fn ResetProtocolRollover(&self) -> windows_core::Result<()>;
    fn GetConnectionBandwidth(&self) -> windows_core::Result<super::DWORD>;
    fn SetConnectionBandwidth(&self, dwconnectionbandwidth: super::DWORD) -> windows_core::Result<()>;
    fn GetNumProtocolsSupported(&self) -> windows_core::Result<super::DWORD>;
    fn GetSupportedProtocolName(&self, dwprotocolnum: super::DWORD, pwszprotocolname: *mut super::WCHAR, pcchprotocolname: *mut super::DWORD) -> windows_core::Result<()>;
    fn AddLoggingUrl(&self, pwszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLoggingUrl(&self, dwindex: super::DWORD, pwszurl: windows_core::PWSTR, pcchurl: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetLoggingUrlCount(&self) -> windows_core::Result<super::DWORD>;
    fn ResetLoggingUrlList(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMReaderNetworkConfig_Vtbl {
    pub const fn new<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBufferingTime<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsbufferingtime: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetBufferingTime(this) {
                    Ok(ok__) => {
                        pcnsbufferingtime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferingTime<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsbufferingtime: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetBufferingTime(this, core::mem::transmute_copy(&cnsbufferingtime)).into()
            }
        }
        unsafe extern "system" fn GetUDPPortRanges<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::GetUDPPortRanges(this, core::mem::transmute_copy(&prangearray), core::mem::transmute_copy(&pcranges)).into()
            }
        }
        unsafe extern "system" fn SetUDPPortRanges<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetUDPPortRanges(this, core::mem::transmute_copy(&prangearray), core::mem::transmute_copy(&cranges)).into()
            }
        }
        unsafe extern "system" fn GetProxySettings<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetProxySettings(this, core::mem::transmute(&pwszprotocol)) {
                    Ok(ok__) => {
                        pproxysetting.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetProxySettings(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&proxysetting)).into()
            }
        }
        unsafe extern "system" fn GetProxyHostName<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pwszhostname: *mut super::WCHAR, pcchhostname: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::GetProxyHostName(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&pwszhostname), core::mem::transmute_copy(&pcchhostname)).into()
            }
        }
        unsafe extern "system" fn SetProxyHostName<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pwszhostname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetProxyHostName(this, core::mem::transmute(&pwszprotocol), core::mem::transmute(&pwszhostname)).into()
            }
        }
        unsafe extern "system" fn GetProxyPort<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pdwport: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetProxyPort(this, core::mem::transmute(&pwszprotocol)) {
                    Ok(ok__) => {
                        pdwport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProxyPort<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, dwport: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetProxyPort(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&dwport)).into()
            }
        }
        unsafe extern "system" fn GetProxyExceptionList<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pwszexceptionlist: *mut super::WCHAR, pcchexceptionlist: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::GetProxyExceptionList(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&pwszexceptionlist), core::mem::transmute_copy(&pcchexceptionlist)).into()
            }
        }
        unsafe extern "system" fn SetProxyExceptionList<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pwszexceptionlist: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetProxyExceptionList(this, core::mem::transmute(&pwszprotocol), core::mem::transmute(&pwszexceptionlist)).into()
            }
        }
        unsafe extern "system" fn GetProxyBypassForLocal<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pfbypassforlocal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetProxyBypassForLocal(this, core::mem::transmute(&pwszprotocol)) {
                    Ok(ok__) => {
                        pfbypassforlocal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProxyBypassForLocal<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, fbypassforlocal: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetProxyBypassForLocal(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&fbypassforlocal)).into()
            }
        }
        unsafe extern "system" fn GetForceRerunAutoProxyDetection<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfforcererundetection: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetForceRerunAutoProxyDetection(this) {
                    Ok(ok__) => {
                        pfforcererundetection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForceRerunAutoProxyDetection<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforcererundetection: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetForceRerunAutoProxyDetection(this, core::mem::transmute_copy(&fforcererundetection)).into()
            }
        }
        unsafe extern "system" fn GetEnableMulticast<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablemulticast: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetEnableMulticast(this) {
                    Ok(ok__) => {
                        pfenablemulticast.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableMulticast<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablemulticast: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetEnableMulticast(this, core::mem::transmute_copy(&fenablemulticast)).into()
            }
        }
        unsafe extern "system" fn GetEnableHTTP<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablehttp: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetEnableHTTP(this) {
                    Ok(ok__) => {
                        pfenablehttp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableHTTP<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablehttp: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetEnableHTTP(this, core::mem::transmute_copy(&fenablehttp)).into()
            }
        }
        unsafe extern "system" fn GetEnableUDP<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenableudp: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetEnableUDP(this) {
                    Ok(ok__) => {
                        pfenableudp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableUDP<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenableudp: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetEnableUDP(this, core::mem::transmute_copy(&fenableudp)).into()
            }
        }
        unsafe extern "system" fn GetEnableTCP<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabletcp: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetEnableTCP(this) {
                    Ok(ok__) => {
                        pfenabletcp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableTCP<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabletcp: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetEnableTCP(this, core::mem::transmute_copy(&fenabletcp)).into()
            }
        }
        unsafe extern "system" fn ResetProtocolRollover<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::ResetProtocolRollover(this).into()
            }
        }
        unsafe extern "system" fn GetConnectionBandwidth<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconnectionbandwidth: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetConnectionBandwidth(this) {
                    Ok(ok__) => {
                        pdwconnectionbandwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConnectionBandwidth<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnectionbandwidth: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::SetConnectionBandwidth(this, core::mem::transmute_copy(&dwconnectionbandwidth)).into()
            }
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcprotocols: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetNumProtocolsSupported(this) {
                    Ok(ok__) => {
                        pcprotocols.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedProtocolName<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprotocolnum: super::DWORD, pwszprotocolname: *mut super::WCHAR, pcchprotocolname: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::GetSupportedProtocolName(this, core::mem::transmute_copy(&dwprotocolnum), core::mem::transmute_copy(&pwszprotocolname), core::mem::transmute_copy(&pcchprotocolname)).into()
            }
        }
        unsafe extern "system" fn AddLoggingUrl<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::AddLoggingUrl(this, core::mem::transmute(&pwszurl)).into()
            }
        }
        unsafe extern "system" fn GetLoggingUrl<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, pwszurl: windows_core::PWSTR, pcchurl: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::GetLoggingUrl(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pcchurl)).into()
            }
        }
        unsafe extern "system" fn GetLoggingUrlCount<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwurlcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig_Impl::GetLoggingUrlCount(this) {
                    Ok(ok__) => {
                        pdwurlcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResetLoggingUrlList<Identity: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig_Impl::ResetLoggingUrlList(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBufferingTime: GetBufferingTime::<Identity, OFFSET>,
            SetBufferingTime: SetBufferingTime::<Identity, OFFSET>,
            GetUDPPortRanges: GetUDPPortRanges::<Identity, OFFSET>,
            SetUDPPortRanges: SetUDPPortRanges::<Identity, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, OFFSET>,
            GetProxyHostName: GetProxyHostName::<Identity, OFFSET>,
            SetProxyHostName: SetProxyHostName::<Identity, OFFSET>,
            GetProxyPort: GetProxyPort::<Identity, OFFSET>,
            SetProxyPort: SetProxyPort::<Identity, OFFSET>,
            GetProxyExceptionList: GetProxyExceptionList::<Identity, OFFSET>,
            SetProxyExceptionList: SetProxyExceptionList::<Identity, OFFSET>,
            GetProxyBypassForLocal: GetProxyBypassForLocal::<Identity, OFFSET>,
            SetProxyBypassForLocal: SetProxyBypassForLocal::<Identity, OFFSET>,
            GetForceRerunAutoProxyDetection: GetForceRerunAutoProxyDetection::<Identity, OFFSET>,
            SetForceRerunAutoProxyDetection: SetForceRerunAutoProxyDetection::<Identity, OFFSET>,
            GetEnableMulticast: GetEnableMulticast::<Identity, OFFSET>,
            SetEnableMulticast: SetEnableMulticast::<Identity, OFFSET>,
            GetEnableHTTP: GetEnableHTTP::<Identity, OFFSET>,
            SetEnableHTTP: SetEnableHTTP::<Identity, OFFSET>,
            GetEnableUDP: GetEnableUDP::<Identity, OFFSET>,
            SetEnableUDP: SetEnableUDP::<Identity, OFFSET>,
            GetEnableTCP: GetEnableTCP::<Identity, OFFSET>,
            SetEnableTCP: SetEnableTCP::<Identity, OFFSET>,
            ResetProtocolRollover: ResetProtocolRollover::<Identity, OFFSET>,
            GetConnectionBandwidth: GetConnectionBandwidth::<Identity, OFFSET>,
            SetConnectionBandwidth: SetConnectionBandwidth::<Identity, OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Identity, OFFSET>,
            GetSupportedProtocolName: GetSupportedProtocolName::<Identity, OFFSET>,
            AddLoggingUrl: AddLoggingUrl::<Identity, OFFSET>,
            GetLoggingUrl: GetLoggingUrl::<Identity, OFFSET>,
            GetLoggingUrlCount: GetLoggingUrlCount::<Identity, OFFSET>,
            ResetLoggingUrlList: ResetLoggingUrlList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderNetworkConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReaderNetworkConfig {}
windows_core::imp::define_interface!(IWMReaderNetworkConfig2, IWMReaderNetworkConfig2_Vtbl, 0xd979a853_042b_4050_8387_c939db22013f);
impl core::ops::Deref for IWMReaderNetworkConfig2 {
    type Target = IWMReaderNetworkConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMReaderNetworkConfig2, windows_core::IUnknown, IWMReaderNetworkConfig);
impl IWMReaderNetworkConfig2 {
    pub unsafe fn GetEnableContentCaching(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableContentCaching)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableContentCaching(&self, fenablecontentcaching: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableContentCaching)(windows_core::Interface::as_raw(self), fenablecontentcaching.into()) }
    }
    pub unsafe fn GetEnableFastCache(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableFastCache)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableFastCache(&self, fenablefastcache: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableFastCache)(windows_core::Interface::as_raw(self), fenablefastcache.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAcceleratedStreamingDuration(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAcceleratedStreamingDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAcceleratedStreamingDuration)(windows_core::Interface::as_raw(self), cnsaccelduration) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAutoReconnectLimit(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutoReconnectLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoReconnectLimit)(windows_core::Interface::as_raw(self), dwautoreconnectlimit) }
    }
    pub unsafe fn GetEnableResends(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableResends)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableResends(&self, fenableresends: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableResends)(windows_core::Interface::as_raw(self), fenableresends.into()) }
    }
    pub unsafe fn GetEnableThinning(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableThinning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableThinning(&self, fenablethinning: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableThinning)(windows_core::Interface::as_raw(self), fenablethinning.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxNetPacketSize(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxNetPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig2_Vtbl {
    pub base__: IWMReaderNetworkConfig_Vtbl,
    pub GetEnableContentCaching: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnableContentCaching: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetEnableFastCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnableFastCache: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetAcceleratedStreamingDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAcceleratedStreamingDuration: usize,
    #[cfg(feature = "shared")]
    pub SetAcceleratedStreamingDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAcceleratedStreamingDuration: usize,
    #[cfg(feature = "shared")]
    pub GetAutoReconnectLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAutoReconnectLimit: usize,
    #[cfg(feature = "shared")]
    pub SetAutoReconnectLimit: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAutoReconnectLimit: usize,
    pub GetEnableResends: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnableResends: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetEnableThinning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnableThinning: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetMaxNetPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxNetPacketSize: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMReaderNetworkConfig2_Impl: IWMReaderNetworkConfig_Impl {
    fn GetEnableContentCaching(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnableContentCaching(&self, fenablecontentcaching: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetEnableFastCache(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnableFastCache(&self, fenablefastcache: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAcceleratedStreamingDuration(&self) -> windows_core::Result<super::QWORD>;
    fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: super::QWORD) -> windows_core::Result<()>;
    fn GetAutoReconnectLimit(&self) -> windows_core::Result<super::DWORD>;
    fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: super::DWORD) -> windows_core::Result<()>;
    fn GetEnableResends(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnableResends(&self, fenableresends: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetEnableThinning(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnableThinning(&self, fenablethinning: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetMaxNetPacketSize(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMReaderNetworkConfig2_Vtbl {
    pub const fn new<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnableContentCaching<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablecontentcaching: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig2_Impl::GetEnableContentCaching(this) {
                    Ok(ok__) => {
                        pfenablecontentcaching.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableContentCaching<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablecontentcaching: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig2_Impl::SetEnableContentCaching(this, core::mem::transmute_copy(&fenablecontentcaching)).into()
            }
        }
        unsafe extern "system" fn GetEnableFastCache<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablefastcache: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig2_Impl::GetEnableFastCache(this) {
                    Ok(ok__) => {
                        pfenablefastcache.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableFastCache<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablefastcache: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig2_Impl::SetEnableFastCache(this, core::mem::transmute_copy(&fenablefastcache)).into()
            }
        }
        unsafe extern "system" fn GetAcceleratedStreamingDuration<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsaccelduration: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig2_Impl::GetAcceleratedStreamingDuration(this) {
                    Ok(ok__) => {
                        pcnsaccelduration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAcceleratedStreamingDuration<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsaccelduration: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig2_Impl::SetAcceleratedStreamingDuration(this, core::mem::transmute_copy(&cnsaccelduration)).into()
            }
        }
        unsafe extern "system" fn GetAutoReconnectLimit<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwautoreconnectlimit: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig2_Impl::GetAutoReconnectLimit(this) {
                    Ok(ok__) => {
                        pdwautoreconnectlimit.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoReconnectLimit<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwautoreconnectlimit: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig2_Impl::SetAutoReconnectLimit(this, core::mem::transmute_copy(&dwautoreconnectlimit)).into()
            }
        }
        unsafe extern "system" fn GetEnableResends<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenableresends: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig2_Impl::GetEnableResends(this) {
                    Ok(ok__) => {
                        pfenableresends.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableResends<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenableresends: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig2_Impl::SetEnableResends(this, core::mem::transmute_copy(&fenableresends)).into()
            }
        }
        unsafe extern "system" fn GetEnableThinning<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablethinning: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig2_Impl::GetEnableThinning(this) {
                    Ok(ok__) => {
                        pfenablethinning.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableThinning<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablethinning: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderNetworkConfig2_Impl::SetEnableThinning(this, core::mem::transmute_copy(&fenablethinning)).into()
            }
        }
        unsafe extern "system" fn GetMaxNetPacketSize<Identity: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxnetpacketsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderNetworkConfig2_Impl::GetMaxNetPacketSize(this) {
                    Ok(ok__) => {
                        pdwmaxnetpacketsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMReaderNetworkConfig_Vtbl::new::<Identity, OFFSET>(),
            GetEnableContentCaching: GetEnableContentCaching::<Identity, OFFSET>,
            SetEnableContentCaching: SetEnableContentCaching::<Identity, OFFSET>,
            GetEnableFastCache: GetEnableFastCache::<Identity, OFFSET>,
            SetEnableFastCache: SetEnableFastCache::<Identity, OFFSET>,
            GetAcceleratedStreamingDuration: GetAcceleratedStreamingDuration::<Identity, OFFSET>,
            SetAcceleratedStreamingDuration: SetAcceleratedStreamingDuration::<Identity, OFFSET>,
            GetAutoReconnectLimit: GetAutoReconnectLimit::<Identity, OFFSET>,
            SetAutoReconnectLimit: SetAutoReconnectLimit::<Identity, OFFSET>,
            GetEnableResends: GetEnableResends::<Identity, OFFSET>,
            SetEnableResends: SetEnableResends::<Identity, OFFSET>,
            GetEnableThinning: GetEnableThinning::<Identity, OFFSET>,
            SetEnableThinning: SetEnableThinning::<Identity, OFFSET>,
            GetMaxNetPacketSize: GetMaxNetPacketSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderNetworkConfig2 as windows_core::Interface>::IID || iid == &<IWMReaderNetworkConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMReaderNetworkConfig2 {}
windows_core::imp::define_interface!(IWMReaderPlaylistBurn, IWMReaderPlaylistBurn_Vtbl, 0xf28c0300_9baa_4477_a846_1744d9cbf533);
windows_core::imp::interface_hierarchy!(IWMReaderPlaylistBurn, windows_core::IUnknown);
impl IWMReaderPlaylistBurn {
    #[cfg(feature = "shared")]
    pub unsafe fn InitPlaylistBurn<P2>(&self, cfiles: super::DWORD, ppwszfilenames: *const windows_core::PCWSTR, pcallback: P2, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWMStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitPlaylistBurn)(windows_core::Interface::as_raw(self), cfiles, ppwszfilenames, pcallback.param().abi(), pvcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInitResults(&self, cfiles: super::DWORD) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInitResults)(windows_core::Interface::as_raw(self), cfiles, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndPlaylistBurn(&self, hrburnresult: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndPlaylistBurn)(windows_core::Interface::as_raw(self), hrburnresult) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderPlaylistBurn_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub InitPlaylistBurn: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::PCWSTR, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    InitPlaylistBurn: usize,
    #[cfg(feature = "shared")]
    pub GetInitResults: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInitResults: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndPlaylistBurn: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMReaderPlaylistBurn_Impl: windows_core::IUnknownImpl {
    fn InitPlaylistBurn(&self, cfiles: super::DWORD, ppwszfilenames: *const windows_core::PCWSTR, pcallback: windows_core::Ref<IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetInitResults(&self, cfiles: super::DWORD) -> windows_core::Result<windows_core::HRESULT>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn EndPlaylistBurn(&self, hrburnresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMReaderPlaylistBurn_Vtbl {
    pub const fn new<Identity: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitPlaylistBurn<Identity: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfiles: super::DWORD, ppwszfilenames: *const windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderPlaylistBurn_Impl::InitPlaylistBurn(this, core::mem::transmute_copy(&cfiles), core::mem::transmute_copy(&ppwszfilenames), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn GetInitResults<Identity: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfiles: super::DWORD, phrstati: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderPlaylistBurn_Impl::GetInitResults(this, core::mem::transmute_copy(&cfiles)) {
                    Ok(ok__) => {
                        phrstati.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderPlaylistBurn_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn EndPlaylistBurn<Identity: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrburnresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderPlaylistBurn_Impl::EndPlaylistBurn(this, core::mem::transmute_copy(&hrburnresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitPlaylistBurn: InitPlaylistBurn::<Identity, OFFSET>,
            GetInitResults: GetInitResults::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            EndPlaylistBurn: EndPlaylistBurn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderPlaylistBurn as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMReaderPlaylistBurn {}
windows_core::imp::define_interface!(IWMReaderStreamClock, IWMReaderStreamClock_Vtbl, 0x96406bed_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMReaderStreamClock, windows_core::IUnknown);
impl IWMReaderStreamClock {
    #[cfg(feature = "shared")]
    pub unsafe fn GetTime(&self, pcnsnow: *const super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), pcnsnow) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetTimer(&self, cnswhen: super::QWORD, pvparam: *const core::ffi::c_void) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetTimer)(windows_core::Interface::as_raw(self), cnswhen, pvparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn KillTimer(&self, dwtimerid: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KillTimer)(windows_core::Interface::as_raw(self), dwtimerid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderStreamClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTime: usize,
    #[cfg(feature = "shared")]
    pub SetTimer: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD, *const core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetTimer: usize,
    #[cfg(feature = "shared")]
    pub KillTimer: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    KillTimer: usize,
}
#[cfg(feature = "shared")]
pub trait IWMReaderStreamClock_Impl: windows_core::IUnknownImpl {
    fn GetTime(&self, pcnsnow: *const super::QWORD) -> windows_core::Result<()>;
    fn SetTimer(&self, cnswhen: super::QWORD, pvparam: *const core::ffi::c_void) -> windows_core::Result<super::DWORD>;
    fn KillTimer(&self, dwtimerid: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMReaderStreamClock_Vtbl {
    pub const fn new<Identity: IWMReaderStreamClock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTime<Identity: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsnow: *const super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderStreamClock_Impl::GetTime(this, core::mem::transmute_copy(&pcnsnow)).into()
            }
        }
        unsafe extern "system" fn SetTimer<Identity: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnswhen: super::QWORD, pvparam: *const core::ffi::c_void, pdwtimerid: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderStreamClock_Impl::SetTimer(this, core::mem::transmute_copy(&cnswhen), core::mem::transmute_copy(&pvparam)) {
                    Ok(ok__) => {
                        pdwtimerid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KillTimer<Identity: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimerid: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderStreamClock_Impl::KillTimer(this, core::mem::transmute_copy(&dwtimerid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, OFFSET>,
            SetTimer: SetTimer::<Identity, OFFSET>,
            KillTimer: KillTimer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderStreamClock as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMReaderStreamClock {}
windows_core::imp::define_interface!(IWMReaderTimecode, IWMReaderTimecode_Vtbl, 0xf369e2f0_e081_4fe6_8450_b810b2f410d1);
windows_core::imp::interface_hierarchy!(IWMReaderTimecode, windows_core::IUnknown);
impl IWMReaderTimecode {
    #[cfg(feature = "shared")]
    pub unsafe fn GetTimecodeRangeCount(&self, wstreamnum: super::WORD) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimecodeRangeCount)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTimecodeRangeBounds(&self, wstreamnum: super::WORD, wrangenum: super::WORD, pstarttimecode: *mut super::DWORD, pendtimecode: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTimecodeRangeBounds)(windows_core::Interface::as_raw(self), wstreamnum, wrangenum, pstarttimecode as _, pendtimecode as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTimecode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetTimecodeRangeCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTimecodeRangeCount: usize,
    #[cfg(feature = "shared")]
    pub GetTimecodeRangeBounds: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTimecodeRangeBounds: usize,
}
#[cfg(feature = "shared")]
pub trait IWMReaderTimecode_Impl: windows_core::IUnknownImpl {
    fn GetTimecodeRangeCount(&self, wstreamnum: super::WORD) -> windows_core::Result<super::WORD>;
    fn GetTimecodeRangeBounds(&self, wstreamnum: super::WORD, wrangenum: super::WORD, pstarttimecode: *mut super::DWORD, pendtimecode: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMReaderTimecode_Vtbl {
    pub const fn new<Identity: IWMReaderTimecode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTimecodeRangeCount<Identity: IWMReaderTimecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pwrangecount: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMReaderTimecode_Impl::GetTimecodeRangeCount(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pwrangecount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimecodeRangeBounds<Identity: IWMReaderTimecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, wrangenum: super::WORD, pstarttimecode: *mut super::DWORD, pendtimecode: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderTimecode_Impl::GetTimecodeRangeBounds(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&wrangenum), core::mem::transmute_copy(&pstarttimecode), core::mem::transmute_copy(&pendtimecode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTimecodeRangeCount: GetTimecodeRangeCount::<Identity, OFFSET>,
            GetTimecodeRangeBounds: GetTimecodeRangeBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderTimecode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMReaderTimecode {}
windows_core::imp::define_interface!(IWMReaderTypeNegotiation, IWMReaderTypeNegotiation_Vtbl, 0xfdbe5592_81a1_41ea_93bd_735cad1adc05);
windows_core::imp::interface_hierarchy!(IWMReaderTypeNegotiation, windows_core::IUnknown);
impl IWMReaderTypeNegotiation {
    #[cfg(feature = "shared")]
    pub unsafe fn TryOutputProps<P1>(&self, dwoutputnum: super::DWORD, poutput: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMOutputMediaProps>,
    {
        unsafe { (windows_core::Interface::vtable(self).TryOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, poutput.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTypeNegotiation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub TryOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    TryOutputProps: usize,
}
#[cfg(feature = "shared")]
pub trait IWMReaderTypeNegotiation_Impl: windows_core::IUnknownImpl {
    fn TryOutputProps(&self, dwoutputnum: super::DWORD, poutput: windows_core::Ref<IWMOutputMediaProps>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMReaderTypeNegotiation_Vtbl {
    pub const fn new<Identity: IWMReaderTypeNegotiation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TryOutputProps<Identity: IWMReaderTypeNegotiation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMReaderTypeNegotiation_Impl::TryOutputProps(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&poutput)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TryOutputProps: TryOutputProps::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderTypeNegotiation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMReaderTypeNegotiation {}
windows_core::imp::define_interface!(IWMRegisterCallback, IWMRegisterCallback_Vtbl, 0xcf4b1f99_4de2_4e49_a363_252740d99bc1);
windows_core::imp::interface_hierarchy!(IWMRegisterCallback, windows_core::IUnknown);
impl IWMRegisterCallback {
    pub unsafe fn Advise<P0>(&self, pcallback: P0, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pvcontext) }
    }
    pub unsafe fn Unadvise<P0>(&self, pcallback: P0, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisterCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMRegisterCallback_Impl: windows_core::IUnknownImpl {
    fn Advise(&self, pcallback: windows_core::Ref<IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Unadvise(&self, pcallback: windows_core::Ref<IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IWMRegisterCallback_Vtbl {
    pub const fn new<Identity: IWMRegisterCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Advise<Identity: IWMRegisterCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMRegisterCallback_Impl::Advise(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IWMRegisterCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMRegisterCallback_Impl::Unadvise(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Advise: Advise::<Identity, OFFSET>, Unadvise: Unadvise::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMRegisterCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMRegisterCallback {}
windows_core::imp::define_interface!(IWMRegisteredDevice, IWMRegisteredDevice_Vtbl, 0xa4503bec_5508_4148_97ac_bfa75760a70d);
windows_core::imp::interface_hierarchy!(IWMRegisteredDevice, windows_core::IUnknown);
impl IWMRegisteredDevice {
    #[cfg(feature = "shared")]
    pub unsafe fn GetDeviceSerialNumber(&self) -> windows_core::Result<DRM_VAL16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceSerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wmsbuffer")]
    pub unsafe fn GetDeviceCertificate(&self) -> windows_core::Result<super::INSSBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDeviceType(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttributeCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttributeByIndex(&self, dwindex: super::DWORD, pbstrname: *mut windows_core::BSTR, pbstrvalue: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeByIndex)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pbstrname), core::mem::transmute(pbstrvalue)) }
    }
    pub unsafe fn GetAttributeByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAttributeByName(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAttributeByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrvalue)) }
    }
    pub unsafe fn Approve(&self, fapprove: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Approve)(windows_core::Interface::as_raw(self), fapprove.into()) }
    }
    pub unsafe fn IsValid(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsApproved(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsApproved)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsWmdrmCompliant(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWmdrmCompliant)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOpened(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpened)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Open(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisteredDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetDeviceSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DRM_VAL16) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDeviceSerialNumber: usize,
    #[cfg(feature = "wmsbuffer")]
    pub GetDeviceCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wmsbuffer"))]
    GetDeviceCertificate: usize,
    #[cfg(feature = "shared")]
    pub GetDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDeviceType: usize,
    #[cfg(feature = "shared")]
    pub GetAttributeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttributeCount: usize,
    #[cfg(feature = "shared")]
    pub GetAttributeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttributeByIndex: usize,
    pub GetAttributeByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAttributeByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Approve: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsApproved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsWmdrmCompliant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsOpened: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMRegisteredDevice_Impl: windows_core::IUnknownImpl {
    fn GetDeviceSerialNumber(&self) -> windows_core::Result<DRM_VAL16>;
    fn GetDeviceCertificate(&self) -> windows_core::Result<super::INSSBuffer>;
    fn GetDeviceType(&self) -> windows_core::Result<super::DWORD>;
    fn GetAttributeCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetAttributeByIndex(&self, dwindex: super::DWORD, pbstrname: *mut windows_core::BSTR, pbstrvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAttributeByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetAttributeByName(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Approve(&self, fapprove: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsValid(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsApproved(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsWmdrmCompliant(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsOpened(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Open(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMRegisteredDevice_Vtbl {
    pub const fn new<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceSerialNumber<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::GetDeviceSerialNumber(this) {
                    Ok(ok__) => {
                        pserialnumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceCertificate<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcertificate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::GetDeviceCertificate(this) {
                    Ok(ok__) => {
                        ppcertificate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceType<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::GetDeviceType(this) {
                    Ok(ok__) => {
                        pdwtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeCount<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcattributes: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::GetAttributeCount(this) {
                    Ok(ok__) => {
                        pcattributes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, pbstrname: *mut *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMRegisteredDevice_Impl::GetAttributeByIndex(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrvalue)).into()
            }
        }
        unsafe extern "system" fn GetAttributeByName<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::GetAttributeByName(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttributeByName<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMRegisteredDevice_Impl::SetAttributeByName(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
            }
        }
        unsafe extern "system" fn Approve<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fapprove: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMRegisteredDevice_Impl::Approve(this, core::mem::transmute_copy(&fapprove)).into()
            }
        }
        unsafe extern "system" fn IsValid<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvalid: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::IsValid(this) {
                    Ok(ok__) => {
                        pfvalid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsApproved<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfapproved: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::IsApproved(this) {
                    Ok(ok__) => {
                        pfapproved.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsWmdrmCompliant<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcompliant: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::IsWmdrmCompliant(this) {
                    Ok(ok__) => {
                        pfcompliant.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOpened<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfopened: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMRegisteredDevice_Impl::IsOpened(this) {
                    Ok(ok__) => {
                        pfopened.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Open<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMRegisteredDevice_Impl::Open(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMRegisteredDevice_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceSerialNumber: GetDeviceSerialNumber::<Identity, OFFSET>,
            GetDeviceCertificate: GetDeviceCertificate::<Identity, OFFSET>,
            GetDeviceType: GetDeviceType::<Identity, OFFSET>,
            GetAttributeCount: GetAttributeCount::<Identity, OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Identity, OFFSET>,
            GetAttributeByName: GetAttributeByName::<Identity, OFFSET>,
            SetAttributeByName: SetAttributeByName::<Identity, OFFSET>,
            Approve: Approve::<Identity, OFFSET>,
            IsValid: IsValid::<Identity, OFFSET>,
            IsApproved: IsApproved::<Identity, OFFSET>,
            IsWmdrmCompliant: IsWmdrmCompliant::<Identity, OFFSET>,
            IsOpened: IsOpened::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMRegisteredDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMRegisteredDevice {}
windows_core::imp::define_interface!(IWMStatusCallback, IWMStatusCallback_Vtbl, 0x6d7cdc70_9888_11d3_8edc_00c04f6109cf);
windows_core::imp::interface_hierarchy!(IWMStatusCallback, windows_core::IUnknown);
impl IWMStatusCallback {
    #[cfg(feature = "shared")]
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatus)(windows_core::Interface::as_raw(self), status, hr, dwtype, pvalue, pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStatusCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub OnStatus: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_STATUS, windows_core::HRESULT, WMT_ATTR_DATATYPE, *const super::BYTE, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnStatus: usize,
}
#[cfg(feature = "shared")]
pub trait IWMStatusCallback_Impl: windows_core::IUnknownImpl {
    fn OnStatus(&self, status: WMT_STATUS, hr: windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMStatusCallback_Vtbl {
    pub const fn new<Identity: IWMStatusCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStatus<Identity: IWMStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: WMT_STATUS, hr: windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStatusCallback_Impl::OnStatus(this, core::mem::transmute_copy(&status), core::mem::transmute_copy(&hr), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnStatus: OnStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStatusCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMStatusCallback {}
windows_core::imp::define_interface!(IWMStreamConfig, IWMStreamConfig_Vtbl, 0x96406bdc_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMStreamConfig, windows_core::IUnknown);
impl IWMStreamConfig {
    pub unsafe fn GetStreamType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamNumber(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetStreamNumber(&self, wstreamnum: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamNumber)(windows_core::Interface::as_raw(self), wstreamnum) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetStreamName(&self, pwszstreamname: *mut super::WCHAR, pcchstreamname: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamName)(windows_core::Interface::as_raw(self), pwszstreamname as _, pcchstreamname as _) }
    }
    pub unsafe fn SetStreamName<P0>(&self, pwszstreamname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStreamName)(windows_core::Interface::as_raw(self), pwszstreamname.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetConnectionName(&self, pwszinputname: *mut super::WCHAR, pcchinputname: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConnectionName)(windows_core::Interface::as_raw(self), pwszinputname as _, pcchinputname as _) }
    }
    pub unsafe fn SetConnectionName<P0>(&self, pwszinputname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetConnectionName)(windows_core::Interface::as_raw(self), pwszinputname.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBitrate(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitrate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetBitrate(&self, pdwbitrate: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBitrate)(windows_core::Interface::as_raw(self), pdwbitrate) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferWindow(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferWindow)(windows_core::Interface::as_raw(self), msbufferwindow) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreamType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetStreamNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamNumber: usize,
    #[cfg(feature = "shared")]
    pub SetStreamNumber: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetStreamNumber: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetStreamName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetStreamName: usize,
    pub SetStreamName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetConnectionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetConnectionName: usize,
    pub SetConnectionName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBitrate: usize,
    #[cfg(feature = "shared")]
    pub SetBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetBitrate: usize,
    #[cfg(feature = "shared")]
    pub GetBufferWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferWindow: usize,
    #[cfg(feature = "shared")]
    pub SetBufferWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetBufferWindow: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMStreamConfig_Impl: windows_core::IUnknownImpl {
    fn GetStreamType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetStreamNumber(&self) -> windows_core::Result<super::WORD>;
    fn SetStreamNumber(&self, wstreamnum: super::WORD) -> windows_core::Result<()>;
    fn GetStreamName(&self, pwszstreamname: *mut super::WCHAR, pcchstreamname: *mut super::WORD) -> windows_core::Result<()>;
    fn SetStreamName(&self, pwszstreamname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetConnectionName(&self, pwszinputname: *mut super::WCHAR, pcchinputname: *mut super::WORD) -> windows_core::Result<()>;
    fn SetConnectionName(&self, pwszinputname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBitrate(&self) -> windows_core::Result<super::DWORD>;
    fn SetBitrate(&self, pdwbitrate: super::DWORD) -> windows_core::Result<()>;
    fn GetBufferWindow(&self) -> windows_core::Result<super::DWORD>;
    fn SetBufferWindow(&self, msbufferwindow: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMStreamConfig_Vtbl {
    pub const fn new<Identity: IWMStreamConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamType<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidstreamtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMStreamConfig_Impl::GetStreamType(this) {
                    Ok(ok__) => {
                        pguidstreamtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamNumber<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstreamnum: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMStreamConfig_Impl::GetStreamNumber(this) {
                    Ok(ok__) => {
                        pwstreamnum.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamNumber<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig_Impl::SetStreamNumber(this, core::mem::transmute_copy(&wstreamnum)).into()
            }
        }
        unsafe extern "system" fn GetStreamName<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszstreamname: *mut super::WCHAR, pcchstreamname: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig_Impl::GetStreamName(this, core::mem::transmute_copy(&pwszstreamname), core::mem::transmute_copy(&pcchstreamname)).into()
            }
        }
        unsafe extern "system" fn SetStreamName<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszstreamname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig_Impl::SetStreamName(this, core::mem::transmute(&pwszstreamname)).into()
            }
        }
        unsafe extern "system" fn GetConnectionName<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinputname: *mut super::WCHAR, pcchinputname: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig_Impl::GetConnectionName(this, core::mem::transmute_copy(&pwszinputname), core::mem::transmute_copy(&pcchinputname)).into()
            }
        }
        unsafe extern "system" fn SetConnectionName<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinputname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig_Impl::SetConnectionName(this, core::mem::transmute(&pwszinputname)).into()
            }
        }
        unsafe extern "system" fn GetBitrate<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbitrate: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMStreamConfig_Impl::GetBitrate(this) {
                    Ok(ok__) => {
                        pdwbitrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBitrate<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbitrate: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig_Impl::SetBitrate(this, core::mem::transmute_copy(&pdwbitrate)).into()
            }
        }
        unsafe extern "system" fn GetBufferWindow<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsbufferwindow: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMStreamConfig_Impl::GetBufferWindow(this) {
                    Ok(ok__) => {
                        pmsbufferwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferWindow<Identity: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msbufferwindow: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig_Impl::SetBufferWindow(this, core::mem::transmute_copy(&msbufferwindow)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamType: GetStreamType::<Identity, OFFSET>,
            GetStreamNumber: GetStreamNumber::<Identity, OFFSET>,
            SetStreamNumber: SetStreamNumber::<Identity, OFFSET>,
            GetStreamName: GetStreamName::<Identity, OFFSET>,
            SetStreamName: SetStreamName::<Identity, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, OFFSET>,
            SetConnectionName: SetConnectionName::<Identity, OFFSET>,
            GetBitrate: GetBitrate::<Identity, OFFSET>,
            SetBitrate: SetBitrate::<Identity, OFFSET>,
            GetBufferWindow: GetBufferWindow::<Identity, OFFSET>,
            SetBufferWindow: SetBufferWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMStreamConfig {}
windows_core::imp::define_interface!(IWMStreamConfig2, IWMStreamConfig2_Vtbl, 0x7688d8cb_fc0d_43bd_9459_5a8dec200cfa);
impl core::ops::Deref for IWMStreamConfig2 {
    type Target = IWMStreamConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMStreamConfig2, windows_core::IUnknown, IWMStreamConfig);
impl IWMStreamConfig2 {
    pub unsafe fn GetTransportType(&self) -> windows_core::Result<WMT_TRANSPORT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransportType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTransportType)(windows_core::Interface::as_raw(self), ntransporttype) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddDataUnitExtension(&self, guidextensionsystemid: windows_core::GUID, cbextensiondatasize: super::WORD, pbextensionsysteminfo: *const super::BYTE, cbextensionsysteminfo: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDataUnitExtension)(windows_core::Interface::as_raw(self), guidextensionsystemid, cbextensiondatasize, pbextensionsysteminfo, cbextensionsysteminfo) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDataUnitExtensionCount(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataUnitExtensionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: super::WORD, pguidextensionsystemid: *mut windows_core::GUID, pcbextensiondatasize: *mut super::WORD, pbextensionsysteminfo: *mut super::BYTE, pcbextensionsysteminfo: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDataUnitExtension)(windows_core::Interface::as_raw(self), wdataunitextensionnumber, pguidextensionsystemid as _, pcbextensiondatasize as _, pbextensionsysteminfo as _, pcbextensionsysteminfo as _) }
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllDataUnitExtensions)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig2_Vtbl {
    pub base__: IWMStreamConfig_Vtbl,
    pub GetTransportType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_TRANSPORT_TYPE) -> windows_core::HRESULT,
    pub SetTransportType: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_TRANSPORT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AddDataUnitExtension: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::WORD, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddDataUnitExtension: usize,
    #[cfg(feature = "shared")]
    pub GetDataUnitExtensionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDataUnitExtensionCount: usize,
    #[cfg(feature = "shared")]
    pub GetDataUnitExtension: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut windows_core::GUID, *mut super::WORD, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDataUnitExtension: usize,
    pub RemoveAllDataUnitExtensions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMStreamConfig2_Impl: IWMStreamConfig_Impl {
    fn GetTransportType(&self) -> windows_core::Result<WMT_TRANSPORT_TYPE>;
    fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> windows_core::Result<()>;
    fn AddDataUnitExtension(&self, guidextensionsystemid: &windows_core::GUID, cbextensiondatasize: super::WORD, pbextensionsysteminfo: *const super::BYTE, cbextensionsysteminfo: super::DWORD) -> windows_core::Result<()>;
    fn GetDataUnitExtensionCount(&self) -> windows_core::Result<super::WORD>;
    fn GetDataUnitExtension(&self, wdataunitextensionnumber: super::WORD, pguidextensionsystemid: *mut windows_core::GUID, pcbextensiondatasize: *mut super::WORD, pbextensionsysteminfo: *mut super::BYTE, pcbextensionsysteminfo: *mut super::DWORD) -> windows_core::Result<()>;
    fn RemoveAllDataUnitExtensions(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMStreamConfig2_Vtbl {
    pub const fn new<Identity: IWMStreamConfig2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransportType<Identity: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMStreamConfig2_Impl::GetTransportType(this) {
                    Ok(ok__) => {
                        pntransporttype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransportType<Identity: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig2_Impl::SetTransportType(this, core::mem::transmute_copy(&ntransporttype)).into()
            }
        }
        unsafe extern "system" fn AddDataUnitExtension<Identity: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidextensionsystemid: windows_core::GUID, cbextensiondatasize: super::WORD, pbextensionsysteminfo: *const super::BYTE, cbextensionsysteminfo: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig2_Impl::AddDataUnitExtension(this, core::mem::transmute(&guidextensionsystemid), core::mem::transmute_copy(&cbextensiondatasize), core::mem::transmute_copy(&pbextensionsysteminfo), core::mem::transmute_copy(&cbextensionsysteminfo)).into()
            }
        }
        unsafe extern "system" fn GetDataUnitExtensionCount<Identity: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdataunitextensions: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMStreamConfig2_Impl::GetDataUnitExtensionCount(this) {
                    Ok(ok__) => {
                        pcdataunitextensions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataUnitExtension<Identity: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wdataunitextensionnumber: super::WORD, pguidextensionsystemid: *mut windows_core::GUID, pcbextensiondatasize: *mut super::WORD, pbextensionsysteminfo: *mut super::BYTE, pcbextensionsysteminfo: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig2_Impl::GetDataUnitExtension(this, core::mem::transmute_copy(&wdataunitextensionnumber), core::mem::transmute_copy(&pguidextensionsystemid), core::mem::transmute_copy(&pcbextensiondatasize), core::mem::transmute_copy(&pbextensionsysteminfo), core::mem::transmute_copy(&pcbextensionsysteminfo)).into()
            }
        }
        unsafe extern "system" fn RemoveAllDataUnitExtensions<Identity: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig2_Impl::RemoveAllDataUnitExtensions(this).into()
            }
        }
        Self {
            base__: IWMStreamConfig_Vtbl::new::<Identity, OFFSET>(),
            GetTransportType: GetTransportType::<Identity, OFFSET>,
            SetTransportType: SetTransportType::<Identity, OFFSET>,
            AddDataUnitExtension: AddDataUnitExtension::<Identity, OFFSET>,
            GetDataUnitExtensionCount: GetDataUnitExtensionCount::<Identity, OFFSET>,
            GetDataUnitExtension: GetDataUnitExtension::<Identity, OFFSET>,
            RemoveAllDataUnitExtensions: RemoveAllDataUnitExtensions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamConfig2 as windows_core::Interface>::IID || iid == &<IWMStreamConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMStreamConfig2 {}
windows_core::imp::define_interface!(IWMStreamConfig3, IWMStreamConfig3_Vtbl, 0xcb164104_3aa9_45a7_9ac9_4daee131d6e1);
impl core::ops::Deref for IWMStreamConfig3 {
    type Target = IWMStreamConfig2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMStreamConfig3, windows_core::IUnknown, IWMStreamConfig, IWMStreamConfig2);
impl IWMStreamConfig3 {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetLanguage(&self, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), pwszlanguagestring as _, pcchlanguagestringlength as _) }
    }
    pub unsafe fn SetLanguage<P0>(&self, pwszlanguagestring: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), pwszlanguagestring.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig3_Vtbl {
    pub base__: IWMStreamConfig2_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetLanguage: usize,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWMStreamConfig3_Impl: IWMStreamConfig2_Impl {
    fn GetLanguage(&self, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::Result<()>;
    fn SetLanguage(&self, pwszlanguagestring: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWMStreamConfig3_Vtbl {
    pub const fn new<Identity: IWMStreamConfig3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLanguage<Identity: IWMStreamConfig3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlanguagestring: *mut super::WCHAR, pcchlanguagestringlength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig3_Impl::GetLanguage(this, core::mem::transmute_copy(&pwszlanguagestring), core::mem::transmute_copy(&pcchlanguagestringlength)).into()
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: IWMStreamConfig3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlanguagestring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamConfig3_Impl::SetLanguage(this, core::mem::transmute(&pwszlanguagestring)).into()
            }
        }
        Self {
            base__: IWMStreamConfig2_Vtbl::new::<Identity, OFFSET>(),
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamConfig3 as windows_core::Interface>::IID || iid == &<IWMStreamConfig as windows_core::Interface>::IID || iid == &<IWMStreamConfig2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWMStreamConfig3 {}
windows_core::imp::define_interface!(IWMStreamList, IWMStreamList_Vtbl, 0x96406bdd_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMStreamList, windows_core::IUnknown);
impl IWMStreamList {
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut super::WORD, pcstreams: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreams)(windows_core::Interface::as_raw(self), pwstreamnumarray as _, pcstreams as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddStream(&self, wstreamnum: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), wstreamnum) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveStream(&self, wstreamnum: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), wstreamnum) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreams: usize,
    #[cfg(feature = "shared")]
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddStream: usize,
    #[cfg(feature = "shared")]
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveStream: usize,
}
#[cfg(feature = "shared")]
pub trait IWMStreamList_Impl: windows_core::IUnknownImpl {
    fn GetStreams(&self, pwstreamnumarray: *mut super::WORD, pcstreams: *mut super::WORD) -> windows_core::Result<()>;
    fn AddStream(&self, wstreamnum: super::WORD) -> windows_core::Result<()>;
    fn RemoveStream(&self, wstreamnum: super::WORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMStreamList_Vtbl {
    pub const fn new<Identity: IWMStreamList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreams<Identity: IWMStreamList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstreamnumarray: *mut super::WORD, pcstreams: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamList_Impl::GetStreams(this, core::mem::transmute_copy(&pwstreamnumarray), core::mem::transmute_copy(&pcstreams)).into()
            }
        }
        unsafe extern "system" fn AddStream<Identity: IWMStreamList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamList_Impl::AddStream(this, core::mem::transmute_copy(&wstreamnum)).into()
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: IWMStreamList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamList_Impl::RemoveStream(this, core::mem::transmute_copy(&wstreamnum)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreams: GetStreams::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMStreamList {}
windows_core::imp::define_interface!(IWMStreamPrioritization, IWMStreamPrioritization_Vtbl, 0x8c1c6090_f9a8_4748_8ec3_dd1108ba1e77);
windows_core::imp::interface_hierarchy!(IWMStreamPrioritization, windows_core::IUnknown);
impl IWMStreamPrioritization {
    #[cfg(feature = "shared")]
    pub unsafe fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPriorityRecords)(windows_core::Interface::as_raw(self), precordarray as _, pcrecords as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriorityRecords)(windows_core::Interface::as_raw(self), precordarray, crecords) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamPrioritization_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPriorityRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WM_STREAM_PRIORITY_RECORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPriorityRecords: usize,
    #[cfg(feature = "shared")]
    pub SetPriorityRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *const WM_STREAM_PRIORITY_RECORD, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPriorityRecords: usize,
}
#[cfg(feature = "shared")]
pub trait IWMStreamPrioritization_Impl: windows_core::IUnknownImpl {
    fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut super::WORD) -> windows_core::Result<()>;
    fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: super::WORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMStreamPrioritization_Vtbl {
    pub const fn new<Identity: IWMStreamPrioritization_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPriorityRecords<Identity: IWMStreamPrioritization_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamPrioritization_Impl::GetPriorityRecords(this, core::mem::transmute_copy(&precordarray), core::mem::transmute_copy(&pcrecords)).into()
            }
        }
        unsafe extern "system" fn SetPriorityRecords<Identity: IWMStreamPrioritization_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMStreamPrioritization_Impl::SetPriorityRecords(this, core::mem::transmute_copy(&precordarray), core::mem::transmute_copy(&crecords)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPriorityRecords: GetPriorityRecords::<Identity, OFFSET>,
            SetPriorityRecords: SetPriorityRecords::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamPrioritization as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMStreamPrioritization {}
windows_core::imp::define_interface!(IWMSyncReader, IWMSyncReader_Vtbl, 0x9397f121_7705_4dc9_b049_98b698188414);
windows_core::imp::interface_hierarchy!(IWMSyncReader, windows_core::IUnknown);
impl IWMSyncReader {
    #[cfg(feature = "winnt")]
    pub unsafe fn Open(&self, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszfilename) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRange(&self, cnsstarttime: super::QWORD, cnsduration: super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRange)(windows_core::Interface::as_raw(self), cnsstarttime, cnsduration) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: super::WORD, qwframenumber: super::QWORD, cframestoread: super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRangeByFrame)(windows_core::Interface::as_raw(self), wstreamnum, qwframenumber, cframestoread) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn GetNextSample(&self, wstreamnum: super::WORD, ppsample: *mut Option<super::INSSBuffer>, pcnssampletime: *mut super::QWORD, pcnsduration: *mut super::QWORD, pdwflags: *mut super::DWORD, pdwoutputnum: *mut super::DWORD, pwstreamnum: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNextSample)(windows_core::Interface::as_raw(self), wstreamnum, core::mem::transmute(ppsample), pcnssampletime as _, pcnsduration as _, pdwflags as _, pdwoutputnum as _, pwstreamnum as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: super::WORD, pwstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamsSelected)(windows_core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: super::WORD) -> windows_core::Result<WMT_STREAM_SELECTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelected)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetReadStreamSamples(&self, wstreamnum: super::WORD, fcompressed: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReadStreamSamples)(windows_core::Interface::as_raw(self), wstreamnum, fcompressed.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: super::WORD) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReadStreamSamples)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputSetting<P1>(&self, dwoutputnum: super::DWORD, pszname: P1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetOutputSetting)(windows_core::Interface::as_raw(self), dwoutputnum, pszname.param().abi(), ptype as _, pvalue as _, pcblength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOutputSetting<P1>(&self, dwoutputnum: super::DWORD, pszname: P1, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputSetting)(windows_core::Interface::as_raw(self), dwoutputnum, pszname.param().abi(), r#type, pvalue, cblength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputProps(&self, dwoutputnum: super::DWORD) -> windows_core::Result<IWMOutputMediaProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOutputProps<P1>(&self, dwoutputnum: super::DWORD, poutput: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMOutputMediaProps>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputProps)(windows_core::Interface::as_raw(self), dwoutputnum, poutput.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputFormatCount)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: super::DWORD, dwformatnum: super::DWORD) -> windows_core::Result<IWMOutputMediaProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputFormat)(windows_core::Interface::as_raw(self), dwoutputnum, dwformatnum, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: super::WORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputNumberForStream)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: super::DWORD) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamNumberForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxOutputSampleSize)(windows_core::Interface::as_raw(self), dwoutput, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: super::WORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxStreamSampleSize)(windows_core::Interface::as_raw(self), wstream, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn OpenStream<P0>(&self, pstream: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenStream)(windows_core::Interface::as_raw(self), pstream.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Open: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetRange: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD, super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRange: usize,
    #[cfg(feature = "shared")]
    pub SetRangeByFrame: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::QWORD, super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRangeByFrame: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub GetNextSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut *mut core::ffi::c_void, *mut super::QWORD, *mut super::QWORD, *mut super::DWORD, *mut super::DWORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    GetNextSample: usize,
    #[cfg(feature = "shared")]
    pub SetStreamsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *const super::WORD, *const WMT_STREAM_SELECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetStreamsSelected: usize,
    #[cfg(feature = "shared")]
    pub GetStreamSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut WMT_STREAM_SELECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamSelected: usize,
    #[cfg(feature = "shared")]
    pub SetReadStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetReadStreamSamples: usize,
    #[cfg(feature = "shared")]
    pub GetReadStreamSamples: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetReadStreamSamples: usize,
    #[cfg(feature = "shared")]
    pub GetOutputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputSetting: usize,
    #[cfg(feature = "shared")]
    pub SetOutputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOutputSetting: usize,
    #[cfg(feature = "shared")]
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputCount: usize,
    #[cfg(feature = "shared")]
    pub GetOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputProps: usize,
    #[cfg(feature = "shared")]
    pub SetOutputProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOutputProps: usize,
    #[cfg(feature = "shared")]
    pub GetOutputFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputFormatCount: usize,
    #[cfg(feature = "shared")]
    pub GetOutputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputFormat: usize,
    #[cfg(feature = "shared")]
    pub GetOutputNumberForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOutputNumberForStream: usize,
    #[cfg(feature = "shared")]
    pub GetStreamNumberForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamNumberForOutput: usize,
    #[cfg(feature = "shared")]
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxOutputSampleSize: usize,
    #[cfg(feature = "shared")]
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxStreamSampleSize: usize,
    #[cfg(feature = "objidlbase")]
    pub OpenStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    OpenStream: usize,
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
pub trait IWMSyncReader_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pwszfilename: *const super::WCHAR) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn SetRange(&self, cnsstarttime: super::QWORD, cnsduration: super::LONGLONG) -> windows_core::Result<()>;
    fn SetRangeByFrame(&self, wstreamnum: super::WORD, qwframenumber: super::QWORD, cframestoread: super::LONGLONG) -> windows_core::Result<()>;
    fn GetNextSample(&self, wstreamnum: super::WORD, ppsample: windows_core::OutRef<super::INSSBuffer>, pcnssampletime: *mut super::QWORD, pcnsduration: *mut super::QWORD, pdwflags: *mut super::DWORD, pdwoutputnum: *mut super::DWORD, pwstreamnum: *mut super::WORD) -> windows_core::Result<()>;
    fn SetStreamsSelected(&self, cstreamcount: super::WORD, pwstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION) -> windows_core::Result<()>;
    fn GetStreamSelected(&self, wstreamnum: super::WORD) -> windows_core::Result<WMT_STREAM_SELECTION>;
    fn SetReadStreamSamples(&self, wstreamnum: super::WORD, fcompressed: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetReadStreamSamples(&self, wstreamnum: super::WORD) -> windows_core::Result<windows_core::BOOL>;
    fn GetOutputSetting(&self, dwoutputnum: super::DWORD, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
    fn SetOutputSetting(&self, dwoutputnum: super::DWORD, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::Result<()>;
    fn GetOutputCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetOutputProps(&self, dwoutputnum: super::DWORD) -> windows_core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&self, dwoutputnum: super::DWORD, poutput: windows_core::Ref<IWMOutputMediaProps>) -> windows_core::Result<()>;
    fn GetOutputFormatCount(&self, dwoutputnum: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn GetOutputFormat(&self, dwoutputnum: super::DWORD, dwformatnum: super::DWORD) -> windows_core::Result<IWMOutputMediaProps>;
    fn GetOutputNumberForStream(&self, wstreamnum: super::WORD) -> windows_core::Result<super::DWORD>;
    fn GetStreamNumberForOutput(&self, dwoutputnum: super::DWORD) -> windows_core::Result<super::WORD>;
    fn GetMaxOutputSampleSize(&self, dwoutput: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn GetMaxStreamSampleSize(&self, wstream: super::WORD) -> windows_core::Result<super::DWORD>;
    fn OpenStream(&self, pstream: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl IWMSyncReader_Vtbl {
    pub const fn new<Identity: IWMSyncReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::Open(this, core::mem::transmute_copy(&pwszfilename)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn SetRange<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstarttime: super::QWORD, cnsduration: super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::SetRange(this, core::mem::transmute_copy(&cnsstarttime), core::mem::transmute_copy(&cnsduration)).into()
            }
        }
        unsafe extern "system" fn SetRangeByFrame<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, qwframenumber: super::QWORD, cframestoread: super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::SetRangeByFrame(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&qwframenumber), core::mem::transmute_copy(&cframestoread)).into()
            }
        }
        unsafe extern "system" fn GetNextSample<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, ppsample: *mut *mut core::ffi::c_void, pcnssampletime: *mut super::QWORD, pcnsduration: *mut super::QWORD, pdwflags: *mut super::DWORD, pdwoutputnum: *mut super::DWORD, pwstreamnum: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::GetNextSample(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&ppsample), core::mem::transmute_copy(&pcnssampletime), core::mem::transmute_copy(&pcnsduration), core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pdwoutputnum), core::mem::transmute_copy(&pwstreamnum)).into()
            }
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cstreamcount: super::WORD, pwstreamnumbers: *const super::WORD, pselections: *const WMT_STREAM_SELECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::SetStreamsSelected(this, core::mem::transmute_copy(&cstreamcount), core::mem::transmute_copy(&pwstreamnumbers), core::mem::transmute_copy(&pselections)).into()
            }
        }
        unsafe extern "system" fn GetStreamSelected<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pselection: *mut WMT_STREAM_SELECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetStreamSelected(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pselection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReadStreamSamples<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, fcompressed: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::SetReadStreamSamples(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&fcompressed)).into()
            }
        }
        unsafe extern "system" fn GetReadStreamSamples<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pfcompressed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetReadStreamSamples(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pfcompressed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputSetting<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::GetOutputSetting(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn SetOutputSetting<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::SetOutputSetting(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
            }
        }
        unsafe extern "system" fn GetOutputCount<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoutputs: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetOutputCount(this) {
                    Ok(ok__) => {
                        pcoutputs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputProps<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetOutputProps(this, core::mem::transmute_copy(&dwoutputnum)) {
                    Ok(ok__) => {
                        ppoutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutputProps<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::SetOutputProps(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pcformats: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetOutputFormatCount(this, core::mem::transmute_copy(&dwoutputnum)) {
                    Ok(ok__) => {
                        pcformats.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, dwformatnum: super::DWORD, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetOutputFormat(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&dwformatnum)) {
                    Ok(ok__) => {
                        ppprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputNumberForStream<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pdwoutputnum: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetOutputNumberForStream(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pdwoutputnum.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamNumberForOutput<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pwstreamnum: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetStreamNumberForOutput(this, core::mem::transmute_copy(&dwoutputnum)) {
                    Ok(ok__) => {
                        pwstreamnum.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutput: super::DWORD, pcbmax: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetMaxOutputSampleSize(this, core::mem::transmute_copy(&dwoutput)) {
                    Ok(ok__) => {
                        pcbmax.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstream: super::WORD, pcbmax: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader_Impl::GetMaxStreamSampleSize(this, core::mem::transmute_copy(&wstream)) {
                    Ok(ok__) => {
                        pcbmax.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenStream<Identity: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader_Impl::OpenStream(this, core::mem::transmute_copy(&pstream)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetRange: SetRange::<Identity, OFFSET>,
            SetRangeByFrame: SetRangeByFrame::<Identity, OFFSET>,
            GetNextSample: GetNextSample::<Identity, OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Identity, OFFSET>,
            GetStreamSelected: GetStreamSelected::<Identity, OFFSET>,
            SetReadStreamSamples: SetReadStreamSamples::<Identity, OFFSET>,
            GetReadStreamSamples: GetReadStreamSamples::<Identity, OFFSET>,
            GetOutputSetting: GetOutputSetting::<Identity, OFFSET>,
            SetOutputSetting: SetOutputSetting::<Identity, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            GetOutputProps: GetOutputProps::<Identity, OFFSET>,
            SetOutputProps: SetOutputProps::<Identity, OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Identity, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, OFFSET>,
            GetOutputNumberForStream: GetOutputNumberForStream::<Identity, OFFSET>,
            GetStreamNumberForOutput: GetStreamNumberForOutput::<Identity, OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Identity, OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Identity, OFFSET>,
            OpenStream: OpenStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSyncReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMSyncReader {}
windows_core::imp::define_interface!(IWMSyncReader2, IWMSyncReader2_Vtbl, 0xfaed3d21_1b6b_4af7_8cb6_3e189bbc187b);
impl core::ops::Deref for IWMSyncReader2 {
    type Target = IWMSyncReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMSyncReader2, windows_core::IUnknown, IWMSyncReader);
impl IWMSyncReader2 {
    #[cfg(feature = "shared")]
    pub unsafe fn SetRangeByTimecode(&self, wstreamnum: super::WORD, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRangeByTimecode)(windows_core::Interface::as_raw(self), wstreamnum, pstart, pend) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRangeByFrameEx(&self, wstreamnum: super::WORD, qwframenumber: super::QWORD, cframestoread: super::LONGLONG) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetRangeByFrameEx)(windows_core::Interface::as_raw(self), wstreamnum, qwframenumber, cframestoread, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllocateForOutput<P1>(&self, dwoutputnum: super::DWORD, pallocator: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMReaderAllocatorEx>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, pallocator.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: super::DWORD) -> windows_core::Result<IWMReaderAllocatorEx> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllocateForOutput)(windows_core::Interface::as_raw(self), dwoutputnum, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllocateForStream<P1>(&self, wstreamnum: super::WORD, pallocator: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMReaderAllocatorEx>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAllocateForStream)(windows_core::Interface::as_raw(self), wstreamnum, pallocator.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: super::WORD) -> windows_core::Result<IWMReaderAllocatorEx> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllocateForStream)(windows_core::Interface::as_raw(self), dwsreamnum, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader2_Vtbl {
    pub base__: IWMSyncReader_Vtbl,
    #[cfg(feature = "shared")]
    pub SetRangeByTimecode: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *const WMT_TIMECODE_EXTENSION_DATA, *const WMT_TIMECODE_EXTENSION_DATA) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRangeByTimecode: usize,
    #[cfg(feature = "shared")]
    pub SetRangeByFrameEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::QWORD, super::LONGLONG, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRangeByFrameEx: usize,
    #[cfg(feature = "shared")]
    pub SetAllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllocateForOutput: usize,
    #[cfg(feature = "shared")]
    pub GetAllocateForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllocateForOutput: usize,
    #[cfg(feature = "shared")]
    pub SetAllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllocateForStream: usize,
    #[cfg(feature = "shared")]
    pub GetAllocateForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllocateForStream: usize,
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
pub trait IWMSyncReader2_Impl: IWMSyncReader_Impl {
    fn SetRangeByTimecode(&self, wstreamnum: super::WORD, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> windows_core::Result<()>;
    fn SetRangeByFrameEx(&self, wstreamnum: super::WORD, qwframenumber: super::QWORD, cframestoread: super::LONGLONG) -> windows_core::Result<super::QWORD>;
    fn SetAllocateForOutput(&self, dwoutputnum: super::DWORD, pallocator: windows_core::Ref<IWMReaderAllocatorEx>) -> windows_core::Result<()>;
    fn GetAllocateForOutput(&self, dwoutputnum: super::DWORD) -> windows_core::Result<IWMReaderAllocatorEx>;
    fn SetAllocateForStream(&self, wstreamnum: super::WORD, pallocator: windows_core::Ref<IWMReaderAllocatorEx>) -> windows_core::Result<()>;
    fn GetAllocateForStream(&self, dwsreamnum: super::WORD) -> windows_core::Result<IWMReaderAllocatorEx>;
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl IWMSyncReader2_Vtbl {
    pub const fn new<Identity: IWMSyncReader2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRangeByTimecode<Identity: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader2_Impl::SetRangeByTimecode(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pstart), core::mem::transmute_copy(&pend)).into()
            }
        }
        unsafe extern "system" fn SetRangeByFrameEx<Identity: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, qwframenumber: super::QWORD, cframestoread: super::LONGLONG, pcnsstarttime: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader2_Impl::SetRangeByFrameEx(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&qwframenumber), core::mem::transmute_copy(&cframestoread)) {
                    Ok(ok__) => {
                        pcnsstarttime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, pallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader2_Impl::SetAllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&pallocator)).into()
            }
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: super::DWORD, ppallocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader2_Impl::GetAllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum)) {
                    Ok(ok__) => {
                        ppallocator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSyncReader2_Impl::SetAllocateForStream(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pallocator)).into()
            }
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsreamnum: super::WORD, ppallocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMSyncReader2_Impl::GetAllocateForStream(this, core::mem::transmute_copy(&dwsreamnum)) {
                    Ok(ok__) => {
                        ppallocator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMSyncReader_Vtbl::new::<Identity, OFFSET>(),
            SetRangeByTimecode: SetRangeByTimecode::<Identity, OFFSET>,
            SetRangeByFrameEx: SetRangeByFrameEx::<Identity, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSyncReader2 as windows_core::Interface>::IID || iid == &<IWMSyncReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMSyncReader2 {}
windows_core::imp::define_interface!(IWMVideoMediaProps, IWMVideoMediaProps_Vtbl, 0x96406bcf_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMVideoMediaProps {
    type Target = IWMMediaProps;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMVideoMediaProps, windows_core::IUnknown, IWMMediaProps);
impl IWMVideoMediaProps {
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxKeyFrameSpacing(&self) -> windows_core::Result<super::LONGLONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxKeyFrameSpacing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMaxKeyFrameSpacing(&self, lltime: super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxKeyFrameSpacing)(windows_core::Interface::as_raw(self), lltime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetQuality(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQuality)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetQuality(&self, dwquality: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQuality)(windows_core::Interface::as_raw(self), dwquality) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    #[cfg(feature = "shared")]
    pub GetMaxKeyFrameSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxKeyFrameSpacing: usize,
    #[cfg(feature = "shared")]
    pub SetMaxKeyFrameSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMaxKeyFrameSpacing: usize,
    #[cfg(feature = "shared")]
    pub GetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetQuality: usize,
    #[cfg(feature = "shared")]
    pub SetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetQuality: usize,
}
#[cfg(feature = "shared")]
pub trait IWMVideoMediaProps_Impl: IWMMediaProps_Impl {
    fn GetMaxKeyFrameSpacing(&self) -> windows_core::Result<super::LONGLONG>;
    fn SetMaxKeyFrameSpacing(&self, lltime: super::LONGLONG) -> windows_core::Result<()>;
    fn GetQuality(&self) -> windows_core::Result<super::DWORD>;
    fn SetQuality(&self, dwquality: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMVideoMediaProps_Vtbl {
    pub const fn new<Identity: IWMVideoMediaProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaxKeyFrameSpacing<Identity: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plltime: *mut super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMVideoMediaProps_Impl::GetMaxKeyFrameSpacing(this) {
                    Ok(ok__) => {
                        plltime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxKeyFrameSpacing<Identity: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lltime: super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMVideoMediaProps_Impl::SetMaxKeyFrameSpacing(this, core::mem::transmute_copy(&lltime)).into()
            }
        }
        unsafe extern "system" fn GetQuality<Identity: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwquality: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMVideoMediaProps_Impl::GetQuality(this) {
                    Ok(ok__) => {
                        pdwquality.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuality<Identity: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwquality: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMVideoMediaProps_Impl::SetQuality(this, core::mem::transmute_copy(&dwquality)).into()
            }
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, OFFSET>(),
            GetMaxKeyFrameSpacing: GetMaxKeyFrameSpacing::<Identity, OFFSET>,
            SetMaxKeyFrameSpacing: SetMaxKeyFrameSpacing::<Identity, OFFSET>,
            GetQuality: GetQuality::<Identity, OFFSET>,
            SetQuality: SetQuality::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMVideoMediaProps as windows_core::Interface>::IID || iid == &<IWMMediaProps as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMVideoMediaProps {}
windows_core::imp::define_interface!(IWMWatermarkInfo, IWMWatermarkInfo_Vtbl, 0x6f497062_f2e2_4624_8ea7_9dd40d81fc8d);
windows_core::imp::interface_hierarchy!(IWMWatermarkInfo, windows_core::IUnknown);
impl IWMWatermarkInfo {
    #[cfg(feature = "shared")]
    pub unsafe fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWatermarkEntryCount)(windows_core::Interface::as_raw(self), wmettype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: super::DWORD, pentry: *mut WMT_WATERMARK_ENTRY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWatermarkEntry)(windows_core::Interface::as_raw(self), wmettype, dwentrynum, pentry as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWatermarkInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetWatermarkEntryCount: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_WATERMARK_ENTRY_TYPE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetWatermarkEntryCount: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetWatermarkEntry: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_WATERMARK_ENTRY_TYPE, super::DWORD, *mut WMT_WATERMARK_ENTRY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetWatermarkEntry: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IWMWatermarkInfo_Impl: windows_core::IUnknownImpl {
    fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> windows_core::Result<super::DWORD>;
    fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: super::DWORD, pentry: *mut WMT_WATERMARK_ENTRY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IWMWatermarkInfo_Vtbl {
    pub const fn new<Identity: IWMWatermarkInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWatermarkEntryCount<Identity: IWMWatermarkInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWatermarkInfo_Impl::GetWatermarkEntryCount(this, core::mem::transmute_copy(&wmettype)) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWatermarkEntry<Identity: IWMWatermarkInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: super::DWORD, pentry: *mut WMT_WATERMARK_ENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWatermarkInfo_Impl::GetWatermarkEntry(this, core::mem::transmute_copy(&wmettype), core::mem::transmute_copy(&dwentrynum), core::mem::transmute_copy(&pentry)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWatermarkEntryCount: GetWatermarkEntryCount::<Identity, OFFSET>,
            GetWatermarkEntry: GetWatermarkEntry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWatermarkInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IWMWatermarkInfo {}
windows_core::imp::define_interface!(IWMWriter, IWMWriter_Vtbl, 0x96406bd4_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMWriter, windows_core::IUnknown);
impl IWMWriter {
    pub unsafe fn SetProfileByID(&self, guidprofile: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProfileByID)(windows_core::Interface::as_raw(self), guidprofile) }
    }
    pub unsafe fn SetProfile<P0>(&self, pprofile: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetOutputFilename(&self, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFilename)(windows_core::Interface::as_raw(self), pwszfilename) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInputCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInputProps(&self, dwinputnum: super::DWORD) -> windows_core::Result<IWMInputMediaProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputProps)(windows_core::Interface::as_raw(self), dwinputnum, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetInputProps<P1>(&self, dwinputnum: super::DWORD, pinput: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMInputMediaProps>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputProps)(windows_core::Interface::as_raw(self), dwinputnum, pinput.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInputFormatCount(&self, dwinputnumber: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputFormatCount)(windows_core::Interface::as_raw(self), dwinputnumber, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInputFormat(&self, dwinputnumber: super::DWORD, dwformatnumber: super::DWORD) -> windows_core::Result<IWMInputMediaProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputFormat)(windows_core::Interface::as_raw(self), dwinputnumber, dwformatnumber, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginWriting(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginWriting)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndWriting(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndWriting)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn AllocateSample(&self, dwsamplesize: super::DWORD) -> windows_core::Result<super::INSSBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocateSample)(windows_core::Interface::as_raw(self), dwsamplesize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn WriteSample<P3>(&self, dwinputnum: super::DWORD, cnssampletime: super::QWORD, dwflags: super::DWORD, psample: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<super::INSSBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteSample)(windows_core::Interface::as_raw(self), dwinputnum, cnssampletime, dwflags, psample.param().abi()) }
    }
    pub unsafe fn Flush(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProfileByID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub SetOutputFilename: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetOutputFilename: usize,
    #[cfg(feature = "shared")]
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInputCount: usize,
    #[cfg(feature = "shared")]
    pub GetInputProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInputProps: usize,
    #[cfg(feature = "shared")]
    pub SetInputProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetInputProps: usize,
    #[cfg(feature = "shared")]
    pub GetInputFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInputFormatCount: usize,
    #[cfg(feature = "shared")]
    pub GetInputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInputFormat: usize,
    pub BeginWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub AllocateSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    AllocateSample: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub WriteSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::QWORD, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    WriteSample: usize,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
pub trait IWMWriter_Impl: windows_core::IUnknownImpl {
    fn SetProfileByID(&self, guidprofile: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetProfile(&self, pprofile: windows_core::Ref<IWMProfile>) -> windows_core::Result<()>;
    fn SetOutputFilename(&self, pwszfilename: *const super::WCHAR) -> windows_core::Result<()>;
    fn GetInputCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetInputProps(&self, dwinputnum: super::DWORD) -> windows_core::Result<IWMInputMediaProps>;
    fn SetInputProps(&self, dwinputnum: super::DWORD, pinput: windows_core::Ref<IWMInputMediaProps>) -> windows_core::Result<()>;
    fn GetInputFormatCount(&self, dwinputnumber: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn GetInputFormat(&self, dwinputnumber: super::DWORD, dwformatnumber: super::DWORD) -> windows_core::Result<IWMInputMediaProps>;
    fn BeginWriting(&self) -> windows_core::Result<()>;
    fn EndWriting(&self) -> windows_core::Result<()>;
    fn AllocateSample(&self, dwsamplesize: super::DWORD) -> windows_core::Result<super::INSSBuffer>;
    fn WriteSample(&self, dwinputnum: super::DWORD, cnssampletime: super::QWORD, dwflags: super::DWORD, psample: windows_core::Ref<super::INSSBuffer>) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl IWMWriter_Vtbl {
    pub const fn new<Identity: IWMWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProfileByID<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprofile: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriter_Impl::SetProfileByID(this, core::mem::transmute_copy(&guidprofile)).into()
            }
        }
        unsafe extern "system" fn SetProfile<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriter_Impl::SetProfile(this, core::mem::transmute_copy(&pprofile)).into()
            }
        }
        unsafe extern "system" fn SetOutputFilename<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriter_Impl::SetOutputFilename(this, core::mem::transmute_copy(&pwszfilename)).into()
            }
        }
        unsafe extern "system" fn GetInputCount<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputs: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriter_Impl::GetInputCount(this) {
                    Ok(ok__) => {
                        pcinputs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputProps<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, ppinput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriter_Impl::GetInputProps(this, core::mem::transmute_copy(&dwinputnum)) {
                    Ok(ok__) => {
                        ppinput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputProps<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, pinput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriter_Impl::SetInputProps(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&pinput)).into()
            }
        }
        unsafe extern "system" fn GetInputFormatCount<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnumber: super::DWORD, pcformats: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriter_Impl::GetInputFormatCount(this, core::mem::transmute_copy(&dwinputnumber)) {
                    Ok(ok__) => {
                        pcformats.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputFormat<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnumber: super::DWORD, dwformatnumber: super::DWORD, pprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriter_Impl::GetInputFormat(this, core::mem::transmute_copy(&dwinputnumber), core::mem::transmute_copy(&dwformatnumber)) {
                    Ok(ok__) => {
                        pprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginWriting<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriter_Impl::BeginWriting(this).into()
            }
        }
        unsafe extern "system" fn EndWriting<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriter_Impl::EndWriting(this).into()
            }
        }
        unsafe extern "system" fn AllocateSample<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsamplesize: super::DWORD, ppsample: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriter_Impl::AllocateSample(this, core::mem::transmute_copy(&dwsamplesize)) {
                    Ok(ok__) => {
                        ppsample.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteSample<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, cnssampletime: super::QWORD, dwflags: super::DWORD, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriter_Impl::WriteSample(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IWMWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriter_Impl::Flush(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProfileByID: SetProfileByID::<Identity, OFFSET>,
            SetProfile: SetProfile::<Identity, OFFSET>,
            SetOutputFilename: SetOutputFilename::<Identity, OFFSET>,
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            GetInputProps: GetInputProps::<Identity, OFFSET>,
            SetInputProps: SetInputProps::<Identity, OFFSET>,
            GetInputFormatCount: GetInputFormatCount::<Identity, OFFSET>,
            GetInputFormat: GetInputFormat::<Identity, OFFSET>,
            BeginWriting: BeginWriting::<Identity, OFFSET>,
            EndWriting: EndWriting::<Identity, OFFSET>,
            AllocateSample: AllocateSample::<Identity, OFFSET>,
            WriteSample: WriteSample::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriter {}
windows_core::imp::define_interface!(IWMWriterAdvanced, IWMWriterAdvanced_Vtbl, 0x96406be3_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMWriterAdvanced, windows_core::IUnknown);
impl IWMWriterAdvanced {
    #[cfg(feature = "shared")]
    pub unsafe fn GetSinkCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSinkCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSink(&self, dwsinknum: super::DWORD) -> windows_core::Result<IWMWriterSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSink)(windows_core::Interface::as_raw(self), dwsinknum, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddSink<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMWriterSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSink)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    pub unsafe fn RemoveSink<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMWriterSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveSink)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn WriteStreamSample<P5>(&self, wstreamnum: super::WORD, cnssampletime: super::QWORD, mssamplesendtime: super::DWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: P5) -> windows_core::HRESULT
    where
        P5: windows_core::Param<super::INSSBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteStreamSample)(windows_core::Interface::as_raw(self), wstreamnum, cnssampletime, mssamplesendtime, cnssampleduration, dwflags, psample.param().abi()) }
    }
    pub unsafe fn SetLiveSource(&self, fislivesource: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLiveSource)(windows_core::Interface::as_raw(self), fislivesource) }
    }
    pub unsafe fn IsRealTime(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRealTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetWriterTime(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatistics(&self, wstreamnum: super::WORD, pstats: *mut WM_WRITER_STATISTICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), wstreamnum, pstats as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSyncTolerance(&self, mswindow: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSyncTolerance)(windows_core::Interface::as_raw(self), mswindow) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSyncTolerance(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSyncTolerance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetSinkCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSinkCount: usize,
    #[cfg(feature = "shared")]
    pub GetSink: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSink: usize,
    pub AddSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub WriteStreamSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::QWORD, super::DWORD, super::QWORD, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    WriteStreamSample: usize,
    pub SetLiveSource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetWriterTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetWriterTime: usize,
    #[cfg(feature = "shared")]
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut WM_WRITER_STATISTICS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatistics: usize,
    #[cfg(feature = "shared")]
    pub SetSyncTolerance: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSyncTolerance: usize,
    #[cfg(feature = "shared")]
    pub GetSyncTolerance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSyncTolerance: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMWriterAdvanced_Impl: windows_core::IUnknownImpl {
    fn GetSinkCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetSink(&self, dwsinknum: super::DWORD) -> windows_core::Result<IWMWriterSink>;
    fn AddSink(&self, psink: windows_core::Ref<IWMWriterSink>) -> windows_core::Result<()>;
    fn RemoveSink(&self, psink: windows_core::Ref<IWMWriterSink>) -> windows_core::Result<()>;
    fn WriteStreamSample(&self, wstreamnum: super::WORD, cnssampletime: super::QWORD, mssamplesendtime: super::DWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: windows_core::Ref<super::INSSBuffer>) -> windows_core::Result<()>;
    fn SetLiveSource(&self, fislivesource: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsRealTime(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetWriterTime(&self) -> windows_core::Result<super::QWORD>;
    fn GetStatistics(&self, wstreamnum: super::WORD, pstats: *mut WM_WRITER_STATISTICS) -> windows_core::Result<()>;
    fn SetSyncTolerance(&self, mswindow: super::DWORD) -> windows_core::Result<()>;
    fn GetSyncTolerance(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMWriterAdvanced_Vtbl {
    pub const fn new<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSinkCount<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsinks: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterAdvanced_Impl::GetSinkCount(this) {
                    Ok(ok__) => {
                        pcsinks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSink<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsinknum: super::DWORD, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterAdvanced_Impl::GetSink(this, core::mem::transmute_copy(&dwsinknum)) {
                    Ok(ok__) => {
                        ppsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSink<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced_Impl::AddSink(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn RemoveSink<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced_Impl::RemoveSink(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn WriteStreamSample<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, cnssampletime: super::QWORD, mssamplesendtime: super::DWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced_Impl::WriteStreamSample(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&mssamplesendtime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn SetLiveSource<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fislivesource: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced_Impl::SetLiveSource(this, core::mem::transmute_copy(&fislivesource)).into()
            }
        }
        unsafe extern "system" fn IsRealTime<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrealtime: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterAdvanced_Impl::IsRealTime(this) {
                    Ok(ok__) => {
                        pfrealtime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterTime<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnscurrenttime: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterAdvanced_Impl::GetWriterTime(this) {
                    Ok(ok__) => {
                        pcnscurrenttime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pstats: *mut WM_WRITER_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced_Impl::GetStatistics(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn SetSyncTolerance<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mswindow: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced_Impl::SetSyncTolerance(this, core::mem::transmute_copy(&mswindow)).into()
            }
        }
        unsafe extern "system" fn GetSyncTolerance<Identity: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmswindow: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterAdvanced_Impl::GetSyncTolerance(this) {
                    Ok(ok__) => {
                        pmswindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSinkCount: GetSinkCount::<Identity, OFFSET>,
            GetSink: GetSink::<Identity, OFFSET>,
            AddSink: AddSink::<Identity, OFFSET>,
            RemoveSink: RemoveSink::<Identity, OFFSET>,
            WriteStreamSample: WriteStreamSample::<Identity, OFFSET>,
            SetLiveSource: SetLiveSource::<Identity, OFFSET>,
            IsRealTime: IsRealTime::<Identity, OFFSET>,
            GetWriterTime: GetWriterTime::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            SetSyncTolerance: SetSyncTolerance::<Identity, OFFSET>,
            GetSyncTolerance: GetSyncTolerance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterAdvanced as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterAdvanced {}
windows_core::imp::define_interface!(IWMWriterAdvanced2, IWMWriterAdvanced2_Vtbl, 0x962dc1ec_c046_4db8_9cc7_26ceae500817);
impl core::ops::Deref for IWMWriterAdvanced2 {
    type Target = IWMWriterAdvanced;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterAdvanced2, windows_core::IUnknown, IWMWriterAdvanced);
impl IWMWriterAdvanced2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetInputSetting<P1>(&self, dwinputnum: super::DWORD, pszname: P1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetInputSetting)(windows_core::Interface::as_raw(self), dwinputnum, pszname.param().abi(), ptype as _, pvalue as _, pcblength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetInputSetting<P1>(&self, dwinputnum: super::DWORD, pszname: P1, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputSetting)(windows_core::Interface::as_raw(self), dwinputnum, pszname.param().abi(), r#type, pvalue, cblength) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced2_Vtbl {
    pub base__: IWMWriterAdvanced_Vtbl,
    #[cfg(feature = "shared")]
    pub GetInputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, *mut WMT_ATTR_DATATYPE, *mut super::BYTE, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInputSetting: usize,
    #[cfg(feature = "shared")]
    pub SetInputSetting: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, WMT_ATTR_DATATYPE, *const super::BYTE, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetInputSetting: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMWriterAdvanced2_Impl: IWMWriterAdvanced_Impl {
    fn GetInputSetting(&self, dwinputnum: super::DWORD, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::Result<()>;
    fn SetInputSetting(&self, dwinputnum: super::DWORD, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMWriterAdvanced2_Vtbl {
    pub const fn new<Identity: IWMWriterAdvanced2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputSetting<Identity: IWMWriterAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut super::BYTE, pcblength: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced2_Impl::GetInputSetting(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn SetInputSetting<Identity: IWMWriterAdvanced2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const super::BYTE, cblength: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced2_Impl::SetInputSetting(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
            }
        }
        Self {
            base__: IWMWriterAdvanced_Vtbl::new::<Identity, OFFSET>(),
            GetInputSetting: GetInputSetting::<Identity, OFFSET>,
            SetInputSetting: SetInputSetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterAdvanced2 as windows_core::Interface>::IID || iid == &<IWMWriterAdvanced as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterAdvanced2 {}
windows_core::imp::define_interface!(IWMWriterAdvanced3, IWMWriterAdvanced3_Vtbl, 0x2cd6492d_7c37_4e76_9d3b_59261183a22e);
impl core::ops::Deref for IWMWriterAdvanced3 {
    type Target = IWMWriterAdvanced2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterAdvanced3, windows_core::IUnknown, IWMWriterAdvanced, IWMWriterAdvanced2);
impl IWMWriterAdvanced3 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatisticsEx(&self, wstreamnum: super::WORD, pstats: *mut WM_WRITER_STATISTICS_EX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatisticsEx)(windows_core::Interface::as_raw(self), wstreamnum, pstats as _) }
    }
    pub unsafe fn SetNonBlocking(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNonBlocking)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced3_Vtbl {
    pub base__: IWMWriterAdvanced2_Vtbl,
    #[cfg(feature = "shared")]
    pub GetStatisticsEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut WM_WRITER_STATISTICS_EX) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatisticsEx: usize,
    pub SetNonBlocking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMWriterAdvanced3_Impl: IWMWriterAdvanced2_Impl {
    fn GetStatisticsEx(&self, wstreamnum: super::WORD, pstats: *mut WM_WRITER_STATISTICS_EX) -> windows_core::Result<()>;
    fn SetNonBlocking(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMWriterAdvanced3_Vtbl {
    pub const fn new<Identity: IWMWriterAdvanced3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatisticsEx<Identity: IWMWriterAdvanced3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pstats: *mut WM_WRITER_STATISTICS_EX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced3_Impl::GetStatisticsEx(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn SetNonBlocking<Identity: IWMWriterAdvanced3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterAdvanced3_Impl::SetNonBlocking(this).into()
            }
        }
        Self {
            base__: IWMWriterAdvanced2_Vtbl::new::<Identity, OFFSET>(),
            GetStatisticsEx: GetStatisticsEx::<Identity, OFFSET>,
            SetNonBlocking: SetNonBlocking::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterAdvanced3 as windows_core::Interface>::IID || iid == &<IWMWriterAdvanced as windows_core::Interface>::IID || iid == &<IWMWriterAdvanced2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterAdvanced3 {}
windows_core::imp::define_interface!(IWMWriterFileSink, IWMWriterFileSink_Vtbl, 0x96406be5_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMWriterFileSink {
    type Target = IWMWriterSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterFileSink, windows_core::IUnknown, IWMWriterSink);
impl IWMWriterFileSink {
    #[cfg(feature = "winnt")]
    pub unsafe fn Open(&self, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszfilename) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    #[cfg(feature = "winnt")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Open: usize,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
pub trait IWMWriterFileSink_Impl: IWMWriterSink_Impl {
    fn Open(&self, pwszfilename: *const super::WCHAR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl IWMWriterFileSink_Vtbl {
    pub const fn new<Identity: IWMWriterFileSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IWMWriterFileSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink_Impl::Open(this, core::mem::transmute_copy(&pwszfilename)).into()
            }
        }
        Self { base__: IWMWriterSink_Vtbl::new::<Identity, OFFSET>(), Open: Open::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterFileSink as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterFileSink {}
windows_core::imp::define_interface!(IWMWriterFileSink2, IWMWriterFileSink2_Vtbl, 0x14282ba7_4aef_4205_8ce5_c229035a05bc);
impl core::ops::Deref for IWMWriterFileSink2 {
    type Target = IWMWriterFileSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterFileSink2, windows_core::IUnknown, IWMWriterSink, IWMWriterFileSink);
impl IWMWriterFileSink2 {
    #[cfg(feature = "shared")]
    pub unsafe fn Start(&self, cnsstarttime: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), cnsstarttime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Stop(&self, cnsstoptime: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), cnsstoptime) }
    }
    pub unsafe fn IsStopped(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStopped)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFileDuration(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsClosed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink2_Vtbl {
    pub base__: IWMWriterFileSink_Vtbl,
    #[cfg(feature = "shared")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Start: usize,
    #[cfg(feature = "shared")]
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Stop: usize,
    pub IsStopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFileDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFileDuration: usize,
    #[cfg(feature = "shared")]
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFileSize: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
pub trait IWMWriterFileSink2_Impl: IWMWriterFileSink_Impl {
    fn Start(&self, cnsstarttime: super::QWORD) -> windows_core::Result<()>;
    fn Stop(&self, cnsstoptime: super::QWORD) -> windows_core::Result<()>;
    fn IsStopped(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetFileDuration(&self) -> windows_core::Result<super::QWORD>;
    fn GetFileSize(&self) -> windows_core::Result<super::QWORD>;
    fn Close(&self) -> windows_core::Result<()>;
    fn IsClosed(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl IWMWriterFileSink2_Vtbl {
    pub const fn new<Identity: IWMWriterFileSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstarttime: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink2_Impl::Start(this, core::mem::transmute_copy(&cnsstarttime)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstoptime: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink2_Impl::Stop(this, core::mem::transmute_copy(&cnsstoptime)).into()
            }
        }
        unsafe extern "system" fn IsStopped<Identity: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfstopped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterFileSink2_Impl::IsStopped(this) {
                    Ok(ok__) => {
                        pfstopped.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileDuration<Identity: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsduration: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterFileSink2_Impl::GetFileDuration(this) {
                    Ok(ok__) => {
                        pcnsduration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbfile: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterFileSink2_Impl::GetFileSize(this) {
                    Ok(ok__) => {
                        pcbfile.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink2_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn IsClosed<Identity: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfclosed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterFileSink2_Impl::IsClosed(this) {
                    Ok(ok__) => {
                        pfclosed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMWriterFileSink_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            IsStopped: IsStopped::<Identity, OFFSET>,
            GetFileDuration: GetFileDuration::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            IsClosed: IsClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterFileSink2 as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID || iid == &<IWMWriterFileSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterFileSink2 {}
windows_core::imp::define_interface!(IWMWriterFileSink3, IWMWriterFileSink3_Vtbl, 0x3fea4feb_2945_47a7_a1dd_c53a8fc4c45c);
impl core::ops::Deref for IWMWriterFileSink3 {
    type Target = IWMWriterFileSink2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterFileSink3, windows_core::IUnknown, IWMWriterSink, IWMWriterFileSink, IWMWriterFileSink2);
impl IWMWriterFileSink3 {
    pub unsafe fn SetAutoIndexing(&self, fdoautoindexing: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoIndexing)(windows_core::Interface::as_raw(self), fdoautoindexing.into()) }
    }
    pub unsafe fn GetAutoIndexing(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutoIndexing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetControlStream(&self, wstreamnumber: super::WORD, fshouldcontrolstartandstop: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetControlStream)(windows_core::Interface::as_raw(self), wstreamnumber, fshouldcontrolstartandstop.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMode(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnDataUnitEx)(windows_core::Interface::as_raw(self), pfilesinkdataunit) }
    }
    pub unsafe fn SetUnbufferedIO(&self, funbufferedio: bool, frestrictmemusage: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnbufferedIO)(windows_core::Interface::as_raw(self), funbufferedio.into(), frestrictmemusage.into()) }
    }
    pub unsafe fn GetUnbufferedIO(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnbufferedIO)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CompleteOperations(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompleteOperations)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink3_Vtbl {
    pub base__: IWMWriterFileSink2_Vtbl,
    pub SetAutoIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAutoIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetControlStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetControlStream: usize,
    #[cfg(feature = "shared")]
    pub GetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMode: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub OnDataUnitEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const WMT_FILESINK_DATA_UNIT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    OnDataUnitEx: usize,
    pub SetUnbufferedIO: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetUnbufferedIO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CompleteOperations: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
pub trait IWMWriterFileSink3_Impl: IWMWriterFileSink2_Impl {
    fn SetAutoIndexing(&self, fdoautoindexing: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAutoIndexing(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetControlStream(&self, wstreamnumber: super::WORD, fshouldcontrolstartandstop: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetMode(&self) -> windows_core::Result<super::DWORD>;
    fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> windows_core::Result<()>;
    fn SetUnbufferedIO(&self, funbufferedio: windows_core::BOOL, frestrictmemusage: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetUnbufferedIO(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CompleteOperations(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl IWMWriterFileSink3_Vtbl {
    pub const fn new<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAutoIndexing<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdoautoindexing: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink3_Impl::SetAutoIndexing(this, core::mem::transmute_copy(&fdoautoindexing)).into()
            }
        }
        unsafe extern "system" fn GetAutoIndexing<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfautoindexing: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterFileSink3_Impl::GetAutoIndexing(this) {
                    Ok(ok__) => {
                        pfautoindexing.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetControlStream<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: super::WORD, fshouldcontrolstartandstop: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink3_Impl::SetControlStream(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&fshouldcontrolstartandstop)).into()
            }
        }
        unsafe extern "system" fn GetMode<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfilesinkmode: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterFileSink3_Impl::GetMode(this) {
                    Ok(ok__) => {
                        pdwfilesinkmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnDataUnitEx<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink3_Impl::OnDataUnitEx(this, core::mem::transmute_copy(&pfilesinkdataunit)).into()
            }
        }
        unsafe extern "system" fn SetUnbufferedIO<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, funbufferedio: windows_core::BOOL, frestrictmemusage: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink3_Impl::SetUnbufferedIO(this, core::mem::transmute_copy(&funbufferedio), core::mem::transmute_copy(&frestrictmemusage)).into()
            }
        }
        unsafe extern "system" fn GetUnbufferedIO<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunbufferedio: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterFileSink3_Impl::GetUnbufferedIO(this) {
                    Ok(ok__) => {
                        pfunbufferedio.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompleteOperations<Identity: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterFileSink3_Impl::CompleteOperations(this).into()
            }
        }
        Self {
            base__: IWMWriterFileSink2_Vtbl::new::<Identity, OFFSET>(),
            SetAutoIndexing: SetAutoIndexing::<Identity, OFFSET>,
            GetAutoIndexing: GetAutoIndexing::<Identity, OFFSET>,
            SetControlStream: SetControlStream::<Identity, OFFSET>,
            GetMode: GetMode::<Identity, OFFSET>,
            OnDataUnitEx: OnDataUnitEx::<Identity, OFFSET>,
            SetUnbufferedIO: SetUnbufferedIO::<Identity, OFFSET>,
            GetUnbufferedIO: GetUnbufferedIO::<Identity, OFFSET>,
            CompleteOperations: CompleteOperations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterFileSink3 as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID || iid == &<IWMWriterFileSink as windows_core::Interface>::IID || iid == &<IWMWriterFileSink2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterFileSink3 {}
windows_core::imp::define_interface!(IWMWriterNetworkSink, IWMWriterNetworkSink_Vtbl, 0x96406be7_2b2b_11d3_b36b_00c04f6108ff);
impl core::ops::Deref for IWMWriterNetworkSink {
    type Target = IWMWriterSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterNetworkSink, windows_core::IUnknown, IWMWriterSink);
impl IWMWriterNetworkSink {
    #[cfg(feature = "shared")]
    pub unsafe fn SetMaximumClients(&self, dwmaxclients: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumClients)(windows_core::Interface::as_raw(self), dwmaxclients) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaximumClients(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumClients)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkProtocol)(windows_core::Interface::as_raw(self), protocol) }
    }
    pub unsafe fn GetNetworkProtocol(&self) -> windows_core::Result<WMT_NET_PROTOCOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkProtocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetHostURL(&self, pwszurl: *mut super::WCHAR, pcchurl: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHostURL)(windows_core::Interface::as_raw(self), pwszurl as _, pcchurl as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Open(&self, pdwportnum: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pdwportnum as _) }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterNetworkSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    #[cfg(feature = "shared")]
    pub SetMaximumClients: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMaximumClients: usize,
    #[cfg(feature = "shared")]
    pub GetMaximumClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaximumClients: usize,
    pub SetNetworkProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, WMT_NET_PROTOCOL) -> windows_core::HRESULT,
    pub GetNetworkProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMT_NET_PROTOCOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetHostURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WCHAR, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetHostURL: usize,
    #[cfg(feature = "shared")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Open: usize,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
pub trait IWMWriterNetworkSink_Impl: IWMWriterSink_Impl {
    fn SetMaximumClients(&self, dwmaxclients: super::DWORD) -> windows_core::Result<()>;
    fn GetMaximumClients(&self) -> windows_core::Result<super::DWORD>;
    fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> windows_core::Result<()>;
    fn GetNetworkProtocol(&self) -> windows_core::Result<WMT_NET_PROTOCOL>;
    fn GetHostURL(&self, pwszurl: *mut super::WCHAR, pcchurl: *mut super::DWORD) -> windows_core::Result<()>;
    fn Open(&self, pdwportnum: *mut super::DWORD) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl IWMWriterNetworkSink_Vtbl {
    pub const fn new<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMaximumClients<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxclients: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterNetworkSink_Impl::SetMaximumClients(this, core::mem::transmute_copy(&dwmaxclients)).into()
            }
        }
        unsafe extern "system" fn GetMaximumClients<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxclients: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterNetworkSink_Impl::GetMaximumClients(this) {
                    Ok(ok__) => {
                        pdwmaxclients.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkProtocol<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterNetworkSink_Impl::SetNetworkProtocol(this, core::mem::transmute_copy(&protocol)).into()
            }
        }
        unsafe extern "system" fn GetNetworkProtocol<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterNetworkSink_Impl::GetNetworkProtocol(this) {
                    Ok(ok__) => {
                        pprotocol.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHostURL<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: *mut super::WCHAR, pcchurl: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterNetworkSink_Impl::GetHostURL(this, core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pcchurl)).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwportnum: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterNetworkSink_Impl::Open(this, core::mem::transmute_copy(&pdwportnum)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterNetworkSink_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterNetworkSink_Impl::Close(this).into()
            }
        }
        Self {
            base__: IWMWriterSink_Vtbl::new::<Identity, OFFSET>(),
            SetMaximumClients: SetMaximumClients::<Identity, OFFSET>,
            GetMaximumClients: GetMaximumClients::<Identity, OFFSET>,
            SetNetworkProtocol: SetNetworkProtocol::<Identity, OFFSET>,
            GetNetworkProtocol: GetNetworkProtocol::<Identity, OFFSET>,
            GetHostURL: GetHostURL::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterNetworkSink as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterNetworkSink {}
windows_core::imp::define_interface!(IWMWriterPostView, IWMWriterPostView_Vtbl, 0x81e20ce4_75ef_491a_8004_fc53c45bdc3e);
windows_core::imp::interface_hierarchy!(IWMWriterPostView, windows_core::IUnknown);
impl IWMWriterPostView {
    pub unsafe fn SetPostViewCallback(&self, pcallback: &Option<IWMWriterPostViewCallback>, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPostViewCallback)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pcallback), pvcontext as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetReceivePostViewSamples(&self, wstreamnum: super::WORD, freceivepostviewsamples: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReceivePostViewSamples)(windows_core::Interface::as_raw(self), wstreamnum, freceivepostviewsamples.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetReceivePostViewSamples(&self, wstreamnum: super::WORD) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReceivePostViewSamples)(windows_core::Interface::as_raw(self), wstreamnum, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPostViewProps(&self, wstreamnumber: super::WORD) -> windows_core::Result<IWMMediaProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPostViewProps)(windows_core::Interface::as_raw(self), wstreamnumber, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPostViewProps<P1>(&self, wstreamnumber: super::WORD, poutput: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMMediaProps>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPostViewProps)(windows_core::Interface::as_raw(self), wstreamnumber, poutput.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPostViewFormatCount(&self, wstreamnumber: super::WORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPostViewFormatCount)(windows_core::Interface::as_raw(self), wstreamnumber, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPostViewFormat(&self, wstreamnumber: super::WORD, dwformatnumber: super::DWORD) -> windows_core::Result<IWMMediaProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPostViewFormat)(windows_core::Interface::as_raw(self), wstreamnumber, dwformatnumber, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllocateForPostView(&self, wstreamnumber: super::WORD, fallocate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllocateForPostView)(windows_core::Interface::as_raw(self), wstreamnumber, fallocate.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllocateForPostView(&self, wstreamnumber: super::WORD) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllocateForPostView)(windows_core::Interface::as_raw(self), wstreamnumber, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPostViewCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetReceivePostViewSamples: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetReceivePostViewSamples: usize,
    #[cfg(feature = "shared")]
    pub GetReceivePostViewSamples: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetReceivePostViewSamples: usize,
    #[cfg(feature = "shared")]
    pub GetPostViewProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPostViewProps: usize,
    #[cfg(feature = "shared")]
    pub SetPostViewProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPostViewProps: usize,
    #[cfg(feature = "shared")]
    pub GetPostViewFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPostViewFormatCount: usize,
    #[cfg(feature = "shared")]
    pub GetPostViewFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPostViewFormat: usize,
    #[cfg(feature = "shared")]
    pub SetAllocateForPostView: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllocateForPostView: usize,
    #[cfg(feature = "shared")]
    pub GetAllocateForPostView: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllocateForPostView: usize,
}
#[cfg(feature = "shared")]
pub trait IWMWriterPostView_Impl: windows_core::IUnknownImpl {
    fn SetPostViewCallback(&self, pcallback: windows_core::Ref<IWMWriterPostViewCallback>, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetReceivePostViewSamples(&self, wstreamnum: super::WORD, freceivepostviewsamples: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetReceivePostViewSamples(&self, wstreamnum: super::WORD) -> windows_core::Result<windows_core::BOOL>;
    fn GetPostViewProps(&self, wstreamnumber: super::WORD) -> windows_core::Result<IWMMediaProps>;
    fn SetPostViewProps(&self, wstreamnumber: super::WORD, poutput: windows_core::Ref<IWMMediaProps>) -> windows_core::Result<()>;
    fn GetPostViewFormatCount(&self, wstreamnumber: super::WORD) -> windows_core::Result<super::DWORD>;
    fn GetPostViewFormat(&self, wstreamnumber: super::WORD, dwformatnumber: super::DWORD) -> windows_core::Result<IWMMediaProps>;
    fn SetAllocateForPostView(&self, wstreamnumber: super::WORD, fallocate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAllocateForPostView(&self, wstreamnumber: super::WORD) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "shared")]
impl IWMWriterPostView_Vtbl {
    pub const fn new<Identity: IWMWriterPostView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPostViewCallback<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPostView_Impl::SetPostViewCallback(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn SetReceivePostViewSamples<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, freceivepostviewsamples: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPostView_Impl::SetReceivePostViewSamples(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&freceivepostviewsamples)).into()
            }
        }
        unsafe extern "system" fn GetReceivePostViewSamples<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, pfreceivepostviewsamples: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterPostView_Impl::GetReceivePostViewSamples(this, core::mem::transmute_copy(&wstreamnum)) {
                    Ok(ok__) => {
                        pfreceivepostviewsamples.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPostViewProps<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: super::WORD, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterPostView_Impl::GetPostViewProps(this, core::mem::transmute_copy(&wstreamnumber)) {
                    Ok(ok__) => {
                        ppoutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostViewProps<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: super::WORD, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPostView_Impl::SetPostViewProps(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn GetPostViewFormatCount<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: super::WORD, pcformats: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterPostView_Impl::GetPostViewFormatCount(this, core::mem::transmute_copy(&wstreamnumber)) {
                    Ok(ok__) => {
                        pcformats.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPostViewFormat<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: super::WORD, dwformatnumber: super::DWORD, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterPostView_Impl::GetPostViewFormat(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&dwformatnumber)) {
                    Ok(ok__) => {
                        ppprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllocateForPostView<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: super::WORD, fallocate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPostView_Impl::SetAllocateForPostView(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&fallocate)).into()
            }
        }
        unsafe extern "system" fn GetAllocateForPostView<Identity: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: super::WORD, pfallocate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterPostView_Impl::GetAllocateForPostView(this, core::mem::transmute_copy(&wstreamnumber)) {
                    Ok(ok__) => {
                        pfallocate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPostViewCallback: SetPostViewCallback::<Identity, OFFSET>,
            SetReceivePostViewSamples: SetReceivePostViewSamples::<Identity, OFFSET>,
            GetReceivePostViewSamples: GetReceivePostViewSamples::<Identity, OFFSET>,
            GetPostViewProps: GetPostViewProps::<Identity, OFFSET>,
            SetPostViewProps: SetPostViewProps::<Identity, OFFSET>,
            GetPostViewFormatCount: GetPostViewFormatCount::<Identity, OFFSET>,
            GetPostViewFormat: GetPostViewFormat::<Identity, OFFSET>,
            SetAllocateForPostView: SetAllocateForPostView::<Identity, OFFSET>,
            GetAllocateForPostView: GetAllocateForPostView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterPostView as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMWriterPostView {}
windows_core::imp::define_interface!(IWMWriterPostViewCallback, IWMWriterPostViewCallback_Vtbl, 0xd9d6549d_a193_4f24_b308_03123d9b7f8d);
impl core::ops::Deref for IWMWriterPostViewCallback {
    type Target = IWMStatusCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterPostViewCallback, windows_core::IUnknown, IWMStatusCallback);
impl IWMWriterPostViewCallback {
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn OnPostViewSample<P4>(&self, wstreamnumber: super::WORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: P4, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::INSSBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPostViewSample)(windows_core::Interface::as_raw(self), wstreamnumber, cnssampletime, cnssampleduration, dwflags, psample.param().abi(), pvcontext) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn AllocateForPostView(&self, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: *mut Option<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AllocateForPostView)(windows_core::Interface::as_raw(self), wstreamnum, cbbuffer, core::mem::transmute(ppbuffer), pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostViewCallback_Vtbl {
    pub base__: IWMStatusCallback_Vtbl,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub OnPostViewSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::QWORD, super::QWORD, super::DWORD, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    OnPostViewSample: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub AllocateForPostView: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::DWORD, *mut *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    AllocateForPostView: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMWriterPostViewCallback_Impl: IWMStatusCallback_Impl {
    fn OnPostViewSample(&self, wstreamnumber: super::WORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: windows_core::Ref<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateForPostView(&self, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: windows_core::OutRef<super::INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMWriterPostViewCallback_Vtbl {
    pub const fn new<Identity: IWMWriterPostViewCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPostViewSample<Identity: IWMWriterPostViewCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: super::WORD, cnssampletime: super::QWORD, cnssampleduration: super::QWORD, dwflags: super::DWORD, psample: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPostViewCallback_Impl::OnPostViewSample(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&psample), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn AllocateForPostView<Identity: IWMWriterPostViewCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: super::WORD, cbbuffer: super::DWORD, ppbuffer: *mut *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPostViewCallback_Impl::AllocateForPostView(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self {
            base__: IWMStatusCallback_Vtbl::new::<Identity, OFFSET>(),
            OnPostViewSample: OnPostViewSample::<Identity, OFFSET>,
            AllocateForPostView: AllocateForPostView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterPostViewCallback as windows_core::Interface>::IID || iid == &<IWMStatusCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterPostViewCallback {}
windows_core::imp::define_interface!(IWMWriterPreprocess, IWMWriterPreprocess_Vtbl, 0xfc54a285_38c4_45b5_aa23_85b9f7cb424b);
windows_core::imp::interface_hierarchy!(IWMWriterPreprocess, windows_core::IUnknown);
impl IWMWriterPreprocess {
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxPreprocessingPasses(&self, dwinputnum: super::DWORD, dwflags: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxPreprocessingPasses)(windows_core::Interface::as_raw(self), dwinputnum, dwflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetNumPreprocessingPasses(&self, dwinputnum: super::DWORD, dwflags: super::DWORD, dwnumpasses: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNumPreprocessingPasses)(windows_core::Interface::as_raw(self), dwinputnum, dwflags, dwnumpasses) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn BeginPreprocessingPass(&self, dwinputnum: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginPreprocessingPass)(windows_core::Interface::as_raw(self), dwinputnum, dwflags) }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn PreprocessSample<P3>(&self, dwinputnum: super::DWORD, cnssampletime: super::QWORD, dwflags: super::DWORD, psample: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<super::INSSBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).PreprocessSample)(windows_core::Interface::as_raw(self), dwinputnum, cnssampletime, dwflags, psample.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EndPreprocessingPass(&self, dwinputnum: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndPreprocessingPass)(windows_core::Interface::as_raw(self), dwinputnum, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPreprocess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetMaxPreprocessingPasses: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxPreprocessingPasses: usize,
    #[cfg(feature = "shared")]
    pub SetNumPreprocessingPasses: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetNumPreprocessingPasses: usize,
    #[cfg(feature = "shared")]
    pub BeginPreprocessingPass: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    BeginPreprocessingPass: usize,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub PreprocessSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::QWORD, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    PreprocessSample: usize,
    #[cfg(feature = "shared")]
    pub EndPreprocessingPass: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EndPreprocessingPass: usize,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMWriterPreprocess_Impl: windows_core::IUnknownImpl {
    fn GetMaxPreprocessingPasses(&self, dwinputnum: super::DWORD, dwflags: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn SetNumPreprocessingPasses(&self, dwinputnum: super::DWORD, dwflags: super::DWORD, dwnumpasses: super::DWORD) -> windows_core::Result<()>;
    fn BeginPreprocessingPass(&self, dwinputnum: super::DWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn PreprocessSample(&self, dwinputnum: super::DWORD, cnssampletime: super::QWORD, dwflags: super::DWORD, psample: windows_core::Ref<super::INSSBuffer>) -> windows_core::Result<()>;
    fn EndPreprocessingPass(&self, dwinputnum: super::DWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMWriterPreprocess_Vtbl {
    pub const fn new<Identity: IWMWriterPreprocess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaxPreprocessingPasses<Identity: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, dwflags: super::DWORD, pdwmaxnumpasses: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterPreprocess_Impl::GetMaxPreprocessingPasses(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        pdwmaxnumpasses.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNumPreprocessingPasses<Identity: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, dwflags: super::DWORD, dwnumpasses: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPreprocess_Impl::SetNumPreprocessingPasses(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwnumpasses)).into()
            }
        }
        unsafe extern "system" fn BeginPreprocessingPass<Identity: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPreprocess_Impl::BeginPreprocessingPass(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn PreprocessSample<Identity: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, cnssampletime: super::QWORD, dwflags: super::DWORD, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPreprocess_Impl::PreprocessSample(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn EndPreprocessingPass<Identity: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPreprocess_Impl::EndPreprocessingPass(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxPreprocessingPasses: GetMaxPreprocessingPasses::<Identity, OFFSET>,
            SetNumPreprocessingPasses: SetNumPreprocessingPasses::<Identity, OFFSET>,
            BeginPreprocessingPass: BeginPreprocessingPass::<Identity, OFFSET>,
            PreprocessSample: PreprocessSample::<Identity, OFFSET>,
            EndPreprocessingPass: EndPreprocessingPass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterPreprocess as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterPreprocess {}
windows_core::imp::define_interface!(IWMWriterPushSink, IWMWriterPushSink_Vtbl, 0xdc10e6a5_072c_467d_bf57_6330a9dde12a);
impl core::ops::Deref for IWMWriterPushSink {
    type Target = IWMWriterSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMWriterPushSink, windows_core::IUnknown, IWMWriterSink);
impl IWMWriterPushSink {
    pub unsafe fn Connect<P0, P1>(&self, pwszurl: P0, pwsztemplateurl: P1, fautodestroy: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), pwsztemplateurl.param().abi(), fautodestroy.into()) }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndSession(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPushSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMWriterPushSink_Impl: IWMWriterSink_Impl {
    fn Connect(&self, pwszurl: &windows_core::PCWSTR, pwsztemplateurl: &windows_core::PCWSTR, fautodestroy: windows_core::BOOL) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn EndSession(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMWriterPushSink_Vtbl {
    pub const fn new<Identity: IWMWriterPushSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pwsztemplateurl: windows_core::PCWSTR, fautodestroy: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPushSink_Impl::Connect(this, core::mem::transmute(&pwszurl), core::mem::transmute(&pwsztemplateurl), core::mem::transmute_copy(&fautodestroy)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPushSink_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn EndSession<Identity: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterPushSink_Impl::EndSession(this).into()
            }
        }
        Self {
            base__: IWMWriterSink_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            EndSession: EndSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterPushSink as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterPushSink {}
windows_core::imp::define_interface!(IWMWriterSink, IWMWriterSink_Vtbl, 0x96406be4_2b2b_11d3_b36b_00c04f6108ff);
windows_core::imp::interface_hierarchy!(IWMWriterSink, windows_core::IUnknown);
impl IWMWriterSink {
    #[cfg(feature = "wmsbuffer")]
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::INSSBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnHeader)(windows_core::Interface::as_raw(self), pheader.param().abi()) }
    }
    pub unsafe fn IsRealTime(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRealTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: super::DWORD) -> windows_core::Result<super::INSSBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocateDataUnit)(windows_core::Interface::as_raw(self), cbdataunit, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wmsbuffer")]
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::INSSBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDataUnit)(windows_core::Interface::as_raw(self), pdataunit.param().abi()) }
    }
    pub unsafe fn OnEndWriting(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEndWriting)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wmsbuffer")]
    pub OnHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wmsbuffer"))]
    OnHeader: usize,
    pub IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "wmsbuffer"))]
    pub AllocateDataUnit: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wmsbuffer")))]
    AllocateDataUnit: usize,
    #[cfg(feature = "wmsbuffer")]
    pub OnDataUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wmsbuffer"))]
    OnDataUnit: usize,
    pub OnEndWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
pub trait IWMWriterSink_Impl: windows_core::IUnknownImpl {
    fn OnHeader(&self, pheader: windows_core::Ref<super::INSSBuffer>) -> windows_core::Result<()>;
    fn IsRealTime(&self) -> windows_core::Result<windows_core::BOOL>;
    fn AllocateDataUnit(&self, cbdataunit: super::DWORD) -> windows_core::Result<super::INSSBuffer>;
    fn OnDataUnit(&self, pdataunit: windows_core::Ref<super::INSSBuffer>) -> windows_core::Result<()>;
    fn OnEndWriting(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl IWMWriterSink_Vtbl {
    pub const fn new<Identity: IWMWriterSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnHeader<Identity: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterSink_Impl::OnHeader(this, core::mem::transmute_copy(&pheader)).into()
            }
        }
        unsafe extern "system" fn IsRealTime<Identity: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrealtime: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterSink_Impl::IsRealTime(this) {
                    Ok(ok__) => {
                        pfrealtime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AllocateDataUnit<Identity: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbdataunit: super::DWORD, ppdataunit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMWriterSink_Impl::AllocateDataUnit(this, core::mem::transmute_copy(&cbdataunit)) {
                    Ok(ok__) => {
                        ppdataunit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnDataUnit<Identity: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataunit: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterSink_Impl::OnDataUnit(this, core::mem::transmute_copy(&pdataunit)).into()
            }
        }
        unsafe extern "system" fn OnEndWriting<Identity: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMWriterSink_Impl::OnEndWriting(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnHeader: OnHeader::<Identity, OFFSET>,
            IsRealTime: IsRealTime::<Identity, OFFSET>,
            AllocateDataUnit: AllocateDataUnit::<Identity, OFFSET>,
            OnDataUnit: OnDataUnit::<Identity, OFFSET>,
            OnEndWriting: OnEndWriting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wmsbuffer"))]
impl windows_core::RuntimeName for IWMWriterSink {}
pub type LPCWSTR_WMSDK_TYPE_SAFE = *const u16;
pub const WAVE_FORMAT_DRM: i32 = 9;
pub const WEBSTREAM_SAMPLE_TYPE_FILE: i32 = 1;
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: super::DWORD,
    pub cbEncryptedSessionKeyMessage: super::DWORD,
    pub pbEncryptedSessionKeyMessage: *mut super::BYTE,
    pub cbEncryptedKeyMessage: super::DWORD,
    pub pbEncryptedKeyMessage: *mut super::BYTE,
}
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: i32 = 1;
pub const WMFORMAT_MPEG2Video: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const WMFORMAT_Script: windows_core::GUID = windows_core::GUID::from_u128(0x5c8510f2_debe_4ca7_bba5_f07a104f8dff);
pub const WMFORMAT_VideoInfo: windows_core::GUID = windows_core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WaveFormatEx: windows_core::GUID = windows_core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WebStream: windows_core::GUID = windows_core::GUID::from_u128(0xda1e6b13_8359_4050_b398_388e965bf00c);
pub const WMMEDIASUBTYPE_ACELPnet: windows_core::GUID = windows_core::GUID::from_u128(0x00000130_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_Base: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_DRM: windows_core::GUID = windows_core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_I420: windows_core::GUID = windows_core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_IYUV: windows_core::GUID = windows_core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_M4S2: windows_core::GUID = windows_core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP3: windows_core::GUID = windows_core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP43: windows_core::GUID = windows_core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP4S: windows_core::GUID = windows_core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const WMMEDIASUBTYPE_MSS1: windows_core::GUID = windows_core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MSS2: windows_core::GUID = windows_core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_P422: windows_core::GUID = windows_core::GUID::from_u128(0x32323450_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_PCM: windows_core::GUID = windows_core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_RGB1: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb78_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB24: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7d_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB32: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7e_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB4: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb79_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB555: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7c_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB565: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7b_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB8: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7a_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_UYVY: windows_core::GUID = windows_core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_VIDEOIMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x1d4a45f2_e5f6_4b44_8388_f0ae5c0e0c37);
pub const WMMEDIASUBTYPE_WMAudioV2: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV7: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV8: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV9: windows_core::GUID = windows_core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudio_Lossless: windows_core::GUID = windows_core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP1: windows_core::GUID = windows_core::GUID::from_u128(0x0000000a_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP2: windows_core::GUID = windows_core::GUID::from_u128(0x0000000b_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV1: windows_core::GUID = windows_core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV2: windows_core::GUID = windows_core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV3: windows_core::GUID = windows_core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVA: windows_core::GUID = windows_core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVP: windows_core::GUID = windows_core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVC1: windows_core::GUID = windows_core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVP2: windows_core::GUID = windows_core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WebStream: windows_core::GUID = windows_core::GUID::from_u128(0x776257d4_c627_41cb_8f81_7ac7ff1c40cc);
pub const WMMEDIASUBTYPE_YUY2: windows_core::GUID = windows_core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YV12: windows_core::GUID = windows_core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVU9: windows_core::GUID = windows_core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVYU: windows_core::GUID = windows_core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Audio: windows_core::GUID = windows_core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_FileTransfer: windows_core::GUID = windows_core::GUID::from_u128(0xd9e47579_930e_4427_adfc_ad80f290e470);
pub const WMMEDIATYPE_Image: windows_core::GUID = windows_core::GUID::from_u128(0x34a50fd8_8aa5_4386_81fe_a0efe0488e31);
pub const WMMEDIATYPE_Script: windows_core::GUID = windows_core::GUID::from_u128(0x73636d64_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Text: windows_core::GUID = windows_core::GUID::from_u128(0x9bba1ea7_5ab2_4829_ba57_0940209bcf3e);
pub const WMMEDIATYPE_Video: windows_core::GUID = windows_core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMSCRIPTFORMAT {
    pub scriptType: windows_core::GUID,
}
pub const WMSCRIPTTYPE_TwoStrings: windows_core::GUID = windows_core::GUID::from_u128(0x82f38a70_c29f_11d1_97ad_00a0c95ea850);
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: core::mem::ManuallyDrop<Option<super::INSSBuffer>>,
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
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: windows_core::GUID = windows_core::GUID::from_u128(0x65221c5a_fa75_4b39_b50c_06c336b6a3ef);
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: windows_core::GUID = windows_core::GUID::from_u128(0x187cc922_8efc_4404_9daf_63f4830df1bc);
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub bKeepPrevImage: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: super::CLSID,
    pub cbDisplayName: super::UINT,
    pub pwszDisplayName: windows_core::PWSTR,
}
pub type WMT_WATERMARK_ENTRY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: super::WORD,
    pub cbSampleHeaderFixedData: super::WORD,
    pub wVersion: super::WORD,
    pub wReserved: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: super::DWORD,
    pub dwPort: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: super::DWORD,
    pub pwszIPAddress: windows_core::PCWSTR,
    pub pwszPort: windows_core::PCWSTR,
    pub pwszDNSName: windows_core::PCWSTR,
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WM_MEDIA_TYPE {
    pub majortype: windows_core::GUID,
    pub subtype: windows_core::GUID,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
    pub lSampleSize: super::ULONG,
    pub formattype: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub cbFormat: super::ULONG,
    pub pbFormat: *mut super::BYTE,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_PICTURE {
    pub pwszMIMEType: windows_core::PWSTR,
    pub bPictureType: super::BYTE,
    pub pwszDescription: windows_core::PWSTR,
    pub dwDataLen: super::DWORD,
    pub pbData: *mut super::BYTE,
}
pub const WM_PLAYBACK_DRC_HIGH: i32 = 0;
pub const WM_PLAYBACK_DRC_LOW: i32 = 2;
pub const WM_PLAYBACK_DRC_MEDIUM: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: super::WORD,
    pub wPortEnd: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub fMandatory: windows_core::BOOL,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: windows_core::GUID,
    pub cbFormat: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: super::BYTE,
    pub bContentType: super::BYTE,
    pub pwszContentDescriptor: windows_core::PWSTR,
    pub dwLyricsLen: super::DWORD,
    pub pbLyrics: *mut super::BYTE,
}
pub const WM_SampleExtensionGUID_ChromaLocation: windows_core::GUID = windows_core::GUID::from_u128(0x4c5acca0_9276_4b2c_9e4c_a0edefdd217e);
pub const WM_SampleExtensionGUID_ColorSpaceInfo: windows_core::GUID = windows_core::GUID::from_u128(0xf79ada56_30eb_4f2b_9f7a_f24b139a1157);
pub const WM_SampleExtensionGUID_ContentType: windows_core::GUID = windows_core::GUID::from_u128(0xd590dc20_07bc_436c_9cf7_f3bbfbf1a4dc);
pub const WM_SampleExtensionGUID_FileName: windows_core::GUID = windows_core::GUID::from_u128(0xe165ec0e_19ed_45d7_b4a7_25cbd1e28e9b);
pub const WM_SampleExtensionGUID_OutputCleanPoint: windows_core::GUID = windows_core::GUID::from_u128(0xf72a3c6f_6eb4_4ebc_b192_09ad9759e828);
pub const WM_SampleExtensionGUID_PixelAspectRatio: windows_core::GUID = windows_core::GUID::from_u128(0x1b1ee554_f9ea_4bc8_821a_376b74e4c4b8);
pub const WM_SampleExtensionGUID_SampleDuration: windows_core::GUID = windows_core::GUID::from_u128(0xc6bd9450_867f_4907_83a3_c77921b733ad);
pub const WM_SampleExtensionGUID_SampleProtectionSalt: windows_core::GUID = windows_core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
pub const WM_SampleExtensionGUID_Timecode: windows_core::GUID = windows_core::GUID::from_u128(0x399595ec_8667_4e2d_8fdb_98814ce76c1e);
pub const WM_SampleExtensionGUID_UserDataInfo: windows_core::GUID = windows_core::GUID::from_u128(0x732bb4fa_78be_4549_99bd_02db1a55b7a8);
pub const WM_SampleExtension_ChromaLocation_Size: i32 = 1;
pub const WM_SampleExtension_ColorSpaceInfo_Size: i32 = 3;
pub const WM_SampleExtension_ContentType_Size: i32 = 1;
pub const WM_SampleExtension_PixelAspectRatio_Size: i32 = 2;
pub const WM_SampleExtension_SampleDuration_Size: i32 = 2;
pub const WM_SampleExtension_Timecode_Size: i32 = 14;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WM_USER_TEXT {
    pub pwszDescription: windows_core::PWSTR,
    pub pwszText: windows_core::PWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: windows_core::PWSTR,
    pub pwszURL: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const g_wszASFLeakyBucketPairs: windows_core::PCWSTR = windows_core::w!("ASFLeakyBucketPairs");
pub const g_wszAllowInterlacedOutput: windows_core::PCWSTR = windows_core::w!("AllowInterlacedOutput");
pub const g_wszAverageLevel: windows_core::PCWSTR = windows_core::w!("AverageLevel");
pub const g_wszBufferAverage: windows_core::PCWSTR = windows_core::w!("Buffer Average");
pub const g_wszComplexity: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEX");
pub const g_wszComplexityLive: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXLIVE");
pub const g_wszComplexityMax: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXMAX");
pub const g_wszComplexityOffline: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXOFFLINE");
pub const g_wszDecoderComplexityRequested: windows_core::PCWSTR = windows_core::w!("_DECODERCOMPLEXITYPROFILE");
pub const g_wszDedicatedDeliveryThread: windows_core::PCWSTR = windows_core::w!("DedicatedDeliveryThread");
pub const g_wszDeinterlaceMode: windows_core::PCWSTR = windows_core::w!("DeinterlaceMode");
pub const g_wszDeliverOnReceive: windows_core::PCWSTR = windows_core::w!("DeliverOnReceive");
pub const g_wszDeviceConformanceTemplate: windows_core::PCWSTR = windows_core::w!("DeviceConformanceTemplate");
pub const g_wszDynamicRangeControl: windows_core::PCWSTR = windows_core::w!("DynamicRangeControl");
pub const g_wszEDL: windows_core::PCWSTR = windows_core::w!("_EDL");
pub const g_wszEarlyDataDelivery: windows_core::PCWSTR = windows_core::w!("EarlyDataDelivery");
pub const g_wszEnableDiscreteOutput: windows_core::PCWSTR = windows_core::w!("EnableDiscreteOutput");
pub const g_wszEnableFrameInterpolation: windows_core::PCWSTR = windows_core::w!("EnableFrameInterpolation");
pub const g_wszEnableWMAProSPDIFOutput: windows_core::PCWSTR = windows_core::w!("EnableWMAProSPDIFOutput");
pub const g_wszFailSeekOnError: windows_core::PCWSTR = windows_core::w!("FailSeekOnError");
pub const g_wszFixedFrameRate: windows_core::PCWSTR = windows_core::w!("FixedFrameRate");
pub const g_wszFold6To2Channels3: windows_core::PCWSTR = windows_core::w!("Fold6To2Channels3");
pub const g_wszFoldToChannelsTemplate: windows_core::PCWSTR = windows_core::w!("Fold%luTo%luChannels%lu");
pub const g_wszInitialPatternForInverseTelecine: windows_core::PCWSTR = windows_core::w!("InitialPatternForInverseTelecine");
pub const g_wszInterlacedCoding: windows_core::PCWSTR = windows_core::w!("InterlacedCoding");
pub const g_wszIsVBRSupported: windows_core::PCWSTR = windows_core::w!("_ISVBRSUPPORTED");
pub const g_wszJPEGCompressionQuality: windows_core::PCWSTR = windows_core::w!("JPEGCompressionQuality");
pub const g_wszJustInTimeDecode: windows_core::PCWSTR = windows_core::w!("JustInTimeDecode");
pub const g_wszMixedClassMode: windows_core::PCWSTR = windows_core::w!("MixedClassMode");
pub const g_wszMusicClassMode: windows_core::PCWSTR = windows_core::w!("MusicClassMode");
pub const g_wszMusicSpeechClassMode: windows_core::PCWSTR = windows_core::w!("MusicSpeechClassMode");
pub const g_wszNeedsPreviousSample: windows_core::PCWSTR = windows_core::w!("NeedsPreviousSample");
pub const g_wszNumPasses: windows_core::PCWSTR = windows_core::w!("_PASSESUSED");
pub const g_wszOriginalSourceFormatTag: windows_core::PCWSTR = windows_core::w!("_SOURCEFORMATTAG");
pub const g_wszOriginalWaveFormat: windows_core::PCWSTR = windows_core::w!("_ORIGINALWAVEFORMAT");
pub const g_wszPeakValue: windows_core::PCWSTR = windows_core::w!("PeakValue");
pub const g_wszPermitSeeksBeyondEndOfStream: windows_core::PCWSTR = windows_core::w!("PermitSeeksBeyondEndOfStream");
pub const g_wszReloadIndexOnSeek: windows_core::PCWSTR = windows_core::w!("ReloadIndexOnSeek");
pub const g_wszScrambledAudio: windows_core::PCWSTR = windows_core::w!("ScrambledAudio");
pub const g_wszSingleOutputBuffer: windows_core::PCWSTR = windows_core::w!("SingleOutputBuffer");
pub const g_wszSoftwareScaling: windows_core::PCWSTR = windows_core::w!("SoftwareScaling");
pub const g_wszSourceBufferTime: windows_core::PCWSTR = windows_core::w!("SourceBufferTime");
pub const g_wszSourceMaxBytesAtOnce: windows_core::PCWSTR = windows_core::w!("SourceMaxBytesAtOnce");
pub const g_wszSpeakerConfig: windows_core::PCWSTR = windows_core::w!("SpeakerConfig");
pub const g_wszSpeechCaps: windows_core::PCWSTR = windows_core::w!("SpeechFormatCap");
pub const g_wszSpeechClassMode: windows_core::PCWSTR = windows_core::w!("SpeechClassMode");
pub const g_wszStreamLanguage: windows_core::PCWSTR = windows_core::w!("StreamLanguage");
pub const g_wszStreamNumIndexObjects: windows_core::PCWSTR = windows_core::w!("StreamNumIndexObjects");
pub const g_wszUsePacketAtSeekPoint: windows_core::PCWSTR = windows_core::w!("UsePacketAtSeekPoint");
pub const g_wszVBRBitrateMax: windows_core::PCWSTR = windows_core::w!("_RMAX");
pub const g_wszVBRBufferWindowMax: windows_core::PCWSTR = windows_core::w!("_BMAX");
pub const g_wszVBREnabled: windows_core::PCWSTR = windows_core::w!("_VBRENABLED");
pub const g_wszVBRPeak: windows_core::PCWSTR = windows_core::w!("VBR Peak");
pub const g_wszVBRQuality: windows_core::PCWSTR = windows_core::w!("_VBRQUALITY");
pub const g_wszVideoSampleDurations: windows_core::PCWSTR = windows_core::w!("VideoSampleDurations");
pub const g_wszWMADID: windows_core::PCWSTR = windows_core::w!("WM/ADID");
pub const g_wszWMASFPacketCount: windows_core::PCWSTR = windows_core::w!("WM/ASFPacketCount");
pub const g_wszWMASFSecurityObjectsSize: windows_core::PCWSTR = windows_core::w!("WM/ASFSecurityObjectsSize");
pub const g_wszWMAlbumArtist: windows_core::PCWSTR = windows_core::w!("WM/AlbumArtist");
pub const g_wszWMAlbumArtistSort: windows_core::PCWSTR = windows_core::w!("WM/AlbumArtistSort");
pub const g_wszWMAlbumCoverURL: windows_core::PCWSTR = windows_core::w!("WM/AlbumCoverURL");
pub const g_wszWMAlbumTitle: windows_core::PCWSTR = windows_core::w!("WM/AlbumTitle");
pub const g_wszWMAlbumTitleSort: windows_core::PCWSTR = windows_core::w!("WM/AlbumTitleSort");
pub const g_wszWMAspectRatioX: windows_core::PCWSTR = windows_core::w!("AspectRatioX");
pub const g_wszWMAspectRatioY: windows_core::PCWSTR = windows_core::w!("AspectRatioY");
pub const g_wszWMAudioFileURL: windows_core::PCWSTR = windows_core::w!("WM/AudioFileURL");
pub const g_wszWMAudioSourceURL: windows_core::PCWSTR = windows_core::w!("WM/AudioSourceURL");
pub const g_wszWMAuthor: windows_core::PCWSTR = windows_core::w!("Author");
pub const g_wszWMAuthorSort: windows_core::PCWSTR = windows_core::w!("AuthorSort");
pub const g_wszWMAuthorURL: windows_core::PCWSTR = windows_core::w!("WM/AuthorURL");
pub const g_wszWMBannerImageData: windows_core::PCWSTR = windows_core::w!("BannerImageData");
pub const g_wszWMBannerImageType: windows_core::PCWSTR = windows_core::w!("BannerImageType");
pub const g_wszWMBannerImageURL: windows_core::PCWSTR = windows_core::w!("BannerImageURL");
pub const g_wszWMBeatsPerMinute: windows_core::PCWSTR = windows_core::w!("WM/BeatsPerMinute");
pub const g_wszWMBitrate: windows_core::PCWSTR = windows_core::w!("Bitrate");
pub const g_wszWMBroadcast: windows_core::PCWSTR = windows_core::w!("Broadcast");
pub const g_wszWMCategory: windows_core::PCWSTR = windows_core::w!("WM/Category");
pub const g_wszWMCodec: windows_core::PCWSTR = windows_core::w!("WM/Codec");
pub const g_wszWMComposer: windows_core::PCWSTR = windows_core::w!("WM/Composer");
pub const g_wszWMComposerSort: windows_core::PCWSTR = windows_core::w!("WM/ComposerSort");
pub const g_wszWMConductor: windows_core::PCWSTR = windows_core::w!("WM/Conductor");
pub const g_wszWMContainerFormat: windows_core::PCWSTR = windows_core::w!("WM/ContainerFormat");
pub const g_wszWMContentDistributor: windows_core::PCWSTR = windows_core::w!("WM/ContentDistributor");
pub const g_wszWMContentGroupDescription: windows_core::PCWSTR = windows_core::w!("WM/ContentGroupDescription");
pub const g_wszWMCopyright: windows_core::PCWSTR = windows_core::w!("Copyright");
pub const g_wszWMCopyrightURL: windows_core::PCWSTR = windows_core::w!("CopyrightURL");
pub const g_wszWMCurrentBitrate: windows_core::PCWSTR = windows_core::w!("CurrentBitrate");
pub const g_wszWMDRM: windows_core::PCWSTR = windows_core::w!("WM/DRM");
pub const g_wszWMDRM_ContentID: windows_core::PCWSTR = windows_core::w!("DRM_ContentID");
pub const g_wszWMDRM_Flags: windows_core::PCWSTR = windows_core::w!("DRM_Flags");
pub const g_wszWMDRM_HeaderSignPrivKey: windows_core::PCWSTR = windows_core::w!("DRM_HeaderSignPrivKey");
pub const g_wszWMDRM_IndividualizedVersion: windows_core::PCWSTR = windows_core::w!("DRM_IndividualizedVersion");
pub const g_wszWMDRM_KeyID: windows_core::PCWSTR = windows_core::w!("DRM_KeyID");
pub const g_wszWMDRM_KeySeed: windows_core::PCWSTR = windows_core::w!("DRM_KeySeed");
pub const g_wszWMDRM_LASignatureCert: windows_core::PCWSTR = windows_core::w!("DRM_LASignatureCert");
pub const g_wszWMDRM_LASignatureLicSrvCert: windows_core::PCWSTR = windows_core::w!("DRM_LASignatureLicSrvCert");
pub const g_wszWMDRM_LASignaturePrivKey: windows_core::PCWSTR = windows_core::w!("DRM_LASignaturePrivKey");
pub const g_wszWMDRM_LASignatureRootCert: windows_core::PCWSTR = windows_core::w!("DRM_LASignatureRootCert");
pub const g_wszWMDRM_Level: windows_core::PCWSTR = windows_core::w!("DRM_Level");
pub const g_wszWMDRM_LicenseAcqURL: windows_core::PCWSTR = windows_core::w!("DRM_LicenseAcqURL");
pub const g_wszWMDRM_SourceID: windows_core::PCWSTR = windows_core::w!("DRM_SourceID");
pub const g_wszWMDRM_V1LicenseAcqURL: windows_core::PCWSTR = windows_core::w!("DRM_V1LicenseAcqURL");
pub const g_wszWMDVDID: windows_core::PCWSTR = windows_core::w!("WM/DVDID");
pub const g_wszWMDescription: windows_core::PCWSTR = windows_core::w!("Description");
pub const g_wszWMDirector: windows_core::PCWSTR = windows_core::w!("WM/Director");
pub const g_wszWMDuration: windows_core::PCWSTR = windows_core::w!("Duration");
pub const g_wszWMEncodedBy: windows_core::PCWSTR = windows_core::w!("WM/EncodedBy");
pub const g_wszWMEncodingSettings: windows_core::PCWSTR = windows_core::w!("WM/EncodingSettings");
pub const g_wszWMEncodingTime: windows_core::PCWSTR = windows_core::w!("WM/EncodingTime");
pub const g_wszWMEpisodeNumber: windows_core::PCWSTR = windows_core::w!("WM/EpisodeNumber");
pub const g_wszWMFileSize: windows_core::PCWSTR = windows_core::w!("FileSize");
pub const g_wszWMGenre: windows_core::PCWSTR = windows_core::w!("WM/Genre");
pub const g_wszWMGenreID: windows_core::PCWSTR = windows_core::w!("WM/GenreID");
pub const g_wszWMHasArbitraryDataStream: windows_core::PCWSTR = windows_core::w!("HasArbitraryDataStream");
pub const g_wszWMHasAttachedImages: windows_core::PCWSTR = windows_core::w!("HasAttachedImages");
pub const g_wszWMHasAudio: windows_core::PCWSTR = windows_core::w!("HasAudio");
pub const g_wszWMHasFileTransferStream: windows_core::PCWSTR = windows_core::w!("HasFileTransferStream");
pub const g_wszWMHasImage: windows_core::PCWSTR = windows_core::w!("HasImage");
pub const g_wszWMHasScript: windows_core::PCWSTR = windows_core::w!("HasScript");
pub const g_wszWMHasVideo: windows_core::PCWSTR = windows_core::w!("HasVideo");
pub const g_wszWMISAN: windows_core::PCWSTR = windows_core::w!("WM/ISAN");
pub const g_wszWMISRC: windows_core::PCWSTR = windows_core::w!("WM/ISRC");
pub const g_wszWMInitialKey: windows_core::PCWSTR = windows_core::w!("WM/InitialKey");
pub const g_wszWMIsCompilation: windows_core::PCWSTR = windows_core::w!("WM/IsCompilation");
pub const g_wszWMIsVBR: windows_core::PCWSTR = windows_core::w!("IsVBR");
pub const g_wszWMLanguage: windows_core::PCWSTR = windows_core::w!("WM/Language");
pub const g_wszWMLyrics: windows_core::PCWSTR = windows_core::w!("WM/Lyrics");
pub const g_wszWMLyrics_Synchronised: windows_core::PCWSTR = windows_core::w!("WM/Lyrics_Synchronised");
pub const g_wszWMMCDI: windows_core::PCWSTR = windows_core::w!("WM/MCDI");
pub const g_wszWMMediaClassPrimaryID: windows_core::PCWSTR = windows_core::w!("WM/MediaClassPrimaryID");
pub const g_wszWMMediaClassSecondaryID: windows_core::PCWSTR = windows_core::w!("WM/MediaClassSecondaryID");
pub const g_wszWMMediaCredits: windows_core::PCWSTR = windows_core::w!("WM/MediaCredits");
pub const g_wszWMMediaIsDelay: windows_core::PCWSTR = windows_core::w!("WM/MediaIsDelay");
pub const g_wszWMMediaIsFinale: windows_core::PCWSTR = windows_core::w!("WM/MediaIsFinale");
pub const g_wszWMMediaIsLive: windows_core::PCWSTR = windows_core::w!("WM/MediaIsLive");
pub const g_wszWMMediaIsPremiere: windows_core::PCWSTR = windows_core::w!("WM/MediaIsPremiere");
pub const g_wszWMMediaIsRepeat: windows_core::PCWSTR = windows_core::w!("WM/MediaIsRepeat");
pub const g_wszWMMediaIsSAP: windows_core::PCWSTR = windows_core::w!("WM/MediaIsSAP");
pub const g_wszWMMediaIsStereo: windows_core::PCWSTR = windows_core::w!("WM/MediaIsStereo");
pub const g_wszWMMediaIsSubtitled: windows_core::PCWSTR = windows_core::w!("WM/MediaIsSubtitled");
pub const g_wszWMMediaIsTape: windows_core::PCWSTR = windows_core::w!("WM/MediaIsTape");
pub const g_wszWMMediaNetworkAffiliation: windows_core::PCWSTR = windows_core::w!("WM/MediaNetworkAffiliation");
pub const g_wszWMMediaOriginalBroadcastDateTime: windows_core::PCWSTR = windows_core::w!("WM/MediaOriginalBroadcastDateTime");
pub const g_wszWMMediaOriginalChannel: windows_core::PCWSTR = windows_core::w!("WM/MediaOriginalChannel");
pub const g_wszWMMediaStationCallSign: windows_core::PCWSTR = windows_core::w!("WM/MediaStationCallSign");
pub const g_wszWMMediaStationName: windows_core::PCWSTR = windows_core::w!("WM/MediaStationName");
pub const g_wszWMModifiedBy: windows_core::PCWSTR = windows_core::w!("WM/ModifiedBy");
pub const g_wszWMMood: windows_core::PCWSTR = windows_core::w!("WM/Mood");
pub const g_wszWMNSCAddress: windows_core::PCWSTR = windows_core::w!("NSC_Address");
pub const g_wszWMNSCDescription: windows_core::PCWSTR = windows_core::w!("NSC_Description");
pub const g_wszWMNSCEmail: windows_core::PCWSTR = windows_core::w!("NSC_Email");
pub const g_wszWMNSCName: windows_core::PCWSTR = windows_core::w!("NSC_Name");
pub const g_wszWMNSCPhone: windows_core::PCWSTR = windows_core::w!("NSC_Phone");
pub const g_wszWMNumberOfFrames: windows_core::PCWSTR = windows_core::w!("NumberOfFrames");
pub const g_wszWMOptimalBitrate: windows_core::PCWSTR = windows_core::w!("OptimalBitrate");
pub const g_wszWMOriginalAlbumTitle: windows_core::PCWSTR = windows_core::w!("WM/OriginalAlbumTitle");
pub const g_wszWMOriginalArtist: windows_core::PCWSTR = windows_core::w!("WM/OriginalArtist");
pub const g_wszWMOriginalFilename: windows_core::PCWSTR = windows_core::w!("WM/OriginalFilename");
pub const g_wszWMOriginalLyricist: windows_core::PCWSTR = windows_core::w!("WM/OriginalLyricist");
pub const g_wszWMOriginalReleaseTime: windows_core::PCWSTR = windows_core::w!("WM/OriginalReleaseTime");
pub const g_wszWMOriginalReleaseYear: windows_core::PCWSTR = windows_core::w!("WM/OriginalReleaseYear");
pub const g_wszWMParentalRating: windows_core::PCWSTR = windows_core::w!("WM/ParentalRating");
pub const g_wszWMParentalRatingReason: windows_core::PCWSTR = windows_core::w!("WM/ParentalRatingReason");
pub const g_wszWMPartOfSet: windows_core::PCWSTR = windows_core::w!("WM/PartOfSet");
pub const g_wszWMPeakBitrate: windows_core::PCWSTR = windows_core::w!("WM/PeakBitrate");
pub const g_wszWMPeriod: windows_core::PCWSTR = windows_core::w!("WM/Period");
pub const g_wszWMPicture: windows_core::PCWSTR = windows_core::w!("WM/Picture");
pub const g_wszWMPlaylistDelay: windows_core::PCWSTR = windows_core::w!("WM/PlaylistDelay");
pub const g_wszWMProducer: windows_core::PCWSTR = windows_core::w!("WM/Producer");
pub const g_wszWMPromotionURL: windows_core::PCWSTR = windows_core::w!("WM/PromotionURL");
pub const g_wszWMProtected: windows_core::PCWSTR = windows_core::w!("Is_Protected");
pub const g_wszWMProtectionType: windows_core::PCWSTR = windows_core::w!("WM/ProtectionType");
pub const g_wszWMProvider: windows_core::PCWSTR = windows_core::w!("WM/Provider");
pub const g_wszWMProviderCopyright: windows_core::PCWSTR = windows_core::w!("WM/ProviderCopyright");
pub const g_wszWMProviderRating: windows_core::PCWSTR = windows_core::w!("WM/ProviderRating");
pub const g_wszWMProviderStyle: windows_core::PCWSTR = windows_core::w!("WM/ProviderStyle");
pub const g_wszWMPublisher: windows_core::PCWSTR = windows_core::w!("WM/Publisher");
pub const g_wszWMRadioStationName: windows_core::PCWSTR = windows_core::w!("WM/RadioStationName");
pub const g_wszWMRadioStationOwner: windows_core::PCWSTR = windows_core::w!("WM/RadioStationOwner");
pub const g_wszWMRating: windows_core::PCWSTR = windows_core::w!("Rating");
pub const g_wszWMSeasonNumber: windows_core::PCWSTR = windows_core::w!("WM/SeasonNumber");
pub const g_wszWMSeekable: windows_core::PCWSTR = windows_core::w!("Seekable");
pub const g_wszWMSharedUserRating: windows_core::PCWSTR = windows_core::w!("WM/SharedUserRating");
pub const g_wszWMSignature_Name: windows_core::PCWSTR = windows_core::w!("Signature_Name");
pub const g_wszWMSkipBackward: windows_core::PCWSTR = windows_core::w!("Can_Skip_Backward");
pub const g_wszWMSkipForward: windows_core::PCWSTR = windows_core::w!("Can_Skip_Forward");
pub const g_wszWMStreamTypeInfo: windows_core::PCWSTR = windows_core::w!("WM/StreamTypeInfo");
pub const g_wszWMStridable: windows_core::PCWSTR = windows_core::w!("Stridable");
pub const g_wszWMSubTitle: windows_core::PCWSTR = windows_core::w!("WM/SubTitle");
pub const g_wszWMSubTitleDescription: windows_core::PCWSTR = windows_core::w!("WM/SubTitleDescription");
pub const g_wszWMSubscriptionContentID: windows_core::PCWSTR = windows_core::w!("WM/SubscriptionContentID");
pub const g_wszWMText: windows_core::PCWSTR = windows_core::w!("WM/Text");
pub const g_wszWMTitle: windows_core::PCWSTR = windows_core::w!("Title");
pub const g_wszWMTitleSort: windows_core::PCWSTR = windows_core::w!("TitleSort");
pub const g_wszWMToolName: windows_core::PCWSTR = windows_core::w!("WM/ToolName");
pub const g_wszWMToolVersion: windows_core::PCWSTR = windows_core::w!("WM/ToolVersion");
pub const g_wszWMTrack: windows_core::PCWSTR = windows_core::w!("WM/Track");
pub const g_wszWMTrackNumber: windows_core::PCWSTR = windows_core::w!("WM/TrackNumber");
pub const g_wszWMTrusted: windows_core::PCWSTR = windows_core::w!("Is_Trusted");
pub const g_wszWMUniqueFileIdentifier: windows_core::PCWSTR = windows_core::w!("WM/UniqueFileIdentifier");
pub const g_wszWMUse_Advanced_DRM: windows_core::PCWSTR = windows_core::w!("Use_Advanced_DRM");
pub const g_wszWMUse_DRM: windows_core::PCWSTR = windows_core::w!("Use_DRM");
pub const g_wszWMUserWebURL: windows_core::PCWSTR = windows_core::w!("WM/UserWebURL");
pub const g_wszWMVideoClosedCaptioning: windows_core::PCWSTR = windows_core::w!("WM/VideoClosedCaptioning");
pub const g_wszWMVideoFrameRate: windows_core::PCWSTR = windows_core::w!("WM/VideoFrameRate");
pub const g_wszWMVideoHeight: windows_core::PCWSTR = windows_core::w!("WM/VideoHeight");
pub const g_wszWMVideoWidth: windows_core::PCWSTR = windows_core::w!("WM/VideoWidth");
pub const g_wszWMWMADRCAverageReference: windows_core::PCWSTR = windows_core::w!("WM/WMADRCAverageReference");
pub const g_wszWMWMADRCAverageTarget: windows_core::PCWSTR = windows_core::w!("WM/WMADRCAverageTarget");
pub const g_wszWMWMADRCPeakReference: windows_core::PCWSTR = windows_core::w!("WM/WMADRCPeakReference");
pub const g_wszWMWMADRCPeakTarget: windows_core::PCWSTR = windows_core::w!("WM/WMADRCPeakTarget");
pub const g_wszWMWMCPDistributor: windows_core::PCWSTR = windows_core::w!("WM/WMCPDistributor");
pub const g_wszWMWMCPDistributorID: windows_core::PCWSTR = windows_core::w!("WM/WMCPDistributorID");
pub const g_wszWMWMCollectionGroupID: windows_core::PCWSTR = windows_core::w!("WM/WMCollectionGroupID");
pub const g_wszWMWMCollectionID: windows_core::PCWSTR = windows_core::w!("WM/WMCollectionID");
pub const g_wszWMWMContentID: windows_core::PCWSTR = windows_core::w!("WM/WMContentID");
pub const g_wszWMWMShadowFileSourceDRMType: windows_core::PCWSTR = windows_core::w!("WM/WMShadowFileSourceDRMType");
pub const g_wszWMWMShadowFileSourceFileType: windows_core::PCWSTR = windows_core::w!("WM/WMShadowFileSourceFileType");
pub const g_wszWMWriter: windows_core::PCWSTR = windows_core::w!("WM/Writer");
pub const g_wszWMYear: windows_core::PCWSTR = windows_core::w!("WM/Year");
pub const g_wszWatermarkCLSID: windows_core::PCWSTR = windows_core::w!("WatermarkCLSID");
pub const g_wszWatermarkConfig: windows_core::PCWSTR = windows_core::w!("WatermarkConfig");
