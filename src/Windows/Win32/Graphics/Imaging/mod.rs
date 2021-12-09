#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Graphics_Imaging_D2D")]
pub mod D2D;
pub const CATID_WICBitmapDecoders: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed96837_96f0_4812_b211_f13c24117ed3);
pub const CATID_WICBitmapEncoders: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac757296_3522_4e11_9862_c17be5a1767e);
pub const CATID_WICFormatConverters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7835eae8_bf14_49d1_93ce_533a407b2248);
pub const CATID_WICMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05af94d8_7174_4cd2_be4a_4124b80ee4b8);
pub const CATID_WICMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabe3b9a4_257d_4b97_bd1a_294af496222e);
pub const CATID_WICPixelFormats: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b46e70f_cda7_473e_89f6_dc9630a2390b);
pub const CLSID_WIC8BIMIPTCDigestMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02805f1e_d5aa_415b_82c5_61c033a988a6);
pub const CLSID_WIC8BIMIPTCDigestMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2db5e62b_0d67_495f_8f9d_c2f0188647ac);
pub const CLSID_WIC8BIMIPTCMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0010668c_0801_4da6_a4a4_826522b6d28f);
pub const CLSID_WIC8BIMIPTCMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00108226_ee41_44a2_9e9c_4be4d5b1d2cd);
pub const CLSID_WIC8BIMResolutionInfoMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5805137a_e348_4f7c_b3cc_6db9965a0599);
pub const CLSID_WIC8BIMResolutionInfoMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ff2fe0e_e74a_4b71_98c4_ab7dc16707ba);
pub const CLSID_WICAPEMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1767b93a_b021_44ea_920f_863c11f4f768);
pub const CLSID_WICAPEMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd6edfca_2890_482f_b233_8d7339a1cf8d);
pub const CLSID_WICAdngDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x981d9411_909e_42a7_8f5d_a747ff052edb);
pub const CLSID_WICApp0MetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43324b33_a78f_480f_9111_9638aaccc832);
pub const CLSID_WICApp0MetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3c633a2_46c8_498e_8fbb_cc6f721bbcde);
pub const CLSID_WICApp13MetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7e3c50_864c_4604_bc04_8b0b76e637f6);
pub const CLSID_WICApp13MetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b19a919_a9d6_49e5_bd45_02c34e4e4cd5);
pub const CLSID_WICApp1MetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdde33513_774e_4bcd_ae79_02f4adfe62fc);
pub const CLSID_WICApp1MetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee366069_1832_420f_b381_0479ad066f19);
pub const CLSID_WICBmpDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b462062_7cbf_400d_9fdb_813dd10f2778);
pub const CLSID_WICBmpEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69be8bb4_d66d_47c8_865a_ed1589433782);
pub const CLSID_WICDdsDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9053699f_a341_429d_9e90_ee437cf80c73);
pub const CLSID_WICDdsEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa61dde94_66ce_4ac1_881b_71680588895e);
pub const CLSID_WICDdsMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276c88ca_7533_4a86_b676_66b36080d484);
pub const CLSID_WICDdsMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd688bbd_31ed_4db7_a723_934927d38367);
pub const CLSID_WICDefaultFormatConverter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a3f11dc_b514_4b17_8c5f_2154513852f1);
pub const CLSID_WICExifMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9403860_297f_4a49_bf9b_77898150a442);
pub const CLSID_WICExifMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9a14cda_c339_460b_9078_d4debcfabe91);
pub const CLSID_WICFormatConverterHighColor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac75d454_9f37_48f8_b972_4e19bc856011);
pub const CLSID_WICFormatConverterNChannel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17cabb2_d4a3_47d7_a557_339b2efbd4f1);
pub const CLSID_WICFormatConverterWMPhoto: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cb5172b_d600_46ba_ab77_77bb7e3a00d9);
pub const CLSID_WICGCEMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb92e345d_f52d_41f3_b562_081bc772e3b9);
pub const CLSID_WICGCEMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf95dc76_16b2_47f4_b3ea_3c31796693e7);
pub const CLSID_WICGifCommentMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32557d3b_69dc_4f95_836e_f5972b2f6159);
pub const CLSID_WICGifCommentMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa02797fc_c4ae_418c_af95_e637c7ead2a1);
pub const CLSID_WICGifDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x381dda3c_9ce9_4834_a23e_1f98f8fc52be);
pub const CLSID_WICGifEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x114f5598_0b22_40a0_86a1_c83ea495adbd);
pub const CLSID_WICGpsMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3697790b_223b_484e_9925_c4869218f17a);
pub const CLSID_WICGpsMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb8c13e4_62b5_4c96_a48b_6ba6ace39c76);
pub const CLSID_WICHeifDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9a4a80a_44fe_4de4_8971_7150b10a5199);
pub const CLSID_WICHeifEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dbecec1_9eb3_4860_9c6f_ddbe86634575);
pub const CLSID_WICHeifHDRMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2438de3d_94d9_4be8_84a8_4de95a575e75);
pub const CLSID_WICHeifMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacddfc3f_85ec_41bc_bdef_1bc262e4db05);
pub const CLSID_WICHeifMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ae45e79_40bc_4401_ace5_dd3cb16e6afe);
pub const CLSID_WICIMDMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7447a267_0015_42c8_a8f1_fb3b94c68361);
pub const CLSID_WICIMDMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c89071f_452e_4e95_9682_9d1024627172);
pub const CLSID_WICIPTCMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICIPTCMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1249b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICIRBMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4dcd3d7_b4c2_47d9_a6bf_b89ba396a4a3);
pub const CLSID_WICIRBMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c5c1935_0235_4434_80bc_251bc1ec39c6);
pub const CLSID_WICIcoDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61bfcdf_2e0f_4aad_a8d7_e06bafebcdfe);
pub const CLSID_WICIfdMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f914656_9d0a_4eb2_9019_0bf96d8a9ee6);
pub const CLSID_WICIfdMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1ebfc28_c9bd_47a2_8d33_b948769777a7);
pub const CLSID_WICImagingCategories: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfae3d380_fea4_4623_8c75_c6b61110b681);
pub const CLSID_WICImagingFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x317d06e8_5f24_433d_bdf7_79ce68d8abc2);
pub const CLSID_WICInteropMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5c8b898_0074_459f_b700_860d4651ea14);
pub const CLSID_WICInteropMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x122ec645_cd7e_44d8_b186_2c8c20c3b50f);
pub const CLSID_WICJpegChrominanceMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50b1904b_f28f_4574_93f4_0bade82c69e9);
pub const CLSID_WICJpegChrominanceMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ff566f0_6e6b_49d4_96e6_b78886692c62);
pub const CLSID_WICJpegCommentMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f66347c_60c4_4c4d_ab58_d2358685f607);
pub const CLSID_WICJpegCommentMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe573236f_55b1_4eda_81ea_9f65db0290d3);
pub const CLSID_WICJpegDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9456a480_e88b_43ea_9e73_0b2d9b71b1ca);
pub const CLSID_WICJpegEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a34f5c1_4a5a_46dc_b644_1f4567e7a676);
pub const CLSID_WICJpegLuminanceMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x356f2f88_05a6_4728_b9a4_1bfbce04d838);
pub const CLSID_WICJpegLuminanceMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d583abc_8a0e_4657_9982_a380ca58fb4b);
pub const CLSID_WICJpegQualcommPhoneEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68ed5c62_f534_4979_b2b3_686a12b2b34c);
pub const CLSID_WICLSDMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41070793_59e4_479a_a1f7_954adc2ef5fc);
pub const CLSID_WICLSDMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73c037e7_e5d9_4954_876a_6da81d6e5768);
pub const CLSID_WICPlanarFormatConverter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x184132b8_32f8_4784_9131_dd7224b23438);
pub const CLSID_WICPngBkgdMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce7a4a6_03e8_4a60_9d15_282ef32ee7da);
pub const CLSID_WICPngBkgdMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68e3f2fd_31ae_4441_bb6a_fd7047525f90);
pub const CLSID_WICPngChrmMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf90b5f36_367b_402a_9dd1_bc0fd59d8f62);
pub const CLSID_WICPngChrmMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe23ce3eb_5608_4e83_bcef_27b1987e51d7);
pub const CLSID_WICPngDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe018945b_aa86_4008_9bd4_6777a1e40c11);
pub const CLSID_WICPngEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27949969_876a_41d7_9447_568f6a35a4dc);
pub const CLSID_WICPngGamaMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3692ca39_e082_4350_9e1f_3704cb083cd5);
pub const CLSID_WICPngGamaMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff036d13_5d4b_46dd_b10f_106693d9fe4f);
pub const CLSID_WICPngHistMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x877a0bb7_a313_4491_87b5_2e6d0594f520);
pub const CLSID_WICPngHistMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a03e749_672e_446e_bf1f_2c11d233b6ff);
pub const CLSID_WICPngIccpMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5d3e63b_cb0f_4628_a478_6d8244be36b1);
pub const CLSID_WICPngIccpMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16671e5f_0ce6_4cc4_9768_e89fe5018ade);
pub const CLSID_WICPngItxtMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaabfb2fa_3e1e_4a8f_8977_5556fb94ea23);
pub const CLSID_WICPngItxtMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31879719_e751_4df8_981d_68dff67704ed);
pub const CLSID_WICPngSrgbMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb40360c_547e_4956_a3b9_d4418859ba66);
pub const CLSID_WICPngSrgbMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ee35c6_87ec_47df_9f22_1d5aad840c82);
pub const CLSID_WICPngTextMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b59afcc_b8c3_408a_b670_89e5fab6fda7);
pub const CLSID_WICPngTextMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ebafb9_253e_4a72_a744_0762d2685683);
pub const CLSID_WICPngTimeMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd94edf02_efe5_4f0d_85c8_f5a68b3000b1);
pub const CLSID_WICPngTimeMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ab78400_b5a3_4d91_8ace_33fcd1499be6);
pub const CLSID_WICRAWDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41945702_8302_44a6_9445_ac98e8afa086);
pub const CLSID_WICSubIfdMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d42f09_ecd1_4b41_b65d_da1fdaa75663);
pub const CLSID_WICSubIfdMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ade5386_8e9b_4f4c_acf2_f0008706b238);
pub const CLSID_WICThumbnailMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICThumbnailMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd049b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICTiffDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb54e85d9_fe23_499f_8b88_6acea713752b);
pub const CLSID_WICTiffEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0131be10_2001_4c5f_a9b0_cc88fab64ce8);
pub const CLSID_WICUnknownMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x699745c2_5066_4b82_a8e3_d40478dbec8c);
pub const CLSID_WICUnknownMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa09cca86_27ba_4f39_9053_121fa4dc08fc);
pub const CLSID_WICWebpAnimMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076f9911_a348_465c_a807_a252f3f2d3de);
pub const CLSID_WICWebpAnmfMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85a10b03_c9f6_439f_be5e_c0fbef67807c);
pub const CLSID_WICWebpDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7693e886_51c9_4070_8419_9f70738ec8fa);
pub const CLSID_WICWmpDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa26cec36_234c_4950_ae16_e34aace71d0d);
pub const CLSID_WICWmpEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac4ce3cb_e1c1_44cd_8215_5a1665509ec2);
pub const CLSID_WICXMPAltMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa94dcc2_b8b0_4898_b835_000aabd74393);
pub const CLSID_WICXMPAltMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076c2a6c_f78f_4c46_a723_3583e70876ea);
pub const CLSID_WICXMPBagMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7e79a30_4f2c_4fab_8d00_394f2d6bbebe);
pub const CLSID_WICXMPBagMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed822c8c_d6be_4301_a631_0e1416bad28f);
pub const CLSID_WICXMPMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b624df_ae11_4948_a65c_351eb0829419);
pub const CLSID_WICXMPMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1765e14e_1bd4_462e_b6b1_590bf1262ac6);
pub const CLSID_WICXMPSeqMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f12e753_fc71_43d7_a51d_92f35977abb5);
pub const CLSID_WICXMPSeqMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d68d1de_d432_4b0f_923a_091183a9bda7);
pub const CLSID_WICXMPStructMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b90d9a_8209_47f7_9c52_e1244bf50ced);
pub const CLSID_WICXMPStructMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22c21f93_7ddb_411c_9b17_c5b7bd064abc);
pub const FACILITY_WINCODEC_ERR: u32 = 2200u32;
pub const GUID_ContainerFormatAdng: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3ff6d0d_38c0_41c4_b1fe_1f3824f17b84);
pub const GUID_ContainerFormatBmp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0af1d87e_fcfe_4188_bdeb_a7906471cbe3);
pub const GUID_ContainerFormatDds: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9967cb95_2e85_4ac8_8ca2_83d7ccd425c9);
pub const GUID_ContainerFormatGif: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f8a5601_7d4d_4cbd_9c82_1bc8d4eeb9a5);
pub const GUID_ContainerFormatHeif: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1e62521_6787_405b_a339_500715b5763f);
pub const GUID_ContainerFormatIco: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a860c4_338f_4c17_919a_fba4b5628f21);
pub const GUID_ContainerFormatJpeg: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const GUID_ContainerFormatPng: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b7cfaf4_713f_473c_bbcd_6137425faeaf);
pub const GUID_ContainerFormatRaw: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe99ce60_f19c_433c_a3ae_00acefa9ca21);
pub const GUID_ContainerFormatTiff: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x163bcc30_e2e9_4f0b_961d_a3e9fdb788a3);
pub const GUID_ContainerFormatWebp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe094b0e2_67f2_45b3_b0ea_115337ca7cf3);
pub const GUID_ContainerFormatWmp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57a37caa_367a_4540_916b_f183c5093a4b);
pub const GUID_MetadataFormat8BIMIPTC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0010568c_0852_4e6a_b191_5c33ac5b0430);
pub const GUID_MetadataFormat8BIMIPTCDigest: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ca32285_9ccd_4786_8bd8_79539db6a006);
pub const GUID_MetadataFormat8BIMResolutionInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x739f305d_81db_43cb_ac5e_55013ef9f003);
pub const GUID_MetadataFormatAPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e043dc2_c967_4e05_875e_618bf67e85c3);
pub const GUID_MetadataFormatApp0: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79007028_268d_45d6_a3c2_354e6a504bc9);
pub const GUID_MetadataFormatApp1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fd3dfc3_f951_492b_817f_69c2e6d9a5b0);
pub const GUID_MetadataFormatApp13: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x326556a2_f502_4354_9cc0_8e3f48eaf6b5);
pub const GUID_MetadataFormatChunkbKGD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe14d3571_6b47_4dea_b60a_87ce0a78dfb7);
pub const GUID_MetadataFormatChunkcHRM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9db3655b_2842_44b3_8067_12e9b375556a);
pub const GUID_MetadataFormatChunkgAMA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf00935a5_1d5d_4cd1_81b2_9324d7eca781);
pub const GUID_MetadataFormatChunkhIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc59a82da_db74_48a4_bd6a_b69c4931ef95);
pub const GUID_MetadataFormatChunkiCCP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb4349ab_b685_450f_91b5_e802e892536c);
pub const GUID_MetadataFormatChunkiTXt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2bec729_0b68_4b77_aa0e_6295a6ac1814);
pub const GUID_MetadataFormatChunksRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc115fd36_cc6f_4e3f_8363_524b87c6b0d9);
pub const GUID_MetadataFormatChunktEXt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x568d8936_c0a9_4923_905d_df2b38238fbc);
pub const GUID_MetadataFormatChunktIME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b00ae2d_e24b_460a_98b6_878bd03072fd);
pub const GUID_MetadataFormatDds: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a064603_8c33_4e60_9c29_136231702d08);
pub const GUID_MetadataFormatExif: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c3c4f9d_b84a_467d_9493_36cfbd59ea57);
pub const GUID_MetadataFormatGCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a25cad8_deeb_4c69_a788_0ec2266dcafd);
pub const GUID_MetadataFormatGifComment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4b6e0e0_cfb4_4ad3_ab33_9aad2355a34a);
pub const GUID_MetadataFormatGps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7134ab8a_9351_44ad_af62_448db6b502ec);
pub const GUID_MetadataFormatHeif: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x817ef3e1_1288_45f4_a852_260d9e7cce83);
pub const GUID_MetadataFormatHeifHDR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x568b8d8a_1e65_438c_8968_d60e1012beb9);
pub const GUID_MetadataFormatIMD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd2bb086_4d52_48dd_9677_db483e85ae8f);
pub const GUID_MetadataFormatIPTC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fab0914_e129_4087_a1d1_bc812d45a7b5);
pub const GUID_MetadataFormatIRB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16100d66_8570_4bb9_b92d_fda4b23ece67);
pub const GUID_MetadataFormatIfd: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x537396c6_2d8a_4bb6_9bf8_2f0a8e2a3adf);
pub const GUID_MetadataFormatInterop: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed686f8e_681f_4c8b_bd41_a8addbf6b3fc);
pub const GUID_MetadataFormatJpegChrominance: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf73d0dcf_cec6_4f85_9b0e_1c3956b1bef7);
pub const GUID_MetadataFormatJpegComment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x220e5f33_afd3_474e_9d31_7d4fe730f557);
pub const GUID_MetadataFormatJpegLuminance: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86908007_edfc_4860_8d4b_4ee6e83e6058);
pub const GUID_MetadataFormatLSD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe256031e_6299_4929_b98d_5ac884afba92);
pub const GUID_MetadataFormatSubIfd: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58a2e128_2db9_4e57_bb14_5177891ed331);
pub const GUID_MetadataFormatThumbnail: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x243dcee9_8703_40ee_8ef0_22a600b8058c);
pub const GUID_MetadataFormatUnknown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa45e592f_9078_4a7c_adb5_4edc4fd61b1f);
pub const GUID_MetadataFormatWebpANIM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc4fda6_78e6_4102_ae35_bcfa1edcc78b);
pub const GUID_MetadataFormatWebpANMF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43c105ee_b93b_4abb_b003_a08c0d870471);
pub const GUID_MetadataFormatXMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb5acc38_f216_4cec_a6c5_5f6e739763a9);
pub const GUID_MetadataFormatXMPAlt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b08a675_91aa_481b_a798_4da94908613b);
pub const GUID_MetadataFormatXMPBag: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x833cca5f_dcb7_4516_806f_6596ab26dce4);
pub const GUID_MetadataFormatXMPSeq: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63e8df02_eb6c_456c_a224_b25e794fd648);
pub const GUID_MetadataFormatXMPStruct: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22383cf1_ed17_4e2e_af17_d85b8f6b30d0);
pub const GUID_VendorMicrosoft: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0e749ca_edef_4589_a73a_ee0e626a2a2b);
pub const GUID_VendorMicrosoftBuiltIn: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x257a30fd_06b6_462b_aea4_63f70b86e533);
pub const GUID_WICPixelFormat112bpp6ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc937);
pub const GUID_WICPixelFormat112bpp7Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92a);
pub const GUID_WICPixelFormat128bpp7ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc938);
pub const GUID_WICPixelFormat128bpp8Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92b);
pub const GUID_WICPixelFormat128bppPRGBAFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91a);
pub const GUID_WICPixelFormat128bppRGBAFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91e);
pub const GUID_WICPixelFormat128bppRGBAFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc919);
pub const GUID_WICPixelFormat128bppRGBFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc941);
pub const GUID_WICPixelFormat128bppRGBFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91b);
pub const GUID_WICPixelFormat144bpp8ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc939);
pub const GUID_WICPixelFormat16bppBGR555: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc909);
pub const GUID_WICPixelFormat16bppBGR565: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90a);
pub const GUID_WICPixelFormat16bppBGRA5551: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05ec7c2b_f1e6_4961_ad46_e1cc810a87d2);
pub const GUID_WICPixelFormat16bppCbCr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff95ba6e_11e0_4263_bb45_01721f3460a4);
pub const GUID_WICPixelFormat16bppCbQuantizedDctCoefficients: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2c4ff61_56a5_49c2_8b5c_4c1925964837);
pub const GUID_WICPixelFormat16bppCrQuantizedDctCoefficients: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fe354f0_1680_42d8_9231_e73c0565bfc1);
pub const GUID_WICPixelFormat16bppGray: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90b);
pub const GUID_WICPixelFormat16bppGrayFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc913);
pub const GUID_WICPixelFormat16bppGrayHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93e);
pub const GUID_WICPixelFormat16bppYQuantizedDctCoefficients: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa355f433_48e8_4a42_84d8_e2aa26ca80a4);
pub const GUID_WICPixelFormat1bppIndexed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc901);
pub const GUID_WICPixelFormat24bpp3Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc920);
pub const GUID_WICPixelFormat24bppBGR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90c);
pub const GUID_WICPixelFormat24bppRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90d);
pub const GUID_WICPixelFormat2bppGray: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc906);
pub const GUID_WICPixelFormat2bppIndexed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc902);
pub const GUID_WICPixelFormat32bpp3ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92e);
pub const GUID_WICPixelFormat32bpp4Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc921);
pub const GUID_WICPixelFormat32bppBGR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90e);
pub const GUID_WICPixelFormat32bppBGR101010: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc914);
pub const GUID_WICPixelFormat32bppBGRA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90f);
pub const GUID_WICPixelFormat32bppCMYK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91c);
pub const GUID_WICPixelFormat32bppGrayFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93f);
pub const GUID_WICPixelFormat32bppGrayFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc911);
pub const GUID_WICPixelFormat32bppPBGRA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc910);
pub const GUID_WICPixelFormat32bppPRGBA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cc4a650_a527_4d37_a916_3142c7ebedba);
pub const GUID_WICPixelFormat32bppR10G10B10A2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x604e1bb5_8a3c_4b65_b11c_bc0b8dd75b7f);
pub const GUID_WICPixelFormat32bppR10G10B10A2HDR10: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c215c5d_1acc_4f0e_a4bc_70fb3ae8fd28);
pub const GUID_WICPixelFormat32bppRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd98c6b95_3efe_47d6_bb25_eb1748ab0cf1);
pub const GUID_WICPixelFormat32bppRGBA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5c7ad2d_6a8d_43dd_a7a8_a29935261ae9);
pub const GUID_WICPixelFormat32bppRGBA1010102: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25238d72_fcf9_4522_b514_5578e5ad55e0);
pub const GUID_WICPixelFormat32bppRGBA1010102XR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00de6b9a_c101_434b_b502_d0165ee1122c);
pub const GUID_WICPixelFormat32bppRGBE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93d);
pub const GUID_WICPixelFormat40bpp4ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92f);
pub const GUID_WICPixelFormat40bpp5Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc922);
pub const GUID_WICPixelFormat40bppCMYKAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92c);
pub const GUID_WICPixelFormat48bpp3Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc926);
pub const GUID_WICPixelFormat48bpp5ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc930);
pub const GUID_WICPixelFormat48bpp6Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc923);
pub const GUID_WICPixelFormat48bppBGR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe605a384_b468_46ce_bb2e_36f180e64313);
pub const GUID_WICPixelFormat48bppBGRFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49ca140e_cab6_493b_9ddf_60187c37532a);
pub const GUID_WICPixelFormat48bppRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc915);
pub const GUID_WICPixelFormat48bppRGBFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc912);
pub const GUID_WICPixelFormat48bppRGBHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93b);
pub const GUID_WICPixelFormat4bppGray: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc907);
pub const GUID_WICPixelFormat4bppIndexed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc903);
pub const GUID_WICPixelFormat56bpp6ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc931);
pub const GUID_WICPixelFormat56bpp7Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc924);
pub const GUID_WICPixelFormat64bpp3ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc934);
pub const GUID_WICPixelFormat64bpp4Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc927);
pub const GUID_WICPixelFormat64bpp7ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc932);
pub const GUID_WICPixelFormat64bpp8Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc925);
pub const GUID_WICPixelFormat64bppBGRA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1562ff7c_d352_46f9_979e_42976b792246);
pub const GUID_WICPixelFormat64bppBGRAFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x356de33c_54d2_4a23_bb04_9b7bf9b1d42d);
pub const GUID_WICPixelFormat64bppCMYK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91f);
pub const GUID_WICPixelFormat64bppPBGRA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c518e8e_a4ec_468b_ae70_c9a35a9c5530);
pub const GUID_WICPixelFormat64bppPRGBA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc917);
pub const GUID_WICPixelFormat64bppPRGBAHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58ad26c2_c623_4d9d_b320_387e49f8c442);
pub const GUID_WICPixelFormat64bppRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1182111_186d_4d42_bc6a_9c8303a8dff9);
pub const GUID_WICPixelFormat64bppRGBA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc916);
pub const GUID_WICPixelFormat64bppRGBAFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91d);
pub const GUID_WICPixelFormat64bppRGBAHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93a);
pub const GUID_WICPixelFormat64bppRGBFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc940);
pub const GUID_WICPixelFormat64bppRGBHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc942);
pub const GUID_WICPixelFormat72bpp8ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc933);
pub const GUID_WICPixelFormat80bpp4ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc935);
pub const GUID_WICPixelFormat80bpp5Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc928);
pub const GUID_WICPixelFormat80bppCMYKAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92d);
pub const GUID_WICPixelFormat8bppAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6cd0116_eeba_4161_aa85_27dd9fb3a895);
pub const GUID_WICPixelFormat8bppCb: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1339f224_6bfe_4c3e_9302_e4f3a6d0ca2a);
pub const GUID_WICPixelFormat8bppCr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8145053_2116_49f0_8835_ed844b205c51);
pub const GUID_WICPixelFormat8bppGray: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc908);
pub const GUID_WICPixelFormat8bppIndexed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc904);
pub const GUID_WICPixelFormat8bppY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91b4db54_2df9_42f0_b449_2909bb3df88e);
pub const GUID_WICPixelFormat96bpp5ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc936);
pub const GUID_WICPixelFormat96bpp6Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc929);
pub const GUID_WICPixelFormat96bppRGBFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc918);
pub const GUID_WICPixelFormat96bppRGBFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3fed78f_e8db_4acf_84c1_e97f6136b327);
pub const GUID_WICPixelFormatBlackWhite: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc905);
pub const GUID_WICPixelFormatDontCare: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc900);
#[repr(transparent)]
pub struct IWICBitmap(::windows::core::IUnknown);
impl IWICBitmap {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn Lock(&self, prclock: *const WICRect, flags: u32) -> ::windows::core::Result<IWICBitmapLock> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(prclock), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapLock>(result__)
    }
    pub unsafe fn SetPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dpix), ::core::mem::transmute(dpiy)).ok()
    }
}
impl ::core::convert::From<IWICBitmap> for IWICBitmapSource {
    fn from(value: IWICBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmap> for IWICBitmapSource {
    fn from(value: &IWICBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmap> for ::windows::core::IUnknown {
    fn from(value: IWICBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmap> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmap {}
unsafe impl ::windows::core::Interface for IWICBitmap {
    type Vtable = IWICBitmapVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000121_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapClipper(::windows::core::IUnknown);
impl IWICBitmapClipper {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pisource: Param0, prc: *const WICRect) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pisource.into_param().abi(), ::core::mem::transmute(prc)).ok()
    }
}
impl ::core::convert::From<IWICBitmapClipper> for IWICBitmapSource {
    fn from(value: IWICBitmapClipper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapClipper> for IWICBitmapSource {
    fn from(value: &IWICBitmapClipper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICBitmapClipper {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICBitmapClipper {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapClipper> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapClipper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapClipper> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapClipper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapClipper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapClipper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapClipper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapClipper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapClipper {}
unsafe impl ::windows::core::Interface for IWICBitmapClipper {
    type Vtable = IWICBitmapClipperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4fbcf03_223d_4e81_9333_d635556dd1b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapClipperVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapCodecInfo(::windows::core::IUnknown);
impl IWICBitmapCodecInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetPixelFormats(&self, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(cformats), ::core::mem::transmute(pguidpixelformats), ::core::mem::transmute(pcactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColorManagementVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchcolormanagementversion: u32, wzcolormanagementversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchcolormanagementversion), wzcolormanagementversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceManufacturer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemanufacturer), wzdevicemanufacturer.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceModels<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemodels: u32, wzdevicemodels: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemodels), wzdevicemodels.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMimeTypes<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchmimetypes: u32, wzmimetypes: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchmimetypes), wzmimetypes.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileExtensions<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfileextensions: u32, wzfileextensions: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfileextensions), wzfileextensions.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesMimeType<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzmimetype: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), wzmimetype.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWICBitmapCodecInfo> for IWICComponentInfo {
    fn from(value: IWICBitmapCodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapCodecInfo> for IWICComponentInfo {
    fn from(value: &IWICBitmapCodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICBitmapCodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICBitmapCodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapCodecInfo> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapCodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapCodecInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapCodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapCodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapCodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapCodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapCodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapCodecInfo {}
unsafe impl ::windows::core::Interface for IWICBitmapCodecInfo {
    type Vtable = IWICBitmapCodecInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe87a44c4_b76e_4c47_8b09_298eb12a2714);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapCodecInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzmimetype: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWICBitmapCodecProgressNotification(::windows::core::IUnknown);
impl IWICBitmapCodecProgressNotification {
    pub unsafe fn RegisterProgressNotification(&self, pfnprogressnotification: PFNProgressNotification, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfnprogressnotification), ::core::mem::transmute(pvdata), ::core::mem::transmute(dwprogressflags)).ok()
    }
}
impl ::core::convert::From<IWICBitmapCodecProgressNotification> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapCodecProgressNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapCodecProgressNotification> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapCodecProgressNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapCodecProgressNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapCodecProgressNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapCodecProgressNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapCodecProgressNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapCodecProgressNotification {}
unsafe impl ::windows::core::Interface for IWICBitmapCodecProgressNotification {
    type Vtable = IWICBitmapCodecProgressNotificationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64c1024e_c3cf_4462_8078_88c2b11c46d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapCodecProgressNotificationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfnprogressnotification: ::windows::core::RawPtr, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapDecoder(::windows::core::IUnknown);
impl IWICBitmapDecoder {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCapability<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0, cacheoptions: WICDecodeOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(cacheoptions)).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetDecoderInfo(&self) -> ::windows::core::Result<IWICBitmapDecoderInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoderInfo>(result__)
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryReader>(result__)
    }
    pub unsafe fn GetPreview(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapSource>(result__)
    }
    pub unsafe fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccount), ::core::mem::transmute(ppicolorcontexts), ::core::mem::transmute(pcactualcount)).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapSource>(result__)
    }
    pub unsafe fn GetFrameCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFrame(&self, index: u32) -> ::windows::core::Result<IWICBitmapFrameDecode> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapFrameDecode>(result__)
    }
}
impl ::core::convert::From<IWICBitmapDecoder> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapDecoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapDecoder> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapDecoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapDecoder {}
unsafe impl ::windows::core::Interface for IWICBitmapDecoder {
    type Vtable = IWICBitmapDecoderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9edde9e7_8dee_47ea_99df_e6faf2ed44bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapDecoderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pdwcapability: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoptions: WICDecodeOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidecoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapDecoderInfo(::windows::core::IUnknown);
impl IWICBitmapDecoderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetPixelFormats(&self, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(cformats), ::core::mem::transmute(pguidpixelformats), ::core::mem::transmute(pcactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColorManagementVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchcolormanagementversion: u32, wzcolormanagementversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchcolormanagementversion), wzcolormanagementversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceManufacturer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemanufacturer), wzdevicemanufacturer.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceModels<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemodels: u32, wzdevicemodels: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemodels), wzdevicemodels.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMimeTypes<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchmimetypes: u32, wzmimetypes: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchmimetypes), wzmimetypes.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileExtensions<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfileextensions: u32, wzfileextensions: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfileextensions), wzfileextensions.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesMimeType<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzmimetype: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), wzmimetype.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPatterns(&self, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsizepatterns), ::core::mem::transmute(ppatterns), ::core::mem::transmute(pcpatterns), ::core::mem::transmute(pcbpatternsactual)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn MatchesPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
}
impl ::core::convert::From<IWICBitmapDecoderInfo> for IWICBitmapCodecInfo {
    fn from(value: IWICBitmapDecoderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapDecoderInfo> for IWICBitmapCodecInfo {
    fn from(value: &IWICBitmapDecoderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapCodecInfo> for IWICBitmapDecoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapCodecInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapCodecInfo> for &IWICBitmapDecoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapCodecInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapDecoderInfo> for IWICComponentInfo {
    fn from(value: IWICBitmapDecoderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapDecoderInfo> for IWICComponentInfo {
    fn from(value: &IWICBitmapDecoderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICBitmapDecoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICBitmapDecoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapDecoderInfo> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapDecoderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapDecoderInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapDecoderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapDecoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapDecoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapDecoderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapDecoderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapDecoderInfo {}
unsafe impl ::windows::core::Interface for IWICBitmapDecoderInfo {
    type Vtable = IWICBitmapDecoderInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8cd007f_d08f_4191_9bfc_236ea7f0e4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapDecoderInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzmimetype: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapEncoder(::windows::core::IUnknown);
impl IWICBitmapEncoder {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(cacheoption)).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetEncoderInfo(&self) -> ::windows::core::Result<IWICBitmapEncoderInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapEncoderInfo>(result__)
    }
    pub unsafe fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccount), ::core::mem::transmute(ppicolorcontext)).ok()
    }
    pub unsafe fn SetPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn SetThumbnail<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pithumbnail: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pithumbnail.into_param().abi()).ok()
    }
    pub unsafe fn SetPreview<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pipreview: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pipreview.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateNewFrame(&self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut ::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppiframeencode), ::core::mem::transmute(ppiencoderoptions)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryWriter>(result__)
    }
}
impl ::core::convert::From<IWICBitmapEncoder> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapEncoder> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapEncoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapEncoder {}
unsafe impl ::windows::core::Interface for IWICBitmapEncoder {
    type Vtable = IWICBitmapEncoderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000103_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapEncoderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiencoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipreview: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, ppiencoderoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapEncoderInfo(::windows::core::IUnknown);
impl IWICBitmapEncoderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetPixelFormats(&self, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(cformats), ::core::mem::transmute(pguidpixelformats), ::core::mem::transmute(pcactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColorManagementVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchcolormanagementversion: u32, wzcolormanagementversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchcolormanagementversion), wzcolormanagementversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceManufacturer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemanufacturer), wzdevicemanufacturer.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceModels<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemodels: u32, wzdevicemodels: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemodels), wzdevicemodels.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMimeTypes<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchmimetypes: u32, wzmimetypes: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchmimetypes), wzmimetypes.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileExtensions<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfileextensions: u32, wzfileextensions: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfileextensions), wzfileextensions.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesMimeType<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzmimetype: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), wzmimetype.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICBitmapEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapEncoder>(result__)
    }
}
impl ::core::convert::From<IWICBitmapEncoderInfo> for IWICBitmapCodecInfo {
    fn from(value: IWICBitmapEncoderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapEncoderInfo> for IWICBitmapCodecInfo {
    fn from(value: &IWICBitmapEncoderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapCodecInfo> for IWICBitmapEncoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapCodecInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapCodecInfo> for &IWICBitmapEncoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapCodecInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapEncoderInfo> for IWICComponentInfo {
    fn from(value: IWICBitmapEncoderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapEncoderInfo> for IWICComponentInfo {
    fn from(value: &IWICBitmapEncoderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICBitmapEncoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICBitmapEncoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapEncoderInfo> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapEncoderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapEncoderInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapEncoderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapEncoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapEncoderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapEncoderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapEncoderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapEncoderInfo {}
unsafe impl ::windows::core::Interface for IWICBitmapEncoderInfo {
    type Vtable = IWICBitmapEncoderInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94c9b4ee_a09f_4f92_8a1e_4a9bce7e76fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapEncoderInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzmimetype: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapFlipRotator(::windows::core::IUnknown);
impl IWICBitmapFlipRotator {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pisource: Param0, options: WICBitmapTransformOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pisource.into_param().abi(), ::core::mem::transmute(options)).ok()
    }
}
impl ::core::convert::From<IWICBitmapFlipRotator> for IWICBitmapSource {
    fn from(value: IWICBitmapFlipRotator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapFlipRotator> for IWICBitmapSource {
    fn from(value: &IWICBitmapFlipRotator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICBitmapFlipRotator {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICBitmapFlipRotator {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapFlipRotator> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapFlipRotator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapFlipRotator> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapFlipRotator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapFlipRotator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapFlipRotator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapFlipRotator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapFlipRotator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapFlipRotator {}
unsafe impl ::windows::core::Interface for IWICBitmapFlipRotator {
    type Vtable = IWICBitmapFlipRotatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5009834f_2d6a_41ce_9e1b_17c5aff7a782);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFlipRotatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, options: WICBitmapTransformOptions) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapFrameDecode(::windows::core::IUnknown);
impl IWICBitmapFrameDecode {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryReader>(result__)
    }
    pub unsafe fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccount), ::core::mem::transmute(ppicolorcontexts), ::core::mem::transmute(pcactualcount)).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapSource>(result__)
    }
}
impl ::core::convert::From<IWICBitmapFrameDecode> for IWICBitmapSource {
    fn from(value: IWICBitmapFrameDecode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapFrameDecode> for IWICBitmapSource {
    fn from(value: &IWICBitmapFrameDecode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICBitmapFrameDecode {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICBitmapFrameDecode {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapFrameDecode> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapFrameDecode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapFrameDecode> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapFrameDecode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapFrameDecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapFrameDecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapFrameDecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapFrameDecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapFrameDecode {}
unsafe impl ::windows::core::Interface for IWICBitmapFrameDecode {
    type Vtable = IWICBitmapFrameDecodeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b16811b_6a43_4ec9_a813_3d930c13b940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFrameDecodeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapFrameEncode(::windows::core::IUnknown);
impl IWICBitmapFrameEncode {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::StructuredStorage::IPropertyBag2>>(&self, piencoderoptions: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), piencoderoptions.into_param().abi()).ok()
    }
    pub unsafe fn SetSize(&self, uiwidth: u32, uiheight: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight)).ok()
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dpix), ::core::mem::transmute(dpiy)).ok()
    }
    pub unsafe fn SetPixelFormat(&self, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppixelformat)).ok()
    }
    pub unsafe fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccount), ::core::mem::transmute(ppicolorcontext)).ok()
    }
    pub unsafe fn SetPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn SetThumbnail<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pithumbnail: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pithumbnail.into_param().abi()).ok()
    }
    pub unsafe fn WritePixels(&self, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(linecount), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbpixels)).ok()
    }
    pub unsafe fn WriteSource<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pibitmapsource: Param0, prc: *const WICRect) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(prc)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryWriter>(result__)
    }
}
impl ::core::convert::From<IWICBitmapFrameEncode> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapFrameEncode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapFrameEncode> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapFrameEncode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapFrameEncode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapFrameEncode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapFrameEncode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapFrameEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapFrameEncode {}
unsafe impl ::windows::core::Interface for IWICBitmapFrameEncode {
    type Vtable = IWICBitmapFrameEncodeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000105_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFrameEncodeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piencoderoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapLock(::windows::core::IUnknown);
impl IWICBitmapLock {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetStride(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcbbuffersize), ::core::mem::transmute(ppbdata)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IWICBitmapLock> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapLock> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapLock {}
unsafe impl ::windows::core::Interface for IWICBitmapLock {
    type Vtable = IWICBitmapLockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000123_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapLockVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbstride: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapScaler(::windows::core::IUnknown);
impl IWICBitmapScaler {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pisource: Param0, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pisource.into_param().abi(), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(mode)).ok()
    }
}
impl ::core::convert::From<IWICBitmapScaler> for IWICBitmapSource {
    fn from(value: IWICBitmapScaler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapScaler> for IWICBitmapSource {
    fn from(value: &IWICBitmapScaler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICBitmapScaler {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICBitmapScaler {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICBitmapScaler> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapScaler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapScaler> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapScaler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapScaler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapScaler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapScaler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapScaler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapScaler {}
unsafe impl ::windows::core::Interface for IWICBitmapScaler {
    type Vtable = IWICBitmapScalerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000302_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapScalerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapSource(::windows::core::IUnknown);
impl IWICBitmapSource {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
}
impl ::core::convert::From<IWICBitmapSource> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapSource> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapSource {}
unsafe impl ::windows::core::Interface for IWICBitmapSource {
    type Vtable = IWICBitmapSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000120_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICBitmapSourceTransform(::windows::core::IUnknown);
impl IWICBitmapSourceTransform {
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pguiddstformat), ::core::mem::transmute(dsttransform), ::core::mem::transmute(nstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn GetClosestSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetClosestPixelFormat(&self, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguiddstformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportTransform(&self, dsttransform: WICBitmapTransformOptions) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dsttransform), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWICBitmapSourceTransform> for ::windows::core::IUnknown {
    fn from(value: IWICBitmapSourceTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICBitmapSourceTransform> for ::windows::core::IUnknown {
    fn from(value: &IWICBitmapSourceTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICBitmapSourceTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICBitmapSourceTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICBitmapSourceTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICBitmapSourceTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapSourceTransform {}
unsafe impl ::windows::core::Interface for IWICBitmapSourceTransform {
    type Vtable = IWICBitmapSourceTransformVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b16811b_6a43_4ec9_b713_3d5a0c13b940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapSourceTransformVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWICColorContext(::windows::core::IUnknown);
impl IWICColorContext {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromFilename<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wzfilename.into_param().abi()).ok()
    }
    pub unsafe fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(cbbuffersize)).ok()
    }
    pub unsafe fn InitializeFromExifColorSpace(&self, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<WICColorContextType> {
        let mut result__: WICColorContextType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICColorContextType>(result__)
    }
    pub unsafe fn GetProfileBytes(&self, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbactual)).ok()
    }
    pub unsafe fn GetExifColorSpace(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWICColorContext> for ::windows::core::IUnknown {
    fn from(value: IWICColorContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICColorContext> for ::windows::core::IUnknown {
    fn from(value: &IWICColorContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICColorContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICColorContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICColorContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICColorContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICColorContext {}
unsafe impl ::windows::core::Interface for IWICColorContext {
    type Vtable = IWICColorContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c613a02_34b2_44ea_9a7c_45aea9c6fd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICColorContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICColorContextType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICColorTransform(::windows::core::IUnknown);
impl IWICColorTransform {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>, Param1: ::windows::core::IntoParam<'a, IWICColorContext>, Param2: ::windows::core::IntoParam<'a, IWICColorContext>>(&self, pibitmapsource: Param0, picontextsource: Param1, picontextdest: Param2, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), picontextsource.into_param().abi(), picontextdest.into_param().abi(), ::core::mem::transmute(pixelfmtdest)).ok()
    }
}
impl ::core::convert::From<IWICColorTransform> for IWICBitmapSource {
    fn from(value: IWICColorTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICColorTransform> for IWICBitmapSource {
    fn from(value: &IWICColorTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICColorTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICColorTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICColorTransform> for ::windows::core::IUnknown {
    fn from(value: IWICColorTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICColorTransform> for ::windows::core::IUnknown {
    fn from(value: &IWICColorTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICColorTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICColorTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICColorTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICColorTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICColorTransform {}
unsafe impl ::windows::core::Interface for IWICColorTransform {
    type Vtable = IWICColorTransformVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb66f034f_d0e2_40ab_b436_6de39e321a94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICColorTransformVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, picontextsource: ::windows::core::RawPtr, picontextdest: ::windows::core::RawPtr, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICComponentFactory(::windows::core::IUnknown);
impl IWICComponentFactory {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDecoderFromFilename<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzfilename: Param0, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wzfilename.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hfile), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<IWICComponentInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsidcomponent), ::core::mem::transmute(&mut result__)).from_abi::<IWICComponentInfo>(result__)
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapEncoder>(result__)
    }
    pub unsafe fn CreatePalette(&self) -> ::windows::core::Result<IWICPalette> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICPalette>(result__)
    }
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::core::Result<IWICFormatConverter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICFormatConverter>(result__)
    }
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::core::Result<IWICBitmapScaler> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapScaler>(result__)
    }
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::core::Result<IWICBitmapClipper> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapClipper>(result__)
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<IWICBitmapFlipRotator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapFlipRotator>(result__)
    }
    pub unsafe fn CreateStream(&self) -> ::windows::core::Result<IWICStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICStream>(result__)
    }
    pub unsafe fn CreateColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICColorContext>(result__)
    }
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::core::Result<IWICColorTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICColorTransform>(result__)
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(option), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromSource<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pibitmapsource: Param0, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(option), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromSourceRect<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pibitmapsource: Param0, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HBITMAP>, Param1: ::windows::core::IntoParam<'a, super::Gdi::HPALETTE>>(&self, hbitmap: Param0, hpalette: Param1, options: WICBitmapAlphaChannelOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), hbitmap.into_param().abi(), hpalette.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::WindowsAndMessaging::HICON>>(&self, hicon: Param0) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), hicon.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(componenttypes), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumUnknown>(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapDecoder>>(&self, pidecoder: Param0) -> ::windows::core::Result<IWICFastMetadataEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pidecoder.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICFastMetadataEncoder>(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapFrameDecode>>(&self, piframedecoder: Param0) -> ::windows::core::Result<IWICFastMetadataEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), piframedecoder.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICFastMetadataEncoder>(result__)
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryWriter>(result__)
    }
    pub unsafe fn CreateQueryWriterFromReader<'a, Param0: ::windows::core::IntoParam<'a, IWICMetadataQueryReader>>(&self, piqueryreader: Param0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), piqueryreader.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMetadataReader<'a, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: Param3) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwoptions), pistream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMetadataReaderFromContainer<'a, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: Param3) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwoptions), pistream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataReader>(result__)
    }
    pub unsafe fn CreateMetadataWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32) -> ::windows::core::Result<IWICMetadataWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwmetadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataWriter>(result__)
    }
    pub unsafe fn CreateMetadataWriterFromReader<'a, Param0: ::windows::core::IntoParam<'a, IWICMetadataReader>>(&self, pireader: Param0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), pireader.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataWriter>(result__)
    }
    pub unsafe fn CreateQueryReaderFromBlockReader<'a, Param0: ::windows::core::IntoParam<'a, IWICMetadataBlockReader>>(&self, piblockreader: Param0) -> ::windows::core::Result<IWICMetadataQueryReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), piblockreader.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryReader>(result__)
    }
    pub unsafe fn CreateQueryWriterFromBlockWriter<'a, Param0: ::windows::core::IntoParam<'a, IWICMetadataBlockWriter>>(&self, piblockwriter: Param0) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), piblockwriter.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CreateEncoderPropertyBag(&self, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropoptions), ::core::mem::transmute(ccount), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::IPropertyBag2>(result__)
    }
}
impl ::core::convert::From<IWICComponentFactory> for IWICImagingFactory {
    fn from(value: IWICComponentFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICComponentFactory> for IWICImagingFactory {
    fn from(value: &IWICComponentFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICImagingFactory> for IWICComponentFactory {
    fn into_param(self) -> ::windows::core::Param<'a, IWICImagingFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICImagingFactory> for &IWICComponentFactory {
    fn into_param(self) -> ::windows::core::Param<'a, IWICImagingFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICComponentFactory> for ::windows::core::IUnknown {
    fn from(value: IWICComponentFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICComponentFactory> for ::windows::core::IUnknown {
    fn from(value: &IWICComponentFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICComponentFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICComponentFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICComponentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICComponentFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICComponentFactory {}
unsafe impl ::windows::core::Interface for IWICComponentFactory {
    type Vtable = IWICComponentFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x412d0c3a_9650_44fa_af5b_dd2a06c8e8fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICComponentFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidcomponent: *const ::windows::core::GUID, ppiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppipalette: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwicstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piframedecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piqueryreader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pireader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piblockreader: ::windows::core::RawPtr, ppiqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piblockwriter: ::windows::core::RawPtr, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[repr(transparent)]
pub struct IWICComponentInfo(::windows::core::IUnknown);
impl IWICComponentInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
}
impl ::core::convert::From<IWICComponentInfo> for ::windows::core::IUnknown {
    fn from(value: IWICComponentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICComponentInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICComponentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICComponentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICComponentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICComponentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICComponentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICComponentInfo {}
unsafe impl ::windows::core::Interface for IWICComponentInfo {
    type Vtable = IWICComponentInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23bc3f0a_698b_4357_886b_f24d50671334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICComponentInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWICDdsDecoder(::windows::core::IUnknown);
impl IWICDdsDecoder {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetParameters(&self) -> ::windows::core::Result<WICDdsParameters> {
        let mut result__: WICDdsParameters = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICDdsParameters>(result__)
    }
    pub unsafe fn GetFrame(&self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> ::windows::core::Result<IWICBitmapFrameDecode> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(arrayindex), ::core::mem::transmute(miplevel), ::core::mem::transmute(sliceindex), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapFrameDecode>(result__)
    }
}
impl ::core::convert::From<IWICDdsDecoder> for ::windows::core::IUnknown {
    fn from(value: IWICDdsDecoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICDdsDecoder> for ::windows::core::IUnknown {
    fn from(value: &IWICDdsDecoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICDdsDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICDdsDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICDdsDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICDdsDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDdsDecoder {}
unsafe impl ::windows::core::Interface for IWICDdsDecoder {
    type Vtable = IWICDdsDecoderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x409cd537_8532_40cb_9774_e2feb2df4e9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsDecoderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICDdsEncoder(::windows::core::IUnknown);
impl IWICDdsEncoder {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetParameters(&self, pparameters: *const WICDdsParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pparameters)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetParameters(&self) -> ::windows::core::Result<WICDdsParameters> {
        let mut result__: WICDdsParameters = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICDdsParameters>(result__)
    }
    pub unsafe fn CreateNewFrame(&self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppiframeencode), ::core::mem::transmute(parrayindex), ::core::mem::transmute(pmiplevel), ::core::mem::transmute(psliceindex)).ok()
    }
}
impl ::core::convert::From<IWICDdsEncoder> for ::windows::core::IUnknown {
    fn from(value: IWICDdsEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICDdsEncoder> for ::windows::core::IUnknown {
    fn from(value: &IWICDdsEncoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICDdsEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICDdsEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICDdsEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICDdsEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDdsEncoder {}
unsafe impl ::windows::core::Interface for IWICDdsEncoder {
    type Vtable = IWICDdsEncoderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cacdb4c_407e_41b3_b936_d0f010cd6732);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsEncoderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *const WICDdsParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICDdsFrameDecode(::windows::core::IUnknown);
impl IWICDdsFrameDecode {
    pub unsafe fn GetSizeInBlocks(&self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwidthinblocks), ::core::mem::transmute(pheightinblocks)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFormatInfo(&self) -> ::windows::core::Result<WICDdsFormatInfo> {
        let mut result__: WICDdsFormatInfo = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICDdsFormatInfo>(result__)
    }
    pub unsafe fn CopyBlocks(&self, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcboundsinblocks), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
}
impl ::core::convert::From<IWICDdsFrameDecode> for ::windows::core::IUnknown {
    fn from(value: IWICDdsFrameDecode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICDdsFrameDecode> for ::windows::core::IUnknown {
    fn from(value: &IWICDdsFrameDecode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICDdsFrameDecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICDdsFrameDecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICDdsFrameDecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICDdsFrameDecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDdsFrameDecode {}
unsafe impl ::windows::core::Interface for IWICDdsFrameDecode {
    type Vtable = IWICDdsFrameDecodeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d4c0c61_18a4_41e4_bd80_481a4fc9f464);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsFrameDecodeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICDevelopRaw(::windows::core::IUnknown);
impl IWICDevelopRaw {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryReader>(result__)
    }
    pub unsafe fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccount), ::core::mem::transmute(ppicolorcontexts), ::core::mem::transmute(pcactualcount)).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapSource>(result__)
    }
    pub unsafe fn QueryRawCapabilitiesInfo(&self, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo)).ok()
    }
    pub unsafe fn LoadParameterSet(&self, parameterset: WICRawParameterSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameterset)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetCurrentParameterSet(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::IPropertyBag2>(result__)
    }
    pub unsafe fn SetExposureCompensation(&self, ev: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ev)).ok()
    }
    pub unsafe fn GetExposureCompensation(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetWhitePointRGB(&self, red: u32, green: u32, blue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)).ok()
    }
    pub unsafe fn GetWhitePointRGB(&self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pred), ::core::mem::transmute(pgreen), ::core::mem::transmute(pblue)).ok()
    }
    pub unsafe fn SetNamedWhitePoint(&self, whitepoint: WICNamedWhitePoint) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(whitepoint)).ok()
    }
    pub unsafe fn GetNamedWhitePoint(&self) -> ::windows::core::Result<WICNamedWhitePoint> {
        let mut result__: WICNamedWhitePoint = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICNamedWhitePoint>(result__)
    }
    pub unsafe fn SetWhitePointKelvin(&self, whitepointkelvin: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(whitepointkelvin)).ok()
    }
    pub unsafe fn GetWhitePointKelvin(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetKelvinRangeInfo(&self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(pminkelvintemp), ::core::mem::transmute(pmaxkelvintemp), ::core::mem::transmute(pkelvintempstepvalue)).ok()
    }
    pub unsafe fn SetContrast(&self, contrast: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(contrast)).ok()
    }
    pub unsafe fn GetContrast(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetGamma(&self, gamma: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(gamma)).ok()
    }
    pub unsafe fn GetGamma(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetSharpness(&self, sharpness: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharpness)).ok()
    }
    pub unsafe fn GetSharpness(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetSaturation(&self, saturation: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(saturation)).ok()
    }
    pub unsafe fn GetSaturation(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetTint(&self, tint: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(tint)).ok()
    }
    pub unsafe fn GetTint(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetNoiseReduction(&self, noisereduction: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(noisereduction)).ok()
    }
    pub unsafe fn GetNoiseReduction(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetDestinationColorContext<'a, Param0: ::windows::core::IntoParam<'a, IWICColorContext>>(&self, pcolorcontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), pcolorcontext.into_param().abi()).ok()
    }
    pub unsafe fn SetToneCurve(&self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbtonecurvesize), ::core::mem::transmute(ptonecurve)).ok()
    }
    pub unsafe fn GetToneCurve(&self, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbtonecurvebuffersize), ::core::mem::transmute(ptonecurve), ::core::mem::transmute(pcbactualtonecurvebuffersize)).ok()
    }
    pub unsafe fn SetRotation(&self, rotation: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(rotation)).ok()
    }
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetRenderMode(&self, rendermode: WICRawRenderMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(rendermode)).ok()
    }
    pub unsafe fn GetRenderMode(&self) -> ::windows::core::Result<WICRawRenderMode> {
        let mut result__: WICRawRenderMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICRawRenderMode>(result__)
    }
    pub unsafe fn SetNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IWICDevelopRawNotificationCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWICDevelopRaw> for IWICBitmapFrameDecode {
    fn from(value: IWICDevelopRaw) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICDevelopRaw> for IWICBitmapFrameDecode {
    fn from(value: &IWICDevelopRaw) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapFrameDecode> for IWICDevelopRaw {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapFrameDecode> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapFrameDecode> for &IWICDevelopRaw {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapFrameDecode> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICDevelopRaw> for IWICBitmapSource {
    fn from(value: IWICDevelopRaw) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICDevelopRaw> for IWICBitmapSource {
    fn from(value: &IWICDevelopRaw) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICDevelopRaw {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICDevelopRaw {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICDevelopRaw> for ::windows::core::IUnknown {
    fn from(value: IWICDevelopRaw) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICDevelopRaw> for ::windows::core::IUnknown {
    fn from(value: &IWICDevelopRaw) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICDevelopRaw {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICDevelopRaw {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICDevelopRaw {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICDevelopRaw {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDevelopRaw {}
unsafe impl ::windows::core::Interface for IWICDevelopRaw {
    type Vtable = IWICDevelopRawVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbec5e44_f7be_4b65_b7f8_c0c81fef026d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDevelopRawVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterset: WICRawParameterSet) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcurrentparameterset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ev: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pev: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, red: u32, green: u32, blue: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointkelvin: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwhitepointkelvin: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contrast: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontrast: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamma: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgamma: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharpness: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharpness: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, saturation: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psaturation: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tint: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptint: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noisereduction: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnoisereduction: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protation: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendermode: WICRawRenderMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICDevelopRawNotificationCallback(::windows::core::IUnknown);
impl IWICDevelopRawNotificationCallback {
    pub unsafe fn Notify(&self, notificationmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationmask)).ok()
    }
}
impl ::core::convert::From<IWICDevelopRawNotificationCallback> for ::windows::core::IUnknown {
    fn from(value: IWICDevelopRawNotificationCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICDevelopRawNotificationCallback> for ::windows::core::IUnknown {
    fn from(value: &IWICDevelopRawNotificationCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICDevelopRawNotificationCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICDevelopRawNotificationCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICDevelopRawNotificationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICDevelopRawNotificationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDevelopRawNotificationCallback {}
unsafe impl ::windows::core::Interface for IWICDevelopRawNotificationCallback {
    type Vtable = IWICDevelopRawNotificationCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95c75a6e_3e8c_4ec2_85a8_aebcc551e59b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDevelopRawNotificationCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationmask: u32) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IWICEnumMetadataItem(::windows::core::IUnknown);
impl IWICEnumMetadataItem {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Next(&self, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgeltschema), ::core::mem::transmute(rgeltid), ::core::mem::transmute(rgeltvalue), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWICEnumMetadataItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICEnumMetadataItem>(result__)
    }
}
impl ::core::convert::From<IWICEnumMetadataItem> for ::windows::core::IUnknown {
    fn from(value: IWICEnumMetadataItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICEnumMetadataItem> for ::windows::core::IUnknown {
    fn from(value: &IWICEnumMetadataItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICEnumMetadataItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICEnumMetadataItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICEnumMetadataItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICEnumMetadataItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICEnumMetadataItem {}
unsafe impl ::windows::core::Interface for IWICEnumMetadataItem {
    type Vtable = IWICEnumMetadataItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc2bb46d_3f07_481e_8625_220c4aedbb33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICEnumMetadataItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienummetadataitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICFastMetadataEncoder(::windows::core::IUnknown);
impl IWICFastMetadataEncoder {
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryWriter>(result__)
    }
}
impl ::core::convert::From<IWICFastMetadataEncoder> for ::windows::core::IUnknown {
    fn from(value: IWICFastMetadataEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICFastMetadataEncoder> for ::windows::core::IUnknown {
    fn from(value: &IWICFastMetadataEncoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICFastMetadataEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICFastMetadataEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICFastMetadataEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICFastMetadataEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICFastMetadataEncoder {}
unsafe impl ::windows::core::Interface for IWICFastMetadataEncoder {
    type Vtable = IWICFastMetadataEncoderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb84e2c09_78c9_4ac4_8bd3_524ae1663a2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFastMetadataEncoderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICFormatConverter(::windows::core::IUnknown);
impl IWICFormatConverter {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>, Param3: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pisource: Param0, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: Param3, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pisource.into_param().abi(), ::core::mem::transmute(dstformat), ::core::mem::transmute(dither), pipalette.into_param().abi(), ::core::mem::transmute(alphathresholdpercent), ::core::mem::transmute(palettetranslate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanConvert(&self, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(srcpixelformat), ::core::mem::transmute(dstpixelformat), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWICFormatConverter> for IWICBitmapSource {
    fn from(value: IWICFormatConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICFormatConverter> for IWICBitmapSource {
    fn from(value: &IWICFormatConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICFormatConverter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICFormatConverter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICFormatConverter> for ::windows::core::IUnknown {
    fn from(value: IWICFormatConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICFormatConverter> for ::windows::core::IUnknown {
    fn from(value: &IWICFormatConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICFormatConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICFormatConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICFormatConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICFormatConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICFormatConverter {}
unsafe impl ::windows::core::Interface for IWICFormatConverter {
    type Vtable = IWICFormatConverterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000301_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFormatConverterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWICFormatConverterInfo(::windows::core::IUnknown);
impl IWICFormatConverterInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetPixelFormats(&self, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(cformats), ::core::mem::transmute(ppixelformatguids), ::core::mem::transmute(pcactual)).ok()
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICFormatConverter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICFormatConverter>(result__)
    }
}
impl ::core::convert::From<IWICFormatConverterInfo> for IWICComponentInfo {
    fn from(value: IWICFormatConverterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICFormatConverterInfo> for IWICComponentInfo {
    fn from(value: &IWICFormatConverterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICFormatConverterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICFormatConverterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICFormatConverterInfo> for ::windows::core::IUnknown {
    fn from(value: IWICFormatConverterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICFormatConverterInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICFormatConverterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICFormatConverterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICFormatConverterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICFormatConverterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICFormatConverterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICFormatConverterInfo {}
unsafe impl ::windows::core::Interface for IWICFormatConverterInfo {
    type Vtable = IWICFormatConverterInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f34fb65_13f4_4f15_bc57_3726b5e53d9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFormatConverterInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICImagingFactory(::windows::core::IUnknown);
impl IWICImagingFactory {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDecoderFromFilename<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzfilename: Param0, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wzfilename.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hfile), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<IWICComponentInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsidcomponent), ::core::mem::transmute(&mut result__)).from_abi::<IWICComponentInfo>(result__)
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapEncoder>(result__)
    }
    pub unsafe fn CreatePalette(&self) -> ::windows::core::Result<IWICPalette> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICPalette>(result__)
    }
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::core::Result<IWICFormatConverter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICFormatConverter>(result__)
    }
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::core::Result<IWICBitmapScaler> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapScaler>(result__)
    }
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::core::Result<IWICBitmapClipper> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapClipper>(result__)
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<IWICBitmapFlipRotator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapFlipRotator>(result__)
    }
    pub unsafe fn CreateStream(&self) -> ::windows::core::Result<IWICStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICStream>(result__)
    }
    pub unsafe fn CreateColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICColorContext>(result__)
    }
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::core::Result<IWICColorTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICColorTransform>(result__)
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(option), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromSource<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pibitmapsource: Param0, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(option), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromSourceRect<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>>(&self, pibitmapsource: Param0, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HBITMAP>, Param1: ::windows::core::IntoParam<'a, super::Gdi::HPALETTE>>(&self, hbitmap: Param0, hpalette: Param1, options: WICBitmapAlphaChannelOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), hbitmap.into_param().abi(), hpalette.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::WindowsAndMessaging::HICON>>(&self, hicon: Param0) -> ::windows::core::Result<IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), hicon.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(componenttypes), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumUnknown>(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapDecoder>>(&self, pidecoder: Param0) -> ::windows::core::Result<IWICFastMetadataEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pidecoder.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICFastMetadataEncoder>(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapFrameDecode>>(&self, piframedecoder: Param0) -> ::windows::core::Result<IWICFastMetadataEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), piframedecoder.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICFastMetadataEncoder>(result__)
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryWriter>(result__)
    }
    pub unsafe fn CreateQueryWriterFromReader<'a, Param0: ::windows::core::IntoParam<'a, IWICMetadataQueryReader>>(&self, piqueryreader: Param0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), piqueryreader.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataQueryWriter>(result__)
    }
}
impl ::core::convert::From<IWICImagingFactory> for ::windows::core::IUnknown {
    fn from(value: IWICImagingFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImagingFactory> for ::windows::core::IUnknown {
    fn from(value: &IWICImagingFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICImagingFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICImagingFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICImagingFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICImagingFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICImagingFactory {}
unsafe impl ::windows::core::Interface for IWICImagingFactory {
    type Vtable = IWICImagingFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec5ec8a9_c395_4314_9c77_54d7a935ff70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidcomponent: *const ::windows::core::GUID, ppiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppipalette: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwicstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piframedecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piqueryreader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICJpegFrameDecode(::windows::core::IUnknown);
impl IWICJpegFrameDecode {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportIndexing(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetIndexing(&self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(horizontalintervalsize)).ok()
    }
    pub unsafe fn ClearIndexing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE> {
        let mut result__: super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scanindex), ::core::mem::transmute(tableindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE> {
        let mut result__: super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(scanindex), ::core::mem::transmute(tableindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE> {
        let mut result__: super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(scanindex), ::core::mem::transmute(tableindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE>(result__)
    }
    pub unsafe fn GetFrameHeader(&self) -> ::windows::core::Result<WICJpegFrameHeader> {
        let mut result__: WICJpegFrameHeader = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICJpegFrameHeader>(result__)
    }
    pub unsafe fn GetScanHeader(&self, scanindex: u32) -> ::windows::core::Result<WICJpegScanHeader> {
        let mut result__: WICJpegScanHeader = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(scanindex), ::core::mem::transmute(&mut result__)).from_abi::<WICJpegScanHeader>(result__)
    }
    pub unsafe fn CopyScan(&self, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(scanindex), ::core::mem::transmute(scanoffset), ::core::mem::transmute(cbscandata), ::core::mem::transmute(pbscandata), ::core::mem::transmute(pcbscandataactual)).ok()
    }
    pub unsafe fn CopyMinimalStream(&self, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamoffset), ::core::mem::transmute(cbstreamdata), ::core::mem::transmute(pbstreamdata), ::core::mem::transmute(pcbstreamdataactual)).ok()
    }
}
impl ::core::convert::From<IWICJpegFrameDecode> for ::windows::core::IUnknown {
    fn from(value: IWICJpegFrameDecode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICJpegFrameDecode> for ::windows::core::IUnknown {
    fn from(value: &IWICJpegFrameDecode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICJpegFrameDecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICJpegFrameDecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICJpegFrameDecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICJpegFrameDecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICJpegFrameDecode {}
unsafe impl ::windows::core::Interface for IWICJpegFrameDecode {
    type Vtable = IWICJpegFrameDecodeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8939f66e_c46a_4c21_a9d1_98b327ce1679);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICJpegFrameDecodeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICJpegFrameEncode(::windows::core::IUnknown);
impl IWICJpegFrameEncode {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE> {
        let mut result__: super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(scanindex), ::core::mem::transmute(tableindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE> {
        let mut result__: super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(scanindex), ::core::mem::transmute(tableindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE> {
        let mut result__: super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(scanindex), ::core::mem::transmute(tableindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE>(result__)
    }
    pub unsafe fn WriteScan(&self, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbscandata), ::core::mem::transmute(pbscandata)).ok()
    }
}
impl ::core::convert::From<IWICJpegFrameEncode> for ::windows::core::IUnknown {
    fn from(value: IWICJpegFrameEncode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICJpegFrameEncode> for ::windows::core::IUnknown {
    fn from(value: &IWICJpegFrameEncode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICJpegFrameEncode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICJpegFrameEncode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICJpegFrameEncode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICJpegFrameEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICJpegFrameEncode {}
unsafe impl ::windows::core::Interface for IWICJpegFrameEncode {
    type Vtable = IWICJpegFrameEncodeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f0c601f_d2c6_468c_abfa_49495d983ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICJpegFrameEncodeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICMetadataBlockReader(::windows::core::IUnknown);
impl IWICMetadataBlockReader {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetReaderByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumUnknown>(result__)
    }
}
impl ::core::convert::From<IWICMetadataBlockReader> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataBlockReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataBlockReader> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataBlockReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataBlockReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataBlockReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataBlockReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataBlockReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataBlockReader {}
unsafe impl ::windows::core::Interface for IWICMetadataBlockReader {
    type Vtable = IWICMetadataBlockReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeaa2a8d_b3f3_43e4_b25c_d1de990a1ae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataBlockReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IWICMetadataBlockWriter(::windows::core::IUnknown);
impl IWICMetadataBlockWriter {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetReaderByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumUnknown>(result__)
    }
    pub unsafe fn InitializeFromBlockReader<'a, Param0: ::windows::core::IntoParam<'a, IWICMetadataBlockReader>>(&self, pimdblockreader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pimdblockreader.into_param().abi()).ok()
    }
    pub unsafe fn GetWriterByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataWriter>(result__)
    }
    pub unsafe fn AddWriter<'a, Param0: ::windows::core::IntoParam<'a, IWICMetadataWriter>>(&self, pimetadatawriter: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pimetadatawriter.into_param().abi()).ok()
    }
    pub unsafe fn SetWriterByIndex<'a, Param1: ::windows::core::IntoParam<'a, IWICMetadataWriter>>(&self, nindex: u32, pimetadatawriter: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), pimetadatawriter.into_param().abi()).ok()
    }
    pub unsafe fn RemoveWriterByIndex(&self, nindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)).ok()
    }
}
impl ::core::convert::From<IWICMetadataBlockWriter> for IWICMetadataBlockReader {
    fn from(value: IWICMetadataBlockWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataBlockWriter> for IWICMetadataBlockReader {
    fn from(value: &IWICMetadataBlockWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataBlockReader> for IWICMetadataBlockWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataBlockReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataBlockReader> for &IWICMetadataBlockWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataBlockReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICMetadataBlockWriter> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataBlockWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataBlockWriter> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataBlockWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataBlockWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataBlockWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataBlockWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataBlockWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataBlockWriter {}
unsafe impl ::windows::core::Interface for IWICMetadataBlockWriter {
    type Vtable = IWICMetadataBlockWriterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08fb9676_b444_41e8_8dbe_6a53a542bff1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataBlockWriterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimdblockreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICMetadataHandlerInfo(::windows::core::IUnknown);
impl IWICMetadataHandlerInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetContainerFormats(&self, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccontainerformats), ::core::mem::transmute(pguidcontainerformats), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceManufacturer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemanufacturer), wzdevicemanufacturer.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceModels<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemodels: u32, wzdevicemodels: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemodels), wzdevicemodels.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWICMetadataHandlerInfo> for IWICComponentInfo {
    fn from(value: IWICMetadataHandlerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataHandlerInfo> for IWICComponentInfo {
    fn from(value: &IWICMetadataHandlerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICMetadataHandlerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICMetadataHandlerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICMetadataHandlerInfo> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataHandlerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataHandlerInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataHandlerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataHandlerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataHandlerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataHandlerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataHandlerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataHandlerInfo {}
unsafe impl ::windows::core::Interface for IWICMetadataHandlerInfo {
    type Vtable = IWICMetadataHandlerInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaba958bf_c672_44d1_8d61_ce6df2e682c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataHandlerInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWICMetadataQueryReader(::windows::core::IUnknown);
impl IWICMetadataQueryReader {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocation<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchmaxlength: u32, wznamespace: Param1, pcchactuallength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchmaxlength), wznamespace.into_param().abi(), ::core::mem::transmute(pcchactuallength)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetMetadataByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzname: Param0, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wzname.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumString>(result__)
    }
}
impl ::core::convert::From<IWICMetadataQueryReader> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataQueryReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataQueryReader> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataQueryReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataQueryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataQueryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataQueryReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataQueryReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataQueryReader {}
unsafe impl ::windows::core::Interface for IWICMetadataQueryReader {
    type Vtable = IWICMetadataQueryReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30989668_e1c9_4597_b395_458eedb808df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataQueryReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: super::super::Foundation::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IWICMetadataQueryWriter(::windows::core::IUnknown);
impl IWICMetadataQueryWriter {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocation<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchmaxlength: u32, wznamespace: Param1, pcchactuallength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchmaxlength), wznamespace.into_param().abi(), ::core::mem::transmute(pcchactuallength)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetMetadataByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzname: Param0, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wzname.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumString>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetMetadataByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzname: Param0, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), wzname.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveMetadataByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), wzname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWICMetadataQueryWriter> for IWICMetadataQueryReader {
    fn from(value: IWICMetadataQueryWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataQueryWriter> for IWICMetadataQueryReader {
    fn from(value: &IWICMetadataQueryWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataQueryReader> for IWICMetadataQueryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataQueryReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataQueryReader> for &IWICMetadataQueryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataQueryReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICMetadataQueryWriter> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataQueryWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataQueryWriter> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataQueryWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataQueryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataQueryWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataQueryWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataQueryWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataQueryWriter {}
unsafe impl ::windows::core::Interface for IWICMetadataQueryWriter {
    type Vtable = IWICMetadataQueryWriterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa721791a_0def_4d06_bd91_2118bf1db10b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataQueryWriterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: super::super::Foundation::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWICMetadataReader(::windows::core::IUnknown);
impl IWICMetadataReader {
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetMetadataHandlerInfo(&self) -> ::windows::core::Result<IWICMetadataHandlerInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataHandlerInfo>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(pvarschema), ::core::mem::transmute(pvarid), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarschema), ::core::mem::transmute(pvarid), ::core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IWICEnumMetadataItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICEnumMetadataItem>(result__)
    }
}
impl ::core::convert::From<IWICMetadataReader> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataReader> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataReader {}
unsafe impl ::windows::core::Interface for IWICMetadataReader {
    type Vtable = IWICMetadataReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9204fe99_d8fc_4fd5_a001_9536b067a899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppihandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICMetadataReaderInfo(::windows::core::IUnknown);
impl IWICMetadataReaderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetContainerFormats(&self, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccontainerformats), ::core::mem::transmute(pguidcontainerformats), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceManufacturer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemanufacturer), wzdevicemanufacturer.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceModels<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemodels: u32, wzdevicemodels: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemodels), wzdevicemodels.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPatterns(&self, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(cbsize), ::core::mem::transmute(ppattern), ::core::mem::transmute(pccount), ::core::mem::transmute(pcbactual)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn MatchesPattern<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, guidcontainerformat: *const ::windows::core::GUID, pistream: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), pistream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataReader>(result__)
    }
}
impl ::core::convert::From<IWICMetadataReaderInfo> for IWICMetadataHandlerInfo {
    fn from(value: IWICMetadataReaderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataReaderInfo> for IWICMetadataHandlerInfo {
    fn from(value: &IWICMetadataReaderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataHandlerInfo> for IWICMetadataReaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataHandlerInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataHandlerInfo> for &IWICMetadataReaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataHandlerInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICMetadataReaderInfo> for IWICComponentInfo {
    fn from(value: IWICMetadataReaderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataReaderInfo> for IWICComponentInfo {
    fn from(value: &IWICMetadataReaderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICMetadataReaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICMetadataReaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICMetadataReaderInfo> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataReaderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataReaderInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataReaderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataReaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataReaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataReaderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataReaderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataReaderInfo {}
unsafe impl ::windows::core::Interface for IWICMetadataReaderInfo {
    type Vtable = IWICMetadataReaderInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeebf1f5b_07c1_4447_a3ab_22acaf78a804);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataReaderInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICMetadataWriter(::windows::core::IUnknown);
impl IWICMetadataWriter {
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetMetadataHandlerInfo(&self) -> ::windows::core::Result<IWICMetadataHandlerInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataHandlerInfo>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(pvarschema), ::core::mem::transmute(pvarid), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarschema), ::core::mem::transmute(pvarid), ::core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IWICEnumMetadataItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICEnumMetadataItem>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarschema), ::core::mem::transmute(pvarid), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValueByIndex(&self, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(pvarschema), ::core::mem::transmute(pvarid), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn RemoveValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarschema), ::core::mem::transmute(pvarid)).ok()
    }
    pub unsafe fn RemoveValueByIndex(&self, nindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)).ok()
    }
}
impl ::core::convert::From<IWICMetadataWriter> for IWICMetadataReader {
    fn from(value: IWICMetadataWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataWriter> for IWICMetadataReader {
    fn from(value: &IWICMetadataWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataReader> for IWICMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataReader> for &IWICMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataWriter {}
unsafe impl ::windows::core::Interface for IWICMetadataWriter {
    type Vtable = IWICMetadataWriterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7836e16_3be0_470b_86bb_160d0aecd7de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataWriterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppihandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICMetadataWriterInfo(::windows::core::IUnknown);
impl IWICMetadataWriterInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetContainerFormats(&self, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccontainerformats), ::core::mem::transmute(pguidcontainerformats), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceManufacturer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemanufacturer), wzdevicemanufacturer.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceModels<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchdevicemodels: u32, wzdevicemodels: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchdevicemodels), wzdevicemodels.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetHeader(&self, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(cbsize), ::core::mem::transmute(pheader), ::core::mem::transmute(pcbactual)).ok()
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICMetadataWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICMetadataWriter>(result__)
    }
}
impl ::core::convert::From<IWICMetadataWriterInfo> for IWICMetadataHandlerInfo {
    fn from(value: IWICMetadataWriterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataWriterInfo> for IWICMetadataHandlerInfo {
    fn from(value: &IWICMetadataWriterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataHandlerInfo> for IWICMetadataWriterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataHandlerInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICMetadataHandlerInfo> for &IWICMetadataWriterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICMetadataHandlerInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICMetadataWriterInfo> for IWICComponentInfo {
    fn from(value: IWICMetadataWriterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataWriterInfo> for IWICComponentInfo {
    fn from(value: &IWICMetadataWriterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICMetadataWriterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICMetadataWriterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICMetadataWriterInfo> for ::windows::core::IUnknown {
    fn from(value: IWICMetadataWriterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICMetadataWriterInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICMetadataWriterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICMetadataWriterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICMetadataWriterInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICMetadataWriterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICMetadataWriterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataWriterInfo {}
unsafe impl ::windows::core::Interface for IWICMetadataWriterInfo {
    type Vtable = IWICMetadataWriterInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22e3fba_3925_4323_b5c1_9ebfc430f236);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataWriterInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICPalette(::windows::core::IUnknown);
impl IWICPalette {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializePredefined<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(epalettetype), faddtransparentcolor.into_param().abi()).ok()
    }
    pub unsafe fn InitializeCustom(&self, pcolors: *const u32, ccount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolors), ::core::mem::transmute(ccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromBitmap<'a, Param0: ::windows::core::IntoParam<'a, IWICBitmapSource>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pisurface: Param0, ccount: u32, faddtransparentcolor: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pisurface.into_param().abi(), ::core::mem::transmute(ccount), faddtransparentcolor.into_param().abi()).ok()
    }
    pub unsafe fn InitializeFromPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<WICBitmapPaletteType> {
        let mut result__: WICBitmapPaletteType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICBitmapPaletteType>(result__)
    }
    pub unsafe fn GetColorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetColors(&self, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccount), ::core::mem::transmute(pcolors), ::core::mem::transmute(pcactualcolors)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBlackWhite(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGrayscale(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasAlpha(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWICPalette> for ::windows::core::IUnknown {
    fn from(value: IWICPalette) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPalette> for ::windows::core::IUnknown {
    fn from(value: &IWICPalette) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICPalette {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICPalette {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICPalette {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICPalette {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPalette {}
unsafe impl ::windows::core::Interface for IWICPalette {
    type Vtable = IWICPaletteVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000040_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPaletteVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolors: *const u32, ccount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisurface: ::windows::core::RawPtr, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWICPersistStream(::windows::core::IUnknown);
impl IWICPersistStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(pguidpreferredvendor), ::core::mem::transmute(dwpersistoptions)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SaveEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pistream: Param0, dwpersistoptions: u32, fcleardirty: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(dwpersistoptions), fcleardirty.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWICPersistStream> for super::super::System::Com::IPersistStream {
    fn from(value: IWICPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWICPersistStream> for super::super::System::Com::IPersistStream {
    fn from(value: &IWICPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersistStream> for IWICPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersistStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersistStream> for &IWICPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersistStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWICPersistStream> for super::super::System::Com::IPersist {
    fn from(value: IWICPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWICPersistStream> for super::super::System::Com::IPersist {
    fn from(value: &IWICPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersist> for IWICPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersist> for &IWICPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICPersistStream> for ::windows::core::IUnknown {
    fn from(value: IWICPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPersistStream> for ::windows::core::IUnknown {
    fn from(value: &IWICPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICPersistStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICPersistStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPersistStream {}
unsafe impl ::windows::core::Interface for IWICPersistStream {
    type Vtable = IWICPersistStreamVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00675040_6908_45f8_86a3_49c7dfd6d9ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPersistStreamVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IWICPixelFormatInfo(::windows::core::IUnknown);
impl IWICPixelFormatInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetFormatGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICColorContext>(result__)
    }
    pub unsafe fn GetBitsPerPixel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChannelMask(&self, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(uichannelindex), ::core::mem::transmute(cbmaskbuffer), ::core::mem::transmute(pbmaskbuffer), ::core::mem::transmute(pcbactual)).ok()
    }
}
impl ::core::convert::From<IWICPixelFormatInfo> for IWICComponentInfo {
    fn from(value: IWICPixelFormatInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPixelFormatInfo> for IWICComponentInfo {
    fn from(value: &IWICPixelFormatInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICPixelFormatInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICPixelFormatInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICPixelFormatInfo> for ::windows::core::IUnknown {
    fn from(value: IWICPixelFormatInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPixelFormatInfo> for ::windows::core::IUnknown {
    fn from(value: &IWICPixelFormatInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICPixelFormatInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICPixelFormatInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICPixelFormatInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICPixelFormatInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPixelFormatInfo {}
unsafe impl ::windows::core::Interface for IWICPixelFormatInfo {
    type Vtable = IWICPixelFormatInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8eda601_3d48_431a_ab44_69059be88bbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPixelFormatInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICPixelFormatInfo2(::windows::core::IUnknown);
impl IWICPixelFormatInfo2 {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__: WICComponentType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICComponentType>(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchauthor: u32, wzauthor: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchauthor), wzauthor.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchversion: u32, wzversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchversion), wzversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchspecversion: u32, wzspecversion: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchspecversion), wzspecversion.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchfriendlyname: u32, wzfriendlyname: Param1, pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchfriendlyname), wzfriendlyname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    pub unsafe fn GetFormatGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWICColorContext>(result__)
    }
    pub unsafe fn GetBitsPerPixel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChannelMask(&self, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(uichannelindex), ::core::mem::transmute(cbmaskbuffer), ::core::mem::transmute(pbmaskbuffer), ::core::mem::transmute(pcbactual)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsTransparency(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetNumericRepresentation(&self) -> ::windows::core::Result<WICPixelFormatNumericRepresentation> {
        let mut result__: WICPixelFormatNumericRepresentation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WICPixelFormatNumericRepresentation>(result__)
    }
}
impl ::core::convert::From<IWICPixelFormatInfo2> for IWICPixelFormatInfo {
    fn from(value: IWICPixelFormatInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPixelFormatInfo2> for IWICPixelFormatInfo {
    fn from(value: &IWICPixelFormatInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICPixelFormatInfo> for IWICPixelFormatInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWICPixelFormatInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICPixelFormatInfo> for &IWICPixelFormatInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWICPixelFormatInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICPixelFormatInfo2> for IWICComponentInfo {
    fn from(value: IWICPixelFormatInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPixelFormatInfo2> for IWICComponentInfo {
    fn from(value: &IWICPixelFormatInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for IWICPixelFormatInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICComponentInfo> for &IWICPixelFormatInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWICComponentInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICPixelFormatInfo2> for ::windows::core::IUnknown {
    fn from(value: IWICPixelFormatInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPixelFormatInfo2> for ::windows::core::IUnknown {
    fn from(value: &IWICPixelFormatInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICPixelFormatInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICPixelFormatInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICPixelFormatInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICPixelFormatInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPixelFormatInfo2 {}
unsafe impl ::windows::core::Interface for IWICPixelFormatInfo2 {
    type Vtable = IWICPixelFormatInfo2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9db33a2_af5f_43c7_b679_74f5984b5aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPixelFormatInfo2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICPlanarBitmapFrameEncode(::windows::core::IUnknown);
impl IWICPlanarBitmapFrameEncode {
    pub unsafe fn WritePixels(&self, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(linecount), ::core::mem::transmute(pplanes), ::core::mem::transmute(cplanes)).ok()
    }
    pub unsafe fn WriteSource(&self, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppplanes), ::core::mem::transmute(cplanes), ::core::mem::transmute(prcsource)).ok()
    }
}
impl ::core::convert::From<IWICPlanarBitmapFrameEncode> for ::windows::core::IUnknown {
    fn from(value: IWICPlanarBitmapFrameEncode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPlanarBitmapFrameEncode> for ::windows::core::IUnknown {
    fn from(value: &IWICPlanarBitmapFrameEncode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICPlanarBitmapFrameEncode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICPlanarBitmapFrameEncode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICPlanarBitmapFrameEncode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICPlanarBitmapFrameEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPlanarBitmapFrameEncode {}
unsafe impl ::windows::core::Interface for IWICPlanarBitmapFrameEncode {
    type Vtable = IWICPlanarBitmapFrameEncodeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf928b7b8_2221_40c1_b72e_7e82f1974d1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarBitmapFrameEncodeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICPlanarBitmapSourceTransform(::windows::core::IUnknown);
impl IWICPlanarBitmapSourceTransform {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportTransform(&self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight), ::core::mem::transmute(dsttransform), ::core::mem::transmute(dstplanaroptions), ::core::mem::transmute(pguiddstformats), ::core::mem::transmute(pplanedescriptions), ::core::mem::transmute(cplanes), ::core::mem::transmute(pfissupported)).ok()
    }
    pub unsafe fn CopyPixels(&self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcsource), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(dsttransform), ::core::mem::transmute(dstplanaroptions), ::core::mem::transmute(pdstplanes), ::core::mem::transmute(cplanes)).ok()
    }
}
impl ::core::convert::From<IWICPlanarBitmapSourceTransform> for ::windows::core::IUnknown {
    fn from(value: IWICPlanarBitmapSourceTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPlanarBitmapSourceTransform> for ::windows::core::IUnknown {
    fn from(value: &IWICPlanarBitmapSourceTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICPlanarBitmapSourceTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICPlanarBitmapSourceTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICPlanarBitmapSourceTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICPlanarBitmapSourceTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPlanarBitmapSourceTransform {}
unsafe impl ::windows::core::Interface for IWICPlanarBitmapSourceTransform {
    type Vtable = IWICPlanarBitmapSourceTransformVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3aff9cce_be95_4303_b927_e7d16ff4a613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarBitmapSourceTransformVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICPlanarFormatConverter(::windows::core::IUnknown);
impl IWICPlanarFormatConverter {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puiwidth), ::core::mem::transmute(puiheight)).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdpix), ::core::mem::transmute(pdpiy)).ok()
    }
    pub unsafe fn CopyPalette<'a, Param0: ::windows::core::IntoParam<'a, IWICPalette>>(&self, pipalette: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer)).ok()
    }
    pub unsafe fn Initialize<'a, Param4: ::windows::core::IntoParam<'a, IWICPalette>>(&self, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: Param4, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppplanes), ::core::mem::transmute(cplanes), ::core::mem::transmute(dstformat), ::core::mem::transmute(dither), pipalette.into_param().abi(), ::core::mem::transmute(alphathresholdpercent), ::core::mem::transmute(palettetranslate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanConvert(&self, psrcpixelformats: *const ::windows::core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(psrcpixelformats), ::core::mem::transmute(csrcplanes), ::core::mem::transmute(dstpixelformat), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWICPlanarFormatConverter> for IWICBitmapSource {
    fn from(value: IWICPlanarFormatConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPlanarFormatConverter> for IWICBitmapSource {
    fn from(value: &IWICPlanarFormatConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for IWICPlanarFormatConverter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWICBitmapSource> for &IWICPlanarFormatConverter {
    fn into_param(self) -> ::windows::core::Param<'a, IWICBitmapSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICPlanarFormatConverter> for ::windows::core::IUnknown {
    fn from(value: IWICPlanarFormatConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICPlanarFormatConverter> for ::windows::core::IUnknown {
    fn from(value: &IWICPlanarFormatConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICPlanarFormatConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICPlanarFormatConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICPlanarFormatConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICPlanarFormatConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPlanarFormatConverter {}
unsafe impl ::windows::core::Interface for IWICPlanarFormatConverter {
    type Vtable = IWICPlanarFormatConverterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbebee9cb_83b0_4dcc_8132_b0aaa55eac96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarFormatConverterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrcpixelformats: *const ::windows::core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWICProgressCallback(::windows::core::IUnknown);
impl IWICProgressCallback {
    pub unsafe fn Notify(&self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uframenum), ::core::mem::transmute(operation), ::core::mem::transmute(dblprogress)).ok()
    }
}
impl ::core::convert::From<IWICProgressCallback> for ::windows::core::IUnknown {
    fn from(value: IWICProgressCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICProgressCallback> for ::windows::core::IUnknown {
    fn from(value: &IWICProgressCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICProgressCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICProgressCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICProgressCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICProgressCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICProgressCallback {}
unsafe impl ::windows::core::Interface for IWICProgressCallback {
    type Vtable = IWICProgressCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4776f9cd_9517_45fa_bf24_e89c5ec5c60c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICProgressCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IWICProgressiveLevelControl(::windows::core::IUnknown);
impl IWICProgressiveLevelControl {
    pub unsafe fn GetLevelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCurrentLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetCurrentLevel(&self, nlevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nlevel)).ok()
    }
}
impl ::core::convert::From<IWICProgressiveLevelControl> for ::windows::core::IUnknown {
    fn from(value: IWICProgressiveLevelControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICProgressiveLevelControl> for ::windows::core::IUnknown {
    fn from(value: &IWICProgressiveLevelControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICProgressiveLevelControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICProgressiveLevelControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICProgressiveLevelControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICProgressiveLevelControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICProgressiveLevelControl {}
unsafe impl ::windows::core::Interface for IWICProgressiveLevelControl {
    type Vtable = IWICProgressiveLevelControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaac296f_7aa5_4dbf_8d15_225c5976f891);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICProgressiveLevelControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclevels: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlevel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nlevel: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWICStream(::windows::core::IUnknown);
impl IWICStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromIStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pistream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromFilename<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wzfilename: Param0, dwdesiredaccess: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), wzfilename.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)).ok()
    }
    pub unsafe fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(cbbuffersize)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromIStreamRegion<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0, uloffset: u64, ulmaxsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(uloffset), ::core::mem::transmute(ulmaxsize)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWICStream> for super::super::System::Com::IStream {
    fn from(value: IWICStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWICStream> for super::super::System::Com::IStream {
    fn from(value: &IWICStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IStream> for IWICStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IStream> for &IWICStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWICStream> for super::super::System::Com::ISequentialStream {
    fn from(value: IWICStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWICStream> for super::super::System::Com::ISequentialStream {
    fn from(value: &IWICStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream> for IWICStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream> for &IWICStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICStream> for ::windows::core::IUnknown {
    fn from(value: IWICStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICStream> for ::windows::core::IUnknown {
    fn from(value: &IWICStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICStream {}
unsafe impl ::windows::core::Interface for IWICStream {
    type Vtable = IWICStreamVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x135ff860_22b7_4ddf_b0f6_218f4f299a43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICStreamVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, uloffset: u64, ulmaxsize: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IWICStreamProvider(::windows::core::IUnknown);
impl IWICStreamProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn GetPersistOptions(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPreferredVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn RefreshStream(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWICStreamProvider> for ::windows::core::IUnknown {
    fn from(value: IWICStreamProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICStreamProvider> for ::windows::core::IUnknown {
    fn from(value: &IWICStreamProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICStreamProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWICStreamProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICStreamProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICStreamProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICStreamProvider {}
unsafe impl ::windows::core::Interface for IWICStreamProvider {
    type Vtable = IWICStreamProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x449494bc_b468_4927_96d7_ba90d31ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICStreamProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpersistoptions: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidpreferredvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
pub type PFNProgressNotification = ::core::option::Option<unsafe extern "system" fn(pvdata: *const ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::HRESULT>;
pub type WIC8BIMIptcDigestProperties = u32;
pub const WIC8BIMIptcDigestPString: WIC8BIMIptcDigestProperties = 1u32;
pub const WIC8BIMIptcDigestIptcDigest: WIC8BIMIptcDigestProperties = 2u32;
pub const WIC8BIMIptcDigestProperties_FORCE_DWORD: WIC8BIMIptcDigestProperties = 2147483647u32;
pub type WIC8BIMIptcProperties = u32;
pub const WIC8BIMIptcPString: WIC8BIMIptcProperties = 0u32;
pub const WIC8BIMIptcEmbeddedIPTC: WIC8BIMIptcProperties = 1u32;
pub const WIC8BIMIptcProperties_FORCE_DWORD: WIC8BIMIptcProperties = 2147483647u32;
pub type WIC8BIMResolutionInfoProperties = u32;
pub const WIC8BIMResolutionInfoPString: WIC8BIMResolutionInfoProperties = 1u32;
pub const WIC8BIMResolutionInfoHResolution: WIC8BIMResolutionInfoProperties = 2u32;
pub const WIC8BIMResolutionInfoHResolutionUnit: WIC8BIMResolutionInfoProperties = 3u32;
pub const WIC8BIMResolutionInfoWidthUnit: WIC8BIMResolutionInfoProperties = 4u32;
pub const WIC8BIMResolutionInfoVResolution: WIC8BIMResolutionInfoProperties = 5u32;
pub const WIC8BIMResolutionInfoVResolutionUnit: WIC8BIMResolutionInfoProperties = 6u32;
pub const WIC8BIMResolutionInfoHeightUnit: WIC8BIMResolutionInfoProperties = 7u32;
pub const WIC8BIMResolutionInfoProperties_FORCE_DWORD: WIC8BIMResolutionInfoProperties = 2147483647u32;
pub type WICBitmapAlphaChannelOption = i32;
pub const WICBitmapUseAlpha: WICBitmapAlphaChannelOption = 0i32;
pub const WICBitmapUsePremultipliedAlpha: WICBitmapAlphaChannelOption = 1i32;
pub const WICBitmapIgnoreAlpha: WICBitmapAlphaChannelOption = 2i32;
pub const WICBITMAPALPHACHANNELOPTIONS_FORCE_DWORD: WICBitmapAlphaChannelOption = 2147483647i32;
pub type WICBitmapCreateCacheOption = i32;
pub const WICBitmapNoCache: WICBitmapCreateCacheOption = 0i32;
pub const WICBitmapCacheOnDemand: WICBitmapCreateCacheOption = 1i32;
pub const WICBitmapCacheOnLoad: WICBitmapCreateCacheOption = 2i32;
pub const WICBITMAPCREATECACHEOPTION_FORCE_DWORD: WICBitmapCreateCacheOption = 2147483647i32;
pub type WICBitmapDecoderCapabilities = i32;
pub const WICBitmapDecoderCapabilitySameEncoder: WICBitmapDecoderCapabilities = 1i32;
pub const WICBitmapDecoderCapabilityCanDecodeAllImages: WICBitmapDecoderCapabilities = 2i32;
pub const WICBitmapDecoderCapabilityCanDecodeSomeImages: WICBitmapDecoderCapabilities = 4i32;
pub const WICBitmapDecoderCapabilityCanEnumerateMetadata: WICBitmapDecoderCapabilities = 8i32;
pub const WICBitmapDecoderCapabilityCanDecodeThumbnail: WICBitmapDecoderCapabilities = 16i32;
pub const WICBITMAPDECODERCAPABILITIES_FORCE_DWORD: WICBitmapDecoderCapabilities = 2147483647i32;
pub type WICBitmapDitherType = i32;
pub const WICBitmapDitherTypeNone: WICBitmapDitherType = 0i32;
pub const WICBitmapDitherTypeSolid: WICBitmapDitherType = 0i32;
pub const WICBitmapDitherTypeOrdered4x4: WICBitmapDitherType = 1i32;
pub const WICBitmapDitherTypeOrdered8x8: WICBitmapDitherType = 2i32;
pub const WICBitmapDitherTypeOrdered16x16: WICBitmapDitherType = 3i32;
pub const WICBitmapDitherTypeSpiral4x4: WICBitmapDitherType = 4i32;
pub const WICBitmapDitherTypeSpiral8x8: WICBitmapDitherType = 5i32;
pub const WICBitmapDitherTypeDualSpiral4x4: WICBitmapDitherType = 6i32;
pub const WICBitmapDitherTypeDualSpiral8x8: WICBitmapDitherType = 7i32;
pub const WICBitmapDitherTypeErrorDiffusion: WICBitmapDitherType = 8i32;
pub const WICBITMAPDITHERTYPE_FORCE_DWORD: WICBitmapDitherType = 2147483647i32;
pub type WICBitmapEncoderCacheOption = i32;
pub const WICBitmapEncoderCacheInMemory: WICBitmapEncoderCacheOption = 0i32;
pub const WICBitmapEncoderCacheTempFile: WICBitmapEncoderCacheOption = 1i32;
pub const WICBitmapEncoderNoCache: WICBitmapEncoderCacheOption = 2i32;
pub const WICBITMAPENCODERCACHEOPTION_FORCE_DWORD: WICBitmapEncoderCacheOption = 2147483647i32;
pub type WICBitmapInterpolationMode = i32;
pub const WICBitmapInterpolationModeNearestNeighbor: WICBitmapInterpolationMode = 0i32;
pub const WICBitmapInterpolationModeLinear: WICBitmapInterpolationMode = 1i32;
pub const WICBitmapInterpolationModeCubic: WICBitmapInterpolationMode = 2i32;
pub const WICBitmapInterpolationModeFant: WICBitmapInterpolationMode = 3i32;
pub const WICBitmapInterpolationModeHighQualityCubic: WICBitmapInterpolationMode = 4i32;
pub const WICBITMAPINTERPOLATIONMODE_FORCE_DWORD: WICBitmapInterpolationMode = 2147483647i32;
pub type WICBitmapLockFlags = i32;
pub const WICBitmapLockRead: WICBitmapLockFlags = 1i32;
pub const WICBitmapLockWrite: WICBitmapLockFlags = 2i32;
pub const WICBITMAPLOCKFLAGS_FORCE_DWORD: WICBitmapLockFlags = 2147483647i32;
pub type WICBitmapPaletteType = i32;
pub const WICBitmapPaletteTypeCustom: WICBitmapPaletteType = 0i32;
pub const WICBitmapPaletteTypeMedianCut: WICBitmapPaletteType = 1i32;
pub const WICBitmapPaletteTypeFixedBW: WICBitmapPaletteType = 2i32;
pub const WICBitmapPaletteTypeFixedHalftone8: WICBitmapPaletteType = 3i32;
pub const WICBitmapPaletteTypeFixedHalftone27: WICBitmapPaletteType = 4i32;
pub const WICBitmapPaletteTypeFixedHalftone64: WICBitmapPaletteType = 5i32;
pub const WICBitmapPaletteTypeFixedHalftone125: WICBitmapPaletteType = 6i32;
pub const WICBitmapPaletteTypeFixedHalftone216: WICBitmapPaletteType = 7i32;
pub const WICBitmapPaletteTypeFixedWebPalette: WICBitmapPaletteType = 7i32;
pub const WICBitmapPaletteTypeFixedHalftone252: WICBitmapPaletteType = 8i32;
pub const WICBitmapPaletteTypeFixedHalftone256: WICBitmapPaletteType = 9i32;
pub const WICBitmapPaletteTypeFixedGray4: WICBitmapPaletteType = 10i32;
pub const WICBitmapPaletteTypeFixedGray16: WICBitmapPaletteType = 11i32;
pub const WICBitmapPaletteTypeFixedGray256: WICBitmapPaletteType = 12i32;
pub const WICBITMAPPALETTETYPE_FORCE_DWORD: WICBitmapPaletteType = 2147483647i32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WICBitmapPattern {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WICBitmapPattern {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICBitmapPattern>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WICBitmapPattern {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WICBitmapPattern {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WICBitmapPlane {
    pub Format: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for WICBitmapPlane {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICBitmapPlane {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICBitmapPlane>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICBitmapPlane {}
impl ::core::default::Default for WICBitmapPlane {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WICBitmapPlaneDescription {
    pub Format: ::windows::core::GUID,
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for WICBitmapPlaneDescription {}
impl ::core::clone::Clone for WICBitmapPlaneDescription {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WICBitmapPlaneDescription {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICBitmapPlaneDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICBitmapPlaneDescription>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICBitmapPlaneDescription {}
impl ::core::default::Default for WICBitmapPlaneDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WICBitmapTransformOptions = i32;
pub const WICBitmapTransformRotate0: WICBitmapTransformOptions = 0i32;
pub const WICBitmapTransformRotate90: WICBitmapTransformOptions = 1i32;
pub const WICBitmapTransformRotate180: WICBitmapTransformOptions = 2i32;
pub const WICBitmapTransformRotate270: WICBitmapTransformOptions = 3i32;
pub const WICBitmapTransformFlipHorizontal: WICBitmapTransformOptions = 8i32;
pub const WICBitmapTransformFlipVertical: WICBitmapTransformOptions = 16i32;
pub const WICBITMAPTRANSFORMOPTIONS_FORCE_DWORD: WICBitmapTransformOptions = 2147483647i32;
pub type WICColorContextType = i32;
pub const WICColorContextUninitialized: WICColorContextType = 0i32;
pub const WICColorContextProfile: WICColorContextType = 1i32;
pub const WICColorContextExifColorSpace: WICColorContextType = 2i32;
pub type WICComponentEnumerateOptions = i32;
pub const WICComponentEnumerateDefault: WICComponentEnumerateOptions = 0i32;
pub const WICComponentEnumerateRefresh: WICComponentEnumerateOptions = 1i32;
pub const WICComponentEnumerateDisabled: WICComponentEnumerateOptions = -2147483648i32;
pub const WICComponentEnumerateUnsigned: WICComponentEnumerateOptions = 1073741824i32;
pub const WICComponentEnumerateBuiltInOnly: WICComponentEnumerateOptions = 536870912i32;
pub const WICCOMPONENTENUMERATEOPTIONS_FORCE_DWORD: WICComponentEnumerateOptions = 2147483647i32;
pub type WICComponentSigning = i32;
pub const WICComponentSigned: WICComponentSigning = 1i32;
pub const WICComponentUnsigned: WICComponentSigning = 2i32;
pub const WICComponentSafe: WICComponentSigning = 4i32;
pub const WICComponentDisabled: WICComponentSigning = -2147483648i32;
pub const WICCOMPONENTSIGNING_FORCE_DWORD: WICComponentSigning = 2147483647i32;
pub type WICComponentType = i32;
pub const WICDecoder: WICComponentType = 1i32;
pub const WICEncoder: WICComponentType = 2i32;
pub const WICPixelFormatConverter: WICComponentType = 4i32;
pub const WICMetadataReader: WICComponentType = 8i32;
pub const WICMetadataWriter: WICComponentType = 16i32;
pub const WICPixelFormat: WICComponentType = 32i32;
pub const WICAllComponents: WICComponentType = 63i32;
pub const WICCOMPONENTTYPE_FORCE_DWORD: WICComponentType = 2147483647i32;
#[inline]
pub unsafe fn WICConvertBitmapSource<'a, Param1: ::windows::core::IntoParam<'a, IWICBitmapSource>>(dstformat: *const ::windows::core::GUID, pisrc: Param1) -> ::windows::core::Result<IWICBitmapSource> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICConvertBitmapSource(dstformat: *const ::windows::core::GUID, pisrc: ::windows::core::RawPtr, ppidst: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WICConvertBitmapSource(::core::mem::transmute(dstformat), pisrc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmapSource>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WICCreateBitmapFromSection<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(width: u32, height: u32, pixelformat: *const ::windows::core::GUID, hsection: Param3, stride: u32, offset: u32) -> ::windows::core::Result<IWICBitmap> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICCreateBitmapFromSection(width: u32, height: u32, pixelformat: *const ::windows::core::GUID, hsection: super::super::Foundation::HANDLE, stride: u32, offset: u32, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WICCreateBitmapFromSection(::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), hsection.into_param().abi(), ::core::mem::transmute(stride), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WICCreateBitmapFromSectionEx<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(width: u32, height: u32, pixelformat: *const ::windows::core::GUID, hsection: Param3, stride: u32, offset: u32, desiredaccesslevel: WICSectionAccessLevel) -> ::windows::core::Result<IWICBitmap> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICCreateBitmapFromSectionEx(width: u32, height: u32, pixelformat: *const ::windows::core::GUID, hsection: super::super::Foundation::HANDLE, stride: u32, offset: u32, desiredaccesslevel: WICSectionAccessLevel, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WICCreateBitmapFromSectionEx(::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), hsection.into_param().abi(), ::core::mem::transmute(stride), ::core::mem::transmute(offset), ::core::mem::transmute(desiredaccesslevel), ::core::mem::transmute(&mut result__)).from_abi::<IWICBitmap>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type WICDdsAlphaMode = i32;
pub const WICDdsAlphaModeUnknown: WICDdsAlphaMode = 0i32;
pub const WICDdsAlphaModeStraight: WICDdsAlphaMode = 1i32;
pub const WICDdsAlphaModePremultiplied: WICDdsAlphaMode = 2i32;
pub const WICDdsAlphaModeOpaque: WICDdsAlphaMode = 3i32;
pub const WICDdsAlphaModeCustom: WICDdsAlphaMode = 4i32;
pub const WICDDSALPHAMODE_FORCE_DWORD: WICDdsAlphaMode = 2147483647i32;
pub type WICDdsDimension = i32;
pub const WICDdsTexture1D: WICDdsDimension = 0i32;
pub const WICDdsTexture2D: WICDdsDimension = 1i32;
pub const WICDdsTexture3D: WICDdsDimension = 2i32;
pub const WICDdsTextureCube: WICDdsDimension = 3i32;
pub const WICDDSTEXTURE_FORCE_DWORD: WICDdsDimension = 2147483647i32;
#[repr(C)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for WICDdsFormatInfo {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for WICDdsFormatInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICDdsFormatInfo>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for WICDdsFormatInfo {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for WICDdsFormatInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for WICDdsParameters {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for WICDdsParameters {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICDdsParameters>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for WICDdsParameters {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for WICDdsParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WICDecodeOptions = i32;
pub const WICDecodeMetadataCacheOnDemand: WICDecodeOptions = 0i32;
pub const WICDecodeMetadataCacheOnLoad: WICDecodeOptions = 1i32;
pub const WICMETADATACACHEOPTION_FORCE_DWORD: WICDecodeOptions = 2147483647i32;
#[inline]
pub unsafe fn WICGetMetadataContentSize<'a, Param1: ::windows::core::IntoParam<'a, IWICMetadataWriter>>(guidcontainerformat: *const ::windows::core::GUID, piwriter: Param1) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICGetMetadataContentSize(guidcontainerformat: *const ::windows::core::GUID, piwriter: ::windows::core::RawPtr, pcbsize: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: u64 = ::core::mem::zeroed();
        WICGetMetadataContentSize(::core::mem::transmute(guidcontainerformat), piwriter.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type WICGifApplicationExtensionProperties = u32;
pub const WICGifApplicationExtensionApplication: WICGifApplicationExtensionProperties = 1u32;
pub const WICGifApplicationExtensionData: WICGifApplicationExtensionProperties = 2u32;
pub const WICGifApplicationExtensionProperties_FORCE_DWORD: WICGifApplicationExtensionProperties = 2147483647u32;
pub type WICGifCommentExtensionProperties = u32;
pub const WICGifCommentExtensionText: WICGifCommentExtensionProperties = 1u32;
pub const WICGifCommentExtensionProperties_FORCE_DWORD: WICGifCommentExtensionProperties = 2147483647u32;
pub type WICGifGraphicControlExtensionProperties = u32;
pub const WICGifGraphicControlExtensionDisposal: WICGifGraphicControlExtensionProperties = 1u32;
pub const WICGifGraphicControlExtensionUserInputFlag: WICGifGraphicControlExtensionProperties = 2u32;
pub const WICGifGraphicControlExtensionTransparencyFlag: WICGifGraphicControlExtensionProperties = 3u32;
pub const WICGifGraphicControlExtensionDelay: WICGifGraphicControlExtensionProperties = 4u32;
pub const WICGifGraphicControlExtensionTransparentColorIndex: WICGifGraphicControlExtensionProperties = 5u32;
pub const WICGifGraphicControlExtensionProperties_FORCE_DWORD: WICGifGraphicControlExtensionProperties = 2147483647u32;
pub type WICGifImageDescriptorProperties = u32;
pub const WICGifImageDescriptorLeft: WICGifImageDescriptorProperties = 1u32;
pub const WICGifImageDescriptorTop: WICGifImageDescriptorProperties = 2u32;
pub const WICGifImageDescriptorWidth: WICGifImageDescriptorProperties = 3u32;
pub const WICGifImageDescriptorHeight: WICGifImageDescriptorProperties = 4u32;
pub const WICGifImageDescriptorLocalColorTableFlag: WICGifImageDescriptorProperties = 5u32;
pub const WICGifImageDescriptorInterlaceFlag: WICGifImageDescriptorProperties = 6u32;
pub const WICGifImageDescriptorSortFlag: WICGifImageDescriptorProperties = 7u32;
pub const WICGifImageDescriptorLocalColorTableSize: WICGifImageDescriptorProperties = 8u32;
pub const WICGifImageDescriptorProperties_FORCE_DWORD: WICGifImageDescriptorProperties = 2147483647u32;
pub type WICGifLogicalScreenDescriptorProperties = u32;
pub const WICGifLogicalScreenSignature: WICGifLogicalScreenDescriptorProperties = 1u32;
pub const WICGifLogicalScreenDescriptorWidth: WICGifLogicalScreenDescriptorProperties = 2u32;
pub const WICGifLogicalScreenDescriptorHeight: WICGifLogicalScreenDescriptorProperties = 3u32;
pub const WICGifLogicalScreenDescriptorGlobalColorTableFlag: WICGifLogicalScreenDescriptorProperties = 4u32;
pub const WICGifLogicalScreenDescriptorColorResolution: WICGifLogicalScreenDescriptorProperties = 5u32;
pub const WICGifLogicalScreenDescriptorSortFlag: WICGifLogicalScreenDescriptorProperties = 6u32;
pub const WICGifLogicalScreenDescriptorGlobalColorTableSize: WICGifLogicalScreenDescriptorProperties = 7u32;
pub const WICGifLogicalScreenDescriptorBackgroundColorIndex: WICGifLogicalScreenDescriptorProperties = 8u32;
pub const WICGifLogicalScreenDescriptorPixelAspectRatio: WICGifLogicalScreenDescriptorProperties = 9u32;
pub const WICGifLogicalScreenDescriptorProperties_FORCE_DWORD: WICGifLogicalScreenDescriptorProperties = 2147483647u32;
pub type WICHeifHdrProperties = u32;
pub const WICHeifHdrMaximumLuminanceLevel: WICHeifHdrProperties = 1u32;
pub const WICHeifHdrMaximumFrameAverageLuminanceLevel: WICHeifHdrProperties = 2u32;
pub const WICHeifHdrMinimumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = 3u32;
pub const WICHeifHdrMaximumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = 4u32;
pub const WICHeifHdrCustomVideoPrimaries: WICHeifHdrProperties = 5u32;
pub const WICHeifHdrProperties_FORCE_DWORD: WICHeifHdrProperties = 2147483647u32;
pub type WICHeifProperties = u32;
pub const WICHeifOrientation: WICHeifProperties = 1u32;
pub const WICHeifProperties_FORCE_DWORD: WICHeifProperties = 2147483647u32;
#[repr(C)]
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
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for WICImageParameters {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for WICImageParameters {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICImageParameters>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for WICImageParameters {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for WICImageParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WICJpegChrominanceProperties = u32;
pub const WICJpegChrominanceTable: WICJpegChrominanceProperties = 1u32;
pub const WICJpegChrominanceProperties_FORCE_DWORD: WICJpegChrominanceProperties = 2147483647u32;
pub type WICJpegCommentProperties = u32;
pub const WICJpegCommentText: WICJpegCommentProperties = 1u32;
pub const WICJpegCommentProperties_FORCE_DWORD: WICJpegCommentProperties = 2147483647u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WICJpegFrameHeader {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICJpegFrameHeader {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICJpegFrameHeader>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICJpegFrameHeader {}
impl ::core::default::Default for WICJpegFrameHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WICJpegIndexingOptions = u32;
pub const WICJpegIndexingOptionsGenerateOnDemand: WICJpegIndexingOptions = 0u32;
pub const WICJpegIndexingOptionsGenerateOnLoad: WICJpegIndexingOptions = 1u32;
pub const WICJpegIndexingOptions_FORCE_DWORD: WICJpegIndexingOptions = 2147483647u32;
pub type WICJpegLuminanceProperties = u32;
pub const WICJpegLuminanceTable: WICJpegLuminanceProperties = 1u32;
pub const WICJpegLuminanceProperties_FORCE_DWORD: WICJpegLuminanceProperties = 2147483647u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WICJpegScanHeader {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICJpegScanHeader {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICJpegScanHeader>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICJpegScanHeader {}
impl ::core::default::Default for WICJpegScanHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WICJpegScanType = u32;
pub const WICJpegScanTypeInterleaved: WICJpegScanType = 0u32;
pub const WICJpegScanTypePlanarComponents: WICJpegScanType = 1u32;
pub const WICJpegScanTypeProgressive: WICJpegScanType = 2u32;
pub const WICJpegScanType_FORCE_DWORD: WICJpegScanType = 2147483647u32;
pub type WICJpegTransferMatrix = u32;
pub const WICJpegTransferMatrixIdentity: WICJpegTransferMatrix = 0u32;
pub const WICJpegTransferMatrixBT601: WICJpegTransferMatrix = 1u32;
pub const WICJpegTransferMatrix_FORCE_DWORD: WICJpegTransferMatrix = 2147483647u32;
pub type WICJpegYCrCbSubsamplingOption = i32;
pub const WICJpegYCrCbSubsamplingDefault: WICJpegYCrCbSubsamplingOption = 0i32;
pub const WICJpegYCrCbSubsampling420: WICJpegYCrCbSubsamplingOption = 1i32;
pub const WICJpegYCrCbSubsampling422: WICJpegYCrCbSubsamplingOption = 2i32;
pub const WICJpegYCrCbSubsampling444: WICJpegYCrCbSubsamplingOption = 3i32;
pub const WICJpegYCrCbSubsampling440: WICJpegYCrCbSubsamplingOption = 4i32;
pub const WICJPEGYCRCBSUBSAMPLING_FORCE_DWORD: WICJpegYCrCbSubsamplingOption = 2147483647i32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WICMapGuidToShortName<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(guid: *const ::windows::core::GUID, cchname: u32, wzname: Param2, pcchactual: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICMapGuidToShortName(guid: *const ::windows::core::GUID, cchname: u32, wzname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT;
        }
        WICMapGuidToShortName(::core::mem::transmute(guid), ::core::mem::transmute(cchname), wzname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WICMapSchemaToName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(guidmetadataformat: *const ::windows::core::GUID, pwzschema: Param1, cchname: u32, wzname: Param3, pcchactual: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICMapSchemaToName(guidmetadataformat: *const ::windows::core::GUID, pwzschema: super::super::Foundation::PWSTR, cchname: u32, wzname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT;
        }
        WICMapSchemaToName(::core::mem::transmute(guidmetadataformat), pwzschema.into_param().abi(), ::core::mem::transmute(cchname), wzname.into_param().abi(), ::core::mem::transmute(pcchactual)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WICMapShortNameToGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(wzname: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICMapShortNameToGuid(wzname: super::super::Foundation::PWSTR, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        WICMapShortNameToGuid(wzname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WICMatchMetadataContent<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, pistream: Param2) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICMatchMetadataContent(guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, pistream: ::windows::core::RawPtr, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        WICMatchMetadataContent(::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), pistream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type WICMetadataCreationOptions = i32;
pub const WICMetadataCreationDefault: WICMetadataCreationOptions = 0i32;
pub const WICMetadataCreationAllowUnknown: WICMetadataCreationOptions = 0i32;
pub const WICMetadataCreationFailUnknown: WICMetadataCreationOptions = 65536i32;
pub const WICMetadataCreationMask: WICMetadataCreationOptions = -65536i32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WICMetadataHeader {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICMetadataHeader {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICMetadataHeader>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICMetadataHeader {}
impl ::core::default::Default for WICMetadataHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WICMetadataPattern {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICMetadataPattern {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICMetadataPattern>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICMetadataPattern {}
impl ::core::default::Default for WICMetadataPattern {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WICNamedWhitePoint = i32;
pub const WICWhitePointDefault: WICNamedWhitePoint = 1i32;
pub const WICWhitePointDaylight: WICNamedWhitePoint = 2i32;
pub const WICWhitePointCloudy: WICNamedWhitePoint = 4i32;
pub const WICWhitePointShade: WICNamedWhitePoint = 8i32;
pub const WICWhitePointTungsten: WICNamedWhitePoint = 16i32;
pub const WICWhitePointFluorescent: WICNamedWhitePoint = 32i32;
pub const WICWhitePointFlash: WICNamedWhitePoint = 64i32;
pub const WICWhitePointUnderwater: WICNamedWhitePoint = 128i32;
pub const WICWhitePointCustom: WICNamedWhitePoint = 256i32;
pub const WICWhitePointAutoWhiteBalance: WICNamedWhitePoint = 512i32;
pub const WICWhitePointAsShot: WICNamedWhitePoint = 1i32;
pub const WICNAMEDWHITEPOINT_FORCE_DWORD: WICNamedWhitePoint = 2147483647i32;
pub type WICPersistOptions = i32;
pub const WICPersistOptionDefault: WICPersistOptions = 0i32;
pub const WICPersistOptionLittleEndian: WICPersistOptions = 0i32;
pub const WICPersistOptionBigEndian: WICPersistOptions = 1i32;
pub const WICPersistOptionStrictFormat: WICPersistOptions = 2i32;
pub const WICPersistOptionNoCacheStream: WICPersistOptions = 4i32;
pub const WICPersistOptionPreferUTF8: WICPersistOptions = 8i32;
pub const WICPersistOptionMask: WICPersistOptions = 65535i32;
pub type WICPixelFormatNumericRepresentation = u32;
pub const WICPixelFormatNumericRepresentationUnspecified: WICPixelFormatNumericRepresentation = 0u32;
pub const WICPixelFormatNumericRepresentationIndexed: WICPixelFormatNumericRepresentation = 1u32;
pub const WICPixelFormatNumericRepresentationUnsignedInteger: WICPixelFormatNumericRepresentation = 2u32;
pub const WICPixelFormatNumericRepresentationSignedInteger: WICPixelFormatNumericRepresentation = 3u32;
pub const WICPixelFormatNumericRepresentationFixed: WICPixelFormatNumericRepresentation = 4u32;
pub const WICPixelFormatNumericRepresentationFloat: WICPixelFormatNumericRepresentation = 5u32;
pub const WICPixelFormatNumericRepresentation_FORCE_DWORD: WICPixelFormatNumericRepresentation = 2147483647u32;
pub type WICPlanarOptions = i32;
pub const WICPlanarOptionsDefault: WICPlanarOptions = 0i32;
pub const WICPlanarOptionsPreserveSubsampling: WICPlanarOptions = 1i32;
pub const WICPLANAROPTIONS_FORCE_DWORD: WICPlanarOptions = 2147483647i32;
pub type WICPngBkgdProperties = u32;
pub const WICPngBkgdBackgroundColor: WICPngBkgdProperties = 1u32;
pub const WICPngBkgdProperties_FORCE_DWORD: WICPngBkgdProperties = 2147483647u32;
pub type WICPngChrmProperties = u32;
pub const WICPngChrmWhitePointX: WICPngChrmProperties = 1u32;
pub const WICPngChrmWhitePointY: WICPngChrmProperties = 2u32;
pub const WICPngChrmRedX: WICPngChrmProperties = 3u32;
pub const WICPngChrmRedY: WICPngChrmProperties = 4u32;
pub const WICPngChrmGreenX: WICPngChrmProperties = 5u32;
pub const WICPngChrmGreenY: WICPngChrmProperties = 6u32;
pub const WICPngChrmBlueX: WICPngChrmProperties = 7u32;
pub const WICPngChrmBlueY: WICPngChrmProperties = 8u32;
pub const WICPngChrmProperties_FORCE_DWORD: WICPngChrmProperties = 2147483647u32;
pub type WICPngFilterOption = i32;
pub const WICPngFilterUnspecified: WICPngFilterOption = 0i32;
pub const WICPngFilterNone: WICPngFilterOption = 1i32;
pub const WICPngFilterSub: WICPngFilterOption = 2i32;
pub const WICPngFilterUp: WICPngFilterOption = 3i32;
pub const WICPngFilterAverage: WICPngFilterOption = 4i32;
pub const WICPngFilterPaeth: WICPngFilterOption = 5i32;
pub const WICPngFilterAdaptive: WICPngFilterOption = 6i32;
pub const WICPNGFILTEROPTION_FORCE_DWORD: WICPngFilterOption = 2147483647i32;
pub type WICPngGamaProperties = u32;
pub const WICPngGamaGamma: WICPngGamaProperties = 1u32;
pub const WICPngGamaProperties_FORCE_DWORD: WICPngGamaProperties = 2147483647u32;
pub type WICPngHistProperties = u32;
pub const WICPngHistFrequencies: WICPngHistProperties = 1u32;
pub const WICPngHistProperties_FORCE_DWORD: WICPngHistProperties = 2147483647u32;
pub type WICPngIccpProperties = u32;
pub const WICPngIccpProfileName: WICPngIccpProperties = 1u32;
pub const WICPngIccpProfileData: WICPngIccpProperties = 2u32;
pub const WICPngIccpProperties_FORCE_DWORD: WICPngIccpProperties = 2147483647u32;
pub type WICPngItxtProperties = u32;
pub const WICPngItxtKeyword: WICPngItxtProperties = 1u32;
pub const WICPngItxtCompressionFlag: WICPngItxtProperties = 2u32;
pub const WICPngItxtLanguageTag: WICPngItxtProperties = 3u32;
pub const WICPngItxtTranslatedKeyword: WICPngItxtProperties = 4u32;
pub const WICPngItxtText: WICPngItxtProperties = 5u32;
pub const WICPngItxtProperties_FORCE_DWORD: WICPngItxtProperties = 2147483647u32;
pub type WICPngSrgbProperties = u32;
pub const WICPngSrgbRenderingIntent: WICPngSrgbProperties = 1u32;
pub const WICPngSrgbProperties_FORCE_DWORD: WICPngSrgbProperties = 2147483647u32;
pub type WICPngTimeProperties = u32;
pub const WICPngTimeYear: WICPngTimeProperties = 1u32;
pub const WICPngTimeMonth: WICPngTimeProperties = 2u32;
pub const WICPngTimeDay: WICPngTimeProperties = 3u32;
pub const WICPngTimeHour: WICPngTimeProperties = 4u32;
pub const WICPngTimeMinute: WICPngTimeProperties = 5u32;
pub const WICPngTimeSecond: WICPngTimeProperties = 6u32;
pub const WICPngTimeProperties_FORCE_DWORD: WICPngTimeProperties = 2147483647u32;
pub type WICProgressNotification = i32;
pub const WICProgressNotificationBegin: WICProgressNotification = 65536i32;
pub const WICProgressNotificationEnd: WICProgressNotification = 131072i32;
pub const WICProgressNotificationFrequent: WICProgressNotification = 262144i32;
pub const WICProgressNotificationAll: WICProgressNotification = -65536i32;
pub const WICPROGRESSNOTIFICATION_FORCE_DWORD: WICProgressNotification = 2147483647i32;
pub type WICProgressOperation = i32;
pub const WICProgressOperationCopyPixels: WICProgressOperation = 1i32;
pub const WICProgressOperationWritePixels: WICProgressOperation = 2i32;
pub const WICProgressOperationAll: WICProgressOperation = 65535i32;
pub const WICPROGRESSOPERATION_FORCE_DWORD: WICProgressOperation = 2147483647i32;
pub type WICRawCapabilities = i32;
pub const WICRawCapabilityNotSupported: WICRawCapabilities = 0i32;
pub const WICRawCapabilityGetSupported: WICRawCapabilities = 1i32;
pub const WICRawCapabilityFullySupported: WICRawCapabilities = 2i32;
pub const WICRAWCAPABILITIES_FORCE_DWORD: WICRawCapabilities = 2147483647i32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WICRawCapabilitiesInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICRawCapabilitiesInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICRawCapabilitiesInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICRawCapabilitiesInfo {}
impl ::core::default::Default for WICRawCapabilitiesInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WICRawChangeNotification_Contrast: u32 = 16u32;
pub const WICRawChangeNotification_DestinationColorContext: u32 = 1024u32;
pub const WICRawChangeNotification_ExposureCompensation: u32 = 1u32;
pub const WICRawChangeNotification_Gamma: u32 = 32u32;
pub const WICRawChangeNotification_KelvinWhitePoint: u32 = 4u32;
pub const WICRawChangeNotification_NamedWhitePoint: u32 = 2u32;
pub const WICRawChangeNotification_NoiseReduction: u32 = 512u32;
pub const WICRawChangeNotification_RGBWhitePoint: u32 = 8u32;
pub const WICRawChangeNotification_RenderMode: u32 = 8192u32;
pub const WICRawChangeNotification_Rotation: u32 = 4096u32;
pub const WICRawChangeNotification_Saturation: u32 = 128u32;
pub const WICRawChangeNotification_Sharpness: u32 = 64u32;
pub const WICRawChangeNotification_Tint: u32 = 256u32;
pub const WICRawChangeNotification_ToneCurve: u32 = 2048u32;
pub type WICRawParameterSet = i32;
pub const WICAsShotParameterSet: WICRawParameterSet = 1i32;
pub const WICUserAdjustedParameterSet: WICRawParameterSet = 2i32;
pub const WICAutoAdjustedParameterSet: WICRawParameterSet = 3i32;
pub const WICRAWPARAMETERSET_FORCE_DWORD: WICRawParameterSet = 2147483647i32;
pub type WICRawRenderMode = i32;
pub const WICRawRenderModeDraft: WICRawRenderMode = 1i32;
pub const WICRawRenderModeNormal: WICRawRenderMode = 2i32;
pub const WICRawRenderModeBestQuality: WICRawRenderMode = 3i32;
pub const WICRAWRENDERMODE_FORCE_DWORD: WICRawRenderMode = 2147483647i32;
pub type WICRawRotationCapabilities = i32;
pub const WICRawRotationCapabilityNotSupported: WICRawRotationCapabilities = 0i32;
pub const WICRawRotationCapabilityGetSupported: WICRawRotationCapabilities = 1i32;
pub const WICRawRotationCapabilityNinetyDegreesSupported: WICRawRotationCapabilities = 2i32;
pub const WICRawRotationCapabilityFullySupported: WICRawRotationCapabilities = 3i32;
pub const WICRAWROTATIONCAPABILITIES_FORCE_DWORD: WICRawRotationCapabilities = 2147483647i32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WICRawToneCurve {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICRawToneCurve {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICRawToneCurve>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICRawToneCurve {}
impl ::core::default::Default for WICRawToneCurve {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WICRawToneCurvePoint {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICRawToneCurvePoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICRawToneCurvePoint>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICRawToneCurvePoint {}
impl ::core::default::Default for WICRawToneCurvePoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WICRect {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WICRect {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WICRect>()) == 0 }
    }
}
impl ::core::cmp::Eq for WICRect {}
impl ::core::default::Default for WICRect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WICSectionAccessLevel = u32;
pub const WICSectionAccessLevelRead: WICSectionAccessLevel = 1u32;
pub const WICSectionAccessLevelReadWrite: WICSectionAccessLevel = 3u32;
pub const WICSectionAccessLevel_FORCE_DWORD: WICSectionAccessLevel = 2147483647u32;
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WICSerializeMetadataContent<'a, Param1: ::windows::core::IntoParam<'a, IWICMetadataWriter>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(guidcontainerformat: *const ::windows::core::GUID, piwriter: Param1, dwpersistoptions: u32, pistream: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WICSerializeMetadataContent(guidcontainerformat: *const ::windows::core::GUID, piwriter: ::windows::core::RawPtr, dwpersistoptions: u32, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        WICSerializeMetadataContent(::core::mem::transmute(guidcontainerformat), piwriter.into_param().abi(), ::core::mem::transmute(dwpersistoptions), pistream.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type WICTiffCompressionOption = i32;
pub const WICTiffCompressionDontCare: WICTiffCompressionOption = 0i32;
pub const WICTiffCompressionNone: WICTiffCompressionOption = 1i32;
pub const WICTiffCompressionCCITT3: WICTiffCompressionOption = 2i32;
pub const WICTiffCompressionCCITT4: WICTiffCompressionOption = 3i32;
pub const WICTiffCompressionLZW: WICTiffCompressionOption = 4i32;
pub const WICTiffCompressionRLE: WICTiffCompressionOption = 5i32;
pub const WICTiffCompressionZIP: WICTiffCompressionOption = 6i32;
pub const WICTiffCompressionLZWHDifferencing: WICTiffCompressionOption = 7i32;
pub const WICTIFFCOMPRESSIONOPTION_FORCE_DWORD: WICTiffCompressionOption = 2147483647i32;
pub type WICWebpAnimProperties = u32;
pub const WICWebpAnimLoopCount: WICWebpAnimProperties = 1u32;
pub const WICWebpAnimProperties_FORCE_DWORD: WICWebpAnimProperties = 2147483647u32;
pub type WICWebpAnmfProperties = u32;
pub const WICWebpAnmfFrameDuration: WICWebpAnmfProperties = 1u32;
pub const WICWebpAnmfProperties_FORCE_DWORD: WICWebpAnmfProperties = 2147483647u32;
pub const WIC_JPEG_HUFFMAN_BASELINE_ONE: u32 = 0u32;
pub const WIC_JPEG_HUFFMAN_BASELINE_THREE: u32 = 1118464u32;
pub const WIC_JPEG_MAX_COMPONENT_COUNT: u32 = 4u32;
pub const WIC_JPEG_MAX_TABLE_INDEX: u32 = 3u32;
pub const WIC_JPEG_QUANTIZATION_BASELINE_ONE: u32 = 0u32;
pub const WIC_JPEG_QUANTIZATION_BASELINE_THREE: u32 = 65792u32;
pub const WIC_JPEG_SAMPLE_FACTORS_ONE: u32 = 17u32;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_420: u32 = 1118498u32;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_422: u32 = 1118497u32;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_440: u32 = 1118482u32;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_444: u32 = 1118481u32;
pub const WINCODEC_ERR_ABORTED: i32 = -2147467260i32;
pub const WINCODEC_ERR_ACCESSDENIED: i32 = -2147024891i32;
pub const WINCODEC_ERR_BASE: u32 = 8192u32;
pub const WINCODEC_ERR_GENERIC_ERROR: i32 = -2147467259i32;
pub const WINCODEC_ERR_INVALIDPARAMETER: i32 = -2147024809i32;
pub const WINCODEC_ERR_NOTIMPLEMENTED: i32 = -2147467263i32;
pub const WINCODEC_ERR_OUTOFMEMORY: i32 = -2147024882i32;
pub const WINCODEC_SDK_VERSION: u32 = 567u32;
pub const WINCODEC_SDK_VERSION1: u32 = 566u32;
pub const WINCODEC_SDK_VERSION2: u32 = 567u32;
