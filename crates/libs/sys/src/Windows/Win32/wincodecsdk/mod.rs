windows_link::link!("windowscodecs.dll" "system" fn WICGetMetadataContentSize(guidcontainerformat : *const windows_sys::core::GUID, piwriter : *mut core::ffi::c_void, pcbsize : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("windowscodecs.dll" "system" fn WICMatchMetadataContent(guidcontainerformat : *const windows_sys::core::GUID, pguidvendor : *const windows_sys::core::GUID, pistream : *mut core::ffi::c_void, pguidmetadataformat : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("windowscodecs.dll" "system" fn WICSerializeMetadataContent(guidcontainerformat : *const windows_sys::core::GUID, piwriter : *mut core::ffi::c_void, dwpersistoptions : u32, pistream : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const CLSID_WIC8BIMIPTCDigestMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x02805f1e_d5aa_415b_82c5_61c033a988a6);
pub const CLSID_WIC8BIMIPTCDigestMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2db5e62b_0d67_495f_8f9d_c2f0188647ac);
pub const CLSID_WIC8BIMIPTCMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0010668c_0801_4da6_a4a4_826522b6d28f);
pub const CLSID_WIC8BIMIPTCMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00108226_ee41_44a2_9e9c_4be4d5b1d2cd);
pub const CLSID_WIC8BIMResolutionInfoMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5805137a_e348_4f7c_b3cc_6db9965a0599);
pub const CLSID_WIC8BIMResolutionInfoMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4ff2fe0e_e74a_4b71_98c4_ab7dc16707ba);
pub const CLSID_WICAPEMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1767b93a_b021_44ea_920f_863c11f4f768);
pub const CLSID_WICAPEMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbd6edfca_2890_482f_b233_8d7339a1cf8d);
pub const CLSID_WICApp0MetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43324b33_a78f_480f_9111_9638aaccc832);
pub const CLSID_WICApp0MetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3c633a2_46c8_498e_8fbb_cc6f721bbcde);
pub const CLSID_WICApp13MetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa7e3c50_864c_4604_bc04_8b0b76e637f6);
pub const CLSID_WICApp13MetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7b19a919_a9d6_49e5_bd45_02c34e4e4cd5);
pub const CLSID_WICApp1MetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdde33513_774e_4bcd_ae79_02f4adfe62fc);
pub const CLSID_WICApp1MetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xee366069_1832_420f_b381_0479ad066f19);
pub const CLSID_WICDdsMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x276c88ca_7533_4a86_b676_66b36080d484);
pub const CLSID_WICDdsMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfd688bbd_31ed_4db7_a723_934927d38367);
pub const CLSID_WICExifMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd9403860_297f_4a49_bf9b_77898150a442);
pub const CLSID_WICExifMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc9a14cda_c339_460b_9078_d4debcfabe91);
pub const CLSID_WICGCEMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb92e345d_f52d_41f3_b562_081bc772e3b9);
pub const CLSID_WICGCEMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaf95dc76_16b2_47f4_b3ea_3c31796693e7);
pub const CLSID_WICGainMapMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3ac32daf_27b9_4af5_b0ab_d1189dcf34b3);
pub const CLSID_WICGainMapMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f845268_a92e_4a02_b002_a67c362800b2);
pub const CLSID_WICGifCommentMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x32557d3b_69dc_4f95_836e_f5972b2f6159);
pub const CLSID_WICGifCommentMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa02797fc_c4ae_418c_af95_e637c7ead2a1);
pub const CLSID_WICGpsMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3697790b_223b_484e_9925_c4869218f17a);
pub const CLSID_WICGpsMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcb8c13e4_62b5_4c96_a48b_6ba6ace39c76);
pub const CLSID_WICHeifHDRMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2438de3d_94d9_4be8_84a8_4de95a575e75);
pub const CLSID_WICHeifHDRMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb83135a2_8e7e_485e_a533_f93621dd93c8);
pub const CLSID_WICHeifMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xacddfc3f_85ec_41bc_bdef_1bc262e4db05);
pub const CLSID_WICHeifMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3ae45e79_40bc_4401_ace5_dd3cb16e6afe);
pub const CLSID_WICIMDMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7447a267_0015_42c8_a8f1_fb3b94c68361);
pub const CLSID_WICIMDMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8c89071f_452e_4e95_9682_9d1024627172);
pub const CLSID_WICIPTCMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICIPTCMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1249b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICIRBMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd4dcd3d7_b4c2_47d9_a6bf_b89ba396a4a3);
pub const CLSID_WICIRBMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5c5c1935_0235_4434_80bc_251bc1ec39c6);
pub const CLSID_WICIfdMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8f914656_9d0a_4eb2_9019_0bf96d8a9ee6);
pub const CLSID_WICIfdMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb1ebfc28_c9bd_47a2_8d33_b948769777a7);
pub const CLSID_WICInteropMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb5c8b898_0074_459f_b700_860d4651ea14);
pub const CLSID_WICInteropMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x122ec645_cd7e_44d8_b186_2c8c20c3b50f);
pub const CLSID_WICJpegChrominanceMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50b1904b_f28f_4574_93f4_0bade82c69e9);
pub const CLSID_WICJpegChrominanceMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3ff566f0_6e6b_49d4_96e6_b78886692c62);
pub const CLSID_WICJpegCommentMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9f66347c_60c4_4c4d_ab58_d2358685f607);
pub const CLSID_WICJpegCommentMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe573236f_55b1_4eda_81ea_9f65db0290d3);
pub const CLSID_WICJpegLuminanceMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x356f2f88_05a6_4728_b9a4_1bfbce04d838);
pub const CLSID_WICJpegLuminanceMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1d583abc_8a0e_4657_9982_a380ca58fb4b);
pub const CLSID_WICJpegXLAnimFrameMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9cdf50a8_8770_4fe6_aef2_d06e2c01744f);
pub const CLSID_WICJpegXLAnimFrameMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd1ce58a8_06e0_4b6f_8fc1_577560bd5ad9);
pub const CLSID_WICJpegXLAnimMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbf8b6eb0_37e2_4ed8_8289_be9ae31d9f03);
pub const CLSID_WICJpegXLAnimMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39d01345_432b_44e6_afd6_f606d20a5571);
pub const CLSID_WICLSDMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41070793_59e4_479a_a1f7_954adc2ef5fc);
pub const CLSID_WICLSDMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x73c037e7_e5d9_4954_876a_6da81d6e5768);
pub const CLSID_WICPngBkgdMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0ce7a4a6_03e8_4a60_9d15_282ef32ee7da);
pub const CLSID_WICPngBkgdMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x68e3f2fd_31ae_4441_bb6a_fd7047525f90);
pub const CLSID_WICPngChrmMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf90b5f36_367b_402a_9dd1_bc0fd59d8f62);
pub const CLSID_WICPngChrmMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe23ce3eb_5608_4e83_bcef_27b1987e51d7);
pub const CLSID_WICPngGamaMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3692ca39_e082_4350_9e1f_3704cb083cd5);
pub const CLSID_WICPngGamaMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xff036d13_5d4b_46dd_b10f_106693d9fe4f);
pub const CLSID_WICPngHistMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x877a0bb7_a313_4491_87b5_2e6d0594f520);
pub const CLSID_WICPngHistMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8a03e749_672e_446e_bf1f_2c11d233b6ff);
pub const CLSID_WICPngIccpMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf5d3e63b_cb0f_4628_a478_6d8244be36b1);
pub const CLSID_WICPngIccpMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x16671e5f_0ce6_4cc4_9768_e89fe5018ade);
pub const CLSID_WICPngItxtMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaabfb2fa_3e1e_4a8f_8977_5556fb94ea23);
pub const CLSID_WICPngItxtMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31879719_e751_4df8_981d_68dff67704ed);
pub const CLSID_WICPngSrgbMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb40360c_547e_4956_a3b9_d4418859ba66);
pub const CLSID_WICPngSrgbMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa6ee35c6_87ec_47df_9f22_1d5aad840c82);
pub const CLSID_WICPngTextMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b59afcc_b8c3_408a_b670_89e5fab6fda7);
pub const CLSID_WICPngTextMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb5ebafb9_253e_4a72_a744_0762d2685683);
pub const CLSID_WICPngTimeMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd94edf02_efe5_4f0d_85c8_f5a68b3000b1);
pub const CLSID_WICPngTimeMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ab78400_b5a3_4d91_8ace_33fcd1499be6);
pub const CLSID_WICSubIfdMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50d42f09_ecd1_4b41_b65d_da1fdaa75663);
pub const CLSID_WICSubIfdMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8ade5386_8e9b_4f4c_acf2_f0008706b238);
pub const CLSID_WICThumbnailMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICThumbnailMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd049b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICUnknownMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x699745c2_5066_4b82_a8e3_d40478dbec8c);
pub const CLSID_WICUnknownMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa09cca86_27ba_4f39_9053_121fa4dc08fc);
pub const CLSID_WICWebpAnimMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x076f9911_a348_465c_a807_a252f3f2d3de);
pub const CLSID_WICWebpAnmfMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x85a10b03_c9f6_439f_be5e_c0fbef67807c);
pub const CLSID_WICXMPAltMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa94dcc2_b8b0_4898_b835_000aabd74393);
pub const CLSID_WICXMPAltMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x076c2a6c_f78f_4c46_a723_3583e70876ea);
pub const CLSID_WICXMPBagMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe7e79a30_4f2c_4fab_8d00_394f2d6bbebe);
pub const CLSID_WICXMPBagMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xed822c8c_d6be_4301_a631_0e1416bad28f);
pub const CLSID_WICXMPMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x72b624df_ae11_4948_a65c_351eb0829419);
pub const CLSID_WICXMPMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1765e14e_1bd4_462e_b6b1_590bf1262ac6);
pub const CLSID_WICXMPSeqMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7f12e753_fc71_43d7_a51d_92f35977abb5);
pub const CLSID_WICXMPSeqMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d68d1de_d432_4b0f_923a_091183a9bda7);
pub const CLSID_WICXMPStructMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x01b90d9a_8209_47f7_9c52_e1244bf50ced);
pub const CLSID_WICXMPStructMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x22c21f93_7ddb_411c_9b17_c5b7bd064abc);
pub const GUID_MetadataFormat8BIMIPTC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0010568c_0852_4e6a_b191_5c33ac5b0430);
pub const GUID_MetadataFormat8BIMIPTCDigest: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ca32285_9ccd_4786_8bd8_79539db6a006);
pub const GUID_MetadataFormat8BIMResolutionInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x739f305d_81db_43cb_ac5e_55013ef9f003);
pub const GUID_MetadataFormatAPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2e043dc2_c967_4e05_875e_618bf67e85c3);
pub const GUID_MetadataFormatApp0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x79007028_268d_45d6_a3c2_354e6a504bc9);
pub const GUID_MetadataFormatApp1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fd3dfc3_f951_492b_817f_69c2e6d9a5b0);
pub const GUID_MetadataFormatApp13: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x326556a2_f502_4354_9cc0_8e3f48eaf6b5);
pub const GUID_MetadataFormatChunkbKGD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe14d3571_6b47_4dea_b60a_87ce0a78dfb7);
pub const GUID_MetadataFormatChunkcHRM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9db3655b_2842_44b3_8067_12e9b375556a);
pub const GUID_MetadataFormatChunkgAMA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf00935a5_1d5d_4cd1_81b2_9324d7eca781);
pub const GUID_MetadataFormatChunkhIST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc59a82da_db74_48a4_bd6a_b69c4931ef95);
pub const GUID_MetadataFormatChunkiCCP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb4349ab_b685_450f_91b5_e802e892536c);
pub const GUID_MetadataFormatChunkiTXt: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc2bec729_0b68_4b77_aa0e_6295a6ac1814);
pub const GUID_MetadataFormatChunksRGB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc115fd36_cc6f_4e3f_8363_524b87c6b0d9);
pub const GUID_MetadataFormatChunktEXt: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x568d8936_c0a9_4923_905d_df2b38238fbc);
pub const GUID_MetadataFormatChunktIME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6b00ae2d_e24b_460a_98b6_878bd03072fd);
pub const GUID_MetadataFormatDds: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4a064603_8c33_4e60_9c29_136231702d08);
pub const GUID_MetadataFormatExif: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1c3c4f9d_b84a_467d_9493_36cfbd59ea57);
pub const GUID_MetadataFormatGCE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a25cad8_deeb_4c69_a788_0ec2266dcafd);
pub const GUID_MetadataFormatGainMap: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x568d3138_c446_4ec2_a7a8_59abb16d21e3);
pub const GUID_MetadataFormatGifComment: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc4b6e0e0_cfb4_4ad3_ab33_9aad2355a34a);
pub const GUID_MetadataFormatGps: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7134ab8a_9351_44ad_af62_448db6b502ec);
pub const GUID_MetadataFormatHeif: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x817ef3e1_1288_45f4_a852_260d9e7cce83);
pub const GUID_MetadataFormatHeifHDR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x568b8d8a_1e65_438c_8968_d60e1012beb9);
pub const GUID_MetadataFormatIMD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbd2bb086_4d52_48dd_9677_db483e85ae8f);
pub const GUID_MetadataFormatIPTC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4fab0914_e129_4087_a1d1_bc812d45a7b5);
pub const GUID_MetadataFormatIRB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x16100d66_8570_4bb9_b92d_fda4b23ece67);
pub const GUID_MetadataFormatIfd: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x537396c6_2d8a_4bb6_9bf8_2f0a8e2a3adf);
pub const GUID_MetadataFormatInterop: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xed686f8e_681f_4c8b_bd41_a8addbf6b3fc);
pub const GUID_MetadataFormatJpegChrominance: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf73d0dcf_cec6_4f85_9b0e_1c3956b1bef7);
pub const GUID_MetadataFormatJpegComment: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x220e5f33_afd3_474e_9d31_7d4fe730f557);
pub const GUID_MetadataFormatJpegLuminance: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86908007_edfc_4860_8d4b_4ee6e83e6058);
pub const GUID_MetadataFormatJpegXLAnim: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x501c2e24_7a7d_42b2_93c7_b4f45bcc92f7);
pub const GUID_MetadataFormatJpegXLAnimFrame: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x958ecc2c_36cb_4af9_9ea8_0b74baccfd3e);
pub const GUID_MetadataFormatLSD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe256031e_6299_4929_b98d_5ac884afba92);
pub const GUID_MetadataFormatSubIfd: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x58a2e128_2db9_4e57_bb14_5177891ed331);
pub const GUID_MetadataFormatThumbnail: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x243dcee9_8703_40ee_8ef0_22a600b8058c);
pub const GUID_MetadataFormatUnknown: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa45e592f_9078_4a7c_adb5_4edc4fd61b1f);
pub const GUID_MetadataFormatWebpANIM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6dc4fda6_78e6_4102_ae35_bcfa1edcc78b);
pub const GUID_MetadataFormatWebpANMF: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43c105ee_b93b_4abb_b003_a08c0d870471);
pub const GUID_MetadataFormatXMP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbb5acc38_f216_4cec_a6c5_5f6e739763a9);
pub const GUID_MetadataFormatXMPAlt: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7b08a675_91aa_481b_a798_4da94908613b);
pub const GUID_MetadataFormatXMPBag: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x833cca5f_dcb7_4516_806f_6596ab26dce4);
pub const GUID_MetadataFormatXMPSeq: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x63e8df02_eb6c_456c_a224_b25e794fd648);
pub const GUID_MetadataFormatXMPStruct: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x22383cf1_ed17_4e2e_af17_d85b8f6b30d0);
pub const WICMetadataCreationAllowUnknown: WICMetadataCreationOptions = 0;
pub const WICMetadataCreationDefault: WICMetadataCreationOptions = 0;
pub const WICMetadataCreationFailUnknown: WICMetadataCreationOptions = 65536;
pub const WICMetadataCreationMask: WICMetadataCreationOptions = -65536;
pub type WICMetadataCreationOptions = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WICMetadataHeader {
    pub Position: u64,
    pub Length: u32,
    pub Header: *mut u8,
    pub DataOffset: u64,
}
impl Default for WICMetadataHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WICMetadataPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub DataOffset: u64,
}
impl Default for WICMetadataPattern {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WICPersistOptionBigEndian: WICPersistOptions = 1;
pub const WICPersistOptionDefault: WICPersistOptions = 0;
pub const WICPersistOptionLittleEndian: WICPersistOptions = 0;
pub const WICPersistOptionMask: WICPersistOptions = 65535;
pub const WICPersistOptionNoCacheStream: WICPersistOptions = 4;
pub const WICPersistOptionPreferUTF8: WICPersistOptions = 8;
pub const WICPersistOptionStrictFormat: WICPersistOptions = 2;
pub type WICPersistOptions = i32;
