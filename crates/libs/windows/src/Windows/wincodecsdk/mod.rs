#[inline]
pub unsafe fn WICGetMetadataContentSize<P1>(guidcontainerformat: *const windows_core::GUID, piwriter: P1) -> windows_core::Result<u64>
where
    P1: windows_core::Param<IWICMetadataWriter>,
{
    windows_core::link!("windowscodecs.dll" "system" fn WICGetMetadataContentSize(guidcontainerformat : *const windows_core::GUID, piwriter : *mut core::ffi::c_void, pcbsize : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WICGetMetadataContentSize(guidcontainerformat, piwriter.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn WICMatchMetadataContent<P2>(guidcontainerformat: *const windows_core::GUID, pguidvendor: Option<*const windows_core::GUID>, pistream: P2) -> windows_core::Result<windows_core::GUID>
where
    P2: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("windowscodecs.dll" "system" fn WICMatchMetadataContent(guidcontainerformat : *const windows_core::GUID, pguidvendor : *const windows_core::GUID, pistream : *mut core::ffi::c_void, pguidmetadataformat : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WICMatchMetadataContent(guidcontainerformat, pguidvendor.unwrap_or(core::mem::zeroed()) as _, pistream.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn WICSerializeMetadataContent<P1, P3>(guidcontainerformat: *const windows_core::GUID, piwriter: P1, dwpersistoptions: u32, pistream: P3) -> windows_core::HRESULT
where
    P1: windows_core::Param<IWICMetadataWriter>,
    P3: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("windowscodecs.dll" "system" fn WICSerializeMetadataContent(guidcontainerformat : *const windows_core::GUID, piwriter : *mut core::ffi::c_void, dwpersistoptions : u32, pistream : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WICSerializeMetadataContent(guidcontainerformat, piwriter.param().abi(), dwpersistoptions, pistream.param().abi()) }
}
pub const CLSID_WIC8BIMIPTCDigestMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x02805f1e_d5aa_415b_82c5_61c033a988a6);
pub const CLSID_WIC8BIMIPTCDigestMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x2db5e62b_0d67_495f_8f9d_c2f0188647ac);
pub const CLSID_WIC8BIMIPTCMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x0010668c_0801_4da6_a4a4_826522b6d28f);
pub const CLSID_WIC8BIMIPTCMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x00108226_ee41_44a2_9e9c_4be4d5b1d2cd);
pub const CLSID_WIC8BIMResolutionInfoMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x5805137a_e348_4f7c_b3cc_6db9965a0599);
pub const CLSID_WIC8BIMResolutionInfoMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x4ff2fe0e_e74a_4b71_98c4_ab7dc16707ba);
pub const CLSID_WICAPEMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x1767b93a_b021_44ea_920f_863c11f4f768);
pub const CLSID_WICAPEMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xbd6edfca_2890_482f_b233_8d7339a1cf8d);
pub const CLSID_WICApp0MetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x43324b33_a78f_480f_9111_9638aaccc832);
pub const CLSID_WICApp0MetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xf3c633a2_46c8_498e_8fbb_cc6f721bbcde);
pub const CLSID_WICApp13MetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xaa7e3c50_864c_4604_bc04_8b0b76e637f6);
pub const CLSID_WICApp13MetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x7b19a919_a9d6_49e5_bd45_02c34e4e4cd5);
pub const CLSID_WICApp1MetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xdde33513_774e_4bcd_ae79_02f4adfe62fc);
pub const CLSID_WICApp1MetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xee366069_1832_420f_b381_0479ad066f19);
pub const CLSID_WICDdsMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x276c88ca_7533_4a86_b676_66b36080d484);
pub const CLSID_WICDdsMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xfd688bbd_31ed_4db7_a723_934927d38367);
pub const CLSID_WICExifMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xd9403860_297f_4a49_bf9b_77898150a442);
pub const CLSID_WICExifMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xc9a14cda_c339_460b_9078_d4debcfabe91);
pub const CLSID_WICGCEMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xb92e345d_f52d_41f3_b562_081bc772e3b9);
pub const CLSID_WICGCEMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xaf95dc76_16b2_47f4_b3ea_3c31796693e7);
pub const CLSID_WICGainMapMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x3ac32daf_27b9_4af5_b0ab_d1189dcf34b3);
pub const CLSID_WICGainMapMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x6f845268_a92e_4a02_b002_a67c362800b2);
pub const CLSID_WICGifCommentMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x32557d3b_69dc_4f95_836e_f5972b2f6159);
pub const CLSID_WICGifCommentMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xa02797fc_c4ae_418c_af95_e637c7ead2a1);
pub const CLSID_WICGpsMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x3697790b_223b_484e_9925_c4869218f17a);
pub const CLSID_WICGpsMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xcb8c13e4_62b5_4c96_a48b_6ba6ace39c76);
pub const CLSID_WICHeifHDRMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x2438de3d_94d9_4be8_84a8_4de95a575e75);
pub const CLSID_WICHeifHDRMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xb83135a2_8e7e_485e_a533_f93621dd93c8);
pub const CLSID_WICHeifMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xacddfc3f_85ec_41bc_bdef_1bc262e4db05);
pub const CLSID_WICHeifMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x3ae45e79_40bc_4401_ace5_dd3cb16e6afe);
pub const CLSID_WICIMDMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x7447a267_0015_42c8_a8f1_fb3b94c68361);
pub const CLSID_WICIMDMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x8c89071f_452e_4e95_9682_9d1024627172);
pub const CLSID_WICIPTCMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x03012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICIPTCMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x1249b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICIRBMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xd4dcd3d7_b4c2_47d9_a6bf_b89ba396a4a3);
pub const CLSID_WICIRBMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x5c5c1935_0235_4434_80bc_251bc1ec39c6);
pub const CLSID_WICIfdMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x8f914656_9d0a_4eb2_9019_0bf96d8a9ee6);
pub const CLSID_WICIfdMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xb1ebfc28_c9bd_47a2_8d33_b948769777a7);
pub const CLSID_WICInteropMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xb5c8b898_0074_459f_b700_860d4651ea14);
pub const CLSID_WICInteropMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x122ec645_cd7e_44d8_b186_2c8c20c3b50f);
pub const CLSID_WICJpegChrominanceMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x50b1904b_f28f_4574_93f4_0bade82c69e9);
pub const CLSID_WICJpegChrominanceMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x3ff566f0_6e6b_49d4_96e6_b78886692c62);
pub const CLSID_WICJpegCommentMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x9f66347c_60c4_4c4d_ab58_d2358685f607);
pub const CLSID_WICJpegCommentMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xe573236f_55b1_4eda_81ea_9f65db0290d3);
pub const CLSID_WICJpegLuminanceMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x356f2f88_05a6_4728_b9a4_1bfbce04d838);
pub const CLSID_WICJpegLuminanceMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x1d583abc_8a0e_4657_9982_a380ca58fb4b);
pub const CLSID_WICJpegXLAnimFrameMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x9cdf50a8_8770_4fe6_aef2_d06e2c01744f);
pub const CLSID_WICJpegXLAnimFrameMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xd1ce58a8_06e0_4b6f_8fc1_577560bd5ad9);
pub const CLSID_WICJpegXLAnimMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xbf8b6eb0_37e2_4ed8_8289_be9ae31d9f03);
pub const CLSID_WICJpegXLAnimMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x39d01345_432b_44e6_afd6_f606d20a5571);
pub const CLSID_WICLSDMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x41070793_59e4_479a_a1f7_954adc2ef5fc);
pub const CLSID_WICLSDMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x73c037e7_e5d9_4954_876a_6da81d6e5768);
pub const CLSID_WICPngBkgdMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x0ce7a4a6_03e8_4a60_9d15_282ef32ee7da);
pub const CLSID_WICPngBkgdMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x68e3f2fd_31ae_4441_bb6a_fd7047525f90);
pub const CLSID_WICPngChrmMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xf90b5f36_367b_402a_9dd1_bc0fd59d8f62);
pub const CLSID_WICPngChrmMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xe23ce3eb_5608_4e83_bcef_27b1987e51d7);
pub const CLSID_WICPngGamaMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x3692ca39_e082_4350_9e1f_3704cb083cd5);
pub const CLSID_WICPngGamaMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xff036d13_5d4b_46dd_b10f_106693d9fe4f);
pub const CLSID_WICPngHistMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x877a0bb7_a313_4491_87b5_2e6d0594f520);
pub const CLSID_WICPngHistMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x8a03e749_672e_446e_bf1f_2c11d233b6ff);
pub const CLSID_WICPngIccpMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xf5d3e63b_cb0f_4628_a478_6d8244be36b1);
pub const CLSID_WICPngIccpMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x16671e5f_0ce6_4cc4_9768_e89fe5018ade);
pub const CLSID_WICPngItxtMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xaabfb2fa_3e1e_4a8f_8977_5556fb94ea23);
pub const CLSID_WICPngItxtMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x31879719_e751_4df8_981d_68dff67704ed);
pub const CLSID_WICPngSrgbMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xfb40360c_547e_4956_a3b9_d4418859ba66);
pub const CLSID_WICPngSrgbMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xa6ee35c6_87ec_47df_9f22_1d5aad840c82);
pub const CLSID_WICPngTextMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x4b59afcc_b8c3_408a_b670_89e5fab6fda7);
pub const CLSID_WICPngTextMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xb5ebafb9_253e_4a72_a744_0762d2685683);
pub const CLSID_WICPngTimeMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xd94edf02_efe5_4f0d_85c8_f5a68b3000b1);
pub const CLSID_WICPngTimeMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x1ab78400_b5a3_4d91_8ace_33fcd1499be6);
pub const CLSID_WICSubIfdMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x50d42f09_ecd1_4b41_b65d_da1fdaa75663);
pub const CLSID_WICSubIfdMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x8ade5386_8e9b_4f4c_acf2_f0008706b238);
pub const CLSID_WICThumbnailMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xfb012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICThumbnailMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xd049b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICUnknownMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x699745c2_5066_4b82_a8e3_d40478dbec8c);
pub const CLSID_WICUnknownMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xa09cca86_27ba_4f39_9053_121fa4dc08fc);
pub const CLSID_WICWebpAnimMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x076f9911_a348_465c_a807_a252f3f2d3de);
pub const CLSID_WICWebpAnmfMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x85a10b03_c9f6_439f_be5e_c0fbef67807c);
pub const CLSID_WICXMPAltMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xaa94dcc2_b8b0_4898_b835_000aabd74393);
pub const CLSID_WICXMPAltMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x076c2a6c_f78f_4c46_a723_3583e70876ea);
pub const CLSID_WICXMPBagMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xe7e79a30_4f2c_4fab_8d00_394f2d6bbebe);
pub const CLSID_WICXMPBagMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xed822c8c_d6be_4301_a631_0e1416bad28f);
pub const CLSID_WICXMPMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x72b624df_ae11_4948_a65c_351eb0829419);
pub const CLSID_WICXMPMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x1765e14e_1bd4_462e_b6b1_590bf1262ac6);
pub const CLSID_WICXMPSeqMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x7f12e753_fc71_43d7_a51d_92f35977abb5);
pub const CLSID_WICXMPSeqMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x6d68d1de_d432_4b0f_923a_091183a9bda7);
pub const CLSID_WICXMPStructMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x01b90d9a_8209_47f7_9c52_e1244bf50ced);
pub const CLSID_WICXMPStructMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x22c21f93_7ddb_411c_9b17_c5b7bd064abc);
pub const GUID_MetadataFormat8BIMIPTC: windows_core::GUID = windows_core::GUID::from_u128(0x0010568c_0852_4e6a_b191_5c33ac5b0430);
pub const GUID_MetadataFormat8BIMIPTCDigest: windows_core::GUID = windows_core::GUID::from_u128(0x1ca32285_9ccd_4786_8bd8_79539db6a006);
pub const GUID_MetadataFormat8BIMResolutionInfo: windows_core::GUID = windows_core::GUID::from_u128(0x739f305d_81db_43cb_ac5e_55013ef9f003);
pub const GUID_MetadataFormatAPE: windows_core::GUID = windows_core::GUID::from_u128(0x2e043dc2_c967_4e05_875e_618bf67e85c3);
pub const GUID_MetadataFormatApp0: windows_core::GUID = windows_core::GUID::from_u128(0x79007028_268d_45d6_a3c2_354e6a504bc9);
pub const GUID_MetadataFormatApp1: windows_core::GUID = windows_core::GUID::from_u128(0x8fd3dfc3_f951_492b_817f_69c2e6d9a5b0);
pub const GUID_MetadataFormatApp13: windows_core::GUID = windows_core::GUID::from_u128(0x326556a2_f502_4354_9cc0_8e3f48eaf6b5);
pub const GUID_MetadataFormatChunkbKGD: windows_core::GUID = windows_core::GUID::from_u128(0xe14d3571_6b47_4dea_b60a_87ce0a78dfb7);
pub const GUID_MetadataFormatChunkcHRM: windows_core::GUID = windows_core::GUID::from_u128(0x9db3655b_2842_44b3_8067_12e9b375556a);
pub const GUID_MetadataFormatChunkgAMA: windows_core::GUID = windows_core::GUID::from_u128(0xf00935a5_1d5d_4cd1_81b2_9324d7eca781);
pub const GUID_MetadataFormatChunkhIST: windows_core::GUID = windows_core::GUID::from_u128(0xc59a82da_db74_48a4_bd6a_b69c4931ef95);
pub const GUID_MetadataFormatChunkiCCP: windows_core::GUID = windows_core::GUID::from_u128(0xeb4349ab_b685_450f_91b5_e802e892536c);
pub const GUID_MetadataFormatChunkiTXt: windows_core::GUID = windows_core::GUID::from_u128(0xc2bec729_0b68_4b77_aa0e_6295a6ac1814);
pub const GUID_MetadataFormatChunksRGB: windows_core::GUID = windows_core::GUID::from_u128(0xc115fd36_cc6f_4e3f_8363_524b87c6b0d9);
pub const GUID_MetadataFormatChunktEXt: windows_core::GUID = windows_core::GUID::from_u128(0x568d8936_c0a9_4923_905d_df2b38238fbc);
pub const GUID_MetadataFormatChunktIME: windows_core::GUID = windows_core::GUID::from_u128(0x6b00ae2d_e24b_460a_98b6_878bd03072fd);
pub const GUID_MetadataFormatDds: windows_core::GUID = windows_core::GUID::from_u128(0x4a064603_8c33_4e60_9c29_136231702d08);
pub const GUID_MetadataFormatExif: windows_core::GUID = windows_core::GUID::from_u128(0x1c3c4f9d_b84a_467d_9493_36cfbd59ea57);
pub const GUID_MetadataFormatGCE: windows_core::GUID = windows_core::GUID::from_u128(0x2a25cad8_deeb_4c69_a788_0ec2266dcafd);
pub const GUID_MetadataFormatGainMap: windows_core::GUID = windows_core::GUID::from_u128(0x568d3138_c446_4ec2_a7a8_59abb16d21e3);
pub const GUID_MetadataFormatGifComment: windows_core::GUID = windows_core::GUID::from_u128(0xc4b6e0e0_cfb4_4ad3_ab33_9aad2355a34a);
pub const GUID_MetadataFormatGps: windows_core::GUID = windows_core::GUID::from_u128(0x7134ab8a_9351_44ad_af62_448db6b502ec);
pub const GUID_MetadataFormatHeif: windows_core::GUID = windows_core::GUID::from_u128(0x817ef3e1_1288_45f4_a852_260d9e7cce83);
pub const GUID_MetadataFormatHeifHDR: windows_core::GUID = windows_core::GUID::from_u128(0x568b8d8a_1e65_438c_8968_d60e1012beb9);
pub const GUID_MetadataFormatIMD: windows_core::GUID = windows_core::GUID::from_u128(0xbd2bb086_4d52_48dd_9677_db483e85ae8f);
pub const GUID_MetadataFormatIPTC: windows_core::GUID = windows_core::GUID::from_u128(0x4fab0914_e129_4087_a1d1_bc812d45a7b5);
pub const GUID_MetadataFormatIRB: windows_core::GUID = windows_core::GUID::from_u128(0x16100d66_8570_4bb9_b92d_fda4b23ece67);
pub const GUID_MetadataFormatIfd: windows_core::GUID = windows_core::GUID::from_u128(0x537396c6_2d8a_4bb6_9bf8_2f0a8e2a3adf);
pub const GUID_MetadataFormatInterop: windows_core::GUID = windows_core::GUID::from_u128(0xed686f8e_681f_4c8b_bd41_a8addbf6b3fc);
pub const GUID_MetadataFormatJpegChrominance: windows_core::GUID = windows_core::GUID::from_u128(0xf73d0dcf_cec6_4f85_9b0e_1c3956b1bef7);
pub const GUID_MetadataFormatJpegComment: windows_core::GUID = windows_core::GUID::from_u128(0x220e5f33_afd3_474e_9d31_7d4fe730f557);
pub const GUID_MetadataFormatJpegLuminance: windows_core::GUID = windows_core::GUID::from_u128(0x86908007_edfc_4860_8d4b_4ee6e83e6058);
pub const GUID_MetadataFormatJpegXLAnim: windows_core::GUID = windows_core::GUID::from_u128(0x501c2e24_7a7d_42b2_93c7_b4f45bcc92f7);
pub const GUID_MetadataFormatJpegXLAnimFrame: windows_core::GUID = windows_core::GUID::from_u128(0x958ecc2c_36cb_4af9_9ea8_0b74baccfd3e);
pub const GUID_MetadataFormatLSD: windows_core::GUID = windows_core::GUID::from_u128(0xe256031e_6299_4929_b98d_5ac884afba92);
pub const GUID_MetadataFormatSubIfd: windows_core::GUID = windows_core::GUID::from_u128(0x58a2e128_2db9_4e57_bb14_5177891ed331);
pub const GUID_MetadataFormatThumbnail: windows_core::GUID = windows_core::GUID::from_u128(0x243dcee9_8703_40ee_8ef0_22a600b8058c);
pub const GUID_MetadataFormatUnknown: windows_core::GUID = windows_core::GUID::from_u128(0xa45e592f_9078_4a7c_adb5_4edc4fd61b1f);
pub const GUID_MetadataFormatWebpANIM: windows_core::GUID = windows_core::GUID::from_u128(0x6dc4fda6_78e6_4102_ae35_bcfa1edcc78b);
pub const GUID_MetadataFormatWebpANMF: windows_core::GUID = windows_core::GUID::from_u128(0x43c105ee_b93b_4abb_b003_a08c0d870471);
pub const GUID_MetadataFormatXMP: windows_core::GUID = windows_core::GUID::from_u128(0xbb5acc38_f216_4cec_a6c5_5f6e739763a9);
pub const GUID_MetadataFormatXMPAlt: windows_core::GUID = windows_core::GUID::from_u128(0x7b08a675_91aa_481b_a798_4da94908613b);
pub const GUID_MetadataFormatXMPBag: windows_core::GUID = windows_core::GUID::from_u128(0x833cca5f_dcb7_4516_806f_6596ab26dce4);
pub const GUID_MetadataFormatXMPSeq: windows_core::GUID = windows_core::GUID::from_u128(0x63e8df02_eb6c_456c_a224_b25e794fd648);
pub const GUID_MetadataFormatXMPStruct: windows_core::GUID = windows_core::GUID::from_u128(0x22383cf1_ed17_4e2e_af17_d85b8f6b30d0);
#[cfg(feature = "wincodec")]
windows_core::imp::define_interface!(IWICComponentFactory, IWICComponentFactory_Vtbl, 0x412d0c3a_9650_44fa_af5b_dd2a06c8e8fb);
#[cfg(feature = "wincodec")]
impl core::ops::Deref for IWICComponentFactory {
    type Target = super::wincodec::IWICImagingFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "wincodec")]
windows_core::imp::interface_hierarchy!(IWICComponentFactory, windows_core::IUnknown, super::wincodec::IWICImagingFactory);
#[cfg(feature = "wincodec")]
impl IWICComponentFactory {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateMetadataReader<P3>(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: P3) -> windows_core::Result<IWICMetadataReader>
    where
        P3: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMetadataReader)(windows_core::Interface::as_raw(self), guidmetadataformat, pguidvendor, dwoptions, pistream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateMetadataReaderFromContainer<P3>(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: P3) -> windows_core::Result<IWICMetadataReader>
    where
        P3: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMetadataReaderFromContainer)(windows_core::Interface::as_raw(self), guidcontainerformat, pguidvendor, dwoptions, pistream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMetadataWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwmetadataoptions: u32) -> windows_core::Result<IWICMetadataWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMetadataWriter)(windows_core::Interface::as_raw(self), guidmetadataformat, pguidvendor, dwmetadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMetadataWriterFromReader<P0>(&self, pireader: P0, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataWriter>
    where
        P0: windows_core::Param<IWICMetadataReader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMetadataWriterFromReader)(windows_core::Interface::as_raw(self), pireader.param().abi(), pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateQueryReaderFromBlockReader<P0>(&self, piblockreader: P0) -> windows_core::Result<super::wincodec::IWICMetadataQueryReader>
    where
        P0: windows_core::Param<IWICMetadataBlockReader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQueryReaderFromBlockReader)(windows_core::Interface::as_raw(self), piblockreader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateQueryWriterFromBlockWriter<P0>(&self, piblockwriter: P0) -> windows_core::Result<super::wincodec::IWICMetadataQueryWriter>
    where
        P0: windows_core::Param<IWICMetadataBlockWriter>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQueryWriterFromBlockWriter)(windows_core::Interface::as_raw(self), piblockwriter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ocidl", feature = "wtypes"))]
    pub unsafe fn CreateEncoderPropertyBag(&self, ppropoptions: *const super::ocidl::PROPBAG2, ccount: u32) -> windows_core::Result<super::ocidl::IPropertyBag2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncoderPropertyBag)(windows_core::Interface::as_raw(self), ppropoptions, ccount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "wincodec")]
#[repr(C)]
#[doc(hidden)]
pub struct IWICComponentFactory_Vtbl {
    pub base__: super::wincodec::IWICImagingFactory_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub CreateMetadataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateMetadataReader: usize,
    #[cfg(feature = "objidlbase")]
    pub CreateMetadataReaderFromContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateMetadataReaderFromContainer: usize,
    pub CreateMetadataWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMetadataWriterFromReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryReaderFromBlockReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryWriterFromBlockWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "ocidl", feature = "wtypes"))]
    pub CreateEncoderPropertyBag: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::ocidl::PROPBAG2, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ocidl", feature = "wtypes")))]
    CreateEncoderPropertyBag: usize,
}
#[cfg(all(feature = "objidlbase", feature = "ocidl", feature = "wincodec", feature = "windef", feature = "wtypes"))]
pub trait IWICComponentFactory_Impl: super::wincodec::IWICImagingFactory_Impl {
    fn CreateMetadataReader(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<IWICMetadataReader>;
    fn CreateMetadataReaderFromContainer(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<IWICMetadataReader>;
    fn CreateMetadataWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwmetadataoptions: u32) -> windows_core::Result<IWICMetadataWriter>;
    fn CreateMetadataWriterFromReader(&self, pireader: windows_core::Ref<IWICMetadataReader>, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataWriter>;
    fn CreateQueryReaderFromBlockReader(&self, piblockreader: windows_core::Ref<IWICMetadataBlockReader>) -> windows_core::Result<super::wincodec::IWICMetadataQueryReader>;
    fn CreateQueryWriterFromBlockWriter(&self, piblockwriter: windows_core::Ref<IWICMetadataBlockWriter>) -> windows_core::Result<super::wincodec::IWICMetadataQueryWriter>;
    fn CreateEncoderPropertyBag(&self, ppropoptions: *const super::ocidl::PROPBAG2, ccount: u32) -> windows_core::Result<super::ocidl::IPropertyBag2>;
}
#[cfg(all(feature = "objidlbase", feature = "ocidl", feature = "wincodec", feature = "windef", feature = "wtypes"))]
impl IWICComponentFactory_Vtbl {
    pub const fn new<Identity: IWICComponentFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMetadataReader<Identity: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: *mut core::ffi::c_void, ppireader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentFactory_Impl::CreateMetadataReader(this, core::mem::transmute_copy(&guidmetadataformat), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&dwoptions), core::mem::transmute_copy(&pistream)) {
                    Ok(ok__) => {
                        ppireader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMetadataReaderFromContainer<Identity: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: *mut core::ffi::c_void, ppireader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentFactory_Impl::CreateMetadataReaderFromContainer(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&dwoptions), core::mem::transmute_copy(&pistream)) {
                    Ok(ok__) => {
                        ppireader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMetadataWriter<Identity: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwmetadataoptions: u32, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentFactory_Impl::CreateMetadataWriter(this, core::mem::transmute_copy(&guidmetadataformat), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&dwmetadataoptions)) {
                    Ok(ok__) => {
                        ppiwriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMetadataWriterFromReader<Identity: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pireader: *mut core::ffi::c_void, pguidvendor: *const windows_core::GUID, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentFactory_Impl::CreateMetadataWriterFromReader(this, core::mem::transmute_copy(&pireader), core::mem::transmute_copy(&pguidvendor)) {
                    Ok(ok__) => {
                        ppiwriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateQueryReaderFromBlockReader<Identity: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piblockreader: *mut core::ffi::c_void, ppiqueryreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentFactory_Impl::CreateQueryReaderFromBlockReader(this, core::mem::transmute_copy(&piblockreader)) {
                    Ok(ok__) => {
                        ppiqueryreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromBlockWriter<Identity: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piblockwriter: *mut core::ffi::c_void, ppiquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentFactory_Impl::CreateQueryWriterFromBlockWriter(this, core::mem::transmute_copy(&piblockwriter)) {
                    Ok(ok__) => {
                        ppiquerywriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEncoderPropertyBag<Identity: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropoptions: *const super::ocidl::PROPBAG2, ccount: u32, ppipropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentFactory_Impl::CreateEncoderPropertyBag(this, core::mem::transmute_copy(&ppropoptions), core::mem::transmute_copy(&ccount)) {
                    Ok(ok__) => {
                        ppipropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::wincodec::IWICImagingFactory_Vtbl::new::<Identity, OFFSET>(),
            CreateMetadataReader: CreateMetadataReader::<Identity, OFFSET>,
            CreateMetadataReaderFromContainer: CreateMetadataReaderFromContainer::<Identity, OFFSET>,
            CreateMetadataWriter: CreateMetadataWriter::<Identity, OFFSET>,
            CreateMetadataWriterFromReader: CreateMetadataWriterFromReader::<Identity, OFFSET>,
            CreateQueryReaderFromBlockReader: CreateQueryReaderFromBlockReader::<Identity, OFFSET>,
            CreateQueryWriterFromBlockWriter: CreateQueryWriterFromBlockWriter::<Identity, OFFSET>,
            CreateEncoderPropertyBag: CreateEncoderPropertyBag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICComponentFactory as windows_core::Interface>::IID || iid == &<super::wincodec::IWICImagingFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "ocidl", feature = "wincodec", feature = "windef", feature = "wtypes"))]
impl windows_core::RuntimeName for IWICComponentFactory {}
windows_core::imp::define_interface!(IWICMetadataBlockReader, IWICMetadataBlockReader_Vtbl, 0xfeaa2a8d_b3f3_43e4_b25c_d1de990a1ae1);
windows_core::imp::interface_hierarchy!(IWICMetadataBlockReader, windows_core::IUnknown);
impl IWICMetadataBlockReader {
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetReaderByIndex(&self, nindex: u32) -> windows_core::Result<IWICMetadataReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReaderByIndex)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<super::objidlbase::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataBlockReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetReaderByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    GetEnumerator: usize,
}
#[cfg(feature = "objidlbase")]
pub trait IWICMetadataBlockReader_Impl: windows_core::IUnknownImpl {
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetReaderByIndex(&self, nindex: u32) -> windows_core::Result<IWICMetadataReader>;
    fn GetEnumerator(&self) -> windows_core::Result<super::objidlbase::IEnumUnknown>;
}
#[cfg(feature = "objidlbase")]
impl IWICMetadataBlockReader_Vtbl {
    pub const fn new<Identity: IWICMetadataBlockReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContainerFormat<Identity: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataBlockReader_Impl::GetContainerFormat(this) {
                    Ok(ok__) => {
                        pguidcontainerformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataBlockReader_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pccount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReaderByIndex<Identity: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppimetadatareader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataBlockReader_Impl::GetReaderByIndex(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppimetadatareader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienummetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataBlockReader_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        ppienummetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetReaderByIndex: GetReaderByIndex::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataBlockReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IWICMetadataBlockReader {}
windows_core::imp::define_interface!(IWICMetadataBlockWriter, IWICMetadataBlockWriter_Vtbl, 0x08fb9676_b444_41e8_8dbe_6a53a542bff1);
impl core::ops::Deref for IWICMetadataBlockWriter {
    type Target = IWICMetadataBlockReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataBlockWriter, windows_core::IUnknown, IWICMetadataBlockReader);
impl IWICMetadataBlockWriter {
    pub unsafe fn InitializeFromBlockReader<P0>(&self, pimdblockreader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICMetadataBlockReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromBlockReader)(windows_core::Interface::as_raw(self), pimdblockreader.param().abi()) }
    }
    pub unsafe fn GetWriterByIndex(&self, nindex: u32) -> windows_core::Result<IWICMetadataWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterByIndex)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddWriter<P0>(&self, pimetadatawriter: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICMetadataWriter>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddWriter)(windows_core::Interface::as_raw(self), pimetadatawriter.param().abi()) }
    }
    pub unsafe fn SetWriterByIndex<P1>(&self, nindex: u32, pimetadatawriter: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWICMetadataWriter>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWriterByIndex)(windows_core::Interface::as_raw(self), nindex, pimetadatawriter.param().abi()) }
    }
    pub unsafe fn RemoveWriterByIndex(&self, nindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveWriterByIndex)(windows_core::Interface::as_raw(self), nindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataBlockWriter_Vtbl {
    pub base__: IWICMetadataBlockReader_Vtbl,
    pub InitializeFromBlockReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWriterByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriterByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveWriterByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IWICMetadataBlockWriter_Impl: IWICMetadataBlockReader_Impl {
    fn InitializeFromBlockReader(&self, pimdblockreader: windows_core::Ref<IWICMetadataBlockReader>) -> windows_core::Result<()>;
    fn GetWriterByIndex(&self, nindex: u32) -> windows_core::Result<IWICMetadataWriter>;
    fn AddWriter(&self, pimetadatawriter: windows_core::Ref<IWICMetadataWriter>) -> windows_core::Result<()>;
    fn SetWriterByIndex(&self, nindex: u32, pimetadatawriter: windows_core::Ref<IWICMetadataWriter>) -> windows_core::Result<()>;
    fn RemoveWriterByIndex(&self, nindex: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IWICMetadataBlockWriter_Vtbl {
    pub const fn new<Identity: IWICMetadataBlockWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeFromBlockReader<Identity: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimdblockreader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataBlockWriter_Impl::InitializeFromBlockReader(this, core::mem::transmute_copy(&pimdblockreader)).into()
            }
        }
        unsafe extern "system" fn GetWriterByIndex<Identity: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataBlockWriter_Impl::GetWriterByIndex(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppimetadatawriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddWriter<Identity: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimetadatawriter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataBlockWriter_Impl::AddWriter(this, core::mem::transmute_copy(&pimetadatawriter)).into()
            }
        }
        unsafe extern "system" fn SetWriterByIndex<Identity: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pimetadatawriter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataBlockWriter_Impl::SetWriterByIndex(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pimetadatawriter)).into()
            }
        }
        unsafe extern "system" fn RemoveWriterByIndex<Identity: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataBlockWriter_Impl::RemoveWriterByIndex(this, core::mem::transmute_copy(&nindex)).into()
            }
        }
        Self {
            base__: IWICMetadataBlockReader_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromBlockReader: InitializeFromBlockReader::<Identity, OFFSET>,
            GetWriterByIndex: GetWriterByIndex::<Identity, OFFSET>,
            AddWriter: AddWriter::<Identity, OFFSET>,
            SetWriterByIndex: SetWriterByIndex::<Identity, OFFSET>,
            RemoveWriterByIndex: RemoveWriterByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataBlockWriter as windows_core::Interface>::IID || iid == &<IWICMetadataBlockReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IWICMetadataBlockWriter {}
#[cfg(feature = "wincodec")]
windows_core::imp::define_interface!(IWICMetadataHandlerInfo, IWICMetadataHandlerInfo_Vtbl, 0xaba958bf_c672_44d1_8d61_ce6df2e682c2);
#[cfg(feature = "wincodec")]
impl core::ops::Deref for IWICMetadataHandlerInfo {
    type Target = super::wincodec::IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "wincodec")]
windows_core::imp::interface_hierarchy!(IWICMetadataHandlerInfo, windows_core::IUnknown, super::wincodec::IWICComponentInfo);
#[cfg(feature = "wincodec")]
impl IWICMetadataHandlerInfo {
    pub unsafe fn GetMetadataFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetContainerFormats(&self, ccontainerformats: u32, pguidcontainerformats: *mut windows_core::GUID, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContainerFormats)(windows_core::Interface::as_raw(self), ccontainerformats, pguidcontainerformats as _, pcchactual as _) }
    }
    pub unsafe fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceManufacturer)(windows_core::Interface::as_raw(self), cchdevicemanufacturer, wzdevicemanufacturer as _, pcchactual as _) }
    }
    pub unsafe fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceModels)(windows_core::Interface::as_raw(self), cchdevicemodels, wzdevicemodels as _, pcchactual as _) }
    }
    pub unsafe fn DoesRequireFullStream(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesRequireFullStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoesSupportPadding(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportPadding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoesRequireFixedSize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesRequireFixedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "wincodec")]
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataHandlerInfo_Vtbl {
    pub base__: super::wincodec::IWICComponentInfo_Vtbl,
    pub GetMetadataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetContainerFormats: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceModels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub DoesRequireFullStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub DoesSupportPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub DoesRequireFixedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "wincodec")]
pub trait IWICMetadataHandlerInfo_Impl: super::wincodec::IWICComponentInfo_Impl {
    fn GetMetadataFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetContainerFormats(&self, ccontainerformats: u32, pguidcontainerformats: *mut windows_core::GUID, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn DoesRequireFullStream(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DoesSupportPadding(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DoesRequireFixedSize(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "wincodec")]
impl IWICMetadataHandlerInfo_Vtbl {
    pub const fn new<Identity: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadataFormat<Identity: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidmetadataformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataHandlerInfo_Impl::GetMetadataFormat(this) {
                    Ok(ok__) => {
                        pguidmetadataformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContainerFormats<Identity: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut windows_core::GUID, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataHandlerInfo_Impl::GetContainerFormats(this, core::mem::transmute_copy(&ccontainerformats), core::mem::transmute_copy(&pguidcontainerformats), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataHandlerInfo_Impl::GetDeviceManufacturer(this, core::mem::transmute_copy(&cchdevicemanufacturer), core::mem::transmute_copy(&wzdevicemanufacturer), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetDeviceModels<Identity: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataHandlerInfo_Impl::GetDeviceModels(this, core::mem::transmute_copy(&cchdevicemodels), core::mem::transmute_copy(&wzdevicemodels), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn DoesRequireFullStream<Identity: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrequiresfullstream: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataHandlerInfo_Impl::DoesRequireFullStream(this) {
                    Ok(ok__) => {
                        pfrequiresfullstream.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoesSupportPadding<Identity: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportspadding: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataHandlerInfo_Impl::DoesSupportPadding(this) {
                    Ok(ok__) => {
                        pfsupportspadding.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoesRequireFixedSize<Identity: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffixedsize: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataHandlerInfo_Impl::DoesRequireFixedSize(this) {
                    Ok(ok__) => {
                        pffixedsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::wincodec::IWICComponentInfo_Vtbl::new::<Identity, OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Identity, OFFSET>,
            GetContainerFormats: GetContainerFormats::<Identity, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, OFFSET>,
            DoesRequireFullStream: DoesRequireFullStream::<Identity, OFFSET>,
            DoesSupportPadding: DoesSupportPadding::<Identity, OFFSET>,
            DoesRequireFixedSize: DoesRequireFixedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataHandlerInfo as windows_core::Interface>::IID || iid == &<super::wincodec::IWICComponentInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wincodec")]
impl windows_core::RuntimeName for IWICMetadataHandlerInfo {}
windows_core::imp::define_interface!(IWICMetadataReader, IWICMetadataReader_Vtbl, 0x9204fe99_d8fc_4fd5_a001_9536b067a899);
windows_core::imp::interface_hierarchy!(IWICMetadataReader, windows_core::IUnknown);
impl IWICMetadataReader {
    pub unsafe fn GetMetadataFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wincodec")]
    pub unsafe fn GetMetadataHandlerInfo(&self) -> windows_core::Result<IWICMetadataHandlerInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataHandlerInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::propidlbase::PROPVARIANT, pvarid: *mut super::propidlbase::PROPVARIANT, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValueByIndex)(windows_core::Interface::as_raw(self), nindex, pvarschema, pvarid, pvarvalue) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue(&self, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pvarschema, pvarid, pvarvalue) }
    }
    #[cfg(feature = "wincodec")]
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<super::wincodec::IWICEnumMetadataItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMetadataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "wincodec")]
    pub GetMetadataHandlerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wincodec"))]
    GetMetadataHandlerInfo: usize,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::propidlbase::PROPVARIANT, *mut super::propidlbase::PROPVARIANT, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetValueByIndex: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *const super::propidlbase::PROPVARIANT, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
    #[cfg(feature = "wincodec")]
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wincodec"))]
    GetEnumerator: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wincodec", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWICMetadataReader_Impl: windows_core::IUnknownImpl {
    fn GetMetadataFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetMetadataHandlerInfo(&self) -> windows_core::Result<IWICMetadataHandlerInfo>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::propidlbase::PROPVARIANT, pvarid: *mut super::propidlbase::PROPVARIANT, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<super::wincodec::IWICEnumMetadataItem>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wincodec", feature = "wtypes", feature = "wtypesbase"))]
impl IWICMetadataReader_Vtbl {
    pub const fn new<Identity: IWICMetadataReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadataFormat<Identity: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidmetadataformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataReader_Impl::GetMetadataFormat(this) {
                    Ok(ok__) => {
                        pguidmetadataformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMetadataHandlerInfo<Identity: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppihandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataReader_Impl::GetMetadataHandlerInfo(this) {
                    Ok(ok__) => {
                        ppihandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataReader_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pccount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValueByIndex<Identity: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pvarschema: *mut super::propidlbase::PROPVARIANT, pvarid: *mut super::propidlbase::PROPVARIANT, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataReader_Impl::GetValueByIndex(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataReader_Impl::GetValue(this, core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienummetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataReader_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        ppienummetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Identity, OFFSET>,
            GetMetadataHandlerInfo: GetMetadataHandlerInfo::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetValueByIndex: GetValueByIndex::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wincodec", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWICMetadataReader {}
#[cfg(feature = "wincodec")]
windows_core::imp::define_interface!(IWICMetadataReaderInfo, IWICMetadataReaderInfo_Vtbl, 0xeebf1f5b_07c1_4447_a3ab_22acaf78a804);
#[cfg(feature = "wincodec")]
impl core::ops::Deref for IWICMetadataReaderInfo {
    type Target = IWICMetadataHandlerInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "wincodec")]
windows_core::imp::interface_hierarchy!(IWICMetadataReaderInfo, windows_core::IUnknown, super::wincodec::IWICComponentInfo, IWICMetadataHandlerInfo);
#[cfg(feature = "wincodec")]
impl IWICMetadataReaderInfo {
    pub unsafe fn GetPatterns(&self, guidcontainerformat: *const windows_core::GUID, cbsize: u32, ppattern: Option<*mut WICMetadataPattern>, pccount: Option<*mut u32>, pcbactual: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPatterns)(windows_core::Interface::as_raw(self), guidcontainerformat, cbsize, ppattern.unwrap_or(core::mem::zeroed()) as _, pccount.unwrap_or(core::mem::zeroed()) as _, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn MatchesPattern<P1>(&self, guidcontainerformat: *const windows_core::GUID, pistream: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MatchesPattern)(windows_core::Interface::as_raw(self), guidcontainerformat, pistream.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICMetadataReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "wincodec")]
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataReaderInfo_Vtbl {
    pub base__: IWICMetadataHandlerInfo_Vtbl,
    pub GetPatterns: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut WICMetadataPattern, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub MatchesPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    MatchesPattern: usize,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "wincodec"))]
pub trait IWICMetadataReaderInfo_Impl: IWICMetadataHandlerInfo_Impl {
    fn GetPatterns(&self, guidcontainerformat: *const windows_core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> windows_core::Result<()>;
    fn MatchesPattern(&self, guidcontainerformat: *const windows_core::GUID, pistream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<windows_core::BOOL>;
    fn CreateInstance(&self) -> windows_core::Result<IWICMetadataReader>;
}
#[cfg(all(feature = "objidlbase", feature = "wincodec"))]
impl IWICMetadataReaderInfo_Vtbl {
    pub const fn new<Identity: IWICMetadataReaderInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPatterns<Identity: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataReaderInfo_Impl::GetPatterns(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&ppattern), core::mem::transmute_copy(&pccount), core::mem::transmute_copy(&pcbactual)).into()
            }
        }
        unsafe extern "system" fn MatchesPattern<Identity: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, pistream: *mut core::ffi::c_void, pfmatches: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataReaderInfo_Impl::MatchesPattern(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&pistream)) {
                    Ok(ok__) => {
                        pfmatches.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppireader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataReaderInfo_Impl::CreateInstance(this) {
                    Ok(ok__) => {
                        ppireader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICMetadataHandlerInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPatterns: GetPatterns::<Identity, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataReaderInfo as windows_core::Interface>::IID || iid == &<super::wincodec::IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICMetadataHandlerInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "wincodec"))]
impl windows_core::RuntimeName for IWICMetadataReaderInfo {}
windows_core::imp::define_interface!(IWICMetadataWriter, IWICMetadataWriter_Vtbl, 0xf7836e16_3be0_470b_86bb_160d0aecd7de);
impl core::ops::Deref for IWICMetadataWriter {
    type Target = IWICMetadataReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataWriter, windows_core::IUnknown, IWICMetadataReader);
impl IWICMetadataWriter {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValue(&self, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), pvarschema, pvarid, pvarvalue) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValueByIndex(&self, nindex: u32, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValueByIndex)(windows_core::Interface::as_raw(self), nindex, pvarschema, pvarid, pvarvalue) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RemoveValue(&self, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveValue)(windows_core::Interface::as_raw(self), pvarschema, pvarid) }
    }
    pub unsafe fn RemoveValueByIndex(&self, nindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveValueByIndex)(windows_core::Interface::as_raw(self), nindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataWriter_Vtbl {
    pub base__: IWICMetadataReader_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *const super::propidlbase::PROPVARIANT, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetValue: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetValueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::propidlbase::PROPVARIANT, *const super::propidlbase::PROPVARIANT, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetValueByIndex: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub RemoveValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    RemoveValue: usize,
    pub RemoveValueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wincodec", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWICMetadataWriter_Impl: IWICMetadataReader_Impl {
    fn SetValue(&self, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn SetValueByIndex(&self, nindex: u32, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn RemoveValue(&self, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn RemoveValueByIndex(&self, nindex: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wincodec", feature = "wtypes", feature = "wtypesbase"))]
impl IWICMetadataWriter_Vtbl {
    pub const fn new<Identity: IWICMetadataWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataWriter_Impl::SetValue(this, core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn SetValueByIndex<Identity: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataWriter_Impl::SetValueByIndex(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn RemoveValue<Identity: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarschema: *const super::propidlbase::PROPVARIANT, pvarid: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataWriter_Impl::RemoveValue(this, core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid)).into()
            }
        }
        unsafe extern "system" fn RemoveValueByIndex<Identity: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataWriter_Impl::RemoveValueByIndex(this, core::mem::transmute_copy(&nindex)).into()
            }
        }
        Self {
            base__: IWICMetadataReader_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, OFFSET>,
            SetValueByIndex: SetValueByIndex::<Identity, OFFSET>,
            RemoveValue: RemoveValue::<Identity, OFFSET>,
            RemoveValueByIndex: RemoveValueByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataWriter as windows_core::Interface>::IID || iid == &<IWICMetadataReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wincodec", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWICMetadataWriter {}
#[cfg(feature = "wincodec")]
windows_core::imp::define_interface!(IWICMetadataWriterInfo, IWICMetadataWriterInfo_Vtbl, 0xb22e3fba_3925_4323_b5c1_9ebfc430f236);
#[cfg(feature = "wincodec")]
impl core::ops::Deref for IWICMetadataWriterInfo {
    type Target = IWICMetadataHandlerInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "wincodec")]
windows_core::imp::interface_hierarchy!(IWICMetadataWriterInfo, windows_core::IUnknown, super::wincodec::IWICComponentInfo, IWICMetadataHandlerInfo);
#[cfg(feature = "wincodec")]
impl IWICMetadataWriterInfo {
    pub unsafe fn GetHeader(&self, guidcontainerformat: *const windows_core::GUID, cbsize: u32, pheader: Option<*mut WICMetadataHeader>, pcbactual: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHeader)(windows_core::Interface::as_raw(self), guidcontainerformat, cbsize, pheader.unwrap_or(core::mem::zeroed()) as _, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICMetadataWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "wincodec")]
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataWriterInfo_Vtbl {
    pub base__: IWICMetadataHandlerInfo_Vtbl,
    pub GetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut WICMetadataHeader, *mut u32) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wincodec")]
pub trait IWICMetadataWriterInfo_Impl: IWICMetadataHandlerInfo_Impl {
    fn GetHeader(&self, guidcontainerformat: *const windows_core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> windows_core::Result<()>;
    fn CreateInstance(&self) -> windows_core::Result<IWICMetadataWriter>;
}
#[cfg(feature = "wincodec")]
impl IWICMetadataWriterInfo_Vtbl {
    pub const fn new<Identity: IWICMetadataWriterInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHeader<Identity: IWICMetadataWriterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataWriterInfo_Impl::GetHeader(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pheader), core::mem::transmute_copy(&pcbactual)).into()
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: IWICMetadataWriterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataWriterInfo_Impl::CreateInstance(this) {
                    Ok(ok__) => {
                        ppiwriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICMetadataHandlerInfo_Vtbl::new::<Identity, OFFSET>(),
            GetHeader: GetHeader::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataWriterInfo as windows_core::Interface>::IID || iid == &<super::wincodec::IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICMetadataHandlerInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wincodec")]
impl windows_core::RuntimeName for IWICMetadataWriterInfo {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IWICPersistStream, IWICPersistStream_Vtbl, 0x00675040_6908_45f8_86a3_49c7dfd6d9ad);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IWICPersistStream {
    type Target = super::objidl::IPersistStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IWICPersistStream, windows_core::IUnknown, super::objidl::IPersist, super::objidl::IPersistStream);
#[cfg(feature = "objidl")]
impl IWICPersistStream {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn LoadEx<P0>(&self, pistream: P0, pguidpreferredvendor: *const windows_core::GUID, dwpersistoptions: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadEx)(windows_core::Interface::as_raw(self), pistream.param().abi(), pguidpreferredvendor, dwpersistoptions) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn SaveEx<P0>(&self, pistream: P0, dwpersistoptions: u32, fcleardirty: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveEx)(windows_core::Interface::as_raw(self), pistream.param().abi(), dwpersistoptions, fcleardirty.into()) }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWICPersistStream_Vtbl {
    pub base__: super::objidl::IPersistStream_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub LoadEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    LoadEx: usize,
    #[cfg(feature = "objidlbase")]
    pub SaveEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    SaveEx: usize,
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
pub trait IWICPersistStream_Impl: super::objidl::IPersistStream_Impl {
    fn LoadEx(&self, pistream: windows_core::Ref<super::objidlbase::IStream>, pguidpreferredvendor: *const windows_core::GUID, dwpersistoptions: u32) -> windows_core::Result<()>;
    fn SaveEx(&self, pistream: windows_core::Ref<super::objidlbase::IStream>, dwpersistoptions: u32, fcleardirty: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
impl IWICPersistStream_Vtbl {
    pub const fn new<Identity: IWICPersistStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadEx<Identity: IWICPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pguidpreferredvendor: *const windows_core::GUID, dwpersistoptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPersistStream_Impl::LoadEx(this, core::mem::transmute_copy(&pistream), core::mem::transmute_copy(&pguidpreferredvendor), core::mem::transmute_copy(&dwpersistoptions)).into()
            }
        }
        unsafe extern "system" fn SaveEx<Identity: IWICPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, dwpersistoptions: u32, fcleardirty: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPersistStream_Impl::SaveEx(this, core::mem::transmute_copy(&pistream), core::mem::transmute_copy(&dwpersistoptions), core::mem::transmute_copy(&fcleardirty)).into()
            }
        }
        Self { base__: super::objidl::IPersistStream_Vtbl::new::<Identity, OFFSET>(), LoadEx: LoadEx::<Identity, OFFSET>, SaveEx: SaveEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPersistStream as windows_core::Interface>::IID || iid == &<super::objidl::IPersist as windows_core::Interface>::IID || iid == &<super::objidl::IPersistStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
impl windows_core::RuntimeName for IWICPersistStream {}
windows_core::imp::define_interface!(IWICStreamProvider, IWICStreamProvider_Vtbl, 0x449494bc_b468_4927_96d7_ba90d31ab505);
windows_core::imp::interface_hierarchy!(IWICStreamProvider, windows_core::IUnknown);
impl IWICStreamProvider {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPersistOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPersistOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPreferredVendorGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreferredVendorGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RefreshStream(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshStream)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICStreamProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    GetStream: usize,
    pub GetPersistOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPreferredVendorGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RefreshStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IWICStreamProvider_Impl: windows_core::IUnknownImpl {
    fn GetStream(&self) -> windows_core::Result<super::objidlbase::IStream>;
    fn GetPersistOptions(&self) -> windows_core::Result<u32>;
    fn GetPreferredVendorGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn RefreshStream(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IWICStreamProvider_Vtbl {
    pub const fn new<Identity: IWICStreamProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStream<Identity: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppistream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICStreamProvider_Impl::GetStream(this) {
                    Ok(ok__) => {
                        ppistream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPersistOptions<Identity: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpersistoptions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICStreamProvider_Impl::GetPersistOptions(this) {
                    Ok(ok__) => {
                        pdwpersistoptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreferredVendorGUID<Identity: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidpreferredvendor: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICStreamProvider_Impl::GetPreferredVendorGUID(this) {
                    Ok(ok__) => {
                        pguidpreferredvendor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RefreshStream<Identity: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICStreamProvider_Impl::RefreshStream(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStream: GetStream::<Identity, OFFSET>,
            GetPersistOptions: GetPersistOptions::<Identity, OFFSET>,
            GetPreferredVendorGUID: GetPreferredVendorGUID::<Identity, OFFSET>,
            RefreshStream: RefreshStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICStreamProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IWICStreamProvider {}
pub const WICMetadataCreationAllowUnknown: WICMetadataCreationOptions = 0;
pub const WICMetadataCreationDefault: WICMetadataCreationOptions = 0;
pub const WICMetadataCreationFailUnknown: WICMetadataCreationOptions = 65536;
pub const WICMetadataCreationMask: WICMetadataCreationOptions = -65536;
pub type WICMetadataCreationOptions = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
