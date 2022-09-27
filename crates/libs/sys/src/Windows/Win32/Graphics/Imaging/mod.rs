#[cfg(feature = "Win32_Graphics_Imaging_D2D")]
pub mod D2D;
#[cfg_attr(windows, link(name = "windows"))]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    pub fn WICConvertBitmapSource(dstformat: *const ::windows_sys::core::GUID, pisrc: IWICBitmapSource, ppidst: *mut IWICBitmapSource) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICCreateBitmapFromSection(width: u32, height: u32, pixelformat: *const ::windows_sys::core::GUID, hsection: super::super::Foundation::HANDLE, stride: u32, offset: u32, ppibitmap: *mut IWICBitmap) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICCreateBitmapFromSectionEx(width: u32, height: u32, pixelformat: *const ::windows_sys::core::GUID, hsection: super::super::Foundation::HANDLE, stride: u32, offset: u32, desiredaccesslevel: WICSectionAccessLevel, ppibitmap: *mut IWICBitmap) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    pub fn WICGetMetadataContentSize(guidcontainerformat: *const ::windows_sys::core::GUID, piwriter: IWICMetadataWriter, pcbsize: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    pub fn WICMapGuidToShortName(guid: *const ::windows_sys::core::GUID, cchname: u32, wzname: ::windows_sys::core::PWSTR, pcchactual: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    pub fn WICMapSchemaToName(guidmetadataformat: *const ::windows_sys::core::GUID, pwzschema: ::windows_sys::core::PCWSTR, cchname: u32, wzname: ::windows_sys::core::PWSTR, pcchactual: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    pub fn WICMapShortNameToGuid(wzname: ::windows_sys::core::PCWSTR, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WICMatchMetadataContent(guidcontainerformat: *const ::windows_sys::core::GUID, pguidvendor: *const ::windows_sys::core::GUID, pistream: super::super::System::Com::IStream, pguidmetadataformat: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WICSerializeMetadataContent(guidcontainerformat: *const ::windows_sys::core::GUID, piwriter: IWICMetadataWriter, dwpersistoptions: u32, pistream: super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
}
pub type IWICBitmap = *mut ::core::ffi::c_void;
pub type IWICBitmapClipper = *mut ::core::ffi::c_void;
pub type IWICBitmapCodecInfo = *mut ::core::ffi::c_void;
pub type IWICBitmapCodecProgressNotification = *mut ::core::ffi::c_void;
pub type IWICBitmapDecoder = *mut ::core::ffi::c_void;
pub type IWICBitmapDecoderInfo = *mut ::core::ffi::c_void;
pub type IWICBitmapEncoder = *mut ::core::ffi::c_void;
pub type IWICBitmapEncoderInfo = *mut ::core::ffi::c_void;
pub type IWICBitmapFlipRotator = *mut ::core::ffi::c_void;
pub type IWICBitmapFrameDecode = *mut ::core::ffi::c_void;
pub type IWICBitmapFrameEncode = *mut ::core::ffi::c_void;
pub type IWICBitmapLock = *mut ::core::ffi::c_void;
pub type IWICBitmapScaler = *mut ::core::ffi::c_void;
pub type IWICBitmapSource = *mut ::core::ffi::c_void;
pub type IWICBitmapSourceTransform = *mut ::core::ffi::c_void;
pub type IWICColorContext = *mut ::core::ffi::c_void;
pub type IWICColorTransform = *mut ::core::ffi::c_void;
pub type IWICComponentFactory = *mut ::core::ffi::c_void;
pub type IWICComponentInfo = *mut ::core::ffi::c_void;
pub type IWICDdsDecoder = *mut ::core::ffi::c_void;
pub type IWICDdsEncoder = *mut ::core::ffi::c_void;
pub type IWICDdsFrameDecode = *mut ::core::ffi::c_void;
pub type IWICDevelopRaw = *mut ::core::ffi::c_void;
pub type IWICDevelopRawNotificationCallback = *mut ::core::ffi::c_void;
pub type IWICEnumMetadataItem = *mut ::core::ffi::c_void;
pub type IWICFastMetadataEncoder = *mut ::core::ffi::c_void;
pub type IWICFormatConverter = *mut ::core::ffi::c_void;
pub type IWICFormatConverterInfo = *mut ::core::ffi::c_void;
pub type IWICImagingFactory = *mut ::core::ffi::c_void;
pub type IWICJpegFrameDecode = *mut ::core::ffi::c_void;
pub type IWICJpegFrameEncode = *mut ::core::ffi::c_void;
pub type IWICMetadataBlockReader = *mut ::core::ffi::c_void;
pub type IWICMetadataBlockWriter = *mut ::core::ffi::c_void;
pub type IWICMetadataHandlerInfo = *mut ::core::ffi::c_void;
pub type IWICMetadataQueryReader = *mut ::core::ffi::c_void;
pub type IWICMetadataQueryWriter = *mut ::core::ffi::c_void;
pub type IWICMetadataReader = *mut ::core::ffi::c_void;
pub type IWICMetadataReaderInfo = *mut ::core::ffi::c_void;
pub type IWICMetadataWriter = *mut ::core::ffi::c_void;
pub type IWICMetadataWriterInfo = *mut ::core::ffi::c_void;
pub type IWICPalette = *mut ::core::ffi::c_void;
pub type IWICPersistStream = *mut ::core::ffi::c_void;
pub type IWICPixelFormatInfo = *mut ::core::ffi::c_void;
pub type IWICPixelFormatInfo2 = *mut ::core::ffi::c_void;
pub type IWICPlanarBitmapFrameEncode = *mut ::core::ffi::c_void;
pub type IWICPlanarBitmapSourceTransform = *mut ::core::ffi::c_void;
pub type IWICPlanarFormatConverter = *mut ::core::ffi::c_void;
pub type IWICProgressCallback = *mut ::core::ffi::c_void;
pub type IWICProgressiveLevelControl = *mut ::core::ffi::c_void;
pub type IWICStream = *mut ::core::ffi::c_void;
pub type IWICStreamProvider = *mut ::core::ffi::c_void;
pub const CATID_WICBitmapDecoders: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7ed96837_96f0_4812_b211_f13c24117ed3);
pub const CATID_WICBitmapEncoders: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac757296_3522_4e11_9862_c17be5a1767e);
pub const CATID_WICFormatConverters: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7835eae8_bf14_49d1_93ce_533a407b2248);
pub const CATID_WICMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05af94d8_7174_4cd2_be4a_4124b80ee4b8);
pub const CATID_WICMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xabe3b9a4_257d_4b97_bd1a_294af496222e);
pub const CATID_WICPixelFormats: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2b46e70f_cda7_473e_89f6_dc9630a2390b);
pub const CLSID_WIC8BIMIPTCDigestMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x02805f1e_d5aa_415b_82c5_61c033a988a6);
pub const CLSID_WIC8BIMIPTCDigestMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2db5e62b_0d67_495f_8f9d_c2f0188647ac);
pub const CLSID_WIC8BIMIPTCMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0010668c_0801_4da6_a4a4_826522b6d28f);
pub const CLSID_WIC8BIMIPTCMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00108226_ee41_44a2_9e9c_4be4d5b1d2cd);
pub const CLSID_WIC8BIMResolutionInfoMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5805137a_e348_4f7c_b3cc_6db9965a0599);
pub const CLSID_WIC8BIMResolutionInfoMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4ff2fe0e_e74a_4b71_98c4_ab7dc16707ba);
pub const CLSID_WICAPEMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1767b93a_b021_44ea_920f_863c11f4f768);
pub const CLSID_WICAPEMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbd6edfca_2890_482f_b233_8d7339a1cf8d);
pub const CLSID_WICAdngDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x981d9411_909e_42a7_8f5d_a747ff052edb);
pub const CLSID_WICApp0MetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x43324b33_a78f_480f_9111_9638aaccc832);
pub const CLSID_WICApp0MetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf3c633a2_46c8_498e_8fbb_cc6f721bbcde);
pub const CLSID_WICApp13MetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaa7e3c50_864c_4604_bc04_8b0b76e637f6);
pub const CLSID_WICApp13MetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b19a919_a9d6_49e5_bd45_02c34e4e4cd5);
pub const CLSID_WICApp1MetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdde33513_774e_4bcd_ae79_02f4adfe62fc);
pub const CLSID_WICApp1MetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xee366069_1832_420f_b381_0479ad066f19);
pub const CLSID_WICBmpDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6b462062_7cbf_400d_9fdb_813dd10f2778);
pub const CLSID_WICBmpEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x69be8bb4_d66d_47c8_865a_ed1589433782);
pub const CLSID_WICDdsDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9053699f_a341_429d_9e90_ee437cf80c73);
pub const CLSID_WICDdsEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa61dde94_66ce_4ac1_881b_71680588895e);
pub const CLSID_WICDdsMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x276c88ca_7533_4a86_b676_66b36080d484);
pub const CLSID_WICDdsMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfd688bbd_31ed_4db7_a723_934927d38367);
pub const CLSID_WICDefaultFormatConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1a3f11dc_b514_4b17_8c5f_2154513852f1);
pub const CLSID_WICExifMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd9403860_297f_4a49_bf9b_77898150a442);
pub const CLSID_WICExifMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc9a14cda_c339_460b_9078_d4debcfabe91);
pub const CLSID_WICFormatConverterHighColor: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac75d454_9f37_48f8_b972_4e19bc856011);
pub const CLSID_WICFormatConverterNChannel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc17cabb2_d4a3_47d7_a557_339b2efbd4f1);
pub const CLSID_WICFormatConverterWMPhoto: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9cb5172b_d600_46ba_ab77_77bb7e3a00d9);
pub const CLSID_WICGCEMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb92e345d_f52d_41f3_b562_081bc772e3b9);
pub const CLSID_WICGCEMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaf95dc76_16b2_47f4_b3ea_3c31796693e7);
pub const CLSID_WICGifCommentMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32557d3b_69dc_4f95_836e_f5972b2f6159);
pub const CLSID_WICGifCommentMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa02797fc_c4ae_418c_af95_e637c7ead2a1);
pub const CLSID_WICGifDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x381dda3c_9ce9_4834_a23e_1f98f8fc52be);
pub const CLSID_WICGifEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x114f5598_0b22_40a0_86a1_c83ea495adbd);
pub const CLSID_WICGpsMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3697790b_223b_484e_9925_c4869218f17a);
pub const CLSID_WICGpsMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcb8c13e4_62b5_4c96_a48b_6ba6ace39c76);
pub const CLSID_WICHeifDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe9a4a80a_44fe_4de4_8971_7150b10a5199);
pub const CLSID_WICHeifEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0dbecec1_9eb3_4860_9c6f_ddbe86634575);
pub const CLSID_WICHeifHDRMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2438de3d_94d9_4be8_84a8_4de95a575e75);
pub const CLSID_WICHeifMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xacddfc3f_85ec_41bc_bdef_1bc262e4db05);
pub const CLSID_WICHeifMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3ae45e79_40bc_4401_ace5_dd3cb16e6afe);
pub const CLSID_WICIMDMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7447a267_0015_42c8_a8f1_fb3b94c68361);
pub const CLSID_WICIMDMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8c89071f_452e_4e95_9682_9d1024627172);
pub const CLSID_WICIPTCMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x03012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICIPTCMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1249b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICIRBMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd4dcd3d7_b4c2_47d9_a6bf_b89ba396a4a3);
pub const CLSID_WICIRBMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5c5c1935_0235_4434_80bc_251bc1ec39c6);
pub const CLSID_WICIcoDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc61bfcdf_2e0f_4aad_a8d7_e06bafebcdfe);
pub const CLSID_WICIfdMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8f914656_9d0a_4eb2_9019_0bf96d8a9ee6);
pub const CLSID_WICIfdMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb1ebfc28_c9bd_47a2_8d33_b948769777a7);
pub const CLSID_WICImagingCategories: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfae3d380_fea4_4623_8c75_c6b61110b681);
pub const CLSID_WICImagingFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x317d06e8_5f24_433d_bdf7_79ce68d8abc2);
pub const CLSID_WICInteropMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb5c8b898_0074_459f_b700_860d4651ea14);
pub const CLSID_WICInteropMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x122ec645_cd7e_44d8_b186_2c8c20c3b50f);
pub const CLSID_WICJpegChrominanceMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50b1904b_f28f_4574_93f4_0bade82c69e9);
pub const CLSID_WICJpegChrominanceMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3ff566f0_6e6b_49d4_96e6_b78886692c62);
pub const CLSID_WICJpegCommentMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9f66347c_60c4_4c4d_ab58_d2358685f607);
pub const CLSID_WICJpegCommentMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe573236f_55b1_4eda_81ea_9f65db0290d3);
pub const CLSID_WICJpegDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9456a480_e88b_43ea_9e73_0b2d9b71b1ca);
pub const CLSID_WICJpegEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1a34f5c1_4a5a_46dc_b644_1f4567e7a676);
pub const CLSID_WICJpegLuminanceMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x356f2f88_05a6_4728_b9a4_1bfbce04d838);
pub const CLSID_WICJpegLuminanceMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1d583abc_8a0e_4657_9982_a380ca58fb4b);
pub const CLSID_WICJpegQualcommPhoneEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x68ed5c62_f534_4979_b2b3_686a12b2b34c);
pub const CLSID_WICLSDMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x41070793_59e4_479a_a1f7_954adc2ef5fc);
pub const CLSID_WICLSDMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73c037e7_e5d9_4954_876a_6da81d6e5768);
pub const CLSID_WICPlanarFormatConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x184132b8_32f8_4784_9131_dd7224b23438);
pub const CLSID_WICPngBkgdMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0ce7a4a6_03e8_4a60_9d15_282ef32ee7da);
pub const CLSID_WICPngBkgdMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x68e3f2fd_31ae_4441_bb6a_fd7047525f90);
pub const CLSID_WICPngChrmMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf90b5f36_367b_402a_9dd1_bc0fd59d8f62);
pub const CLSID_WICPngChrmMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe23ce3eb_5608_4e83_bcef_27b1987e51d7);
pub const CLSID_WICPngDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe018945b_aa86_4008_9bd4_6777a1e40c11);
pub const CLSID_WICPngEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x27949969_876a_41d7_9447_568f6a35a4dc);
pub const CLSID_WICPngGamaMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3692ca39_e082_4350_9e1f_3704cb083cd5);
pub const CLSID_WICPngGamaMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xff036d13_5d4b_46dd_b10f_106693d9fe4f);
pub const CLSID_WICPngHistMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x877a0bb7_a313_4491_87b5_2e6d0594f520);
pub const CLSID_WICPngHistMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8a03e749_672e_446e_bf1f_2c11d233b6ff);
pub const CLSID_WICPngIccpMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf5d3e63b_cb0f_4628_a478_6d8244be36b1);
pub const CLSID_WICPngIccpMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x16671e5f_0ce6_4cc4_9768_e89fe5018ade);
pub const CLSID_WICPngItxtMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaabfb2fa_3e1e_4a8f_8977_5556fb94ea23);
pub const CLSID_WICPngItxtMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31879719_e751_4df8_981d_68dff67704ed);
pub const CLSID_WICPngSrgbMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb40360c_547e_4956_a3b9_d4418859ba66);
pub const CLSID_WICPngSrgbMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa6ee35c6_87ec_47df_9f22_1d5aad840c82);
pub const CLSID_WICPngTextMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4b59afcc_b8c3_408a_b670_89e5fab6fda7);
pub const CLSID_WICPngTextMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb5ebafb9_253e_4a72_a744_0762d2685683);
pub const CLSID_WICPngTimeMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd94edf02_efe5_4f0d_85c8_f5a68b3000b1);
pub const CLSID_WICPngTimeMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1ab78400_b5a3_4d91_8ace_33fcd1499be6);
pub const CLSID_WICRAWDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x41945702_8302_44a6_9445_ac98e8afa086);
pub const CLSID_WICSubIfdMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50d42f09_ecd1_4b41_b65d_da1fdaa75663);
pub const CLSID_WICSubIfdMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8ade5386_8e9b_4f4c_acf2_f0008706b238);
pub const CLSID_WICThumbnailMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICThumbnailMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd049b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICTiffDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb54e85d9_fe23_499f_8b88_6acea713752b);
pub const CLSID_WICTiffEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0131be10_2001_4c5f_a9b0_cc88fab64ce8);
pub const CLSID_WICUnknownMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x699745c2_5066_4b82_a8e3_d40478dbec8c);
pub const CLSID_WICUnknownMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa09cca86_27ba_4f39_9053_121fa4dc08fc);
pub const CLSID_WICWebpAnimMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x076f9911_a348_465c_a807_a252f3f2d3de);
pub const CLSID_WICWebpAnmfMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x85a10b03_c9f6_439f_be5e_c0fbef67807c);
pub const CLSID_WICWebpDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7693e886_51c9_4070_8419_9f70738ec8fa);
pub const CLSID_WICWmpDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa26cec36_234c_4950_ae16_e34aace71d0d);
pub const CLSID_WICWmpEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac4ce3cb_e1c1_44cd_8215_5a1665509ec2);
pub const CLSID_WICXMPAltMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaa94dcc2_b8b0_4898_b835_000aabd74393);
pub const CLSID_WICXMPAltMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x076c2a6c_f78f_4c46_a723_3583e70876ea);
pub const CLSID_WICXMPBagMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe7e79a30_4f2c_4fab_8d00_394f2d6bbebe);
pub const CLSID_WICXMPBagMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xed822c8c_d6be_4301_a631_0e1416bad28f);
pub const CLSID_WICXMPMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72b624df_ae11_4948_a65c_351eb0829419);
pub const CLSID_WICXMPMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1765e14e_1bd4_462e_b6b1_590bf1262ac6);
pub const CLSID_WICXMPSeqMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7f12e753_fc71_43d7_a51d_92f35977abb5);
pub const CLSID_WICXMPSeqMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d68d1de_d432_4b0f_923a_091183a9bda7);
pub const CLSID_WICXMPStructMetadataReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x01b90d9a_8209_47f7_9c52_e1244bf50ced);
pub const CLSID_WICXMPStructMetadataWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x22c21f93_7ddb_411c_9b17_c5b7bd064abc);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const FACILITY_WINCODEC_ERR: u32 = 2200u32;
pub const GUID_ContainerFormatAdng: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf3ff6d0d_38c0_41c4_b1fe_1f3824f17b84);
pub const GUID_ContainerFormatBmp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0af1d87e_fcfe_4188_bdeb_a7906471cbe3);
pub const GUID_ContainerFormatDds: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9967cb95_2e85_4ac8_8ca2_83d7ccd425c9);
pub const GUID_ContainerFormatGif: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1f8a5601_7d4d_4cbd_9c82_1bc8d4eeb9a5);
pub const GUID_ContainerFormatHeif: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe1e62521_6787_405b_a339_500715b5763f);
pub const GUID_ContainerFormatIco: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa3a860c4_338f_4c17_919a_fba4b5628f21);
pub const GUID_ContainerFormatJpeg: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const GUID_ContainerFormatPng: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b7cfaf4_713f_473c_bbcd_6137425faeaf);
pub const GUID_ContainerFormatRaw: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfe99ce60_f19c_433c_a3ae_00acefa9ca21);
pub const GUID_ContainerFormatTiff: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x163bcc30_e2e9_4f0b_961d_a3e9fdb788a3);
pub const GUID_ContainerFormatWebp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe094b0e2_67f2_45b3_b0ea_115337ca7cf3);
pub const GUID_ContainerFormatWmp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57a37caa_367a_4540_916b_f183c5093a4b);
pub const GUID_MetadataFormat8BIMIPTC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0010568c_0852_4e6a_b191_5c33ac5b0430);
pub const GUID_MetadataFormat8BIMIPTCDigest: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1ca32285_9ccd_4786_8bd8_79539db6a006);
pub const GUID_MetadataFormat8BIMResolutionInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x739f305d_81db_43cb_ac5e_55013ef9f003);
pub const GUID_MetadataFormatAPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2e043dc2_c967_4e05_875e_618bf67e85c3);
pub const GUID_MetadataFormatApp0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x79007028_268d_45d6_a3c2_354e6a504bc9);
pub const GUID_MetadataFormatApp1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8fd3dfc3_f951_492b_817f_69c2e6d9a5b0);
pub const GUID_MetadataFormatApp13: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x326556a2_f502_4354_9cc0_8e3f48eaf6b5);
pub const GUID_MetadataFormatChunkbKGD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe14d3571_6b47_4dea_b60a_87ce0a78dfb7);
pub const GUID_MetadataFormatChunkcHRM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9db3655b_2842_44b3_8067_12e9b375556a);
pub const GUID_MetadataFormatChunkgAMA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf00935a5_1d5d_4cd1_81b2_9324d7eca781);
pub const GUID_MetadataFormatChunkhIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc59a82da_db74_48a4_bd6a_b69c4931ef95);
pub const GUID_MetadataFormatChunkiCCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeb4349ab_b685_450f_91b5_e802e892536c);
pub const GUID_MetadataFormatChunkiTXt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc2bec729_0b68_4b77_aa0e_6295a6ac1814);
pub const GUID_MetadataFormatChunksRGB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc115fd36_cc6f_4e3f_8363_524b87c6b0d9);
pub const GUID_MetadataFormatChunktEXt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x568d8936_c0a9_4923_905d_df2b38238fbc);
pub const GUID_MetadataFormatChunktIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6b00ae2d_e24b_460a_98b6_878bd03072fd);
pub const GUID_MetadataFormatDds: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a064603_8c33_4e60_9c29_136231702d08);
pub const GUID_MetadataFormatExif: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1c3c4f9d_b84a_467d_9493_36cfbd59ea57);
pub const GUID_MetadataFormatGCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a25cad8_deeb_4c69_a788_0ec2266dcafd);
pub const GUID_MetadataFormatGifComment: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc4b6e0e0_cfb4_4ad3_ab33_9aad2355a34a);
pub const GUID_MetadataFormatGps: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7134ab8a_9351_44ad_af62_448db6b502ec);
pub const GUID_MetadataFormatHeif: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x817ef3e1_1288_45f4_a852_260d9e7cce83);
pub const GUID_MetadataFormatHeifHDR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x568b8d8a_1e65_438c_8968_d60e1012beb9);
pub const GUID_MetadataFormatIMD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbd2bb086_4d52_48dd_9677_db483e85ae8f);
pub const GUID_MetadataFormatIPTC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4fab0914_e129_4087_a1d1_bc812d45a7b5);
pub const GUID_MetadataFormatIRB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x16100d66_8570_4bb9_b92d_fda4b23ece67);
pub const GUID_MetadataFormatIfd: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x537396c6_2d8a_4bb6_9bf8_2f0a8e2a3adf);
pub const GUID_MetadataFormatInterop: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xed686f8e_681f_4c8b_bd41_a8addbf6b3fc);
pub const GUID_MetadataFormatJpegChrominance: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf73d0dcf_cec6_4f85_9b0e_1c3956b1bef7);
pub const GUID_MetadataFormatJpegComment: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x220e5f33_afd3_474e_9d31_7d4fe730f557);
pub const GUID_MetadataFormatJpegLuminance: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x86908007_edfc_4860_8d4b_4ee6e83e6058);
pub const GUID_MetadataFormatLSD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe256031e_6299_4929_b98d_5ac884afba92);
pub const GUID_MetadataFormatSubIfd: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x58a2e128_2db9_4e57_bb14_5177891ed331);
pub const GUID_MetadataFormatThumbnail: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x243dcee9_8703_40ee_8ef0_22a600b8058c);
pub const GUID_MetadataFormatUnknown: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa45e592f_9078_4a7c_adb5_4edc4fd61b1f);
pub const GUID_MetadataFormatWebpANIM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6dc4fda6_78e6_4102_ae35_bcfa1edcc78b);
pub const GUID_MetadataFormatWebpANMF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x43c105ee_b93b_4abb_b003_a08c0d870471);
pub const GUID_MetadataFormatXMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb5acc38_f216_4cec_a6c5_5f6e739763a9);
pub const GUID_MetadataFormatXMPAlt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b08a675_91aa_481b_a798_4da94908613b);
pub const GUID_MetadataFormatXMPBag: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x833cca5f_dcb7_4516_806f_6596ab26dce4);
pub const GUID_MetadataFormatXMPSeq: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x63e8df02_eb6c_456c_a224_b25e794fd648);
pub const GUID_MetadataFormatXMPStruct: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x22383cf1_ed17_4e2e_af17_d85b8f6b30d0);
pub const GUID_VendorMicrosoft: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf0e749ca_edef_4589_a73a_ee0e626a2a2b);
pub const GUID_VendorMicrosoftBuiltIn: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x257a30fd_06b6_462b_aea4_63f70b86e533);
pub const GUID_WICPixelFormat112bpp6ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc937);
pub const GUID_WICPixelFormat112bpp7Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92a);
pub const GUID_WICPixelFormat128bpp7ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc938);
pub const GUID_WICPixelFormat128bpp8Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92b);
pub const GUID_WICPixelFormat128bppPRGBAFloat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91a);
pub const GUID_WICPixelFormat128bppRGBAFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91e);
pub const GUID_WICPixelFormat128bppRGBAFloat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc919);
pub const GUID_WICPixelFormat128bppRGBFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc941);
pub const GUID_WICPixelFormat128bppRGBFloat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91b);
pub const GUID_WICPixelFormat144bpp8ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc939);
pub const GUID_WICPixelFormat16bppBGR555: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc909);
pub const GUID_WICPixelFormat16bppBGR565: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90a);
pub const GUID_WICPixelFormat16bppBGRA5551: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05ec7c2b_f1e6_4961_ad46_e1cc810a87d2);
pub const GUID_WICPixelFormat16bppCbCr: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xff95ba6e_11e0_4263_bb45_01721f3460a4);
pub const GUID_WICPixelFormat16bppCbQuantizedDctCoefficients: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd2c4ff61_56a5_49c2_8b5c_4c1925964837);
pub const GUID_WICPixelFormat16bppCrQuantizedDctCoefficients: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2fe354f0_1680_42d8_9231_e73c0565bfc1);
pub const GUID_WICPixelFormat16bppGray: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90b);
pub const GUID_WICPixelFormat16bppGrayFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc913);
pub const GUID_WICPixelFormat16bppGrayHalf: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93e);
pub const GUID_WICPixelFormat16bppYQuantizedDctCoefficients: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa355f433_48e8_4a42_84d8_e2aa26ca80a4);
pub const GUID_WICPixelFormat1bppIndexed: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc901);
pub const GUID_WICPixelFormat24bpp3Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc920);
pub const GUID_WICPixelFormat24bppBGR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90c);
pub const GUID_WICPixelFormat24bppRGB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90d);
pub const GUID_WICPixelFormat2bppGray: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc906);
pub const GUID_WICPixelFormat2bppIndexed: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc902);
pub const GUID_WICPixelFormat32bpp3ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92e);
pub const GUID_WICPixelFormat32bpp4Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc921);
pub const GUID_WICPixelFormat32bppBGR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90e);
pub const GUID_WICPixelFormat32bppBGR101010: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc914);
pub const GUID_WICPixelFormat32bppBGRA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90f);
pub const GUID_WICPixelFormat32bppCMYK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91c);
pub const GUID_WICPixelFormat32bppGrayFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93f);
pub const GUID_WICPixelFormat32bppGrayFloat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc911);
pub const GUID_WICPixelFormat32bppPBGRA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc910);
pub const GUID_WICPixelFormat32bppPRGBA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cc4a650_a527_4d37_a916_3142c7ebedba);
pub const GUID_WICPixelFormat32bppR10G10B10A2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x604e1bb5_8a3c_4b65_b11c_bc0b8dd75b7f);
pub const GUID_WICPixelFormat32bppR10G10B10A2HDR10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c215c5d_1acc_4f0e_a4bc_70fb3ae8fd28);
pub const GUID_WICPixelFormat32bppRGB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd98c6b95_3efe_47d6_bb25_eb1748ab0cf1);
pub const GUID_WICPixelFormat32bppRGBA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf5c7ad2d_6a8d_43dd_a7a8_a29935261ae9);
pub const GUID_WICPixelFormat32bppRGBA1010102: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x25238d72_fcf9_4522_b514_5578e5ad55e0);
pub const GUID_WICPixelFormat32bppRGBA1010102XR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00de6b9a_c101_434b_b502_d0165ee1122c);
pub const GUID_WICPixelFormat32bppRGBE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93d);
pub const GUID_WICPixelFormat40bpp4ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92f);
pub const GUID_WICPixelFormat40bpp5Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc922);
pub const GUID_WICPixelFormat40bppCMYKAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92c);
pub const GUID_WICPixelFormat48bpp3Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc926);
pub const GUID_WICPixelFormat48bpp5ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc930);
pub const GUID_WICPixelFormat48bpp6Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc923);
pub const GUID_WICPixelFormat48bppBGR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe605a384_b468_46ce_bb2e_36f180e64313);
pub const GUID_WICPixelFormat48bppBGRFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x49ca140e_cab6_493b_9ddf_60187c37532a);
pub const GUID_WICPixelFormat48bppRGB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc915);
pub const GUID_WICPixelFormat48bppRGBFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc912);
pub const GUID_WICPixelFormat48bppRGBHalf: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93b);
pub const GUID_WICPixelFormat4bppGray: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc907);
pub const GUID_WICPixelFormat4bppIndexed: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc903);
pub const GUID_WICPixelFormat56bpp6ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc931);
pub const GUID_WICPixelFormat56bpp7Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc924);
pub const GUID_WICPixelFormat64bpp3ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc934);
pub const GUID_WICPixelFormat64bpp4Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc927);
pub const GUID_WICPixelFormat64bpp7ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc932);
pub const GUID_WICPixelFormat64bpp8Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc925);
pub const GUID_WICPixelFormat64bppBGRA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1562ff7c_d352_46f9_979e_42976b792246);
pub const GUID_WICPixelFormat64bppBGRAFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x356de33c_54d2_4a23_bb04_9b7bf9b1d42d);
pub const GUID_WICPixelFormat64bppCMYK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91f);
pub const GUID_WICPixelFormat64bppPBGRA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8c518e8e_a4ec_468b_ae70_c9a35a9c5530);
pub const GUID_WICPixelFormat64bppPRGBA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc917);
pub const GUID_WICPixelFormat64bppPRGBAHalf: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x58ad26c2_c623_4d9d_b320_387e49f8c442);
pub const GUID_WICPixelFormat64bppRGB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa1182111_186d_4d42_bc6a_9c8303a8dff9);
pub const GUID_WICPixelFormat64bppRGBA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc916);
pub const GUID_WICPixelFormat64bppRGBAFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91d);
pub const GUID_WICPixelFormat64bppRGBAHalf: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93a);
pub const GUID_WICPixelFormat64bppRGBFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc940);
pub const GUID_WICPixelFormat64bppRGBHalf: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc942);
pub const GUID_WICPixelFormat72bpp8ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc933);
pub const GUID_WICPixelFormat80bpp4ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc935);
pub const GUID_WICPixelFormat80bpp5Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc928);
pub const GUID_WICPixelFormat80bppCMYKAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92d);
pub const GUID_WICPixelFormat8bppAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe6cd0116_eeba_4161_aa85_27dd9fb3a895);
pub const GUID_WICPixelFormat8bppCb: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1339f224_6bfe_4c3e_9302_e4f3a6d0ca2a);
pub const GUID_WICPixelFormat8bppCr: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb8145053_2116_49f0_8835_ed844b205c51);
pub const GUID_WICPixelFormat8bppGray: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc908);
pub const GUID_WICPixelFormat8bppIndexed: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc904);
pub const GUID_WICPixelFormat8bppY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x91b4db54_2df9_42f0_b449_2909bb3df88e);
pub const GUID_WICPixelFormat96bpp5ChannelsAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc936);
pub const GUID_WICPixelFormat96bpp6Channels: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc929);
pub const GUID_WICPixelFormat96bppRGBFixedPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc918);
pub const GUID_WICPixelFormat96bppRGBFloat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe3fed78f_e8db_4acf_84c1_e97f6136b327);
pub const GUID_WICPixelFormatBlackWhite: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc905);
pub const GUID_WICPixelFormatDontCare: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc900);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Contrast: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_DestinationColorContext: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_ExposureCompensation: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Gamma: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_KelvinWhitePoint: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_NamedWhitePoint: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_NoiseReduction: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_RGBWhitePoint: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_RenderMode: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Rotation: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Saturation: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Sharpness: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Tint: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_ToneCurve: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_HUFFMAN_BASELINE_ONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_HUFFMAN_BASELINE_THREE: u32 = 1118464u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_MAX_COMPONENT_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_MAX_TABLE_INDEX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_QUANTIZATION_BASELINE_ONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_QUANTIZATION_BASELINE_THREE: u32 = 65792u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_ONE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_420: u32 = 1118498u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_422: u32 = 1118497u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_440: u32 = 1118482u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_444: u32 = 1118481u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_ABORTED: i32 = -2147467260i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_ACCESSDENIED: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_BASE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_GENERIC_ERROR: i32 = -2147467259i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_INVALIDPARAMETER: i32 = -2147024809i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_NOTIMPLEMENTED: i32 = -2147467263i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_OUTOFMEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_SDK_VERSION: u32 = 567u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_SDK_VERSION1: u32 = 566u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_SDK_VERSION2: u32 = 567u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WIC8BIMIptcDigestProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcDigestPString: WIC8BIMIptcDigestProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcDigestIptcDigest: WIC8BIMIptcDigestProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcDigestProperties_FORCE_DWORD: WIC8BIMIptcDigestProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WIC8BIMIptcProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcPString: WIC8BIMIptcProperties = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcEmbeddedIPTC: WIC8BIMIptcProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcProperties_FORCE_DWORD: WIC8BIMIptcProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WIC8BIMResolutionInfoProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoPString: WIC8BIMResolutionInfoProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoHResolution: WIC8BIMResolutionInfoProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoHResolutionUnit: WIC8BIMResolutionInfoProperties = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoWidthUnit: WIC8BIMResolutionInfoProperties = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoVResolution: WIC8BIMResolutionInfoProperties = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoVResolutionUnit: WIC8BIMResolutionInfoProperties = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoHeightUnit: WIC8BIMResolutionInfoProperties = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoProperties_FORCE_DWORD: WIC8BIMResolutionInfoProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapAlphaChannelOption = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapUseAlpha: WICBitmapAlphaChannelOption = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapUsePremultipliedAlpha: WICBitmapAlphaChannelOption = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapIgnoreAlpha: WICBitmapAlphaChannelOption = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPALPHACHANNELOPTIONS_FORCE_DWORD: WICBitmapAlphaChannelOption = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapCreateCacheOption = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapNoCache: WICBitmapCreateCacheOption = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapCacheOnDemand: WICBitmapCreateCacheOption = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapCacheOnLoad: WICBitmapCreateCacheOption = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPCREATECACHEOPTION_FORCE_DWORD: WICBitmapCreateCacheOption = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapDecoderCapabilities = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilitySameEncoder: WICBitmapDecoderCapabilities = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilityCanDecodeAllImages: WICBitmapDecoderCapabilities = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilityCanDecodeSomeImages: WICBitmapDecoderCapabilities = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilityCanEnumerateMetadata: WICBitmapDecoderCapabilities = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilityCanDecodeThumbnail: WICBitmapDecoderCapabilities = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPDECODERCAPABILITIES_FORCE_DWORD: WICBitmapDecoderCapabilities = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapDitherType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeNone: WICBitmapDitherType = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeSolid: WICBitmapDitherType = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeOrdered4x4: WICBitmapDitherType = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeOrdered8x8: WICBitmapDitherType = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeOrdered16x16: WICBitmapDitherType = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeSpiral4x4: WICBitmapDitherType = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeSpiral8x8: WICBitmapDitherType = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeDualSpiral4x4: WICBitmapDitherType = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeDualSpiral8x8: WICBitmapDitherType = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeErrorDiffusion: WICBitmapDitherType = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPDITHERTYPE_FORCE_DWORD: WICBitmapDitherType = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapEncoderCacheOption = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapEncoderCacheInMemory: WICBitmapEncoderCacheOption = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapEncoderCacheTempFile: WICBitmapEncoderCacheOption = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapEncoderNoCache: WICBitmapEncoderCacheOption = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPENCODERCACHEOPTION_FORCE_DWORD: WICBitmapEncoderCacheOption = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapInterpolationMode = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeNearestNeighbor: WICBitmapInterpolationMode = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeLinear: WICBitmapInterpolationMode = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeCubic: WICBitmapInterpolationMode = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeFant: WICBitmapInterpolationMode = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeHighQualityCubic: WICBitmapInterpolationMode = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPINTERPOLATIONMODE_FORCE_DWORD: WICBitmapInterpolationMode = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapLockFlags = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapLockRead: WICBitmapLockFlags = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapLockWrite: WICBitmapLockFlags = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPLOCKFLAGS_FORCE_DWORD: WICBitmapLockFlags = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapPaletteType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeCustom: WICBitmapPaletteType = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeMedianCut: WICBitmapPaletteType = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedBW: WICBitmapPaletteType = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone8: WICBitmapPaletteType = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone27: WICBitmapPaletteType = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone64: WICBitmapPaletteType = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone125: WICBitmapPaletteType = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone216: WICBitmapPaletteType = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedWebPalette: WICBitmapPaletteType = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone252: WICBitmapPaletteType = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone256: WICBitmapPaletteType = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedGray4: WICBitmapPaletteType = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedGray16: WICBitmapPaletteType = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedGray256: WICBitmapPaletteType = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPPALETTETYPE_FORCE_DWORD: WICBitmapPaletteType = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICBitmapTransformOptions = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformRotate0: WICBitmapTransformOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformRotate90: WICBitmapTransformOptions = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformRotate180: WICBitmapTransformOptions = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformRotate270: WICBitmapTransformOptions = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformFlipHorizontal: WICBitmapTransformOptions = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformFlipVertical: WICBitmapTransformOptions = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPTRANSFORMOPTIONS_FORCE_DWORD: WICBitmapTransformOptions = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICColorContextType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICColorContextUninitialized: WICColorContextType = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICColorContextProfile: WICColorContextType = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICColorContextExifColorSpace: WICColorContextType = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICComponentEnumerateOptions = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateDefault: WICComponentEnumerateOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateRefresh: WICComponentEnumerateOptions = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateDisabled: WICComponentEnumerateOptions = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateUnsigned: WICComponentEnumerateOptions = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateBuiltInOnly: WICComponentEnumerateOptions = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICCOMPONENTENUMERATEOPTIONS_FORCE_DWORD: WICComponentEnumerateOptions = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICComponentSigning = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentSigned: WICComponentSigning = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentUnsigned: WICComponentSigning = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentSafe: WICComponentSigning = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentDisabled: WICComponentSigning = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICCOMPONENTSIGNING_FORCE_DWORD: WICComponentSigning = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICComponentType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDecoder: WICComponentType = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICEncoder: WICComponentType = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatConverter: WICComponentType = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataReader: WICComponentType = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataWriter: WICComponentType = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormat: WICComponentType = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICAllComponents: WICComponentType = 63i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICCOMPONENTTYPE_FORCE_DWORD: WICComponentType = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICDdsAlphaMode = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModeUnknown: WICDdsAlphaMode = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModeStraight: WICDdsAlphaMode = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModePremultiplied: WICDdsAlphaMode = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModeOpaque: WICDdsAlphaMode = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModeCustom: WICDdsAlphaMode = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDDSALPHAMODE_FORCE_DWORD: WICDdsAlphaMode = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICDdsDimension = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsTexture1D: WICDdsDimension = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsTexture2D: WICDdsDimension = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsTexture3D: WICDdsDimension = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsTextureCube: WICDdsDimension = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDDSTEXTURE_FORCE_DWORD: WICDdsDimension = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICDecodeOptions = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDecodeMetadataCacheOnDemand: WICDecodeOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDecodeMetadataCacheOnLoad: WICDecodeOptions = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMETADATACACHEOPTION_FORCE_DWORD: WICDecodeOptions = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICGifApplicationExtensionProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifApplicationExtensionApplication: WICGifApplicationExtensionProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifApplicationExtensionData: WICGifApplicationExtensionProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifApplicationExtensionProperties_FORCE_DWORD: WICGifApplicationExtensionProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICGifCommentExtensionProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifCommentExtensionText: WICGifCommentExtensionProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifCommentExtensionProperties_FORCE_DWORD: WICGifCommentExtensionProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICGifGraphicControlExtensionProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionDisposal: WICGifGraphicControlExtensionProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionUserInputFlag: WICGifGraphicControlExtensionProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionTransparencyFlag: WICGifGraphicControlExtensionProperties = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionDelay: WICGifGraphicControlExtensionProperties = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionTransparentColorIndex: WICGifGraphicControlExtensionProperties = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionProperties_FORCE_DWORD: WICGifGraphicControlExtensionProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICGifImageDescriptorProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorLeft: WICGifImageDescriptorProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorTop: WICGifImageDescriptorProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorWidth: WICGifImageDescriptorProperties = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorHeight: WICGifImageDescriptorProperties = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorLocalColorTableFlag: WICGifImageDescriptorProperties = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorInterlaceFlag: WICGifImageDescriptorProperties = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorSortFlag: WICGifImageDescriptorProperties = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorLocalColorTableSize: WICGifImageDescriptorProperties = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorProperties_FORCE_DWORD: WICGifImageDescriptorProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICGifLogicalScreenDescriptorProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenSignature: WICGifLogicalScreenDescriptorProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorWidth: WICGifLogicalScreenDescriptorProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorHeight: WICGifLogicalScreenDescriptorProperties = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorGlobalColorTableFlag: WICGifLogicalScreenDescriptorProperties = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorColorResolution: WICGifLogicalScreenDescriptorProperties = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorSortFlag: WICGifLogicalScreenDescriptorProperties = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorGlobalColorTableSize: WICGifLogicalScreenDescriptorProperties = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorBackgroundColorIndex: WICGifLogicalScreenDescriptorProperties = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorPixelAspectRatio: WICGifLogicalScreenDescriptorProperties = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorProperties_FORCE_DWORD: WICGifLogicalScreenDescriptorProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICHeifHdrProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrMaximumLuminanceLevel: WICHeifHdrProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrMaximumFrameAverageLuminanceLevel: WICHeifHdrProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrMinimumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrMaximumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrCustomVideoPrimaries: WICHeifHdrProperties = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrProperties_FORCE_DWORD: WICHeifHdrProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICHeifProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifOrientation: WICHeifProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifProperties_FORCE_DWORD: WICHeifProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICJpegChrominanceProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegChrominanceTable: WICJpegChrominanceProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegChrominanceProperties_FORCE_DWORD: WICJpegChrominanceProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICJpegCommentProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegCommentText: WICJpegCommentProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegCommentProperties_FORCE_DWORD: WICJpegCommentProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICJpegIndexingOptions = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegIndexingOptionsGenerateOnDemand: WICJpegIndexingOptions = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegIndexingOptionsGenerateOnLoad: WICJpegIndexingOptions = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegIndexingOptions_FORCE_DWORD: WICJpegIndexingOptions = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICJpegLuminanceProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegLuminanceTable: WICJpegLuminanceProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegLuminanceProperties_FORCE_DWORD: WICJpegLuminanceProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICJpegScanType = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegScanTypeInterleaved: WICJpegScanType = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegScanTypePlanarComponents: WICJpegScanType = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegScanTypeProgressive: WICJpegScanType = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegScanType_FORCE_DWORD: WICJpegScanType = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICJpegTransferMatrix = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegTransferMatrixIdentity: WICJpegTransferMatrix = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegTransferMatrixBT601: WICJpegTransferMatrix = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegTransferMatrix_FORCE_DWORD: WICJpegTransferMatrix = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICJpegYCrCbSubsamplingOption = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsamplingDefault: WICJpegYCrCbSubsamplingOption = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsampling420: WICJpegYCrCbSubsamplingOption = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsampling422: WICJpegYCrCbSubsamplingOption = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsampling444: WICJpegYCrCbSubsamplingOption = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsampling440: WICJpegYCrCbSubsamplingOption = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJPEGYCRCBSUBSAMPLING_FORCE_DWORD: WICJpegYCrCbSubsamplingOption = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICMetadataCreationOptions = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataCreationDefault: WICMetadataCreationOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataCreationAllowUnknown: WICMetadataCreationOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataCreationFailUnknown: WICMetadataCreationOptions = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataCreationMask: WICMetadataCreationOptions = -65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICNamedWhitePoint = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointDefault: WICNamedWhitePoint = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointDaylight: WICNamedWhitePoint = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointCloudy: WICNamedWhitePoint = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointShade: WICNamedWhitePoint = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointTungsten: WICNamedWhitePoint = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointFluorescent: WICNamedWhitePoint = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointFlash: WICNamedWhitePoint = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointUnderwater: WICNamedWhitePoint = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointCustom: WICNamedWhitePoint = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointAutoWhiteBalance: WICNamedWhitePoint = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointAsShot: WICNamedWhitePoint = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICNAMEDWHITEPOINT_FORCE_DWORD: WICNamedWhitePoint = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPersistOptions = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionDefault: WICPersistOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionLittleEndian: WICPersistOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionBigEndian: WICPersistOptions = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionStrictFormat: WICPersistOptions = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionNoCacheStream: WICPersistOptions = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionPreferUTF8: WICPersistOptions = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionMask: WICPersistOptions = 65535i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPixelFormatNumericRepresentation = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationUnspecified: WICPixelFormatNumericRepresentation = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationIndexed: WICPixelFormatNumericRepresentation = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationUnsignedInteger: WICPixelFormatNumericRepresentation = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationSignedInteger: WICPixelFormatNumericRepresentation = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationFixed: WICPixelFormatNumericRepresentation = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationFloat: WICPixelFormatNumericRepresentation = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentation_FORCE_DWORD: WICPixelFormatNumericRepresentation = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPlanarOptions = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPlanarOptionsDefault: WICPlanarOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPlanarOptionsPreserveSubsampling: WICPlanarOptions = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPLANAROPTIONS_FORCE_DWORD: WICPlanarOptions = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngBkgdProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngBkgdBackgroundColor: WICPngBkgdProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngBkgdProperties_FORCE_DWORD: WICPngBkgdProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngChrmProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmWhitePointX: WICPngChrmProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmWhitePointY: WICPngChrmProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmRedX: WICPngChrmProperties = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmRedY: WICPngChrmProperties = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmGreenX: WICPngChrmProperties = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmGreenY: WICPngChrmProperties = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmBlueX: WICPngChrmProperties = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmBlueY: WICPngChrmProperties = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmProperties_FORCE_DWORD: WICPngChrmProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngFilterOption = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterUnspecified: WICPngFilterOption = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterNone: WICPngFilterOption = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterSub: WICPngFilterOption = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterUp: WICPngFilterOption = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterAverage: WICPngFilterOption = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterPaeth: WICPngFilterOption = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterAdaptive: WICPngFilterOption = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPNGFILTEROPTION_FORCE_DWORD: WICPngFilterOption = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngGamaProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngGamaGamma: WICPngGamaProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngGamaProperties_FORCE_DWORD: WICPngGamaProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngHistProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngHistFrequencies: WICPngHistProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngHistProperties_FORCE_DWORD: WICPngHistProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngIccpProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngIccpProfileName: WICPngIccpProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngIccpProfileData: WICPngIccpProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngIccpProperties_FORCE_DWORD: WICPngIccpProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngItxtProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtKeyword: WICPngItxtProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtCompressionFlag: WICPngItxtProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtLanguageTag: WICPngItxtProperties = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtTranslatedKeyword: WICPngItxtProperties = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtText: WICPngItxtProperties = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtProperties_FORCE_DWORD: WICPngItxtProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngSrgbProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngSrgbRenderingIntent: WICPngSrgbProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngSrgbProperties_FORCE_DWORD: WICPngSrgbProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICPngTimeProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeYear: WICPngTimeProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeMonth: WICPngTimeProperties = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeDay: WICPngTimeProperties = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeHour: WICPngTimeProperties = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeMinute: WICPngTimeProperties = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeSecond: WICPngTimeProperties = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeProperties_FORCE_DWORD: WICPngTimeProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICProgressNotification = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressNotificationBegin: WICProgressNotification = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressNotificationEnd: WICProgressNotification = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressNotificationFrequent: WICProgressNotification = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressNotificationAll: WICProgressNotification = -65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPROGRESSNOTIFICATION_FORCE_DWORD: WICProgressNotification = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICProgressOperation = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressOperationCopyPixels: WICProgressOperation = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressOperationWritePixels: WICProgressOperation = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressOperationAll: WICProgressOperation = 65535i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPROGRESSOPERATION_FORCE_DWORD: WICProgressOperation = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICRawCapabilities = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawCapabilityNotSupported: WICRawCapabilities = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawCapabilityGetSupported: WICRawCapabilities = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawCapabilityFullySupported: WICRawCapabilities = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRAWCAPABILITIES_FORCE_DWORD: WICRawCapabilities = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICRawParameterSet = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICAsShotParameterSet: WICRawParameterSet = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICUserAdjustedParameterSet: WICRawParameterSet = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICAutoAdjustedParameterSet: WICRawParameterSet = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRAWPARAMETERSET_FORCE_DWORD: WICRawParameterSet = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICRawRenderMode = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRenderModeDraft: WICRawRenderMode = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRenderModeNormal: WICRawRenderMode = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRenderModeBestQuality: WICRawRenderMode = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRAWRENDERMODE_FORCE_DWORD: WICRawRenderMode = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICRawRotationCapabilities = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRotationCapabilityNotSupported: WICRawRotationCapabilities = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRotationCapabilityGetSupported: WICRawRotationCapabilities = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRotationCapabilityNinetyDegreesSupported: WICRawRotationCapabilities = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRotationCapabilityFullySupported: WICRawRotationCapabilities = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRAWROTATIONCAPABILITIES_FORCE_DWORD: WICRawRotationCapabilities = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICSectionAccessLevel = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICSectionAccessLevelRead: WICSectionAccessLevel = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICSectionAccessLevelReadWrite: WICSectionAccessLevel = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICSectionAccessLevel_FORCE_DWORD: WICSectionAccessLevel = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICTiffCompressionOption = i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionDontCare: WICTiffCompressionOption = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionNone: WICTiffCompressionOption = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionCCITT3: WICTiffCompressionOption = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionCCITT4: WICTiffCompressionOption = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionLZW: WICTiffCompressionOption = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionRLE: WICTiffCompressionOption = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionZIP: WICTiffCompressionOption = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionLZWHDifferencing: WICTiffCompressionOption = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTIFFCOMPRESSIONOPTION_FORCE_DWORD: WICTiffCompressionOption = 2147483647i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICWebpAnimProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWebpAnimLoopCount: WICWebpAnimProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWebpAnimProperties_FORCE_DWORD: WICWebpAnimProperties = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type WICWebpAnmfProperties = u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWebpAnmfFrameDuration: WICWebpAnmfProperties = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWebpAnmfProperties_FORCE_DWORD: WICWebpAnmfProperties = 2147483647u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WICBitmapPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub EndOfStream: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WICBitmapPattern {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WICBitmapPattern {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICBitmapPlane {
    pub Format: ::windows_sys::core::GUID,
    pub pbBuffer: *mut u8,
    pub cbStride: u32,
    pub cbBufferSize: u32,
}
impl ::core::marker::Copy for WICBitmapPlane {}
impl ::core::clone::Clone for WICBitmapPlane {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICBitmapPlaneDescription {
    pub Format: ::windows_sys::core::GUID,
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for WICBitmapPlaneDescription {}
impl ::core::clone::Clone for WICBitmapPlaneDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct WICDdsFormatInfo {
    pub DxgiFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub BytesPerBlock: u32,
    pub BlockWidth: u32,
    pub BlockHeight: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for WICDdsFormatInfo {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for WICDdsFormatInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct WICDdsParameters {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub DxgiFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub Dimension: WICDdsDimension,
    pub AlphaMode: WICDdsAlphaMode,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for WICDdsParameters {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for WICDdsParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct WICImageParameters {
    pub PixelFormat: super::Direct2D::Common::D2D1_PIXEL_FORMAT,
    pub DpiX: f32,
    pub DpiY: f32,
    pub Top: f32,
    pub Left: f32,
    pub PixelWidth: u32,
    pub PixelHeight: u32,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for WICImageParameters {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for WICImageParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICJpegFrameHeader {
    pub Width: u32,
    pub Height: u32,
    pub TransferMatrix: WICJpegTransferMatrix,
    pub ScanType: WICJpegScanType,
    pub cComponents: u32,
    pub ComponentIdentifiers: u32,
    pub SampleFactors: u32,
    pub QuantizationTableIndices: u32,
}
impl ::core::marker::Copy for WICJpegFrameHeader {}
impl ::core::clone::Clone for WICJpegFrameHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICJpegScanHeader {
    pub cComponents: u32,
    pub RestartInterval: u32,
    pub ComponentSelectors: u32,
    pub HuffmanTableIndices: u32,
    pub StartSpectralSelection: u8,
    pub EndSpectralSelection: u8,
    pub SuccessiveApproximationHigh: u8,
    pub SuccessiveApproximationLow: u8,
}
impl ::core::marker::Copy for WICJpegScanHeader {}
impl ::core::clone::Clone for WICJpegScanHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICMetadataHeader {
    pub Position: u64,
    pub Length: u32,
    pub Header: *mut u8,
    pub DataOffset: u64,
}
impl ::core::marker::Copy for WICMetadataHeader {}
impl ::core::clone::Clone for WICMetadataHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICMetadataPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub DataOffset: u64,
}
impl ::core::marker::Copy for WICMetadataPattern {}
impl ::core::clone::Clone for WICMetadataPattern {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICRawCapabilitiesInfo {
    pub cbSize: u32,
    pub CodecMajorVersion: u32,
    pub CodecMinorVersion: u32,
    pub ExposureCompensationSupport: WICRawCapabilities,
    pub ContrastSupport: WICRawCapabilities,
    pub RGBWhitePointSupport: WICRawCapabilities,
    pub NamedWhitePointSupport: WICRawCapabilities,
    pub NamedWhitePointSupportMask: u32,
    pub KelvinWhitePointSupport: WICRawCapabilities,
    pub GammaSupport: WICRawCapabilities,
    pub TintSupport: WICRawCapabilities,
    pub SaturationSupport: WICRawCapabilities,
    pub SharpnessSupport: WICRawCapabilities,
    pub NoiseReductionSupport: WICRawCapabilities,
    pub DestinationColorProfileSupport: WICRawCapabilities,
    pub ToneCurveSupport: WICRawCapabilities,
    pub RotationSupport: WICRawRotationCapabilities,
    pub RenderModeSupport: WICRawCapabilities,
}
impl ::core::marker::Copy for WICRawCapabilitiesInfo {}
impl ::core::clone::Clone for WICRawCapabilitiesInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICRawToneCurve {
    pub cPoints: u32,
    pub aPoints: [WICRawToneCurvePoint; 1],
}
impl ::core::marker::Copy for WICRawToneCurve {}
impl ::core::clone::Clone for WICRawToneCurve {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICRawToneCurvePoint {
    pub Input: f64,
    pub Output: f64,
}
impl ::core::marker::Copy for WICRawToneCurvePoint {}
impl ::core::clone::Clone for WICRawToneCurvePoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICRect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl ::core::marker::Copy for WICRect {}
impl ::core::clone::Clone for WICRect {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type PFNProgressNotification = ::core::option::Option<unsafe extern "system" fn(pvdata: *const ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows_sys::core::HRESULT>;
