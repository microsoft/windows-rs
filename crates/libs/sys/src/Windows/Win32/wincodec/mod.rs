windows_link::link!("windowscodecs.dll" "system" fn WICConvertBitmapSource(dstformat : REFWICPixelFormatGUID, pisrc : *mut core::ffi::c_void, ppidst : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("windowscodecs.dll" "system" fn WICCreateBitmapFromSection(width : u32, height : u32, pixelformat : REFWICPixelFormatGUID, hsection : super::HANDLE, stride : u32, offset : u32, ppibitmap : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("windowscodecs.dll" "system" fn WICCreateBitmapFromSectionEx(width : u32, height : u32, pixelformat : REFWICPixelFormatGUID, hsection : super::HANDLE, stride : u32, offset : u32, desiredaccesslevel : WICSectionAccessLevel, ppibitmap : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("windowscodecs.dll" "system" fn WICMapGuidToShortName(guid : *const windows_sys::core::GUID, cchname : u32, wzname : *mut u16, pcchactual : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("windowscodecs.dll" "system" fn WICMapSchemaToName(guidmetadataformat : *const windows_sys::core::GUID, pwzschema : windows_sys::core::PCWSTR, cchname : u32, wzname : *mut u16, pcchactual : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("windowscodecs.dll" "system" fn WICMapShortNameToGuid(wzname : windows_sys::core::PCWSTR, pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
pub const CATID_WICBitmapDecoders: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7ed96837_96f0_4812_b211_f13c24117ed3);
pub const CATID_WICBitmapEncoders: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac757296_3522_4e11_9862_c17be5a1767e);
pub const CATID_WICFormatConverters: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7835eae8_bf14_49d1_93ce_533a407b2248);
pub const CATID_WICMetadataReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05af94d8_7174_4cd2_be4a_4124b80ee4b8);
pub const CATID_WICMetadataWriter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xabe3b9a4_257d_4b97_bd1a_294af496222e);
pub const CATID_WICPixelFormats: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2b46e70f_cda7_473e_89f6_dc9630a2390b);
pub const CLSID_WICAdngDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x981d9411_909e_42a7_8f5d_a747ff052edb);
pub const CLSID_WICBmpDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6b462062_7cbf_400d_9fdb_813dd10f2778);
pub const CLSID_WICBmpEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69be8bb4_d66d_47c8_865a_ed1589433782);
pub const CLSID_WICDdsDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9053699f_a341_429d_9e90_ee437cf80c73);
pub const CLSID_WICDdsEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa61dde94_66ce_4ac1_881b_71680588895e);
pub const CLSID_WICDefaultFormatConverter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1a3f11dc_b514_4b17_8c5f_2154513852f1);
pub const CLSID_WICFormatConverterHighColor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac75d454_9f37_48f8_b972_4e19bc856011);
pub const CLSID_WICFormatConverterNChannel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc17cabb2_d4a3_47d7_a557_339b2efbd4f1);
pub const CLSID_WICFormatConverterWMPhoto: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9cb5172b_d600_46ba_ab77_77bb7e3a00d9);
pub const CLSID_WICGifDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x381dda3c_9ce9_4834_a23e_1f98f8fc52be);
pub const CLSID_WICGifEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x114f5598_0b22_40a0_86a1_c83ea495adbd);
pub const CLSID_WICHeifDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe9a4a80a_44fe_4de4_8971_7150b10a5199);
pub const CLSID_WICHeifEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0dbecec1_9eb3_4860_9c6f_ddbe86634575);
pub const CLSID_WICIcoDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc61bfcdf_2e0f_4aad_a8d7_e06bafebcdfe);
pub const CLSID_WICImagingCategories: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfae3d380_fea4_4623_8c75_c6b61110b681);
pub const CLSID_WICImagingFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x317d06e8_5f24_433d_bdf7_79ce68d8abc2);
pub const CLSID_WICJpegDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9456a480_e88b_43ea_9e73_0b2d9b71b1ca);
pub const CLSID_WICJpegEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1a34f5c1_4a5a_46dc_b644_1f4567e7a676);
pub const CLSID_WICJpegQualcommPhoneEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x68ed5c62_f534_4979_b2b3_686a12b2b34c);
pub const CLSID_WICJpegXLDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfc6ceece_aef5_4a23_96ec_5984ffb486d9);
pub const CLSID_WICJpegXLEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e4ecd3b_1ba6_4636_8198_56c73040964a);
pub const CLSID_WICPlanarFormatConverter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x184132b8_32f8_4784_9131_dd7224b23438);
pub const CLSID_WICPngDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe018945b_aa86_4008_9bd4_6777a1e40c11);
pub const CLSID_WICPngEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x27949969_876a_41d7_9447_568f6a35a4dc);
pub const CLSID_WICRAWDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41945702_8302_44a6_9445_ac98e8afa086);
pub const CLSID_WICTiffDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb54e85d9_fe23_499f_8b88_6acea713752b);
pub const CLSID_WICTiffEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0131be10_2001_4c5f_a9b0_cc88fab64ce8);
pub const CLSID_WICWebpDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7693e886_51c9_4070_8419_9f70738ec8fa);
pub const CLSID_WICWmpDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa26cec36_234c_4950_ae16_e34aace71d0d);
pub const CLSID_WICWmpEncoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac4ce3cb_e1c1_44cd_8215_5a1665509ec2);
pub const FACILITY_WINCODEC_ERR: u32 = 2200;
pub const GUID_ContainerFormatAdng: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3ff6d0d_38c0_41c4_b1fe_1f3824f17b84);
pub const GUID_ContainerFormatBmp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0af1d87e_fcfe_4188_bdeb_a7906471cbe3);
pub const GUID_ContainerFormatDds: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9967cb95_2e85_4ac8_8ca2_83d7ccd425c9);
pub const GUID_ContainerFormatGif: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1f8a5601_7d4d_4cbd_9c82_1bc8d4eeb9a5);
pub const GUID_ContainerFormatHeif: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe1e62521_6787_405b_a339_500715b5763f);
pub const GUID_ContainerFormatIco: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa3a860c4_338f_4c17_919a_fba4b5628f21);
pub const GUID_ContainerFormatJpeg: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const GUID_ContainerFormatJpegXL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfec14e3f_427a_4736_aae6_27ed84f69322);
pub const GUID_ContainerFormatPng: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b7cfaf4_713f_473c_bbcd_6137425faeaf);
pub const GUID_ContainerFormatRaw: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfe99ce60_f19c_433c_a3ae_00acefa9ca21);
pub const GUID_ContainerFormatTiff: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x163bcc30_e2e9_4f0b_961d_a3e9fdb788a3);
pub const GUID_ContainerFormatWebp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe094b0e2_67f2_45b3_b0ea_115337ca7cf3);
pub const GUID_ContainerFormatWmp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57a37caa_367a_4540_916b_f183c5093a4b);
pub const GUID_VendorMicrosoft: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf0e749ca_edef_4589_a73a_ee0e626a2a2b);
pub const GUID_VendorMicrosoftBuiltIn: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x257a30fd_06b6_462b_aea4_63f70b86e533);
pub const GUID_WICPixelFormat112bpp6ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc937);
pub const GUID_WICPixelFormat112bpp7Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92a);
pub const GUID_WICPixelFormat128bpp7ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc938);
pub const GUID_WICPixelFormat128bpp8Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92b);
pub const GUID_WICPixelFormat128bppPRGBAFloat: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91a);
pub const GUID_WICPixelFormat128bppRGBAFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91e);
pub const GUID_WICPixelFormat128bppRGBAFloat: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc919);
pub const GUID_WICPixelFormat128bppRGBFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc941);
pub const GUID_WICPixelFormat128bppRGBFloat: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91b);
pub const GUID_WICPixelFormat144bpp8ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc939);
pub const GUID_WICPixelFormat16bppBGR555: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc909);
pub const GUID_WICPixelFormat16bppBGR565: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90a);
pub const GUID_WICPixelFormat16bppBGRA5551: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05ec7c2b_f1e6_4961_ad46_e1cc810a87d2);
pub const GUID_WICPixelFormat16bppCbCr: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xff95ba6e_11e0_4263_bb45_01721f3460a4);
pub const GUID_WICPixelFormat16bppCbQuantizedDctCoefficients: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd2c4ff61_56a5_49c2_8b5c_4c1925964837);
pub const GUID_WICPixelFormat16bppCrQuantizedDctCoefficients: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2fe354f0_1680_42d8_9231_e73c0565bfc1);
pub const GUID_WICPixelFormat16bppGray: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90b);
pub const GUID_WICPixelFormat16bppGrayFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc913);
pub const GUID_WICPixelFormat16bppGrayHalf: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93e);
pub const GUID_WICPixelFormat16bppYQuantizedDctCoefficients: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa355f433_48e8_4a42_84d8_e2aa26ca80a4);
pub const GUID_WICPixelFormat1bppIndexed: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc901);
pub const GUID_WICPixelFormat24bpp3Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc920);
pub const GUID_WICPixelFormat24bppBGR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90c);
pub const GUID_WICPixelFormat24bppRGB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90d);
pub const GUID_WICPixelFormat24bppRGBGain: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa5022b24_7109_443b_9948_25b6ed8f39fd);
pub const GUID_WICPixelFormat2bppGray: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc906);
pub const GUID_WICPixelFormat2bppIndexed: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc902);
pub const GUID_WICPixelFormat32bpp3ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92e);
pub const GUID_WICPixelFormat32bpp4Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc921);
pub const GUID_WICPixelFormat32bppBGR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90e);
pub const GUID_WICPixelFormat32bppBGR101010: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc914);
pub const GUID_WICPixelFormat32bppBGRA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90f);
pub const GUID_WICPixelFormat32bppBGRGain: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x837d6738_208a_43e0_8995_79ab74407402);
pub const GUID_WICPixelFormat32bppCMYK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91c);
pub const GUID_WICPixelFormat32bppGrayFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93f);
pub const GUID_WICPixelFormat32bppGrayFloat: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc911);
pub const GUID_WICPixelFormat32bppPBGRA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc910);
pub const GUID_WICPixelFormat32bppPRGBA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cc4a650_a527_4d37_a916_3142c7ebedba);
pub const GUID_WICPixelFormat32bppR10G10B10A2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x604e1bb5_8a3c_4b65_b11c_bc0b8dd75b7f);
pub const GUID_WICPixelFormat32bppR10G10B10A2HDR10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9c215c5d_1acc_4f0e_a4bc_70fb3ae8fd28);
pub const GUID_WICPixelFormat32bppRGB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd98c6b95_3efe_47d6_bb25_eb1748ab0cf1);
pub const GUID_WICPixelFormat32bppRGBA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf5c7ad2d_6a8d_43dd_a7a8_a29935261ae9);
pub const GUID_WICPixelFormat32bppRGBA1010102: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25238d72_fcf9_4522_b514_5578e5ad55e0);
pub const GUID_WICPixelFormat32bppRGBA1010102XR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00de6b9a_c101_434b_b502_d0165ee1122c);
pub const GUID_WICPixelFormat32bppRGBE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93d);
pub const GUID_WICPixelFormat40bpp4ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92f);
pub const GUID_WICPixelFormat40bpp5Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc922);
pub const GUID_WICPixelFormat40bppCMYKAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92c);
pub const GUID_WICPixelFormat48bpp3Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc926);
pub const GUID_WICPixelFormat48bpp5ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc930);
pub const GUID_WICPixelFormat48bpp6Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc923);
pub const GUID_WICPixelFormat48bppBGR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe605a384_b468_46ce_bb2e_36f180e64313);
pub const GUID_WICPixelFormat48bppBGRFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x49ca140e_cab6_493b_9ddf_60187c37532a);
pub const GUID_WICPixelFormat48bppRGB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc915);
pub const GUID_WICPixelFormat48bppRGBFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc912);
pub const GUID_WICPixelFormat48bppRGBHalf: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93b);
pub const GUID_WICPixelFormat4bppGray: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc907);
pub const GUID_WICPixelFormat4bppIndexed: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc903);
pub const GUID_WICPixelFormat56bpp6ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc931);
pub const GUID_WICPixelFormat56bpp7Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc924);
pub const GUID_WICPixelFormat64bpp3ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc934);
pub const GUID_WICPixelFormat64bpp4Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc927);
pub const GUID_WICPixelFormat64bpp7ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc932);
pub const GUID_WICPixelFormat64bpp8Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc925);
pub const GUID_WICPixelFormat64bppBGRA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1562ff7c_d352_46f9_979e_42976b792246);
pub const GUID_WICPixelFormat64bppBGRAFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x356de33c_54d2_4a23_bb04_9b7bf9b1d42d);
pub const GUID_WICPixelFormat64bppCMYK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91f);
pub const GUID_WICPixelFormat64bppPBGRA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8c518e8e_a4ec_468b_ae70_c9a35a9c5530);
pub const GUID_WICPixelFormat64bppPRGBA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc917);
pub const GUID_WICPixelFormat64bppPRGBAHalf: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x58ad26c2_c623_4d9d_b320_387e49f8c442);
pub const GUID_WICPixelFormat64bppRGB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1182111_186d_4d42_bc6a_9c8303a8dff9);
pub const GUID_WICPixelFormat64bppRGBA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc916);
pub const GUID_WICPixelFormat64bppRGBAFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91d);
pub const GUID_WICPixelFormat64bppRGBAHalf: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93a);
pub const GUID_WICPixelFormat64bppRGBFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc940);
pub const GUID_WICPixelFormat64bppRGBHalf: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc942);
pub const GUID_WICPixelFormat72bpp8ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc933);
pub const GUID_WICPixelFormat80bpp4ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc935);
pub const GUID_WICPixelFormat80bpp5Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc928);
pub const GUID_WICPixelFormat80bppCMYKAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92d);
pub const GUID_WICPixelFormat8bppAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe6cd0116_eeba_4161_aa85_27dd9fb3a895);
pub const GUID_WICPixelFormat8bppCb: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1339f224_6bfe_4c3e_9302_e4f3a6d0ca2a);
pub const GUID_WICPixelFormat8bppCr: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb8145053_2116_49f0_8835_ed844b205c51);
pub const GUID_WICPixelFormat8bppDepth: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4c9c9f45_1d89_4e31_9bc7_69343a0dca69);
pub const GUID_WICPixelFormat8bppGain: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa884022a_af13_4c16_b746_619bf618b878);
pub const GUID_WICPixelFormat8bppGray: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc908);
pub const GUID_WICPixelFormat8bppIndexed: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc904);
pub const GUID_WICPixelFormat8bppY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x91b4db54_2df9_42f0_b449_2909bb3df88e);
pub const GUID_WICPixelFormat96bpp5ChannelsAlpha: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc936);
pub const GUID_WICPixelFormat96bpp6Channels: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc929);
pub const GUID_WICPixelFormat96bppRGBFixedPoint: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc918);
pub const GUID_WICPixelFormat96bppRGBFloat: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe3fed78f_e8db_4acf_84c1_e97f6136b327);
pub const GUID_WICPixelFormatBlackWhite: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc905);
pub const GUID_WICPixelFormatDontCare: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc900);
pub type PFNProgressNotification = Option<unsafe extern "system" fn(pvdata: *mut core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_sys::core::HRESULT>;
pub type REFWICPixelFormatGUID = *const windows_sys::core::GUID;
pub const WIC8BIMIptcDigestIptcDigest: WIC8BIMIptcDigestProperties = 2;
pub const WIC8BIMIptcDigestPString: WIC8BIMIptcDigestProperties = 1;
pub type WIC8BIMIptcDigestProperties = i32;
pub const WIC8BIMIptcDigestProperties_FORCE_DWORD: WIC8BIMIptcDigestProperties = 2147483647;
pub const WIC8BIMIptcEmbeddedIPTC: WIC8BIMIptcProperties = 1;
pub const WIC8BIMIptcPString: WIC8BIMIptcProperties = 0;
pub type WIC8BIMIptcProperties = i32;
pub const WIC8BIMIptcProperties_FORCE_DWORD: WIC8BIMIptcProperties = 2147483647;
pub const WIC8BIMResolutionInfoHResolution: WIC8BIMResolutionInfoProperties = 2;
pub const WIC8BIMResolutionInfoHResolutionUnit: WIC8BIMResolutionInfoProperties = 3;
pub const WIC8BIMResolutionInfoHeightUnit: WIC8BIMResolutionInfoProperties = 7;
pub const WIC8BIMResolutionInfoPString: WIC8BIMResolutionInfoProperties = 1;
pub type WIC8BIMResolutionInfoProperties = i32;
pub const WIC8BIMResolutionInfoProperties_FORCE_DWORD: WIC8BIMResolutionInfoProperties = 2147483647;
pub const WIC8BIMResolutionInfoVResolution: WIC8BIMResolutionInfoProperties = 5;
pub const WIC8BIMResolutionInfoVResolutionUnit: WIC8BIMResolutionInfoProperties = 6;
pub const WIC8BIMResolutionInfoWidthUnit: WIC8BIMResolutionInfoProperties = 4;
pub const WICAllComponents: WICComponentType = 63;
pub const WICAsShotParameterSet: WICRawParameterSet = 1;
pub const WICAutoAdjustedParameterSet: WICRawParameterSet = 3;
pub const WICBITMAPALPHACHANNELOPTIONS_FORCE_DWORD: WICBitmapAlphaChannelOption = 2147483647;
pub const WICBITMAPCHAINTYPE_FORCE_DWORD: WICBitmapChainType = 2147483647;
pub const WICBITMAPCREATECACHEOPTION_FORCE_DWORD: WICBitmapCreateCacheOption = 2147483647;
pub const WICBITMAPDECODERCAPABILITIES_FORCE_DWORD: WICBitmapDecoderCapabilities = 2147483647;
pub const WICBITMAPDITHERTYPE_FORCE_DWORD: WICBitmapDitherType = 2147483647;
pub const WICBITMAPENCODERCACHEOPTION_FORCE_DWORD: WICBitmapEncoderCacheOption = 2147483647;
pub const WICBITMAPINTERPOLATIONMODE_FORCE_DWORD: WICBitmapInterpolationMode = 2147483647;
pub const WICBITMAPLOCKFLAGS_FORCE_DWORD: WICBitmapLockFlags = 2147483647;
pub const WICBITMAPPALETTETYPE_FORCE_DWORD: WICBitmapPaletteType = 2147483647;
pub const WICBITMAPTONEMAPPINGMODE_FORCE_DWORD: WICBitmapToneMappingMode = 2147483647;
pub const WICBITMAPTRANSFORMOPTIONS_FORCE_DWORD: WICBitmapTransformOptions = 2147483647;
pub type WICBitmapAlphaChannelOption = i32;
pub const WICBitmapCacheOnDemand: WICBitmapCreateCacheOption = 1;
pub const WICBitmapCacheOnLoad: WICBitmapCreateCacheOption = 2;
pub type WICBitmapChainType = i32;
pub const WICBitmapChainType_AlphaMap: WICBitmapChainType = 5;
pub const WICBitmapChainType_Alternate: WICBitmapChainType = 1;
pub const WICBitmapChainType_DepthMap: WICBitmapChainType = 6;
pub const WICBitmapChainType_GainMap: WICBitmapChainType = 7;
pub const WICBitmapChainType_Layer: WICBitmapChainType = 2;
pub const WICBitmapChainType_Preview: WICBitmapChainType = 3;
pub const WICBitmapChainType_Thumbnail: WICBitmapChainType = 4;
pub type WICBitmapCreateCacheOption = i32;
pub type WICBitmapDecoderCapabilities = i32;
pub const WICBitmapDecoderCapabilityCanDecodeAllImages: WICBitmapDecoderCapabilities = 2;
pub const WICBitmapDecoderCapabilityCanDecodeSomeImages: WICBitmapDecoderCapabilities = 4;
pub const WICBitmapDecoderCapabilityCanDecodeThumbnail: WICBitmapDecoderCapabilities = 16;
pub const WICBitmapDecoderCapabilityCanEnumerateMetadata: WICBitmapDecoderCapabilities = 8;
pub const WICBitmapDecoderCapabilitySameEncoder: WICBitmapDecoderCapabilities = 1;
pub type WICBitmapDitherType = i32;
pub const WICBitmapDitherTypeDualSpiral4x4: WICBitmapDitherType = 6;
pub const WICBitmapDitherTypeDualSpiral8x8: WICBitmapDitherType = 7;
pub const WICBitmapDitherTypeErrorDiffusion: WICBitmapDitherType = 8;
pub const WICBitmapDitherTypeNone: WICBitmapDitherType = 0;
pub const WICBitmapDitherTypeOrdered16x16: WICBitmapDitherType = 3;
pub const WICBitmapDitherTypeOrdered4x4: WICBitmapDitherType = 1;
pub const WICBitmapDitherTypeOrdered8x8: WICBitmapDitherType = 2;
pub const WICBitmapDitherTypeSolid: WICBitmapDitherType = 0;
pub const WICBitmapDitherTypeSpiral4x4: WICBitmapDitherType = 4;
pub const WICBitmapDitherTypeSpiral8x8: WICBitmapDitherType = 5;
pub const WICBitmapEncoderCacheInMemory: WICBitmapEncoderCacheOption = 0;
pub type WICBitmapEncoderCacheOption = i32;
pub const WICBitmapEncoderCacheTempFile: WICBitmapEncoderCacheOption = 1;
pub const WICBitmapEncoderNoCache: WICBitmapEncoderCacheOption = 2;
pub const WICBitmapIgnoreAlpha: WICBitmapAlphaChannelOption = 2;
pub type WICBitmapInterpolationMode = i32;
pub const WICBitmapInterpolationModeCubic: WICBitmapInterpolationMode = 2;
pub const WICBitmapInterpolationModeFant: WICBitmapInterpolationMode = 3;
pub const WICBitmapInterpolationModeHighQualityCubic: WICBitmapInterpolationMode = 4;
pub const WICBitmapInterpolationModeLinear: WICBitmapInterpolationMode = 1;
pub const WICBitmapInterpolationModeNearestNeighbor: WICBitmapInterpolationMode = 0;
pub type WICBitmapLockFlags = i32;
pub const WICBitmapLockRead: WICBitmapLockFlags = 1;
pub const WICBitmapLockWrite: WICBitmapLockFlags = 2;
pub const WICBitmapNoCache: WICBitmapCreateCacheOption = 0;
pub type WICBitmapPaletteType = i32;
pub const WICBitmapPaletteTypeCustom: WICBitmapPaletteType = 0;
pub const WICBitmapPaletteTypeFixedBW: WICBitmapPaletteType = 2;
pub const WICBitmapPaletteTypeFixedGray16: WICBitmapPaletteType = 11;
pub const WICBitmapPaletteTypeFixedGray256: WICBitmapPaletteType = 12;
pub const WICBitmapPaletteTypeFixedGray4: WICBitmapPaletteType = 10;
pub const WICBitmapPaletteTypeFixedHalftone125: WICBitmapPaletteType = 6;
pub const WICBitmapPaletteTypeFixedHalftone216: WICBitmapPaletteType = 7;
pub const WICBitmapPaletteTypeFixedHalftone252: WICBitmapPaletteType = 8;
pub const WICBitmapPaletteTypeFixedHalftone256: WICBitmapPaletteType = 9;
pub const WICBitmapPaletteTypeFixedHalftone27: WICBitmapPaletteType = 4;
pub const WICBitmapPaletteTypeFixedHalftone64: WICBitmapPaletteType = 5;
pub const WICBitmapPaletteTypeFixedHalftone8: WICBitmapPaletteType = 3;
pub const WICBitmapPaletteTypeFixedWebPalette: WICBitmapPaletteType = 7;
pub const WICBitmapPaletteTypeMedianCut: WICBitmapPaletteType = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WICBitmapPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub EndOfStream: windows_sys::core::BOOL,
}
impl Default for WICBitmapPattern {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WICBitmapPlane {
    pub Format: WICPixelFormatGUID,
    pub pbBuffer: *mut u8,
    pub cbStride: u32,
    pub cbBufferSize: u32,
}
impl Default for WICBitmapPlane {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WICBitmapPlaneDescription {
    pub Format: WICPixelFormatGUID,
    pub Width: u32,
    pub Height: u32,
}
pub type WICBitmapToneMappingMode = i32;
pub const WICBitmapToneMappingMode_D2D: WICBitmapToneMappingMode = 2;
pub const WICBitmapToneMappingMode_Default: WICBitmapToneMappingMode = 1;
pub const WICBitmapToneMappingMode_GainMap: WICBitmapToneMappingMode = 3;
pub const WICBitmapToneMappingMode_None: WICBitmapToneMappingMode = 0;
pub const WICBitmapTransformFlipHorizontal: WICBitmapTransformOptions = 8;
pub const WICBitmapTransformFlipVertical: WICBitmapTransformOptions = 16;
pub type WICBitmapTransformOptions = i32;
pub const WICBitmapTransformRotate0: WICBitmapTransformOptions = 0;
pub const WICBitmapTransformRotate180: WICBitmapTransformOptions = 2;
pub const WICBitmapTransformRotate270: WICBitmapTransformOptions = 3;
pub const WICBitmapTransformRotate90: WICBitmapTransformOptions = 1;
pub const WICBitmapUseAlpha: WICBitmapAlphaChannelOption = 0;
pub const WICBitmapUsePremultipliedAlpha: WICBitmapAlphaChannelOption = 1;
pub const WICCOMPONENTENUMERATEOPTIONS_FORCE_DWORD: WICComponentEnumerateOptions = 2147483647;
pub const WICCOMPONENTSIGNING_FORCE_DWORD: WICComponentSigning = 2147483647;
pub const WICCOMPONENTTYPE_FORCE_DWORD: WICComponentType = 2147483647;
pub type WICColor = u32;
pub const WICColorContextExifColorSpace: WICColorContextType = 2;
pub const WICColorContextProfile: WICColorContextType = 1;
pub type WICColorContextType = i32;
pub const WICColorContextUninitialized: WICColorContextType = 0;
pub const WICComponentDisabled: WICComponentSigning = -2147483648;
pub const WICComponentEnumerateBuiltInOnly: WICComponentEnumerateOptions = 536870912;
pub const WICComponentEnumerateDefault: WICComponentEnumerateOptions = 0;
pub const WICComponentEnumerateDisabled: WICComponentEnumerateOptions = -2147483648;
pub type WICComponentEnumerateOptions = i32;
pub const WICComponentEnumerateRefresh: WICComponentEnumerateOptions = 1;
pub const WICComponentEnumerateUnsigned: WICComponentEnumerateOptions = 1073741824;
pub const WICComponentSafe: WICComponentSigning = 4;
pub const WICComponentSigned: WICComponentSigning = 1;
pub type WICComponentSigning = i32;
pub type WICComponentType = i32;
pub const WICComponentUnsigned: WICComponentSigning = 2;
pub const WICDDSALPHAMODE_FORCE_DWORD: WICDdsAlphaMode = 2147483647;
pub const WICDDSTEXTURE_FORCE_DWORD: WICDdsDimension = 2147483647;
pub type WICDdsAlphaMode = i32;
pub const WICDdsAlphaModeCustom: WICDdsAlphaMode = 4;
pub const WICDdsAlphaModeOpaque: WICDdsAlphaMode = 3;
pub const WICDdsAlphaModePremultiplied: WICDdsAlphaMode = 2;
pub const WICDdsAlphaModeStraight: WICDdsAlphaMode = 1;
pub const WICDdsAlphaModeUnknown: WICDdsAlphaMode = 0;
pub type WICDdsDimension = i32;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct WICDdsFormatInfo {
    pub DxgiFormat: super::DXGI_FORMAT,
    pub BytesPerBlock: u32,
    pub BlockWidth: u32,
    pub BlockHeight: u32,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct WICDdsParameters {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub DxgiFormat: super::DXGI_FORMAT,
    pub Dimension: WICDdsDimension,
    pub AlphaMode: WICDdsAlphaMode,
}
pub const WICDdsTexture1D: WICDdsDimension = 0;
pub const WICDdsTexture2D: WICDdsDimension = 1;
pub const WICDdsTexture3D: WICDdsDimension = 2;
pub const WICDdsTextureCube: WICDdsDimension = 3;
pub const WICDecodeMetadataCacheOnDemand: WICDecodeOptions = 0;
pub const WICDecodeMetadataCacheOnLoad: WICDecodeOptions = 1;
pub type WICDecodeOptions = i32;
pub const WICDecoder: WICComponentType = 1;
pub const WICEncoder: WICComponentType = 2;
pub const WICGainMapMetadata: WICGainMapProperties = 1;
pub type WICGainMapProperties = i32;
pub const WICGainMapProperties_FORCE_DWORD: WICGainMapProperties = 2147483647;
pub const WICGifApplicationExtensionApplication: WICGifApplicationExtensionProperties = 1;
pub const WICGifApplicationExtensionData: WICGifApplicationExtensionProperties = 2;
pub type WICGifApplicationExtensionProperties = i32;
pub const WICGifApplicationExtensionProperties_FORCE_DWORD: WICGifApplicationExtensionProperties = 2147483647;
pub type WICGifCommentExtensionProperties = i32;
pub const WICGifCommentExtensionProperties_FORCE_DWORD: WICGifCommentExtensionProperties = 2147483647;
pub const WICGifCommentExtensionText: WICGifCommentExtensionProperties = 1;
pub const WICGifGraphicControlExtensionDelay: WICGifGraphicControlExtensionProperties = 4;
pub const WICGifGraphicControlExtensionDisposal: WICGifGraphicControlExtensionProperties = 1;
pub type WICGifGraphicControlExtensionProperties = i32;
pub const WICGifGraphicControlExtensionProperties_FORCE_DWORD: WICGifGraphicControlExtensionProperties = 2147483647;
pub const WICGifGraphicControlExtensionTransparencyFlag: WICGifGraphicControlExtensionProperties = 3;
pub const WICGifGraphicControlExtensionTransparentColorIndex: WICGifGraphicControlExtensionProperties = 5;
pub const WICGifGraphicControlExtensionUserInputFlag: WICGifGraphicControlExtensionProperties = 2;
pub const WICGifImageDescriptorHeight: WICGifImageDescriptorProperties = 4;
pub const WICGifImageDescriptorInterlaceFlag: WICGifImageDescriptorProperties = 6;
pub const WICGifImageDescriptorLeft: WICGifImageDescriptorProperties = 1;
pub const WICGifImageDescriptorLocalColorTableFlag: WICGifImageDescriptorProperties = 5;
pub const WICGifImageDescriptorLocalColorTableSize: WICGifImageDescriptorProperties = 8;
pub type WICGifImageDescriptorProperties = i32;
pub const WICGifImageDescriptorProperties_FORCE_DWORD: WICGifImageDescriptorProperties = 2147483647;
pub const WICGifImageDescriptorSortFlag: WICGifImageDescriptorProperties = 7;
pub const WICGifImageDescriptorTop: WICGifImageDescriptorProperties = 2;
pub const WICGifImageDescriptorWidth: WICGifImageDescriptorProperties = 3;
pub const WICGifLogicalScreenDescriptorBackgroundColorIndex: WICGifLogicalScreenDescriptorProperties = 8;
pub const WICGifLogicalScreenDescriptorColorResolution: WICGifLogicalScreenDescriptorProperties = 5;
pub const WICGifLogicalScreenDescriptorGlobalColorTableFlag: WICGifLogicalScreenDescriptorProperties = 4;
pub const WICGifLogicalScreenDescriptorGlobalColorTableSize: WICGifLogicalScreenDescriptorProperties = 7;
pub const WICGifLogicalScreenDescriptorHeight: WICGifLogicalScreenDescriptorProperties = 3;
pub const WICGifLogicalScreenDescriptorPixelAspectRatio: WICGifLogicalScreenDescriptorProperties = 9;
pub type WICGifLogicalScreenDescriptorProperties = i32;
pub const WICGifLogicalScreenDescriptorProperties_FORCE_DWORD: WICGifLogicalScreenDescriptorProperties = 2147483647;
pub const WICGifLogicalScreenDescriptorSortFlag: WICGifLogicalScreenDescriptorProperties = 6;
pub const WICGifLogicalScreenDescriptorWidth: WICGifLogicalScreenDescriptorProperties = 2;
pub const WICGifLogicalScreenSignature: WICGifLogicalScreenDescriptorProperties = 1;
pub const WICHEIFCOMPRESSIONOPTION_FORCE_DWORD: WICHeifCompressionOption = 2147483647;
pub const WICHeifCompressionAV1: WICHeifCompressionOption = 3;
pub const WICHeifCompressionBrotli: WICHeifCompressionOption = 5;
pub const WICHeifCompressionDeflate: WICHeifCompressionOption = 6;
pub const WICHeifCompressionDontCare: WICHeifCompressionOption = 0;
pub const WICHeifCompressionHEVC: WICHeifCompressionOption = 2;
pub const WICHeifCompressionJpegXL: WICHeifCompressionOption = 4;
pub const WICHeifCompressionNone: WICHeifCompressionOption = 1;
pub type WICHeifCompressionOption = i32;
pub const WICHeifHdrCustomVideoPrimaries: WICHeifHdrProperties = 5;
pub const WICHeifHdrMaximumFrameAverageLuminanceLevel: WICHeifHdrProperties = 2;
pub const WICHeifHdrMaximumLuminanceLevel: WICHeifHdrProperties = 1;
pub const WICHeifHdrMaximumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = 4;
pub const WICHeifHdrMinimumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = 3;
pub type WICHeifHdrProperties = i32;
pub const WICHeifHdrProperties_FORCE_DWORD: WICHeifHdrProperties = 2147483647;
pub const WICHeifLayeredImageCanvasColor: WICHeifProperties = 2;
pub const WICHeifLayeredImageLayerPositions: WICHeifProperties = 3;
pub const WICHeifOrientation: WICHeifProperties = 1;
pub type WICHeifProperties = i32;
pub const WICHeifProperties_FORCE_DWORD: WICHeifProperties = 2147483647;
#[repr(C)]
#[cfg(all(feature = "dcommon", feature = "dxgi"))]
#[derive(Clone, Copy, Default)]
pub struct WICImageParameters {
    pub PixelFormat: super::D2D1_PIXEL_FORMAT,
    pub DpiX: f32,
    pub DpiY: f32,
    pub Top: f32,
    pub Left: f32,
    pub PixelWidth: u32,
    pub PixelHeight: u32,
}
pub type WICInProcPointer = *mut u8;
pub const WICJPEGYCRCBSUBSAMPLING_FORCE_DWORD: WICJpegYCrCbSubsamplingOption = 2147483647;
pub type WICJpegChrominanceProperties = i32;
pub const WICJpegChrominanceProperties_FORCE_DWORD: WICJpegChrominanceProperties = 2147483647;
pub const WICJpegChrominanceTable: WICJpegChrominanceProperties = 1;
pub type WICJpegCommentProperties = i32;
pub const WICJpegCommentProperties_FORCE_DWORD: WICJpegCommentProperties = 2147483647;
pub const WICJpegCommentText: WICJpegCommentProperties = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub type WICJpegIndexingOptions = i32;
pub const WICJpegIndexingOptionsGenerateOnDemand: WICJpegIndexingOptions = 0;
pub const WICJpegIndexingOptionsGenerateOnLoad: WICJpegIndexingOptions = 1;
pub const WICJpegIndexingOptions_FORCE_DWORD: WICJpegIndexingOptions = 2147483647;
pub type WICJpegLuminanceProperties = i32;
pub const WICJpegLuminanceProperties_FORCE_DWORD: WICJpegLuminanceProperties = 2147483647;
pub const WICJpegLuminanceTable: WICJpegLuminanceProperties = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub type WICJpegScanType = i32;
pub const WICJpegScanTypeInterleaved: WICJpegScanType = 0;
pub const WICJpegScanTypePlanarComponents: WICJpegScanType = 1;
pub const WICJpegScanTypeProgressive: WICJpegScanType = 2;
pub const WICJpegScanType_FORCE_DWORD: WICJpegScanType = 2147483647;
pub type WICJpegTransferMatrix = i32;
pub const WICJpegTransferMatrixBT601: WICJpegTransferMatrix = 1;
pub const WICJpegTransferMatrixIdentity: WICJpegTransferMatrix = 0;
pub const WICJpegTransferMatrix_FORCE_DWORD: WICJpegTransferMatrix = 2147483647;
pub const WICJpegXLAnimFrameDurationInTicks: WICJpegXLAnimFrameProperties = 1;
pub const WICJpegXLAnimFrameName: WICJpegXLAnimFrameProperties = 2;
pub type WICJpegXLAnimFrameProperties = i32;
pub const WICJpegXLAnimFrameProperties_FORCE_DWORD: WICJpegXLAnimFrameProperties = 2147483647;
pub const WICJpegXLAnimFrameTicksPerSecondDenominator: WICJpegXLAnimProperties = 3;
pub const WICJpegXLAnimFrameTicksPerSecondNumerator: WICJpegXLAnimProperties = 2;
pub const WICJpegXLAnimLoopCount: WICJpegXLAnimProperties = 1;
pub type WICJpegXLAnimProperties = i32;
pub const WICJpegXLAnimProperties_FORCE_DWORD: WICJpegXLAnimProperties = 2147483647;
pub const WICJpegYCrCbSubsampling420: WICJpegYCrCbSubsamplingOption = 1;
pub const WICJpegYCrCbSubsampling422: WICJpegYCrCbSubsamplingOption = 2;
pub const WICJpegYCrCbSubsampling440: WICJpegYCrCbSubsamplingOption = 4;
pub const WICJpegYCrCbSubsampling444: WICJpegYCrCbSubsamplingOption = 3;
pub const WICJpegYCrCbSubsamplingDefault: WICJpegYCrCbSubsamplingOption = 0;
pub type WICJpegYCrCbSubsamplingOption = i32;
pub const WICMETADATACACHEOPTION_FORCE_DWORD: WICDecodeOptions = 2147483647;
pub const WICMetadataReader: WICComponentType = 8;
pub const WICMetadataWriter: WICComponentType = 16;
pub const WICNAMEDWHITEPOINT_FORCE_DWORD: WICNamedWhitePoint = 2147483647;
pub type WICNamedWhitePoint = i32;
pub const WICPLANAROPTIONS_FORCE_DWORD: WICPlanarOptions = 2147483647;
pub const WICPNGFILTEROPTION_FORCE_DWORD: WICPngFilterOption = 2147483647;
pub const WICPROGRESSNOTIFICATION_FORCE_DWORD: WICProgressNotification = 2147483647;
pub const WICPROGRESSOPERATION_FORCE_DWORD: WICProgressOperation = 2147483647;
pub const WICPixelFormat: WICComponentType = 32;
pub const WICPixelFormatConverter: WICComponentType = 4;
pub type WICPixelFormatGUID = windows_sys::core::GUID;
pub type WICPixelFormatNumericRepresentation = i32;
pub const WICPixelFormatNumericRepresentationFixed: WICPixelFormatNumericRepresentation = 4;
pub const WICPixelFormatNumericRepresentationFloat: WICPixelFormatNumericRepresentation = 5;
pub const WICPixelFormatNumericRepresentationIndexed: WICPixelFormatNumericRepresentation = 1;
pub const WICPixelFormatNumericRepresentationSignedInteger: WICPixelFormatNumericRepresentation = 3;
pub const WICPixelFormatNumericRepresentationUnsignedInteger: WICPixelFormatNumericRepresentation = 2;
pub const WICPixelFormatNumericRepresentationUnspecified: WICPixelFormatNumericRepresentation = 0;
pub const WICPixelFormatNumericRepresentation_FORCE_DWORD: WICPixelFormatNumericRepresentation = 2147483647;
pub type WICPlanarOptions = i32;
pub const WICPlanarOptionsDefault: WICPlanarOptions = 0;
pub const WICPlanarOptionsPreserveSubsampling: WICPlanarOptions = 1;
pub const WICPngBkgdBackgroundColor: WICPngBkgdProperties = 1;
pub type WICPngBkgdProperties = i32;
pub const WICPngBkgdProperties_FORCE_DWORD: WICPngBkgdProperties = 2147483647;
pub const WICPngChrmBlueX: WICPngChrmProperties = 7;
pub const WICPngChrmBlueY: WICPngChrmProperties = 8;
pub const WICPngChrmGreenX: WICPngChrmProperties = 5;
pub const WICPngChrmGreenY: WICPngChrmProperties = 6;
pub type WICPngChrmProperties = i32;
pub const WICPngChrmProperties_FORCE_DWORD: WICPngChrmProperties = 2147483647;
pub const WICPngChrmRedX: WICPngChrmProperties = 3;
pub const WICPngChrmRedY: WICPngChrmProperties = 4;
pub const WICPngChrmWhitePointX: WICPngChrmProperties = 1;
pub const WICPngChrmWhitePointY: WICPngChrmProperties = 2;
pub const WICPngFilterAdaptive: WICPngFilterOption = 6;
pub const WICPngFilterAverage: WICPngFilterOption = 4;
pub const WICPngFilterNone: WICPngFilterOption = 1;
pub type WICPngFilterOption = i32;
pub const WICPngFilterPaeth: WICPngFilterOption = 5;
pub const WICPngFilterSub: WICPngFilterOption = 2;
pub const WICPngFilterUnspecified: WICPngFilterOption = 0;
pub const WICPngFilterUp: WICPngFilterOption = 3;
pub const WICPngGamaGamma: WICPngGamaProperties = 1;
pub type WICPngGamaProperties = i32;
pub const WICPngGamaProperties_FORCE_DWORD: WICPngGamaProperties = 2147483647;
pub const WICPngHistFrequencies: WICPngHistProperties = 1;
pub type WICPngHistProperties = i32;
pub const WICPngHistProperties_FORCE_DWORD: WICPngHistProperties = 2147483647;
pub const WICPngIccpProfileData: WICPngIccpProperties = 2;
pub const WICPngIccpProfileName: WICPngIccpProperties = 1;
pub type WICPngIccpProperties = i32;
pub const WICPngIccpProperties_FORCE_DWORD: WICPngIccpProperties = 2147483647;
pub const WICPngItxtCompressionFlag: WICPngItxtProperties = 2;
pub const WICPngItxtKeyword: WICPngItxtProperties = 1;
pub const WICPngItxtLanguageTag: WICPngItxtProperties = 3;
pub type WICPngItxtProperties = i32;
pub const WICPngItxtProperties_FORCE_DWORD: WICPngItxtProperties = 2147483647;
pub const WICPngItxtText: WICPngItxtProperties = 5;
pub const WICPngItxtTranslatedKeyword: WICPngItxtProperties = 4;
pub type WICPngSrgbProperties = i32;
pub const WICPngSrgbProperties_FORCE_DWORD: WICPngSrgbProperties = 2147483647;
pub const WICPngSrgbRenderingIntent: WICPngSrgbProperties = 1;
pub const WICPngTimeDay: WICPngTimeProperties = 3;
pub const WICPngTimeHour: WICPngTimeProperties = 4;
pub const WICPngTimeMinute: WICPngTimeProperties = 5;
pub const WICPngTimeMonth: WICPngTimeProperties = 2;
pub type WICPngTimeProperties = i32;
pub const WICPngTimeProperties_FORCE_DWORD: WICPngTimeProperties = 2147483647;
pub const WICPngTimeSecond: WICPngTimeProperties = 6;
pub const WICPngTimeYear: WICPngTimeProperties = 1;
pub type WICProgressNotification = i32;
pub const WICProgressNotificationAll: WICProgressNotification = -65536;
pub const WICProgressNotificationBegin: WICProgressNotification = 65536;
pub const WICProgressNotificationEnd: WICProgressNotification = 131072;
pub const WICProgressNotificationFrequent: WICProgressNotification = 262144;
pub type WICProgressOperation = i32;
pub const WICProgressOperationAll: WICProgressOperation = 65535;
pub const WICProgressOperationCopyPixels: WICProgressOperation = 1;
pub const WICProgressOperationWritePixels: WICProgressOperation = 2;
pub const WICRAWCAPABILITIES_FORCE_DWORD: WICRawCapabilities = 2147483647;
pub const WICRAWPARAMETERSET_FORCE_DWORD: WICRawParameterSet = 2147483647;
pub const WICRAWRENDERMODE_FORCE_DWORD: WICRawRenderMode = 2147483647;
pub const WICRAWROTATIONCAPABILITIES_FORCE_DWORD: WICRawRotationCapabilities = 2147483647;
pub type WICRawCapabilities = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const WICRawCapabilityFullySupported: WICRawCapabilities = 2;
pub const WICRawCapabilityGetSupported: WICRawCapabilities = 1;
pub const WICRawCapabilityNotSupported: WICRawCapabilities = 0;
pub const WICRawChangeNotification_Contrast: u32 = 16;
pub const WICRawChangeNotification_DestinationColorContext: u32 = 1024;
pub const WICRawChangeNotification_ExposureCompensation: u32 = 1;
pub const WICRawChangeNotification_Gamma: u32 = 32;
pub const WICRawChangeNotification_KelvinWhitePoint: u32 = 4;
pub const WICRawChangeNotification_NamedWhitePoint: u32 = 2;
pub const WICRawChangeNotification_NoiseReduction: u32 = 512;
pub const WICRawChangeNotification_RGBWhitePoint: u32 = 8;
pub const WICRawChangeNotification_RenderMode: u32 = 8192;
pub const WICRawChangeNotification_Rotation: u32 = 4096;
pub const WICRawChangeNotification_Saturation: u32 = 128;
pub const WICRawChangeNotification_Sharpness: u32 = 64;
pub const WICRawChangeNotification_Tint: u32 = 256;
pub const WICRawChangeNotification_ToneCurve: u32 = 2048;
pub type WICRawParameterSet = i32;
pub type WICRawRenderMode = i32;
pub const WICRawRenderModeBestQuality: WICRawRenderMode = 3;
pub const WICRawRenderModeDraft: WICRawRenderMode = 1;
pub const WICRawRenderModeNormal: WICRawRenderMode = 2;
pub type WICRawRotationCapabilities = i32;
pub const WICRawRotationCapabilityFullySupported: WICRawRotationCapabilities = 3;
pub const WICRawRotationCapabilityGetSupported: WICRawRotationCapabilities = 1;
pub const WICRawRotationCapabilityNinetyDegreesSupported: WICRawRotationCapabilities = 2;
pub const WICRawRotationCapabilityNotSupported: WICRawRotationCapabilities = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WICRawToneCurve {
    pub cPoints: u32,
    pub aPoints: [WICRawToneCurvePoint; 1],
}
impl Default for WICRawToneCurve {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WICRawToneCurvePoint {
    pub Input: f64,
    pub Output: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WICRect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
pub type WICSectionAccessLevel = i32;
pub const WICSectionAccessLevelRead: WICSectionAccessLevel = 1;
pub const WICSectionAccessLevelReadWrite: WICSectionAccessLevel = 3;
pub const WICSectionAccessLevel_FORCE_DWORD: WICSectionAccessLevel = 2147483647;
pub const WICTIFFCOMPRESSIONOPTION_FORCE_DWORD: WICTiffCompressionOption = 2147483647;
pub const WICTiffCompressionCCITT3: WICTiffCompressionOption = 2;
pub const WICTiffCompressionCCITT4: WICTiffCompressionOption = 3;
pub const WICTiffCompressionDontCare: WICTiffCompressionOption = 0;
pub const WICTiffCompressionLZW: WICTiffCompressionOption = 4;
pub const WICTiffCompressionLZWHDifferencing: WICTiffCompressionOption = 7;
pub const WICTiffCompressionNone: WICTiffCompressionOption = 1;
pub type WICTiffCompressionOption = i32;
pub const WICTiffCompressionRLE: WICTiffCompressionOption = 5;
pub const WICTiffCompressionZIP: WICTiffCompressionOption = 6;
pub const WICUserAdjustedParameterSet: WICRawParameterSet = 2;
pub const WICWebpAnimLoopCount: WICWebpAnimProperties = 1;
pub type WICWebpAnimProperties = i32;
pub const WICWebpAnimProperties_FORCE_DWORD: WICWebpAnimProperties = 2147483647;
pub const WICWebpAnmfFrameDuration: WICWebpAnmfProperties = 1;
pub type WICWebpAnmfProperties = i32;
pub const WICWebpAnmfProperties_FORCE_DWORD: WICWebpAnmfProperties = 2147483647;
pub const WICWhitePointAsShot: WICNamedWhitePoint = 1;
pub const WICWhitePointAutoWhiteBalance: WICNamedWhitePoint = 512;
pub const WICWhitePointCloudy: WICNamedWhitePoint = 4;
pub const WICWhitePointCustom: WICNamedWhitePoint = 256;
pub const WICWhitePointDaylight: WICNamedWhitePoint = 2;
pub const WICWhitePointDefault: WICNamedWhitePoint = 1;
pub const WICWhitePointFlash: WICNamedWhitePoint = 64;
pub const WICWhitePointFluorescent: WICNamedWhitePoint = 32;
pub const WICWhitePointShade: WICNamedWhitePoint = 8;
pub const WICWhitePointTungsten: WICNamedWhitePoint = 16;
pub const WICWhitePointUnderwater: WICNamedWhitePoint = 128;
pub const WIC_JPEG_HUFFMAN_BASELINE_ONE: u32 = 0;
pub const WIC_JPEG_HUFFMAN_BASELINE_THREE: u32 = 1118464;
pub const WIC_JPEG_MAX_COMPONENT_COUNT: u32 = 4;
pub const WIC_JPEG_MAX_TABLE_INDEX: u32 = 3;
pub const WIC_JPEG_QUANTIZATION_BASELINE_ONE: u32 = 0;
pub const WIC_JPEG_QUANTIZATION_BASELINE_THREE: u32 = 65792;
pub const WIC_JPEG_SAMPLE_FACTORS_ONE: u32 = 17;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_420: u32 = 1118498;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_422: u32 = 1118497;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_440: u32 = 1118482;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_444: u32 = 1118481;
pub const WINCODEC_ERR_ABORTED: i32 = -2147467260;
pub const WINCODEC_ERR_ACCESSDENIED: i32 = -2147024891;
pub const WINCODEC_ERR_BASE: u32 = 8192;
pub const WINCODEC_ERR_GENERIC_ERROR: i32 = -2147467259;
pub const WINCODEC_ERR_INVALIDPARAMETER: i32 = -2147024809;
pub const WINCODEC_ERR_NOTIMPLEMENTED: i32 = -2147467263;
pub const WINCODEC_ERR_OUTOFMEMORY: i32 = -2147024882;
pub const WINCODEC_ERR_VALUEOVERFLOW: i32 = -2147024362;
pub const WINCODEC_SDK_VERSION: u32 = 567;
pub const WINCODEC_SDK_VERSION1: u32 = 566;
pub const WINCODEC_SDK_VERSION2: u32 = 567;
